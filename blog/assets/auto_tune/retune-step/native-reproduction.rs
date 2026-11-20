use rustfft::{FftPlanner, num_complex::Complex32};
const YIN_FRAME_SIZE: usize = 2048;

/// Hop between successive YIN frames (samples). Matches the PSOLA hop_size.
const YIN_HOP_SIZE: usize = 512;

/// Voiced-segment onset: seed `ema_zero_phase_midi` from the short-run median
/// of the first few YIN frames so the *intent* curve at the attack reflects
/// the steady-state pitch rather than a single unstable detection.
///
/// Critically, this only seeds the **intent** smoother — `src_midi` and
/// `source_pitches` themselves are left untouched, so the natural pitch
/// transient at the attack is fully captured by the `vib = src - intent`
/// term and re-added on top of the scale-snapped centre.  That preserves
/// the original audio's pitch trajectory at the start of every voiced
/// phrase and avoids the "glide pulled down from above" artefact that
/// appears when the target curve is forced toward the scale snap before
/// the audio has settled.
const SEGMENT_ONSET_ANCHOR_FRAMES: usize = 6;

/// Minimum detectable fundamental (Hz) — covers male bass.
const YIN_F_MIN: f32 = 70.0;

/// Maximum detectable fundamental (Hz) — covers soprano upper range.
const YIN_F_MAX: f32 = 1000.0;

/// CMNDF threshold below which a dip is accepted as a pitch period.
/// Literature-recommended value (de Cheveigné & Kawahara 2002).
const YIN_THRESHOLD: f64 = 0.1;

// ─── pitch math helpers ───────────────────────────────────────────────────────

fn freq_to_midi(freq: f64) -> f64 {
    69.0 + 12.0 * (freq / 440.0).log2()
}

fn midi_to_freq(midi: f64) -> f64 {
    440.0 * 2.0_f64.powf((midi - 69.0) / 12.0)
}

/// All MIDI note numbers 0-127 that belong to `root_pc` + `intervals`.
fn scale_midi_set(root_pc: i32, intervals: &[i32]) -> Vec<i32> {
    let mut out = Vec::new();
    for octave in 0i32..=9 {
        for &iv in intervals {
            let midi = octave * 12 + root_pc + iv;
            if (0..=127).contains(&midi) {
                out.push(midi);
            }
        }
    }
    out
}

