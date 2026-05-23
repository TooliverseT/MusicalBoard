---
title: "How to Set Frequency Range and Sensitivity in a Spectrum Analyzer — Optimizing Vocal Spectrum for Vocal Practice"
date: 23 May 2026
tags: ["Vocal Spectrum"]
excerpt: "A spectrum analyzer only delivers the information you need when you set the right frequency range and sensitivity for your purpose. This guide explains how to adjust Vocal Spectrum's frequency range presets and sensitivity slider to match your vocal practice goals."
---

When you open a spectrum analyzer for the first time, how do you know whether what you see actually reflects your voice accurately? It's easy to leave everything at the default and carry on — but if the frequency range and sensitivity settings don't match your practice purpose, you'll either miss important information or fill the graph with unnecessary noise. This guide explains how to understand and adjust the two core settings of [Vocal Spectrum](https://www.musicalboard.com/audio-spectrum-analyzer/) — frequency range and sensitivity — so you can configure them for any situation.

## What the Frequency Range Setting Does — Choosing Which Band to View

The frequency range setting determines the horizontal span of the graph. Set it to 20 Hz–22 kHz and the entire range of human hearing spreads across the screen. Narrow it to 80 Hz–8000 Hz and only that band is shown, enlarged.

Narrowing the range makes the fine structure of that band more visible. A setting of 80 Hz–1000 Hz, for example, shows only the fundamental frequencies and lower harmonics of the voice in larger detail, making subtle changes in the low register much easier to read. Conversely, a wide range gives a broad view of overall energy distribution but reduces the visible detail in any particular band.

In Vocal Spectrum, frequency range is set using **two combo boxes — a minimum (Min) and a maximum (Max)**. Each one offers a list of preset values to select from. The available values are:

**Available frequency values (Hz):** 20, 40, 60, 80, 125, 200, 250, 500, 750, 1000, 1500, 2000, 3000, 4000, 6000, 8000, 12000, 16000, 22000

The minimum can be set as low as 20 Hz and the maximum as high as 22000 Hz.

## Why 80 Hz–16 kHz Is the Default for Vocals

