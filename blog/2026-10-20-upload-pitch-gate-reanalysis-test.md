---
title: "Does Pitch Gate Reanalyze an Uploaded Take? A Nine-Import Test"
date: 20 October 2026
tags: ["Vocal Pitch Monitor"]
excerpt: "We imported three generated signal levels at Pitch gate 0%, 50%, and 100%. The stored pitch data stayed identical at each level, showing the difference between upload analysis and what the graph displays."
---

If a quiet uploaded take has a sparse pitch graph, turning **Pitch gate** up can feel like asking the browser to analyze the file more carefully. That is not how MusicalBoard’s upload path works today. Analysis runs once at import with fixed detection sensitivity; Pitch gate then controls which stored pitch samples appear on the graph.

We checked this with **nine imports**: three generated signal levels, each imported at 0%, 50%, and 100%. For each level, the **stored pitch data after upload** was identical across all three slider positions. The stronger two signals produced pitch points; the extremely quiet signal produced none. This is an experiment on synthetic files, not a test of human hearing or a measurement of room noise.

## The controlled inputs

Each source is a three-second, 48 kHz, 16-bit mono WAV with a 220 Hz fundamental and second and third harmonics. Each has 20 ms fades and no background noise. Only the nominal peak amplitude before 16-bit rounding changes:

| Source | Nominal peak before rounding | Approximate peak dBFS |
|---|---:|---:|
| Strong tone | 0.4 | −8.0 dBFS |
| Quiet tone | 0.02 | −34.0 dBFS |
| Extremely quiet tone | 0.00005 | −86.0 dBFS |

The dB column uses `20 × log10(peak)`. It describes the requested peak before rounding, not its RMS, microphone input gain, or sound pressure in a room. At the lowest level, 16-bit quantization has a substantial effect; the waveform has only a few available sample values (the saved peak rounds to about −84.3 dBFS). This is deliberately an extreme input, not a normal singing level.

The run used **Chromium on Linux** on 16 October 2026. Every import used a fresh browser session, with the visible Pitch gate slider set before uploading.

## What we observed

After each upload, we recorded how many pitch samples the tool retained for the graph. “Zero entries” below means no pitch samples were stored for that import, not that the file had no audio or that nothing could be heard on playback.

| Generated peak | Gate 0%: stored entries | Gate 50% | Gate 100% | Same history at all three settings? |
|---|---:|---:|---:|---|
| 0.4 | 119 | 119 | 119 | Yes |
| 0.02 | 119 | 119 | 119 | Yes |
| 0.00005 | 0 | 0 | 0 | Yes |

For the first two signals, the typical stored pitch was about **220.5 Hz**, approximately +3.9 cents relative to the generated 220 Hz. Browser pitch tracking uses short time windows, so a perfect 220.000 Hz readout is not expected even on a synthetic tone.

Within each signal level, all three gate settings produced the same stored pitch timeline—not just the same number of dots on screen.

## Why moving the slider can still change the graph

Upload analysis runs once when the file is imported, using fixed detection sensitivity that does not follow the Pitch gate slider. Pitch gate then controls how strong a stored pitch sample must be before the graph shows it, together with the Low–High note window.

Raising the slider can reveal a pitch sample that was already stored but hidden at a lower gate. It does not add new analysis that import missed. Our extremely quiet source makes that limit concrete: changing the slider before import still produced no stored pitch data at any tested setting.

This experiment checks the **stored analysis**, not a claim that the graph must look identical for every source. Different display thresholds can produce different visible results from the same stored data. Live microphone analysis is also a separate case: its detection thresholds respond to Pitch gate while new frames are captured. Do not infer live-microphone behavior from these imports.

## A practical troubleshooting order

For an uploaded practice take:

1. Listen to the file first. Check that the intended voice is actually present and that the file is not nearly silent.
2. Put the intended register inside **Low–High**.
3. Adjust Pitch gate to examine what was already detected. If more dots appear, check them against the reference sound rather than assuming more dots mean more accurate analysis.
4. If the line follows accompaniment or jumps octaves, try a clean voice-only take. A display slider cannot identify which singer or instrument you intended.
5. Keep the original file. If you raise its level externally, remember that this also raises its noise and cannot undo quantization or clipping.

For a new microphone recording, improve capture conditions instead: comfortable input level, a stable mic position, and headphones to keep backing out of the microphone. We did not measure a physical microphone or distance in this test, so there is no recommendation here that a particular centimeter value will fix your room.

## Repeat the test and check its limits

The sources are available as [strong](/blog/assets/vocal_pitch_monitor/level-0.4.wav), [quiet](/blog/assets/vocal_pitch_monitor/level-0.02.wav), and [extremely quiet](/blog/assets/vocal_pitch_monitor/level-5e-05.wav) WAVs. Import the same file at each setting in fresh sessions and compare whether the graph changes only in visibility or in the underlying stored analysis.

A clean stationary tone is easier to detect than a breathy vowel, transient consonant, or full mix. The results therefore describe how MusicalBoard handled these particular imports on the date above. They do not validate every threshold for every voice, or prove that a blank graph means the source is silent.

For the broader controls, see [How Browser Pitch Detection Works](/blog/2026-05-05-pitch-detection/) and [Vocal Pitch Monitor](/vocal-pitch-monitor/). For preserving the test file while experimenting, see [The Original Download Test](/blog/2026-10-14-original-download-byte-preservation-test/).
