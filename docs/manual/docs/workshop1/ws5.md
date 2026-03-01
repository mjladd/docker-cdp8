# Workshop 5: Time - Holding & Stretching

*by Dr Archer Endrich*

CDP Sound Transformation Explorations - Worksheet 5

The locations of the processing functions are shown under the program name.
- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

Input soundfile: [flex.wav](../../sounds/CSRCWS03SF01flex.wav) (6.6 sec., 44100 SR, Mono) converted to `flex.ana`.
Also needed: `flexfrz1.txt`, `flexfrz2.txt`, and `holdtimes.txt`.


## A. FOCUS FREEZE - Freeze the Spectrum Both Forward and Backward from Time Points

**SS: Spectral > Time > Freeze** / **SL: FOCUS > freeze > amps & frqs**

### ST1. Symmetric freeze lengths

Input: `flex.ana` → Output: `flexfrz1.ana`

The letter 'a' is used for a freeze-forward.  The spectrum is frozen until the next time.
The letter 'b' is used for a freeze-backward.  The spectrum at that time is frozen, but playback begins at the previous time.

The text file with the freeze data is `flexfrz1.txt` as follows:

```
;COMMENTS - FREEZES CREATE A LENGTHEN-SHORTEN SYMMETRY
 0.0    ;start normally
a0.5    ;freeze until 0.75, i.e., for 0.25 sec.
 0.75   ;unfreeze, now normal playback until 1.0
a1.0    ;freeze until 1.5, i.e., for 0.5 sec.
 1.5    ;unfreeze, now normal playback until 2.0
a2.0    ;freeze until 3.0, i.e., for 1.0 sec.
 3.0    ;unfreeze -- would be normal playback. BUT:
b4.0    ;freeze at 4.0 but start it at 3.0, i.e., for 1.0 sec.
 5.0    ;unfreeze -- would be normal playback, BUT
b5.5    ;freeze at 5.5, but start at 5.0, i.e., for 0.5 sec.
 6.0    ;unfreeze -- would be normal playback, BUT:
b6.25   ;freeze at 6.25, but start at 6.0, i.e., for 0.25 sec.
 6.6    ;unfreeze, end of file
```

Note that the comments in the file are only in this document; they are invalid in the file you actually use.

Name the output `flexfrz1.ana`, convert to a soundfile `flexfrz1.wav` and delete the analysis file.

### ST2. Short adjacent freezes and one long one

Input: `flex.ana` → Output: `flexfrz2.ana`

The text file with the freeze data is `flexfrz2.txt` as follows:

```
;COMMENTS - short, adjacent freezes, 1 backwards freeze and a long
;  forwards freeze
 0.0    ;these comments are invalid in the actual file
a0.5    ;freeze forward
 1.0    ;unfreeze
a1.2    ;freeze forward (very short)
 1.5    ;unfreeze
a1.75   ;freeze forward (short, adjacent freezes)
a2.0    ;freeze forward
a2.25   ;freeze forward
 2.5    ;unfreeze
b3.0    ;freeze sound at 3.0 but start playing it at 2.5
a3.5    ;freeze forward (long: held for 2 sec.)
 5.5    ;unfreeze
a5.6    ;freeze forward
 6.6    ;unfreeze, end of file
```

Name the output `flexfrz2.ana`, convert to a soundfile `flexfrz2.wav` and delete the analysis file.

This process is admittedly fiddly, but it does provide a facility to create patterns with changing lengths.


### Special Examples of Developed Applications

It is also possible to pick out an interesting moment in the sound, freeze it for a while, and then later CUT out that portion and use it. Here we make textures with the frozen portion.

**Example** (optional - you can just play the outputs provided - intermediate steps have not been saved):

1. CUT `flexfrz2.wav` from 3.5 to 5.5, making `flexfrz2c.wav`
2. DOVETAIL `flexfrz2c.wav` Mode 2 (steeper slope), 0.3 sec at start and 0.5 sec at end, making `flexfrz2cdt.wav`
3. TEXTURE SIMPLE with `ndf60.txt` (just the number 60 in it), 20 sec. output duration, 'Use whole sound' ticked. Packing = 0.5 sec, Scatter = 0.23 sec, Amplitude 64-84, Pitches 58 min and 62 max, Attenuation 0.9, Position = 0.5, Spread = 1. Output name is `flexfrz2cdttx1.wav` (processing of this one continues)

   (For a more edgy result, try this with Packing = 0.1 sec, Scatter = 0.06 sec., and the pitches ranging from 48 to 72 -- a 2-octave spread, making `flexfrz2cdttx2.wav`. This one is saved for Playback.)

