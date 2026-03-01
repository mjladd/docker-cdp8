# Play List for CDP Sound Transformation Worksheets 6 through 9

*by Dr Archer Endrich*

(Revised June 2005)

---

## Workshop 6 - Texture Building with Texture Simple

| Track | Soundfile | Description |
|-------|-----------|-------------|
| | **Input soundfiles** | |
| 1 | `cymcdt.wav` | Input soundfile for this workshop (4.78 sec. mono, suspended cymbal) |
| 2 | `dingcdt.wav` | Alternative input (5.1 sec. mono, metal beater on saucepan lid) |
| | **One pitch repeated regularly, with specified duration** | |
| 3 | `cymcdtsame1.wav` | TEXTURE SIMPLE - duration parameters apply |
| | **One pitch repeated regularly, using whole sound** | |
| 4 | `cymcdtsame2.wav` | TEXTURE SIMPLE - 'whole input' is ticked |
| | **Tight pitch range with microtonal transpositions** | |
| 5 | `cymcdt58-62.wav` | TEXTURE SIMPLE - pitch range is 58-62 (MPV) |
| | **One pitch with time-varying packing -- note the symmetric pattern** | |
| 6 | `cymcdtpksymsame.wav` | TEXTURE SIMPLE - same pitch, time-varying packing: pksym.brk |
| | **Tight pitch range with time-varying packing** | |
| 7 | `cymcdtpksym58-62.wav` | TEXTURE SIMPLE - pitch 58-62 and pksym.brk |
| | **Wider pitch range with time-varying packing** | |
| 8 | `cymcdtpksym55-67.wav` | TEXTURE SIMPLE - pitch 55-67 and pksym.brk |
| | **Snapping pitches to a user-defined harmonic grid (C-7th) - Mode 3 Harmonic Set** | |
| 9 | `cymcdtpksymC7th.wav` | TEXTURE SIMPLE - using ndfC7th.txt, pitch range matches harmonic grid, and with pksym.brk |
| | **Change to rich chord by making the packing much faster** | |
| 10 | `cymcdtC7th.wav` | TEXTURE SIMPLE - packing: note events 4 times per sec (0.25), with tiny offset |
| | **Using changing harmonic sets to create melodic outline, dense packing (Mode 4)** | |
| 11 | `cymcdtchng1.wav` | TEXTURE SIMPLE - using ndfchng.txt, pitch 55-72, packing = 0.25 |
| | **Using changing harmonic sets, fairly fast symmetric packing (Mode 4)** | |
| 12 | `cymcdtchng2.wav` | TEXTURE SIMPLE - packing: pksym2.brk (doubles density of pksym.brk) |
| | **Using changing harmonic sets, slower symmetric packing (Mode 4)** | |
| 13 | `cymcdtchng3.wav` | TEXTURE SIMPLE - back to pksym.brk |
| 14 | | 6 sec. silence |

---

## Workshop 7 - Envelope Transfers

| Track | Soundfile | Description |
|-------|-----------|-------------|
| | **Input soundfiles** | |
| 15 | `trcdt.wav` | Idling tractor (10.98 sec. mono) |
| 16 | `frogs3cdt.wav` | Frog chirrups (13.58 sec. mono) |
| | **Extract & Replace Time Domain Amplitude Envelope** | |
| 17 | `trcdtfrogenv.wav` | ENVEL REPLACE - take envelope of second input and impose it on the first input |
| 18 | `f-tfstag0.wav` | SUBMIX MERGE (Mix Two) - confirm synchronous envelope shapes |
| 19 | `f-tfstag&1.wav` | SUBMIX MERGE (Mix Two) - second file starts 0.1 sec later to create a 'shadow' effect |
| | **Extract & Replace Spectral Domain Spectral Envelope (= Vocoding)** | |
| 20 | `t-fvocode.wav` | FORMANTS VOCODE - Mode 2 (pitch) with 12 bands and Mode 1 (frequency) with 4 channels give similar results: we hear the timbre of the frog voice IN the tractor sound. |
| 21 | `f-tvocode.wav` | FORMANTS VOCODE - Mode 1 (frequency) with 4 channels: we hear the chirping amplitude envelope shapes of the frogs with the sound of the tractor. |
| 22 | | 6 sec. silence |

---

## Workshop 8 - Transitions / Morphing

