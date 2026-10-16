---
title: "How Browser-Based Pitch Detection Works — From Web Audio API to WebAssembly"
date: 05 May 2026
tags: ["Vocal Pitch Monitor"]
excerpt: "A look at how real-time pitch detection works in the browser, from Web Audio API to Rust and WebAssembly. Understanding the technology behind Vocal Pitch Monitor — which runs with nothing to install — helps you get more out of the tool."
---

Open a browser tab, allow microphone access, and the moment you start singing, a note name and pitch graph appear on screen. No app installation required. How does that actually work? This post walks through the technology behind MusicalBoard's [Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/) — [Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API) and [WebAssembly](https://webassembly.org/) — step by step. By the end, you'll be able to answer practical questions like "why do I need to raise the sensitivity?" or "why does it track better in a quiet room?" on your own.

## From microphone to graph: the path a signal travels

When you start singing, the browser goes through several steps to turn a microphone signal into a note name. Broadly speaking:

1. **Microphone access request** — [`getUserMedia({ audio: true })`](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia)
2. **Audio context creation** — spins up the browser's internal audio processing engine
3. **Audio analysis node connection** — pulls waveform data in real time
4. **Pitch detection algorithm** — calculates the fundamental frequency from the waveform
5. **MIDI number conversion and display** — maps the frequency to a note name (e.g. A4 = 440 Hz) and draws it on the graph

The entire process happens inside your device without a single round-trip to a server. You can disconnect from the internet while the tab is open and pitch detection will keep working just fine.

<p class="mb-blog-image">
  <img src="/blog/assets/vocal_pitch_monitor/pitch_detection_flow.png" alt="Browser pitch detection signal flow — from microphone input to note name display">
</p>

<p align="center">
  <em>How a microphone signal gets turned into a note name inside the browser. Nothing is sent to a server.</em>
</p>

## Web Audio API — the browser's built-in audio engine

[Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API) has been a browser standard since W3C began standardizing it in 2013, and today Chrome, Firefox, Safari, and Edge all support it reliably. The core idea is straightforward: **connect nodes together to build an audio graph**.

For pitch detection specifically, you take the microphone stream as a source node and attach an `AnalyserNode` to it. The `AnalyserNode` lets you pull waveform data (`getFloatTimeDomainData`) or frequency spectrum data (`getFloatFrequencyData`) from the current audio buffer in real time. On MusicalBoard's [Vocal Spectrum](https://www.musicalboard.com/audio-spectrum-analyzer/) page, that frequency data is rendered as FFT bars on a logarithmic Hz axis and dBFS scale, which makes vocal-band energy changes easier to read in practice.

That said, `AnalyserNode` alone isn't great at extracting a precise fundamental frequency from a vocal signal. FFT (Fast Fourier Transform) breaks a signal down into frequency bins, but a human voice — rich with harmonics — doesn't sit cleanly in a single bin. That's why you need a dedicated pitch detection algorithm on top of it.

## What "calculating" a pitch actually means — autocorrelation and FFT

There are two common approaches to pitch detection.

**Autocorrelation** works by sliding a copy of the waveform along the time axis and measuring how closely it matches the original at each offset. The lag where the similarity peaks corresponds to the waveform's period, and the inverse of that period is the fundamental frequency. It's intuitive, not too hard to implement, and shows up in a lot of introductory examples. It handles simple sine waves and single instrument notes reasonably well, though accuracy can degrade with background noise or mixed signals.

**Frequency-domain methods** inspect a spectrum and use relationships among harmonics to estimate pitch. YIN and MPM are other period-based approaches; they should not be treated as names for simply choosing the tallest FFT bin. MusicalBoard’s live pitch monitor uses autocorrelation of the waveform. Auto-Tune has its own YIN-family estimator, so the two tools can disagree on difficult input.

## The live analysis loop — roughly 40 updates per second

MusicalBoard requests a live analysis update every **25 milliseconds**, or roughly **40 times per second**. A browser timer sends a message to the page, which reads a 4,096-sample waveform from the `AnalyserNode`. Rust code compiled to WebAssembly then finds autocorrelation peaks and keeps up to three pitch candidates for that slice. The graph consumes that history separately from the audio capture.

This calculation runs on the **main thread**. WebAssembly changes how the computation is executed; it does not move it to another thread automatically. A busy UI, background tab, or slow device can delay analysis updates. The 25 ms interval is a target, not a promise of exact timing or measured microphone-to-screen latency.

[AudioWorklet](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet) is a browser API for custom processing on the audio rendering thread. It is useful in other audio designs, but it is **not the live pitch analysis path used here**. Browser audio capture and playback still have their own internal processing; that does not make the page’s pitch estimator an AudioWorklet processor.

```
Microphone → Web Audio AnalyserNode
                         ↓ float waveform snapshot
                25 ms page timer → Rust/WASM autocorrelation
                                         ↓
                              pitch history → canvas graph
```

## WebAssembly — near-native speed inside the browser

You can absolutely write autocorrelation or FFT calculations in plain JavaScript and have them run. But if you want to use more sophisticated algorithms (YIN, MPM, etc.), track multiple frequencies at once, or make things run smoothly on lower-end devices, you need headroom.