4. MODIFY SPEED, semitones mode to lower it by 2 octaves (24 semitones), making `flexfrz2cdttx1d24.wav` (this brings out the latent harmony)
5. CUT `flexfrz2cdttx1d24.wav` (it is rather long) from (e.g., 28 to 51 sec.-- check with 'Play using markers'), making `flexfrz2cdttx1d24c.wav`
6. DOVETAIL Mode 1, 1 sec at start, 2 sec. at end to smooth the edges (0 = Linear fades), making `flexfrz2cdttx1d24cdt.wav` (This one is saved for Playback).


## B. FOCUS HOLD - Prolong Portions of Spectrum, Defining When and For How Long

**SS: Spectral > Time > Hold** / **SL: FOCUS > hold**

### ST3.

Input: [CSRCWS02SF01trcdt.wav](../../sounds/CSRCWS02SF01trcdt.wav) (converted to `trcdt.ana`) → Output: `trcdthold.ana`

`holdtimes.txt` (left col is time to start holding, right col is for how long; file has a compression pattern, i.e., hold durations get gradually smaller):

```
0.35    2.0
1.0     1.8
1.5     1.6
2.0     1.4
2.5     1.2
3.0     1.0
3.5     0.9
4.0     0.8
4.5     0.7
5.0     0.6
5.5     0.5
6.0     0.4
6.5     0.3
6.7     0.2
6.9     0.1
```

Name output `trcdthold.ana`, convert to soundfile `trcdthold.wav` and delete the analysis file.

### ST5.

Let's try the second of these with [capm.wav](../../sounds/CSRCWS01SF01capm.wav), first reducing it to a 'trace' of its former self.

Input: `capm.ana` → First let's run it thru HILITE TRACE

**SS: Spectral > Amplitude > Trace** / **SL: HIGHLIGHT > tracery > trace all**

Retain only the 10 loudest channels. Output: `capmtra10.ana`

Now use `capmtra10.ana` as the input for FOCUS HOLD, with `holdtimes.txt`:

```
0.35    2.0
1.0     1.8
1.5     1.6
2.0     1.4
2.5     1.2
3.0     1.0
3.5     0.9
4.0     0.8
4.5     0.7
5.0     0.6
5.5     0.5
6.0     0.4
6.5     0.3
6.7     0.2
6.9     0.1
```

Name the output: `capmtra10hold.ana`, convert to soundfile `capmtra10hold.wav` and delete the analysis file.


## C. FOCUS STEP

**SS: Spectral > Time > Stepfreeze** / **SL: FOCUS > step through**

Input sounds: [trcdt.wav](../../sounds/CSRCWS02SF01trcdt.wav) Convert to analysis file: **Spectral-Convert-Analyse** → `trcdt.ana`
[flex.wav](../../sounds/CSRCWS03SF01flex.wav) Convert to analysis file: **Spectral-Convert-Analyse** → `flex.ana`

### ST5. Small step

Input is `trcdt.ana`. Step = 0.1 sec. Output is `trcdtstp&1.ana` (mechanical churning)
Convert to `trcdtstp&1.wav` and delete the analysis file

### ST6. Larger step

Input is `flex.ana`. Step = 0.25 sec. Output is `flexstp&25.ana` (a bit tuneful)
Convert to `trcdtstp&25.wav` and delete the analysis file


### Special Example of Developed Application

**Example** (optional: you can just play the final result provided)

OK, let's develop this idea further with a series of operations on [flex.wav](../../sounds/CSRCWS03SF01flex.wav):

1. MODIFY BRASSAGE, Mode 2 (timestretch) with `grnvelocity.brk` as follows:
   ```
   0.0     0.25
   3.0     2
   6.6     0.25
   ```
   This draws out the wavy resonances in the sound. Output name: `flextvgrn.wav`

2. Convert to an analysis file and STRETCH TIME x2. Output name: `flextvgrnx2.ana`

3. Now use stepfreeze: FOCUS STEP, step = 0.25. Output name: `flextvgrnx2stp&25.ana` (the steps pick out different pitch levels)

4. It's tinny, so we can warm it up with the blur+trace combination (BLUR BLTR). Blur = 100 windows, Trace = 20 channels. Output name: `flextvgrnx2stp&25bltr.ana`. Convert to a soundfile in preparation for the last step: `flextvgrnx2stp&25bltr.wav`

