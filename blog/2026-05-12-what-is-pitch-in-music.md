---
title: "What Is Pitch in Music? — Frequency, Note Names, and the Connection to Singing"
date: 12 May 2026
tags: ["Vocal Pitch Monitor"]
excerpt: "Pitch is more than just 'high' or 'low.' Understanding the relationships between frequency, note names, A4=440Hz, and octaves makes it clear why intonation training matters — and how Vocal Pitch Monitor supports that process."
---

You've probably heard someone say a singer's "pitch is off," or conversely felt that someone's pitch was remarkably accurate. But if someone asked you to explain exactly what pitch is and why it matters, "it's the highness or lowness of a sound" is about as far as most people get. This article is meant to give a proper answer to that question. Starting from the physical definition of pitch, it works through note names, octaves, semitones, MIDI, and why pitch accuracy is so important in singing. At the end, we'll look at how MusicalBoard's [Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/) ties all of these concepts together in real practice.

## The Physics of Pitch — Frequency and Vibration

Sound is vibrating air. When an object (vocal cords, a guitar string, a piano string) moves rapidly back and forth, it pushes and pulls the surrounding air molecules, creating a wave. When that wave reaches your ears, you perceive it as sound.

**Pitch is determined by the speed of that vibration — in other words, by frequency.** Frequency is measured in Hz (hertz), which represents how many times something vibrates per second. 440 Hz means 440 vibrations per second. The faster the vibration (the higher the Hz), the higher the pitch; the slower (the lower the Hz), the lower the pitch.

In singing, the vocal cords perform this role. The more the vocal cord muscles tense, the faster they vibrate and the higher the pitch produced. The more they relax, the slower the vibration and the lower the pitch. This is why the muscles in the throat tighten when a singer reaches for a high note.

### The Three Properties of Sound

Sound is generally described by three properties:

- **Pitch**: The speed of vibration (frequency). How high or low it sounds.
- **Loudness (Amplitude)**: The size of the vibration (amplitude). How loud or soft it sounds.
- **Timbre**: The shape of the waveform. Why a piano and a violin sound different even at the same pitch and volume.

Pitch is the most fundamental of these properties and the most direct factor in whether a singing performance sounds "in tune" or "out of tune."

## Pitch and Note Names — What A4=440Hz Means

To make physical frequencies usable in music, people gave them names. Those are **note names**. The globally standardized reference point is:

**A4 = 440 Hz**