/// Nearest MIDI note (as float) in `candidates` to `midi_float`.
fn nearest_scale_midi(midi_float: f64, candidates: &[i32]) -> f64 {
    candidates
        .iter()
        .min_by(|&&a, &&b| {
            (midi_float - a as f64)
                .abs()
                .partial_cmp(&(midi_float - b as f64).abs())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|&m| m as f64)
        .unwrap_or(midi_float)
}

/// Cents the held scale note must lose by before the target switches.
/// Applied to the slow intent curve so vibrato does not flip the note.
const SCALE_NOTE_HOLD_CENTS: f64 = 40.0;

/// Stay on `prev` until another scale note is closer by `hold_cents`.
/// A new phrase (`prev == None`) takes the nearest note.
fn nearest_scale_midi_sticky(
    midi_float: f64,
    candidates: &[i32],
    prev: Option<f64>,
    hold_cents: f64,
) -> f64 {
    let nearest = nearest_scale_midi(midi_float, candidates);
    let Some(prev) = prev else {
        return nearest;
    };
    if (nearest - prev).abs() < 1e-6 {
        return nearest;
    }
    let hold = hold_cents / 100.0;
    let dist_prev = (midi_float - prev).abs();
    let dist_new = (midi_float - nearest).abs();
    if dist_new + hold < dist_prev {
        nearest
    } else {
        prev
    }
}

type YinMidiSeries = Vec<(f64, f64)>;

/// Voiced YIN frames: mean absolute distance (¢) to the nearest tone on `scale_midi`.
fn mean_abs_scale_cents(series: &YinMidiSeries, scale_midi: &[i32]) -> Option<f64> {
    if series.is_empty() || scale_midi.is_empty() {
        return None;
    }
    let sum: f64 = series
        .iter()
        .map(|(_, midi)| {
            let target = nearest_scale_midi(*midi, scale_midi);
            (*midi - target).abs() * 100.0
        })
        .sum();
    Some(sum / series.len() as f64)
}

fn intonation_apply_summary(before_cents: f64, after_cents: f64) -> String {
    let delta = before_cents - after_cents;
    if before_cents < 0.5 && after_cents < 0.5 {
        return "Voiced frames were already within half a cent of the selected scale on average — this take sat on the scale tones before and after tuning.".to_string();
    }
    if delta >= 8.0 {
        format!(
            "Before tuning, voiced frames averaged {:.0}¢ from the nearest scale tone; after Auto-Tune {:.0}¢ — most of the drift toward the scale was pulled in. Use Load Original / Load Auto-Tuned to hear the difference.",
            before_cents, after_cents
        )
    } else if delta >= 2.0 {
        format!(
            "Before tuning, voiced frames averaged {:.0}¢ from the nearest scale tone; after Auto-Tune {:.0}¢ — a clear tightening toward the scale. Use Load Original / Load Auto-Tuned to compare.",
            before_cents, after_cents
        )
    } else if after_cents <= before_cents {
        format!(
            "Before tuning, voiced frames averaged {:.0}¢ from the nearest scale tone; after Auto-Tune {:.0}¢ — similar average offset. Try higher Strength or a narrower scale if you want more correction.",
            before_cents, after_cents
        )
    } else {
        format!(
            "Before tuning, voiced frames averaged {:.0}¢ from the nearest scale tone; after Auto-Tune {:.0}¢. Check Tolerance — notes left inside the dead zone are not pulled toward the scale.",
            before_cents, after_cents
        )
    }
}

// ─── pitch sequence smoother ──────────────────────────────────────────────────

/// 3-element sliding median over a pitch sequence (0 = unvoiced, pass through).
///
/// Kept intentionally narrow (half-width = 1) so it removes *single-frame*
/// detector glitches without smearing real pitch motion.  Anything wider
/// would start eating vibrato amplitude — vibrato (≈5–7 Hz, ≈150–200 ms
/// period) is barely longer than one analysis frame, so a 7-tap median
/// like the old version effectively flattened it before any tuning ran.
fn median_smooth(pitches: &[f64]) -> Vec<f64> {
    let n = pitches.len();
    let half = 1usize;
    let mut out = pitches.to_vec();
    for i in 0..n {
        let start = i.saturating_sub(half);
        let end = (i + half + 1).min(n);
        let mut window: Vec<f64> = pitches[start..end]
            .iter()
            .filter(|&&v| v > 0.0)
            .cloned()
            .collect();
        if window.is_empty() {
            out[i] = 0.0;
        } else {
            window.sort_by(|a, b| a.partial_cmp(b).unwrap());
            out[i] = window[window.len() / 2];
        }
    }
    out
}

/// Linear MIDI interpolation of a hop-sampled pitch contour at time `ts`.
fn sample_pitch_hz_at_time(
    pitches: &[f64],
    sample_rate: f32,
    hop_size: usize,
    ts: f64,
) -> f64 {
    if pitches.is_empty() {
        return 0.0;
    }
    let fi = (ts * sample_rate as f64 / hop_size as f64).max(0.0);
    let i0 = fi.floor() as usize;
    let frac = fi - i0 as f64;
    let i1 = (i0 + 1).min(pitches.len() - 1);
    let p0 = pitches.get(i0).copied().unwrap_or(0.0);
    let p1 = pitches.get(i1).copied().unwrap_or(0.0);
    if p0 <= 0.0 && p1 <= 0.0 {
        return 0.0;
    }
    if p0 <= 0.0 {
        return p1;
    }
    if p1 <= 0.0 {
        return p0;
    }
    let m0 = freq_to_midi(p0);
    let m1 = freq_to_midi(p1);
    midi_to_freq(m0 + (m1 - m0) * frac)
}

// ─── pitch contour decomposition (retune speed) ──────────────────────────────

/// Zero-phase EMA over a MIDI-pitch sequence, run per *voiced segment* so
/// silences never leak old state into a new phrase.  Returns the **intent**
/// curve — the slow, scale-relevant component of the pitch contour.
///
/// `alpha` ∈ (0, 1]: smaller = slower retune (more vibrato/ornament preserved
/// in `src - intent`).  α = 1.0 disables smoothing entirely (intent = src,
/// which reduces the pipeline to the classic "hard snap" autotune).
fn ema_zero_phase_midi(midi: &[f64], voiced: &[bool], alpha: f64) -> Vec<f64> {
    let n = midi.len();
    let mut out = vec![0.0f64; n];
    if alpha >= 1.0 - 1e-9 {
        // Effectively no smoothing — intent equals source, vibrato is fully
        // captured by the snap step (i.e., wiped out).  Robotic / T-Pain.
        return midi.to_vec();
    }
    let alpha = alpha.clamp(1e-6, 1.0);

    let mut i = 0usize;
    while i < n {
        while i < n && !voiced[i] {
            i += 1;
        }
        let seg_start = i;
        while i < n && voiced[i] {
            i += 1;
        }
        let seg_end = i;
        if seg_end <= seg_start {
            continue;
        }
        let len = seg_end - seg_start;

        // Forward pass — seed from median of early stable frames, not frame 0 alone.
        let seed_end = (seg_start + SEGMENT_ONSET_ANCHOR_FRAMES.min(len)).min(seg_end);
        let mut seed_vals: Vec<f64> = (seg_start..seed_end)
            .filter_map(|i| {
                let m = midi[i];
                (m > 0.0).then_some(m)
            })
            .collect();
        seed_vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut state = if seed_vals.is_empty() {
            midi[seg_start]
        } else {
            seed_vals[seed_vals.len() / 2]
        };

        // Forward pass.
        let mut fwd = vec![0.0f64; len];
        fwd[0] = state;
        for k in 1..len {
            state = alpha * midi[seg_start + k] + (1.0 - alpha) * state;
            fwd[k] = state;
        }

        // Backward pass on fwd — gives zero phase delay so the intent
        // curve stays aligned with the original pitch in time.
        let mut state = fwd[len - 1];
        out[seg_start + len - 1] = state;
        for k in (0..len - 1).rev() {
            state = alpha * fwd[k] + (1.0 - alpha) * state;
            out[seg_start + k] = state;
        }
    }

    out
}

/// Convert a 0–100 "retune speed" percentage to an EMA alpha given the
/// per-frame hop in seconds.
///
/// Mapping:
///   - 0%   → α = 1.0  (instant retune, classic robotic autotune)
///   - 50%  → ≈ 80 ms time constant (musical, light vibrato preserved)
///   - 100% → ≈ 500 ms time constant (gentle, full vibrato + ornament preserved)
///
/// Log-spaced so the slider feel is musically uniform across the range.
fn retune_alpha(speed_pct: u32, hop_sec: f64) -> f64 {
    if speed_pct == 0 {
        return 1.0; // disables intent smoothing entirely
    }
    let ms = 10.0_f64 * 50.0_f64.powf(speed_pct as f64 / 100.0);
    let tc_frames = (ms / 1000.0 / hop_sec).max(1.0e-3);
    1.0 - (-1.0 / tc_frames).exp()
}

/// Strength multiplier for one intent-vs-scale offset.
///
/// Tolerance is a dead zone: pitches already within `tol_cents` of the
/// target note are left alone. Correction fades in across `falloff_cents`
/// and is full after that, so notes that are actually out of tune move.
///
///       1.0 ┤         ╭──────
///           │        /
///       0.0 ┼───────╯
///           tol    tol+falloff   → err_cents
fn tolerance_gate(err_cents: f64, tol_cents: f64, falloff_cents: f64) -> f64 {
    if falloff_cents <= 1e-6 {
        return if err_cents <= tol_cents { 0.0 } else { 1.0 };
    }
    if err_cents <= tol_cents {
        0.0
    } else if err_cents >= tol_cents + falloff_cents {
        1.0
    } else {
        (err_cents - tol_cents) / falloff_cents
    }
}

/// Keep the pitch correction from jumping faster than `max_cents_per_sec`.
///
/// A scale-note change can move the target by a semitone in one analysis
/// frame. PSOLA then changes grain spacing in a single hop and the splice
/// clicks. Spreading that move over a few frames leaves a short glide.
fn limit_pitch_shift_slew(
    source: &[f64],
    target: &[f64],
    hop_sec: f64,
    max_cents_per_sec: f64,
) -> Vec<f64> {
    let max_step = (max_cents_per_sec * hop_sec).max(1.0);
    let mut out = target.to_vec();
    let mut prev_shift: Option<f64> = None;
    let mut gap = 0u32;
    for i in 0..source.len() {
        let src = source[i];
        let tgt = target.get(i).copied().unwrap_or(0.0);
        if src <= 0.0 || tgt <= 0.0 {
            gap += 1;
            if gap > 2 {
                prev_shift = None;
            }
            continue;
        }
        gap = 0;
        let mut shift = 1200.0 * (tgt / src).log2();
        if let Some(prev) = prev_shift {
            shift = prev + (shift - prev).clamp(-max_step, max_step);
        }
        prev_shift = Some(shift);
        out[i] = src * 2.0_f64.powf(shift / 1200.0);
    }
    out
}

// ─── zero-phase lowpass (used only for GCI detection) ────────────────────────

/// 2nd-order Butterworth lowpass, applied forward then backward (zero-phase).
///
/// Used to strip formant ripple from the signal *before* peak-picking pitch
/// marks.  The mark search then sees a near-sinusoid at the f0 + first few
/// harmonics, where the dominant peak per cycle is unambiguous and matches
/// the glottal closure instant in polarity.  The output of this function is
/// **never** mixed into the audio — only into the mark detector.
fn biquad_lowpass_zero_phase(input: &[f32], sample_rate: f32, cutoff_hz: f32) -> Vec<f32> {
    if input.is_empty() {
        return Vec::new();
    }
    let omega = 2.0 * std::f32::consts::PI * cutoff_hz / sample_rate;
    let sn = omega.sin();
    let cs = omega.cos();
    let q = std::f32::consts::FRAC_1_SQRT_2; // Butterworth Q
    let alpha = sn / (2.0 * q);

    let b0 = (1.0 - cs) * 0.5;
    let b1 = 1.0 - cs;
    let b2 = (1.0 - cs) * 0.5;
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * cs;
    let a2 = 1.0 - alpha;

    let b0n = b0 / a0;
    let b1n = b1 / a0;
    let b2n = b2 / a0;
    let a1n = a1 / a0;
    let a2n = a2 / a0;

    fn run(coeffs: (f32, f32, f32, f32, f32), x: &[f32]) -> Vec<f32> {
        let (b0, b1, b2, a1, a2) = coeffs;
        let mut y = vec![0.0f32; x.len()];
        let mut x1 = 0.0f32;
        let mut x2 = 0.0f32;
        let mut y1 = 0.0f32;
        let mut y2 = 0.0f32;
        for i in 0..x.len() {
            let yi = b0 * x[i] + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
            x2 = x1;
            x1 = x[i];
            y2 = y1;
            y1 = yi;
            y[i] = yi;
        }
        y
    }

    let coeffs = (b0n, b1n, b2n, a1n, a2n);
    let fwd = run(coeffs, input);
    let mut rev: Vec<f32> = fwd.iter().rev().copied().collect();
    rev = run(coeffs, &rev);
    rev.iter().rev().copied().collect()
}

// ─── YIN pitch detection ──────────────────────────────────────────────────────

/// Estimate fundamental frequency of `frame` using the YIN algorithm
/// (de Cheveigné & Kawahara 2002).
///
/// Returns the estimated Hz if voiced, or `0.0` if unvoiced / out of range.
///
/// Steps:
///   1. Difference function  `d[τ] = Σ (x[i] − x[i+τ])²`
///   2. CMNDF               `d'[τ] = d[τ] · τ / Σ_{j=1}^{τ} d[j]`  (d'[0] = 1)
///   3. Threshold search    first local minimum of d' below `YIN_THRESHOLD`
///   4. Parabolic interp.   sub-sample refinement of the best τ
pub(crate) fn yin_pitch(frame: &[f32], sample_rate: f32, f_min: f32, f_max: f32) -> f64 {
    let n = frame.len();
    if n == 0 {
        return 0.0;
    }

    let tau_min = (sample_rate / f_max).floor() as usize;
    let tau_max = ((sample_rate / f_min).ceil() as usize).min(n / 2);
    if tau_min == 0 || tau_min >= tau_max {
        return 0.0;
    }

    // Step 1 & 2: difference function + CMNDF. The difference is the
    // autocorrelation of a fixed window, computed with one FFT pair.
    let cmndf = yin_cmndf(frame, tau_max);

    // Step 3: Find the first local minimum of CMNDF below YIN_THRESHOLD.
    let mut best_tau = 0usize;
    'search: for tau in tau_min..tau_max {
        if cmndf[tau] < YIN_THRESHOLD && cmndf[tau] <= cmndf[tau + 1] {
            best_tau = tau;
            break 'search;
        }
    }

    // Fallback: global minimum in range (accept if < 0.5, else treat as unvoiced).
    if best_tau == 0 {
        let mut min_val = f64::MAX;
        for tau in tau_min..=tau_max {
            if cmndf[tau] < min_val {
                min_val = cmndf[tau];
                best_tau = tau;
            }
        }
        if min_val > 0.5 {
            return 0.0;
        }
    }

    if best_tau == 0 {
        return 0.0;
    }

    // Step 4: Parabolic interpolation for sub-sample τ*.
    //   offset = (d'[τ−1] − d'[τ+1]) / (2 · (d'[τ−1] − 2·d'[τ] + d'[τ+1]))
    let tau_f = if best_tau > 0 && best_tau < tau_max {
        let y0 = cmndf[best_tau - 1];
        let y1 = cmndf[best_tau];
        let y2 = cmndf[best_tau + 1];
        let denom = 2.0 * (y0 - 2.0 * y1 + y2);
        if denom.abs() > 1e-10 {
            best_tau as f64 + (y0 - y2) / (2.0 * denom)
        } else {
            best_tau as f64
        }
    } else {
        best_tau as f64
    };

    if tau_f <= 0.0 {
        return 0.0;
    }

    sample_rate as f64 / tau_f
}

