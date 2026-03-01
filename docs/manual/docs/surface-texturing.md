# Surface Texturing

*by Dr Archer Endrich*

## Surface Texturing {#INTRODUCTION}

*alterations to the apparent 'surface' of the sound*

| **This is a very small selection!** | | |
|---|---|---|
| [**About 'Surface'**](#INTRODUCTION) | [**Adding Shimmer**](#SHIMMER) | [**Chorusing Effects**](#CHORUS) |
| [**Random Chunks**](#RANDCHUNKS) | [**Segmentations**](#SEGMENTS) | [**Wavecycle Distortion**](#DISTORT) |

**About Surface** {#INTRODUCTION}

Altering the surface texture of a sound can give it more impact and, depending on the degree of texturing, diminish the ability to recognise the original source sound, thereby enabling more abstract formal constructions. The use of the word 'surface' is fuzzy. There is no clear distinction between the 'outside' and 'inside' of a sound. What I mean here are those transformations that affect the apparent texture of a sound, such as segmentation and re-ordering, rather than those processes that change its internal spectral structure or harmonicity.

**Adding Shimmer** {#SHIMMER} – [TEXTURE SIMPLE]

The goal here is to create a long, full, subtly shimmering tone. To do this, we need to use TEXTURE SIMPLE, which is covered in Topic 5. Its first sub-topic discusses the creation of strong, rich tones. Rather than get ahead of ourselves, here we are just going to illustrate a subtle change of surface by playing two of the sounds detailed in that document. How to use the TEXTURE Set of programs will be saved for later.

First is a strong tone: [bellaedtbtobtone.wav](../sounds/bellaedtbtobtone.wav) made with TEXTURE SIMPLE. The second is a subtly shimmering version of the same tone: [bellaedtbtobshimmertone.wav](../sounds/bellaedtbtobshimmertone.wav). The difference is this: the first sound reiterates its source every 0.1 second (lots of overlap!) using the *same pitch* for the *pitchlo* and *pitchhi* parameters (MPV 67, G above Middle-C).

The shimmering version only changes the pitch, giving it a tiny pitch range from 66.9 to 67.1 MPV – note that these microtonal pitches can be specified in TEXTURE (and CDP generally). A subtle beat pattern is introduced as the overlapping iterations of the source are transposed randomly within this microtonal range. See [*How to Create Strong Tones*](how-to-create-strong-tones.md) for full details.

**Chorusing Effects** {#CHORUS} – [BLUR CHORUS]

SL: `BLUR->chorus->scatter amps & freqs`
SSh: `Soundfiles->FREQ/PITCH->chorus`

A more complex 'chorusing' effect can be made with BLUR CHORUS. We will set only 2 of the available parameters: *amplitude* to 100 (out of 1024) and *frequency* to 1.4 (out of 4). Each source has been converted to an analysis (.ana) file before processing with BLUR CHORUS, and then resynthesised back to .wav afterwards. Bellaetone.wav was stereo after the TEXTURE SIMPLE processing, so it was converted to MONO [HOUSEKEEP CHANS, Mode 4] before anlaysing i, hence the 'm' in the name. Clash.wav was time-stretched by a factor of 3 before subkitting it for chorusing. Compare what happens with several source files with which we are now familiar:

- Source (1 to 10): [count.wav](../sounds/count.wav)   Chorused: [countchorus.wav](../sounds/countchorus.wav)
- Source (count reversed): [countr.wav](../sounds/countr.wav)   Chorused: [countrchorus.wav](../sounds/countrchorus.wav)
- Source (the long tone): [bellaedtbtobtonem.wav](../sounds/bellaedtbtobtonem.wav)   Chorused: [bellaedtbtobtonemchorus.wav](../sounds/bellaedtbtobtonemchorus.wav)
- Source: [clashmx.wav](../sounds/clashmx.wav)   Time-stretched x 3: [clashmxstrx3.wav](../sounds/clashmxstrx3.wav)   Chorused: [clashmxstrx3chorus.wav](../sounds/clashmxstrx3chorus.wav)

**Random Chunks** {#RANDCHUNKS} – EXTEND SCRAMBLE Mode 1

This process will cut out chunks of soundfile and rearrange them randomly.

SL: `EXTEND->scramble->completely random`
SSh: `Soundfiles->EXTEND/SEGMENT->scramble`

Typical chunk sizes are 0.1 to 0.2 and can be slightly smaller (they need to be long enough to accommodate a splice) or considerably larger (hear more of the source in each chunk). This program can be a good starting point for starting to turn the source sound into something more abstract. An example would be to run [countr.wav](../sounds/countr.wav) (reversed [count.wav](../sounds/count.wav) – notice that the first word you hear is 'net', i.e. 'ten' backwards). Countr.wav is already disguised by being speech backwards. With *segments* from 0.1 to 0.2, *duration* 16 seconds, *splice* 25ms, *seed* 0, you hear stuttering incoherent speech: [countrrandchunks1.wav](../sounds/countrrandchunks1.wav)). With segments between 0.2 and 0.4 (nearly half a second), it sounds more like normal speech, but with strange words: [countrrandchunks2.wav](../sounds/countrrandchunks2.wav). (I'm fond of these!)

**Segmentations** {#SEGMENTS} – [EXTEND ZIGZAG]

There are many other processes in EXTEND to explore. ZIGZAG mixes randomised forward and backward movement in the soundfile.

SL: `EXTEND->zigzag->random`
SSh: `Soundfile->EXTEND/SEGMENT->Zig-Zag`

This function Applied to [countr.wav](../sounds/countr.wav), notice how some of the backwards speech comes out forward (the words 'ten' and 'nine': backwards movement on a reversed word = forwards). The *start* and *end* parameters are automatically set to the whole length of the source sound, but can be altered. Try *minzig* 0.04 and *maxzig* 1.0, with a 10 second output soundfile: [countrzigzag.wav](../sounds/countrzigzag.wav). The effect is more abstract when applied to more complex, non-verbal sounds.

**Wavecycle Distortion** {#DISTORT} – [DISTORT MULTIPLY]

The DISTORT suite of programs provide many different ways to make a sound bubble and squeak. The wave-cycles are the times between zero-crossings of the wavecycle. Unless it is an exceedingly regular sound, these are of unpredictably uneven lengths. One example here:

SL: `DISTORT->multiply`
SSh: `Soundfiles->DISTORT Cycles->multiply`

Apply a *multiplier* of 5 to the tractor sound [trcdtg.wav](../sounds/trcdtg.wav) and you will hear it turn into a higher-pitched bubbling: [trcdtgdistortmultx5.wav](../sounds/trcdtgdistortmultx5.wav). A more irregular sound such as [seven.wav](../sounds/seven.wav) will sound more broken up: DISTORT MULTIPLY with *multiplier* 3 produces [sevendistmult.wav](../sounds/sevendistmult.wav). DISTORT REPEAT with *multiplier* 4 and *number of wavecycles in a group* 5 produces [sevendistrpt.wav](../sounds/sevendistrpt.wav). Fewer wavecycles in a group uses less of the input soundfile at a time and therefore will sound more irregular.

Much more should be said about the DISTORT series of programs because they are one of the most original and powerful of Trevor Wishart's creations. They wreak havoc with sounds, making them useful when you need to create gritty and more abstract material. DISTORT MULTIPLY gives some idea of what is possible, but there are many other programs in this set. Some more examples will follow below when discussing sound design and sounds far removed from their source.

---

Last updated: 23 August 2021
