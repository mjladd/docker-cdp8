**Play List for CDP Sound Transformation Worksheets- 1 through 5**

(Prepared by A Endrich, March 2005)

**Workshop 1 - WAYS TO ALTER PITCH LEVELS - SUMMARY & PLAYLIST**

**Track - Soundfile Description**

1. capm.wav Input soundfile for most operations below (7.2 sec. mono,
male voice)

**Transpose**

2. capmu12.wav MODIFY SPEED - sound is raised 12 semitones (1 octave)

3. capmd12.wav MODIFY SPEED - sound is lowered 12 semitones (1 octave)

4. capmtvtru.wav MODIFY SPEED - sound slides upwards over time for 12
semitones

according to breakpoint file **tvtru.brk**

5. capmtvtrd.wav MODIFY SPEED - sound slides downwards over time for 12
semitones

according to breakpoint file **tvtrd.brk**

6. capmtrchng.wav MODIFY SPEED - sound moves both up & down according
to contents

of the breakpoint file **trchng.brk** sometimes sliding & sometimes
instantaneous

changes

**Distortion and pitch warping**

7. capmtrpw.wav DISTORT PITCHWARP - wavecycle distortion with
time-varying sliding

transposition across specified octaves or parts of octaves, using
**trpw.brk**

8. capmpw&33.wav DISTORT PITCHWARP - wavecycle distortion with a single
fraction of an

octave specified for the pitchwarp (the '&' in the name = a decimal
point,

so the value used here is 0.33 of an octave)

**Cycles of pitch transposition**

9. capmgrn&9.wav MODIFY BRASSAGE, Mode 5 - make a soundfile a little
bit grainy (0.9)

10. capmgrn&9m1.wav GRAIN PITCH - previous sound is input for series of
pitch transpositions that

cycle round, according to the text data file **grntr.txt**

11. capmgrn&9m2.wav The previous operation is done again, but using
Mode 2 so that the whole

cycle of transpositions is applied to each grain before moving on

**Transposition with internal split point**

12. capmtranspm4.wav REPITCH TRANSP - Spectral transposition upwards
with

frequency split point

**Ring modulation**

13. capmtranspm4rmg.wav MODIFY RADICAL, Mode 5 (ring modulation) is
applied to the previous

output and Gain is applied

**Multiple Echoes**

14. capmtranspm4rmgste.wav MODIFY REVECHO, Mode 3 (Stadium Echo) - the
previous output is given

multiple echoes, as if reflected off the walls of a large stadium

**Frequency shift**

15. capmtvssm1.wav STRANGE SHIFT - time-varying shift that squeezes the
partials closer together

as one goes higher, using **ss.brk**

**Internal glissandi**

16. capmaccu&01.wav FOCUS ACCUMULATE - spectral process that can create
internal glissandi. The

soundfile name indicates that 0.01 was used for the decay &/or gliss
parameters

17. 6 sec. silence

**Workshop 2 -WAYS TO FILTER SOUNDS - SUMMARY & PLAYLIST**

18. trcdt.wav Main input soundfile for these processes (10 sec. mono, a
tractor):

**Low-pass & High-pass**

19. trcdtlo.wav FILTER LOHI - low pass filter (cuts off the higher
frequencies: nothing above 400)

20. trcdthi1.wav FILTER LOHI - high pass filter (cuts off the lower
frequencies: nothing below 796)

21. trcdthi2.wav FILTER LOHI - high pass with a much higher stop
frequency: nothing below 4796)

**Band-pass & Band-reject (Notch)**

22. trcdtbp1.wav FILTER VARIABLE - in band pass mode to retain a
specified band of frequencies: 796

23. trcdtbp2.wav FILTER VARIABLE - band of frequencies retained is
considerably higher: 1796

24. trcdtbp3.wav FILTER VARIABLE - band of frequencies retained is much
higher: 4796

25. trcdtbr1.wav FILTER VARIABLE - in band reject mode to hollow out a
specified band of frequencies: 296

26. trcdtbr2.wav FILTER VARIABLE - band of frequencies rejected is
considerably higher: 796

27. trcdtbr3.wav FILTER VARIABLE - band of frequencies rejected is much
higher: 1796

**Various preset filter banks**

28. trcdtfbm1.wav FILTER BANK - filters follow the harmonic overtone
series

29. trcdtfbm2.wav FILTER BANK - filters follow alternate harmonics of
the harmonic overtone series

30. trcdtfbm3.wav FILTER BANK - filters follow the subharmonic series

31. trcdtfbm4.wav FILTER BANK - filters follow the harmonic overtone
series, with linear offset

32. trcdtfbm5a.wav FILTER BANK - filter according to a pattern of
equally spaced intervals - high

33. trcdtfbm5b.wav FILTER BANK - filter according to a pattern of
equally spaced intervals - lower

34. trcdtfbm6a.wav FILTER BANK - filter according to a pattern of
equally spaced minor thirds (3 semitones)