[Vocal Spectrum's](https://www.musicalboard.com/audio-spectrum-analyzer/) default frequency range is **80 Hz–16 kHz**. There are good reasons this range was chosen as the starting point for vocal practice.

The fundamental frequency (F0) of the human voice spans roughly 80 Hz to 1050 Hz depending on voice type — from the lowest note of a bass (E2, 82 Hz) to the highest note of a soprano (C6, 1047 Hz). The harmonics that define timbre and formants extend far above this, distributed at integer multiples of the fundamental.

Sibilant consonants (S, Z, SH, etc.) and breath noise concentrate their energy primarily in the 4 kHz–12 kHz range. Setting the upper limit at 16 kHz captures this sibilance energy fully. Above 16 kHz, however, most recording environments are dominated by background noise, with almost nothing vocally relevant to observe.

Below 80 Hz, the fundamental frequency of most voices does not reach, and the band is dominated by room noise and microphone handling noise. Setting the lower limit at 80 Hz prevents unnecessary low-frequency noise from crowding the left side of the graph.

For these reasons, 80 Hz–16 kHz provides the most useful information for the widest range of vocal practice situations.

In choral SATB (soprano, alto, tenor, bass) terms, the **fundamental frequency (F0)** shown on the left and center of the spectrum overlaps between voice types but centers in different bands. According to [DPA's guide to singing voices and frequencies](https://www.dpamicrophones.com/dict/singing-voices-and-frequencies/) and [Stanford CCRMA singing acoustics](https://ccrma.stanford.edu/CCRMA/Courses/152/singing.html), typical choral ranges are bass E2 (82 Hz)–E4 (330 Hz), tenor C3 (131 Hz)–C5 (523 Hz), alto F♯3 (185 Hz)–A5 (880 Hz), and soprano C4 (262 Hz)–C6 (1047 Hz). Research on adult male and female F0 distribution is also available in [PMC's formant range profile for singers](https://pmc.ncbi.nlm.nih.gov/articles/PMC5409887/).

## Fundamental Frequency Ranges by Voice Type

Different voice types require attention to different parts of the spectrum. Knowing your voice type's fundamental frequency range clarifies which part of the graph matters most.

**Soprano (approximately 260–1050 Hz).** The highest fundamental frequency among voice types. The primary singing range runs from C4 (261 Hz) to C6 (1047 Hz). Harmonics in the soprano spread into the several-kHz region, and the bright, luminous soprano timbre is closely linked to harmonic energy in the 2–4 kHz band.

**Alto (approximately 190–880 Hz).** Spanning from roughly F#3 (185 Hz) to A5 (880 Hz). The warm, full alto timbre comes from the strength of mid-to-low harmonics. The 250–500 Hz band is central to the characteristic alto sound.

**Tenor (approximately 130–523 Hz).** The highest male voice type, ranging from C3 (131 Hz) to C5 (523 Hz). The clear, ringing head voice quality of the tenor is closely associated with the Singer's Formant in the 2–3 kHz region.

**Bass (approximately 80–329 Hz).** The lowest voice type, covering from E2 (82 Hz) to around E4 (330 Hz). The deep, rich bass timbre is produced when the fundamental and lower harmonics in the 80–200 Hz range are strongly present.

Because fundamental frequency ranges differ by voice type, adjusting the lower limit of the spectrum display can help. A soprano can raise the minimum to 125–200 Hz without losing important information. A bass who wants to examine sub-80 Hz content can lower the minimum to 40 Hz or even 20 Hz.

## The High-Frequency Region (5 kHz and Above) — Sibilance and Breath

Above 5 kHz, the spectrum captures phenomena other than the vocal fundamental and its main harmonics.

**Sibilance.** Fricative consonants such as S, Z, and SH create strong energy peaks in the 5–10 kHz range. When these peaks are too prominent, the result is the sharp, harsh "hissing" quality that stands out in recordings. To check for sibilance, set the upper limit to 16–22 kHz and observe the energy in that band.

**Breath noise.** Breath sounds before and after phonation, as well as breath noise captured by the microphone, appear mainly in the 4–8 kHz range. If energy in this band rises sharply just before or after a sung note begins, it may be worth adjusting the microphone position or distance.

**Air noise and ambient noise.** Above 12 kHz, there is almost no vocal content — the band is dominated by environmental and self-noise from the microphone. Persistent energy at these frequencies suggests that improving the recording environment or lowering the sensitivity setting would help.

## Recommended Frequency Ranges by Purpose

Choosing the optimal range for your practice goal makes the relevant information much easier to read.

**Harmonic analysis — 80–8000 Hz.**
This range captures the fundamental and primary harmonics, including the Singer's Formant (2500–3000 Hz) and formants F1 and F2. It is the first range to reach for in most vocal timbre analysis. Preset: Min 80, Max 8000.

**Sibilance check — 3000–16000 Hz or 3000–22000 Hz.**
This range isolates the band where fricatives and breath noise concentrate, letting you see exactly how high a sibilant consonant peaks and which consonants are most prominent. Preset: Min 3000, Max 16000 or 22000.

**Low-frequency inspection — 20–500 Hz.**
Useful when checking the fundamental frequencies of a bass or low-register alto or countertenor, or when verifying whether proximity effect is boosting low frequencies from the microphone. This range will also show room noise below 80 Hz, so it works best in a quiet environment. Preset: Min 20, Max 500.

**Full-range overview — 20–22000 Hz.**
Covers the entire range of human hearing at once. Useful for a quick check of whether unusual energy is appearing anywhere in the spectrum. Detail in any particular band will be small due to the wide span. Preset: Min 20, Max 22000.

## What the Sensitivity Slider (0–100%) Controls

If the frequency range sets the horizontal axis, the sensitivity slider sets how the vertical axis responds — specifically, how weak a signal still registers on the graph.

Vocal Spectrum sensitivity is set with a **0% to 100%** slider. It uses the same 0–100% scale as [Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/), so matching the same percentage in both tools gives a similar response strength during practice. The default is about **50%**.

**High sensitivity (about 50–100%).** Even very low-level energy appears on the graph. Raise sensitivity when the signal is hard to see while singing quietly in a clean room. As you move the slider up, the bottom of the vertical axis (the display floor) drops to lower dB values, so weak harmonics and breath noise become visible. The drawback is that ambient noise, microphone self-noise, and other unwanted signals also grow prominent, making the actual vocal signal harder to read.

**Low sensitivity (about 0–20%).** Only signals above a certain energy level appear. Lower sensitivity when background or microphone noise is cluttering the graph and you want to focus on the vocal signal. The drawback is that very soft passages or weak signal just before phonation may not register.

**Relationship with Vocal Pitch Monitor sensitivity.** Both tools use 0–100% sensitivity. Find a percentage that works well in the pitch monitor (for example, 40%), then apply the same value in Vocal Spectrum for a fast, consistent setup.

## Practical Adjustment Scenarios

**Quiet room with a quality microphone.**
With minimal background noise, you can set sensitivity in the mid-to-high range (about 30–70%) and still get a clean signal free of noise. Quiet passages for pianissimo analysis are rendered clearly. Start with a frequency range of 80–8000 Hz and adjust as needed.

**Room with air-conditioning, fan, or ventilation noise.**
Background noise competing with the vocal signal makes the graph hard to read. Set sensitivity to the low end (about 5–20%) so only signals of a certain strength appear. When you sing, the vocal signal will appear much more strongly than the background noise; when you stop singing, the graph will be nearly empty.

**Using a laptop's built-in microphone without an external mic.**
Built-in microphones tend to have a higher noise floor. If you raise sensitivity too high, the microphone's own noise fills the graph. Start around 10–30% and find the point where a clear signal appears only when you sing. Setting the lower frequency limit to 80 Hz or 125 Hz also reduces the low-frequency noise typical of built-in microphones.

**Focused sibilance analysis in the high-frequency range.**
With the frequency range set to the sibilance band (3000–16000 Hz), fricative consonants will show concentrated energy bursts. Setting sensitivity to a mid level (about 20–50%) lets you read sibilant peaks clearly while still distinguishing them from background noise.

## Using Vocal Spectrum Alongside Other MusicalBoard Tools

A Vocal Spectrum configured with an optimized frequency range and sensitivity becomes even more powerful in combination with other tools.

Keeping [Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/) open alongside Vocal Spectrum lets you monitor pitch accuracy and spectral content simultaneously. Because both tools use the same sensitivity scale, maintaining consistent settings across the two is straightforward.

When you play back a recording made with [Singing Recorder](https://www.musicalboard.com/singing-recorder/), the spectrum graph syncs to the playback position. You can move the playback bar to a consonant where sibilance was heavy, raise the upper limit to 16000 Hz or 22000 Hz, and perform a focused analysis of the high-frequency energy at that moment.

After checking your range with [Vocal Range Test](https://www.musicalboard.com/vocal-range-test/), use the voice-type fundamental frequency data from that session to set optimal Min and Max values in Vocal Spectrum — building the analysis view best suited to your voice type.

## Practicing with MusicalBoard Vocal Spectrum

Setting up [Vocal Spectrum's](https://www.musicalboard.com/audio-spectrum-analyzer/) frequency range and sensitivity to a good starting point takes about ten minutes.

If this is your first time, start with the default (80 Hz–16 kHz, sensitivity about 50%). Enable the microphone and speak or sing at your normal volume. Confirm that the signal registers clearly on the graph. If background noise appears prominently, lower the sensitivity. If the signal looks too faint, raise it.

Next, experiment with switching ranges for different purposes. Switch to 80–8000 Hz for harmonic analysis, or 3000–16000 Hz for sibilance inspection. After a few attempts you will develop an intuitive sense of which settings suit your environment and goals. Once you find those settings, you can minimize setup time in future sessions and spend more time actually practicing.

## References

- [Singing Voice Acoustics — CCRMA, Stanford University](https://ccrma.stanford.edu/CCRMA/Courses/152/singing.html)
- [A Formant Range Profile for Singers — PMC/NIH](https://pmc.ncbi.nlm.nih.gov/articles/PMC5409887/)
- [Singing Voices and Frequencies — DPA Microphones](https://www.dpamicrophones.com/dict/singing-voices-and-frequencies/)
- [What is de-essing? The dos and don'ts of using a de-esser — iZotope](https://www.izotope.com/community/blog/the-dos-and-donts-of-de-essing)
- [Web Audio API — MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
- [The Science of the Singing Voice — Johan Sundberg](https://archive.org/details/scienceofsinging0000sund)
- [National Center for Voice and Speech](https://ncvs.org/)
