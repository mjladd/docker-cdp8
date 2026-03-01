# Workshop 2: Ways to Filter Sounds

*by Dr Archer Endrich*

CDP Sound Transformation Explorations - Worksheet 2

The locations of the processing functions are shown under the program name.
- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

Copy [trcdt.wav](../../sounds/trcdt.wav), `phasdlay.brk` and `eqint6.txt` into your working directory. Open your working directory in Soundshaper: **Tools - Options/Settings > File (in top left corner) > Open config file**. In Sound Loom, **FIND DIRECTORY > ANY DIRECTORY > SELECT**.

Analyse [trcdt.wav](../../sounds/trcdt.wav) to form `trcdt.ana` (**SS: Spectral > Convert > Analyse** / **SL: PVOC > analysis**).

### Acquiring Information About the Important Frequencies in Your Sound

**SPECINFO REPORT** - **SS: Info > Spectral > Report** / **SL: SPECTRAL INFO**

Using `trcdt.ana` as the infile, create a report about which frequencies are most prominent in the sound. This step is to illustrate acquiring background information about a sound.

SPECINFO REPORT Mode 3 (**SS:** 'Frequencies only' / **SL:** 'print freq peaks to file' > 'order by frequency & time') with **SS:** Freqs set to 4 and Peaks set to 4 / **SL:** formant bands = 4 and peaks to find = 4.

**SS:** The output is a text file based on the name of the Infile (named automatically).
**SL:** SAVE AS and name the output text file.

Having made the file, find it and drag it to a text editor. Note that 296Hz and 796Hz repeat frequently as the lowest (loud) frequencies shown. These are the key frequencies to go above and below when filtering the sound.

Now re-open `trcdt.wav` and use it as the infile for most of the following filtering operations. (A few ask for `capm.wav`.)

We are changing the 'tonal' qualities of the sound in various ways and for various reasons.


## A. Filter Lohi - Remove Top or Bottom Portions of the Sound

**SS: Soundfiles > Filter > Lohi**
**SL: FILTER > lopass/hipass > bands as frq**

ST1. Remove top portion of sound = low-pass

FILTER LOHI attenuation = `-12`, pass = `296`, stop = `400` -> `trcdtlo.wav`
(everything below 296 passes, and fades up to 400, above which everything is cut off)

ST2. Remove bottom portion of sound = high-pass

FILTER LOHI attenuation = `-12`, pass = `1000`, stop = `796` -> `trcdthi1.wav`
(everything above 1000 passes, and fades down to 796, below which everything is cut off)

Now try pass = `5000`, stop = `4796` -> `trcdthi2.wav`
(a thinner sound, with less of the lower frequencies -- nothing below 4796 Hz -- remaining, in fact, very quiet because not very much of the sound is left)


## B. Filter Variable - Band and Notch Functions

**SS: Soundfiles > Filter > Variable**
**SL: FILTER > variable > band & notch**

ST3. Filter a defined band (Mode 3: band-pass - cut away above & below a band around the frequency)

FILTER VARIABLE:
- acuity = `0.1` (sharpness of the filter fadeout across the frequencies)
- gain = `0.6` (the sharper the filter, the more resonance is caused, which can be heard as a pitched tone or a whistling sound; this resonance can cause overload, so the sharper the filter, the more the gain may need to be reduced. If you get a message reporting overload, delete the file BEFORE PLAYING IT and reduce the gain)
- band centre frequency = `796`
- output name: `trcdtbp1.wav`

Run again with:
- band centre frequency = `1796` (-> `trcdtbp2.wav`) and
- band centre frequency = `4796` (-> `trcdtbp3.wav`)

Compare these 3 outputs. This is how different tones can be made and then combined by mixing to get tonally varying textures.

ST4. Filter a defined notch (Mode 4: band-reject - cut away within a band around the frequency) - band-reject is also sometimes called a 'notch' filter

