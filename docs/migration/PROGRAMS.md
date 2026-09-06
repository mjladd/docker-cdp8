# Program Inventory (generated from usage output of the Linux build)

Generated from the Docker image built at commit 94dbfc1. Sub-command lists are parsed from the "where NAME can be" usage blocks and are indicative. The first usage line is shown for single-purpose programs.

Group programs: 44. Single-purpose programs: 176.

## Group programs

| Program | Source dir | Sub-commands |
|---|---|---|
| `blur` | dev/blur | avrg, blur, chorus, drunk, noise, scatter, shuffle, spread, suppress, weave |
| `combine` | dev/combine | cross, diff, interleave, make, make2, max, mean, sum |
| `distmore` | dev/standnew | bright, double, segsbkwd, segszig |
| `distort` | dev/distort | average, cyclecnt, delete, divide, envel, filter, fractal, harmonic, interact, interpolate, multiply, omit, overload, pulsed, reform, repeat, repeat2, replace, replim, reverse, shuffle, telescope |
| `envel` | dev/env | attack, brktoenv, create, curtail, cyclic, dbtoenv, dbtogain, dovetail, envtobrk, envtodb, extract, gaintodb, impose, pluck, replace, replot, reshape, scaled, swell, timegrid, tremolo, warp |
| `envnu` | dev/standalone | envnu, exit, expdecay, info, more, on, peakchop |
| `extend` | dev/extend | baktobak, doublets, drunk, freeze, iterate, loop, repetitions, scramble, sequence, sequence2, zigzag |
| `filter` | dev/filter | bank, bankfrqs, fixed, iterated, lohi, phasing, sweeping, userbank, variable, varibank, varibank2, vfilters |
| `focus` | dev/focus | accu, exag, focus, fold, freeze, hold, step |
| `fofex` | dev/standalone | construct, exit, extract, fofex, info, more, on |
| `formants` | dev/formants | get, getsee, put, see, vocode |
| `fractal` | dev/standnew | spectrum, wave |
| `fturanal` | dev/standnew | anal, exit, fturanal, info, more, on, option, synth |
| `grain` | dev/grain | align, assess, count, duplicate, find, grev, noise_extend, omit, r_extend, remotif, reorder, repitch, reposition, rerhythm, reverse, timewarp |
| `hfperm` | dev/hfperm | be, can, delperm, delperm2, hfchords, hfchords2, hfperm, infile, mode, outfile, parameters, where |
| `hilite` | dev/hilite | arpeg, band, bltr, filter, greq, pluck, trace, vowels |
| `housekeep` | dev/houskeep | bakup, batchexpand, bundle, chans, copy, deglitch, disk, endclicks, extract, gate, remove, respec, sort |
| `modify` | dev/modify | brassage, convolve, findpan, loudness, radical, revecho, sausage, scaledpan, shudder, space, spaceform, speed, stack |
| `morph` | dev/morph | bridge, glide, morph |
| `newmorph` | dev/new | newmorph, newmorph2 |
| `oneform` | dev/standalone | combine, get, put |
| `pitch` | dev/pitch | altharms, chord, chordf, octmove, pick, transp, tune |
| `pitchinfo` | dev/pitchinfo | convert, hear, info, see, zeros |
| `psow` | dev/standalone | chop, cutatgrain, delete, dupl, features, grab, impose, interleave, interp, locate, reinforce, replace, space, split, stretch, strtrans, sustain, sustain2, synth |
| `pulser` | dev/science | multi, pulser, synth |
| `pvoc` | dev/pv | anal, extract, synth |
| `repitch` | dev/repitch | analenv, approx, combine, combineb, cut, exag, fix, generate, getpitch, insertsil, insertzeros, interp, invert, noisetosil, pchshift, pchtotext, pitchtosil, quantise, randomise, smooth, synth, transpose, transposef, vibrato, vowels |
| `sfedit` | dev/editsf | cut, cutend, cutmany, excise, excises, insert, insil, join, joindyn, joinseq, masks, noisecut, randchunks, randcuts, replace, sphinx, syllables, twixt, zcut, zcuts |
| `sndinfo` | dev/sndinfo | chandiff, diff, findhole, len, lens, loudchan, maxi, maxsamp, maxsamp2, prntsnd, props, smptime, sumlen, timediff, timesmp, units, zcross |
| `spec` | dev/spec | bare, clean, cut, gain, gate, grab, magnify |
| `specinfo` | dev/specinfo | channel, frequency, level, octvu, peak, print, report, windowcnt |
| `speclean` | dev/standalone | clean |
| `specnu` | dev/standalone | clean, rand, remove, slice, squeeze, subtract |
| `spectrum` | dev/new | fixed, format, lines, varying |
| `speculate` | dev/standnew | speculation |
| `spin` | dev/science | quad, stereo |
| `strange` | dev/strange | glis, invert, shift, waver |
| `stretch` | dev/stretch | spectrum, time |
| `submix` | dev/submix | addtomix, atstep, attenuate, balance, crossfade, dummy, faders, fileformat, getlevel, inbetween, inbetween2, interleave, merge, mergemany, mix, model, ongrid, pan, shuffle, spacewarp, sync, syncattack, test, timewarp |
| `synth` | dev/synth | chord, clicks, noise, silence, spectra, wave |
| `tangent` | dev/new | list, onefile, sequence, twofiles |
| `texture` | dev/texture | decorated, grouped, motifs, motifsin, ornate, postdecor, postornate, predecor, preornate, simple, tgrouped, timed, tmotifs, tmotifsin |
| `transit` | dev/new | doplfilt, doppler, filtered, list, sequence, simple |
| `ts` | dev/science | oscil, trace |