| Track | Soundfile | Description |
|-------|-----------|-------------|
| | **Input soundfiles** | |
| 23 | `femwheeze.wav` | Lowered female vocal sound (1.55 sec.) |
| 24 | `gongvib.wav` | Gong sound, with forwards-backwards splicing and time-varying vibrato (12.598 sec) |
| 25 | `femwhx2-7r.wav` | femwheeze.wav time-stretched and reversed (5.2 sec) |
| 26 | `oingseq.wav` | Tube toy that makes a vocal-like sound spliced forwards & backwards (5.2 sec) |
| | **Time Domain Crossfade** | |
| 27 | `fem-xfskew-oing.wav` | SUBMIX CROSSFADE - 1st fades out while 2nd fades in & use of stagger=0.5, and skew=1.5 to slow the entry of the 2nd input |
| 28 | `trcdt-xf-femwheeze.wav` | SUBMIX CROSSFADE - longer first sound |
| | **Spectral Domain Cross** | |
| 29 | `fem-ccross-oing.wav` | COMBINE CROSS - spectral amplitude replacement with balanced weighting (replace=0.25) |
| 30 | `trcdt-tvccross-frogs.wav` | COMBINE CROSS - differing inputs: tractor & frogs (time-varying replace) |
| | **Spectral Domain Morph** | |
| 31 | `fem-m-oing.wav` | MORPH MORPH - transition by spectral interpolation (nearly full length used) |
| 32 | `trcdt-m-femwheeze.wav` | MORPH MORPH - long stagger employed - compare with trcdt-xf-femwheeze.wav |
| 33 | `gongvoccount.wav` | FORMANTS VOCODE - Stage 1: gongvib.wav vocoded with count.wav |
| 34 | `gongvoccount-m-count.wav` | MORPH MORPH - Stage 2: morph vocoded sound with unaltered count.wav |
| 35 | `gong-m-gongvoccount-m-count.wav` | MORPH MORPH - Stage 3: unaltered gongvib.wav morphed to vocoded gong & count, which is morphed to unaltered count.wav |
| | **Spectral Domain Glide** | |
| 36 | `tglidef.wav` | MORPH GLIDE - long glide between spectra from two single analysis windows |
| 37 | | 6 sec. silence |

---

## Workshop 9 - Speech / Narrative