| FILTER VARIABLE | acuity | gain | frequency | output name |
|-----------------|--------|------|-----------|-------------|
| | 0.1 | 0.6 | 296 | `trcdtbr1.wav` |
| | 0.1 | 0.6 | 796 | `trcdtbr2.wav` |
| | 0.1 | 0.6 | 1796 | `trcdtbr3.wav` |

Again, compare the 3 outputs, noting tonal differences. Are the differences greater than with band-pass?


## C. Filter Bank - 6 Specialised Functions

**SS: Soundfiles > Filter > Bank**
**SL: FILTER > variable > bank**

ST5. Mode 1: Filter the harmonic series over a low frequency

| infile | outfile | Q | Gain | Lofreq | Hifreq | otherparam | double-filtering |
|--------|---------|---|------|--------|--------|------------|------------------|
| trcdt | `trcdtfbm1.wav` | 150 | 5 | 50 | 10000 | - | ON |

ST6. Mode 2: Filter alternate harmonics over a low frequency

| infile | outfile | Q | Gain | Lofreq | Hifreq | otherparam | double-filtering |
|--------|---------|---|------|--------|--------|------------|------------------|
| trcdt | `trcdtfbm2.wav` | 150 | 5 | 50 | 10000 | - | ON |

ST7. Mode 3: Filter the subharmonic series below a high frequency

| infile | outfile | Q | Gain | Lofreq | Hifreq | otherparam | double-filtering |
|--------|---------|---|------|--------|--------|------------|------------------|
| trcdt | `trcdtfbm3.wav` | 50 | 5 | 50 | 10000 | - | ON |

ST8. Mode 4: Filter the harmonic series, specifying a linear offset

| infile | outfile | Q | Gain | Lofreq | Hifreq | otherparam | double-filtering |
|--------|---------|---|------|--------|--------|------------|------------------|
| trcdt | `trcdtfbm4.wav` | 100 | 5 | 50 | 10000 | 796 (offset) | ON |

ST9. Mode 5: Filter with equal intervals, specifying number of filters

| infile | outfile | Q | Gain | Lofreq | Hifreq | otherparam | double-filtering |
|--------|---------|---|------|--------|--------|------------|------------------|
| trcdt | `trcdtfbm5a.wav` | 150 | 5 | 50 | 10000 | 1000 (num filters) | ON |
| trcdt | `trcdtfbm5b.wav` | 100 | 5 | 50 | 10000 | 100 | ON |

ST10. Mode 6: Filter with equal intervals, specifying interval size in semitones (harmonises!)

| infile | outfile | Q | Gain | Lofreq | Hifreq | otherparam | double-filtering |
|--------|---------|---|------|--------|--------|------------|------------------|
| trcdt | `trcdtfbm6a.wav` | 50 | 5 | 50 | 10000 | 3 (semitone size) | ON |
| trcdt | `trcdtfbm6b.wav` | 25 | 5 | 50 | 10000 | 12 | ON |
| trcdt | `trcdtfbm6c.wav` | 25 | 5 | 50 | 10000 | 7 | ON |
| trcdt | `trcdtfbm6d.wav` | 25 | 5 | 50 | 10000 | 6 | ON |


## D. Filter Phasing - Phasing Delay Effects

**SS: Soundfiles > Filter > Phasing**
**SL: FILTER > phasing > phase effect**

ST11. Mode 2 for stronger reverberant effects - single delay time (sounds like a small reverberant enclosed space)

| infile | outfile | Gain | Delay (ms) | Linear option |
|--------|---------|------|------------|---------------|
| capm | `capmphasm2-35.wav` | 0.25 | 35 | (not used) |

Try it again with a delay time of 500 ms. Output: `capmphasm2-500.wav`

ST12. Mode 2 for stronger reverberant effects - time-varying delay times (speaker moving from distant echoey to closer location)

