# Workshop 4: Harmonic Tuning / Mix

*by Dr Archer Endrich*

CDP Sound Transformation Explorations - Worksheet 4

The locations of the processing functions are shown under the program name.
- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

Copy [flex.wav](../../sounds/flex.wav) to your working directory, if not already there. (All files associated with this input soundfile start with 'flex' to show that they belong together.) `Flex.wav` is 6.6 sec. mono, sampling a flexatone percussion instrument.

Also copy `wcc.dat`, `flexgrains.grn` and the breakpoint files: `flexdens.brk`, `flexgsize.brk`, `flexpchlo.brk`, `flexpchhi.brk`, `flextxplo.brk` and `flextxphi.brk`.

In the Time Domain, we will use FILTER USERBANK and TEXTURE SIMPLE with a harmonic grid. Then we will look at tuning in the Spectral Domain, using PITCH TUNE and then multiple transpositions with REPITCH TRANSPOSE. SUBMIX MIX will be used to create more complex effects, using the results of these processes. Skip ST1 if `flexgrainy.wav` is already present.


## A. Prepare the Input Soundfile

(NB: ST1 = Worksheet 3, ST11. ST2 is the same as Worksheet 3 ST12 except that `flexgrainy.wav` is used as the input instead of `flex.wav`.)

ST1. Create a 'grainy' soundfile (skip this if `flexgrainy.wav` is already made).

**SS: Soundfiles > Grain > Granulate SF**
**SL: BRASSAGE > granulate**

Input soundfile: [flex.wav](../../sounds/flex.wav). We can make use of a soundfile created with MODIFY BRASSAGE, Mode 5 (Granulate SF), as on the Worksheet for 'Lengthen/Shorten, Roughening the Surface'. We used a density of `0.75` to granulate the sound, creating tiny gaps between grains. Output: `flexgrainy.wav`

ST2. Create a richer granular texture

**SS: Soundfiles - Grain - 'Whole-grain'**
**SL: BRASSAGE > 'full monty'**

Now we'll run MODIFY BRASSAGE, Mode 7 ('Whole-grain'), using `flexgrainy.wav` as the input soundfile. File-Load (Global) Preset collection `wcc.dat` and use the Preset `flexbrassage`. This is what we did before, but now we are using `flexgrainy.wav` instead of `flex.wav`.

