# Mingling via Sonic Morphs

*by Dr Archer Endrich*

*Super-Transitions*

| [About Morphing Sounds](#INTRODUCTION) | | |
|---|---|---|
| [MORPH MORPH](#MORPH) | [MORPH BRIDGE](#BRIDGE) | [MORPH GLIDE](#GLIDE) |

## About Morphing Sounds {#INTRODUCTION}

The aim of a morph is to make a transition between (usually similar) sounds which feels like a *transformation* of the first sound into the second sound (rather than a simple cross-fade). This is not easy to achieve in sound because the ear is so very sensitive. 24 frames a second is enough to make the eye think it is perceiving seamless motion, but high level sound sampling needs to be 44100 samples per second or double or quadruple that rate because the ear is so sensitive to minute changes. It is important to give a sonic morph plenty of time to unfurl. Sometimes it helps to design a middle step which mingles the two sounds where neither is heard clearly but both are present.

Imposing the amplitude envelope of one of the sounds onto the other sound is one way to create this middle ground. The first sound thereby assumes the rhythm of the second sound (i.e. its amplitude shape). Then the morph gradually fills in this rhythm with the sound of the second sound. Any mingling technique may find a place in the middle step of a morph.

Morphing is an advanced and difficult procedure and there are a number of things to consider, particularly *when* in the files you want the morph to take place. In MORPH the parameters *start* and *stagger* are used to set this up. Please refer to the Reference Documentation for MORPH MORPH for more information about sonic morphing.

## Morph {#MORPH}

**Morph** – [MORPH MORPH]

Although MORPH works most readily with similar sounds, here contrasting sounds are used to try to illustrate the process more clearly: the sound of a tractor engine [trcdtg.wav](../sounds/trcdtg.wav) and the [count.wav](../sounds/count.wav) sound used in many other examples. These are converted to analysis files before submitting them to MORPH.

SL: `MORPH->morph->linear`
SSh: `Spectral->MORPH/FORMANTS->morph`

The goal is to achieve a transformative transition from the sound of the tractor to the voice counting one to ten. (In the file naming, T = trcdtg.wav and C = count.wav. To deal with these contrasting sounds **we will use count.wav's amplitude envelope imposed on the tractor sound as the middle ground** (i.e, 'countenvelope-on-tractor' or in shorthand, 'CenvonT').

To do this, ENVEL EXTRACT mode 1 is used to extract the amplitude envelope from [countr.wav](../sounds/countr.wav) with *wsize* 5, forming countenv.evl. (The backwards version of count.wav worked well for the envelope and added a little bit of abstraction.) ENVEL IMPOSE mode 4 was then used to impose countenv.evl onto trcdtg.wav to make [CenvonT.wav](../sounds/CenvonT.wav), then analysed to **.ana** for use later. The tractor sound now has the rhythm of the vocal sound.

Using this middle ground, the morph is done in three steps:

1. from tractor to middle ground,
2. from middle ground to count,
3. result of step 1 morphed to result of step 2.

The salient question is when to overlap the files, which usually takes a bit of trial and error to get the most convincing results.

Planning a morph is a lot like planning a mix because overlapping files are involved. The suggestion is to draw it out on paper and imaginatively try to hear the files combining, just as a chef imagines the result when different amounts of ingredients are combined. The plan here is to use the three stages outlined above. All the morphs here use linear transitions – they could be exponential. 'Mto' means 'morph to'. Analysis files are converted to soundfiles here so they can be auditioned in HTML, but MORPH uses only analysis files.

- **Step 1)** morph from trcdtg.ana to middle stage CenvonT.ana to make T-Mto-CenvonT.ana ([T-Mto-CenvonT.wav](../sounds/T-Mto-CenvonT.wav)). We don't hear the words of the count, just the envelope shape. In this first morph, both files begin at the same time (no *stagger/offset*), but the amplitude transition begins at 2.5 sec and the frequency transition at 4.0 sec. We hear the tractor-only sound changing to tractor sound with the rhythm of the words.

- **Step 2)** morph from the middle stage CenvonT.ana to count.ana to make CenvonT-Mto-C.ana ([CenvonT-Mto-C.wav](../sounds/CenvonT-Mto-C.wav): 'envelope on tractor morph to count'. In this second morph, the second file begins at 1.5 (*stagger*: comes in 1.5 seconds after the start of the first file), the amplitude transition also starts at 1.5, and the frequency transition a little later at 2.5 (leaving plenty of time for the count to emerge clearly at the end). We hear the tractor sound with the rhythm of the words changing to the words spoken clearly.

- **Step 3)** morph between the two above resultant files: from T-Mto-CenvonT.ana ('tractor **morphed to** countenvelope-on-tractor') morphed to CenvonT-Mto-C.ana ('countenvelope-on-tractor **morphed to** count) to make the complete morph T-Mto-CenvonT-Mto-C.ana: [T-Mto-CenvonT-Mto-C.wav](../sounds/T-Mto-CenvonT-Mto-C.wav) (to form trcdtgtocountmorph.ana: tractor to middle ground, middleground to count). The parameters were set as in Step 2. Perhaps the morph in Step 1 could have begun sooner. We hear the tractor sound become rhythmic (the middle ground) and then hear the words gradually emerge until only clearly spoken words are heard.

Notice how in the 'middle ground' the voice sounds quite rough in the middle (has taken on aspects of the tractor sound) and the tractor sound starts to pulsate with the rhythm of the voice before the voice emerges speaking clearly.

## Bridge {#BRIDGE}

**Bridge** – [MORPH BRIDGE]

The MORPH BRIDGE procedure as well as the next one (MORPH GLIDE) creates a transition between the spectral components contained in the two analysis windows selected.

SL: `MORPH->bridge`
SSh: `Spectral->Morph/Formants->Bridge`

Best results – a sense of transition – take place when the sonic material in these two analysis windows is strongly contrasting, so select them with care. SPEC GRAB is used to extract a single analysis window at a specified time. In *Soundshaper* the 'grab' part is built into BRIDGE and you just have to specify the *start* and *end* times, and if you open a .wav file as the second file, it is automatically converted to the required analysis file input. You can open the same or a different sound as the second file.

SL: `SIMPLE->grab window`
SSh: `Spectral->Spec UTILS->Grab`

Note that the time in the second file *does not have to be later than the time in the first file*. Let's try *time* 1.1 in balsamana.ana and *time* 0.1 in clashana.ana. The result is a nice transition from the word 'Nature' to tinkly bits of the clash sound: balsambridgeclash.ana resynthesised to [balsambridgeclash.wav](../sounds/balsambridgeclash.wav).

## Glide {#GLIDE}

**Glide** – [MORPH GLIDE]

MORPH GLIDE is great for long, slow transitions.

SL: `MORPH->glide`
SSh: `Spectral->Morph/Formants->Glide`

It makes a transition between two single analysis windows, each taken from the same or different sounds with SPEC GRAB – more contrast gives a better result. We can use the same files and times as BRIDGE but are able to make an *outlength* of 20 sec. The result is 20 seconds of slow, subtle tonal changes: balglideclash.ana resynthesised to [balsamglideclash.wav](../sounds/balsamglideclash.wav). In *Soundshaper* the 'grab' part is built into GLIDE and you just have to specify the *From* and *To* times, and if you open a .wav file as the second file, it is automatically converted to the required analysis file input. You can open the same or a different sound as the second file.

[**RETURN**](index.md#TOPIC4) to A Learning Manual for CDP, Topic 4

---

Last updated: 19 September 2021