thread_local! {
    static YIN_FFT_PLANNER: std::cell::RefCell<FftPlanner<f32>> =
        std::cell::RefCell::new(FftPlanner::<f32>::new());
}

/// CMNDF for lags `1..=tau_max`. `d'[0] = 1`.
///
/// `d[τ] = Σ_{i=0}^{win-1} (x[i] − x[i+τ])²` with `win = n − tau_max`,
/// via `Σx² + Σx_shifted² − 2·Σ x[i] x[i+τ]`. The cross term is one FFT
/// cross-correlation.
fn yin_cmndf(frame: &[f32], tau_max: usize) -> Vec<f64> {
    let n = frame.len();
    let mut cmndf = vec![1.0f64; tau_max + 1];
    if tau_max == 0 || n <= tau_max {
        return cmndf;
    }
    let win = n - tau_max;
    let mut cum = vec![0.0f64; n + 1];
    for i in 0..n {
        let s = frame[i] as f64;
        cum[i + 1] = cum[i] + s * s;
    }

    let fft_len = (win + n).next_power_of_two().max(2);
    let mut spec_a = vec![Complex32::new(0.0, 0.0); fft_len];
    let mut spec_b = vec![Complex32::new(0.0, 0.0); fft_len];
    for i in 0..win {
        spec_a[i].re = frame[i];
    }
    for i in 0..n {
        spec_b[i].re = frame[i];
    }

    YIN_FFT_PLANNER.with(|planner| {
        let mut planner = planner.borrow_mut();
        let fwd = planner.plan_fft_forward(fft_len);
        let inv = planner.plan_fft_inverse(fft_len);
        fwd.process(&mut spec_a);
        fwd.process(&mut spec_b);
        for i in 0..fft_len {
            let a = spec_a[i];
            let b = spec_b[i];
            // ifft(fft(a) * conj(fft(b)))[k] = Σ a[n] b[n−k]
            spec_a[i] = Complex32::new(
                a.re * b.re + a.im * b.im,
                a.im * b.re - a.re * b.im,
            );
        }
        inv.process(&mut spec_a);
    });

    let scale = fft_len as f64;
    let sum_a = cum[win];
    let mut running_sum = 0.0f64;
    for tau in 1..=tau_max {
        let sum_b = cum[tau + win] - cum[tau];
        // Σ a[i] b[i+τ] sits at lag (N − τ) under the n−k convention.
        let corr = spec_a[fft_len - tau].re as f64 / scale;
        let diff = (sum_a + sum_b - 2.0 * corr).max(0.0);
        running_sum += diff;
        cmndf[tau] = if running_sum > 1e-10 {
            diff * tau as f64 / running_sum
        } else {
            1.0
        };
    }
    cmndf
}