## Single-purpose programs

| Program | Source dir | First usage line |
|---|---|---|
| `abfdcode` | dev/externals/mctools | CDP MCTOOLS: ABFDCODE v 1.2.1: CDP 1999,2004,2005 |
| `abfpan` | dev/externals/mctools | CDP MCTOOLS V1.5.3 (c) RWD,CDP 2009,2010,2013 |
| `abfpan2` | dev/externals/mctools | CDP MCTOOLS V1.0.1 beta (c) RWD,CDP 2010 |
| `analjoin` | dev/standalone | JOIN ANALYSIS FILES TOGeTHER |
| `asciiget` | dev/standnew | USAGE: getascii filename: where file is an ascii textfile |
| `bounce` | dev/science | USAGE: |
| `brkdur` | dev/misc | ERROR: Bad function call. |
| `brktopi` | dev/standalone | USAGE:     brktopi brktopi pitch-textfile binary-outfile |
| `brownian` | dev/science | USAGE: |
| `caltrain` | dev/standnew | USAGE: |
| `cantor` | dev/new | USAGE: |
| `cascade` | dev/science | USAGE: cascade cascade 1-5  inf outf clipsize  echos clipmax [-eechosmax] |
| `cdparams` | dev/cdparams | ERROR: Wrong number of params to cdparams() |
| `cdparams_other` | dev/cdparams_other | ERROR: Wrong number of params to cdparams() |
| `cdparse` | dev/cdparse | ERROR: Incorrect call to cdparse() |
| `ceracu` | dev/new | USAGE: |
| `channelx` | dev/externals/mctools | CDP MCTOOLS: CHANNELX V1.6 (c) RWD, CDP 2010 |
| `chanphase` | dev/new | USAGE: |
| `chirikov` | dev/science | USAGE: |
| `chorder` | dev/externals/mctools | CDP MCTOOLS: CHORDER V1.2 (c) RWD,CDP 2009,2010 |
| `chxformat` | dev/externals/mctools | CDP MCTOOLS: CHXFORMAT v1.0.1beta (c) RWD,CDP 2009 |
| `clicknew` | dev/standnew | USAGE: clicknew clicks outfile clicktimes_datafile srate |
| `clip` | dev/standnew | USAGE: |
| `columns` | dev/tabedit | USAGE: columns infile [outfile] -flag[@] [{threshold/}threshold] [--cCOLCNT] |
| `constrict` | dev/standalone | USAGE: |
| `convert_to_midi` | dev/standnew | Converts frqequency-brkpnt and peakdata textfiles to a standard midi data file. |
| `copysfx` | dev/externals/mctools | CDP MCTOOLS: COPYSFX  (c) RWD,CDP Revision: 2.1.1 2020 |
| `crumble` | dev/science | USAGE: crumble sound 1 inf outf stt dur1 dur2      params |
| `crystal` | dev/science | crystal rotate 1-10 fi [fi2 fi3..] fo vdat rota rotb twidth tstep dur plo phi |
| `cubicspline` | dev/science | USAGE: cubicspline datafile outdatafile pointcnt srate [-s] |
| `dirsf` | dev/sfutils | Sfsys Version 8.01 |
| `diskspace` | dev/misc | ERROR: Cannot run this process. |
| `distcut` | dev/standnew | USAGE: |
| `distmark` | dev/standnew | USAGE: |
| `distortt` | dev/science | USAGE: |
| `distrep` | dev/standnew | USAGE: |
| `distshift` | dev/standnew | USAGE: |
| `dshift` | dev/standalone | USAGE:	dshift [-dN] infile outfile [distance between speakers] |
| `dvdwind` | dev/science | USAGE: dvdwind dvdwind infile outfile contraction clipsize |
| `envcut` | dev/standnew | USAGE: |
| `envspeak` | dev/standnew | USAGE: |
| `fastconv` | dev/externals/fastconv | fastconv v1.2 RWD,CDP July 2010,2013 |
| `features` | dev/standalone | Use an anlysis file to find the MOST PROMINENT FEATURES in a sound source. |
| `filtrage` | dev/new | USAGE: |
| `fixgobo` | dev/misc | ERROR: Bad call to fixgobo. |
| `flatten` | dev/science | USAGE: |
| `flutter` | dev/standalone | USAGE: flutter flutter inf outf chanseq freq depth gain [-r] |
| `fmdcode` | dev/externals/mctools | CDP MCTOOLS: FMDCODE v 1.0beta: RWD,CDP 2009 |
| `fracture` | dev/new | USAGE:fracture fracture 1 |
| `frame` | dev/standalone | USAGE: |
| `freeze` | dev/standalone | FREEZE A SEGMENT OF A SOUND BY ITERATION IN A FLUID MANNER |
| `frfractal` | dev/standnew | USAGE: |
| `gate` | dev/standalone | USAGE: |
| `get_partials` | dev/standalone | USAGE: get_partials harmonic 1-2 inanalfile outfile fundamental threshold [-v] |
| `getcol` | dev/tabedit | USAGE : getcol infile outfile colno [skiplines] [-e] |
| `glisten` | dev/new | glisten glisten inf outf grpdiv setdur [-ppitchshift] [-ddurrand] [-vdivrand] |
| `gobo` | dev/misc | TO TEST THE GOBO SETTING FOR A NEW PROGRAM |
| `gobosee` | dev/misc | ERROR: Bad call 1 to gobosee |
| `grainex` | dev/standalone | FIND GRAINS IN A SOUND, AND EXTEND AREA THAT CONTAINS THEM |
| `histconv` | dev/misc | USAGE: histconv infile outfile |
| `hover` | dev/standalone | USAGE: |
| `hover2` | dev/standnew | USAGE: |
| `impulse` | dev/science | USAGE: |
| `interlx` | dev/externals/mctools | interlx: insufficient arguments |
| `isolate` | dev/new | USAGE: |
| `iterfof` | dev/new | USAGE: |
| `iterline` | dev/new | USAGE: |
| `iterlinef` | dev/new | USAGE: |
| `listdate` | dev/misc | Sep06_03-06-52.2026[exit=0] |
| `logdate` | dev/misc | Sep6_03-06.2026[exit=0] |
| `madrid` | dev/new | USAGE: |
| `manysil` | dev/standalone | USAGE: |
| `matrix` | dev/standnew | MATRIX MANIPULATION OF SPECTRUM OF SOUND |
| `maxsamp2` | dev/misc | ERROR: wrong number of arguments. |
| `mchanpan` | dev/standalone | USAGE: mchanpan mchanpan 1 inf outf panfile outchans [-ffocus] |
| `mchanrev` | dev/standalone | CREATE MULTICHANNEL ECHOS OR REVERB |
| `mchiter` | dev/standalone | ITERATE INPUT SOUND IN A FLUID MANNER, SCATTERING TO MULTICHANNEL SPACE |
| `mchshred` | dev/standalone | USAGE: |
| `mchstereo` | dev/standalone | USAGE: |
| `mchzig` | dev/standalone | USAGE: mchzig zag 1 infile outfile start end dur minzig outchans |
| `motor` | dev/science | USAGE: motor motor 1,4,7 infile outfile params |
| `mton` | dev/standalone | USAGE: |
| `multimix` | dev/standalone | CONVERT LIST OF SNDFILES TO A MULTICHANNEL MIXFILE |
| `multiosc` | dev/science | USAGE: |
| `multisynth` | dev/science | USAGE: |
| `newdelay` | dev/new | USAGE: newdelay newdelay infile outfile midipitch mix feedback |
| `newmix` | dev/standalone | USAGE:     newmix multichan mixfile outsndfile [-sSTART] [-eEND] [-gATTENUATION] |
| `newscales` | dev/science | USAGE: newscales outfile datafile spectrumfile [srate] |
| `newsynth` | dev/science | USAGE: |
| `newtex` | dev/new |  |
| `njoin` | dev/externals/mctools | njoin: insufficient arguemnts |
| `nmix` | dev/externals/mctools | CDP MCTOOLS: NMIX V2.0.1 (c) RWD,CDP 1999,2009 |
| `notchinvert` | dev/science | USAGE: notchinvert datafile outdatafile srate [minnotch] |
| `onset` | dev/standnew | USAGE: |
| `packet` | dev/new | USAGE: |
| `pagrab` | dev/pagrab | ERROR: Wrong number of arguments. |
| `pairex` | dev/standnew | USAGE: |
| `panorama` | dev/new | USAGE: panorama panorama 1 infile infile2 [infile3.....] outmixfile |
| `partition` | dev/standnew | USAGE: |
| `paudition` | dev/misc | ERROR: Incorrect call to program which writes the sound. |
| `paview` | dev/paview | ERROR: Wrong number of arguments. |
| `pdisplay` | dev/misc | ERROR: Wrong number of arguments: pdisplay sndfilename. |
| `peak` | dev/standalone | USAGE: peak extract |
| `peakfind` | dev/standalone | USAGE: |
| `peakiso` | dev/science | USAGE: peakiso datafile outdatafile srate [minnotch] |
| `phase` | dev/standalone | USAGE: |
| `phasor` | dev/standnew | USAGE: |
| `pmodify` | dev/misc | ERROR: Incorrect call to program which writes the pitch data. |
| `prefix` | dev/standalone | USAGE: |
| `progmach` | dev/misc | ERROR: Bad number of arguments to progmach. |
| `ptobrk` | dev/standalone | USAGE:     ptobrk withzeros binary-pitchfile outtextfile min-pitch-dur |
| `putcol` | dev/tabedit | USAGE : putcol columnfile intofile outfile colno -r/-i [[-]skiplines] [-e] |
| `pview` | dev/pview | ERROR: Wrong number of arguments. |
| `quirk` | dev/standnew | USAGE: |
| `refocus` | dev/science | USAGE: refocus refocus 1-5 outname dur bandcnt focratio tstep trand |
| `rejoin` | dev/new | USAGE: |
| `repair` | dev/standnew | USAGE: |
| `repeater` | dev/science | USAGE: repeater repeater |
| `retime` | dev/standalone | USAGE: |
| `reverb` | dev/externals/reverb | reverb: Multi-channel reverberator |
| `rmresp` | dev/externals/reverb | *********  ROOMRESP.EXE: by Tom Zudock  CDP build 1998,1999 ****** |
| `rmsinfo` | dev/externals/mctools | CDP MCTOOLS: RMSINFO v1.0.1 (c) RWD, CDP 2009 |
| `rmverb` | dev/externals/reverb | rmverb: Multi-channel reverb with room simulation |
| `rotor` | dev/science | USAGE: |
| `scramble` | dev/science | USAGE: |
| `search` | dev/standalone | USAGE: |
| `selfsim` | dev/new | INCREASE SPECTRAL SELF-SIMILARITY |
| `sfecho` | dev/new | USAGE: |
| `sfprops` | dev/externals/mctools | CDP MCTOOLS: SFPROPS v2.2.1 (c) RWD,CDP,1999,2009,2010,2013,2023 |
| `shifter` | dev/new | USAGE: shifter shifter |
| `shrink` | dev/new | USAGE: shrink shrink 1-3 infile outfile shrinkage |
| `silend` | dev/new | USAGE: |
| `smooth` | dev/science | USAGE: smooth datafile outdatafile pointcnt srate [-s] |
| `sorter` | dev/science | USAGE: |
| `spacedesign` | dev/standalone | ERROR: USAGE: spacedesign mode outfile params |
| `specanal` | dev/science | Generate various types of analysis data, or filter data from sound. |
| `specav` | dev/science | USAGE: specav specav 1 inafil outfil starttime endtime [-n] |
| `specenv` | dev/standnew | USAGE: |
| `specfnu` | dev/science | USAGE: specfnu specfnu 1-23 inanalfile outfile [params] |
| `specfold` | dev/science | USAGE: specfold specfold 1 inanalfile outanalfile stt len cnt  [-a] |
| `specgrids` | dev/new | PARTITION SPECTRUM INTO PARTS, OVER A GRID |
| `specross` | dev/standalone | INTERPOLATE PARTIALS OF PITCHED SRC1 TOWARDS THOSE OF PITCHED SRC 2 |
| `specsphinx` | dev/new | USAGE: specsphinx specsphinx 1 analfile1 analfile2 outanalfile |
| `spectstr` | dev/science | spectstr stretch time infile outfile timestretch d-ratio di-rand |
| `spectune` | dev/science | FURTHER OPERATIONS ON ANALYSIS FILES |
| `spectwin` | dev/new | USAGE: spectwin spectwin |
| `specvu` | dev/new | USAGE: |
| `spike` | dev/standnew | USAGE: |
| `splinter` | dev/science | USAGE: |
| `strands` | dev/science | USAGE: strands strands 1 generic_outdatafilename ... |
| `strans` | dev/standalone | CHANGE SPEED & PITCH OF (MULTICHANNEL) SRC SOUND, OR ADD VIBRATO. |
| `stretcha` | dev/misc | USAGES: |
| `stutter` | dev/science | USAGE: |
| `subtract` | dev/new | USAGE: |
| `superaccu` | dev/new | superaccu superaccu 1 inanalfile outanalfile [-ddecay] [-gglis] [-r] |
| `suppress` | dev/standnew | USAGE: suppress partials inanal outanal timeslots lofrq hifrq chancnt |
| `synfilt` | dev/science | NOISE FILTERED BY TIME_VARYING FILTERBANK,WITH TIME-VARIABLE Q |
| `synspline` | dev/science | USAGE: synspline synspline outfile srate dur frq splinecnt interpval seed |
| `tapdelay` | dev/externals/reverb | *******  STEREO MULTI-TAPPED DELAY WITH PANNING v1.0 1998 : CDP Release 4 ****** |
| `tesselate` | dev/science | USAGE: |
| `texmchan` | dev/standalone | USAGE: |
| `tkusage` | dev/misc | ERROR: Error in usage program tkusage |
| `tkusage_other` | dev/misc | ERROR: Error in usage program tkusage_other |
| `topantail2` | dev/standalone | USAGE: |
| `tostereo` | dev/standnew | USAGE: |
| `tremenv` | dev/science | TREMOLO A SOUND, WIDTH NARROWED, AFTER PEAK |
| `tremolo` | dev/new | TREMOLO A SOUND, WITH WIDTH NARROWING |
| `tsconvert` | dev/science | USAGE: tsconvert indata outdata min max [-cminstep/-r/-q/-Q] [-ddur [-ftimes]] [-mmaxoutdu |
| `tunevary` | dev/new | tunevary tunevary infile outfile pitch_template |
| `tweet` | dev/science | USAGE: |
| `unknot` | dev/science | USAGE: unknot unknot 1-2 inf1 inf2 [inf3 ...]    outfile combos r_pats r_combos r_all r_un |
| `vectors` | dev/tabedit | ERROR: Invalid command |
| `verges` | dev/science | USAGE: |
| `vuform` | dev/misc | ERROR: Insufficient params:: vuform formantfile. |
| `waveform` | dev/science | USAGE: waveform make 1 infile outfile time cnt. |
| `wrappage` | dev/standalone | GRANULAR RECONSTITUTION OF ONE OR MORE SOUNDFILES OVER MULTICHANNEL SPACE. |
