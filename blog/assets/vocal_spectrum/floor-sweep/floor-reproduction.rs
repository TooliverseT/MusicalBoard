#[derive(Default, Clone, Copy)]
struct SpectrumStats {
    peak_hz: f64,
    peak_db: f64,
    centroid_hz: f64,
    rms_db: f64,
    has: bool,
}

fn sensitivity_to_floor_db(sens: f32) -> f64 {
    let t = sensitivity_percent(sens) as f64 / 100.0;
    -30.0 - t * 45.0
}

fn analyze_spectrum(
    spectrum: &[f32],
    sample_rate: f64,
    floor_db: f64,
    band_lo: f64,
    band_hi: f64,
) -> SpectrumStats {
    if spectrum.is_empty() || sample_rate <= 0.0 {
        return SpectrumStats::default();
    }
    let nyquist = sample_rate / 2.0;
    let bin_hz = (nyquist / spectrum.len() as f64).max(1.0);
    let mut peak_db = f64::NEG_INFINITY;
    let mut peak_hz = 0.0;
    let mut sum_lin = 0.0_f64;
    let mut sum_lin_w = 0.0_f64;
    let mut sum_pow = 0.0_f64;
    let mut count: u32 = 0;
    for (i, v) in spectrum.iter().enumerate() {
        let f = (i as f64 + 0.5) * bin_hz;
        if f < band_lo || f > band_hi {
            continue;
        }
        if !v.is_finite() {
            continue;
        }
        let db = (*v as f64).max(-120.0);
        // 노이즈 게이트: floor 미만은 분석에 반영하지 않음(센서티비티가 높을수록 더 많이 받음)
        if db < floor_db {
            continue;
        }
        if db > peak_db {
            peak_db = db;
            peak_hz = f;
        }
        let lin = 10.0_f64.powf(db / 20.0);
        sum_lin += lin;
        sum_lin_w += lin * f;
        sum_pow += lin * lin;
        count += 1;
    }
    if count == 0 || !peak_db.is_finite() {
        return SpectrumStats::default();
    }
    let centroid = if sum_lin > 0.0 { sum_lin_w / sum_lin } else { 0.0 };
    let rms_lin = (sum_pow / count as f64).sqrt().max(1e-12);
    let rms_db = 20.0 * rms_lin.log10();
    SpectrumStats {
        peak_hz,
        peak_db,
        centroid_hz: centroid,
        rms_db,
        has: true,
    }
}

pub fn sensitivity_slider_unit(sensitivity: f32) -> f32 {
    let s = sensitivity.clamp(0.001, 0.1);
    (s - 0.001) / (0.1 - 0.001)
}

pub fn sensitivity_percent(sensitivity: f32) -> f32 {
    sensitivity_slider_unit(sensitivity) * 100.0
}

pub fn sensitivity_from_percent(percent: f32) -> f32 {
    let p = percent.clamp(0.0, 100.0) / 100.0;
    (0.001 + p * 0.099).clamp(0.001, 0.1)
}

fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    println!("case_id,percent,floor_db,band_lo,band_hi,kept,peak_hz,level_db,centroid_hz,rms_db,has");
    for line in input.lines().skip(1) {
        let c: Vec<&str> = line.split(',').collect();
        assert_eq!(c.len(), 6);
        let percent: f32 = c[1].parse().unwrap();
        let sr: f64 = c[2].parse().unwrap();
        let lo: f64 = c[3].parse().unwrap();
        let hi: f64 = c[4].parse().unwrap();
        let spectrum: Vec<f32> = c[5].split(';').map(|s| s.parse().unwrap()).collect();
        let floor = sensitivity_to_floor_db(sensitivity_from_percent(percent));
        let result = analyze_spectrum(&spectrum, sr, floor, lo, hi);
        let step = (sr / 2.0 / spectrum.len() as f64).max(1.0);
        let kept = spectrum.iter().enumerate().filter(|(i, db)| {
            let f = (*i as f64 + 0.5) * step;
            f >= lo && f <= hi && db.is_finite() && (**db as f64).max(-120.0) >= floor
        }).count();
        println!("{},{},{:.8},{},{},{},{:.8},{:.8},{:.8},{:.8},{}",
            c[0], percent, floor, lo, hi, kept, result.peak_hz,
            result.peak_db, result.centroid_hz, result.rms_db, result.has);
    }
}
