# Workshop 9: On Speech Material (Voiceover / Narration)

*by Dr Archer Endrich*

The locations of the processing functions are shown under the program name.

- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

For the most part, the emphasis will be on modest (but many different) alterations made to the voice, to adapt the tone of voice for different purposes, while keeping the text clearly audible. A few exceptions to this show us some other vistas.

**Sources:** [count.wav](../../sounds/count.wav) (an emotionally neutral count from 1 to 10), [trcdt.wav](../../sounds/trcdt.wav) and [donkey1g44.wav](../../sounds/CSRCWS09SF03donkey1g44.wav). Also convert the first two (**SS: Spectral > Convert > Analyse** / **SL: PVOC > analysis**) to form `count.ana` & `trcdt.ana`. You will also need the note data files: `ndf60.txt` and `ndf60Amin.txt`.

## Change of Tone

ST4. **lighter** -- `MODIFY SPEED` (**SS: Soundfiles > Pitch > Transpose/Speed (semitones)** / **SL: PITCH: SPEED > transpose > tape transpose in semitones**) - up 3 semitones (enter 3). Outfile: `countu3.wav` (more than this tends to distort the voice)

ST5. **heavier** -- `MODIFY SPEED` (**SS: Soundfiles > Pitch > Transpose/Speed (semitones)** / **SL: PITCH: SPEED > transpose > tape transpose in semitones**) - down 3 semitones (enter -3). Outfile: `countd3.wav` (more than this tends to distort the voice)

ST6. **muffled** -- `FILTER LOW PASS` (**SS: Soundfiles > Filter > Low Pass** / **SL: FILTER > lopass:hipass > bands as frq**) - Stopband 1000, Passband 500, Atten 0. Outfile: `countlop.wav` (nothing above 1000 Hz)

ST7. **thin/tinny** -- `FILTER HIGH PASS` (**SS: Soundfiles > Filter > High Pass** / **SL: FILTER > lopass:hipass > bands as frq**) - Passband 3050, Stopband 3000, Atten 0. Outfile: `counthip.wav` (nothing below 3000 Hz)

## Reverb

ST8. **small** -- `REVERB` (**SS: Soundfiles > Reverb/Echo > Reverb** / **SL: REVERB:ECHO > reverb with room characteristics**) - set Reverb time to 1 (sec) and leave other values with existing defaults. Note that the additional files remain optional as long as they are not ticked. Outfile: `countrvb1.wav`

ST9. **medium** -- `REVERB` (**SS: Soundfiles > Reverb/Echo > Reverb** / **SL: REVERB:ECHO > reverb with room characteristics**) - set Reverb time to 2 (sec) and leave other values with existing defaults. Outfile: `countrvb2.wav`

ST10. **large** -- `REVERB` (**SS: Soundfiles > Reverb/Echo > Reverb** / **SL: REVERB:ECHO > reverb with room characteristics**) - set Reverb time to 3 (sec) and leave other values with existing defaults. Outfile: `countrvb3.wav`

ST11. **huge** -- `STADIUM ECHO` (**SS: Soundfiles > Reverb/Echo > Stadium echo** / **SL: REVERB: ECHO > rev/echo > stadium**) - leave all defaults in place (gain = 0.646, roll-off = 1, size = 1, count = 23). Outfile: `countste1.wav`

ST12. **vast** -- `STADIUM ECHO` (**SS: Soundfiles > Reverb/Echo > Stadium echo** / **SL: REVERB: ECHO > rev/echo > stadium**) - just change size to 2 (multiplies the time between echoes 0.1 x 2 to become 0.2 second) and count to 15. Outfile: `countste2.wav`

## Harmonise

ST13. **intervals** -- `FILTER BANK` (**SS: Soundfiles > Filter > Bank > equal intervals** / **SL: FILTER > bank > fixed interval between**) - Mode 6 ('Equal intervals 2') - Q = 100 (to let recognisable amount of the text through), Gain = 3 (to compensate for the filtering), Interval = 4 (semitones), and tick 'double filtering' (to bring out the harmony more). Outfile: `countfit4q100df.wav`

ST14. **slow** -- `TEXTURE SIMPLE` (Mode 3 - harmonic set) with `ndf60Amin.txt` (the note data file, which you need to 'Open' or type in):

```
60
#3
0 1 57 0 0    (this is A below Middle-C)
0 1 60 0 0    (this is Middle-C)
0 1 64 0 0    (this is E above Middle-C)
```