| infile | outfile | Gain | Delay (ms) | Linear option |
|--------|---------|------|------------|---------------|
| capm | `capmphasm2tv.wav` | 0.25 | `phasdlay.brk` | (not used) |

Click on 'TV' in the Delay parameter to activate having a breakpoint file. `phasdlay.brk` is (if already made, just click in the edit box, then on Open):

```
0.0   300
1.5   200
3.0    80
4.5    80
6.0    40
7.0    20
```

The Linear option (not used here) seems to reduce the reverberant effect, make it 'cleaner'.


## E. Filter Iterated - Cumulative Filtering

**SS: Soundfiles > Filter > Iterated**
**SL: FILTER > iterated > bands as frq**

Use `capm.wav` as the infile. To get a shorter source file, use CUT (**SS: SFEDIT CUT** / **SL: cutout & keep > time in seconds**), `capm.wav` -> `capmblow.wav` from 5.8 to 7.2, which places "she's goin' to blow, captain" in a separate soundfile.

FILTER ITERATED requires a datafile, which can be made with FILTER BANKFRQS. Let's use Mode 6: 'equal intervals - interval in semitones' and give it 3 semitones between banks, 20 (low) to 5000 Hz (high) - name the output text file `eqint6.txt`.

`FILTER BANKFRQS capmblow.wav eqint6.txt 20 5000 3`, which produces a list of frequencies (the 0dB's are edited in afterwards: do one, then CUT -- copy to clipboard with Control-C -- & PASTE it with Control-V):

```
[frequency   level]
20.000000    0dB
23.784142    0dB
etc. until
4305.389646  0dB
```

ST13. Now we can use `eqint.txt` with FILTER ITERATED, Mode 1 (frequency in Hz)

| infile | outfile | Datafile | Mode | Q | Gain | Delay | OutDur |
|--------|---------|----------|------|---|------|-------|--------|
| capmblow | `capmblowit1.wav` | `eqint6.txt` | 1 | 50 | 5 | 0.25 | 20 |

There is also a randomised pitch shift in FILTER ITERATED. Try running it again with `capmblow.wav` with a Pitch Shift of 3 semitones, producing `capmvblowit6ps.wav`.


## F. Filter Sweeping - Filter Sweeps Through the Frequencies of the Sound

**SS: Soundfiles > Filter > Sweeping**
**SL: FILTER > sweeping**

Use `trcdt.wav` for the first two, and `capm.wav` for the last two.

Modes are: 1: Hi-pass (hard to get a result), 2: Lo-pass, 3: Band-pass, 4: Band-reject. (Seems best with Modes 2 (low-pass) and 3 (band-pass).) Acuity - (i.e., 'Q') - in this case lower values are tighter.

Wider frequency ranges seem to need lower Gain values. Watch for overloads. If overloads are reported, delete the outfile BEFORE playing it, reduce the gain and run again.

ST14.

| infile | outfile | mode | acuity | gain | lof | hif | sweepfrq |
|--------|---------|------|--------|------|-----|-----|----------|
| trcdt | `trcdtswm3.wav` | 3 (band) | 0.05 | 0.9 | 200 | 220 | 0.5 |

Mode 4 (Band-reject [Notch]) - (same parameters) - hear much more of the tractor.

Try Mode 2 (Lo-pass):

ST15.

| infile | outfile | mode | acuity | gain | lof | hif | sweepfrq |
|--------|---------|------|--------|------|-----|-----|----------|
| trcdt | `trcdtswm2.wav` | 2 (low) | 0.05 | 0.5 | 200 | 1000 | 0.1 |

and

ST16.

| infile | outfile | mode | acuity | gain | lof | hif | sweepfrq |
|--------|---------|------|--------|------|-----|-----|----------|
| capm | `capmswm2.wav` | 2 (low) | 0.05 | 0.3 | 200 | 1000 | 0.1 |

The same parameters with Mode 4 produce a more 'hollow' effect `capmswm4.wav`.


## Summary and Playlist

`trcdt.wav` - Main input soundfile for these processes (10 sec. mono, a tractor)

### Low-pass & High-pass

| File | Description |
|------|-------------|
| `trcdtlo.wav` | FILTER LOHI - low pass filter (cuts off the higher frequencies -- nothing above 400) |
| `trcdthi1.wav` | FILTER LOHI - high pass filter (cuts off the lower frequencies -- nothing below 796) |
| `trcdthi2.wav` | FILTER LOHI - high pass with a much higher stop frequency -- nothing below 4796 |

### Band-pass & Band-reject (Notch)

| File | Description |
|------|-------------|
| `trcdtbp1.wav` | FILTER VARIABLE - in band pass mode to retain a specified band of frequencies: 796 |
| `trcdtbp2.wav` | FILTER VARIABLE - band of frequencies retained is considerably higher: 1796 |
| `trcdtbp3.wav` | FILTER VARIABLE - band of frequencies retained is much higher: 4796 |
| `trcdtbr1.wav` | FILTER VARIABLE - in band reject mode to hollow out a specified band of frequencies: 296 |
| `trcdtbr2.wav` | FILTER VARIABLE - band of frequencies rejected is considerably higher: 796 |
| `trcdtbr3.wav` | FILTER VARIABLE - band of frequencies rejected is much higher: 1796 |

### Various Preset Filter Banks

| File | Description |
|------|-------------|
| `trcdtfbm1.wav` | FILTER BANK - filters follow the harmonic overtone series |
| `trcdtfbm2.wav` | FILTER BANK - filters follow alternate harmonics of the harmonic overtone series |
| `trcdtfbm3.wav` | FILTER BANK - filters follow the subharmonic series |
| `trcdtfbm4.wav` | FILTER BANK - filters follow the harmonic overtone series, with linear offset |
| `trcdtfbm5a.wav` | FILTER BANK - filter according to a pattern of equally spaced intervals - high |
| `trcdtfbm5b.wav` | FILTER BANK - filter according to a pattern of equally spaced intervals - lower |
| `trcdtfbm6a.wav` | FILTER BANK - filter according to equally spaced minor thirds (3 semitones) |
| `trcdtfbm6b.wav` | FILTER BANK - filter according to equally spaced octaves (12 semitones) |
| `trcdtfbm6c.wav` | FILTER BANK - filter according to equally spaced perfect 5ths (7 semitones) |
| `trcdtfbm6d.wav` | FILTER BANK - filter according to equally spaced augmented 4ths (6 semitones) |

### Phasing Filter

`capm.wav` - New input soundfile for:

| File | Description |
|------|-------------|
| `capmphasm2-35.wav` | FILTER PHASING, Mode 2 - 35ms delay for a reverberant, enclosed space |
| `capmphasm2tv.wav` | FILTER BANK, Mode 2 - `phasdlay.brk` for a time-varying reverberation pattern (big space moving to a more enclosed space) |

### Cumulative Filtering

| File | Description |
|------|-------------|
| `capmblow.wav` | New input cut from last part of `capm.wav` |
| `capmblowit1.wav` | FILTER ITERATED - `eqint6.txt` (produced by BANKFRQS) for cumulative filtering of an equal interval pattern (minor thirds are specified) |
| `capmblowit6ps.wav` | FILTER ITERATED - with a randomised pitch shift (Q = 75 and delay time is 0.25 sec.) |

### Sweeping Filter

| File | Description |
|------|-------------|
| `trcdtswm3.wav` | FILTER SWEEPING - Mode 3 to sweep filter within a specified band of frequencies (200-220) |
| `trcdtswm2.wav` | FILTER SWEEPING - Mode 2 to sweep lo-pass (bottom part of sound -- 200 up to 1000) |
| `capmswm2.wav` | FILTER SWEEPING - Mode 2: the same operation adjusted and tried out on the vocal sound |