5. `flextvgrnx2stp&25bltr.wav` is now the input for MODIFY SPEED, semitones mode, down 24 semitones (2 octaves: -24). Output name: `flextvgrnx2stp&25bltrd24.wav`. This gives us a ca 55 sec. smoothly changing, somewhat ethereal sound.


## D. STRETCH TIME - Spectral Time-Stretching

**SS: Spectral > Stretch > Time** / **SL: STRETCH > time > do time-stretch**

### ST6. A timestretch 64 times longer in several steps

Source: [capm.wav](../../sounds/CSRCWS01SF01capm.wav)

CUT [capm.wav](../../sounds/CSRCWS01SF01capm.wav) from 0 to 1.9 sec to form [capmc.wav](../../sounds/CSRCWS05SF02capmc.wav). Convert to `capmc.ana`

Now it is stretched x2 repeatedly, using LAST PROCESS and renaming the output. Each time the previous stretch is doubled (the Timestretch = 2 is left in place). At the end, each of the analysis files is converted to a soundfile and the analysis files deleted. Output names: `capmcx2.ana`, `capmcx4.ana`, `capmcx8.ana`, `capmcx16.ana`, `capmcx32.ana` and `capmcx64.ana`. The last one is CUT from 16 sec to the end (after converting to a soundfile) to eliminate the accumulated silence at the start.

The whole sequence of outputs is therefore: [capmc.wav](../../sounds/CSRCWS05SF02capmc.wav) → `capmcx2.wav` → `capmcx4.wav` → `capmcx8.wav` → `capmcx16.wav` → `capmcx32.wav` → `capmcx64.wav` → `capmcx64c.wav`


## Worksheet 5 - TIME: HOLDING & STRETCHING - Summary and Main Outputs

### Freeze forwards & backwards

| File | Description |
|------|-------------|
| `flexfrz1.wav` | FOCUS FREEZE - using `flexfrz1.txt` to create a pattern of symmetric lengths |
| `flexfrz2.wav` | FOCUS FREEZE - using `flexfrz2.txt` to create a pattern of varying lengths, often adjacent |

### Examples of developed applications

| File | Description |
|------|-------------|
| `flexfrz2cdttx1d24cdt.wav` | CUT a frozen portion & DOVETAIL + TEXTURE with a narrow pitch range + down 2 octaves and tidied up with a CUT & DOVETAIL. |
| `flexfrz2cdttx2.wav` | CUT a frozen portion & DOVETAIL + TEXTURE with a 2 octave pitch range. This leaves a high, somewhat piercing but rich sound, changing mechanically. |

### Freeze (hold) for specified lengths

| File | Description |
|------|-------------|
| `trcdthold.wav` | FOCUS HOLD - using `holdtimes.txt` to create a pattern of compressing lengths |
| `capmtra10hold.wav` | FOCUS HOLD - the same again, with a different input, first 'reduced' to 10 analysis channels |

### Stepfreeze (regular, cannot time-vary)

| File | Description |
|------|-------------|
| `trcdtstp&1.wav` | FOCUS STEP - mechanical churning with a regular stepfreeze of 0.1 sec. |
| `flexstp&25.wav` | FOCUS STEP - the flexatone becomes a bit tuneful with a stepfreeze of 0.25 sec. |

### Example of a developed application

| File | Description |
|------|-------------|
| `flextvgrnx2stp&25bltrd24.wav` | Time-varying timestretch granulation + spectral timestretch + stepfreeze 0.25 + blur (100 windows) & trace (20 analysis channels) + down 2 octaves. Here we've stretched two different ways before the stepfreeze, then smoothed with BLTR and enriched the (still high) sound by lowering it. Notice how you can read the whole processing sequence in the name: **flex** → **t**ime-**v**arying **gr**ai**n**s → stretched **x2** → **st**e**p** 0.**25** → **bl**ur-**tr**ace → **d**own **24** semitones. |

### Stretch Time

(a massive 64 times -- and some sounds benefit from even more -- there is no limit)

| File | Description |
|------|-------------|
| [capmc.wav](../../sounds/CSRCWS05SF02capmc.wav) | Starting point: "The extravehicular momentum shields" |
| `capmcx2.wav` | The words are elongated. |
| `capmcx4.wav` | A real drawl. |
| `capmcx8.wav` | Getting silly. |
| `capmcx16.wav` | Very drawn out now, and we especially notice the consonants as sound objects in themselves. They in turn can become useful source material. |
| `capmx32.wav` | Becoming abstract. |
| `capmx64c.wav` | Extremely slow and abstract, with accumulated silence at the beginning removed. |
