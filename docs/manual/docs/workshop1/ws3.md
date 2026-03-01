# Workshop 3: Lengthen/Shorten, Roughening the Surface

*by Dr Archer Endrich*

CDP Sound Transformation Explorations - Worksheet 3

The locations of the processing functions are shown under the program name.
- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

Copy [flex.wav](../../sounds/flex.wav) to your working directory. (All filenames start with 'flex' to show they belong together.) Also `wcc.dat`, `flexgrains.grn` and the breakpoint files: `flexdens.brk`, `flexgsize.brk`, `flexpchlo.brk`, `flexpchhi.brk`, `flextxplo.brk` and `flextxphi.brk`.


## A. Distort Repeat - Distort and Lengthen

**SS: Soundfiles > Distort cycles > Repeat**
**SL: DISTORT > repeat**

ST1. Least effect

repeat (groups) = `2`, cycles (in a group) = `2`. Output: `flexdr2-2.wav`

ST2. Highly distorted and considerably longer

repeat (groups) = `4`, cycles (in a group) = `5`. Output: `flexdr4-5.wav`


## B. Extend Loop - Extend with Separated or Overlapping Same-Size Segments

**SS: Soundfiles > Extend > Loop**
**SL: EXTEND > loop > loop advances to end**

ST3. Create overlaps, longish segments (works/phrases repeat)

- loop_length = 300ms (i.e., 0.3 sec)
- step = 100ms (i.e., 0.1 sec)
- (step moves 1/3 along the previous segment, then cuts out another 300ms, thus the overlap)
- Output: `flexloops.wav`

ST4. Create overlaps, shorter segments, with more overlap (more textured)

- segment_length = 100
- step = 20
- Output: `flextexture.wav`


## C. Extend Scramble - Randomised Segmentation and Jumbling

**SS: Soundfiles > Extend > Scramble > randomised**
**SL: EXTEND > scramble > completely random**

ST5. Mild, possibly funny result

- Mode 1 (Random chunks)
- shortest segment = 0.1
- longest segment = 0.3
- output duration = 10
- Output: `flexscram1a.wav`

ST6. Output is more erratic

- Mode 1 (Random chunks)
- shortest segment = 0.06
- longest segment = 0.2
- output duration = 10
- Output: `flexscram1b.wav`

ST7. Really mince it up (Mode 2 also rearranges the segments)

