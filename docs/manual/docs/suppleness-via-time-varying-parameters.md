# Suppleness via Time-varying Parameters

*by Dr Archer Endrich*

## Suppleness via Time-varying Parameters {#INTRODUCTION}

*Two examples and a suggestion for further study*

| [**Time-varying: fundamental to musical design**](#TIMEVARYING) | | |
|---|---|---|
| [**Prepare the sound(s)**](#PREPARESOUND) | [**The upper texture**](#UPPERTEXTURE) | [**The lower texture**](#LOWERTEXTURE) |
| [**The mix**](#THEMIX) | [**Time-varying distortion**](#TVDISTORT) | [**Further study: DRUNK**](#EXPLOREDRUNK) |

**Time-varying: fundamental to musical design** {#TIMEVARYING}

Music is the art of time. When we hear acoustic instruments, the tones we hear are not static. They are *dynamic* in the sense that they are constantly changing. How a tone starts, how for example a string or trumpet tone blooms and fades, whether it changes to a tremolo or a flutter-tongue, how vibrato is applied – all these 'articulations' bring the note event to life. Overall, an orchestral sound changes in tempo, instrumental combinations, and textural density.

The same requirement for motion over time applies to electroacoustic music, but is perhaps not so easy to apply when the intervention of a human performer is not present. This is why *time-varying* facilities are so essential (sometimes referred to as 'automation'), and in the CDP software, almost every parameter can change over time. Making use of these facilities will help to give life to your musical creations. We therefore examine some examples of time-varying processes more closely to show how change over time is achieved with the CDP software.

**Prepare the sound(s)** {#PREPARESOUND}

The following example builds a time-varying multi-event texture from a single source, the cry of a hawk. The original source is modified by applying a *gain* of 3x (rather a lot) and then cut at 1.49 sec to make [hawkgc.wav](../sounds/hawkgc.wav). To make a contrasting sound, a lion-like roar, this same hawkgc.wav is lowered an octave and then that sound is lowered by another 18 semitones to make [hawkgcd30.wav](../sounds/hawkgcd30.wav): it has been lowered in two stages by a total of 2 1/2 octaves. Two 30 second time-varying **textures** are made with TEXTURE SIMPLE from these two sounds and then very simply mixed together with MERGE TWO, which does not need a mixfile. (TEXTURE is covered in Topic 5.)

**The upper texture** {#UPPERTEXTURE}

The upper texture [hawkgchitx1.wav](../sounds/hawkgchitx1.wav) is made from [hawkgc.wav](../sounds/hawkgc.wav) and is like a flight of birds rising in a group. The time-varying features – pan (in TEXTURE) from 0 (L) to 1 (R), pitch transposition upwards, and *packing* (in seconds or parts of seconds) that expands – suggest that they are flying from Left to Right until they are out of sight, all the time getting higher and with their calls becoming more sparse and distant (softer).

The pitch in the note data file for the upper texture was given as 72, so 72 in the pitch file means no change. Note the slight slowing in the packing over 30 seconds, the rising pitch level, with the pitch range increasing to 6 semitones at the end, the gradual attenuation and Left->Right pan over the 30 seconds (*position*). The *spread* parameter is set to 0.25 to concentrate the sound as it moves across the 0 to 1 pan range. *Scatter* is 0.31 to randomise the timing of the event onsets. Here are the time-varying values in the files for the higher texture:

```
time packing pchlo pchhi atten position
0    0.2     68    72    0.9   0
10           76    80
20           76    80
30   0.4     82    80    0.2   1
```

**The lower texture** {#LOWERTEXTURE}

The lower texture [hawkgcd30lotx1.wav](../sounds/hawkgcd30lotx1.wav) has strange roars (made from the same hawk sound), [hawkgcd30.wav](../sounds/hawkgcd30.wav). There are slightly different pitches and the roars come from different places in the stereo field. The roars are a little more widely spaced towards the middle of the output sound (*packing* changes from 3 second intervals to 4 second intervals, then back to 3).

In the lower texture the pitch in the ndf file was given as 48, and a pitch range set between 47 and 51 (note event pitch levels are randomised between these two pitches). The other parameter values are constants: *attenuation* is a constant 0.9, *position* is 0.5 (the middle) with a *spread* of 1 to fill the whole L-R range. *Scatter* is a rather large 1.7 to make the next 'roar' more unpredictable. The only time-varying file for the lower texture is therefore for *packing*:

```
time packing
0    3
8    4
22   4
30   3
```

**The mix** {#THEMIX}

These two sounds are mixed together with MERGE TWO, giving the lower sound first and the higher sound second. This is so that *stagger* can make the higher sound come in 1.5 seconds after the lower sound – the roars frighten the birds as it were. The roars were too quiet on the first run, so it was done again with *skew* set to 1.2, making the first sound, the roars, a bit louder. This improved the balance, but then the whole sound was gained by 2. The MERGE TWO result is: [hawkgclohitx1merge2g.wav](../sounds/hawkgclohitx1merge2g.wav). This result could use some more variety in the modifications to the source sound so that the end result would be less repetitive. However, the example should be clear enough to be able to pick out the various time-varying changes taking place. It helps to listen to it while looking at the contents of the breakpoint files.

**Time-varying distortion** {#TVDISTORT}

Moving towards the more abstract, DISTORT REPEAT provides some interesting results. The source sound is [seven.wav](../sounds/seven.wav), 1.7 seconds long. The program repeats groups of wavecyles *via* the parameter *repeat*, with a certain number of wavecycles in each group (*cycles*. Here is the result with *repeat* = 4 and *cycles* = 10: [sevenrpt.wav](../sounds/sevenrpt.wav). The sound is stretched out a bit due to the repeating groups of cycles, and the wavecycle distortion produces a slightly bubbly and 'blistered' surface texture.

A time-varying development of the above sound can be achieved with a breakpoint file that uses low values for *repeat*. The aim is to strike a balance between recognising the word 'seven' and disguising it. This result is [seventvrptlowvals.wav](../sounds/seventvrptlowvals.wav) and the breakpoint file is:

```
time repeat
0.0    3
0.1    4
0.2    5
0.3    8
0.4    8
0.5    5
0.6    4
0.7    3
```

This result can be greatly changed by using a more adventurous time-varying breakpoint file for the *repeat* parameter. The breakpoint file is applied to [sevenrpt.wav](../sounds/sevenrpt.wav) (2.7 sec long), which had constant values for *repeat* and *cycles*, rather than to the original [seven.wav](../sounds/seven.wav). The now time-varying result sounds like this: [sevenrpttvrpt.wav](../sounds/sevenrpttvrpt.wav). This is the breakpoint file that was used:

```
time repeat
0.0     4
0.3    10
0.6     3
1.0    12
1.3     5
1.7     8
2.0     2
2.5     7
```

**Further study: DRUNK** {#EXPLOREDRUNK}

A good program to explore as a way of honing your time-varying skills is EXTEND DRUNK, which is based on a program originally developed by Miller Puckette (of MAX\MSP fame). It makes use of a great number of breakpoint files as illustrated in my CDP tutorial for DRUNK. There are many breakpoint files to examine. The resulting sounds are available to audition and you can listen while looking at the breakpoint files. It takes practice to be able to hear what the breakpoint files are doing. Follow the link to TUTORIALS on the main page of the CDP Reference Documentation. Such a program opens up vistas of possibility – an entire piece could be written with it – and start to reveal the depth and power of the CDP software.

---

Last updated: 20 August 2021
