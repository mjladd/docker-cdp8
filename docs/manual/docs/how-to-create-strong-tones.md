# How to Create Strong Tones

*by Dr Archer Endrich*

*tones ... texture ... melody*

| | |
|---|---|
| [**The Task**](#INTRODUCTION) | [**Preparation**](#PREPARATION) |
| [**A Strong Tone**](#STRONGTONE) | [**Adding some Shimmer**](#PLUSSHIMMER) |
| [**Making a Dense Texture**](#REALTEXTURE) | |
| [**Melody *via* Sequencer**](#MELODYSEQ) | [**Melody *via* TEXTURE**](#MELODYTX) |

## The Task in Hand {#INTRODUCTION}

The fact that TEXTURE SIMPLE can act as a kind of semi-algorithmic mixer program was mentioned earlier. One way for it to do this is the overlap the whole source sound at time points specified in the *packing* (density) parameter. To make a strong tone out of a single stroke on a Tibetan bowl ([bellae.wav](../sounds/bellae.wav)) we are going to overlap a sound every 0.1 seconds, i.e., 10 times a second. That's a lot of overlap.

## Preparation {#PREPARATION}

To get the effect we want, i.e., a mixture of these overlaps that comes across as a single, fused tone, there is a bit of preparation work to do. The first step is to take away the strong attack, the 'ping'. ENVEL DOVETAIL Mode 1 is used to do this with these parameter settings: *infadedur* 1.1 sec. with an exponential fade (the linear fade left some ping at the beginning), and *outfadedur* 0.5 with a linear fade. This formed [bellaedt.wav](../sounds/bellaedt.wav). It is somewhat quieter because the intial ping is gone. (Some gain could be applied at this point.) Having done this, EXTEND BAKTOBAK is used to create a longer, smooth tone.

SL: `EXTEND->back to back`
SSh: `Soundfile->EXTEND/SEGMENT->Back to Back`

This program joins a backwards to a forwards version of bellaedt.wav. The join was set to occur at time 2.2 seconds with a (larger than usual) 50 ms splice. Now we have [bellaedtbotob.wav](../sounds/bellaedtbotob.wav). By the way, BAKTOBAK was used extensively in Trevor Wishat's fantastic piece *Imago* to create powerful climactic moments.

## Making the Strong Tone {#STRONGTONE}

Now to make the long, rich tone. TEXTURE SIMPLE Mode 5 is used. As this is the first time that we are officially invoking a TEXTURE program, it might be useful to go have a look at the whole set of parameters. This is the listing as it appears on the command line, the concise, behind the scenes, program. Having this listing to hand is helpful because it quickly shows the correct order of the parameters. More information about what they mean and valid ranges can be found in the Reference Documentation. They are of course presented graphically in the GUIs.

```
USAGE:
texture simple mode infile [infile2...] outfile notedata outdur packing scatter
        tgrid sndfirst sndlast  mingain maxgain  mindur maxdur  minpich maxpich
             [-aatten] [-pposition] [-sspread] [-rseed] [-w -c -p]
```

Using the back-to-back sound as source, i.e., as the input sound for TEXTURE SIMPLE Mode 5, the main thing we are going to do is to iterate it every 10th of a second, keeping every iteration on the same pitch. Thus *packing* is set to 0.1 with *scatter* to 0.05 just to loosen it up a bit, and both *minpich* and *maxpich* are set to MPV 67. The nominal pitch in the note data file *ndf67.txt* is also set to 67 so no transposition is done (only the number 67 is in the ndf). How the reference pitch in the ndf can affect transposition is explained elsewhere. These are the key settings to achieve the tone we're after, and the result is: [bellaedtbtobtone.wav](../sounds/bellaedtbtobtone.wav). As a TEXTURE output, this file will be stereo. In order to re-introduce it to TEXTURE, it needs to be mixed down to mono with HOUSEKEEP CHANS, Mode 4, becoming [bellaedtbtobtonem.wav](../sounds/bellaedtbtobtonem.wav).

SL: `CHANNELS->extract/convert channels->convert to mono`
SSh: `Edit/Mix->CHANNELS->stereo/m-c -->,mono`

In TEXTURE SIMPLE Mode 5, the *outdur* is set to 12 seconds, but the actual result will be longer because the last iteration will occur just before then and will be allowed to run its course.

SL: `TEXTURE->simple/neutral`
SSh: `Soundfile->TEXTURE->simple` (then `MODE None` for Mode 5, 'Neutral')

For your information the other parameters were set as follows: *tgrid* 0, *snds* 1 1, *min/gain* 90 90, *min/max dur* 4.3 4.8 (ignored), *attenuation* 0.7 (full), *position* 0.5 (middle), *spread* 1.0 (full width), and the -w flag is invoked (use whole sound).

## Adding some Shimmer {#PLUSSHIMMER}

The document on Surface Texture carried this a step further to create subtle fluctuations in the surface of the long tone. This was done simply by introducing a tiny pitch range of MPV 66.8 to MPV 67.2: tiny beat patterns appear as a result of the randomised transpositions between these microtonal values. All other parameters were the same as those given above, and the result is [bellaedtbtobshimmertone.wav](../sounds/bellaedtbtobshimmertone.wav). Sometimes the long envelope introduced earlier causes the signal level to drop even when overlaid like this. If so, a bit of gain may need to be applied to the tone produced by TEXTURE. The fluctuations can be increased and made more fluid by using time-varying transposition breakpont files.

## Making a Texture {#REALTEXTURE}

OK, the time has come for a real 'texture'. If we take the same source (bellaedtbtob.wav) and alter only the pitch parameters in TEXTURE SIMPLE to low 65 (F) and high 70 (B-flat) – *packing* is still 0.1 sec, each new overlaid note event ~0.1 second apart will now be chosen at random somewhere within a *Perfect 4th range* (F to Bb). This illustrates just the beginning of the possibilities for more extensive texturing: [bellaebtobtonetx.wav](../sounds/bellaebtobtonetx.wav). See [How to Create Multi-event Textures](how-to-create-multi-event-textures.md) for some of these possibilities.

## Tones to Melody *via* EXTEND SEQUENCE {#MELODYSEQ}

A strong tone provides a good basis for a commanding melodic presence. One way to create a melody with the CDP software is with EXTEND SEQUENCE:

SL: `RHYTHM->sequencer`
SSh: `EDIT/MIX->Sequence-sampler`

A supplementary file is needed. It specifies the pitches of the melody and their amplitude weighting. Pitch is in semitone transpositions. Level is on a 0 to 1 scale. This is the file that defines the tune for EXTEND SEQUENCE.

```
;artoffuguetuneseq.txt
time    pitch   level
0.0	 0.0	0.9
2.0	 7.0	0.8
4.0	 3.0	0.7
6.0	 0.0	0.6
8.0	-1.0	0.5
10.0	 0.0	0.6
12.0	 2.0	0.7
13.0	 3.0	0.8
```

This will transpose from the pitch level of the original sound. If this pitch is the D, the tune will be D-A-F-D-C#-D-E-F. Note that all the transpositions are calculated from *the first pitch*: 7 semitones above 0, 3 semitones above 0 etc. (See the *Equivalent Pitch Notations* Chart for the MIDI Pitch Values.)

The source sound here is again [bellaedtbtob.wav](../sounds/bellaedtbtob.wav) (7.1 sec.). The resulting soundfile [artoffugueseqmelody.wav](../sounds/artoffugueseqmelody.wav) produces a melodic line, the opening of Bach's *The Art of Fugue*. If the length of the source sound is longer than the time between the notes (as here), the program will create overlaps, producing legato or harmonic effects depending on the amount of overlap.

## Tones to Melody *via* TEXTURE SIMPLE {#MELODYTX}

Using the same source sound, a melody can also be made with TEXTURE SIMPLE Mode 5. This involves making a time-varying pitch file and using it for both the low and high pitch parameters; **the *packing* parameter (the time between note events) is set to the same time interval as in the pitch breakpoint file (2 seconds) so that a clear melody is achieved**. There remains the possibility with TEXTURE to adjust *packing* (as a constant or in a time-varying way) to produce much denser harmonic effects and texturing. The same breakpoint file is used for both *minpitch* and *maxpitch*: *artoffugetunetx.txt* (D-A-F-D-Csharp-D-E-F):

```
time pitch
0    62
2    69
4    65
6    62
8    61
10   62
12   64
14   65
```

The note data file is set to the same pitch as the starting note of the tune: *ndf62.txt*: Midi Pitch Value 62. This is a reference pitch. It may or may not be at the same pitch as the soundfile, but if it isn't the soundfile will be transposed from the given reference pitch to the start pitch in the melody definition file. For example if the ndf gives a reference pitch of 55 and the tune starts on 62, the actual sound (which could be at any pitch) will be transposed up 7 semitones. It would be sensible to make the ndf reference pitch the same as the start pitch of the tune, unless there's a reason to do otherwise. The tune definition file then specifies the transpositions that create the tune. Thus the pitch at time 2 seconds is 69, 7 semitones higher, meaning that the second note of the tune will be a Perfect 5th above the first note.

The output duration is set to 18 seconds to give the last note time to finish: [artoffuguetxmelody.wav](../sounds/artoffuguetxmelody.wav). In this version, the 'line' produced will glissando from one note to the next. When there are different values at different times, the CDP breakpoint files interpolate, i.e., create intermediate values over the time period.

We can texture this melody by one simple change: i.e., to change *packing* to 0.25. This means that the sound will repeat 4 times a second. This makes more audible the fact that the pitch is interpolating between its *time pitch* values. The tune is heard as a glissandoing texture: [artoffuguetxmelodytx.wav](../sounds/artoffuguetxmelodytx.wav).

Another option is to specify discrete (stepped) pitch changes. This is easily done in TEXTURE. The 'use whole sound' (-w) flag is removed, making the duration parameters active. These are set to the length of note wanted, a constant if the same, time-varying if different. In this case *mindur* 2.4 and *maxdur* 2.5 (longer than the 2 seconds to keep it legato). The result is: [artoffuguetxmelodystepped.wav](../sounds/artoffuguetxmelodystepped.wav).

Other TEXTURE programs enable the creation of 'fully defined motifs' (pitch + duration + loudness) in various ways, with and without rhythmic overlap. See especially TEXTURE POSTORNATE. This is illustrated in [How to Create Rhythms and Handle Durations](how-to-create-rhythms-and-handle-durations.md).

---

Last updated: 25 August 2021
