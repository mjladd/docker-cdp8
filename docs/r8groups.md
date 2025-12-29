# CDP Release 8 Programs - Core Groups

*[Any programs not listed here fall into a general group of "misc" processing tools, some as background utility programs for Soundloom]*

## 1. Synthesis

**General note:** CDP hitherto has essentially limited pure synthesis processes to the generation of basic signals ("synth", "newsynth").

Release 8 introduces a number of new tools, all idiosyncratic one way and another, extending to note and rudimentary score-based composition. We suggest they should all be regarded as R&D tools, maybe production of raw source material, rather than anything that would directly form or contribute to a completed piece. However, for developers they may provide opportunities for custom adaptations and extensions.

- **clicknew:** Make clicktrack using times listed in textfile.

- **multiosc:** FM-style chain of sines, four "operators". Limited envelope control.

- **multisynth:** Synthesize several sound-streams from a score. Targetted at multichannel projects, but stereo also supported.

- **newscales:** Basic note generation using a fixed spectrum, no enveloping. The program name suggests it is used for simple "scale" generation, e.g. for experimental tunings.

- **newsynth:** Updated for R8, new mode "fractally arrayed spikes"

- **pulser, Mode 3 "synth":** Generate wave-packets with fixed or varying spectrum.

- **synspline:** Synthesis from waveforms made by smoothly joining randomly generated points.

- **ts:** Waveform generation/plot using time-series data, as in "drawing" a waveform. Should this be called "Fairlight"?

## 2. Waveset Distortion Processes

- **distcut:** Cut sound into elements (multiple outfiles) with falling envelope.

- **distmark:** Interpolate between waveset groups.

- **distmore:** Multiple new distortion processes. Mostly aimed at speech processing.

- **distrep:** Waveset-based timestretching.

- **distshift:** Literally, shift groups of half-wavecycles in time.

- **(distwarp:** Warp wavecycles. *Not in Soundloom)*

- **partition:** Partition infile into multiple files by waveset blocks.

- **quirk:** Raise samples to a power (<>1), based on wavecycles.

- **scramble:** Scramble order of wavesets in infile. 14 processes.

- **splinter:** Creates splinters by repeating & shrinking selected waveset-group in sound. Either splinters repeat before merging with orig snd at time-in-src specified, OR original sound plays up to selected time, then splinters.

See also **clip mode 2** (half-waveforms)

## 3. Pvoc/Amp/Freq Analysis/Transformation Tools

Several of these appear to be primarily tools for Soundloom data display.

- **caltrain:** Time-blur upper partials.

- **cubicspline:** (datafile only): Smooth amp/freq data points to make spectrum.

- **specanal:** Custom (complex) version of pvoc, generate data files, e.g. to visualise spectrum.

- **specav:** Average spectrum, output multiple spectrum datafiles.

- **specenv:** Spectral envelope transfer, probably like classic "pvcross".

- **specfnu:** Large complex program to process formants, spectral shape.

- **specfold:** Fold/invert/randomize part of the spectrum.

- **spectstr:** Time-stretching.

- **spectune:** Find and manipulate pitches in input file. Options to map to pitch data in "tuning file".

- **speculate:** Permutations of channel data, creates large number of outfiles.

- **suppress:** Suppress most prominent partials in chosen freq band.

## 4. Multichannel Tools

Primarily for discrete surround, e.g. 8 or even 16 chans. Many support simple stereo too.

- **brownian:** Generate texture of [short] sampled elements, based on brownian motion.

- **cascade:** Segments of source repeat-echoed, accumulated. Many options.

- **crumble:** Project mono source to all chans, plus segmentation etc. Complex, output either 8 chans or 16 chans(!).

- **crystal:** Based on model of crystal (regular polyhedron?), with 3d rotations.

- **multisynth:** (see above), includes multichan options.

- **onset:** Find successive onsets in m/c file. May be utility for Soundloom.

- **pairex:** Extract arbitrary pair of chans from m/c file.

- **pulser:** (see above) Streams of pulses, some m/c options.

- **repair:** Join mono sounds into stereo/multichan outputs. So name = "re-pair".

- **rotate:** Generate list of x/y coordinates - for Soundloom?

- **spin:** Spin stereo image across m/c stage, usage msg a bit ambiguous, seems to offer general n-chan options, while also specifying stereo, quad, 5-chan.

- **tesselate:** Repeating and shifting patterns (in space, time). Outchans >= 2.

- **tremenv:** Tremolo a sound, width narrowed, after peak.

## 5. Speech Processing

These explicitly reference syllables, etc, in their usage messages.

- **distmore:** (see above) Mode USAGE msgs cite application to vowels/consonants.

- **envspeak:** Process speech "syllables". *[NOT IN SOUNDLOOM]*

- **flatten:** Equalise sound elements. Usage implies application to syllables etc.

- **stutter:** Slice source into elements (e.g. words or syllables).
