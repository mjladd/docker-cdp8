# Listening5-Textures

*by Dr Archer Endrich*

*featuring an inharmonic sound source*

## Listening 5: Textures {#LISTENING2}

This second group of sounds comes from Worksheet 6 of *CDP Tutorial Workshop 1*. The source sound is track 1, followed by tracks 3 to 10 which illustrate some of the more straightforward texture variants, all made with TEXTURE SIMPLE: [clip5-all.wav](../sounds/clip5-all.wav). Examples 2 to 7 use Mode 5 (neutral), and examples 8 & 9 use Mode 3 (Harmonic Set). The inharmonic sound source is chosen in order to illustrate how the (semi-) algorithmic mixing potential of TEXTURE programs can be used to build up timbral richness. I leave it to you to imagine and try out the same settings with a more harmonic, tonally focused, sound source.

1. Source sound - [cymcdt.wav](../sounds/cymcdt.wav): suspended cymbal

2. Duration lo/hi parameters active - [cymcdtsame1.wav](../sounds/cymcdtsame1.wav): These parameters take effect as long as the 'use whole sound' flag is NOT invoked. In this sound, both lo and hi are set to 1.5 sec., meaning that the shortest the extract can be is 1.5 sec. and the longest the extract can be is 1.5 sec. Note that all note events begin at the beginning of the sound. These settings therefore mean that the first 1.5 seconds of the sound is what will be used. The end point is automatically spliced (enveloped to zero to avoid producing a click). *Packing* (density) is set to 2 seconds, causing a 1/2 sec. gap between the note events. All note events are at C-60, and the amplitude range is high (94 to 104 on the MIDI range).

3. Whole input used - [cymcdtsame2.wav](../sounds/cymcdtsame2.wav): This time the 'use whole sound' flag IS invoked, so the values given for the duration parameters are ignored. Therefore we hear the whole sound at each note event. As the duration of the input soundfile is 4.785 seconds, the sounds overlap. All note events are still at C-60 resulting in a constant reinforcing of the sound. The attack envelope is crucial in how distinct we hear these overlaps: a 'sharp' one makes the aurally distinct, but a long, slowly opening envelope will cause the note events to blend. This example illustrates how TEXTURE become can become a mixer with various parameter controls, i.e., an (at least semi-) 'algorithmic mixer'.

4. Randomised microtonal transpositions - [cymcdt58-62.wav](../sounds/cymcdt58-62.wav): Events transpose microtonally, down to *pitchlo* 58 or up to *pitchhi* 62 from the nominal reference pitch of '60' in the note data file. The result is randomised selections within a (tight) pitchrange of 58-62 MPV, not only including 59 and 61, but also possibly 59.6 etc. Therefore listen for inharmonic relationships. It could be forced to diatonic intervals or a specified chord by mapping to a Harmonic Field or Set, illustrated below.

5. Time-varyinging density - [cymcdtpksymsame.wav](../sounds/cymcdtpksymsame.wav): The note-events keep to the same pitch (C-60) and the whole sound is used, but the density, the *packing* of the note-events, changes over time. Instead of a single value for *packing*, a breakpoint file is used (*pksym.brk*) in which a symmetric pattern is created: a note-event every second, every 2 seconds, every 1/2 sec, every 2 seconds, every second, changing at specified times.

```
time  packing
  0   1
  5   2
 10   0.5
 15   2
 20   1
```

6. Time-varying density with a tight pitch range - [cymcdtpksym58-62.wav](../sounds/cymcdtpksym58-62.wav): Here we listen for what happens when the same time-varying packing breakpoint file *pksym.brk* interacts with the tight 58-62 MPV pitch range. A considerably denser and inharmonic sonic complex results, thus showing how this kind of texture can be created.

7. Time-varying density with a wider pitch range - [cymcdtpksym55-67.wav](../sounds/cymcdtpksym55-67.wav): The same *packing* breakpoint file is used (*pksym.brk*) but the note-events are selected from within an octave: MPV 55 (G below Middle-C) to MPV 67 (G above Middle-C). Inharmonic relationships are still (possibly) formed as the pitches are randomly selected, but the whole texture is broader and more 'open'.

8. Increasing the sonority - [cymcdtpksymC7th.wav](../sounds/cymcdtpksymC7th.wav): The same *packing* file is used, but this time the pitches are restricted to, mapped to, a specified C-7th chord, a 'harmonic set'. This chord is defined in the note data file and invoked with Mode 3. The chordal mapping opens the texture further and lends a certain degree of harmonic sonority, but this is limited by the complex nature of the cymbal sound itsef. The note data file *ndfC7th.txt* is written like this:

```
60
#4
0 1 55 0 0
0 1 60 0 0
0 1 64 0 0
0 1 70 0 0
```

9. Density! - [cymcdtC7th.wav](../sounds/cymcdtC7th.wav): Everything is the same as the previous example except that a constant value is used for *packing* instead of a breakpoint file. This value is 0.25 sec so that each note event enters after 1/4 sec. As using the whole sound is still invoked, the result is a much denser texture, but somewhat more harmonious because the increased number of sound events, each mapped to one of the pitches of the chord, increases the number of times the chordal pitches are repeated. Sometimes a texture like this will overload, and if it does TEXTURE issues a warning and a suggestion as to how much the gain should be reduced. This is applied *via* the *attenuation* parameter (**-a** on the command line).

---

Last updated: 20 August 2021