/// Analysis rate near 12 kHz. Pitch tracking does not need the full file rate,
/// and the YIN search shrinks with the rate.
fn analysis_decimation(sample_rate: f32) -> usize {
    if sample_rate <= 16_000.0 {
        1
    } else {
        (sample_rate / 12_000.0).round().clamp(2.0, 8.0) as usize
    }
}

fn decimate_box(input: &[f32], factor: usize) -> Vec<f32> {
    if factor <= 1 {
        return input.to_vec();
    }
    let n_out = input.len() / factor;
    let mut out = Vec::with_capacity(n_out);
    let inv = 1.0 / factor as f32;
    for i in 0..n_out {
        let start = i * factor;
        let mut sum = 0.0f32;
        for k in 0..factor {
            sum += input[start + k];
        }
        out.push(sum * inv);
    }
    out
}

/// `(analysis_samples, analysis_rate, frame, analysis_hop, original_hop)`.
fn analysis_grid(mono: &[f32], sample_rate: f32) -> (Vec<f32>, f32, usize, usize, usize) {
    let dec = analysis_decimation(sample_rate);
    let analysis = if dec == 1 {
        mono.to_vec()
    } else {
        decimate_box(mono, dec)
    };
    let analysis_sr = sample_rate / dec as f32;
    let frame = (YIN_FRAME_SIZE / dec).max(64);
    let hop_analysis = (YIN_HOP_SIZE / dec).max(1);
    let hop_orig = hop_analysis * dec;
    (analysis, analysis_sr, frame, hop_analysis, hop_orig)
}

/// Pitch track. `progress_start`..`progress_end` advances with the frame loop.
fn yin_pitch_sequence(
    mono: &[f32],
    sample_rate: f32,
    frame_size: usize,
    hop_size: usize,
    progress_start: f64,
    progress_end: f64,
) -> Result<Vec<f64>, String> {
    let n = mono.len();
    if n == 0 || hop_size == 0 || frame_size == 0 {
        return Ok(Vec::new());
    }
    let n_frames = (n + hop_size - 1) / hop_size;
    let mut pitches = Vec::with_capacity(n_frames);
    let lead = yin_center_lead(frame_size, sample_rate);

    for frame_idx in 0..n_frames {
        if frame_idx % 12 == 0 {
            let denom = n_frames.max(1) as f64;
            let t = frame_idx as f64 / denom;
        }
        let frame = yin_frame(mono, frame_size, (frame_idx * hop_size) as isize - lead as isize);
        pitches.push(yin_pitch(&frame, sample_rate, YIN_F_MIN, YIN_F_MAX));
    }

    Ok(pitches)
}

/// Samples from the start of a YIN frame to the moment its pitch describes.
///
/// The difference function compares `x[0..win]` with `x[τ..τ+win]`, so the
/// estimate belongs near `(win + τ) / 2`. Frame `i` is centred there on
/// sample `i·hop`. Stamping it at the frame start put every pitch ~20 ms
/// early: the grain spacing then disagreed with the waveform on every
/// note change and every vibrato swing.
fn yin_center_lead(frame_size: usize, sample_rate: f32) -> usize {
    let tau_max = ((sample_rate / YIN_F_MIN).ceil() as usize).min(frame_size / 2);
    let tau_typ = ((sample_rate / 200.0) as usize).min(tau_max);
    (frame_size - tau_max + tau_typ) / 2
}

/// `frame_size` samples starting at `start`, zero outside the signal.
fn yin_frame(mono: &[f32], frame_size: usize, start: isize) -> Vec<f32> {
    let mut f = vec![0.0f32; frame_size];
    let lo = start.max(0) as usize;
    let hi = ((start + frame_size as isize).max(0) as usize).min(mono.len());
    if lo < hi {
        let off = (lo as isize - start) as usize;
        f[off..off + (hi - lo)].copy_from_slice(&mono[lo..hi]);
    }
    f
}

fn pitches_to_yin_series(pitches: &[f64], hop_sec: f64) -> YinMidiSeries {
    pitches
        .iter()
        .enumerate()
        .filter_map(|(i, &f)| {
            if f > 0.0 {
                Some((i as f64 * hop_sec, freq_to_midi(f)))
            } else {
                None
            }
        })
        .collect()
}

// ─── per-sample pitch contour ────────────────────────────────────────────────

/// Expand the frame-level pitch estimates to a per-sample contour by
/// linearly interpolating in log-frequency space.  Returns
/// `(src_period_samples, tgt_period_samples, is_voiced)` for every sample.
///
/// Log-frequency interpolation matches how humans perceive pitch glides
/// (linear semitones), so the grain spacing stays musically smooth across
/// vibrato and portamento instead of jumping at frame boundaries.
fn build_period_contour(
    n: usize,
    sample_rate: f64,
    source_pitches: &[f64],
    target_pitches: &[f64],
    hop_size: usize,
) -> (Vec<f64>, Vec<f64>, Vec<bool>) {
    let num_frames = source_pitches.len();
    let mut src_p = vec![0.0f64; n];
    let mut tgt_p = vec![0.0f64; n];
    let mut voiced = vec![false; n];

    if num_frames == 0 || hop_size == 0 {
        return (src_p, tgt_p, voiced);
    }

    for i in 0..n {
        let f = i as f64 / hop_size as f64;
        let f0 = (f.floor() as usize).min(num_frames - 1);
        let f1 = (f0 + 1).min(num_frames - 1);
        let a = (f - f0 as f64).clamp(0.0, 1.0);

        let sp0 = source_pitches[f0];
        let sp1 = source_pitches[f1];
        let tp0 = target_pitches[f0];
        let tp1 = target_pitches[f1];

        if sp0 > 0.0 && sp1 > 0.0 && tp0 > 0.0 && tp1 > 0.0 {
            let sp = (sp0.ln() * (1.0 - a) + sp1.ln() * a).exp();
            let tp = (tp0.ln() * (1.0 - a) + tp1.ln() * a).exp();
            src_p[i] = sample_rate / sp;
            tgt_p[i] = sample_rate / tp;
            voiced[i] = true;
        }
    }

    (src_p, tgt_p, voiced)
}

