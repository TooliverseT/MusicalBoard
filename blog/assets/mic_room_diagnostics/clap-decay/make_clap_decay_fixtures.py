"""Reproduce B1 clap-decay failure modes with the checkout's compute_rt60.

Run: python3 scripts/make_clap_decay_fixtures.py
Requires Python, NumPy, Matplotlib, and rustc.
The measurements use idealized RMS-level curves, not microphone captures.
WAVs are separate listening illustrations; they do not produce the table.
"""
from __future__ import annotations

import csv
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile
import wave

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

DURATION_MS = 3000
INTERVALS_MS = (10, 25, 50, 100)
CASES = (
    ("strong-low-floor", "hard_floor", -10, -70, 300),
    ("strong-quiet-floor", "hard_floor", -10, -50, 300),
    ("weak-quiet-floor", "hard_floor", -30, -50, 300),
    ("weak-power-floor", "power_sum", -30, -50, 300),
    ("borderline-power-floor", "power_sum", -20, -45, 300),
    ("clean-500ms", "hard_floor", -10, -70, 500),
    ("power-500ms", "power_sum", -10, -50, 500),
    ("two-slope", "two_slope", -10, -70, None),
)
RUNNER = r'''
fn main() {
    // CSV input: case_id,time_ms,rms_db, grouped by case_id.
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut id = String::new();
    let mut samples = Vec::new();
    for line in input.lines().skip(1) {
        let cells: Vec<&str> = line.split(',').collect();
        assert_eq!(cells.len(), 3);
        if !id.is_empty() && cells[0] != id {
            println!("{},{}", id, compute_rt60(&samples));
            samples.clear();
        }
        id = cells[0].to_string();
        let time: f64 = cells[1].parse().unwrap();
        let db: f64 = cells[2].parse().unwrap();
        assert!(time.is_finite() && db.is_finite());
        samples.push((time, db));
    }
    if !id.is_empty() { println!("{},{}", id, compute_rt60(&samples)); }
}
'''


def level_at(age_ms, model, peak_db, floor_db, t60_ms):
    age_ms = np.asarray(age_ms, dtype=float)
    if model == "two_slope":
        # Early decay: 12 dB in 50 ms. Late decay: 60 dB per 900 ms.
        decay = peak_db - np.where(age_ms <= 50, .24*age_ms,
                                   12 + (age_ms-50)*60/900)
    else:
        decay = peak_db - 60*age_ms/t60_ms
    if model == "power_sum":
        # Idealized sum of uncorrelated decaying and stationary powers.
        return 10*np.log10(10**(decay/10) + 10**(floor_db/10))
    return np.maximum(decay, floor_db)


def crossings(samples):
    # Supplementary explanation of the native result; not the estimator.
    peak_index = max(range(len(samples)), key=lambda i: samples[i][1])
    peak_t, peak_db = samples[peak_index]
    after = samples[peak_index:]
    t5 = next((t-peak_t for t,db in after if db <= peak_db-5), None)
    t25 = next((t-peak_t for t,db in after if db <= peak_db-25), None)
    drop = max(peak_db-after[-1][1], 1)
    elapsed = max(after[-1][0]-peak_t, 1)
    return {"captured_peak_dbfs": peak_db, "t5_ms": t5, "t25_ms": t25,
            "branch": "T20 crossings" if t5 is not None and t25 is not None else "fallback",
            "last_point_drop_db": drop, "last_point_elapsed_ms": elapsed,
            "uncapped_fallback_ms": elapsed*60/drop if t5 is None or t25 is None else None}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def listening_wav(path, peak_db):
    # A decaying 3 kHz tone + steady 6 kHz floor, with a separate 1 ms trigger pulse.
    # Orthogonal carriers illustrate power addition without random-seed differences.
    rate = 48000
    time = np.arange(rate*4)/rate
    age = time-.5
    decay_rms = np.where(age >= 0, 10**(peak_db/20)*10**(-3*np.maximum(age,0)/.3), 0)
    signal = np.sqrt(2)*decay_rms*np.sin(2*np.pi*3000*time)
    signal += np.sqrt(2)*10**(-50/20)*np.sin(2*np.pi*6000*time)
    pulse_start = int(.5*rate)
    signal[pulse_start:pulse_start+48] += .20
    fade = np.minimum(1, np.minimum(np.arange(len(time)), np.arange(len(time))[::-1])/960)
    pcm = np.rint(signal*fade*32767).astype('<i2')
    with wave.open(str(path),'wb') as wav:
        wav.setparams((1,2,rate,len(pcm),'NONE','not compressed'))
        wav.writeframes(pcm.tobytes())
    return {"file": path.name, "sha256": sha(path), "sample_rate": rate,
            "duration_s": 4, "tail_onset_s": .5, "decaying_component_initial_rms_dbfs": peak_db,
            "stationary_component_rms_dbfs": -50, "component_t60_ms": 300,
            "trigger_pulse_amplitude": .20, "trigger_pulse_duration_ms": 1,
            "peak_dbfs": float(20*np.log10(np.max(np.abs(pcm.astype(float)))/32768)),
            "purpose": "Listening illustration only; not an input to the level-curve simulation."}


