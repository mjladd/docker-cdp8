# How to Tune Sounds

*by Dr Archer Endrich*

*Three ways to create harmonic effects*

| | | |
|---|---|---|
| [**Composition Context**](#INTRODUCTION) | | |
| [**Harmonic Set or Field**](#TXSETFIELD) | [**Mapping to Frequencies**](#SPECTRALTUNING) | [**Variable Filtering**](#FILTERVARIABLE) |

## Composition Context {#INTRODUCTION}

A great deal of electroacoustic music uses sounds with considerable inharmonic content, in varying degrees more noise than focused pitch. However, there is often more clearly defined pitch content than at first meets the ear, if one is listening out for it. In the electroacoustic aesthetic so far, this seldom takes the form of extended melody. Rather it can be a matter of pitched motifs, tones woven into the overall texture, or establishing a certain level of (harmonious) sonority.

The CDP software offers several approaches to the creation and use of pitched sounds. This document illustrates three of them. Clarity of pitch is emphasised, but it is assumed that a given application may very well mix pitched content into a broader sonic context. The nature of the source sound affects the result, as does the handling of parameters which increase or decrease the amount of focus on the selected pitches. It is a matter of just how much harmonic sonority should be infuse the sounds in the given context.

## Harmonic Textures {#TXSETFIELD} – [TEXTURE SIMPLE, Mode3 and Mode1]

Harmonic textures with randomised pitch selection can be assembled with TEXTURE SIMPLE using either Mode 3 (Harmonic Set) or Mode 1 (Harmonic Field).

[SL] `TEXTURE->tune spectrum`
[SSh] `Soundfiles->TEXTURE`

You may recall that 'Set' (Mode 3) uses only the pitches specified in the note data file, while 'Field' (Mode 1) allows octave duplicates. The pitch range (command line) parameters (*pitchlo* to *pitchhi*) need to accomodate the range of notes that you want to be produced. Here are four examples so that results can be compared.

The harmonic set is defined in the note data file and the program selects notes from it at random, a nicely flexible way to produce a harmonic texture (but not a melody). Note that all times durations and velocities are 0. Pitch is the only active field. This is the note data file containing the usual reference pitch and a 5-pitch harmonic set: *ndfhs62-74.txt*

```
62
#5
0 1 62 0 0
0 1 65 0 0
0 1 69 0 0
0 1 71 0 0
0 1 74 0 0
```

Although all the times (column 1) in the note data file are zero, this does not mean that the 5 pitches occur at once as a chord. Rather, the pitch of each given note event is selected from the list and played at a time determined by *packing*. Small values for *packing* (0.1 or less) are what create the illusion of chords because many note events are selected in quick succession. Creating chords can also be done by transposition and mixing, or by the two programs described below.

Two different time-varying *packing* files are used to bring a modest amount of rhythmic variety into the texture. The first of these files moves back and forth between longer and shorter durations, while the second file is symmetric, moving to shorter values at the middle and then back again. The two *packing* breakpoint files are:

```
;pktv1.brk         ;pktv2.brk [comments are allowed in .brk files]
 0  1.6              0  1.6
 6  0.25             4  1.0
12  0.6              8  0.5
18  0.25            12  0.25
24  1.6             16  0.5
30  0.9             20  1.0
                    24  1.6
                    30  1.6
```

The four versions use [bellaedtbtob.wav](../sounds/bellaedtbtob.wav) as the input soundfile, specify a 30 second *outfile* (it will be a little longer than this), *scatter* is 0.3 seconds to loosen up the timing of entries, a wide *amplitude* range of 60 to 96 is intended to make the note events more individually perceptible, and *min/max duration* is ignored because 'use whole sound' is invoked. *Attenuation* is a cautious 0.8, with *position* 0.5 and *spread* 1.0 (full width).

These examples try to show an interaction between the Modes, the pitches specified in the note data file, the pitch range specified by the (command line) parameters, and the effect of time-variable packing. All use the same note data file (*ndf62-74.txt*, as above). The following table lays out the examples.

| Soundfile Result | Mode | Packing File | Pitch Parameters | Description |
|---|:---:|---|:---:|---|
| [bellaedtbtobtxpktv1m3.wav](../sounds/bellaedtbtobtxpktv1m3.wav) | 3 | *pktv1.brk* | 62-74 | tight range, changing rhythms |
| [bellaedtbtobtxpktv1m1.wav](../sounds/bellaedtbtobtxpktv1m1.wav) | 1 | *pktv1.brk* | 62-74 | pitches forced into tight range |
| [bellaedtbtobtxpktv2m3.wav](../sounds/bellaedtbtobtxpktv2m3.wav) | 3 | *pktv2.brk* | 62-74 | tight range, symmetric rhythms |
| [bellaedtbtobtxpktv2m1.wav](../sounds/bellaedtbtobtxpktv2m1.wav) | 1 | *pktv2.brk* | 50-81 | wide range, symmetric rhythms |

The second example illustrates how the parameters pitch range force note events that might have been at different octaves (Mode 1) to stay within the exact pitches specified in the note data file. Contrast this with the fourth example, where the parameters pitch range was much wider (D just above Low-C to A above High-C). The difference between the two *packing* files shows how the texture can be shaped rhythmically.

An example of changing chords will round off this look at tuning sounds with TEXTURE. The note data file defines the Harmonic Set for three chords: C-major (C E G at time 0 sec.), E-flat minor (E-Flat G-Flat B-Flat at time 3 sec.), and D-major with added 6th (D F-Sharp A B D' at time 5 sec., continuing to the end). The *packing* breakpoint file ensures that the selected pitches are heard as a chord by setting the timing to 0.1 second, easing in and out of the chords at the outer edges with slightly longer time-gaps.

```
;ndf3chords.txt    ;pk3chords.brk  [NB: comments
 0 1 60 0 0        0.0  0.25        in note data files (as here) are
 0 1 64 0 0        1.0  0.2         not actually accepted]
 0 1 67 0 0        1.5  0.1
 3 1 63 0 0        3.0  0.1
 3 1 66 0 0        5.0  0.1
 3 1 70 0 0        6.4  0.1
 5 1 62 0 0        6.5  0.2
 5 1 66 0 0        8.0  0.25
 5 1 69 0 0
 5 1 71 0 0
 5 1 74 0 0
```

Mode 4 is used to duplicate the pitches in different octaves and the *outdur* is 10 seconds. The result is [bellaedt3chords.wav](../sounds/bellaedt3chords.wav). The other parameters are pretty much as in the earlier examples: *scatter* is a tighter 0.075 seconds to very slightly loosen up the timing of entries, a wide *amplitude* range of 60 to 96 is intended to make the note events more individually perceptible, and *min/max duration* is ignored because 'use whole sound' is invoked. *Attenuation* is 0.7 after a warning that 0.9 overloaded, with *position* 0.5 and *spread* 1.0 (full width).

## Tuning in the Spectral Domain {#SPECTRALTUNING} – [PITCH TUNE]

The next effect tunes a sound to a chord by picking out the partials in the source sound that (most closely) match the pitches specified as MIDI Pitch Values. This supplementary text file has the file extension '.tun' (or '.txt'). To do this, we first need to analyse the source soundfile with PVOC, then use

[SL] `PITCH:HARMONY->tune spectrum`
[SSh] `Spectral->FREQ/PITCH->Tune`

and write the required text file (just a list of MIDI pitch values – frequencies in Hz could also be used):

```
MINOR (tunecmin1-8ve.tun)   MAJOR  (tunecmaj1-8ve.tun)
60 63 67 72                 60 64 67 72
```

*i.e.*, the minor chord: middle-C, E-flat, G, High-C and the major chord: 63 changed to 64 (E-natural). If the source sound is already focused pitch, the tune program probably won't find the frequencies it needs to accommodate the pitches in the tuning file. Threfore the source sound needs to be a fairly rich and even better a noisy sound for best results. The source used here, [springc1gcdt.wav](../sounds/springc1gcdt.wav), is a (very) large metal spring (Cut and Dovetailed to remove unneeded tail). Then it was back-to-backed to make [sprbtob.wav](../sounds/sprbtob.wav) and analysed with PVOC because PITCH TUNE takes an analysis file as input. The results were [sprbtobtunedCmin1-8ve.wav](../sounds/sprbtobtunedCmin1-8ve.wav) (minor) and [sprbtobtunedCmaj1-8ve.wav](../sounds/sprbtobtunedCmaj1-8ve.wav)

## Tuning with a Variable Filterbank {#FILTERVARIABLE} – [FILTER VARIBANK]

Another way to tune a sound is to use time-varying filtering *via* FILTER VARIBANK.

[SL] `FILTER->varibank`
[SSh] `Soundfile->FILTER->varibank`

Here is a file to filter a sound at three different times, moving from C-major to F-sharp major and then to E-major: *varfbank1.txt*. Note that the file extension can be either .txt or .brk.

```
; NB:  have to have the same number of filters in each line
; Transitions between chords are rapid, but not instant

0.00  36 -3dB 43 -3dB 48 -3dB 52 -3dB 55 -3dB 60 -3dB 67 -3dB 72 -3dB 76 -3dB
2.50  36 -3dB 43 -3dB 48 -3dB 52 -3dB 55 -3dB 60 -3dB 67 -3dB 72 -3dB 76 -3dB
3.00  54 -3dB 54 -3dB 54 -3dB 61 -3dB 61 -3dB 66 -3dB 66 -3dB 70 -3dB 70 -3dB
4.50  54 -3dB 54 -3dB 54 -3dB 61 -3dB 61 -3dB 66 -3dB 66 -3dB 70 -3dB 70 -3dB
5.00  40 -3dB 43 -3dB 47 -3dB 52 -3dB 55 -3dB 59 -3dB 67 -3dB 71 -3dB 76 -3dB
```

This text/breakpoint file for a time-varying filterbank covers the pitches of a chord in several octaves. Note how, as can be done in a transposition breakpoint file, the first chord is held the same for 2.5 seconds and then transitions to the next chord over half a second. Then the second chord does the same after 1.5 seconds. Closer times, such as holding until 2.99 sec. and then moving to the next chord at 3.00 sec., would make the change virtually instant. Implied here is that besides quick changes, prolonged glissandi between chords could also be produced. In doing this, it would probably help to have a brief period of stasis before and after the glissando – otherwise the original chord would disappear immediately and the effect of the glissando would be diminished.

The two main parameters for FILTER VARIBANK are *Q* and *gain*. Higher values for *Q* tighten the focus, and more of the original signal is lost. The range is huge, from 0.001 to 10,000 and the original sound can be completely replaced by the pitches/frequencies defined in the textfile. It becomes a composition issue where one wants to strike the balance. Also, when a considerable amount of the original signal is lost, more *gain* is needed, which also becomes a balancing act, because too much gain can cause overflows or unwanted resonance. (The program reports overflows.)

It therefore took quite a bit of experimentation to produce the following examples, one with [count.wav](../sounds/count.wav) and one with the spring sound, made 'back to back' at 0.1 second with a 30 ms splice: [sprbtob2.wav](../sounds/sprbtob2.wav). With the first of these sources, the result was [countfvb1.wav](../sounds/countfvb1.wav) with *Q* set at 120 and *gain* set at 3 (and *tail* 2.5). With the second source, the result was [sprbtob2fvb3dt.wav](../sounds/sprbtob2fvb3dt.wav) with *Q* set at 5000 and *gain* set at 1500, also with a 2.5 second tail. There was a sharp cut-off at the end, so this was removed with ENVEL DOVETAIL with linear beginning and end splices at 0.5 seconds each; hence the 'dt' in the name. These sounds are still a bit on the quiet side, but applying gain with MODIFY LOUDNESS produced the unwanted resonance mentioned above, at least in my speakers, so I did not apply gain.

---

Last updated: 06 September 2021
