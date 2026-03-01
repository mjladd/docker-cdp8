# Workshop 8: Transitions / Morphing

*by Dr Archer Endrich*

The locations of the processing functions are shown under the program name.

- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

Sonic transitions involve moving from one sound to another sound. CDP has 5 programs specifically designed to make transitions. Other processes such as transposition, vocoding and envelope transfers may help create intermediate stages that make the transitions more psychoacoustically effective.

The programs presented here achieve transitions in various ways, with varying degrees of smoothness, which is generally the objective. Timing is crucial: to hear as much as possible of both the first sound (aural starting point) and the second sound (aural end point), while taking as long as necessary over the transition itself. The ear is acutely sensitive to the finest detail, and a longer transition helps to make it more gradual.

It is not easy to get effective results with transitions, and morphing in particular. In this Worksheet I try to explain in some detail why things are being done as they are.

**Source files needed:** [femwheeze.wav](../../sounds/CSRCWS08SF01femwheeze.wav), [oing.wav](../../sounds/CSRCWS08SF04oing.wav), [trcdt.wav](../../sounds/trcdt.wav), [frogs3cdt.wav](../../sounds/WS07SF01frogs3cdt.wav), [gongvib.wav](../../sounds/CSRCWS08SF02gongvib.wav) and [count.wav](../../sounds/count.wav). Make analysis file versions (`.ana`) of each of these: **SS: Spectral > Convert > Analyse** or **SL: PVOC > analysis**.

The first two are too short, so we make more of them by time-stretching the 'wheeze' and splicing the 'oings'.

Time-stretch `femwheeze.ana` by a factor of 2.7. This lengthens a 1.55 sec. file to 3.2 sec. (3.2/1.55 = 2.64). Name this result `femwhx2-7.ana`. Convert this back to a soundfile and REVERSE it (**SS: Soundfiles > Radical > Reverse SF** / **SL: RADICAL > radical > reverse**), making `femwhx2-7r.wav` and also Convert this to an analysis file: `femwhx2-7r.ana`.

REVERSE `oing.wav` to make `oingr.wav` and then extend by splicing the forwards and backwards versions, in this pattern: F-B-B-F-B-F: **SS: Soundfiles > Edit > Splice** / **SL: Edit > join**. In CDP Release 5 you can use JOINSEQ (**SL: EDIT > join in patterned sequence**). You put `oing.wav` and `oingr.wav` on CHOSEN FILES in that order and then get or make the file `joinseq.txt` containing the numbers `1 2 2 1 2 1`. Name the output `oingseq.wav` and also Convert it to an analysis file: `oingseq.ana`.

Now we're ready to roll.

## A. In the Time Domain - SUBMIX CROSSFADE

- **SS:** Edit/Mix > Mix > Crossfade > cosinusoidal (Mode 2)
- **SL:** MIX > crossfade > cosinusoidal

### ST1.

This is a MIX of the first file fading away while the second file starts quietly and gets louder. You should hear an overlap of the two sounds. However, it is a very simple process, so getting a smooth transition is not easy. Notice how the amplitude envelopes of both sounds are taken into consideration when working out an effective timing for the crossfade.