**SL:** Ensure that `flex.wav` is in Chosen Files, and that the breakpoint files listed below are on the Workspace (they should be there already as a result of Opening and grabbing the files from the directory in which you've placed the files for this Worksheet). Go to **BRASSAGE > full monty**. Then LOAD the `flexbrassage` patch.

All of the following data should now be in place -- it was made earlier and saved as a Preset / Patch. velocity (lo & hi) = 0.25 (i.e., 4x timestretch). In BRASSAGE the pitch shift parameter is transposition in semitones.

| Density = `flexdens.brk` | Grainsize = `flexgsize.brk` | Pitchshift = `flexpchlo.brk` | Pitchshifthi = `flexpchhi.brk` |
|---|---|---|---|
```
0.0    75                   0.0    30                    0.0   -7                      0.0    7
1.0     2                   1.0   100                    1.0   -1                      1.0    1
6.0   100                   6.0    60                    6.0  -11                      6.0   11
```

- amp = 0.5, amphi = 1
- space = 0.5, spacehi = 1
- all splices = 10ms
- Output: `flexgybr.wav` ('flex-grainy-brassage')

ST4. Cut out the portion we want to use

**SS: Edit/Mix > Edit > Cut**
**SL: EDIT > cutout & keep > time in seconds**

`Flexgybr.wav` is much longer than we need, so we will CUT it. Tick Play FROM-TO and enter 20.5 as the time to start and 26.1 as the time to end. Check that this sounds OK and adjust if it doesn't. Now go to Edit/Mix - Edit - Cut. The edit times will be in place. Name the outfile `flexgybrc.wav`.

ST5. Smooth the edges of the amplitude envelope (at start and end of the sound)

**SS: Soundfiles - Envelope - Dovetail**
**SL: ENVELOPE > dovetailing > standard**

Because we're going for a smooth result, we will envelope the beginning and end of the sound using Mode 2 ('steeper slope') and entering `0.5` both for the beginning and the end of the sound. Output: `flexgybrcdt.wav`.

Now we have a 6.6 sec. rich, grainy sound with soft edges, plus some pulses from the klangs of the flexatone.


## Time Domain Tuning

## B. Filter Userbank - Tune to Create Major and Minor Chords

ST6. Tune this new sound, `flexgybrcdt.wav`, to a C-major chord, then a C-minor chord

**SS: Soundfiles > Filter > Userbank**
**SL: FILTER > userbank > bands as midi**

The input is `flexgybrcdt.wav`. When we go to USERBANK we find that we need to supply a data text file containing the chord (harmony) we want to use. FILTER VARIABLE and FILTER BANKFRQS used preset options. Here we can design any harmony that we would like, even microtonal ones, using specific frequencies or Midi Pitch Values with decimal places. Using Midi Pitch Values (Mode 2) we define our major and minor chords -- see the Chart of Equivalent Pitch Notations (`Notechrt.htm`) in these USERBANK filter data files. 0dB is full amplitude. We can get away with this because the Q is fairly low.

`flexgyCmaj.txt`:
```
36    0dB       (v. low C)
43    0dB       (v. low G)
48    0dB       (low C)
55    0dB       (low G)
64    0dB       (E-natural is 64)
67    0dB       (G above middle C)
72    0dB       (high C)
76    0dB       (E-natural is 76)
79    0dB       (G above high C)
```

`flexgyCmin.txt`:
```
36    0dB       (v. low C)
43    0dB       (v. low G)
48    0dB       (low C)
55    0dB       (low G)
63    0dB       (E-flat is 63)
67    0dB       (G above middle C)
72    0dB       (high C)
75    0dB       (E-flat is 75)
79    0dB       (G above high C)
```

Click in the 'Filter Data' dialogue and enter the data or Load the file if already made. **SS:** If entering the data, name the output text file first. Then enter the first line, with amplitude information, Copy it to the clipboard and Paste it back as many times as you need notes. Now you can quickly edit the pitch values. SAVE. **SL:** name the file after making it.

We also need 'Q' (filter sharpness) data, and we'll vary this over time. The values chosen below allow a little bit of the roughness of the original sound to come through. This is deliberate! But you can have a purer harmonic result if you use higher values, e.g., 150+.

`flexgyQ.brk`:
```
0.0    15
2.6   100
5.2    50
```

The Gain parameter is important here. Our Q is still high enough to remove a good part of the signal. A gain of 150 seemed to work.

Also tick 'Double filtering'.

Use `flexgyCmaj.txt` to create the C-major tuning, naming the output `flexgybrcdtfb1.wav`.

Now re-open `flexgybrcdt.wav` and run with the note data file `flexgyCmin.txt` to create the C-minor tuning, naming the output `flexgybrcdtfb2.wav`.

Both of these peter out too soon, so we can CUT some silence from their ends with **Edit/Mix-Edit-Cut** starting at 0 and ending at 6. Add 'c' to both filenames. Thus we have `flexgybrcdtfb1c.wav` and `flexgybrcdtfb2c.wav`.


## C. Texture Simple (Mode 4) - Create a Flowing Texture, Using These Major and Minor Sounds

ST7. Replicate these two sounds on a C-based harmonic grid (using a Preset)

**SS: Soundfiles > Texture > Simple > changing harmonic set**
**SL: TEXTURE > simple > over harmonic sets**

The first thing we need to do is create a 'note data file' containing a reference pitch and the harmonic grid. Presets (one for major and one for minor) are stored in `wcc.dat` (which we've already loaded). When you open them in the TEXTURE SIMPLE dialogue, you will see contents of the note data files `ndf60hsarp1.txt` (major) and `ndf60hsarp2.txt` (minor, when you come to make the second texture), shown here with extra annotations:

```
60              (MIDI Pitch Value transposition reference tone)
#9              (informs the program that there are 9 lines in the grid)
0 1 55 0 0      (contents of the harmonic grid -- the first column is
1 1 60 0 0       time, with the times changing on each line. This
2 1 64 0 0       is why Mode 4 is used: 'Changing harmonic set'. The
3 1 67 0 0       third column has the MIDI Pitch Values for the grid.
4 1 72 0 0       Note that it sweeps upwards then rocks downwards)
5 1 60 0 0      (The 4th & 5th columns are not used and contain 0's)
6 1 67 0 0
7 1 48 0 0
8 1 55 0 0
```

The other parameters are:
- output duration = 10 sec.
- 'Use whole sound' is ticked
- packing is 1 (second apart)
- scatter is 0.1
- timegrid = 0
- sounds are 1 and 1 (only one input sound)
- gain (velocity) is 74 and 104
- durations are 1 and 1 (ignored because 'use whole sound' is ticked)
- amp = 1 (full), pos = 0.5 and space = 1 (full stereo spread)

Be sure 'Changing harmonic set' (Mode 4) is ticked, because the Preset may not retain this information.

Again, this is done twice, once for major and once for minor:

**MAJOR:** infile is `flexgybrcdtfb1c.wav`, Preset is `Mode4arpegmaj`, Note data file is `ndf60hsarp1.txt`. Name the output `flexgybrcdtfb1ctxarp.wav`.

**MINOR:** infile is `flexgybrcdtfb2c.wav`, Preset is `Mode4arpegmin`, Note data file is `ndf60hsarp2.txt`. Name the output `flexgybrcdtfb2ctxarp.wav`.

Both of these outputs have 'dead space' at the end, and are too quiet, so we CUT and GAIN:

CUT 0 to 18.2 (**Edit/Mix-Edit-Cut**) both files and GAIN (**Edit/Mix-Loudness-Gain**) with a gain factor of 2.5. (Two operations: add the 'c' in the CUT operation, input this file into Gain and add the 'g'). Now we have: `flexgybrcdtfb1ctxarpcg.wav` and `flexgybrcdtfb2ctxarpcg.wav`.

Note how the whole filterbank-harmonised chord now moves up and down on the pitches specified in the Texture note data file (major and minor versions).


## D. Submix Mix - Create a Passage of Music That Alternates These Major and Minor Textures

**SS: Edit/Mix > Mix > Create mixfile**
**SL: MIX > create a mixfile > superimposed**

ST8. At this point, we won't go into creating a mixfile from scratch, but will open an existing mixfile. First go to **SS: Edit/Mix > Mix > Create mixfile**. Select 'File-Open mixfile' (top left part of the screen) and open `arpmajmindt.mix`. It looks like this -- and won't work if your soundfile names are not the same. If there is a discrepancy or the program doesn't run or produce its output soundfile, edit and re-save this mixfile if you need to adjust anything, or rename the soundfiles to match the mixfile.

**SL:** In Sound Loom, first make sure all the files you are going to mix are in your SOURCE DIRECTORY and the mixfile, `arpmajmindt.mix` is both on the Workspace and on the left panel as the 'chosen file'. Then go to **SL: MIX > MIX WITH MIXFILE** and run the mix.

NB: If you should find that the mixfile as below does not run for you, you may need to alter the path to the soundfiles, matching the directory where you are working. For example if I were in directory `e:\aesndwork`, I would put: `e:\aesndwork\flexgybrcdtfbctxarpcg.wav`.

```
[soundfile names with .wav extension]          time  chans  L level  L pan  R level  R pan
c:\cdpws\flexgybrcdtfbctxarpcg.wav              0     2      0.8    -1.0    0.8     -0.5
c:\cdpws\flexgybrcdtfb2ctxarpcg.wav             9     2      0.8    -0.5    0.8      0
c:\cdpws\flexgybrcdtfbctxarpcg.wav             15     2      0.8     0      0.8      0.5
c:\cdpws\flexgybrcdtfb2ctxarpcg.wav            19.5   2      0.8     0.5    0.8      1
```

Now select **SS: Mix with mixfile** in the top left part of the screen or **SL: Use mixfile** and name the output `flexarpmajmindt.wav`. (You might replay `flex.wav` as a reminder of where all this began.)


## Spectral Domain Tuning

## E. Pitch Tune - Tuning in the Spectral Domain

**SS: Spectral - Freq\Pitch > Tune > MIDI pitches mode**
**SL: PITCH:HARMONY > tune spectrum > tunings as midi**

ST9. Tune a sound by moving its partials to user-defined pitch levels. We need to go back to `flexgybrcdt.wav` and convert it to an analysis file, but we would find that BRASSAGE created a Stereo file and this function will only handle Mono files.

Therefore we have first to convert the Stereo file to a Mono file: **SS: Edit/Mix - Channels - Stereo to Mono** / **SL: CHANNELS > extract/convert channels > convert stereo to mono**. You will notice that SS automatically adds an 'm' (for Mono) to the end of the soundfile name, so you just have to click on OK. In SoundLoom, name it with an 'm' at the end.

Now run **SS: Spectral > Convert - Analyse** / **SL: PVOC > analysis** with the new, mono soundfile. This gives us `flexgybrcdtm.ana`.

**SS: Spectral - Freq\Pitch > Tune > MIDI pitches mode**
**SL: PITCH:HARMONY > tune spectrum > tunings as midi**

As with the other tuning programs, we need to specify the chord/harmony we would like to have. It is particularly important in the spectral domain to start with a rich sound that covers a wide spectrum (i.e., both low and high partials/frequencies) -- otherwise, there may be nothing near the pitch levels we want and the program will struggle and produce unwanted artefacts, heard as strange whispery and gliding noises.

Let's make the diminished chord C-Eb-F#-A spread out in open position: C-F#-Eb'-A' (the ' means that these notes are in the next octave higher), in MIDI pitch values, `60 66 75 81`. This information is provided to the program in a data file: `flextune.tun`. Either create or Open this file if already present, leave the other parameters at their defaults (focus = 1, clarity = 1, trace = 1).

Name the output sound `flexgybrcdtmtune.ana`, convert back to a soundfile (**Spectral-Convert-Synthesise**) naming the output `flexgybrcdtmtune.wav`.

Now delete the analysis file, which is taking up space and isn't needed any more. Notice that this sound allows quite a bit of the roughness of the original to come through, even though the focusing parameters are at their maximum. The result is 'cleaner' if purer pitched tones are used as the input.

Save your History now.


## F. Repitch Transpose - Tune by Multiple Transpositions and Mixing

**SS: Spectral - Freq\Pitch > Transpose > Semitones mode**
**SL: REPITCH > TRANSPOSE > transpos in semitones**

For time reasons, the section below is for your information only. Just read the text and play the soundfile outputs: `flexcimchord.wav`, `capmvoices.wav` and `capmvoicesoffset.wav`.

ST10. Another way to go is to transpose the original sound several times, once for each pitch required, without changing its length. If the inputs are fairly clear pitches, this will produce a strong harmony; if they are more noise-laden, the process will produce a rich texture. Voices do something else again, as illustrated below. As we have seen before, this is done in the Spectral Domain with REPITCH TRANSPOSE (or TRANSPOSEF if we want to keep the formants of a vocal sound).

This time, we can make the same diminished chord, but with its tones within the same octave (a 'closed' position): C-Eb-F#-A. If we take Eb to be about right for the original tone, we go down 3 semitones to transpose `flexgybrcdtm.ana` to C, up 3 semitones to transpose to F# and up 6 semitones to transpose to A.

Thus we run this transposition function three times, creating `flexgybrcdtmd3.ana`, `flexgybrcdtmu3.ana`, and `flexgybrcdtmu6.ana`.

Using Last infile and Last process enables you to do this in about 20 seconds. Each of these have to be converted to soundfiles and the analysis files deleted, for good hard disk housekeeping (analysis files take up a great deal of space).

Now we MIX `flexgybrcdtm.wav`, `flexgybrcdtmd3.wav`, `flexgybrcdtmu3.wav`, and `flexgybrcdtmu6.wav` together to create the chord. The mixing operation is as above, or you could do it in an audio sequencer (though it might be harder to get the start times exact).

**SS: Edit/Mix - Mixfiles - Create mixfile.** Click on 'Create mixfile' in the top left corner, then Open `flexdimchord.mix` (File-Open mixfile in very top left corner), then select 'Mix with mixfile'. **SL: MIX > MIX WITH MIXFILE**, having placed `flexdimchord.mix` in CHOSEN FILES. Name the output `flexdimchord.wav` and Run.

ST11. Another application of this particular technique is to create beat patterns (if clear pitches are involved) or fuller sounds / fuzzy voices by making transpositions that are close to each other. Try running this again with `capm.ana`, transposing upwards 2 semitones and downwards 2 semitones. Mode Semitones (Fmts:Pch), entering the transposition value (number of semitones) and set Bands = 4, naming the outputs as usual (add u2 or d2). Convert to soundfiles as above and delete the analysis files.

The original and these two outputs are then combined in a MIX operation. Use Soundshaper as above and open the mixfile (**SS: File-Open mixfile** in very top left corner) `capmvoices.mix` etc. The output will be named `capmvoices.wav`.

```
[soundfiles          time  chans  level  pan]
c:\cdpws\capmd2.wav    0     1     0.5   -1
c:\cdpws\capm.wav      0     1     0.4    0
c:\cdpws\capmu2.wav    0     1     0.5    1
```

Listen to `capmvoices.wav`. Notice that the output is spread across the full stereo field, from left to right.

Another change is introduced by giving the soundfiles in the mix a little temporal offset. As in the mixfile below, `capmvoicesoffset.mix`, we hear the lowest voice on the left, then the original voice a tenth of a second later in the middle, and the highest voice another tenth of a second later on the right.

```
[soundfiles              time  chans  level  pan]
c:\cdpws\capmd2.wav       0     1     0.5   -1
c:\cdpws\capm.wav         0.1   1     0.4    0
c:\cdpws\capmu2.wav       0.2   1     0.5    1
```

Name the output `capmvoicesoffset.wav`. Note that the way the voices bounce around is different from a reverb / echo treatment.


## Summary and Playlist

### Prepare the Soundfile

| File | Description |
|------|-------------|
| `flex.wav` | Original input |
| `flexgrainy.wav` | MODIFY BRASSAGE, Mode 5 density 0.5 |
| `flexgybrcdt.wav` | MODIFY BRASSAGE, Mode 7 with Preset `flexbrassage`, then CUT and DOVETAILED |

### Time Domain Tuning

| File | Description |
|------|-------------|
| `flexgybrcdtfb1c.wav` | FILTER USERBANK with `flexgyCmaj.txt` to tune to a C-major chord, CUT 0 to 6 sec. |
| `flexgybrcdtfb2c.wav` | FILTER USERBANK with `flexgyCmin.txt` to tune to a C-minor chord, CUT 0 to 6 sec. |

### Texture

| File | Description |
|------|-------------|
| `flexgybrcdtfb1ctxarpcg.wav` | TEXTURE SIMPLE with Preset `Mode4majarpeg` and `ndf60hsarp1.txt` (C-major grid), CUT 0 to 18.2 and GAIN x 2.5 |
| `flexgybrcdtfb2ctxarpcg.wav` | TEXTURE SIMPLE with Preset `Mode4minarpeg` and `ndf60hsarp2.txt` (C-minor grid), CUT 0 to 18.2 and GAIN x 2.5 |

### Mix

| File | Description |
|------|-------------|
| `flexarpmajmindt.wav` | SUBMIX MIX, alternating the major and minor textures, using the mixfile `arpmajmindt.mix` |

### Spectral Domain Tuning

| File | Description |
|------|-------------|
| `flexgybrcdtmtune.wav` | PITCH TUNE using `flextune.tun` (Spectral menu) |
| `flexdimchord.wav` | REPITCH TRANSPOSE (transpose without changing the duration) and MIX the results |

### Mix (Voices)

| File | Description |
|------|-------------|
| `capmvoices.wav` | SUBMIX MIX, voices tuned 2 semitones above and below the original, using the mixfile `capmvoices.mix` |
| `capmvoicesoffset.wav` | SUBMIX MIX, voices tuned 2 semitones above and below the original, with start time offset, using the mixfile `capmvoicesoffset.mix` |