Outdur = 40 sec., packing = 5, scatter = 0.5, gain 64-84, pch 57-64, Atten 0.75, Pos 0.5, Spread 0.5 and tick 'whole input'. 3 different-sounding voices overlap. Outfile: `counttxAmin.wav`

ST15. **chord** -- `PITCH TUNE` (**SS: Spectral > Pitch > Tune** / **SL: PITCH: HARMONY > tune spectrum > tunings as midi**) - input is `count.ana`. Enter `57 60 64` into the `.tun` file and save as `Amin.tun`. Keep existing defaults: Focus = 1, Clarity = 1 and Trace = 1. Outfile: `counttuneAmin.ana` and Convert to `counttuneAmin.wav`

## Pseudo-conversations

ST16. **overlap** -- `TEXTURE SIMPLE` (Mode 5 - random / neutral) - as ST14, except with `ndf60.txt` (just the number '60' in it), packing = 2 and pch is 58-62. Also tick 'whole input'. Voices overlap more, are closer together, and the harmonic element is removed. Outfile: `counttx58-62pk2.wav`

ST17. **faster** -- `TEXTURE SIMPLE` (Mode 5 - random / neutral) - as ST16, with `ndf60.txt` (just the number '60' in it), packing = 1 (and pch is still 58-62). Tick 'whole input'. Voices overlap a lot and words of the text are heard to repeat. A dense, fraught conversation could be made by taking this process further, especially if the text is randomly rearranged first (see below, SCRAMBLE example), or different texts were used (multiple infiles -- then Snds ('sounds') is given a range: 1 for the lower value, and how many input soundfiles there for the second value, e.g, 1 and 3 for 3 infiles). Outfile: `counttx58-62pk1.wav`

## Roughen the Voice

ST18. **blur** -- `BLUR BLUR` (**SS: Spectral > Time > Blur** / **SL: BLUR > blur**) - 20 windows keeps the text clear, but adds a tinny sheen to it. Input is `count.ana`. Outfile: `countblur20.ana` and Convert to `countblur20.wav`.

ST19. **blur more** -- `BLUR BLUR` (**SS: Spectral > Time > Blur** / **SL: BLUR > blur**) - 70 windows slurs the speech a little and makes the tinny sheen more prominent. Input is `count.ana`. Outfile: `countblur70.ana` and Convert to `countblur70.wav`.

(Very minimal values in the next DISTORT functions, to keep the text recognisable.)

ST20. **uncertain** -- `DISTORT PITCHWARP` (**SS: Soundfiles > Distort > Pitch** / **SL: DISTORT > pitch**) - octvary = 0.2 puts a slight quaver into the voice that makes it sound a little uncertain. Outfile: `countpw&2.wav`

ST21. **quavery** -- `DISTORT REPLACE` (**SS: Soundfiles > Distort > Replace** / **SL: DISTORT > replace**) - cycles = 2 (and skip = 0, the default). Now the voice sounds really old and quavery. Outfile: `countdrpl2.wav`

ST22. **sinister** -- `DISTORT INTERPOLATE` (**SS: Soundfiles > Distort > Interpolate** / **SL: DISTORT > interpolate**) - cycles = 2 (and skip = 0, the default). Elongates the voice with a rough tone. Outfile: `countdinterp.wav`

ST23. **scared** -- `DISTORT AVERAGE` (**SS: Soundfiles > Distort > Average** / **SL: DISTORT > average**) - cycles = 2 and wavelength = 0.9. The voice goes high and thin, with intermittent rasps. Outfile: `countdavrg.wav`

ST24. **younger** -- `DISTORT MULTIPLY` (**SS: Soundfiles > Distort > Multiply** / **SL: DISTORT > multiply**) - cycles = 2. The voice becomes higher / younger, but the distortion makes it sound perhaps a bit cheeky? Outfile: `countdmult.wav`

ST25. **fuller** -- `DISTORT ENVELOPE` (**SS: Soundfiles > Distort > Envelope** / **SL: DISTORT > envelope**) - various options are possible. Here we use Mode 1 ('rising') with the Attack envelope set to 2 cycles. Very modest change to the voice. Outfile: `countdenv.wav`

ST26. **tough** -- `DISTORT REPEAT` (**SS: Soundfiles > Distort > Repeat** / **SL: DISTORT > repeat**) - even the default values of 2 repeats of 2 cycles makes the voice much deeper and rougher, a tough hombre. Outfile: `countdrpt2-2.wav`

## Stranger Modulations

ST27. **broken** -- `DISTORT REVERSE` (**SS: Soundfiles > Distort > Reverse** / **SL: DISTORT > reverse**) - cycles = 50. Fragments of the voice get turned around backwards. Outfile: `countdrev50.wav`

