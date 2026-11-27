"""B2: sweep the current Rust spectrum floor/stats over controlled FFT bins.

Run: python3 scripts/make_spectrum_floor_fixtures.py
Requires NumPy, Matplotlib and rustc. WAVs are listening illustrations only;
the numerical tables use the supplied synthetic FFT-bin CSV, not WAV analysis.
"""
from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import wave

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "blog/assets/vocal_spectrum/floor-sweep"
RATE = 48000
BINS = 1024
PERCENTS = (0, 25, 50, 75, 100)


def extract(source, marker):
    """Copy a complete brace-delimited item without rewriting its calculations."""
    start = source.index(marker)
    opening = source.index("{", start)
    depth = 1
    end = opening + 1
    while depth:
        depth += (source[end] == "{") - (source[end] == "}")
        end += 1
    return source[start:end]


RUNNER = r'''
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
'''


def reference(spectrum, floor, lo, hi):
    frequencies = (np.arange(BINS) + .5) * RATE / 2 / BINS
    keep = (frequencies >= lo) & (frequencies <= hi) & (spectrum >= floor)
    db = spectrum[keep]
    if not len(db):
        return 0, None, None
    amplitudes = 10.0 ** (db.astype(float) / 20)
    return len(db), float(np.sum(frequencies[keep] * amplitudes) / amplitudes.sum()), \
        float(10 * np.log10(np.mean(amplitudes ** 2)))


