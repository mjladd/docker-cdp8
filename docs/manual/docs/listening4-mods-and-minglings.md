# Listening4-Mods and Minglings

*by Dr Archer Endrich*

*with an emphasis on vocal source material*

| [Source Sounds](#SOURCES) | [Set 1 - Internal](#SETONE) | [Set 2 - Filtering](#SETTWO) |
|---|---|---|
| [Set 3 - Mingling](#SETTHREE) | [Set 4 - Blurring (and Tuning)](#SETFOUR) | |

## Source Sounds {#SOURCES}

This group of sounds comes from several of the Worksheets in *CDP Tutorial Workshop 1* where the examples transform vocal sounds. The all but one [vocal source sounds](../sounds/vocalsources.wav) are:

1. [capm.wav](../sounds/capm.wav): "The extraticular ... "
2. [count.wav](../sounds/count.wav): "One, two ..."
3. [femwheeze.wav](../sounds/femwheeze.wav): lowered exhalation
4. [gongvib.wav](../sounds/gongvib.wav): gong with time-varying vibrato
5. [oingseq.wav](../sounds/oingseq.wav): several glissandoing warbles

## Set 1 - Internal {#SETONE}

There are two examples from Worksheet 1 (tracks 15 and 16): [clip4-wksh1.wav](../sounds/clip4-wksh1.wav)

1. STRANGE SHIFT - [capmtvssm1.wav](../sounds/capmtvssm1.wav): time-varying spectral shift that squeezes the partials closer together as it rises. This is the time-varying breakpoint file that was used for the *frequency shift* parameter:

   ```
   time   frequency shift
   0.0    0
   1.0    1000
   3.99   1000
   4.0    300
   7.2    0
   ```

2. FOCUS ACCUMULATE - [capmaccu_01.wav](../sounds/capmaccu_01.wav): internal glissandi with 0.01 for both the *decay* and the *gliss* parameters

## Set 2 - Filtering {#SETTWO}

The examples from Worksheet 2 use three different FILTER routines: [clip4-wksh2.wav](../sounds/clip4-wksh2.wav)

1. FILTER PHASING - [capmphasm2tv.wav](../sounds/capmphasm2tv.wav): with a time-varying reverberant effect. *Gain* was 0.25 and the *delay* parameter (ms) used this timve-varying breakpoint file:

   ```
   time   frequency shift
   0.0    300
   1.5    200
   3.0     80
   4.5     80
   6.0     40
   7.0     20
   ```

2. FILTER ITERATED - [capmblowit6ps.wav](../sounds/capmblowit6ps.wav): *Q* 75 (higher is tighter), *delay* 0.25 and randomised pitch shift: the sound dissolves as it rises and is repeatedly filtered

3. FILTER SWEEPING (mode 2) - [capmswm2.wav](../sounds/capmswm2.wav): a downward lo-pass filter sweep between 1000 and 200 Hz. *acuitry* is 0.05, *gain* is 0.3 and the *sweepfrq* is 0.1

## Set 3 - Mingling {#SETTHREE}

The examples from Worksheet 8 are in the 'mingling' category: [clip4-wksh8.wav](../sounds/clip4-wksh8.wav)

1. COMBINE CROSS - [fem-ccross-oing.wav](../sounds/fem-ccross-oing.wav): gradual spectral amplitude replacement

2. MORPH MORPH - [fem-m-oing.wav](../sounds/fem-m-oing.wav): transition by spectral interpolation (full length)

3. FORMANTS VOCODE - [gongvoccount.wav](../sounds/gongvoccount.wav): voice speaks with the sound of the gong

4. MORPH MORPH - [gongvoccount-m-count.wav](../sounds/gongvoccount-m-count.wav): the normal voice sound emerges from the gong-voice

## Set 4 - Blurring (and Tuning) {#SETFOUR}

And we finish with three examples from Worksheet 9 that use BLUR: [clip4-wksh9.wav](../sounds/clip4-wksh9.wav)

1. BLUR BLUR - [countblur70.wav](../sounds/countblur70.wav): blurred speech (70 analysis windows at a time are blurred)

2. BLUR CHORUS - [countchorus.wav](../sounds/countchorus.wav): one voice becomes many bubbling voices (Mode 5 with amplitude spread 100 and frequency spread 1.2)

3. BLUR BLUR - [counttuneAminbl70.wav](../sounds/counttuneAminbl70.wav): on a [previously tuned voice](../sounds/counttuneAmin.wav), the added blurring reduces the vocal articulation and makes it more chord-like

[**RETURN**](index.md#TOPIC4) to A Learning Manual for CDP, Topic 4

---

Last updated: 19 August 2021