ST28. **unfamiliar** -- `DISTORT REVERSE` (**SS: Soundfiles > Distort > Reverse** / **SL: DISTORT > reverse**) - cycles = 500. Fragments of the voice get turned around backwards. Sounds like a different language! Outfile: `countdrev500.wav`

ST29. **multiple** -- `BLUR CHORUS` (**SS: Spectral > Freq/Pitch > Chorus** / **SL: BLUR > chorus > scatter amps & frqs**) - input is `count.ana` - Mode 5 ('Amps + Freqs'): amp spread = 100, freq spread = 1.2 (both are low values). We hear a flurry of voices. Outfile: `countchorus.wav`

ST30. **synthetic** -- `BLUR BLUR` (**SS: Spectral > Time > Blur** / **SL: BLUR > blur**) - input is `counttuneAmin.ana` (BLUR applied to the output of the PITCH TUNE process) - windows = 70. This produces a rather synthetic sounding voice. Outfile: `counttuneAminbl70.ana` Convert to `counttuneAminbl70.wav`.

## Fragmentation

ST31. **mild** -- `EXTEND SCRAMBLE` Mode 1 (**SS: Soundfiles > Extend > Scramble > Random chunks** / **SL: EXTEND > scramble > completely random**) - minseg=0.5, maxseg=1, outdur=10 and Splice left at the default (25ms). The long segments give a very mild rearrangement. Outfile: `countscram1.wav`

ST32. **displaced** -- `EXTEND SCRAMBLE` Mode 1 (**SS: Soundfiles > Extend > Scramble > Random chunks** / **SL: EXTEND > scramble > completely random**) - minseg=0.1, maxseg=0.5, outdur=10 and Splice left at the default (25ms). The shorter segments break it apart much more. Outfile: `countscram2.wav`

ST33. **confused** -- `EXTEND SCRAMBLE` Mode 1 (**SS: Soundfiles > Extend > Scramble > Random chunks** / **SL: EXTEND > scramble > completely random**) - minseg=0.06, maxseg=0.1, outdur=10 and Splice left at the default (25ms). The tiny segments confuse the speech entirely. Outfile: `countscram3.wav`

ST34. **broken** -- `EXTEND SCRAMBLE` Mode 2 (**SS: Soundfiles > Extend > Scramble > segment soundfile** / **SL: EXTEND > scramble > scramble src: then again**) - minseg=0.1, scatter=5, outdur=10 and Splice left at the default (25ms). The scatter function breaks it apart even more, introducing some pauses as well. Outfile: `countscramscat2.wav`

ST35. **burble** -- `EXTEND SHRED` (**SS: Soundfiles > Extend > Shred** / **SL: RADICAL > radical > shred**) - repeats=10, chunks=0.1, scatter=4. The text becomes a burble of broken syllables. Outfile: `countshred1.wav`

ST36. **water** -- `EXTEND SHRED` (**SS: Soundfiles > Extend > Shred** / **SL: RADICAL > radical > shred**) - repeats=100, chunks=0.1, scatter=8. The text becomes a watery flow. Outfile: `countshred2.wav`

## Vibrato Effects

ST37. **wobble** -- `MODIFY RADICAL` Mode 5 (**SS: Soundfiles > Radical > Ring-mod** / **SL: RADICAL > radical > ring modulate**) - mod-freq=3 (Hz). These tight sidebands just create a slight wobble in the voice. Outfile: `countrm3.wav`

ST38. **flutter** -- `MODIFY RADICAL` Mode 5 (**SS: Soundfiles > Radical > Ring-mod** / **SL: RADICAL > radical > ring modulate**) - mod-freq=10 (Hz). The wider sidebands create enough beat-pattern to put a flutter into the voice. Outfile: `countrm10.wav`

ST39. **synthetic** -- `MODIFY RADICAL` Mode 5 (**SS: Soundfiles > Radical > Ring-mod** / **SL: RADICAL > radical > ring modulate**) - mod-freq=500 (Hz). This wide spacing makes the voice hollow and synthetic (It can go much wider!). Outfile: `countrm500.wav`

ST40. **smooth** -- `MODIFY SPEED` Mode 6 (**SS: Soundfiles > Pitch > Vibrato** / **SL: PITCH:SPEED > pitch > tape vibrato**) - rate=5, width=1. Fairly slow and tight, creating a smooth undulation. Outfile: `countvib5-1.wav`

