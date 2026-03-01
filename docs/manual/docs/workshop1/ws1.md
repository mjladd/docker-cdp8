# Workshop 1: Ways to Alter Pitch Levels

*by Dr Archer Endrich*

CDP Sound Transformation Explorations - Worksheet 1

The locations of the processing functions are shown under the program name.
- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

The input sound for this Worksheet: [capm.wav](../../sounds/capm.wav) 7.2 sec., Mono.

Copy `capm.wav` to your working directory as well as these additional files: `grntr.txt`, `ss.brk`, `Trchng.brk`, `Trpw.brk`, `tvtrd.brk`, `trtvu.brk` and `wcc.dat`. (The source sound on the CD is named `CSRCWS01SF01capm.wav`. This strange name is to ensure that the source sounds come last on the CD. Having copied these Worksheets data to your hard disk, you can Open the source sounds as is, or rename them to e.g., just `capm.wav`, naming the outputs as shown on the Worksheets.)

## CDP Mixfiles and Paths - Important!

I suggest that you name your working directory `\cdpws` (e.g., `c:\cdpws`) and copy the files for each Worksheet to it in turn as you work through them. Untick Read-only when you need to edit `.txt` or `.brk` files (if you find that it's on). Then the mixfiles will run as is (they have this path in them). Otherwise you will have to edit the paths to match your setup. For example, if you want to work on Drive D:, change the path in the mixfiles to `d:\cdpws`, or `d:\myfolder`.

In Sound Loom, mixfiles will work without a specified path for the soundfiles only if the soundfiles are in the 'base directory', i.e., the directory where `soundloom.tcl` is placed. In Soundshaper, if mixfiles and their soundfiles are in the current directory, they may work without the path, but operation will be more reliable with a specified path.

Soundshaper Presets and Sound Loom Patches should be path independent.

**SS:** Open Soundshaper, open your working directory in OPTIONS/SETTINGS and Open [capm.wav](../../sounds/capm.wav). (Suggestion: save `cdpws.cfg` as a Personal directory that you can re-open.)

**SL:** In Sound Loom, Select your working directory and GRAB [capm.wav](../../sounds/capm.wav) to the Workspace. Follow this basic procedure for the other Worksheets.


## A. Modify Speed - Simple Pitch Transposition, Using Semitones

**SS: Soundfiles > Pitch > Transpose/Speed > semitones**
**SL: PITCH: SPEED > pitch > tape transpose by semitones**

ST1. Up 12 semitones to form `capmu12.wav` (enter `12`). (When naming the output (before Running by clicking on 'OK'), take away the '_1' and add 'u12'. You don't have to add the `.wav` extension in Soundshaper)

ST2. Down 12 semitones to form `capmd12.wav` (enter `-12`)


## B. Modify Speed - Time-Varying Pitch Transposition, Using Semitones, to Create a Glissando

ST3. **SS:** Click on 'TV' in the transposition parameter box. This opens a text window. Open the file `tvtru.brk`, or write out the following breakpoint file and finally click on SAVE. (If you write the data first and then alter the name, the data will disappear and you'll have to start over, so get your name entered before the data.) Left col is time, Right col is transposition value in semitones.

**SL:** Click on 'Get File' to open `tvtru.brk`, or 'Make File' to enter (and save) the data.

```
0.0    0
7.2   12
```

Name as `capmtvtru.wav` and Run (click on 'OK').

ST4. Click on 'TV' in the transposition parameter box (Sound Loom: 'Get File' or 'Make File'). This opens a text window. **SS:** First name the file `tvtrd.brk`, then write out the following breakpoint file and finally click on SAVE (or just Open the one provided). **SL:** Make the file before naming and saving it, or just use 'Get file'.

```
0.0    0
7.2  -12
```

Name as `capmtvtrd.wav` and Run.


## C. Modify Speed - Time-Varying Pitch Transposition, Using Semitones, to Create Changing Levels (Both Instant and Sliding)

ST5. Click on 'TV' in the transposition parameter box. This opens a text window. **SS:** Open the file `trchng.brk`, **SL:** 'Get file'.

```
0.0    0
1.4    0
1.5    7
2.9    7
3.0    3
4.5    3
7.2   12   ;this one slides upward from time 4.5 to 7.2
```

Save the result as `capmtrchng.wav`


## D. Distort Pitchwarp - Pseudo-Wavecycle Distortion with Pitch Bend

**SS: Soundfiles > Distort cycles > Pitch warp**
**SL: DISTORT > pitch**

ST6. Go to the program, click on 'TV' in the Oct vary parameter and OPEN `tvtrpw.brk`. The left column is transposition values to parts of an octave (1 = 1 octave, 0.33 = 1/3rd of an octave, etc.).

```
0.0   0.02
1.4   0.02
1.5   0.58
2.9   0.58
3.0   0.33
4.5   0.33
7.2   1      ;this one slides upward from time 4.5 to 7.2
```

Now name the output `capmtrpw.wav`, and Run, using this breakpoint file but leaving the other parameters as they are.

ST7. For a more controlled result, try entering just the value `0.33` in the Oct vary parameter box. Name as `capmpw&33.wav` and Run.


## E. Granulation with Cycling Pitch Change

**SS: Soundfiles > Grain - Granulate SF**
**SL: BRASSAGE > brassage > granulate**

The input is `capm.wav`.

ST8. The granulation program Soundshaper calls with Granulate SF is MODIFY BRASSAGE, Mode 5 (Granulate). Use `0.9` as the density value (to separate the grains a little), and name the output `capmgrn&9.wav`.

**SS: Soundfiles > Grain > Pitch > cycle without repetition**
**SL: GRAIN > repitch > no grain repeats**

Now use `capmgrn&9.wav` as the input for **SS: Soundfiles > Grain > Pitch** / **SL: GRAIN > repitch > no grain repeats** to create a cycling grainy pitch change. Open/Get the TRANSPOSN DATA file `grntr.txt`, which should contain:

```
0 1 2 3 2 1 0 -1 -2 -3 -2 -1 0
```

(What shape does this make?)

Make sure you have Mode 1 (SS: 'Cycle without repetition', SL: 'no grain repeats'), Click on the 'Ignore last grain' box, name the output `capmgrn&9m1.wav` and Run.

ST9. Go to this function again ('Last Infile' 'Last Process'), name the output `capmgrn&9m2.wav`, select Mode 2 ('Repeat grains') and Run.


## F. Transposition Without Changing the Length of the Soundfile

**SS: Spectral > Freq/Pitch > Transpose**
**SL: REPITCH > TRANSPOSE > transpos in semitones**

First we need to create an 'analysis file': Open [capm.wav](../../sounds/capm.wav) and go to **SS: Spectral - Convert - Analyse** and click on 'ANALYSE'. **SL: PVOC > analysis**. The result is automatically named `capm.ana`.

ST10. Now we go to the Spectral Transposition program (**SS: Spectral > Freq/Pitch > Transpose** / **SL: REPITCH > TRANSPOSE > transpos in semitones**) and enter `12`, naming the output `capmspecu12.ana` and then `-12`, naming this output `capmspecd12.ana` (the `.ana` is put on automatically). When we go to play them, a special program PVPLAY is called that plays analysis files -- press the Space Bar to activate play.


## G. Transposition with a Frequency Split Point. Then Add Ring-Modulation and Echo.

**SS: Spectral - Freq/Pitch - Pitch shift (Transp) > pitch shift up**
**SL: PITCH: HARMONY > pitch shift > shift above frq divide**

The input is `capm.ana`.

ST11. We will use Mode 4 to shift above the split point ('Pitch shift up'). Enter `200` (Hz) as the frequency split point and `7` as the number of semitones to shift upwards. Name the output `capmtranspm4.ana`. Convert this to a soundfile with **SS: Spectral - Convert - Synth.** / **SL: PVOC > synthesis** and click on SYNTHESISE. It (automatically) becomes `capmtranspm4.wav`.

ST12. Now, using `capmtranspm4.wav` as the input, go to **SS: Soundfiles > Radical > Ring-Mod** / **SL: RADICAL > radical > ring modulate** and modulate with a Mod-Freq value of `10`. Name the output `capmtranspm4rm.wav`. (NB: in Sound Loom, you will first have to click on To Wkspace: New Files and, with CHOSEN FILES mode on, deselect the analysis file and select `capmtranspm4.wav` as the new file to process.)

It's a bit too quiet now, so use this output as the input to **SS: Edit/Mix > Loudness > Gain** / **SL: LOUDNESS > gain** and put in a value of `2` to double the volume. Name the output `capmtranspm4rmg.wav`.

ST13. Now go to **SS: Soundfiles > Reverb/Echo > Stadium Echo** / **SL: REVERB/ECHO > rev/echo > stadium** and put in these values:

| Parameter | Value | Notes |
|-----------|-------|-------|
| gain | 0.65 | compensate for overlap of echoes |
| roll-off | 1 | rate of amplitude reduction |
| size | 1 | time between echoes: your value multiplies 0.1, so here it is 0.1 |
| count | 15 | number of echoes |

Name the output `capmtranspm4rmste.wav`.


## H. Shift the Frequencies of the Spectrum in a Time-Varying Way

**SS: Spectral - Freq/Pitch - Shift freq**, which calls up STRANGE SHIFT
**SL: STRANGE > linear shift > shift all**

The input is `capm.ana` (SL: reselect in CHOSEN FILES mode).

ST14. Go to the program. To get a time-varying result, click on 'TV', name the file `ss.brk` and enter:

```
0.0      0
1.0   1000
3.99  1000
4.0    300
7.2      0
```

Click on SAVE. Name the output analysis file `capmtvssm1.ana`, ensure 'Shift whole spectrum' is ticked (Mode 1) and Run. Convert to a soundfile `capmtvssm1.wav`.


## I. Create Internal Glissandi in the Spectrum

**SS: Spectral > Amplitude > Accumulate**
**SL: FOCUS > accumulate**

The input is `capm.ana`.

ST15. Go to the program and enter these values:

| Parameter | Value |
|-----------|-------|
| decay | 0.01 |
| gliss | 0.01 |

Name the output `capmaccu&01.ana`. Convert back to a soundfile, forming `capmaccu&01.wav`.

End the Session with **SS: SAVE HISTORY** to keep a record of this work in your folder. A name such as `WS1-3-06-2005.hst` will indicate which Worksheet it documents. **SL:** you will find a log of your session saved in the `_userlog` folder.

Clean out (DELETE) all analysis files (`.ana`) and any soundfiles you don't want to keep.


## Summary and Playlist

| File | Description |
|------|-------------|
| `capm.wav` | Input soundfile for most operations below (7.2 sec. mono, male voice) |
| **Transpose** | |
| `capmu12.wav` | MODIFY SPEED - sound is raised 12 semitones (1 octave) |
| `capmd12.wav` | MODIFY SPEED - sound is lowered 12 semitones (1 octave) |
| `capmtvtru.wav` | MODIFY SPEED - sound slides upwards over time for 12 semitones according to breakpoint file `tvtru.brk` |
| `capmtvtrd.wav` | MODIFY SPEED - sound slides downwards over time for 12 semitones according to breakpoint file `tvtrd.brk` |
| `capmtrchng.wav` | MODIFY SPEED - sound moves both up & down according to contents of the breakpoint file `trchng.brk` sometimes sliding & sometimes instantaneous changes |
| **Distortion and pitch warping** | |
| `capmtrpw.wav` | DISTORT PITCHWARP - wavecycle distortion with time-varying sliding transposition across specified octaves or parts of octaves, using `trpw.brk` |
| `capmpw&33.wav` | DISTORT PITCHWARP - wavecycle distortion with a single fraction of an octave specified for the pitchwarp (the '&' in the name = a decimal point, so the value used here is 0.33 of an octave) |
| **Cycles of pitch transposition** | |
| `capmgrn&9.wav` | MODIFY BRASSAGE, Mode 5 - make a soundfile a little bit grainy |
| `capmgrn&9m1.wav` | GRAIN PITCH - previous sound is input for series of pitch transpositions that cycle round, according to the text data file `grntr.txt` |
| `capmgrn&9m2.wav` | The previous operation is done again, but using Mode 2 so that the whole cycle of transpositions is applied to each grain before moving on |
| **Transposition with internal split point** | |
| `capmtranspm4.wav` | REPITCH TRANSP - Spectral transposition upwards with frequency split point |
| **Ring modulation** | |
| `capmtranspm4rmg.wav` | MODIFY RADICAL, Mode 5 (ring modulation) is applied to the previous output and Gain is applied |
| **Multiple Echoes** | |
| `capmtranspm4rmgste.wav` | MODIFY REVECHO, Mode 3 (Stadium Echo) - the previous output is given multiple echoes, as if reflected off the walls of a large stadium |
| **Frequency shift** | |
| `capmtvssm1.wav` | STRANGE SHIFT - time-varying shift that squeezes the partials closer together as one goes higher, using `ss.brk` |
| **Internal glissandi** | |
| `capmaccu&01.wav` | FOCUS ACCUMULATE - spectral process that can create internal glissandi. The soundfile name indicates that 0.01 was used for the decay &/or gliss parameters |
