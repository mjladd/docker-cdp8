# Mingling Involving Analysis Windows

*by Dr Archer Endrich*

*Making the most of analysis file data*

| [Introduction](#INTRODUCTION) | [Interleave](#INTERLEAVE) | [Max Amplitude](#MAXAMPWINDOWS) |
|---|---|---|
| [Convolution](#FASTCONV) | [Shuffle](#SHUFFLE) | [Weave](#WEAVE) |

## Introduction {#INTRODUCTION}

Mingling can also involve manipulating the 'windows' in which the analysis data is grouped and stored. These are very short segments of the soundfile which contain *frequency and amplitude* data. Put together, they describe the ever-changing spectral envelope of the sound, which defines its 'colour': its timbral characteristics. The windows of different sounds can be combined in various ways, and the windows of the same sound can be rearranged.

## Interleave Windows {#INTERLEAVE}

**Interleave Windows** – [COMBINE INTERLEAVE]

COMBINE INTERLEAVE is a Spectral Domain program and requires two analysis file inputs. This means that the 'Two Inputs' issue comes up, *q.v.* above. The parameter is *leafsize*, which means the number of windows to take alternately from each analysis file. Smaller values give a finer-textured interleaving (try '5'). The degree of contrast between the input files affects the result.

SL: `COMBINE->interleave`
SSh: `Spectral->Combine->Interleave`

and enter a value for *leafsize*. The soundfile [balsamjoinr.wav](../sounds/balsamjoinr.wav) is analysed to make balsamjoinr.ana and then interleaved with countana.ana with *leafsize* = 5 to make balscntinterl.ana which is put through PVOC Synthesise to make the soundfile [balscntinterl.wav](../sounds/balscntinterl.wav). The output duration is that of the shortest input.

## Maximum Amplitude Windows {#MAXAMPWINDOWS}

**Maximum Amplitude Windows** – [COMBINE MAX]

Another way to mingle analysis files is to select, on a window-by-window basis, the analysis window from the file which has the highest amplitude in that window. This results in a seamless and sometimes unpredictable mix of the two sounds – the more contrasting the inputs, the more unpredictable the results, but both need to have fairly equally high levels of amplitude for it to work, ideally spikes of amplitude that are differently placed in each file. (The composer Rob Waring got a spectacular result with this program while he was visiting years ago which I have never been able to reproduce, but keep trying!) NB: two (analysis file) inputs!

SL: `COMBINE->windowwisemax`
SSh: `Spectral->COMBINE->maximum`

There are no parameters, so it runs right away. Balsamjoinr.ana and countana.ana to form [balscntmax.wav](../sounds/balscntmax.wav) (after resynthesis).

## Convolution {#FASTCONV}

**Convolution** – [FASTCONV]

Convolution is normally used to impose specific reverberation characteristics on a sound by applying an 'impulse file'. However, as the impulse file can be a text file (.txt) OR a soundfile, its algorithm can also be used to mingle any two soundfiles. For example, [horn.wav](../sounds/horn.wav) (5.386 sec.) was convolved with [wavesdt.wav](../sounds/wavesdt.wav) (10.269 sec.) with this result: [horn-conv-waves.wav](../sounds/horn-conv-waves.wav).

The FASTCONV parameters for this were (-a is the flag for amplitude gain, and the 0.7 at the end is a dry-to-wet scale on a 0 to 1 range):

```
fastconv -a1.5 horn.wav wavesdt.wav 0.7
```

Impulse files can be readily garnered from the Net. One set is available via the 'Darkside 50' page Richard Dobson's Website:

[http://www.rwdobson.com/sspaces/sciencespaces.html](http://www.rwdobson.com/sspaces/sciencespaces.html)

The sounds in the zip download are in a 4-channel ambisonic format, so need a bit of attention before they can be used with other CDP soundfiles. Given that the impulse file is the 4-channel ambisonic `cylinder11A_amb.wav` at 48K SR and the CDP file to be used at is at 44.1 SR, 16-bit shorts, a brief summary of the procedure to do this is:

1. EXTRACT the four channels to a base name (the .wav extension is required and all four channels are named)
   `channelx cylinder11A_amb.wav -ocylch.wav 1 2 3 4`

2. CUT to a reasonable length (6 seconds)
   `sfedit cut 1 cylch3.wav cylch3c.wav 0 6`

3. CONVERT sample rate (SR) from 48000 to 44100
   `housekeep respec 1 cylch3c.wav cylch3c44.wav 44100`

4. CONVERT from 24-bit packed to 16-bit shorts
   `copysfx -s1 cylch3c44.wav cylch3c16bit.wav`

The amplitude level of cylch3c16bit.wav was very low (0.09), so I applied an 8x gain to it: [cylch3c16bitgx8.wav](../sounds/cylch3c16bitgx8.wav). Even so, when convolved with the horn (or any other file), the reverb part was barely audible.

## Shuffle Analysis Windows {#SHUFFLE}

**Shuffle Analysis Windows** – [BLUR SHUFFLE]

SL: `BLUR -> shuffle`
SSh: `Spectral -> TIME -> Shuffle`

BLUR SHUFFLE is a fairly straightforward file, taking an input analysis file and producing an output analysis file. The windows of the input analysis file are shuffled according to a *domain-image*, such as **abc-cba**. The *image* part must to use the same letters as the *domain* but rearranges them any which way, with or without duplicate letters. The *groupsize* parameter makes each letter stand for *N* analysis windows.

The first example produces a slow wave-like motion: [sprbtobshuf4.wav](../sounds/sprbtobshuf4.wav). The input sound is [sprbtob.wav](../sounds/sprbtob.wav) and the *domain-image* for this example is **abcdef-aaafffcdefffaaa** with *groupsize* 3.

The second example, using the same infile, produces the more disjunct sound [sprbtobshuf7.wav](../sounds/sprbtobshuf7.wav) because the letters stand for bigger chunks of windows: *groupsize* is set to 40. The *domain-image* for this example is **abcdef-afbfcfdfeffefdfcfbfa**, which jumps around rather a lot.

BLUR SHUFFLE is a fun program to explore. It tests ingenuity in the *image* construction and can extend a sound in interesting ways.

## Weave Windows {#WEAVE}

**Weave Windows** – [BLUR WEAVE]

BLUR WEAVE is similar in that it moves analysis windows around. This time the pattern is defined in a text file, the *weavefile* parameter.

SL: `BLUR -> weave`
SSh: `Spectral -> TIME -> Weave`

The weave pattern consists of steps by which one moves forwards or backwards a certain number of windows, and then that single window is written to the *outfile*. Backwards is indicated by a minus number. A step is the number of windows to move. The window rearrangement pattern established in the *weavefile* moves forward and is repeated until the end of the analysis file is reached.

For example, 7 -3 would select the window 7 steps ahead, and then the window 3 steps back from that point. The range of steps is limited to 127 forwards and 128 backwards, with the understandable provisos that no move can go back *before the start* window of the weave and the final window must be *after the start* window of the weave.

Analysis windows are in the order of 0.02 seconds long, so rearranging nearby windows doesn't alter the sound as much as one might expect. As the maximum step (forwards) is 127, 127 * 0.02 = 2.54 seconds, so there is scope for more disjointed rearrangements.

This process can create output files that can be shorter or longer than the input file. The number of moves in the *weavefile* divided by the number of the *last window moved to* gives the ratio of duration change. Just how this works is illustrated in the examples below. Simple arithmetic calculates at which window the moves ( i.e. *steps* as +/- numbers) in the *weavefile* terminate. One of the most observable effects is the stretching that occurs when the moves hover around the same area for a while. The source sound for these examples is again [count.wav](../sounds/count.wav), 8.066 seconds in duration.

**Example 1 - a shorter outfile**: a perky [countweave1.wav](../sounds/countweave1.wav) Here is *weavefile1.txt* with the windows-moved-to shown below in [ ] brackets:

```
 3 7  -4 (three moves)
[3 + 7 = 10 -4 = 6] (six is the last window; window sequence is 3 10 6)
```

3 (moves) / 6 (last window) = 0.5: the outfile is half the length of the infile.

**Example 2 - a longer outfile**: a drawled [countweave2.wav](../sounds/countweave2.wav) Here is *weavefile2.txt* with the windows-moved-to shown below in [ ] brackets:

```
 3  -1   2  -1   2   -1   -1   1   -2   1  (ten moves)
[3    2   4   3   5    4    3    4    2   3] (three is the last window)
```

10 (moves) / 3 (last window) = 3.33: the outfile is 3x the length of the infile.

**Example 3 - a much longer outfile**: a laborious [countweave3.wav](../sounds/countweave3.wav) Here is *weavefile3.txt* with the windows-moved-to shown below in [ ] brackets:

```
 1  1  1  1  1  1  -1  -1  -1  -1  -1  (eleven moves)
[1    2  3  4  5  6   5   4   3   2   1] (one is the last window)
```

11 (moves) / 1 (last window) = 11: the outfile is 11x the length of the infile.

In Example 2 the intake and exhalation of breath that surrounded the sounding letters began to be audible. In the 11x expanded Example 3, the breath has become prominent. Again, we have an example of a transformation that can provide source material for something else besides being interesting in its own right. Little chunks of that breath could come in handy...

**Example 4 - windows further apart**: a burbly [countweave4.wav](../sounds/countweave4.wav) It uses *weavefile4.txt* with the windows-moved-to shown below in [ ] brackets:

```
 3  10  -3  10  -3  10  -3  20  -6  20  -6  20  -6  10  -50  3  -20  (17 moves)
[3    13  10  20  17  27  24  44  38  58  52  72  66  76   26 29   9]  (9 is the last window)
```

17 (moves) / 9 (last window) = 1.89; the outfile is 1.89x the length of the infile.

In the 4th Example, the sound is only modestly lengthened, but enough to enhance the more disjointed window arrangement. Some experiments with larger numbers somehow produced a soundfile that was too short to play – perhaps the large numbers prevented the pattern from repeating enough to produce an outfile as long as it should have been. This matter remains unresolved but is mentioned for your information.

[**RETURN**](index.md#TOPIC4) to A Learning Manual for CDP, Topic 4

---

Last updated: 25 August 2022
