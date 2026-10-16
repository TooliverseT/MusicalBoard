---
title: "What Is a Vocal Range Map? — How to Read Session SCORE, LIVE, RANGE, and VOICE"
date: 15 May 2026
tags: ["Vocal Range Test"]
excerpt: "What the chromatic heatmap and the four indicators — SCORE, LIVE, RANGE, and VOICE — in Vocal Range Test actually mean, and how to turn that data into real practice gains."
---

Open [Vocal Range Test](https://www.musicalboard.com/vocal-range-test/), turn on your microphone, and the chromatic strip begins filling with color while four numbers — SCORE, LIVE, RANGE, and VOICE — update in the upper right. At first glance it can be hard to tell what any of it means. This article explains exactly what each indicator measures, how to interpret the heatmap colors, and how to connect all that data to your actual practice.

## What Is a Vocal Range Map?

A vocal range map is a tool that **visualizes pitch accuracy and occurrence frequency for each note** during a singing session. Arranged like piano keys from low to high in semitone steps, the chromatic strip uses color to show how accurately you sang each note.

The key insight is that instead of simply telling you "your range spans this many octaves," it lets you see **which individual notes are on pitch and which are unstable** — note by note. That is what separates a vocal range map from an ordinary pitch monitor.

### What Is the Chromatic Strip?

"Chromatic" means "constructed in semitone steps." The strip is a horizontal band in which the twelve pitches of one octave — C, C#, D, D#, E, F, F#, G, G#, A, A#, B — repeat across multiple octaves. Vocal Range Test's chromatic strip spans the wide range that the human voice can reach.

Each cell in the strip represents one semitone. When you sing a note, that cell fills with color.

## How to Read the Four Indicators

### SCORE — Overall Session Pitch Accuracy

SCORE is the **mean 0–100 score of accepted pitch frames**, measured against the **nearest equal-tempered note**. Frames must pass the heatmap’s signal and sustained-voicing checks, and their mapped notes must be inside Low–High. It is not a score against an uploaded song, a score, or the note currently played by Vocal Scales.

A score closer to 100 means the accepted frames are close to equal-tempered note centers. A clean C#4 can score 100 even when the exercise called for C4. More samples can make the average less sensitive to one moment, but a longer recording does not automatically make it an assessment of musical accuracy, rhythm, breath support, or vocal technique.

**Practical interpretation:**
- Use the score to compare the **same exercise under the same conditions**, not as a universal ability grade.
- Listen and verify the intended notes against a piano or score before calling a high score “correct.”
- Check the per-note colors and recording: one session average can hide a difficult note.
- If the result changes after adjusting Low–High or Pitch gate, compare again with matching settings.

Record SCORE together with the exercise, range, gate setting, and microphone setup. The number can help track nearest-note centering under comparable conditions; it cannot tell whether you selected the correct notes in a song.

<p class="mb-blog-image">
  <img src="/blog/assets/vocal_range_test/vocal_range_test_indicators.png" alt="Vocal Range Test SCORE, LIVE, RANGE, and VOICE indicator display area">
</p>

<p align="center">
  <em>The four indicators in the upper right: SCORE (session-wide average), LIVE (current real-time), RANGE (detected pitch range), VOICE (estimated voice type).</em>
</p>

### LIVE — Current Real-Time Pitch Status

LIVE is a **current nearest-note score**, with display smoothing. Its curve is more forgiving than the curve used for SCORE and Accuracy colors, so the two readings are not interchangeable. It reports distance from a detected pitch to its nearest equal-tempered note; it does not know your intended target.

A high LIVE value means the detected pitch is near an equal-tempered note center. It can remain high on the wrong note or during movement between centered notes, so use the reference sound and trace to judge the intended exercise.

**How to use it:**

Hold a reference note and compare its name with the detected note before interpreting LIVE. A low reading can reflect pitch movement, weak input, noise, or a detection error. A fluctuating reading does not diagnose insufficient breath support. Look at the recording and graph before choosing a vocal adjustment.

When using vibrato, periodic rises and falls in LIVE are normal. The vibrato center — its pitch center — should sit on the target note so that the average LIVE value stays high.

### RANGE — Lowest to Highest Note Detected in the Session

RANGE shows the **lowest and highest sustained notes that pass the range checks**. Those checks include signal strength, nearest-note score, continuous voicing, accumulated time, and edge cleanup. “C3 ~ A4” is the accepted span for this session, not every transient pitch heard by the microphone.

Beyond simply measuring your range, logging RANGE after each session lets you track how your range shifts over time. Comparing RANGE on good voice days versus difficult ones also helps you understand your voice's variability.

**Important caveat:**

A brief squeak, slide, or weak frame may color the heatmap without moving RANGE. The range gates are stricter than the heatmap gates. Conversely, a forced note may still qualify if it sustains and passes them: the tool cannot judge comfort. Treat RANGE as an observed, filtered span, not proof of usable vocal limits.

Your **tessitura** needs comfortable sustained singing and listening over several sessions. A solid green column only describes this session’s accepted frames; it cannot establish comfort, timbre, or endurance.

### VOICE — Estimated Voice Type

VOICE displays **the voice type estimated from the RANGE detected in the session** — classifications such as soprano, mezzo-soprano, alto, tenor, baritone, and bass.

**An important limitation:**

Voice type classification is not determined by range alone. Traditional classical voice classification weighs range alongside timbre, the passaggio (the register break between chest and head voice), volume, and tessitura (the range where the voice is most comfortable). The VOICE indicator is a quick reference based solely on range data.

VOICE compares RANGE with six reference bands and chooses the largest semitone overlap. Ties go to the first matching band in the tool’s list. A limited exercise can produce a label different from your usual voice type without anything being wrong with your voice. Do not infer a specific classification change just from skipping high or low notes; a teacher is the right resource for an actual voice-type assessment.

## The Two Channels of the Heatmap — Accuracy and Frequency

How each cell in the chromatic strip is colored and how intensely it fills can be read along two dimensions.

### Accuracy Channel

Accuracy color summarizes each note’s **lower-quartile nearest-note score**: the lower 25% value, or the minimum when fewer than four samples are available. This makes rough moments more visible than a simple average would. SCORE uses the accepted-frame mean, so a high SCORE and a less favorable color can coexist.

- **Green**: A favorable lower-quartile score near the nearest equal-tempered note, with no guarantee that it is the intended note.
- **Yellow–Orange**: Slightly off. You are producing the note but landing a little flat or sharp.
- **Red**: A low lower-quartile nearest-note score. Check the pitch trace and recording before deciding whether drift, noise, or a detection error caused it.

### Frequency / Density Channel

Coverage reflects **accepted frame count relative to the most-visited note** in the displayed map, shown by opacity. Darker means more samples relative to that session’s busiest note; it is not an absolute count or a taller bar. When Coverage is off, opacity is fixed. With Accuracy off, the columns use a neutral color.

You need both dimensions together for the heatmap to be meaningful. Dark green means "sung often and accurately." Light green means "accurate but not sung much." Bright, intense red means "you hit this note often but intonation was consistently off" — a clear signal for targeted work.

## Reading Colors on the Chromatic Strip — In Practice

After a session, scan the full strip and note these patterns.

**1. The darkest green zone = frequently detected, well-centered notes in this session**

A dark green cluster is a starting point for review: you spent time there and the lower-quartile score was favorable. Confirm that those notes feel comfortable and sound right before using the cluster for song-key decisions. Repeating one note for a long time can darken it without making it your tessitura.

**2. Yellow / orange zones = your next practice targets**

The yellow and orange zones between green and red are "almost there but still unstable" notes. Focused work on these zones tends to convert them to green relatively quickly.

**3. Red zones = limit notes or weak spots**

Red cells deserve a listening check. They do not establish a physical range limit or diagnose a technical fault: noise, pitch-candidate errors, and brief off-center moments can affect the color. Work comfortably and compare repeated attempts with a reference.

**4. Empty cells = notes not yet attempted**

Empty cells mean no frames for that note passed the current checks. The note may be unattempted, too brief, too quiet, outside the chosen window, or not reliably detected; emptiness does not prove you cannot sing it.

<p class="mb-blog-image">
  <img src="/blog/assets/vocal_range_test/vocal_range_heatmap.png" alt="Vocal Range Test chromatic strip heatmap example — green, yellow, and red color zones">
</p>

<p align="center">
  <em>Dark green = note sung often and accurately. Yellow / orange = slightly unstable. Red = significantly off pitch. Empty cells = no accepted frames for those notes.</em>
</p>

## Practical Application — Identifying Weak Notes and Setting Practice Priorities

Once you have read the heatmap and the four indicators, it is time to connect them to your practice.

### Step 1: List Your Weak Notes from the Heatmap

Note down any notes showing heavy red or orange. If F#4, G4, and G#4 are orange, those are your currently unstable zone.

### Step 2: Targeted Scale Drills

Open [Vocal Scales](https://www.musicalboard.com/vocal-scales/) and choose a scale that includes your weak notes, then open [Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/) to watch real-time pitch as you practice. Observe where the pitch graph wavers around F#4, G4, and G#4, and adjust your breath, posture, and vowel shape accordingly.

### Step 3: Drone Locking on a Specific Note

Choose one of your weak notes and press it on [Virtual Piano](https://www.musicalboard.com/virtual-piano/) with Sustain enabled. Replay the finite sample when it fades. Match it with an "aah" or "ee" vowel and repeat. Keep Vocal Pitch Monitor open at the same time to confirm that your voice is landing on the intended reference note.

### Step 4: Re-measure to Confirm the Change

After focused practice, run Vocal Range Test again. You may see a previously orange cell turn green, or your SCORE climb. Seeing that change reinforces the value of your effort and sustains motivation.

### Step 5: Range Extension Practice

Review edge notes with the same settings over several sessions. A better color supports a claim about detected nearest-note alignment; usable range also requires comfortable, repeatable singing and a listening check. Do not force a note merely to fill the map.

## Using It with a Metronome

If you want to train pitch and rhythm simultaneously, keeping [Online Metronome](https://www.musicalboard.com/online-metronome/) open alongside is very useful. Sing scales containing your weak notes in time with the metronome click while Vocal Range Test fills in the heatmap for each note. Once intonation is stable at a slow tempo, gradually raise the BPM to practice maintaining the same accuracy at faster tempos.

## Building a Habit of Logging Session Data

Vocal Range Test's indicators persist only within the session. Refreshing the page resets them. If you want to track your own data over time, you need to record it manually.

A simple approach is to log the following in a notebook or spreadsheet:

- Date
- SCORE value
- RANGE (lowest to highest note)
- Weak notes noticed today
- Practice conditions (voice health, weather, whether you warmed up beforehand)

Comparing this data over two-to-four-week periods makes visible the trend in your SCORE, shifts in your range, and the progress of specific weak notes improving. Seeing growth in numbers is a powerful motivator for keeping up consistent practice.

## Practicing with MusicalBoard Vocal Range Test

[Vocal Range Test](https://www.musicalboard.com/vocal-range-test/) requires only a microphone permission grant in your browser — no installation, no login. Here is how to run your first session:

1. Allow the microphone and warm up for a few minutes (humming, lip trills, etc.).
2. Start in your comfortable middle range, then explore slowly downward and then upward.
3. Sustain each note for 2–3 seconds on an "aah" vowel and watch the heatmap fill in.
4. When the session ends, note your SCORE, RANGE, and VOICE, then review the full heatmap.
5. Identify your yellow and orange zones and plan your next practice session.

Running Vocal Range Test regularly lets you move beyond a simple "my range is X octaves" answer to a systematic understanding of which notes are stable and which are not. That is the difference between repeating songs aimlessly and practicing with data.

## References

- [Voice type — Wikipedia](https://en.wikipedia.org/wiki/Voice_type)
- [Tessitura — Wikipedia](https://en.wikipedia.org/wiki/Tessitura)
- [Vocal range — Wikipedia](https://en.wikipedia.org/wiki/Vocal_range)
- How to Find Your Vocal Range — School of Rock
- [Pitch Detection Algorithm — Wikipedia](https://en.wikipedia.org/wiki/Pitch_detection_algorithm)
- [Web Audio API — MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