- Mode 2 (Segment s'file)
- shortest segment = 0.1
- scatter = 3
- output duration = 10
- Output: `flexscram2.wav`


## D. Modify Brassage - Granular Manipulations ('brassage' = 'mashing' [Fr.])

**SS: Soundfiles > Grain > Brassage**
**SL: BRASSAGE**

ST8. Mode 2 - Granulation with time-stretch (Sound Loom says 'timeshrink': values less than 1 stretch because this slows the speed through the infile)

- Mode 2 (SS: time-stretch / SL: timeshrink)
- Timestretch ('velocity') = `0.25` (4x as long - an inverse number: 1 divided by 4 = 0.25)
- Output: `flexgrnstr.wav`

ST9. Mode 2 - Granulation with time-varying time-stretch

- Mode 2 (SS: time-stretch / SL: timeshrink)
- `flex.wav` is 6.6 seconds long
- Click on 'TV' and enter the filename (or open): `flexgrnstrtv.brk`, then put in:

```
0     0.25
3     1.0
6     0.1
```

and SAVE and RUN. Output: `flexgrnstrtv.wav`

Run this again with `capm.wav` (timings in `flexgrnstrtv.brk` are close enough to stay the same).

ST10. Mode 4 - Scramble (variable grainsize)

- Mode 4 (Scramble)
- Click on 'TV' and enter the filename (or open): `flextvgrains.brk`

```
0    25    ;shortest grainsize = 25ms (i.e., 0.025 sec)
6   200    ;longest grainsize = 200ms (i.e., 0.2 sec)
```

Output: `flextvgrains.wav`

(This function currently has a bug -- it was realised using min (25ms) & max (200ms) values via the Command Line, a diagnostic option available in CDP.)

ST11. Mode 5 - Granulate the sound

- Mode 5
- density = `0.75` (creates little gaps to roughen the surface)
- Output: `flexgrainy.wav`

Try using `flexgrainy.wav` with Mode 2, time-varying time-stretch. Click in the time-stretch parameter box to activate it, then load `grnstr.brk`. Look at the length of `flexgrainy.wav` and re-do `flexgrnstrtv.brk` to scale the file to the new length. Output: `flexgrainystrtv.wav`

ST12. Mode 7 - Full granulation parameter set - Mode 7 ('Whole-grain')

**SS:** Load `wcc.dat` Preset collection (File - Load (Global) Preset collection), then open `flex.wav` and go to **Soundfiles - Grain - 'Wholegrain'** and use the Preset `flexbrassage`.

**SL:** Ensure that `flex.wav` is in Chosen Files, and that the breakpoint files listed below are on the Workspace (they should be there already as a result of Opening and grabbing the files from the directory in which you've placed the files for this Worksheet). Go to **BRASSAGE > full monty**. Then LOAD the `flexbrassage` patch.

All of the following data should now be in place -- it was made earlier and saved as a Preset / Patch. velocity (lo & hi) = 0.25 (i.e., 4x timestretch). In BRASSAGE the pitch shift parameter is transposition in semitones.

| Density = `flexdens.brk` | Grainsize = `flexgsize.brk` | Pitchshift = `flexpchlo.brk` | Pitchshifthi = `flexpchhi.brk` |
|---------------------------|-----------------------------|-----------------------------|-------------------------------|
```
0.0    75                   0.0    30                    0.0   -7                      0.0    7
1.0     2                   1.0   100                    1.0   -1                      1.0    1
6.0   100                   6.0    60                    6.0  -11                      6.0   11
```

- amp = 0.5, amphi = 1
- space = 0.5, spacehi = 1
- all splices = 10ms
- Output: `flexbrassage.wav`

ST13. Re-open `flex.wav` and run again in TEXTURE SIMPLE using the Preset / Patch `flextexture`. Note that in TEXTURE, pitch shift is given as MIDI Pitch Values relative to the reference pitch in the note data file. Thus the pitchshifting (transposition) uses `flextxplo.brk` and `flextxphi.brk`.

Output duration = 30 sec., `ndf60.txt` just contains '60', packing uses `flexpack.brk`, scatter is 0.02, sounds 1 and 1, velocity 64 and 84, dur 0.75 and 1.3, 'Use whole soundfile' is NOT ticked.

| Packing = `flexpack.brk` | Pitchshift = `flextxplo.brk` | Pitchshifthi = `flextxphi.brk` |
|---------------------------|------------------------------|-------------------------------|
```
0.0   0.05                  0.0   53                     0.0   67
1.5   0.1                   1.5   58                     1.5   62
6.0   0.025                 6.0   36                     6.0   84
```

Output: `flextxsimple.wav`


## E. GrainMill Version of the Granulation Software

(SAVE HISTORY in Soundshaper, Exit and Open GrainMill)

ST14. Granular manipulation with similar settings. Open GrainMill and, under OPTIONS, Set Working Directory to the directory in which you have the files for this Worksheet. Open soundfile `flex.wav` and then close the Dialogue Box without doing anything yet, because you're going to use a Preset.

Load the Preset (File - Load settings) `flexgrains.grn`. The dialogue box now reappears, filled in with the stored data: `flexdens.brk`, `flexgsize.brk`, `flexpchlo.brk` and `flexpchhi.brk`. Click on Make it and you will see the grains appear on the screen as they are made. The different colours relate to variations in amplitude.

When finished, you can PLAY the result from the Toolbar at the top of the screen. GrainMill makes a temporary file, so you can return to the dialogue box, alter the settings and run it again when working on a sound. When finished, go to File - Save soundfile and name it `flexgrainmill.wav`.


## Summary and Playlist

`flex.wav` - The input soundfile for all of the following, unless otherwise specified.

### Distortion

| File | Description |
|------|-------------|
| `flex2-2.wav` | DISTORT REPEAT, repeating 2 groups, with 2 cycles in a group |
| `flex4-5.wav` | DISTORT REPEAT, repeating 4 groups, with 5 cycles in a group (much longer) |

### Looping

| File | Description |
|------|-------------|
| `flexloops.wav` | EXTEND LOOP, with 1/3 overlap of the loop_lengths (len 300, step 100) |
| `flextexture.wav` | EXTEND LOOP, more textured because of 4/5 overlap of the loop_lengths (len 100 step 20) |

### Scramble Segments

| File | Description |
|------|-------------|
| `flexscram1a.wav` | EXTEND SCRAMBLE, random chunks somewhere between 0.1 and 0.3 sec in length |
| `flexscram1b.wav` | EXTEND SCRAMBLE, random chunks between 0.06 and 0.2 sec in length |
| `flexscram2.wav` | EXTEND SCRAMBLE, Mode 2 to rearrange segments, with scatter = 3 |

### Brassage ('mash')

| File | Description |
|------|-------------|
| `flexgrnstr.wav` | MODIFY BRASSAGE, Mode 2 with 4x timestretch (0.25 divided into 1 = 4) |
| `flexgrnstrtv.wav` | MODIFY BRASSAGE, Mode 2, with time-varying timestretch `flexgrnstr.brk` |
| `flextvgrains.wav` | MODIFY BRASSAGE, Mode 4, with time-varying grainsize (start at 25 and -> 200 ms) `flextvgrains.brk` |
| `flexgrainy.wav` | MODIFY BRASSAGE, Mode 5, granulate the sound, with tiny gaps (0.75) |
| `flexbrassage.wav` | Using `flexgrainy.wav` as the input - MODIFY BRASSAGE, Mode 7, multi-parameter time-varying brassage, using Preset `flexbrassage`, stored in `wcc.dat` with `flexdens.brk`, `flexgsize.brk`, `flexpchlo.brk` and `flexpchhi.brk` or as a Sound Loom Patch. Note how Brassage (and GrainMill) work their way through the whole sound. |

### Texture

| File | Description |
|------|-------------|
| `flextxsimple.wav` | TEXTURE SIMPLE, Mode 5 ('None') for a different approach (always starts from the beginning of the sound). The segments are between 0.75 and 1.3 sec. long, so we repeatedly hear a small part of the beginning of the sound, unless 'Use whole sound' is ticked. Uses the Preset `flextexture` from `wcc.dat`, with `flexpack.brk`, `flextxplo.brk` and `flextxphi.brk`. |
| `flexgrainmill.wav` | GRAINMILL - virtually the same settings as Brassage, but the pitch range expands more. This process also works its way through the whole sound. Preset: `flexgrains.grn` from `wcc.dat`, with `flexdens.brk`, `flexgsize.brk`, `flexpchlo.brk` and `flexpchhi.brk`. |