ST41. **faster** -- `MODIFY SPEED` Mode 6 (**SS: Soundfiles > Pitch > Vibrato** / **SL: PITCH:SPEED > pitch > tape vibrato**) - rate=10, width=1. The vibrato is faster. Outfile: `countvib10-1.wav`

ST42. **deep** -- `MODIFY SPEED` Mode 6 (**SS: Soundfiles > Pitch > Vibrato** / **SL: PITCH:SPEED > pitch > tape vibrato**) - rate=10, width=3. The vibrato's pitch change is wider. Outfile: `countvib10-3.wav`

ST43. **alien** -- `MODIFY SPEED` Mode 6 (**SS: Soundfiles > Pitch > Vibrato** / **SL: PITCH:SPEED > pitch > tape vibrato**) - rate=25, width=4. The fast and wide fluctuations give the voice a non-human quality. Outfile: `countvib25-4.wav`

ST44. **hollow** -- `STRANGE WAVER` (**SS: Spectral > Pitch > Waver (vib.)** / **SL: **) - input is `count.ana`. Vib freq=5, stretch=5. Note that values for vib freq less than zero turn it into a very slow pitch-change. In this case, the voice is hollow with an artificial resonance. Outfile: `countspvib5-5.wav`

ST45. **robotic** -- `STRANGE WAVER` (**SS: Spectral > Pitch > Waver (vib.)** / **SL: **) - input is `count.ana`. Vib freq=25, stretch=10. The higher values give the voice a very synthetic, robotic quality. Outfile: `countspvib25-10.wav`

## Speech-like Up to a Point

ST46. **grungy** -- `FORMANTS VOCODE` (**SS: Spectral > Morph/Formants > Vocode** / **SL: FORMANTS > vocode**) - inputs are `count.ana` and `trcdt.ana` in that order. Mode 2 ('Formants by pitch') with bands = 4. The tractor sound is put into the voice, making it extremely gruff and grungy. Outfile: `count-voc-tractor.ana` Convert to `count-voc-tractor.wav`

ST47. **stressed** -- `FORMANTS VOCODE` (**SS: Spectral > Morph/Formants > Vocode** / **SL: FORMANTS > vocode**) - inputs are `trcdt.ana` and `count.ana` in that order. Mode 2 ('Formants by pitch') with bands = 4. The vocal sound is put into the tractor, making it sound rough and stressed. Outfile: `tractor-voc-count.ana` Convert to `tractor-voc-count.wav`

ST48. **modulated** -- `MODIFY RADICAL` Mode 6 ('Cross-modulate') (**SS: Soundfiles > Radical > Cross-mod** / **SL: RADICAL > radical > cross modulate**) - Here is something different again. There are no parameters, just two infiles: `count.wav` and `donkey1g44.wav`. The two sounds are multiplied together, somehow putting them together. Only some sounds respond well to this process. Outfile: `count-xm-donkey.wav` Now NORMALISE it (**Edit/Mix > Loudness > Normalise**), adding an 'n' to the name, and then add REVERB (**Soundfiles > Reverb/Echo > Reverb**), giving it a Reverb time of 5 seconds. Final outfile: `count-xm-donkeynrvb5.wav`

ST49. **reverse** -- `MODIFY RADICAL` Mode 1 ('Reverse') (**SS: Soundfiles > Radical > Reverse SF** / **SL: RADICAL > radical > reverse**) - just give the outfile name and the sound will be turned round back to front: it reads from the last sample back to the first sample. Voices do unexpected things when heard backwards. Outfile: `countR.wav`

**AT THIS POINT SAVE YOUR HISTORY AND EXIT FROM SOUNDSHAPER**

ST50. **swarm** -- `GRAINMILL` - open GrainMill and Set Working Directory to the directory where you have been working. Open `count.wav` and a dialogue box will appear. Enter these parameter values:

- Timestretch = 2
- Density - tick range and H to get a Higher high value - enter 64, and M to get a medium low value - enter 16
- Grainsize = 20.56 (with L ticked) -- this value is in milliseconds
- Pitch shift - tick range and enter 3 as the upper value and -3 for the lower value (semitones)
- Loudness - tick range and enter 1 as the upper value and -30 for the lower value (dB)
- Space - tick range and enter 1 (Right) as the upper value and 0 (Left) for the lower value
- Wander - leave at 0.5

Now click on MAKE and you will see 27,555 grains (or so) appear on the screen, with different colours for different degrees of loudness. PLAY through it when finished, listening to a swarm of voices speaking the text.

Go to File and SAVE AS `countgrainmill.wav`
