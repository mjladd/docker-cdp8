# Workshop 6: Texture Building with Texture Simple

*by Dr Archer Endrich*

The locations of the processing functions are shown under the program name.

- **SS** = the Soundshaper GUI: Soundfiles > Texture > Simple (Mode 5 'random' or Mode 3 'harmonic set' or Mode 4 'changing harmonic set')
- **SL** = the Sound Loom GUI: TEXTURE > simple (Mode 5 'neutral' or Mode 3 'over harmonic set' or Mode 4 'over harmonic sets')

## A. The Parameters of TEXTURE SIMPLE

| Parameter | Description |
|-----------|-------------|
| outdur | length of output soundfile |
| note data file | essential text file with instructions for the program |
| packing | event density |
| scatter | randomisation of event placement in time |
| timegrid | quantisation factor (usually 0) |
| sounds | 1 - 1 for one input sound, 1 - 2 for two inputs, etc. |
| gain | amplitude range: 1 - 127 (e.g., 64 - 84). Each event will have its own loudness somewhere within this range. |
| duration | length from beginning of sound to use, e.g., 1 - 1 is one second, 1 - 2: each event will have its own length somewhere within this range. When the 'Use whole sound' option (`-w` flag) is selected, duration is ignored (just use 1 - 1). |
| pitch | pitch range (1 - 127). E.g., 60 - 60 gives all events at the same pitch. 58 - 62 and each event has its own pitch somewhere within this range (will be microtonal variants unless a harmonic grid is specified). |
| position | pan location (0 = Left, 1 = Right, 0.5 = Centre) |
| spread | pan spread. E.g., 1 gives full width |
| attenuation | gain reduction multiple - make smaller (< 1) if program warns of overload. |

## B. Files Used in the Examples

**Soundfile inputs:**

- [cymcdt.wav](../../sounds/cymcdt.wav) - 3.8 sec, 44100, Mono - produces soft wash effects (used for the Worksheets CD)
- `dingcdt.wav` - 4.15 sec, 44100, Mono - alternative input: a clearly pitched tone that makes the harmonic pitching easier to hear, but the packing could be tighter if this sound is used

**Text file inputs to TEXTURE SIMPLE:**

Note data files:

`ndf60.txt`:
```
60
```

`ndfC7th.txt`:
```
60
#4
0 1 55 0 0
0 1 60 0 0
0 1 64 0 0
0 1 70 0 0
```

`ndfchng.txt`:
```
60
#12
0  1 55 0 0
1  1 60 0 0
3  1 55 0 0
4  1 64 0 0
6  1 55 0 0
7  1 70 0 0
10 1 72 0 0
11 1 70 0 0
13 1 72 0 0
14 1 64 0 0
16 1 72 0 0
17 1 60 0 0
```

Breakpoint files for the 'packing' parameter:

`pksym.brk`:
```
0    1
5    2
10   0.5
15   2
20   1
```

`pksym2.brk`:
```
0    0.5
5    1.0
10   0.25
15   1.0
20   0.5
```

## C. Key Possibilities of TEXTURE SIMPLE

Recommended infiles: `cymcdt.wav` (wash effects) or `dingcdt.wav` (clearer pitches)

### ST1. One pitch repeated regularly, with specified duration (Mode 5)

Infile: [cymcdt.wav](../../sounds/cymcdt.wav)

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|
| 20 | ndf60.txt | 2 | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 60 - 60 | 1 | 0.5 | 1 |

Outfile: `cymcdtsame1.wav`

### ST2. One pitch repeated regularly, using whole sound (Mode 5)

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndf60.txt | 2 | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 60 - 60 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdtsame2.wav`

### ST3. Tight pitch range with microtonal transpositions (Mode 5)

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndf60.txt | 2 | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 58 - 62 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdt58-62.wav`

### ST4. One pitch with time-varying packing -- note symmetric pattern (Mode 5)

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndf60.txt | pksym.brk | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 60 - 60 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdtpksymsame.wav`

### ST5. Tight pitch range with time-varying packing (Mode 5)

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndf60.txt | pksym.brk | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 58 - 62 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdtpksym58-62.wav`