35. trcdtfbm6b.wav FILTER BANK - filter according to a pattern of
equally spaced octaves (12 semitones)

36. trcdtfbm6c.wav FILTER BANK - filter according to a pattern of
equally spaced perfect 5ths (7 semitones)

37. trcdtfbm6d.wav FILTER BANK - filter according to a pattern of
equally spaced augmented 4ths (6 semitones)

38. capm.wav New input soundfile (also on Track 1)

**Phasing filter**

39. capmphasm2-35.wav FILTER PHASING, Mode 2 - 35ms delay for a
reverberant, enclosed space

40. capmphasm2tv.wav FILTER PHASING, Mode 2 - **phasdlay.brk** for a
time-varying reverberation pattern

(big space moving to a more enclosed space)

**Cumulative filtering**

41. capmblow.wav New input cut from last part of capm.wav

42. capmblowit1.wav FILTER ITERATED - **eqint6.txt** (produced by
BANKFRQS) for cumulative filtering

of an equal interval pattern (minor thirds are specified)

43. capmblowit6ps.wav FILTER ITERATED - with a randomised pitch shift
(Q = 75 and delay time is 0.25 sec.)

**Sweeping filter**

44. trcdtswm3.wav FILTER SWEEPING - Mode 3 to sweep filter within a
specified band of frequencies

(200-220)

45. trcdtswm2.wav FILTER SWEEPING - Mode 2 to sweep lo-pass (bottom
part of sound -- 200 up to 1000)

46. capmswm2.wav FILTER SWEEPING - Mode 2: the same operation adjusted
and tried out on the vocal

sound

47. 6 sec. silence

**Workshop 3 - LENGTHEN/ROUGHEN & LENGTHEN - SUMMARY & PLAYLIST**

48. flex.wav ****The input soundfile for all of the following, unless
otherwise specified -- also on Track 17)

**Distortion**

49. flexdr2-2.wav DISTORT REPEAT, repeating 2 groups, with 2 cycles in
a group

50. flexdr4-5.wav DISTORT REPEAT, repeating 4 groups, with 5 cycles in
a group (much longer)

**Looping**

51. flexloops.wav EXTEND LOOP, with 1/3 overlap of the loop_lengths
(len 300, step 100)

52. flextexture.wav EXTEND LOOP, more textured because of 4/5 overlap
of the loop_lengths (len 100 step 20)

**Scramble segments**

53. flexscram1a.wav ****EXTEND SCRAMBLE, random chunks somewhere
between 0.1 and 0.3 sec in length

54. flexscram1b.wav EXTEND SCRAMBLE, random chunks between 0.06 and 0.2
sec in length

55. flexscram2.wav EXTEND SCRAMBLE, Mode 2 to rearrange segments, with
scatter = 3

**Brassage ('mash')**

56. flexgrnstr.wav MODIFY BRASSAGE, Mode 2 with 4 x timestretch (0.25
divided into 1 = 4)

57. flexgrnstrtv.wav ****MODIFY BRASSAGE, Mode 2, with time-varying
timestretch **flexgrnstrtv.brk**

58. flextvgrains.wav ****MODIFY BRASSAGE, Mode 4, with randomly varying
grainsize between 25 and 200 ms

59. flexgrainy.wav MODIFY BRASSAGE, Mode 5, granulate the sound, with
tiny gaps (0.75)

60. flexbrassage.wav Using **flexgrainy.wav** as the input - MODIFY
BRASSAGE, Mode 7, multi-parameter

time-varying brassage, using Preset **flexbrassageC** or
**flexbrassageD**, stored

in the Preset collection **wcc.dat** with **flexdens.brk flexgsize.brk
flexpchlo.brk**

and **flexpchhi.brk** (Brassage and GrainMill both work their way
through the whole sound)

**Texture**

61. flextxsimple.wav TEXTURE SIMPLE, Mode 5 ('None') for a different
approach (always starts from the

beginning of the sound). The segments are between 0.75 and 1.3 sec.
long, so we repeatedly

hear a small part of the beginning of the sound, unless 'Use whole
sound' is ticked.

Uses the Preset **flextextureC** or **FlextextureD** from the Preset
collection **wcc.dat**,

with **flexpack.brk** **flextxplo.brk** and **flextxphi.brk**

62. flexgrainmill.wav GRAINMILL - virtually the same settings as
Brassage, but the pitch range expands more.

This process also works its way through the whole sound. Preset
**flexC** or **flexD** from the

Preset collection **wcc.dat**, with the files **flexdens.brk
flexgsize.brk flexpchlo.brk** and

**flexpchhi.brk**

63. 6 sec. silence

**Workshop 4 - HARMONIC TUNING / MIX - SUMMARY & PLAYLIST**

**Prepare the soundfile**

64. flex.wav original input (a tractor, 10 sec. mono -- also on Track
17)

