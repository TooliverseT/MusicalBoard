---
title: "What Survives a Refresh: MusicalBoard Layout, Takes, Backing Tracks, and the 24-Hour Browser Cache"
date: 04 October 2026
tags: ["MusicalBoard"]
excerpt: "After a refresh on MusicalBoard, what keeps your layout, recording takes A–D, and loaded backing tracks? This article explains localStorage vs IndexedDB, the limits of the roughly 24-hour cache, and how to clear data yourself."
---

Sometimes when you practice vocals at home and hit refresh, the window arrangement stays put and the take you just recorded or the backing track you loaded shows up again. That does not mean those files were uploaded to a MusicalBoard server. MusicalBoard keeps layout and some tool settings in the browser’s **localStorage**, and keeps recorded or uploaded audio plus backing files for a while in the same browser’s **IndexedDB**. Before you use [Singing Recorder](https://www.musicalboard.com/singing-recorder/), it helps to know what persists, when it disappears, and how to clear it yourself — so an accidental refresh mid-practice is less of a shock.

## First, separate screen settings from audio files

“Stored in the browser” is not one single thing. On MusicalBoard, lightweight settings and large audio files live in different places.

**Layout and settings** stay in localStorage. That is why, after you open and close windows on Home and adjust their size and position, the previous arrangement shows up first on your next visit. Settings you want to carry into the next session — mic sensitivity, display range, monitor volume — belong here too. The same idea applies when you apply a [Practice layout on Home](https://www.musicalboard.com/blog/2026-09-20-home-practice-layout-presets-four-workflows/) once and tweak it bit by bit: that layout can still be there the next day.

**Recorded or uploaded takes and backing files** are kept in IndexedDB. IndexedDB is the storage web apps use when they need to keep relatively large data inside the browser. MusicalBoard stores the audio bytes there along with small details needed for restore — playback position, file format, save time, sensitivity, and the like. After a refresh, instead of fetching the file from a server again, the app can read the audio left in the browser and rebuild pitch and spectrum analysis.

Both stores are tied to a specific browser and device. A take you record in Chrome on a laptop does not automatically appear in Safari on your phone or in a browser on another computer. MusicalBoard does not use login accounts or cloud sync for this.

## What actually comes back after a refresh

During practice, these three restorations are usually the most visible.

### Window layout on the Home workspace

If you place Vocal Pitch Monitor and Vocal Scales side by side on Home and later open Vocal Spectrum, that arrangement persists on the next visit in the same browser. This is not a feature that saves “today’s practice order.” It remembers workspace settings such as which windows are open and their size and position.

For example, you might warm up in the Tone Check layout on Monday and start there again on Tuesday. Conversely, if someone else uses the same browser profile on the same computer, they might see your previous layout. On a shared machine, it is safer to clear site data after practice or use a separate profile.

### Recent takes in Singing Recorder

After you record or upload a local file in [Singing Recorder](https://www.musicalboard.com/singing-recorder/) and finish analysis, recent takes can be restored inside this browser. When you use Takes A–D, a new take becomes A and earlier takes shift back for comparison. If the analysis view does not return fully right after refresh, you may see a short restore step that reads the saved audio and rebuilds pitch and spectrum.

The important point is that **the app does not permanently store the analysis graphs themselves; it rebuilds what you need from the audio and minimal metadata**. Refresh does not delete your takes, but you may wait briefly while analysis is prepared again. For how to compare takes A and B, see [practicing the same phrase with A/B playback in the browser](https://www.musicalboard.com/blog/2026-09-25-takes-a-d-compare-vocal-sessions-browser/).

### Backing files, start position, and mix choices

If you loaded a backing track, more than the file name is kept. The backing file, cue position, backing volume, whether to play along while recording, and whether to mix the backing into the recording result are stored together, within what recent session restore needs. After refresh or a detour to another tool page, you can start again from the same backing if the retention window has not passed.

This is not a music library feature. Do not treat it as long-term storage for large backing files or a stack of multi-day projects. For recordings you want to keep, do not rely on the browser cache alone — download the file after practice and keep it in a folder on your device.

## “24-hour retention” is a short practice session, not a backup

MusicalBoard’s audio cache uses **about 24 hours** from when each item was stored. Takes or backing tracks past that window are removed on the next page load. Closing the tab does not always wipe them immediately, but cleanup also runs when the page is hidden or you navigate away. So there is no promise that everything will still be there tomorrow.

That limit is meant to let you continue a practice session without keeping audio long term. It is useful when you record twice in the evening, step away for dinner, come back to compare A/B, or refresh the page by mistake. For audition demos, class submissions, or reference takes you will compare over several days, download first.

Clearing site data in the browser, leaving private browsing, browser storage cleanup, or switching browsers can remove the cache sooner than 24 hours. Behavior depends on browser and device settings. For general behavior of browser storage and how to delete it, see MDN’s [Web Storage API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API) and [IndexedDB API](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API) documentation.

## When you want to keep data vs when you want to clear it

To keep a session going, think along these lines:

1. If you will compare a take again today, reopen Singing Recorder in the same browser and check restored takes A–D.
2. If you plan to reuse a backing track from the same position, confirm the backing tray restored and check the cue position before choosing the file again.
3. If you need a recording through tomorrow, verify playback works, then download the file and save it in a folder on your device.
4. If you need to listen on another device, move the downloaded file — not the local cache.

If you practiced on a shared device or do not want leftover audio, use the trash (Delete) control in Recorder to remove the current take and its cached copy. You can also clear MusicalBoard site data in browser settings, but that may reset layout and other saved settings too. Download anything you need before you do that.

## Using it in a practice flow

Imagine practicing one short phrase. Open the **Record a Phrase** layout on Home, check your starting pitch with Virtual Piano, then record Take A to Online Metronome or a backing track you prepared. On the next take, change only one thing — for example, “start the first note cleanly from below without sliding up.” Even if you need to refresh or look at another page for a while, within about 24 hours in the same browser you can load Take A and B again and compare.

What the cache does here is bridge a gap in the middle of practice. It is not a long-term memory substitute or a cloud drive for sharing with others. Keeping files safe and quickly replaying and comparing inside the browser are two jobs worth keeping separate.

## What MusicalBoard does and does not handle for privacy

MusicalBoard pitch, spectrum, and recording analysis run in your browser. Files you record or upload are not sent to MusicalBoard servers. Other services — such as ads and ordinary web traffic — are separate from audio processing; see the full scope in the [Privacy Policy](https://www.musicalboard.com/privacy/).

In short, layout and settings stay in the same browser so the next session starts faster; takes and backing tracks stay in IndexedDB for about 24 hours to help comparison after refresh. Download important files, and use the trash for audio you do not want — those two habits make the in-browser practice environment easier to live with.