// ─── pitch outlier removal ───────────────────────────────────────────────────

/// Replace single-frame octave jumps with the *octave-snapped* value, so a
/// stray harmonic in the detector's "loudest peak" pick doesn't propagate
/// into PSOLA as a halved or doubled grain period.
///
/// Detection: a frame counts as an outlier when both neighbouring voiced
/// frames agree (within ~3 semitones of each other) but this frame is ≥ 6
/// semitones away from their geometric mean.  We then snap by ×2 or ×½ to
/// pull it back into the same octave — this is the only ambiguity our
/// detector typically falls into in practice.
fn remove_octave_outliers(pitches: &mut [f64]) {
    let n = pitches.len();
    if n < 3 {
        return;
    }
    for i in 1..n - 1 {
        let p = pitches[i];
        if p <= 0.0 {
            continue;
        }
        let prev = pitches[i - 1];
        let next = pitches[i + 1];
        if prev <= 0.0 || next <= 0.0 {
            continue;
        }

        let neighbor_mean = (prev * next).sqrt();
        let neighbor_diff_semi = (prev / next).log2().abs() * 12.0;
        if neighbor_diff_semi > 3.0 {
            continue; // neighbours disagree — can't decide
        }
        let semi_from_mean = (p / neighbor_mean).log2().abs() * 12.0;
        if semi_from_mean < 6.0 {
            continue; // already in range
        }

        // Try octave snaps in both directions; pick the one closest to mean.
        let candidates = [p, p * 2.0, p * 0.5, p * 4.0, p * 0.25];
        let mut best = p;
        let mut best_err = semi_from_mean;
        for &c in &candidates {
            let e = (c / neighbor_mean).log2().abs() * 12.0;
            if e < best_err {
                best_err = e;
                best = c;
            }
        }
        if best_err < 3.0 {
            pitches[i] = best;
        } else {
            // No octave alias brings it close — treat as bad detection.
            pitches[i] = 0.0;
        }
    }
}

/// Fold short runs of octave errors back to the singer's register.
///
/// YIN on a breathy or strongly second-harmonic voice can lock onto the
/// wrong octave for several frames in a row, which the single-frame repair
/// above cannot see. Each frame is compared with the median of the voiced
/// frames within ±`OCTAVE_FOLD_RADIUS` in the same segment; anything 9+
/// semitones away is moved by whole octaves toward it. A real octave leap
/// moves the median with it, so it is kept.
const OCTAVE_FOLD_RADIUS: usize = 12;

fn fold_octave_runs(pitches: &mut [f64]) {
    let n = pitches.len();
    let src = pitches.to_vec();
    let mut i = 0usize;
    while i < n {
        if src[i] <= 0.0 {
            i += 1;
            continue;
        }
        let start = i;
        while i < n && src[i] > 0.0 {
            i += 1;
        }
        let end = i;
        let midi: Vec<f64> = src[start..end].iter().map(|&f| freq_to_midi(f)).collect();
        for k in 0..midi.len() {
            let lo = k.saturating_sub(OCTAVE_FOLD_RADIUS);
            let hi = (k + OCTAVE_FOLD_RADIUS + 1).min(midi.len());
            let mut win: Vec<f64> = midi[lo..hi].to_vec();
            win.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let med = win[win.len() / 2];
            let mut m = midi[k];
            while m - med >= 9.0 {
                m -= 12.0;
            }
            while med - m >= 9.0 {
                m += 12.0;
            }
            if m != midi[k] {
                pitches[start + k] = midi_to_freq(m);
            }
        }
    }
}

// ─── pitch mark detection ────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct AnalysisMark {
    pos: usize,
    /// Sub-sample peak position. Spacing between marks is the singer's own
    /// cycle length, so it must not carry ±1-sample rounding.
    pos_f: f64,
    src_period: usize,
    /// Index of the contiguous voiced segment this mark belongs to.
    /// PSOLA grains never cross segment boundaries — that's the fix
    /// for the inter-segment buzz where the synthesis loop kept re-using
    /// a stale mark while crossing a silent gap.
    seg_id: u32,
}

/// Detect analysis pitch marks across the whole signal.
///
/// Strategy:
///   1. Process the signal in *contiguous voiced segments*.  We never reset
///      the mark chain inside a segment — that was the source of the
///      "click every frame" buzz in the previous version.
///   2. For each segment, decide whether the dominant peak per cycle is
///      positive- or negative-going by sampling the LP signal across the
///      first few periods.  All subsequent marks in this segment use the
///      same polarity, eliminating cycle-to-cycle phase inversion.
///   3. From the first mark, chain forward: estimate `next ≈ last + period`
///      and refine inside a ±period/6 window by picking the
///      strongest *signed* peak in the LP signal.  The narrow search keeps
///      the chain from skipping or doubling a cycle.
fn segment_gci_cutoff(sample_rate: f32, periods: &[f64]) -> f32 {
    let mut voiced: Vec<f64> = periods.iter().copied().filter(|p| *p > 1.0).collect();
    if voiced.is_empty() {
        return 700.0;
    }
    voiced.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = voiced[voiced.len() / 2];
    let f0 = sample_rate as f64 / median;
    // A few harmonics above this segment's f0, not a fixed 700 Hz.
    (f0 * 3.5).clamp(160.0, 1100.0) as f32
}

/// Hann half-width for one synthesis grain.
///
/// One source period: the classic two-period PSOLA grain. Auto-tune moves
/// at most about a semitone and a half, so grains one target period apart
/// still overlap and the weight normaliser covers the small dip. Only an
/// extreme drop (target over two source periods) widens it, so grains
/// never leave a gap.
fn psola_half_window(src_period: usize, tgt_period: usize, n: usize) -> usize {
    if src_period == 0 || n < 2 {
        return 0;
    }
    let need = src_period.max((tgt_period + 1) / 2);
    need.min(src_period.saturating_mul(2)).min(n / 2)
}