65. flexgrainy.wav MODIFY BRASSAGE, Mode 5 density 0.5

66. flexgybrcdt.wav MODIFY BRASSAGE, Mode 7 with **flexbrassageC** or
**flexbrassageD** Preset,

then CUT and DOVETAILED

**Time Domain Tuning**

**Filterbank**

67. flexgybrcdtfb1c.wav FILTER USERBANK with **flexgyCmaj.txt** to tune
to a C-major chord, and

time-varying Q: **flexgyQ.brk**, CUT 0 to 6 sec.

68. flexgybrcdtfb2c.wav FILTER USERBANK with **flexgyCmin.txt** to tune
to a C-minor chord, and

time-varying Q: **flexgyQ.brk**, CUT 0 to 6 sec.

**Texture**

69. flexgybrcdtfb1ctxarpcg.wav TEXTURE SIMPLE with Preset
**Mode4majarpegC** or **Mode4majarpegD**

and **ndf60hsarp1.txt** (C-major grid), CUT 0 to 18.2 and GAIN x 2.5

70. flexgybrcdtfb2ctxarpcg.wav TEXTURE SIMPLE with Preset
**Mode4minarpegC** or **Mode4minarpegD**

and **ndf60hsarp2.txt** (C-minor grid), CUT 0 to 18.2 and GAIN x 2.5

**Mix**

71. flexarpmajmindt.wav SUBMIX MIX, alternating the major and minor
textures, using the mixfile

**arpmajmindtC.mix** or **arpmajmindtD.mix**

**Spectral Domain Tuning**

**Tune**

72. flexgybrcdtmtune.wav PITCH TUNE using **flextune.tun** (Spectral
menu)

73. flexdimchord.wav REPITCH TRANSPOSE (transpose without changing the
duration)

and MIX the results: **flexdimchordC.mix** or **flexdimchordD.mix**.

**Mix**

74. capmvoices.wav SUBMIX MIX, voices tune 2 semitones above and below
the original, using the

mixfile **capmvoicesC.mix** or **capmvoicesD.mix**

75. capmvoicesoffset.wav SUBMIX MIX, voices tune 2 semitones above and
below the original, with

start time offset, using the mixfile **capmvoicesoffsetC.mix** or

**capmvoicesoffsetD.mix**

76. 6 sec. silence

**Workshop 5 - TIME: HOLDING AND STRETCHING - SUMMARY & PLAYLIST**

**Freeze forwards & backwards**

77. flexfrz1.wav FOCUS FREEZE - using **flexfrz1.txt** to create a
pattern of symmetric lengths

78. flexfrz2.wav FOCUS FREEZE - using **flexfrz2.txt** to create a
pattern of varying lengths, often adjacent

**Examples of developed applications**

79. flexfrz2cdttx1d24cdt.wav CUT a frozen portion & DOVETAIL + TEXTURE
with a narrow pitch range + down

2 octaves and tidied up with a CUT & DOVETAIL.

80. flexfrz2cdttx2.wav CUT a frozen portion & DOVETAIL + TEXTURE with a
2 octave pitch range. This

leaves a high, somewhat piercing but rich sound, changing mechanically.

**Freeze (hold) for specified lengths**

81. trcdthold.wav FOCUS HOLD - using **holdtimes.txt** to create a
pattern of compressing lengths

82. capmtra10hold.wav FOCUS HOLD - the same again, with a different
input, first 'reduced' to 10 analysis

channels

**Stepfreeze (regular, cannot time-vary)**

83. trcdtstp&1.wav FOCUS STEP - mechanical churning with a regular
stepfreeze of 0.1 sec.

84. flexstp&25.wav FOCUS STEP - the flexatone becomes a bit tuneful
with a stepfreeze of 0.25 sec.

**Example of a developed application**

85. flextvgrnx2stp&25bltrd24.wav Time-varying timestretch granulation +
spectral timestretch + stepfreeze 0.25 +

blur (100 windows) & trace (20 analysis channels) + down 2 octaves. Here
we've

stretched two different ways before the stepfreeze, then smoothed with
BLTR and

enriched the (still high) sound by lowering it. Notice how you can read
the whole

processing sequence in the name: **flex** -\> **t**ime-**v**arying
**gr**ai**n**s -\> stretched **x2**

-\> **st**e**p** 0.**25** -\> **bl**ur-**tr**ace -\> **d**own **24**
semitones.

**Stretch Time** (a massive 64 times -- and some sounds benefit from
even more -- there is no limit)

86. capmc.wav Starting point: "The extraticular momentum shields"

87. capmcx2.wav The words are elongated.

88. capmcx4.wav A real drawl.

89. capmcx8.wav Getting silly

90. capmcx16.wav Very drawn out now, and we especially notice the
consonants as sound objects in themselves. They in turn can become
useful source material.

91. capmx32.wav Becoming abstract.

92. capmx64c.wav Extremely slow and abstract, with accumulated silence
at the beginning removed.
