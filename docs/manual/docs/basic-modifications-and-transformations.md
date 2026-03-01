# Basic Modifications and Transformations

*by Dr Archer Endrich*

## Basic Modifications and Transformations {#INTRODUCTION}

*12 basic operations in alphabetical order*

| [**About taking the long view**](#INTRODUCTION) | | |
|---|---|---|
| [**Echo**](#ECHO) | [**Filter**](#FILTER) | [**Glissando**](#GLISSANDO) |
| [**Loop**](#LOOP) | [**Pan**](#PAN) | [**Reverb**](#REVERB) |
| [**Reverse**](#REVERSE) | [**Ring Modulation**](#RINGMOD) | [**Timestretch**](#TIMESTRETCH) |
| [**Trace**](#TRACE) | [**Transpose**](#TRANSPOSE) | [**Vibrato/Tremolo**](#VIBTREM) |

**Perspective – just the beginning ...taking the long view** {#INTRODUCTION}

In presenting these basic modifications and transformations, I am very aware that the text and examples remain at a fairly introductory level. They do not reveal the powerful sound sculpting abilities of the CDP software. The aim here is to enable use of the software, and to do this the illustrations can't really be all that complex. The reader would become bogged down in the detail. The CDP software is not designed for 'black box' solutions. The strength of the software comes to the fore when processes are combined, the output of one becoming the input to another. It would be good if we could start to put together and contribute to a common store some properly advanced transformations. Having said that, the following processes are effective and used frequently.

**Echo** {#ECHO} – [MODIFY REVECHO Mode 3 (stadium echo)]

Rather than just producing resonance, this 'echo' lives up to its name by producing multiple fading images of the sound.

SL: `REVERB:ECHO->rev/echo->stadium`.
SSh: `Soundfiles->REVERB/ECHO->Echo`.

The key parameter here is *count*, which is the number of echoes to produce. Audibility of the echoes is affected by *gain* and *roll-off* (= 'level loss with distance' in SL). This is a typical result with *roll-off* 0.7 and *count* 10: [countstadiumecho.wav](../sounds/countstadiumecho.wav).

**Filter** {#FILTER} – Filtering is a very big topic, with many functions available. It is a great example of subtractive processing: part of the sound is cut away, leaving the rest to 'pass' through. This implies that there needs to be something to subtract from. Filtering is more audibly effective when applied to timbrally rich sounds, including white noise!

The frequency at which the cutting away begins is the 'passband', and the frequency at which the cutting away ends is the 'stopband'. The distance between these softens the edge of the band, 'feathers' it into the rest of the sound, as one does when painting. It controls how much of the sound beyond the start of the cut is allowed through, gradually tailing off in amplitude until the stopband is reached. The width of the frequency gap between the *passband* and the *stopband* affects the 'hardness' of the edge.

Another important filter parameter is the ratio of the resonance to the bandwidth, known as 'Q', or *acuity* or *focus*. In general it controls the ability of the filter to focus on the centre frequencies, sometimes by boosting the resonance of these frequencies, and more generally by the sharpness of the angle with which the amplitude of adjacent frequencies is reduced. Different programs can handle this with varying terminology and parameter values.

- PITCH TUNE (spectral) is designed to emphasise specified pitches, thus tuning a sound to those pitches. Its *focus* parameter controls the level of emphasis and its range is 0 to 1, 1 being maximum focus, a clearer tone. Values getting closer to zero increase the fuzziness of the tone.
- TUNEVARY TUNEVARY (spectral) enables time-varying tuning by including time information along with the target pitches.
- FILTER LOHI does not have a *Q* parameter, only the *passband* and *stopband* combination. If *stopband* is above the *passband*, it lets through lower frequencies ('lo pass'), stopping the higher ones; if *stopband* is below *passband*, it lets through higher frequencies, stopping the lower ones. The gap between these, as noted above, affects the edge of the filter band.
- FILTER USERBANK has time-variable *Q* (a very important feature). In this case, the range is 0.001 to 10000 and higher values make the filter 'tighter', i.e., more focused.
- FILTER VARIABLE also has time-varying *Q* to control the acuity of the filter. This time the range is 0.0001 to 1.0 and *smaller values give a tighter filter.*
- FILTER BANK offers a variety of ways to colour a sound harmonically with: the harmonic series, alternate harmonics, subharmonics, or equal intervals.

**Examples**: FILTER VARIABLE produces the following results with [seven.wav](../sounds/seven.wav) as the input soundfile:

- **notch:** one or more bands of frequencies are cut away, in effect creating tunnels through the sound – [sevennotch.wav](../sounds/sevennotch.wav) (*acuity* 0.5, *gain* 2, *frequency* 196, and *tail* 1 sec.)
- **band-pass:** several bands of frequencies are allowed to pass – [sevenband.wav](../sounds/sevenband.wav) (*acuity* 0.2, *gain* 1.3, *frequency* 196, and *tail* 1 sec.)
- **low-pass:** the lower portion passes and the upper portion is cut away: the stopband is *above* the passband – [sevenlop.wav](../sounds/sevenlop.wav) (*acuity* 0.1, *gain* 0.8, *frequency* 196, and *tail* 1 sec.)
- **high-pass:** the upper portion passes and the lower portion is cut away; the stopband is *below* the passband – [sevenhip.wav](../sounds/sevenhip.wav) (*acuity* 0.1, *gain* 0.6, *frequency* 783.99, and *tail* 1 sec.)

FILTER VARIABLE uses the *acuity* parameter, and FILTER LOHI uses the *passband*/*stopband* mechanism. The latter gives more control over the range of frequencies over which the sound is faded out. For comparison, with *passband* 440 Hz and *stopband* 783 Hz, consider this Lo-Pass result: [sevenlohi-lop.wav](../sounds/sevenlohi-lop.wav). With *passband* 783 Hz and *stopband* 440 Hz, consider this Hi-Pass result: [sevenlohi-hip.wav](../sounds/sevenlohi-hip.wav). There seems to be more resonance in the FILTER VARIABLE result.

Among other FILTER functions, 'subharmonic' mode (3) of FILTER BANK produces a low resonant sound: [sevensubhdt.wav](../sounds/sevensubhdt.wav), and the 'equal intervals' mode (6) applies a series of filters to frequencies between 130Hz and 1000Hz range of frequencies at every 3 semitones [seveneqintdt.wav](../sounds/seveneqintdt.wav) (*Q* is 500 and *gain* is 10 to compensate for all the filters. This can be used as a method of tuning *via* filters. FILTER USERBANK allows the user to define a bank of frequencies at which to filter. FILTER VARIBANK does the same, but with time-varying filters. This is a very powerful way to tune a sound in a way that can for example change from one chord to another over time.

**Glissando** {#GLISSANDO} – [MODIFY SPEED / STRANGE SHIFT / FOCUS ACCU]

One of the basic ways to create glissandi is by *time-varying transposition* with MODIFY SPEED (or its Spectral Domain equivalent, REPITCH TRANSPOSE). The Spectral Domain program STRANGE SHIFT provides another way to achieve glissandi. As the name suggests, it shifts the frequencies in the analysis file, and this can be done in a time-varying way. For example, try this: create a breakpoint file that shifts frequencies up very high (e.g. to 3000 or more) at time 0 and then sets a much lower value (e.g. 200) at a later time. The result will glissando between the two values. A more intricate type of glissando effect is produced with FOCUS ACCU.

SL: `Focus->accumulate`
SSh: `Spectral->Emphasize->accumulate`

A *gliss* factor of 0.9 or thereabouts seems to produce interesting results: [countaccu.wav](../sounds/countaccu.wav).

**Loop** {#LOOP} – [EXTEND LOOP]

SL: `EXTEND->loop->loop advances to end`
SSh: `Soundfiles->REVERB/ECHO->Loop`

This process cuts out a set *length* of soundfile, moves on a *step* and cuts another, and assembles all of these cuts into one soundfile. The *length* and the *step* play off against each other in ways that affect recognisability of the source, overlap and gaps. With *length* 30ms and *gap* 5ms (and a bit of gain afterwards), [springc1gcdt.wav](../sounds/springc1gcdt.wav) becomes [springloopg.wav](../sounds/springloopg.wav) and [seven.wav](../sounds/seven.wav) becomes [sevenloopg.wav](../sounds/sevenloopg.wav). This function can give gritty multiple attacks to a sound with a strong onset transient.

**Pan** {#PAN} – [MODIFY SPACE Mode 1]

'Pan' is movement of the sound in the spatial field, usually between Left and Right in a stereo field, but in a multi-channel context, the movement can be much more complex. The basic program is MODIFY SPACE Mode 1 (pan), which takes a MONO input.

SL: `SPACE->spatialisation->pan`.
SSh: `Edit/Mix->Spatial->pan`.

The sound can be placed in a single fixed position, but mostly this program is used to create movement. For this a text breakpoint file is needed which contains *times* and *pan* positions (-1 is far Left, 0 is Center, and 1 is far Right). As with the transposition breakpoint file, interpolation takes place between different values at different times. For example, if a 10 second soundfile is panned Left to Right, the pan breakpoint file would read:

```
time pan
0.0  -1
10.0  1
```

The following pan breakpoint file (*panlr-rlr-rl.brk*) does this, moving slowly to the Right, quickly to the Left and back again to the Right, and slowly back to the Left (observe the change of pace):

```
time pan
0.0  -1
5.5   1
6.5  -1
9.0   1
16.0 -1
```

Because the source here ([bellaebtobtonem.wav](../sounds/bellaebtobtonem.wav)) was the result of a TEXTURE operation, it was a stereo file, so this TEXTURE soundfile was first changed from stereo to mono (the 'm' in the name) before using it as an infile for PAN. This is the result: [bellaebtobtonempan.wav](../sounds/bellaebtobtonempan.wav).

Note that in a SUBMIX MIX mixfile, L (left) C (Centre) and R (Right) are also valid, and if all the positions are L, a mono output soundfile will be produced. **Also note that in the TEXTURE Set, the pan range is 0 (L) to 1 (R), not -1 to 1**.

**Reverb** {#REVERB} – [MODIFY REVECHO Mode 1]

'Reverberation' provides a sonorous haze/resonance around a sound. It is usually applied late on in the sound design process to give the final result some warmth and depth. Applied too soon, it can lead to unwanted artefacts when using other processes. The basic program is MODIFY REVECHO Mode 1 (*delay* in milliseconds).

SL: `REVERB:ECHO->Rev/delay` (or `delay with feedback`)
SSh: `Soundfiles->REVERB/ECHO->Delay`.

A short delay produces a 'reverb' effect, while a longer delay produces actual repeats of the sound mixed in with the original. The following values used in this example are typical but the effect varies quite a bit with different weightings: *delay* at 15 ms, *mix* at 0.9 (closer to 1 is 'wetter' – more reverb – in this program), *feedback* at 0.9 and *tail* 3. The result is a rather metallic sound: [countrreverb1.wav](../sounds/countrreverb1.wav). Real delay is introduced by changing the *delay* parameter to e.g., 4000 (observe how the sound repeats and overlaps after 4 seconds): [countrdelay.wav](../sounds/countrdelay.wav).

There is also **Reverb** (multi-channel reverb). The main parameters are *mix* (closer to 0 is 'wetter' in this program: more reverb) and *reverbtime* (longer gives more reverberation).

SL: `REVERB:ECHO->Rev/delay` (or `delay with feedback`).
SSh: `Soundfiles->REVERB/ECHO->Delay`.

The basic idea here is to provide a delay time, set the balance between source and delayed signal (the *mix* parameter) and provide a value for *feedback*. The values here are *mix* 0.4, *reverbtime* 1.2 and *trail* 2 produce this result: [countrreverb2.wav](../sounds/countrreverb2.wav).

**Reverse** {#REVERSE} – [MODIFY RADICAL Mode 1]

SL: `RADICAL->radical->reverse`
SSh: `Soundfiles->EXTEND/SEGMENT->reverse`

This function simply plays a soundfile backward: [count.wav](../sounds/count.wav) becomes [countr.wav](../sounds/countr.wav). Language becomes obscure, fades become crescendi (a piano starts to sound like an organ – until the sudden cutoff). It is a great place to start with a sound when recognisability is not meant to be retained.

**Ring Modulation** {#RINGMOD} – [MODIFY RADICAL Mode 5]

SL: `RADICAL->radical->ring modulate`
SSh: `Soundfiles->PITCH->Ring-mod`

This procedure multiplies a soundfile by a (modulating) frequency to create 'sidebands' which are the sum and difference of the two signals, and then removes the carrier signal. The net result is a roughened sound that is somewhat hollowed out. When a 25 Hz modulating frequency is applied to [count.wav](../sounds/count.wav), the result is [countrm25.wav](../sounds/countrm25.wav), and when applied to a metal beam [girder1gdc.wav](../sounds/girder1gdc.wav), the result is [girder1gdcrm25.wav](../sounds/girder1gdcrm25.wav) It is often used to develop a sound prior to further processing.

**Time Stretch** {#TIMESTRETCH} – [STRETCH TIME]

SL: `STRETCH->time->do time_stretch`
SSh: `Spectral->TIME->Time Stretch`

The timestretch spectral process literally 'pulls' the partials of the input analysis file as if they were rubber bands, stretching out these spectral components without changing their pitch. This is a most effective way to get inside a sound and hear what is there. For example, the metal beam sound that we heard in the previous example [girder1gdc.wav](../sounds/girder1gdc.wav) becomes [girder1gdcx4c2dt.wav](../sounds/girder1gdcx4c2dt.wav) when stretched with a *timestretch* of 4. (The result of the timestretch was unnecessarily long, so the first 10 seconds or so were cut & kept, and then dovetailed to soften the onset envelope.) Listen to what happens when this sound is lowered by two octaves: [girder1gdcx4c2dtd24.wav](../sounds/girder1gdcx4c2dtd24.wav). Further time stretches and cuts to the salient part of the sound would exaggerate these effects.

There is another kind of stretching that can be done in the Spectral Domain. STRANGE WAVER oscillates between harmonic and inharmonic states.

SL: `STRANGE->waver->standard`
SSh: `Spectral->FREQ/PITCH->waver`

In this example, count.ana (analysis file!) is stretched with these parameter values: *vibration frequency* 2, *spectral stretch* 100, and *base frequency of stretch* 80. Note that a slow vibration frequency is combined with a fairly large stretch to produce (the resynthesised) [countwaver.wav](../sounds/countwaver.wav). Compare this with a much faster vibration frequency (50): [countwaver50.wav](../sounds/countwaver50.wav).

**Trace** {#TRACE} – [HILITE TRACE]

SL: `HIGHLIGHT->tracery->trace all`
SSh: `Spectral->Filter->Trace`)

This powerful function works on analysis files and retains a user-specified number of spectral components. In doing so, it pares down a sound to a ghostly image of itself. As it is keeping the loudest components, it can be surprising how few must be retained to interfere with the recognisability of the sound: below 10 starts to do the job, and with 1 or 2, just a thin burbling sound remains. It works best on complex sounds with rapidly changing amplitudes. This example uses [count.wav](../sounds/count.wav) as the input (analysed to .ana) and produces (.ana converted to .wav) [counttrace.wav](../sounds/counttrace.wav) with 1 spectral component retained. As the Reference Manual suggests, HILITE TRACE can be used to clean (i.e. remove) some unwanted background from a sound. It can also be a way to prepare a recognisable source sound for use in an abstract way.

**Transpose** {#TRANSPOSE} – [MODIFY SPEED / REPITCH TRANSPOSE]

The program MODIFY SPEED handles Time Domain transposition (higher - faster - shorter, OR lower - slower - longer).

SL: `PITCH:SPEED->pitch->tape transppose by semitones`
SSh: `Soundfiles->PITCH->Speed/Transpose`.

I usually find it easiest to use Mode 2 in which the transposition is entered as + or - semitones. CDP also enables Spectral Domain transposition *which does not alter the length of the sound*, for which an analysis file is required as input. This function is REPITCH TRANSPOSE or REPITCH TRANSPOSEF – the latter retains formants.

SL: `REPITCH->transpose->transpos in semitones` (or `REPITCH->transpose (keep formants)->transpos in semitones`
SSh: `Spectral->FREQ/PITCH->transpose` (the formants option is in the process dialogue).

*Soundshaper's* range of the transposition value for both TRANSPOSE(F) and MODIFY SPEED is +/- 96 semitones. Here is the word [seven.wav](../sounds/seven.wav) lowered by three octaves: [sevend36.wav](../sounds/sevend36.wav). *Sound Loom* initially shows a range of 24 semitones. This range can be increased to the full +/- 96 semitones by clicking on the `RANGE` button.

An Aside: – The name of the above file looks very strange, and not a few users prefer simpler names, in this case for example, 'girderstretch.wav'. I prefer a name that tells me more about how it was made. So I would read
'girder1gdcx4c2dtd24' like this:
'girder1-gd-c-x4-c2-dt-d24', i.e.,
'girder1-gaindown-cut-stretchtimes4-cut2-dovetail-down24'.
You will no doubt want to create your own conventions for filenames. I also keep a handwritten notebook to keep a record of processing work. It is very useful for looking up past successes and problems. *Soundshaper* HISTORY retains all the details about what has been done in the current session. By default, it is saved in the **\Txt** folder inside the *Soundshaper* folder. *Sound Loom* keeps a log.

**Vibrato** {#VIBTREM} – [MODIFY SPEED Mode 6 / ENVEL TREMOLO]

These programs wobble frequencies to a specified pitch depth.

MODIFY SPEED Mode 6 is first used to apply some vibrato to [count.wav](../sounds/count.wav). This producese [countvib.wav](../sounds/countvib.wav) with parameter settings: *frequency rate* 5 (times per second), and *vibrato depth* 3 (semitones).

SL: `PITCH:SPEED->tape vibrato` [parameter names: *cycles per second* and *semitone depth*]
SSh: `Soundfiles->PITCH->vibrato` [parameter names: *rate* and *width*]

Then ENVEL TREMOLO is applied to the above vibrato result, yielding: [countvibtrem.wav](../sounds/countvibtrem.wav). The parameter settings are: *tremfreq* 25 and *tremdepth* is 0.75. We hear wobbles within wobbles.

SL: `ENVELOPE->tremolo->frequency` [parameter names: *tremolo frequency* and *tremolo depth*]
SSh: `Soundfiles->ENVELOPE->tremolo` [parameter names: *tremfreq* and *tremdepth*]

You can see that the naming of these functions and their parameters is not entirely consistent. To help avoid confusion the parameter names for the functions as displayed in the two GUIs have been shown as well.

---

Last updated: 21 August 2021