A4 is the A note above middle C (C4) on a piano keyboard. This standard was established by the [International Organization for Standardization (ISO 16)](https://www.iso.org/standard/3601.html) and is used worldwide as the orchestral tuning reference. That said, not all music follows this standard. Some period instrument ensembles use A4=415Hz, and some orchestras tune higher, at A4=442Hz or A4=444Hz.

### The Octave — Doubling the Frequency

One of the most important relationships in note naming is the **octave**. An octave refers to the note of the same name directly above or below, where the frequency is exactly **doubled**.

| Note | Frequency |
|---|---|
| A3 | 220 Hz |
| A4 | 440 Hz |
| A5 | 880 Hz |
| A6 | 1760 Hz |

Going up one octave from A4 to A5 doubles the frequency from 440 to 880 Hz. This relationship exists because the human auditory system perceives frequency in **ratios**, not absolute values. Moving from 220 Hz to 440 Hz and from 440 Hz to 880 Hz both sound like "one octave up" to the ear.

<p class="mb-blog-image">
  <img src="/blog/assets/vocal_pitch_monitor/piano_note_frequency.png" alt="Piano keyboard chart showing note names and their corresponding frequencies">
</p>

<p align="center">
  <em>With A4=440Hz as the reference, the frequency doubles with each octave going up.</em>
</p>

## Semitones, Whole Tones, and 12-Tone Equal Temperament

Within one octave in Western music, there are **12 semitones**. On a piano keyboard, moving one key at a time — including both white and black keys — is one semitone. Two semitones make a whole tone.

The tuning system used in most music today is **12-tone equal temperament (12-TET)**. In this system, one octave (a 2:1 frequency ratio) is divided into 12 semitones of **equal** size. Each semitone is 2^(1/12) ≈ 1.0595 times the frequency of the previous one.

So going one semitone up from A4 (440 Hz) gives A#4/Bb4 = 440 × 1.0595 ≈ 466.2 Hz.

This equal division means the intervals between notes in a scale are consistent in any key, making transposition freely possible.

### MIDI Numbers

MIDI (Musical Instrument Digital Interface) is the system used in digital music to represent pitch as a number. **MIDI numbers run from 0 to 127**, with middle C (C4) assigned 60 and A4 assigned 69. Each semitone up increases the number by 1, and one octave is a difference of 12.

| Note | MIDI Number | Frequency |
|---|---|---|
| C4 (Middle C) | 60 | 261.6 Hz |
| A4 | 69 | 440 Hz |
| C5 | 72 | 523.3 Hz |

The conversion formula between MIDI number and frequency: `Hz = 440 × 2^((MIDI-69)/12)`

## Human Hearing Range vs. Vocal Range

The frequency range of sounds humans can hear is roughly **20 Hz to 20,000 Hz (20 kHz)**. With age, the ability to perceive high frequencies tends to decline; in practice, many adults have an effective hearing range closer to 20 Hz–16 kHz.

The fundamental pitch range of the human singing voice is much narrower:

| Voice type | Typical range | Frequency range (fundamental) |
|---|---|---|
| Bass | E2–E4 | 82–330 Hz |
| Baritone | A2–A4 | 110–440 Hz |
| Tenor | C3–C5 | 131–523 Hz |
| Alto | F3–F5 | 175–698 Hz |
| Mezzo-soprano | A3–A5 | 220–880 Hz |
| Soprano | C4–C6 | 262–1047 Hz |

These ranges refer to the fundamental frequency, but the voice also contains harmonics (overtones) that extend energy into much higher frequency regions. This is why a spectrum analysis of a singing voice shows energy reaching into the several-kHz range.

<p class="mb-blog-image">
  <img src="/blog/assets/vocal_pitch_monitor/vocal_range_frequency_chart.png" alt="Comparison chart of human hearing range and vocal ranges by voice type">
</p>

<p align="center">
  <em>The vocal range is only a slice of the full human hearing range, but it is the most musically complex and expressive region.</em>
</p>

## Pitch vs. Interval — Two Easily Confused Concepts

**Pitch** and **interval** are concepts that are frequently confused.

- **Pitch**: The absolute height of a single note. A property of one note — like "A4=440Hz."
- **Interval**: The **relative distance** between two notes. Described in terms like "major third," "perfect fifth," etc.

For example, the distance from C4 to E4 is a major third (4 semitones). Pitch is the *location* of C4; interval is the *distance* between C4 and E4.

Both concepts matter in singing. **Pitch accuracy** is about whether you're hitting your target note precisely. **Interval sense** is the ability to accurately perceive the distance when jumping from one note to the next. Perfect pitch accuracy with no interval sense means the shape of the melody itself will be wrong. And knowing your intervals but having unstable pitch on each note makes the singing sound shaky and unreliable.

## Why Accurate Pitch Matters in Singing

### The Problem of Dissonance

When the notes in a key are sung at precise pitches, harmonics align and the music sounds stable. Even a small deviation in pitch causes overtones that should ring together to clash slightly — a psychoacoustic phenomenon known as **beating**, in which two close but non-identical frequencies create a periodic fluctuation in volume. This is one of the main reasons a slightly off pitch creates a vague sense that "something sounds wrong."

### Precision of Emotional Expression

Pitch accuracy isn't simply about being "right" or "wrong." A slightly flat pitch (♭) sounds sad or heavy; a slightly sharp pitch (♯) sounds tense or bright. These can be used expressively with intention, but unintentional pitch deviation blurs the emotional message. In a love song, if the pitch keeps drifting flat, it can start to sound like a sad song.

### Pitch in Choir and Ensemble Singing

When singing alone, pitch errors are heard in isolation. But when multiple voices sing together, pitch discrepancies become much more obvious. Two singers singing the same note with even a 5 Hz difference will produce audible beating. This is why pitch training is especially emphasized in choral and ensemble contexts.

### Pitch and Vibrato

Good vibrato is an even oscillation of roughly ±0.5 semitones above and below a target pitch. If the center pitch is unstable, the vibrato skews upward or droops downward, sounding uneven. When watching your vibrato in [Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/), checking whether the waveform is symmetric is exactly the point.

## MusicalBoard Vocal Pitch Monitor — Seeing Your Pitch in Real Time

[Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/) analyzes sound from your microphone in real time and displays the pitch as a graph. All you need to do is allow microphone access in the browser and you're ready to go.

### Key Features

**Real-time pitch graph**
The Y-axis shows note names (C, C#, D, D#, etc.) and the X-axis shows time. As you sing, a line is drawn. If the line holds steady at a particular note name position, you're hitting that note well. If the line wavers or keeps oscillating above and below the target note, your pitch is unstable.

**Logarithmic Y-axis**
The Y-axis displays frequency on a **logarithmic scale**. Because the human auditory system perceives frequency as ratios (as explained above), a log scale is far more natural and readable than a linear one. Each semitone occupies the same distance on the Y-axis, allowing you to read note names directly off the graph.

**Mpte range setting**
Adjust the minimum and maximum note to narrow the Y-axis range to fit your own vocal range. A soprano, for example, might narrow the display to C4–C6 for a more detailed view.

**Sensitivity adjustment**
In a quiet environment with soft singing, increasing sensitivity allows weaker signals to appear on the graph. In a noisy environment, lowering sensitivity reduces background noise from appearing on the graph.

### How to Use It

- Sustain a note and watch whether the line stays steady. If it wavers, check your breath support and vocal cord control.
- Going into high notes, see if there's a tendency for pitch to fall flat. This is often a sign of insufficient support.
- During vibrato training, check whether the waveform oscillates evenly around the target pitch.
- Use alongside [Vocal Range Test](https://www.musicalboard.com/vocal-range-test/) to systematically identify which parts of your range have unstable pitch.

<p class="mb-blog-image">
  <img src="/blog/assets/vocal_pitch_monitor/pitch_monitor.png" alt="Vocal Pitch Monitor real-time pitch graph display">
</p>

<p align="center">
  <em>The more steadily the graph line holds to a single position, the more accurate the pitch. The width and direction of any wobble tells you what kind of pitch issue you're dealing with.</em>
</p>

## Connecting to Other Tools

Pitch is one of several elements in singing. Accurate pitch is the foundation; rhythm, timbre, and dynamics are built on top of it.

- **To identify your vocal range**: Use [Vocal Range Test](https://www.musicalboard.com/vocal-range-test/) to see your lowest and highest notes and the accuracy of each pitch, displayed in color.
- **For scale practice**: Practice scales with [Vocal Scales](https://www.musicalboard.com/vocal-scales/) while checking each note's accuracy in real time with Vocal Pitch Monitor.
- **To practice with rhythm**: Open [Online Metronome](https://www.musicalboard.com/online-metronome/) alongside and watch the pitch graph while singing in time with the click.
- **To analyze your sound's texture**: Use [Vocal Spectrum](https://www.musicalboard.com/audio-spectrum-analyzer/) to examine harmonic structure and formant energy.

## Practicing with the MusicalBoard Vocal Pitch Monitor

Now that you understand the concept of pitch, it's time to train both your ears and your eyes together. Open [Vocal Pitch Monitor](https://www.musicalboard.com/vocal-pitch-monitor/), allow microphone access, and start with a long "ah" sound in a comfortable part of your range. Simply observing where and how the line is drawn will start to reveal your own pitch habits.

The next step is to play a specific note on [Virtual Piano](https://www.musicalboard.com/virtual-piano/) and use that note as a drone to match against. The core of pitch training is experiencing the feeling of your line landing right on the target note name in the graph.

## References

- [Pitch (music) — Wikipedia](https://en.wikipedia.org/wiki/Pitch_(music))
- [Frequency and Pitch — The Physics Classroom](https://www.physicsclassroom.com/class/sound/Lesson-2/Frequency-and-Pitch)
- [Musical Note — Wikipedia](https://en.wikipedia.org/wiki/Musical_note)
- [ISO 16: Acoustics — Reference frequency for the calibration of pure-tone audiometers — ISO](https://www.iso.org/standard/3601.html)
- [MIDI Tuning Standard — Wikipedia](https://en.wikipedia.org/wiki/MIDI_tuning_standard)
- [Equal Temperament — Wikipedia](https://en.wikipedia.org/wiki/Equal_temperament)
- [Web Audio API — MDN Web Docs](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)
