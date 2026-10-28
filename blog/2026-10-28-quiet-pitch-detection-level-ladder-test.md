---
title: "How Quiet Is Too Quiet: Pitch Detection from −20 to −60 dBFS"
date: 28 October 2026
tags: ["Vocal Pitch Monitor"]
excerpt: "Eight synthetic tones step down in level while Pitch gate stays at 0%, 50%, or 100%. Through −60 dBFS the tool still saved a full pitch timeline on upload, but at 0% gate the graph can look empty because the display hides what was already detected."
---

A practice WAV can sound fine in headphones while the pitch graph looks empty. That gap is often about **how loud the file is, and how strict Pitch gate is**, not about the browser “refusing” quiet audio.

MusicalBoard’s [Vocal Pitch Monitor](/vocal-pitch-monitor/) analyzes an upload **once**, at a fixed high-sensitivity setting. **Pitch gate** then decides which of those saved points appear on the graph. An earlier [nine-import test](/blog/2026-10-20-upload-pitch-gate-reanalysis-test/) showed that moving the slider before import does not change what gets saved. This experiment asks a different question: **if you keep the same tone and only lower the level, where does the pitch line actually break?**

We used **eight** three-second, 48 kHz, 16-bit mono tones: a 220 Hz fundamental plus second and third harmonics (relative amplitudes 0.35 and 0.15), 20 ms fades, no noise bed. Target peaks were −20, −26, −32, −38, −44, −50, −56, and −60 [dBFS](https://en.wikipedia.org/wiki/DBFS) in six-decibel steps. After 16-bit rounding the measured peaks sit about **2 dB lower** than the target because of the harmonic mix and fades. Those dB values are **digital sample peak**, not RMS, microphone gain, or loudness in a room.

**Try a step in the tool** (loads the tone like **Upload audio**, no save-to-disk step): [−20](/vocal-pitch-monitor/?ladder=20) · [−26](/vocal-pitch-monitor/?ladder=26) · [−32](/vocal-pitch-monitor/?ladder=32) · [−38](/vocal-pitch-monitor/?ladder=38) · [−44](/vocal-pitch-monitor/?ladder=44) · [−50](/vocal-pitch-monitor/?ladder=50) · [−56](/vocal-pitch-monitor/?ladder=56) · [−60](/vocal-pitch-monitor/?ladder=60)

If the page is still restoring a saved take from this browser, the sample may appear after a short wait (up to about 15 seconds) once analysis is idle.

**Download WAV** (same files; these links ask the browser to save instead of playing in a new tab):

<p>
<a href="/static/samples/quiet-ladder/level--20dbfs.wav" download="level--20dbfs.wav">−20 dBFS</a> ·
<a href="/static/samples/quiet-ladder/level--26dbfs.wav" download="level--26dbfs.wav">−26</a> ·
<a href="/static/samples/quiet-ladder/level--32dbfs.wav" download="level--32dbfs.wav">−32</a> ·
<a href="/static/samples/quiet-ladder/level--38dbfs.wav" download="level--38dbfs.wav">−38</a> ·
<a href="/static/samples/quiet-ladder/level--44dbfs.wav" download="level--44dbfs.wav">−44</a> ·
<a href="/static/samples/quiet-ladder/level--50dbfs.wav" download="level--50dbfs.wav">−50</a> ·
<a href="/static/samples/quiet-ladder/level--56dbfs.wav" download="level--56dbfs.wav">−56</a> ·
<a href="/static/samples/quiet-ladder/level--60dbfs.wav" download="level--60dbfs.wav">−60</a>
</p>

If a plain audio link still opens the player, use **Download** on that page or right-click → **Save link as**, then upload the file from Singing Recorder or Vocal Pitch Monitor.

## How we counted the pitch line

After each upload we asked two questions at Pitch gate **0%, 50%, and 100%**:

- **How much of the 3-second tone was saved?** The tool stores a pitch reading about every 25 milliseconds. A complete steady tone fills **119 readings** — that is almost the whole file, not a score out of 100. **0** means no pitch was saved. A number in between means the line covers only part of the tone.
- **How much of that line is visible?** Pitch gate can hide weaker points without running analysis again. The visible count is what you would see on the main pitch line at that slider setting.

We imported the **−20 dBFS** file in **Chromium on Linux** on 28 October 2026 and got the same pattern as the table below. You can repeat any row with the **Try a step** links above or by uploading a downloaded WAV in a fresh session.

This is a **synthetic tone** experiment. A breathy vowel, a consonant, room reverb, or a full mix is harder than a steady harmonic tone. The numbers describe this ladder on the date above, not every voice or room.

## From −20 to −60 dBFS: level did not thin the saved line

| Target peak (dBFS) | Measured peak (dBFS) | Pitch readings saved | Visible at 0% gate | 50% | 100% |
|---:|---:|---:|---:|---:|---:|
| −20 | −22.0 | 119 (full) | 0 (empty graph) | 119 | 119 |
| −26 | −28.0 | 119 (full) | 0 | 119 | 119 |
| −32 | −34.0 | 119 (full) | 0 | 119 | 119 |
| −38 | −40.0 | 119 (full) | 0 | 119 | 119 |
| −44 | −46.0 | 119 (full) | 0 | 119 | 119 |
| −50 | −52.0 | 119 (full) | 0 | 119 | 119 |
| −56 | −58.0 | 119 (full) | 0 | 119 | 119 |
| −60 | −62.0 | 119 (full) | 0 | 119 | 119 |

Every row is the same on purpose. In this “quiet but still intentional” range, **upload still saved the whole 3-second pitch line**. Typical detected pitch was about **220.2 Hz** (~+3.9 cents vs 220 Hz), which is normal for short-window tracking on a generated tone.

The change is in **what the graph shows**, not in what was saved:

- At **0% Pitch gate**, the main pitch line was **empty** at every level even though the full timeline was stored. That setting only draws the strongest readings. On these tones the tracked pitch sat just below that bar, so the graph looked broken while the data was still there.
- At **50% and 100%**, the **full line** showed at every level. Raising the gate did not invent missing analysis. It **drew** analysis that upload had already stored.

So **−60 dBFS was not too quiet to analyze** on upload. It was still **too strict for the 0% display setting**. That is not a recording target — most usable vocal files sit much louder than this ladder.

## Where saving actually stops (quieter than −60)

The published files stop at −60 dBFS because that span matches quiet practice peaks, not silence. Using the same generator at **lower levels not included in the download set**, the saved line thins out like this:

| Target peak (dBFS) | Pitch readings saved |
|---:|---|
| −66 | 119 (full) |
| −72 | 119 (full) |
| −74 | 119 (full) |
| −75 | 118 (almost full) |
| −76 | 58 (about half the tone) |
| −77 | 0 (nothing saved) |

On this tone, **saved pitch disappears around −77 dBFS**. Near **−76 dBFS** the line covers only about half the file. That matches the spirit of the [extremely quiet source](/blog/assets/vocal_pitch_monitor/level-5e-05.wav) in the nine-import post (~−86 dBFS), which saved **nothing** at any gate.

At the very bottom, **16-bit files run out of detail**: an extremely small peak has only a few distinct sample values. Playback may still make a faint sound, but there is not enough waveform left for a stable pitch reading.

## What this means for an uploaded practice take

A quiet phone recording of a scale, or a take exported too low from a DAW, can look like “the detector failed.” On this ladder, two different failures get mixed together:

1. **Listen first.** If the voice is buried, no slider replaces gain at capture.
2. **Set Low–High** to your range so notes you did not sing do not take over the display.
3. **If the take sounds loud enough but the line is sparse**, raise **Pitch gate**. Dots that appear came from **already saved** readings — the same lesson as the nine-import test, not a second analysis pass.
4. **If the line stays empty at every gate**, the file may be below what upload can save, or mostly breath and noise. Export a hotter voice-only file. Turning the level up later also raises noise, and it cannot undo clipping or a file that was already quantized to almost nothing.
5. **A new session does not start at 0% gate.** The default is less strict, so “it works for me” and “0% looks blank” can both be true.

Live microphone analysis **does** change detection with Pitch gate while you sing. These uploads do not describe that path. See [How Browser Pitch Detection Works](/blog/2026-05-05-pitch-detection/).

## Repeat the ladder

Pick a **Try a step** link above, or download a WAV and import it at Pitch gate **0%, 50%, and 100%** in fresh sessions. Check whether the graph changes only in visibility, or whether the line is missing at every setting.

The home page [**Load sample**](/) uses a different phrase for reading pitch **offsets**, not this level ladder.

For what Pitch gate does and does not re-run on upload, read [Does Pitch Gate Reanalyze an Uploaded Take?](/blog/2026-10-20-upload-pitch-gate-reanalysis-test/) next to this one. That post uses **three** widely spaced levels. This post **steps level** from −20 to −60 dBFS and separates **where saving stops** (near −77 dBFS on this generator) from **where the strictest gate hides a line that was already saved**.