[WebAssembly (WASM)](https://webassembly.org/) is a binary format that runs code compiled from languages such as Rust inside the browser. MusicalBoard uses it to run its signal processing and UI code. Performance depends on the algorithm, browser, and device; there is no site benchmark here establishing a fixed speed advantage over JavaScript. If the graph lags, reduce other work in the tab and check on the device you actually practice with.

In MusicalBoard, the page reads waveform and spectrum snapshots from an `AnalyserNode`. The waveform is passed to the Rust/WASM pitch calculation on the main thread, and its results feed the canvas graph. Audio recording uses the browser’s MediaRecorder separately; recording a file does not require every graph update to run exactly on schedule.

## What the Pitch gate slider actually does

The pitch settings bar uses a **0–100%** slider. Its internal storage value is 0.001–0.1, but you do not enter those decimals in the UI. Higher percentages make detection more receptive; lower percentages demand stronger, clearer input.

Three thresholds change together: the waveform’s minimum RMS level, the relative autocorrelation strength needed to keep a candidate, and the relative strength needed to show a main pitch on the graph. On desktop, the RMS gate spans about 0.10 at 0% to 0.00008 at 100%; mobile input uses a lower effective gate. These are signal-processing thresholds, not calibrated microphone levels.

Raising Pitch gate can reveal soft notes and more weak candidates. It can also admit noise or an octave error. Lower it when the trace is cluttered, and widen or narrow Low–High to the register you are actually practicing. The Low–High window selects the displayed main candidate; it does not change the live autocorrelation lag search.

Changing Pitch gate **does not remove noise from the recording** or change input gain. For a noisy take, improve the room or microphone setup first. The [Vocal Spectrum](https://www.musicalboard.com/audio-spectrum-analyzer/) has a separate **Spectrum floor** slider: the same percentage on the two tools does not mean the same response.

## The logarithmic Y axis — why semitones look evenly spaced

Looking at the pitch graph, the gap between C3 and C4 appears equal to the gap between C4 and C5. In actual frequency terms, C3 is around 130 Hz, C4 around 262 Hz, and C5 around 523 Hz — each octave doubles the frequency. The reason they look equal on screen is that the Y axis uses a **logarithmic scale**.

An octave up is always exactly 2× the frequency. Human hearing also perceives pitch logarithmically, which is why a log-scale Y axis makes musical sense: every semitone takes up the same visual height, and note labels are distributed evenly. The conversion formula between MIDI number and Hz (`440 * 2^((midi-69)/12)`) is the direct mathematical expression of that relationship.

<p class="mb-blog-image">
  <img src="/blog/assets/vocal_pitch_monitor/log_scale_pitch_axis.png" alt="Linear scale vs. logarithmic scale pitch axis comparison">
</p>

<p align="center">
  <em>On a logarithmic Y axis, semitone intervals are visually equal — the graph matches musical intuition.</em>
</p>

## How knowing this changes your practice

A bit of technical background goes a long way in getting better results out of the tool.

**If dots on the graph are sparse or intermittent**, your sensitivity is probably too low, the microphone is too far away, or background noise is masking your signal just enough that the algorithm won't accept it as valid. Try nudging sensitivity up one step, or move a little closer to the mic.

**Leaving “Other pitches” on** shows additional autocorrelation pitch candidates, with up to three candidates stored per slice. These can include octave or subharmonic ambiguities; they are not a direct measurement of separate harmonics or vocal resonances. Turn it off for a simpler main trace. Use Vocal Spectrum for frequency-band energy, and remember that a spectrum peak alone is not a measured formant.

**With "Highlight" enabled**, the last stably detected pitch stays visible as a reference line for a few seconds. That's why it doesn't disappear the instant you take a breath or pass through a consonant — it gives you a visual anchor to come back to.

**You can drag the timeline** to review a specific moment after you've sung through a phrase. Double-clicking returns you to auto-follow mode. All of this state management — the timeline position, your settings, your recording data — lives entirely in your browser's local memory and storage. Nothing leaves your device.

## Practicing with MusicalBoard

[Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/) is the most direct place to see this whole technical stack in action. Start singing and your note name and pitch graph appear in real time; when you're done, you can play back the same timeline and review it. Settings like sensitivity and Y-axis range are saved automatically in your browser so they're waiting for you next time.

If you want spectrum context alongside pitch information, [Vocal Spectrum](https://www.musicalboard.com/audio-spectrum-analyzer/) shares the same microphone pipeline and opens in the same interface. It is especially useful for reading harmonic balance, register-shift patterns, sibilance peaks, and breath-noise bands while you monitor pitch separately in the graph. For scale practice you can run [Vocal Scales](https://www.musicalboard.com/vocal-scales/) alongside it, and [Virtual Piano](https://www.musicalboard.com/virtual-piano/) is there when you need to find a reference pitch.

Technically it's a lot of moving parts. From where you're sitting, it's just one browser tab.

## References

- [Web Audio API — MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
- [AudioWorklet — MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/API/AudioWorklet)
- [Using AudioWorklet — MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Using_AudioWorklet)
- [WebAssembly official site](https://webassembly.org/)