def plot_style(axes):
    for ax in axes:
        ax.spines[["top","right"]].set_visible(False)
        ax.grid(alpha=.15)
        ax.set_axisbelow(True)


def main():
    root = Path(__file__).resolve().parents[1]
    out = root/"blog/assets/mic_room_diagnostics/clap-decay"
    out.mkdir(parents=True, exist_ok=True)
    source = (root/"src/tools/mic_diagnostics.rs").read_text()
    start = source.index("fn compute_rt60(")
    end = source.index("\n// ---------------------------------------------------------------------------",start)
    function = source[start:end].rstrip()+"\n"
    constants = {name: float(re.search(r'const '+name+r':[^=]+=[ \t]*([\d.\-]+)',source)[1])
                 for name in ("RT60_SAMPLE_MS","RT60_DURATION_MS","NOISE_FLOOR_THRESHOLD_DB","CLAP_ATTACK_THRESHOLD")}
    assert constants['RT60_SAMPLE_MS']==50 and constants['RT60_DURATION_MS']==DURATION_MS
    runner = function+RUNNER
    (out/'decay-reproduction.rs').write_text(runner)
    rows = []
    csv_path = out/'level-curves.csv'
    with csv_path.open('w',newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['case_id','time_ms','rms_db'])
        for name,model,peak,floor,t60 in CASES:
            for hop in INTERVALS_MS:
                for offset in (0,hop/2):
                    times = np.arange(0,DURATION_MS,hop,dtype=float)
                    levels = level_at(times+offset,model,peak,floor,t60)
                    samples = list(zip(times.tolist(),levels.tolist()))
                    case_id = f'{name}-hop{hop}-offset{offset:g}'
                    writer.writerows((case_id,t,format(db,'.17g')) for t,db in samples)
                    rows.append({'case_id':case_id,'scenario':name,'model':model,
                                 'component_peak_rms_dbfs':peak,'floor_dbfs':floor,
                                 'component_t60_ms':t60,'late_slope_t60_ms':900 if model=='two_slope' else None,
                                 'sample_interval_ms':hop,'tail_age_at_first_sample_ms':offset,
                                 'sample_count':len(samples),
                                 'noise_level_meets_step1_threshold':floor <= constants['NOISE_FLOOR_THRESHOLD_DB'],
                                 **crossings(samples)})
    with tempfile.TemporaryDirectory() as directory:
        tmp=Path(directory)
        (tmp/'decay.rs').write_text(runner)
        executable=tmp/'decay'
        subprocess.run(['rustc','--edition=2021','-O',str(tmp/'decay.rs'),'-o',str(executable)],check=True)
        results=subprocess.check_output([str(executable),str(csv_path)],text=True)
    native=dict(line.split(',') for line in results.splitlines())
    assert len(native)==len(rows)
    for row in rows:
        value=float(native[row['case_id']])
        assert np.isfinite(value) and 0 <= value <= 8000
        row.update({'native_decay_ms':value,'space_value_meets_step2_threshold':value < 500})
        if row['sample_interval_ms']==50:
            print(row['case_id'],row['branch'],value,flush=True)
    wavs=[listening_wav(out/'strong-tail-quiet-floor.wav',-10),
          listening_wav(out/'weak-tail-quiet-floor.wav',-30)]
    result={'method':'Native compute_rt60 extracted verbatim, fed idealized RMS-level curves; not browser or room measurements.',
            'curve_models':{'hard_floor':'max(component_dB, floor_dB)',
                            'power_sum':'10*log10(10^(component_dB/10)+10^(floor_dB/10))',
                            'two_slope':'12 dB in first 50 ms, then 60 dB per 900 ms; hard floor'},
            'timing':'Uniform synthetic sample timestamps from 0 to less than 3000 ms. Offset is tail age at the first sample, not a reconstruction of browser timers.',
            'omitted':'Clap detection, PCM snapshot averaging, device/browser processing, timer jitter, real room response.',
            'native_function_sha256':hashlib.sha256(function.encode()).hexdigest(),
            'runner_sha256':sha(out/'decay-reproduction.rs'),'generator_sha256':sha(Path(__file__)),
            'csv_sha256':sha(csv_path),'rustc':subprocess.check_output(['rustc','--version'],text=True).strip(),
            'numpy':np.__version__,'matplotlib':matplotlib.__version__,
            'source_constants':constants,'cases':rows,'listening_wavs':wavs}
    (out/'results.json').write_text(json.dumps(result,indent=2)+'\n')
    # Downloadable generator: in a checkout save under scripts/ before executing.
    (out/'make_clap_decay_fixtures.py').write_bytes(Path(__file__).read_bytes())
    plt.rcParams.update({'font.family':'JetBrains Mono','font.size':9})
    colors=['#667eea','#242a42','#aa5577']
    fig,axes=plt.subplots(2,1,figsize=(9,6),constrained_layout=True,sharex=True)
    fine=np.linspace(0,400,2000)
    for ax,peak in zip(axes,(-10,-30)):
        ax.plot(fine,level_at(fine,'hard_floor',peak,-50,300),color=colors[0],linewidth=2)
        times=np.arange(0,401,50)
        ax.plot(times,level_at(times,'hard_floor',peak,-50,300),'o',color=colors[0],markersize=4)
        ax.axhline(-50,color=colors[1],linewidth=1,label='Noise floor: -50 dBFS')
        ax.axhline(peak-25,color=colors[2],linestyle='--',label='Required 25 dB drop')
        ax.set(ylabel='Modeled RMS (dBFS)',ylim=(-62,-5))
        ax.set_title(f'Captured RMS peak: {peak} dBFS; gap: {peak+50} dB',
                     loc='left', fontsize=9, pad=8)
        ax.legend(loc='lower left',fontsize=8,ncol=2)
    axes[1].set(xlabel='Time from first sample (ms)')
    plot_style(axes)
    fig.savefig(out/'quiet-floor-clap-margin.png',dpi=160,facecolor='white')
    plt.close(fig)
    fig,axes=plt.subplots(1,2,figsize=(9,4),constrained_layout=True)
    actual=[]
    for hop in INTERVALS_MS:
        row=next(r for r in rows if r['scenario']=='clean-500ms' and r['sample_interval_ms']==hop and r['tail_age_at_first_sample_ms']==0)
        actual.append(row['native_decay_ms'])
    axes[0].bar([str(h) for h in INTERVALS_MS],actual,color=colors[0],width=.55)
    axes[0].axhline(500,color=colors[1],linestyle='--',label='Model: 500 ms')
    axes[0].set(xlabel='Level-sample interval (ms)',ylabel='Returned decay (ms)',ylim=(0,700))
    axes[0].legend(loc='upper left',fontsize=8)
    for i,value in enumerate(actual):
        axes[0].text(i,value-45,f'{value:g}',ha='center',va='top',fontsize=9)
    for offset,color in zip((0,25),colors[:2]):
        times=np.arange(0,601,50)
        axes[1].plot(times,level_at(times+offset,'two_slope',-10,-70,None),'o-',
                     color=color,markersize=3,label=f'First sample at tail age {offset} ms')
    axes[1].set(xlabel='Time from first sample (ms)',ylabel='Two-slope RMS (dBFS)')
    axes[1].legend(loc='lower left',fontsize=7)
    plot_style(axes)
    fig.savefig(out/'sampling-and-two-slope.png',dpi=160,facecolor='white')
    plt.close(fig)


if __name__=='__main__':
    main()