In this case, we are going to use `femwhx2-7r.wav` (5.2 sec) as the first file and `oingseq.wav` (5.2) as the second. We set stagger to 0.5 to give the 'wheeze' time to get started. The crossfade then starts also at 0.5 and ends at 3.2 (we don't go to the end of the sound). `oing.wav` begins strongly, so we use the 'cosinusoidal' option to skew the transition, making the entry of `oing.wav` quieter. Skew = 1.5 seems to work OK. Name the output `fem-xfskew-oing.wav`. Thus we have:

```
submix crossfade Mode 2 femwhx2-7r.wav oingseq.wav fem-xfskew-oing.wav stagger = 0.5,
begin = 0.5, end = 3.20 skew = 1.5
```

Both stagger and skew are key parameters.

Other values for skew will usually lead to `oing.wav` coming in too strongly, so that aural overlap is minimal and it almost sounds like the two sounds are being spliced. This is especially the case if the transition is left too late (e.g., stagger = 1.00) because then there is not much left of `femwheeze.wav` to cover / mask the second sound.

### ST2.

For comparison, crossfade `trcdt.wav` (10 sec, steady tractor sound) with `femwheeze.wav` (1.55 sec, funny groan). The aim is to be listening to the tractor, and then it groans at the end, as if suddenly taken ill. As the tractor sound doesn't fade, we can afford to put the wheeze right at the end: 10-1.55 = 8.45 stagger and 8.5 start time (slightly later). Mode 1 for a Linear crossfade seems to work well. We can name this `trcdt-xf-femwheeze.wav`.

```
submix crossfade Mode 1 trcdt.wav femwheeze.wav trcdt-xf-femwheeze.wav stagger = 8.45,
begin = 8.50, end = 10.00
```

## B. Spectral Domain

### 1. COMBINE CROSS

- **SS:** Spectral > Morph/Formants > Cross
- **SL:** COMBINE > cross channels

#### ST3.

This function replaces the spectral amplitudes (loudness of each partial) of the first file with those of the second. The **SS: replace** / **SL: interpolate** parameter sets the weighting of the second file relative to first.

The first thing we notice is that there is no 'stagger' parameter. This is one reason why we've made longer versions of our first two source files, and matched their durations. The inputs are therefore `femwhx2-7r.ana` and `oingseq.ana`, in that order.

The amplitude of the `oingseq.ana` is fairly high, so we will set replace/interpolate to 0.25. We can name the output `fem-ccross-oing.ana` and then Convert it to a soundfile: **SS: Spectral > Convert > Synthesise** / **SL: PVOC > synthesis**, forming `fem-ccross-oing.wav`. The two rather strange but similar files make an effective mix which mingles both sounds.

```
combine cross femwhx2-7r.ana oingseq.ana fem-ccross-oing.ana replace/interpolate = 0.25
```

The result is not entirely satisfactory, but can be improved by making the replace/interpolate parameter time-varying, which we will do in the next example.

#### ST4.

For comparison we can try two files that are timbrally dissimilar. We can use `trcdt.ana` (10 sec) and `frogs3cdt.ana` (frogs 10 sec) because they're about the same length. We create a gradual changeover to the frogs with the breakpoint file `tvcross.brk` (or `tvcross.txt`, SL may use a `.txt` extension):

```
0   0
10  1
```

(time / weight of 2nd file: 0 = none, 1 = full)

This gives a smooth transition. Name the output `trcdt-tvccross-frogs.ana` and convert to a soundfile.

```
combine cross trcdt.ana frogs3cdt.ana trcdt-tvccross-frogs.ana replace/interpolate = tvcross.brk
```

So we hear a transition, but not any real confusion of timbres. For this we can turn to MORPH.

### 2. MORPH MORPH

- **SS:** Spectral > Morph/Formants > Morph
- **SL:** MORPH > morph

What is exciting is that we hear something completely different when we try a MORPH proper. Now we have a real transition from one sound to another in which the timbral colour of the two sounds are gradually combined, moving from the first input to the second input.

#### ST5.

Our first trial will be a straight morph between `femwhx2-7r.ana` and `oingseq.ana`, sounds that are similar both in length and timbral qualities. We will do the morph over almost the entire length of both files (3.2 sec).

Note that in Soundshaper, you go to the dialogue box and there open the second file. In Sound Loom, you make sure that both inputs are CHOSEN FILES before you go to PROCESS.

Enter the following values in the dialogue box:

| Parameter | Value | Description |
|-----------|-------|-------------|
| amp start | 0.5 | transition between spectral amplitudes (loudness of the frequencies) |
| amp end | 3.0 | |
| freq start | 0.5 | transition between frequencies (timbral colour) |
| freq end | 3.0 | |
| amp exp | 1.5 | transition line or curve |
| freq exp | 1.5 | |
| stagger | 0.5 | the second sound comes in at 0.5 seconds |

RUN and save as `fem-m-oing.ana` and convert to a soundfile: `fem-m-oing.wav`.

#### ST6.

If we create a MORPH version of SUBMIX CROSSFADE `trcdt.ana` + `femwheeze.ana` as in ST2, we can hear a slight difference: when the wheeze comes in, we hear a bit more of the tractor sound blended in with the start of the wheeze. This difference would be more noticeable with a longer morph. We will use a stagger parameter as in the crossfade to place the wheeze at the end: 1st file (10 sec) minus 2nd file (1.55 sec) = start time of 8.45 so that both end together. Thus the parameter values are:

| Parameter | Value | Description |
|-----------|-------|-------------|
| amp start | 8.45 | transition between spectral amplitudes |
| amp end | 10.00 | |
| freq start | 8.45 | transition between frequencies |
| freq end | 10.00 | |
| amp exp | 1 | transition line or curve |
| freq exp | 1 | |
| stagger | 8.45 | 2nd sound starts at 8.45 sec |

Output name: `trcdt-m-femwheeze.ana` and convert to `trcdt-m-femwheeze.wav`.

#### ST7a. Stage 1 - FORMANTS VOCODE

- **SS:** Spectral > Morph/Formants > Vocode
- **SL:** FORMANTS > vocode

Now we will try to create a 'real' morph of a gong into a female voice via an intermediate stage, such that we really hear the original file being gradually 'before our ears' reshaped into the second.

As a half-way house, we can put the sound of the voice 'into' the gong. `gongvib.ana` is infile1 and `count.ana` is infile2. Name the output `gongvoccount.ana` and convert to `gongvoccount.wav` to audition later.

#### ST7b. Stage 2 - MORPH

- **SS:** Spectral > Morph/formants > Morph
- **SL:** MORPH > morph

Now morph this vocoded sound with the voice, to move from the vocoded sound to the clear sound of the voice. `gongvoccount.ana` is infile 1 and `count.ana` is infile2.

```
morph morph 1 gongvoccount.ana count.ana gongvoccount-m-count.ana 0 8 0 8 1.5 1.5 0
```

Convert to `gongvoccount-m-count.wav` to listen to later, but use the `.ana` for Stage 3.

#### ST7c. Stage 3 - MORPH

Now create the first part of the morph by morphing from the clear sound of the gong to the intermediate (vocoded) sound morphing to the clear sound of the voice. `gongvib.ana` to `gongvoccount-m-count.ana`, forming `gong-m-gongvoccount-m-count.ana`. Stagger = 5 to give time for the clear gong and then the intermediate (vocoded) sound to come through before getting to the morph to the clear sound of the voice. The morph transition ends at 10 seconds, so the gong sound will end here, leaving the clear sound of the voice to come through.

```
gongvib.ana gongvoccount-m-count.ana gong-m-gongvoccount-m-count.ana 5 10 5 10 1 1 5
```

The output `gong-m-gongvoccount-m-count.ana` is 9.13 seconds long. Convert to `gong-m-gongvoccount-m-count.wav`: i.e., 'gong morphed towards gong-vocoded-with-voice morphed towards voice'.

### 3. MORPH GLIDE

- **SS:** Spectral > Morph/Formants > Glide
- **SL:** MORPH > glide

#### ST8.

MORPH GLIDE creates a gliding movement between the spectra in two single analysis windows, drawn from the same or a different sound (analysis file). You can specify the duration of the glide.

First we have to get single analysis windows: SPEC GRAB (**SS: Spectral > Utils > Grab**). We can use `trcdt.ana` and `frogs3cdt.ana`.

```
spec grab trcdt.ana tgrab.ana 1.0
spec grab frogs3cdt.ana fgrab.ana 0.05
```

(The number is the time in seconds of the grabbed analysis window.)

And then:

```
morph glide tgrab.ana fgrab.ana tglidef.ana 20
```

(20 is the duration of the output.)

Convert `tglidef.ana` to `tglidef.wav`.

To tidy up, delete the various analysis files (`.ana`) made for this Worksheet.

## Worksheet 8 - Transitions: Summary and Main Outputs

### Altered Sources

| Soundfile | Process |
|-----------|---------|
| `femwhx2-7r.wav` | STRETCH TIME and RADICAL REVERSE |
| `oingseq.wav` | EDITSF JOIN or JOINSEQ to splice forwards & backwards versions |

### Time Domain Crossfade

| Soundfile | Process |
|-----------|---------|
| `fem-xfskew-oing.wav` | SUBMIX CROSSFADE - 1st fades out while 2nd fades in & use of stagger=0.5, and skew=1.5 to slow the entry of the 2nd input |
| `trcdt-xf-femwheeze.wav` | SUBMIX CROSSFADE - longer first sound, stagger=8.45 |

### Spectral Domain Cross

| Soundfile | Process |
|-----------|---------|
| `fem-ccross-oing.wav` | COMBINE CROSS - spectral amplitude replacement with balanced weighting (replace=0.25) |
| `trcdt-tvccross-frogs.wav` | COMBINE CROSS - differing inputs, with time-varying tvcross.brk |

### Spectral Domain Morph

| Soundfile | Process |
|-----------|---------|
| `fem-m-oing.wav` | MORPH MORPH - transition by spectral interpolation (over nearly full length of sounds) |
| `trcdt-m-femwheeze.wav` | MORPH MORPH - stagger employed - compare with trcdt-xf-femwheeze.wav |
| `gongvoccount.wav` | FORMANTS VOCODE - Stage 1: vibrating gong vocoded with female voice |
| `gongvoccount-m-count.wav` | MORPH MORPH - Stage 2: vocoded sound morphed towards clear voice |
| `gong-m-gongvoccount-m-count.wav` | MORPH MORPH - Stage 3: clear gong morphed towards vocoded sound which is (already) morphed towards the clear voice |

### Spectral Domain Glide

| Soundfile | Process |
|-----------|---------|
| `tglidef.wav` | MORPH GLIDE - long glide between spectra from two single analysis windows |