| Track | Soundfile | Description |
|-------|-----------|-------------|
| | **Input soundfiles** | |
| 38 | `count.wav` | Female person counting from 1 to 10 (8.066 sec) |
| 39 | `trcdt.wav` | Idling tractor (10.98 sec.) |
| 40 | `donkey1g44.wav` | Donkey brays (11.0 sec.) |
| | **Change of tone** | |
| 41 | `countu3.wav` | lighter - MODIFY SPEED - transposed up 3 semitones |
| 42 | `countd3.wav` | heavier - MODIFY SPEED - transposed down 3 semitones |
| 43 | `countlop.wav` | muffled - FILTER LOW PASS - Stopband 1000, Passband 500 (nothing above 1000 Hz) |
| 44 | `counthip.wav` | thin/tinny - FILTER HIGH PASS - Passband 3050 Stopband 3000 (nothing below 3000 Hz) |
| | **Reverb** | |
| 45 | `countrvb1.wav` | small - REVERB (Dobson) - Reverb time is 1 (sec) |
| 46 | `countrvb2.wav` | medium - REVERB (Dobson) - Reverb time is 2 (sec) |
| 47 | `countrvb3.wav` | large - REVERB (Dobson) - Reverb time is 3 (sec) |
| 48 | `countste1.wav` | huge - STADIUM ECHO (Wishart) - gain = 0.646, roll-off = 1, size = 1, count = 23 |
| 49 | `countste2.wav` | vast - STADIUM ECHO - size is 2, count is 15 |
| | **Harmonise** | |
| 50 | `countfit4q100.wav` | intervals - FILTER BANK - Mode 6, Q = 100, Gain = 3, Interval = 4 |
| 51 | `counttxAmin.wav` | slow - TEXTURE SIMPLE - ndf60Amin.txt (A minor: A-MiddleC-E), packing = 5 `-w` |
| 52 | `counttuneAmin.wav` | chord - PITCH TUNE - MIDI Pitch Values 57 60 64 = A-minor triad: Amin.tun |
| | **Pseudo-conversations** | |
| 53 | `counttx58-62pk2.wav` | overlap - TEXTURE SIMPLE - ndf60.txt, packing = 2, pch = 58-62 `-w` |
| 54 | `counttx58-62pk1.wav` | faster - TEXTURE SIMPLE - ndf60.txt, packing = 1, pch = 58-62 `-w` |
| | **Roughen the voice** | |
| 55 | `countblur20.wav` | blur - BLUR BLUR - 20 windows keeps the text clear, adds a tinny sheen |
| 56 | `countblur70.wav` | blur more - BLUR BLUR - 70 windows, speech slurred and more hollow resonance |
| 57 | `countpw&2.wav` | uncertain - DISTORT PITCHWARP - octvary = 0.2 |
| 58 | `countdrpl2.wav` | quavery - DISTORT REPLACE - cycles = 2 |
| 59 | `countdinterp.wav` | sinister - DISTORT INTERPOLATE - cycles = 2 |
| 60 | `countdavg.wav` | scared - DISTORT AVERAGE - cycles = 2, wavelength = 0.9 |
| 61 | `countdmult.wav` | younger - DISTORT MULTIPLY - cycles = 2 |
| 62 | `countdenv.wav` | fuller - DISTORT ENVELOPE - Attack envelope = 2 cycles |
| 63 | `countdrpt2-2.wav` | tough - DISTORT REPEAT - 2 repeats of 2 cycles |
| | **Stranger modulations** | |
| 64 | `countdrev50.wav` | broken - DISTORT REVERSE - cycles = 50 |
| 65 | `countdrev500.wav` | unfamiliar - DISTORT REVERSE - cycles = 500 |
| 66 | `countchorus.wav` | multiple - BLUR CHORUS - Mode 5: amp spread = 100, freq spread = 1.2 |
| 67 | `counttuneAminbl70.wav` | synthetic - BLUR BLUR - 70 window blur applied to counttuneAmin |
| | **Fragmentation** | |
| 68 | `countscram1.wav` | mild - EXTEND SCRAMBLE Mode 1, minseg=0.5, maxseg=1 |
| 69 | `countscram2.wav` | displaced - EXTEND SCRAMBLE Mode 1, minseg=0.1, maxseg=0.5 |
| 70 | `countscram3.wav` | confused - EXTEND SCRAMBLE Mode 1, minseg=0.06, maxseg=0.1 |
| 71 | `countscramscat2.wav` | broken - EXTEND SCRAMBLE Mode 2, minseg=0.1, scatter=5 |
| 72 | `countshred1.wav` | burble - EXTEND SHRED - repeats=10, chunks=0.1, scatter=4 |
| 73 | `countshred2.wav` | water - EXTEND SHRED - repeats=100, chunks=0.1, scatter=8 |
| | **Vibrato effects** | |
| 74 | `countrm3.wav` | wobble - MODIFY RADICAL Mode 5 - mod-freq=3 Hz |
| 75 | `countrm10.wav` | flutter - MODIFY RADICAL Mode 5 - mod-freq=10 Hz |
| 76 | `countrm500.wav` | synthetic - MODIFY RADICAL Mode 5 - mod-freq=500 Hz |
| 77 | `countvib5-1.wav` | smooth - MODIFY SPEED Mode 6 - rate=5, width=1 |
| 78 | `countvib10-1.wav` | faster - MODIFY SPEED Mode 6 - rate=10, width=1 |
| 79 | `countvib10-3.wav` | deep - MODIFY SPEED Mode 6 - rate=10, width=3 |
| 80 | `countvib25-4.wav` | alien - MODIFY SPEED Mode 6 - rate=25, width=4 |
| 81 | `countspvib5-5.wav` | hollow - STRANGE WAVER - Vib freq=5, stretch=5 |
| 82 | `countspvib25-10.wav` | robotic - STRANGE WAVER - Vib freq=25, stretch=10 |
| | **Speech-like up to a point** | |
| 83 | `count-voc-tractor.wav` | grungy - FORMANTS VOCODE - count 1st and trcdt 2nd |
| 84 | `tractor-voc-count.wav` | stressed - FORMANTS VOCODE - trcdt 1st and count 2nd |
| 85 | `count-xm-donkeynrvb5.wav` | modulated - MODIFY RADICAL Mode 6 - count.wav and donkey1g44.wav, normalised with 5 sec. reverb |
| 86 | `countR.wav` | reverse - MODIFY RADICAL Mode 1 |
| 87 | `countgrainmill.wav` | swarm - GRAINMILL - timestretch=2, density 16-64, grainsize 20.56ms, pitch range 6 semitones. ~27,555 granulations turn the single voice into a swarm of voices. |