fn detect_pitch_marks(
    signal: &[f32],
    sample_rate: f32,
    src_period_per_sample: &[f64],
    voiced: &[bool],
) -> Vec<AnalysisMark> {
    let n = signal.len();
    let mut marks: Vec<AnalysisMark> = Vec::new();
    if n == 0 {
        return marks;
    }

    let mut i = 0usize;
    let mut seg_id: u32 = 0;
    while i < n {
        // Advance to start of next voiced segment.
        while i < n && !voiced[i] {
            i += 1;
        }
        if i >= n {
            break;
        }
        let seg_start = i;
        let mut seg_end = seg_start;
        while seg_end < n && voiced[seg_end] {
            seg_end += 1;
        }

        let p0_f = src_period_per_sample[seg_start];
        let p0 = p0_f.round() as usize;
        if p0 == 0 || seg_end - seg_start < p0 + 2 {
            i = seg_end;
            continue;
        }
        let cur_seg = seg_id;
        seg_id += 1;

        let cutoff = segment_gci_cutoff(sample_rate, &src_period_per_sample[seg_start..seg_end]);
        let pad = p0.saturating_mul(2).min(seg_start);
        let slice_start = seg_start - pad;
        let slice_end = (seg_end + p0.saturating_mul(2)).min(n);
        let lp_slice =
            biquad_lowpass_zero_phase(&signal[slice_start..slice_end], sample_rate, cutoff);
        let lp_at = |k: usize| -> f32 { lp_slice.get(k - slice_start).copied().unwrap_or(0.0) };

        // Pick segment polarity from the dominant peak in the first ≤3 periods.
        let probe_end = (seg_start + 3 * p0).min(seg_end);
        let mut pos_max = 0.0f32;
        let mut neg_max = 0.0f32;
        for k in seg_start..probe_end {
            let v = lp_at(k);
            if v > pos_max {
                pos_max = v;
            }
            if -v > neg_max {
                neg_max = -v;
            }
        }
        let polarity: f32 = if pos_max >= neg_max { 1.0 } else { -1.0 };

        // First mark: strongest polarity-matched peak within the first period.
        let first_end = (seg_start + p0).min(seg_end);
        let mut first_pos = seg_start;
        let mut first_val = polarity * lp_at(seg_start);
        for k in seg_start..first_end {
            let v = polarity * lp_at(k);
            if v > first_val {
                first_val = v;
                first_pos = k;
            }
        }
        let refine = |k: usize| -> f64 {
            if k == 0 || k + 1 >= n {
                return k as f64;
            }
            let (a, b, c) = (
                (polarity * lp_at(k - 1)) as f64,
                (polarity * lp_at(k)) as f64,
                (polarity * lp_at(k + 1)) as f64,
            );
            let d = a - 2.0 * b + c;
            if d >= -1e-12 {
                return k as f64;
            }
            k as f64 + (0.5 * (a - c) / d).clamp(-0.5, 0.5)
        };
        marks.push(AnalysisMark {
            pos: first_pos,
            pos_f: refine(first_pos),
            src_period: p0,
            seg_id: cur_seg,
        });

        // Chain forward: refine each next mark in a narrow window around
        // (last + local_period).
        loop {
            let last = *marks.last().unwrap();
            let p_local = src_period_per_sample
                .get(last.pos)
                .copied()
                .unwrap_or(p0_f)
                .round() as usize;
            if p_local == 0 {
                break;
            }

            let estimated = last.pos + p_local;
            if estimated + 1 >= seg_end {
                break;
            }

            // Tight ±p/6 search keeps the chain on the *current* cycle.
            let search_half = (p_local / 6).max(1);
            let lo = (estimated.saturating_sub(search_half)).max(last.pos + 1);
            let hi = (estimated + search_half).min(seg_end - 1);
            if lo > hi {
                break;
            }

            let mut best_pos = lo;
            let mut best_val = polarity * lp_at(lo);
            for k in (lo + 1)..=hi {
                let v = polarity * lp_at(k);
                if v > best_val {
                    best_val = v;
                    best_pos = k;
                }
            }

            marks.push(AnalysisMark {
                pos: best_pos,
                pos_f: refine(best_pos),
                src_period: p_local,
                seg_id: cur_seg,
            });
        }

        i = seg_end;
    }

    marks
}

// ─── TD-PSOLA core ───────────────────────────────────────────────────────────

/// Pitch-correct `samples` using formant-preserving TD-PSOLA.
///
/// Pipeline:
///   1. Build per-sample interpolated source/target periods (log-Hz space).
///   2. Compute per-sample pitch shift in cents.  If the shift is zero
///      everywhere, return the input untouched — PSOLA reconstruction is not
///      bit-perfect (mark jitter, sub-sample drift, lost aperiodic detail),
///      so we must avoid running it where no correction is needed.
///   3. Lowpass the signal to ~700 Hz for stable GCI detection.
///   4. Detect analysis marks globally with polarity-consistent chaining.
///   5. Walk synthesis position by the *local* target period at each step,
///      copy the windowed grain from the nearest analysis mark, OLA.
///   6. Each voiced segment is either untouched or PSOLA end to end
///      (`psola_mix`).
fn td_psola(
    samples: &[f32],
    sample_rate: f32,
    source_pitches: &[f64],
    target_pitches: &[f64],
    hop_size: usize,
) -> Result<Vec<f32>, String> {
    let n = samples.len();
    if n == 0 {
        return Ok(Vec::new());
    }
    if source_pitches.is_empty() || target_pitches.is_empty() {
        return Ok(samples.to_vec());
    }

    // 1. Per-sample period contour & voicing.
    let (src_period_at, tgt_period_at, voiced) = build_period_contour(
        n,
        sample_rate as f64,
        source_pitches,
        target_pitches,
        hop_size,
    );

    // 2. Per-sample pitch shift in cents.  shift = 1200·log2(src_period / tgt_period).
    //    Used both for the global passthrough check and the wet-mix mask.
    let mut shift_cents_at = vec![0.0f32; n];
    let mut max_shift: f32 = 0.0;
    for i in 0..n {
        if voiced[i] && src_period_at[i] > 0.0 && tgt_period_at[i] > 0.0 {
            let ratio = src_period_at[i] / tgt_period_at[i];
            let c = (1200.0 * ratio.ln() / std::f64::consts::LN_2).abs() as f32;
            shift_cents_at[i] = c;
            if c > max_shift {
                max_shift = c;
            }
        }
    }

    // 2a. Global passthrough: if no sample needs more than ~1¢ correction,
    //     skip PSOLA entirely.  This is the *only* way to guarantee that
    //     Strength=0 (or a flawless take) produces output identical to input.
    if max_shift < 1.0 {
        return Ok(samples.to_vec());
    }

    // 3–4. Pitch marks. Each voiced segment is lowpassed around its own f0
    //     so peak picking is not stuck at a single 700 Hz cutoff.
    let analysis_marks = detect_pitch_marks(samples, sample_rate, &src_period_at, &voiced);
    if analysis_marks.is_empty() {
        return Ok(samples.to_vec());
    }

    // 4a. Index marks by segment so the synthesis loop can never cross a
    //     voiced→unvoiced→voiced boundary while re-using a stale grain.
    //     `seg_ranges[s] = (start_mark_idx, end_mark_idx_exclusive)`.
    let mut seg_ranges: Vec<(usize, usize)> = Vec::new();
    {
        let mut cur_seg = analysis_marks[0].seg_id;
        let mut cur_start = 0usize;
        for (idx, m) in analysis_marks.iter().enumerate().skip(1) {
            if m.seg_id != cur_seg {
                seg_ranges.push((cur_start, idx));
                cur_start = idx;
                cur_seg = m.seg_id;
            }
        }
        seg_ranges.push((cur_start, analysis_marks.len()));
    }

    // 5. OLA synthesis, one voiced segment at a time. Grains never cross a
    //    silence or consonant.
    let mut voiced_sum = vec![0.0f32; n];
    let mut voiced_w = vec![0.0f32; n];
    for (seg_idx, &(am_start, am_end)) in seg_ranges.iter().enumerate() {
        if seg_idx % 4 == 0 {
        }
        psola_overlap_add(
            samples,
            &voiced,
            &src_period_at,
            &tgt_period_at,
            &analysis_marks[am_start..am_end],
            &mut voiced_sum,
            &mut voiced_w,
        );
    }

    Ok(psola_mix(
        samples,
        sample_rate,
        &voiced,
        &shift_cents_at,
        &voiced_sum,
        &voiced_w,
    ))
}

