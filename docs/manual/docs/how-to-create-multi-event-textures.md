# How to Create Multi-event Textures

*by Dr Archer Endrich*

*washes, motivic mosaics, canonic repetitions*

| | |
|---|---|
| [**Texture in Contemporary Composition**](#ABOUTTEXTURE) | |
| [**Multi-event Textures**](#MULTIEVENT) | [**Parameter Reminder**](#TXPARAMS) |
| [**A Multi-event Texture**](#AMULTIEVENTTX) | [**A Multi-sound Texture**](#MULTISOUNDTX) |

## Texture in Contemporary Composition {#ABOUTTEXTURE}

Texture, the vertical configuration of musical material, is a familiar concept. There have been many forms of texture, such as: the single line (monody), two or more aurally independent coherent lines (polyphony, sometimes as imitation, canon, or fugue with subject and countersubject), homophony (melody plus chordal accompaniment), mosaics of motifs, weavings on a harmonic field, rhythmic layering.

In the 20th century, texture began to emerge as a multi-event complex with its own identity. We hear a texture like this form towards the end of the Introduction to Stravinsky's *Rite of Spring*: motifs start to pile up until we hear a wash of notes – individual lines become aurally mixed. Stockhausen's *Gruppen* for two orchestras exemplifies all manner of densities such that whole complexes are heard as such, each with its own sonic character. Edgard Varese's music is marked by the formation of vertical sonic configurations ('constellations'). Examples proliferate as textural thinking became a standard feature of 20th century composition. The concept of multi-event texture complexes was already well developed before the advent of electroacousic music composition.

Electroacoustic music of various types used sound in a broader sense: not just pitched sounds and standard percussion instruments, but any kind of sound. Thus inharmonic and noise elements became accepted as musical components. These sounds often resisted rhythmic regimentation and tended more towards fusion than independence. This is all the more true when the sounds are from the natural world. Some degree of randomness of rhythm and pitch seem more suitable for them. Multi-event texture complexes were therefore grist in the mill for electroacousic music and used widely.

We have touched on these matters a couple of times in [*How to Create Strong Tones*](how-to-create-strong-tones.md). In that document a *packing* value of 0.1 along with a pitch range was used to create a sonic texture. A little later on *packing* = 0.25 was used to overlay a melody four times a second to make a thick glissandoing version of the tune. Similarly [*How to Tune Sounds*](how-to-tune-sounds.md) included an illustration of a randomised pitched event texture made with a TEXTURE harmonic field or set. More examples of textures can be found in the *DiceDemo* demo that comes with the CDP software, showing all the steps from soup to nuts.

We now focus on the powerful TEXTURE Set of programs to go deeper into how it provides a tool for creating differing multi-event textures with finely tweaked features.

## Multi-event Textures {#MULTIEVENT}

We can't leave our survey of the TEXTURE programs without an example of a straightforward multi-event texture. Previous examples focused on the rhythmic template constraint. We now focus on different types of texture. The starting point is TEXTURE SIMPLE Mode 5 ('neutral', i.e., no harmonic field or set) which

- repeats the input sound (starting each time at its beginning) at *packing* event onset times
- can set *packing* as a constant (single value) or a time-varying breakpoint file
- can produce somewhat 'visual' shapes *via* time-varying upper & lower limits in pitch breakpoint files
- has an option to play a limited amount of the input sound or the whole length of the sound: i.e., *mindur maxdur* parameters OR 'play whole sound' (**-w** flag on the command line), ignoring whatever is set for *mindur maxdur*

## TEXTURE Parameter Reminder {#TXPARAMS}

The parameter list for the TEXTURE programs can be rather daunting at first, though this is partly because there are several min-max parameters. Here is a little reminder of the parameters and what they mean with the lo-hi items put together:

`texture simple mode infile [infile2...] outfile notedata outdur packing scatter tgrid sndfirst-last  gainmin-max  durmin-max  pitchmin-max [-aatten] [-pposition] [-sspread] [-rseed] [-w -c -p]`

The parameters in square brackets [ ] are optional. The parameters are:

- *mode* (5 in this example)
- *input soundfile(s)*
- *output soundfile*
- *note data file* - just the reference pitch MPV is needed, e.g. 60
- *output duration in seconds*
- *packing*: temporal density of the 'note' events
- *scatter*: optional offset value in seconds to 'humanise' the *packing*
- *tgrid*: 0 unless quantising is to be specified
- *sndfirst* - *sndlast* (1 1 if one input, 1 2 if two etc.; can be time-varying)
- *mingain* - *maxgain* (level in MIDI values: 1 to 127)
- *mindur* - *maxdur* (amount of input soundfile to use - ignored if whole soundfile is used, so can put anything if 'whole length' is selected)
- *minpitch* - *maxpitch* (pitch range in MIDI Pitch Values: the same value for all on the same pitch, different values for random selection within a range, or time-varying breakpoint files
- [**-a***attenuation*] - it is often wise to set this at 0.9 or so to give some headroom for further processing, especially mixing
- [**-p***position*] - where in a 0 to 1 stereo field the sounds will be centred
- [**-s***spread*] - how spread out around the above position the sounds will be (1 is full spread; **-s**0 keeps the sound output from swinging between the Left & Right speakers)
- [**-r***seed*] - output differs on each repeat unless this is ON
- [**-w**] whole soundfile option – if ON, the whole input soundfile will be used for every note event; if not on, the *mindur* - *maxdur* parameters need to be carefully set
- [**-c**] - choose soundfiles cyclically in listed order ignoring sndfirst-last (i.e., in the order in which they occur on the command line or are presented to the program in a graphic interface); the default operation is to choose one of the soundfiles randomly for each *packing* (or nodal substructure line) event
- [**-p**] - when **-c** flag is set, randomly permutate each cycle

## A Multi-event Texture {#AMULTIEVENTTX}

The following example is a simple multi-event texture: one sound, using its whole length, rapidfire (0.15 sec between each event), pitches randomised within a 2 octave range (48-72), positioned at stereo centre with full spread. Thus the full set of parameters that produced this sound were:

**texture simple 5** *infile outfile notedatafile outdur packing scatter*
`texture simple 5 ballc.wav ballctx.wav ndf60.txt 12  0.15 0.02`
  *timegrid sndfirst sndlast vello velhi  durlo durhi pchlo pchhi*
  `0   1 1  50 90  0.5 1  48 72`
    *attenuation position spread usewholesound*
    `-a0.7 -p0.5 -s1  -w`

The result is a tightly packed [*ballctx.wav*](../sounds/ballctx.wav) (the mad basketball dribbler!). Introducing a time-varying breakpoint file for *packing* can make the texture more supple and interesting. Thus the breakpoint file *balltv.brk* produces [*ballctxtv.wav*](../sounds/ballctxtv.wav) – observe how the tempo timings interpolate (change gradually) between the breakpoint time-points. All we need now is a pause and a swish as the ball goes through the net.

```
;balltv.brk
 0.0  0.15   ;start fast
 3.0  0.40   ;slow down
 6.0  0.07   ;to very fast
 9.0  0.30   ;slow down again
12.0  0.10   ;to a little faster than original tempo
```

## A Multi-sound Texture {#MULTISOUNDTX}

It's time to push the envelope. The next four examples illustrate the use of several input soundfiles with increasing aural complexity. They all use TEXTURE POSTORNATE and demonstrate using:

- four different soundfiles as inputs
- time-specified soundfile entry
- a nodal substructure (defines times and pitches) which illustrates separate notes, overlapping motifs, and parallel dyads
- various motifs
- each motif begins at time 0

[Aside: I tried starting each motif at non-zero times, attempting to match them with the soundfile entry times. This did not work – if more than one motif, one of them was randomly chosen at any nodal substructure time. However, I discovered that the nodal substructure could start at a non-zero time, e.g., if the first line sets the start time to 5.0 seconds, the sound produced was silent until the first motif came in at 5 seconds. There's probably a composition application lurking there somewhere.]

The **first** of these four examples uses the four input soundfiles in a symmetric pattern, one pair of nodes 0.001 second apart to create dyads, and a 4 second *skiptime* to make the entries aurally clear, though there is some overlap because of the length of the descending scale motif. Below are the time-varying breakpoint files that time the soundfile entries. They are used for all four examples. Both 'first' and 'last' have the same soundfile order. (If 'first' was 1 and 'last' was 4 at a given time point, the program would randomly select one of the four for use at that time point, so a controlled randomised file selection can be achieved in this way.)

```
;posnd1st.brk  ;posndlast.brk
 0.0  1    ;sprbtob.wav          0.0  1   ;sprbtob.wav
 2.5  2    ;marbtob.wav          2.5  2   ;markbtob.wav
 4.0  3    ;marimba.wav          4.0  3   ;marimba.wav
 7.0  4    ;bellaedtbtob.wav     7.0  4   ;bellaedtbtob.wav
10.0  3    ;marimba.wav         10.0  3   ;marimba.wav
15.0  2    ;marbtob.wav         15.0  2   ;marbtob.wav
20.0  1    ;sprbtob.wav         20.0  1   ;sprbtob.wav
```

The note data file for the first example is *ndfPO1.txt* and uses only a descending scale motif.

```
60 60 60 60       [nominal reference pitches]
#2
0.000 1 63 0 0    [nodal substructure]
0.001 1 66 0 0    [minimal time gap produces dyads]

#7
0.000 1 76 96 1   [descending scale motif]
0.100 1 75 96 1
0.200 1 73 96 1
0.300 1 72 96 1
0.400 1 70 96 1
0.500 1 69 96 1
1.000 1 66 96 2
```

As expected, four nominal reference pitches are needed, one for each soundfile. As they are MPV 60, the higher pitches in the nodal substructure and the motif will cause the sounds to be transposed upwards. The motif creates a quintuplet over half a second, and then a single note on the next beat. Here is the result (listen for the soundfile symmetry): [podescendingscaletx.wav](../sounds/podescendingscaletx.wav). The full list of parameters for this example is:

```
texture postornate 5 sprbtob.wav marbtob.wav marimba.wav bellaedtbtob.wav
podescendingscaletx.wav ndfPO1.txt 20 4 posnd1st.brk posndlast.brk 70 96
1 1.5 0 0 0 0 0 1 1 -a0.6 -p0.5 -s1 -w
```

The **second** example also uses a single motif, this time an ascending crotchet triplet, also presented in dyads, which are most clearly heard when the marimba comes in: [poascendingtriplettx.wav](../sounds/poascendingtriplettx.wav). Here is the note data file *ndfPO2.txt* with the same nodal substructure. The *skiptime* is reduced to 2 to bind the textures together a bit more.

```
60 60 60 60       [nominal reference pitches]
#2
0.000 1 63 0 0    [nodal substructure]
0.001 1 66 0 0

#3
0.000 1 60 84 2   [ascending crotchet triplet motif]
0.670 1 62 84 2
1.340 1 63 84 2
```

The **third** example puts both of the above together. However, the same soundfile order is used, so that the basic symmetry remains. Here is the result: [pomultitx.wav](../sounds/pomultitx.wav).

This note data file *ndfPO3.txt* uses the same nodal substructure and both motifs.

```
60 60 60 60       [nominal reference pitches]
#2
0.000 1 63 0 0    [nodal substructure]
0.001 1 66 0 0

#3
0.000 1 60 84 2   [ascending crotchet triplet motif]
0.670 1 62 84 2
1.340 1 63 84 2

#7
0.000 1 76 96 1   [descending scale motif]
0.100 1 75 96 1
0.200 1 73 96 1
0.300 1 72 96 1
0.400 1 70 96 1
0.500 1 69 96 1
1.000 1 66 96 2
```

To round things off a **fourth** example includes the two motifs used above and adds 2 more. It also increases the complexity by having more time points and pitch change in the nodal substructure. *Skiptime* remains set at 2 seconds. While what is going on is still discernible in [pomorecomplextx.wav](../sounds/pomorecomplextx.wav), I hope that it indicates how pushing this and that a bit further can lead towards all sorts of possibilities.

The note data file for this example is *ndfPO4.txt*:

```
60 60 60 60      [nominal reference pitch for four sounds]
#11
0.000 1 60 0 0   [nodal substructure]
1.000 1 70 0 0
1.500 1 66 0 0
2.500 1 69 0 0
3.000 1 63 0 0
5.000 1 63 0 0
5.010 1 66 0 0
6.000 1 76 0 0
6.010 1 73 0 0
7.000 1 78 0 0
7.010 1 76 0 0

#2
0.000 1 60 84 2   [augmented 4th motif]
1.500 1 66 84 2

#6
0.000 1 66 88 1   [sextuplet motif]
0.170 1 67 88 1
0.340 1 69 88 1
0.500 1 66 88 1
0.670 1 67 88 1
0.840 1 69 88 1

#3
0.000 1 60 90 2   [ascending crotchet triplet motif]
0.670 1 62 80 2
1.340 1 63 70 2

#7
0.000 1 76 96 1   [descending scale motif]
0.100 1 75 96 1
0.200 1 73 96 1
0.300 1 72 96 1
0.400 1 70 96 1
0.500 1 69 96 1
1.000 1 66 96 2
```

Parameters for the fourth example:
`texture postornate 5 sprbtob.wav marbtob.wav marimba.wav bellaedtbtob.wav pomorecomplextx.wav ndfPO4.txt 20 2 posnd1st.brk posndlast.brk 70 96 1 1.5 0 0 0 0 0 1 1 -a0.7 -p0.5 -s1 -w`

More ideas.

- Games with transposition could be played by making the nominal reference pitches different. How close the reference pitches are to the actual pitches of the respective soundfiles shouldn't matter. The reference pitch is 'nominal' in the sense that whatever it is, it is taken as the root pitch of the soundfile, and all transposition specified by different pitches in the nodal substructure or motif(s) is relative to that.

- The way motifs overlap is controlled by the times of the nodal substructure lines and by the duration of the motifs.

- *Skiptime* can also be lengthened or shortened, or made time-varying – but remember that it only relates to the time gap between iterations of the nodal substructure. Some overlap can be achieved by having a tiny *skiptime* such that the next iteration will begin before the last motif has ended.

- Harmonic Fields or Sets can be used to create sonorous effects.

- The *multiplier* parameters (*multlo/multhi*) can speed up or slow down the tempo, or can cause each motif to be played at a tempo randomised within a range, which loosens up the texture, sometimes chaotically depending on the rhythms of the motifs. I couldn't resist a trial with *multlo* 0.3 and *multihi* 0.8 (values below 1 increase the tempo), and *skiptime* reduced to 0.5 to create a little more overlap: [pomorecomplextxtv.wav](../sounds/pomorecomplextxtv.wav).

- The sonic qualities of the input soundfiles are a crucial factor, and more often than not the original source is transformed so that it becomes fertile ground for achieving specific textural results. One's abilities to imagine new sounds and textures are challenged. As my first composition teacher, Edgar Curtis, recommended: Be still. Listen. Imagine.

---

Last updated: 25 August 2022
(c) 2022 Archer Endrich Plymouth UK