### ST6. Wider pitch range with time-varying packing (Mode 5)

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndf60.txt | pksym.brk | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 55 - 67 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdtpksym55-67.wav`

### ST7. Snapping pitches to a user-defined harmonic grid (C-7th) (Mode 3 - Harmonic Set)

Pitch has to match range of harmony.

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndfC7th.txt | pksym.brk | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 55 - 70 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdtpksymC7th.wav`

### ST8. Change to rich chord by making the packing much faster (Mode 3 - Harmonic Set)

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndfC7th.txt | 0.25 | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 55 - 70 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdtC7th.wav`

### ST9. Using changing harmonic sets to create melodic outline, dense packing (Mode 4 - Changing Harmonic Set)

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd | whole-input |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|-------------|
| 20 | ndfchng.txt | 0.25 | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 55 - 72 | 1 | 0.5 | 1 | `-w` |

Outfile: `cymcdtchng.wav`

### ST10. Using changing harmonic sets to create melodic outline, fairly fast symmetric packing (Mode 4 - Changing Harmonic Set)

Duration of note events specified (and not ignored): untick 'whole input'.

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|
| 20 | ndfchng.txt | pksym2.brk | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 55 - 72 | 1 | 0.5 | 1 |

Outfile: `cymcdtchng2.wav`

### ST11. Using changing harmonic sets to create melodic outline, slower symmetric packing (Mode 4 - Changing Harmonic Set)

Duration of note events specified (and not ignored -- gaps are bigger now).

Infile: `cymcdt.wav`

| outdur | ndf | pk | scat | snds | gain | dur | pch | atten | pos | sprd |
|--------|-----|----|------|------|------|-----|-----|-------|-----|------|
| 20 | ndfchng.txt | pksym.brk | 0.01 | 1 - 1 | 94 - 104 | 1.5 - 1.5 | 55 - 72 | 1 | 0.5 | 1 |

Outfile: `cymcdtchng3.wav`

## Workshop 6 - Texture Building - Summary and Playlist

| Description | Soundfile | Process |
|-------------|-----------|---------|
| One pitch repeated regularly, with specified duration | `cymcdtsame1.wav` | TEXTURE SIMPLE - duration parameters apply |
| One pitch repeated regularly, using whole sound | `cymcdtsame2.wav` | TEXTURE SIMPLE - 'whole input' is ticked |
| Tight pitch range with microtonal transpositions | `cymcdt58-62.wav` | TEXTURE SIMPLE - pitch range is 58-62 (MPV) |
| One pitch with time-varying packing -- note the symmetric pattern | `cymcdtpksymsame.wav` | TEXTURE SIMPLE - same pitch, time-varying packing: pksym.brk |
| Tight pitch range with time-varying packing | `cymcdtpksym58-62.wav` | TEXTURE SIMPLE - pitch 58-62 and pksym.brk |
| Wider pitch range with time-varying packing | `cymcdtpksym55-67.wav` | TEXTURE SIMPLE - pitch 55-67 and pksym.brk |
| Snapping pitches to a user-defined harmonic grid (C-7th), Mode 3 | `cymcdtpksymC7th.wav` | TEXTURE SIMPLE - using ndfC7th.txt, pitch range matches harmonic grid, and with pksym.brk |
| Change to rich chord by making the packing much faster | `cymcdtC7th.wav` | TEXTURE SIMPLE - packing: note events 4 times per sec (0.25), with tiny offset |
| Using changing harmonic sets to create melodic outline, dense packing (Mode 4) | `cymcdtchng1.wav` | TEXTURE SIMPLE - using ndfchng.txt, pitch 55-72, packing = 0.25 |
| Using changing harmonic sets, fairly fast symmetric packing (Mode 4) | `cymcdtchng2.wav` | TEXTURE SIMPLE - packing: pksym2.brk (doubles density of pksym.brk) |
| Using changing harmonic sets, slower symmetric packing (Mode 4) | `cymcdtchng3.wav` | TEXTURE SIMPLE - back to pksym.brk |