/// Overlap-add one voiced segment.
///
/// Synthesis epochs advance by the target period. Each takes the grain at
/// the nearest analysis mark: a Hann two source periods wide. Neighbouring
/// grains then differ only by the shift itself, so their overlap stays in
/// phase and the level holds steady. Wider windows pull in cycles that no
/// longer line up after the shift and the level wobbles.
///
/// The step between synthesis epochs is the singer's own cycle length at
/// that mark, scaled by the requested shift. Using the frame pitch
/// estimate as the step instead replaced the voice's cycle-to-cycle
/// micro-variation with a smooth curve — the robotic sound — and at zero
/// shift the output no longer matched the input.
fn psola_overlap_add(
    samples: &[f32],
    voiced: &[bool],
    src_period_at: &[f64],
    tgt_period_at: &[f64],
    marks: &[AnalysisMark],
    sum: &mut [f32],
    weight: &mut [f32],
) {
    let n = samples.len();
    let (Some(&first), Some(&last)) = (marks.first(), marks.last()) else {
        return;
    };
    let end = (last.pos + last.src_period).min(n);
    let mut synth_pos = first.pos_f;
    let mut idx = 0usize;
    while synth_pos < end as f64 {
        let synth_i = synth_pos.round().max(0.0) as usize;
        if synth_i >= n || !voiced[synth_i] {
            break;
        }
        while idx + 1 < marks.len() {
            let cur_d = (marks[idx].pos_f - synth_pos).abs();
            let nxt_d = (marks[idx + 1].pos_f - synth_pos).abs();
            if nxt_d < cur_d {
                idx += 1;
            } else {
                break;
            }
        }
        let mark = marks[idx];
        let src_est = src_period_at[synth_i];
        let tgt_est = tgt_period_at[synth_i];
        // tgt / src period: < 1 raises the pitch.
        let ratio = if src_est > 0.0 && tgt_est > 0.0 {
            tgt_est / src_est
        } else {
            1.0
        };
        let cycle = mark_cycle_len(marks, idx);
        let tgt_period = cycle * ratio;
        let half_w = psola_half_window(mark.src_period, tgt_period.round() as usize, n);
        if half_w == 0 {
            break;
        }
        // Place the grain so its true (sub-sample) peak lands on synth_pos.
        let out_center = (synth_pos - (mark.pos_f - mark.pos as f64)).round() as isize;
        let inv_half = 1.0 / half_w as f32;
        for k in -(half_w as isize)..=(half_w as isize) {
            let in_i = mark.pos as isize + k;
            let out_i = out_center + k;
            if in_i < 0 || in_i as usize >= n || out_i < 0 || out_i as usize >= n {
                continue;
            }
            if !voiced[in_i as usize] || !voiced[out_i as usize] {
                continue;
            }
            let phase = std::f32::consts::PI * (k as f32) * inv_half;
            let w = 0.5 * (1.0 + phase.cos());
            sum[out_i as usize] += samples[in_i as usize] * w;
            weight[out_i as usize] += w;
        }
        synth_pos += tgt_period.max(1.0);
    }
}

/// Measured cycle length starting at mark `idx` (the gap to the next mark;
/// the previous gap at the end of a chain). With no shift, stepping by it
/// lands every epoch back on its own mark and the output equals the input.
/// If the chain slipped (gap far from the tracked period) fall back to the
/// tracked period so one bad mark cannot throw the pitch.
fn mark_cycle_len(marks: &[AnalysisMark], idx: usize) -> f64 {
    let est = marks[idx].src_period.max(1) as f64;
    let gap = |a: usize, b: usize| -> Option<f64> {
        let g = marks[b].pos_f - marks[a].pos_f;
        ((g - est).abs() <= est * 0.2).then_some(g)
    };
    let next = (idx + 1 < marks.len()).then(|| gap(idx, idx + 1)).flatten();
    let prev = (idx > 0).then(|| gap(idx - 1, idx)).flatten();
    next.or(prev).unwrap_or(est)
}

/// Mix PSOLA back into the original.
///
/// The choice is made once per voiced segment. A segment that needs any
/// real correction is PSOLA from its first sample to its last, so the
/// output never toggles between the original and the shifted copy inside
/// a note. Toggling crossfades two waveforms of different pitch; for a
/// few milliseconds they cancel, and the note pumps in level. The only
/// crossfade is a 10 ms fade at the segment's own edges, where the voice
/// is starting or ending anyway. Segments that need under 3¢ stay
/// untouched.
fn psola_mix(
    samples: &[f32],
    sample_rate: f32,
    voiced: &[bool],
    shift_cents_at: &[f32],
    sum: &[f32],
    weight: &[f32],
) -> Vec<f32> {
    let n = samples.len();
    let mut out = samples.to_vec();
    let edge = ((sample_rate as f64) * 0.010).round().max(1.0) as usize;
    let mut i = 0usize;
    while i < n {
        if !voiced[i] {
            i += 1;
            continue;
        }
        let start = i;
        while i < n && voiced[i] {
            i += 1;
        }
        let end = i;
        let max_shift = shift_cents_at[start..end]
            .iter()
            .fold(0.0f32, |m, &c| m.max(c));
        if max_shift < 3.0 {
            continue;
        }
        let fade = edge.min((end - start) / 2).max(1);
        for j in start..end {
            let w = weight[j];
            if w <= 1e-3 {
                continue;
            }
            let y = sum[j] / w;
            if !y.is_finite() {
                continue;
            }
            let from_edge = (j - start).min(end - 1 - j);
            let env = if from_edge >= fade {
                1.0
            } else {
                let x = (from_edge as f32 + 0.5) / fade as f32;
                0.5 - 0.5 * (std::f32::consts::PI * x).cos()
            };
            // A grain tail with almost no overlap is a hole, not signal.
            // Lean on the original there instead of dividing it up.
            let g = env * (w / 0.3).min(1.0);
            out[j] = soft_limit(y) * g + samples[j] * (1.0 - g);
        }
    }
    out
}