def listening_wav(path, weak_frequency):
    time = np.arange(RATE * 3) / RATE
    signal = .2 * np.sin(2 * np.pi * 1000 * time)
    signal += .012 * np.sin(2 * np.pi * weak_frequency * time)
    envelope = np.minimum(1, np.minimum(time / .02, (time[-1] - time) / .02))
    pcm = np.rint(signal * envelope * 32767).astype('<i2')
    with wave.open(str(path), 'wb') as output:
        output.setnchannels(1)
        output.setsampwidth(2)
        output.setframerate(RATE)
        output.writeframes(pcm.tobytes())
    return {'filename': path.name, 'sample_rate': RATE, 'duration_s': 3,
            'sample_width_bits': 16, 'strong_tone_hz': 1000,
            'strong_peak_amplitude': .2, 'weak_tone_hz': weak_frequency,
            'weak_peak_amplitude': .012, 'fade_s': .02,
            'role': 'Listening illustration only; not the input to the numerical table.'}


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    visualizer = (ROOT / 'src/tools/amplitude_visualizer.rs').read_text()
    pitch = (ROOT / 'src/tools/pitch_plot.rs').read_text()
    parts = ['#[derive(Default, Clone, Copy)]\n' + extract(visualizer, 'struct SpectrumStats {')]
    parts += [extract(visualizer, 'fn sensitivity_to_floor_db('),
              extract(visualizer, 'fn analyze_spectrum(')]
    parts += [extract(pitch, 'pub fn ' + name + '(') for name in
              ('sensitivity_slider_unit', 'sensitivity_percent', 'sensitivity_from_percent')]
    runner = OUT / 'floor-reproduction.rs'
    runner.write_text('\n\n'.join(parts) + '\n' + RUNNER)
    cases = {}
    for name, indices in [('weak-high', range(200, 220)), ('weak-low', range(4, 24))]:
        spectrum = np.full(BINS, -90, dtype=np.float32)
        spectrum[42], spectrum[85] = -20, -26
        spectrum[list(indices)] = -45
        spectrum[300:320] = -65
        cases[name] = spectrum
    empty = np.full(BINS, -90, dtype=np.float32)
    cases['below-floor'] = empty
    boundary = empty.copy()
    boundary[42] = -30
    cases['at-floor'] = boundary
    csv_path = OUT / 'spectrum-inputs.csv'
    inputs = []
    with csv_path.open('w', newline='') as output:
        writer = csv.writer(output)
        writer.writerow(['case_id', 'percent', 'sample_rate', 'band_lo', 'band_hi', 'spectrum_db'])
        for name, spectrum in cases.items():
            bands = [(80, 16000), (80, 4000)] if name == 'weak-high' else [(80, 16000)]
            for lo, hi in bands:
                for percent in PERCENTS:
                    writer.writerow([name, percent, RATE, lo, hi, ';'.join(map(str, spectrum))])
                    inputs.append((spectrum, lo, hi))
    with tempfile.TemporaryDirectory() as directory:
        executable = Path(directory) / 'floor-sweep'
        subprocess.run(['rustc', '--edition=2021', '-O', str(runner), '-o', str(executable)], check=True)
        output = subprocess.check_output([str(executable), str(csv_path)], text=True)
    (OUT / 'results.csv').write_text(output)
    rows = list(csv.DictReader(output.splitlines()))
    assert len(rows) == len(inputs) == 25
    for row, (spectrum, lo, hi) in zip(rows, inputs):
        kept, centroid, rms = reference(spectrum, float(row['floor_db']), lo, hi)
        assert kept == int(row['kept'])
        assert abs(float(row['floor_db']) - (-30 - .45 * float(row['percent']))) < 1e-5
        if kept:
            assert abs(centroid - float(row['centroid_hz'])) < 1e-5
            assert abs(rms - float(row['rms_db'])) < 1e-5
            assert row['has'] == 'true'
        else:
            assert row['has'] == 'false'
    wavs = [listening_wav(OUT / 'strong-mid-weak-high.wav', 6000),
            listening_wav(OUT / 'strong-mid-weak-low.wav', 200)]
    sources = [ROOT / 'src/tools/amplitude_visualizer.rs', ROOT / 'src/tools/pitch_plot.rs', runner]
    metadata = {
        'method': 'Unmodified Rust floor mapping and analyze_spectrum, extracted from source; synthetic FFT bins, not browser measurements.',
        'sample_rate': RATE, 'fft_bin_count': BINS, 'implied_fft_size': BINS * 2,
        'frequency_label_rule': '(i + 0.5) * sample_rate / (2 * bin_count)',
        'percent_settings': PERCENTS, 'background_bins_db': -90,
        'controlled_bins': {'strong': {'42': -20, '85': -26},
                            'weak_high': {'200..219': -45}, 'weak_low': {'4..23': -45},
                            'very_weak_high_both_cases': {'300..319': -65}},
        'rustc_version': subprocess.check_output(['rustc', '--version'], text=True).strip(),
        'source_sha256': {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources},
        'verification': '25 native results checked against independent NumPy formulas; floor mapping, retained count, centroid, RMS, empty and exact-boundary cases.',
        'wavs': wavs, 'results': rows,
    }
    (OUT / 'results.json').write_text(json.dumps(metadata, indent=2) + '\n')
    plt.rcParams.update({'font.family': 'DejaVu Sans', 'font.size': 11})
    fig, axes = plt.subplots(1, 2, figsize=(10, 3.8), constrained_layout=True)
    for name, label, color in [('weak-high', 'Weak bins above main tones', '#667eea'),
                               ('weak-low', 'Weak bins below main tones', '#6b7280')]:
        subset = [r for r in rows if r['case_id'] == name and float(r['band_hi']) == 16000]
        x = [float(r['percent']) for r in subset]
        for ax, metric in zip(axes, ['rms_db', 'centroid_hz']):
            ax.plot(x, [float(r[metric]) for r in subset], 'o-', color=color, label=label)
    for ax, ylabel in zip(axes, ['Retained-bin RMS (dB)', 'Amplitude-weighted centroid (Hz)']):
        ax.set(xlabel='Spectrum floor slider (%)', ylabel=ylabel, xticks=PERCENTS)
        ax.grid(alpha=.18)
        ax.spines[['top', 'right']].set_visible(False)
    axes[1].legend(fontsize=9)
    fig.savefig(OUT / 'floor-rms-centroid.png', dpi=170, facecolor='white')
    plt.close(fig)
    shutil.copyfile(Path(__file__), OUT / Path(__file__).name)
    print('Verified 25 native calculation cases. Outputs:', OUT)
    for row in rows:
        if row['case_id'] in ('weak-high', 'weak-low') and row['percent'] in ('0', '50', '100'):
            print({k: row[k] for k in ('case_id', 'percent', 'band_hi', 'kept', 'centroid_hz', 'rms_db')})


if __name__ == '__main__':
    main()
