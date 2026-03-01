# How to Create Rhythms and Handle Durations

*by Dr Archer Endrich*

*Thinking rhythm in a text context*

| | | |
|---|---|---|
| [**Durations: expressing in seconds**](#DURATIONS) | | |
| [**Rhythmic Effects**](#RHYTHMICEFFECTS) | [**Creating Rhythms**](#CREATINGRHYTHMS) | [**TEXTURE TIMED**](#TXTIMED) |
| [**TEXTURE TMOTIFS**](#TXTMOTIFS) | [**Project Suggestions**](#PROJECTS) | [**Listening 1**](#LISTENING1) |

## Durations {#DURATIONS}

Durations and rhythms need to be expressed in text form in the CDP system. While it may take a little while to get used to, the process is easy enough and enables very precise rhythms to be defined. In preparation for the following discussion of rhythm, here is a list of some basic musical durations expressed as numbers. The point of reference is 1 second at crotchet = 60. Note that subdivisions have to add up to a whole beat (an integer) or timings go awry, but the component numbers can appear in any order.

- crotchet: 1 (one beat)
- minim: 2 (two beats)
- dotted minim: 3 (three beats)
- whole note: 4 (four beats)
- crotchet triplet: 0.67 0.67 0.66 (3 crotchets in the time of 2 crotchets, 1 minim – not 0.67 0.66 0.66 which adds up to only 1.99; these three durations can be in any order)
- quaver triplet: 0.34 0.33 0.33 (3 quavers in the time of 2 quavers, 1 crotchet – make any of the three the longest one)
- semiquaver tripet: 0.17 0.17 0.16 (3 semiquavers in the time of 2 semiquavers, 1 quaver, 1/2 sec)
- semiquaver sextuplet: 0.17 0.17 0.16 0.17 0.17 0.16 (6 semiquavers in the time of 4 semiquavers, 1 crotchet, 1 second)
- quaver: 0.5 (half a beat)
- semiquaver: 0.25 (quarter of a beat)
- quintuplet: 0.20 0.20 0.20 0.20 0.20 (5 notes in the time of 1 crotchet)

It is generally easier to enter durations with these numbers based on multiples and subdivisions of 1 and then alter the actual speed with a *tempo* parameter when available. In the Texture Set, the tempo parameter is called *mult* because it applies a multiplier. Studying the above list will help to think durations as numbers. The numbers can then be used for durations in supplementary files for CDP, such as *packing*, *duration* and *note data files*.

The process of turning a list of durations into a list of ascending times is complicated by the fact that the times need to start at zero. To illustrate this, I have prepared [Durations as Numbers.pdf](data/ALM Durations as Numbers.pdf). For example, the first bar of the notated example begins with two minims. At crotchet = 60, this will be 2 for the first minim and 2 for the second minim. As accumulating times, this becomes:

```
time duration
0.0   2  (0 + 2 = 2:  the next note event starts at time 2 sec.)
2.0   2  (2 + 2 = 4:  the next note event starts at time 4 sec.)
4.0   (etc.)
```

The rest of the example in *Duration as Number* carries on from here, using various rhythmic patterns as well as tied notes. (This example was prepared in Lilypond Book which enters the music as text and then renders both the music and text as a pdf.)

## Rhythmic Effects {#RHYTHMICEFFECTS}

Before moving on to the TEXTURE programs, mention should be made of three Spectral Domain programs from the FOCUS Group that can create various limited rhythmic effects. Sounds with lively spectral changes, including noisy sounds, produce the best results. If the source is clearly pitched, the effect is rather subtle. The source sound used is [wavesdt.wav](../sounds/wavesdt.wav) 10.628 seconds in duration (converted with PVOC to be an analysis file).

FOCUS FREEZE – freezes the spectrum after ('a') or before ('b') times specified in a text file. The result is [wavesfrz5.wav](../sounds/wavesfrz5.wav). Some attempt was made to make the rhythms regular, but the interaction with the changing sound has mostly obscured the regularity. The gap that occurs was not deliberate but produced, I presume, by a dip in the spectrum. The text file for this example is *freeze5.txt*:

```
a1.0
 1.2
a1.4
 1.8


a2.0
 2.2
a2.4
 2.8

a3.0
 3.2
a3.4
 3.8

a4.0
 4.2
a4.4
 4.8

a5.0
 5.2
a5.4
 5.8

a6.0
 6.2
a6.4
 6.8

a7.0
 7.2
a7.4
 7.8

a8.0
 8.2
a8.4
 8.8

a9.0
 9.5
```

FOCUS HOLD – holds the spectrum at specified time points for specified durations given in a text file. The result [waveshold5.wav](../sounds/waveshold5.wav) is similar to FOCUS FREEZE, but with easier control over the length of the holds. In *hold5.txt* the durations leave a tiny gap before the next hold.

```
 1.0  0.1
 1.2  0.1
 1.4  0.3
 1.8  0.2


 2.0  0.1
 2.2  0.1
 2.4  0.3
 2.8  0.2

 3.0  0.1
 3.2  0.1
 3.4  0.3
 3.8  0.2

 4.0  0.1
 4.2  0.1
 4.4  0.3
 4.8  0.2

 5.0  0.1
 5.2  0.1
 5.4  0.3
 5.8  0.2

 6.0  0.1
 6.2  0.1
 6.4  0.3
 6.8  0.2

 7.0  0.1
 7.2  0.1
 7.4  0.3
 7.8  0.2

 8.0  0.1
 8.2  0.1
 8.4  0.3
 8.8  0.2

 9.0  0.3
 9.5  0.3
```

FOCUS STEP – freezes the spectrum at regular time intervals. In this example the *timestep* is 0.25 sec.: [wavesstep_25.wav](../sounds/wavesstep_25.wav). Steps can be as small as 0.006 seconds, but this seems to have little effect on the sound. With a *timestep* such as 0.2 or 0.1 the steps are audible and makes for a lively sound. A large value could be too slow for a step, but the frozen sound may provide useful source material for another process.

## Rhythmic Templates {#RHYTHMICTEMPLATES}

TEXTURE programs provide a way to create a **rhythmic template** which determines the onset times of the input sound or sounds. The sound repeats at each of these time points, thereby creating a rhythm. This mechanism is present in both TEXTURE TIMED and TEXTURE TMOTIFS. In TEXTURE TIMED there is only the template, so there is just a single iteration of the sound at each time point. The rhythm defined by the rhythmic template is what is heard.

However, TEXTURE TMOTIFS adds to this the ability to use 'fully defined' motifs, and the whole motif repeats at each time point. A 'fully defined' motif has start times for each of its note events, pitch, velocity and duration. The velocity parameter is especially important because it enables variation in the way each note of the motif is stressed. This helps rhythmic clarity, because if all the notes are equally stressed, the rhythmic pattern of the motif(s) flows together and the repetitions become indistinguishable from one another – which could also be an objective!.

Here is an example of a very simple motif: a crotchet followed by two quavers. It is defined in a note data file, so begins with '#number of notes in the motif'. If the value for the duration is longer than the time between the notes of the motif, it will produce legato effects, and *v.vs.* The motif repeats at every time point of the rhythmic template.

```
#3
0.0 1 60 96 1.2  (Middle-C, loud)
1.0 1 55 60 0.6  (G below Middle-C, much quieter)
1.5 1 55 60 0.6  (etc.)
```

When you think about it, there are mind-bending possibilities implied by the TMOTIFS mechanism. These possibilities emerge *when the length of the motif exceeds the the amount of time between time points on the rhythmic template (or nodal substructure)*. In other words, the motifs can overlap, and this overlapping can be made to be very complex. It is a real composition challenge to work out how to set up either regular or complex rhythmic interactions.

Note that the *skiptime* parameter determines the time between iterations of the rhythmic template and cannot be used to create overlap.

Rhythmic effects can be achieved with TEXTURE SIMPLE in more than one way. We described one of these in [*How to Create Strong Tones*](how-to-create-strong-tones.md#MELODYTX). A clear rhythm resulted when *packing* timing matched that of the changing pitches. The TEXTURE ORNATE set has a 'nodal substructure' mechanism. This is similar to a rhythmic template because it defines time points. However, it also allows a pitch value to be defined, which opens up another range of possibilities such as (virtually) parallel intervals, canons, or even more complex interactions of motifs.

The tempo parameter *mult* is available in some of the TEXTURE programs, notably MOTIFS, the ORNATE set and TMOTIFS. This parameter can be used to increase or decrease the speed of events, and a range of speeds can be created with the *multlo* and *multhi* parameters, causing motifs of differing speeds to overlap in unpredictable ways. It is recommended to work out rhythms at crotchet = 60 to keep the arithmetic simple, and then change the speed with this tempo parameter.

This section is important because, in a software system that emphasises sound transformations, the ability to create defined and focused rhythmic events (and melodies) can broaden the musical palette. This definition can be counterbalanced by more complex verging on unpredictable interactions of template, nodal substructure, motif, and variable tempos.

May I refer you again to the *Texture Tutorial Workshop* which illustrates the whole set of TEXTURE programs with lots of text and sound examples. The Reference Manual also has many examples to study.

## TEXTURE TIMED {#TXTIMED}

TEXTURE TIMED repeats a sound rhythmically; this rhythm is defined in the note data file as a rhythmic template. If there is more than one pitch (e.g., because of a pitch range specified with the *min* and *max* pitch parameters, or due to the presence of a harmonic set), the pitches are selected at random – but the rhythmic template stays the same. There is no tempo parameter in TEXTURE TIMED.

The definition of the rhythmic template requires several fields but only the first field, the times, is actually active. The column of 1s has to be there and zeros for the other fields. In the note data file below, the times are based on 1.0 = 1 second. The very first line of the file is the required nominal reference pitch as a MIDI note value, in this case Middle C. The next line, beginning with a # states how many lines are in the rhythmic template. This example uses a bouncing ball [ballc.wav](../sounds/ballc.wav) as the source, and the note data file which defines the rhythm (*rhythtempl.txt*) is as follows:

```
60
#11
0.0   1  0  0  0   ;quintuplet (start of beat 1 at crotchet = 60)
0.2   1  0  0  0
0.4   1  0  0  0
0.6   1  0  0  0
0.8   1  0  0  0
1.0   1  0  0  0   ;1 1/2 beats  (start of beat 2)
2.5   1  0  0  0   ;quaver
3.0   1  0  0  0   ;quaver triplet  (start of beat 4)
3.34  1  0  0  0
3.67  1  0  0  0
4.0   1  0  0  0   ;start of beat 5 (last rhythm event) and of skiptime
                   ;skiptime = 1 makes beat 5 1 second long: 4.0 to 5.0
                   ;5.0 = 0.0 because the repeat starts here (bar of 5/4)
                   ;skiptime = 2 makes beat 5 2 seconds long: 4.0 to 6.0
                   ;6.0 = 0.0 because the repeat starts here (bar of 6/4)
```

The *skiptime* parameter in TEXTURE TIMED is vital. It is discussed more thoroughly in [*Getting a Grip on Packing and Skiptime*](getting-a-grip-on-packing-and-skiptime.md), but it is worth highlighting key points here. *Skiptime* in the *Sound Loom* menu is: `pause between line repeats`. *It is the time between the start of the last event of the rhythm definition and the start of the repetition of the whole rhythmic template.* With the above file, if *outdur* is greater than 5 or 6 seconds, the sequence will start over, but to know *when* to repeat, it needs to know how long the last note event is supposed to be. *Skiptime* provides this information. The **actual sound** that starts at time 4.0 may not last all the way to the start of the repetition (leaving a gap), or, if longer than *skiptime*, may overlap the start of the next repetition.

*To remain on the beat* at crotchet = 60, *skiptime* needs in this case to be a whole number, e.g., 1 or 2 seconds. This pause is calculated from the last time in the file: 4.0 which is the 5th musical beat.

- Therefore a 1 second *skiptime* creates a 5 beat sequence which repeats without a gap because the ball sound is less than 1 second in duration: time 4.0 + 1 = 5.0 = time 0.0 of the repeat: bars of 5/4. Listen to the pattern repeat 1 second after the final beat at 4.0 sec.: [ballctxtimedskip1.wav](../sounds/ballctxtimedskip1.wav).

- The full command line shows the rest of the parameters for this example:
`texture timed 5 infile outfile ndf outdur skiptime snd1st sndlast`
`texture timed 5 ballc.wav ballctxtimedskip1.wav rhythtempl.txt 20 1 1 1`
`vello velhi mindur maxdur (ignored) minpch maxpch atten pos spread whole_sound`
`60 90  0.6 0.8  60 60  -a0.9 -p0.5 -s1 -w`

- A 2 second pause creates a 6 second sequence which repeats with a 1 second gap (with this input sound): time 4.0 + 2 = 6.0 = time 0.0 of the repeat. The result with a 2 second *skiptime* is in effect 6/4 bars at crotchet = 60: [ballctxtimedskip2.wav](../sounds/ballctxtimedskip2.wav). With this input sound, the 6th beat forms a rest. Listen to this sound and tap out the beats and the *skiptime* will become apparent.

Because the amplitudes are not set for each note, observe that the rhythmic features of the motif tend to blend together with not much sense of beginning and end. This can be useful when continuous blending is appropriate. When more dynamically shaped motifs are required, other Texture programs such as TMOTIFS or POSTORNATE are available in which the note events defined by motifs have a velocity parameter.

When the parameters *minpitch* and *maxpitch* are the same value (e.g., 60 as in the note data file), all the notes remain at the same pitch. By changing these pitch values to 48 and 72 respectively, the pitches now span a 2-octave range. The (randomised) transpositions drawn from this range create quite a different sonic environment: [ballctxtimed48-72.wav](../sounds/ballctxtimed48-72.wav).

## TEXTURE TMOTIFS {#TXTMOTIFS}

Moving on to TEXTURE TMOTIFS/lN (Texture timed motifs either without or 'in' a harmonic set – Mode 3), a more flexible result regarding timing can be achieved because one or more motifs can be defined as well as the rhythmic template. A motif is initiated at each time point in the rhythmic template. Consider this (annotated) note data file: *tmotifsinhf.txt* in which a Harmonic Set is also employed. (The comments are only put here to help explain the note data file. Comments are not allowed in the actual ndf.

```
60                      ;reference pitch
#5                      ;five timed 'lines' to follow (same pitch)
0.0 1 55  0 0
2.0 1 55  0 0
3.0 1 55  0 0
5.0 1 55  0 0           ;note change to 1 second interval
6.0 1 55  0 0
#6                      ;six harmonic set pitches: G-minor-sharp7
0 1 55 0 0
0 1 58 0 0
0 1 62 0 0
0 1 66 0 0
0 1 67 0 0
0 1 70 0 0
#11                     ;eleven-note motif starts with a quintuplet
0.0   1  55  96  0.5    ;same rhythm as above, but now with pitch etc.
0.2   1  57  80  0.5
0.4   1  58  82  0.5
0.6   1  57  84  0.5
0.8   1  55  86  0.5
1.0   1  62  90  0.5
2.5   1  63  76  0.5
3.0   1  66  80  0.5
3.34  1  67  78  0.5
3.67  1  66  76  0.5
4.0   1  70  82  0.5
```

With *ballc.wav* as source sound as with TEXTURE TIMED, and a motif with the same rhythm as before but now given a rising pitch contour, the motif starts over on timed lines sometimes spaced two seconds apart and once 1 second apart. The pause between repeats is 1 second, keeping the motif 'on the beat'. Because of the timed 'lines' (nodal substructure), *there is a contrapuntal overlap that wasn't there before*, sounding much more like a whole team moving the ball about: [ballctxtmotifsin.wav](../sounds/ballctxtmotifsin.wav). The harmonic nature of this pattern is brought out if we redo the sound with the pitched source *bellaebtob.wav*: [bellaebtobtxtmotifsin.wav](../sounds/bellaebtobtxtmotifsin.wav). All the rhythmic overlaps of the eleven-note motif placed on the timed lines rise in pitch because the pitches of the motif rise. You will notice that not all the pitches in the motif are in the harmonic field. Internal algorithms cause these to snap to the field pitches. To have *all* the pitches of a motif play when there is a harmonic field, the harmonic field must include *all of these pitches*.

---

Last updated: 25 August 2022
(c) 2022 Archer Endrich Plymouth UK