/// Unity below 0.9, rounds into ±1.0 above. The WAV writer hard-clips,
/// and a sheared peak is what a torn note sounds like.
fn soft_limit(x: f32) -> f32 {
    let a = x.abs();
    if a <= 0.9 {
        x
    } else {
        x.signum() * (0.9 + 0.1 * ((a - 0.9) / 0.1).tanh())
    }
}

// ─── WAV encoder (pure Rust, no extra deps) ───────────────────────────────────

fn encode_wav(samples: &[f32], sample_rate: u32) -> Vec<u8> {
    let data_size = (samples.len() * 2) as u32;
    let mut wav = Vec::with_capacity(44 + samples.len() * 2);
    // RIFF header
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    // fmt chunk
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());   // chunk size
    wav.extend_from_slice(&1u16.to_le_bytes());    // PCM
    wav.extend_from_slice(&1u16.to_le_bytes());    // mono
    wav.extend_from_slice(&sample_rate.to_le_bytes());
    wav.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // byte rate
    wav.extend_from_slice(&2u16.to_le_bytes());    // block align
    wav.extend_from_slice(&16u16.to_le_bytes());   // bits per sample
    // data chunk
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for &s in samples {
        wav.extend_from_slice(&((s.clamp(-1.0, 1.0) * 32767.0) as i16).to_le_bytes());
    }
    wav
}


fn read_pcm(path: &str) -> Vec<f32> {
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(&bytes[36..40], b"data");
    assert_eq!(u16::from_le_bytes([bytes[22], bytes[23]]), 1);
    assert_eq!(u32::from_le_bytes(bytes[24..28].try_into().unwrap()), 48000);
    bytes[44..].chunks_exact(2)
        .map(|b| i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0).collect()
}
fn clean_track(mono: &[f32], sample_rate: f32) -> Vec<f64> {
    let (analysis, sr, frame, hop, _) = analysis_grid(mono, sample_rate);
    let mut pitches = yin_pitch_sequence(&analysis, sr, frame, hop, 0.0, 1.0).unwrap();
    remove_octave_outliers(&mut pitches);
    fold_octave_runs(&mut pitches);
    median_smooth(&pitches)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ch0 = read_pcm(&args[1]);
    let sample_rate = 48000.0f32;
    let source_pitches = clean_track(&ch0, sample_rate);
    let n_frames = source_pitches.len();
    let (_, _, _, _, hop_size) = analysis_grid(&ch0, sample_rate);
    let scale_midi = scale_midi_set(9, &[0, 2, 3, 5, 7, 8, 10]); // A natural minor
    let strength = 1.0f32;
    let retune_speed_pct: u32 = args[2].parse().unwrap();
    let tolerance_cents = 0.0f64;
    // BEGIN VERBATIM PIPELINE STEPS 5–7
    let mut src_midi: Vec<f64> = vec![0.0; n_frames];
    let mut voiced: Vec<bool> = vec![false; n_frames];
    for (i, &f) in source_pitches.iter().enumerate() {
        if f > 0.0 {
            src_midi[i] = freq_to_midi(f);
            voiced[i] = true;
        }
    }

    // 6. Retune-speed decomposition:
    //      midi_src(t) = midi_intent(t) + vib(t)
    //    The slow `intent` is what we apply scale-snapping to — vibrato and
    //    short ornaments live in `vib` and are added back verbatim on top of
    //    the corrected centre so they're never flattened by the snap step.
    let hop_sec = hop_size as f64 / sample_rate as f64;
    let alpha = retune_alpha(retune_speed_pct, hop_sec);
    let intent_midi = ema_zero_phase_midi(&src_midi, &voiced, alpha);

    // 7. Target pitch per frame. Scale-snap applies only to `intent`;
    //    vibrato (`src - intent`) is added back, so Strength moves the
    //    center and Retune Speed decides how much of the motion counts as
    //    vibrato. Tolerance is a dead zone around the held scale note:
    //    small errors stay, larger ones are pulled. The held note does not
    //    flip until the intent is clearly closer to another scale tone.
    let tol_main = tolerance_cents.max(0.0);
    let tol_falloff = 20.0_f64;
    let mut held_note: Option<f64> = None;
    let mut target_pitches: Vec<f64> = Vec::with_capacity(n_frames);
    for i in 0..n_frames {
        if !voiced[i] || scale_midi.is_empty() {
            held_note = None;
            target_pitches.push(source_pitches[i]);
            continue;
        }
        let intent = intent_midi[i];
        let vib = src_midi[i] - intent;

        let scale_target = nearest_scale_midi_sticky(
            intent,
            &scale_midi,
            held_note,
            SCALE_NOTE_HOLD_CENTS,
        );
        held_note = Some(scale_target);
        let err_cents = ((intent - scale_target).abs() * 100.0).max(0.0);
        let gate = tolerance_gate(err_cents, tol_main, tol_falloff);
        let eff_strength = (strength as f64) * gate;

        let snapped_intent = intent + (scale_target - intent) * eff_strength;
        let target_midi = snapped_intent + vib;
        target_pitches.push(midi_to_freq(target_midi));
    }
    // Remove single-frame spikes in the target contour.  The `vib` term can
    // inherit a brief outlier that survived the source median filter; a 3-tap
    // median here prevents those from creating PSOLA grain-step discontinuities
    // (audible as clicks or a single pitch point jumping in the comparison plot).
    //
    // Guard: re-zero frames where the source is unvoiced.  The 3-tap median
    // bleeds a voiced pitch value one frame past a voiced segment boundary.
    // Without this guard, those bleed frames register as voiced in the pitch
    // comparison visualizer — causing silence and breath sounds to appear as
    // pitched notes in the history plot.
    let target_pitches = {
        let smoothed = median_smooth(&target_pitches);
        smoothed
            .into_iter()
            .enumerate()
            .map(|(i, v)| if source_pitches[i] <= 0.0 { 0.0 } else { v })
            .collect::<Vec<f64>>()
    };
    // A semitone snap in one hop clicks. ~1100¢/s is a semitone in about
    // 110 ms: still a correction, slow enough that the grain spacing glides.
    let target_pitches = limit_pitch_shift_slew(&source_pitches, &target_pitches, hop_sec, 1100.0);
    // END VERBATIM PIPELINE STEPS 5–7
    let corrected = td_psola(&ch0, sample_rate, &source_pitches, &target_pitches, hop_size).unwrap();
    std::fs::write(&args[3], encode_wav(&corrected, 48000)).unwrap();
    // Measure the downloadable PCM16 output, including its encoding quantization.
    let heard = clean_track(&read_pcm(&args[3]), sample_rate);
    for i in 0..n_frames {
        println!("{:.9}\t{:.9}\t{:.9}\t{:.9}\t{:.9}",
            i as f64 * hop_sec, source_pitches[i], intent_midi[i], target_pitches[i], heard[i]);
    }
}
