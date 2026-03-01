# Transposition & Shifting Inputs and Outputs

| CDP FUNCTION | Mode | Input 1 | Input 2 OR Output | Output | Key Parameters/[Comments] |
|--------------|------|---------|-------------------|--------|---------------------------|
| REPITCH GETPITCH<br>(get pitch trace) | 1 | `in.ana` | `outlisten.ana` | `out.frq` | [binary pitch data file] |
| REPITCH GETPITCH | 2 | `in.ana` | `outlisten.ana` | `out.brk` | [breakpoint (text) file] |
| REPITCH TRANSPOSE<br>(M4: route for .trn to .ana) | 1–3 | `in.ana` | | `out.ana` | *transpose* (ratio, 8<sup>ves</sup>, semitones) |
| REPITCH TRANSPOSE | 4 | `in.ana` | `in.trn` | `out.ana` | [ready to synthesise] |
| REPITCH TRANSPOSEF<br>(M4: route for .trn to .ana) | 1–3 | `in.ana` | | `out.ana` | `-p` or `-f` (formant extraction)<br>*transpose* (ratio, 8<sup>ves</sup>, semitones) |
| REPITCH TRANSPOSEF | 4 | `in.ana` | `in.trn` | `out.ana` | `-p` or `-f` (formant extraction)<br>[ready to synthesise] |
| REPITCH COMBINE<br>NB: creates .trn outputs | 1 | `in1.frq` | `in2.frq` | `out.trn` | [can be input to REPITCH TRANSPOSE/F] |
| REPITCH COMBINE | 2 | `in.frq` | `in.trn` | `out.frq` | [can be given to another .frq processor] |
| REPITCH COMBINE | 3 | `in1.trn` | `in2.trn` | `out.trn` | [can be input to REPITCH TRANSPOSE/F] |
| PITCH TRANSP | 1-3 | `in.ana` | | `out.ana` | *frq_split* [frequency split point] |
| PITCH TRANSP | 4-5 | `in.ana` | | `out.ana` | *frq_split* [frequency split point]<br>*transpose* [semitones] |
| PITCH TRANSP | 6 | `in.ana` | | `out.ana` | *frq_split* [frequency split point]<br>*transpose1 transpose2* [semitones] |
| PITCH OCTMOVE | 1-2 | `in.ana` | `in.frq` | `out.ana` | *transposition* (integer >0:<br>follows harmonic series)<br>[in.frq is derived from in.ana] |
| PITCH OCTMOVE | 3 | `in.ana` | `in.frq` | `out.ana` | *transposition bassboost* |
| STRANGE SHIFT | 1 | `in.ana` | | `out.ana` | *frqshift* (in Hz) |
| STRANGE SHIFT | 2-3 | `in.ana` | | `out.ana` | *frqshift frq_divide* (in Hz) |
| STRANGE SHIFT | 4-5 | `in.ana` | | `out.ana` | *frqshift frqlo frqhi* (in Hz) |
| REPITCH APPROX | 1 | `in.frq` | | `out.frq` | *prange trange srange* |
| REPITCH APPROX | 2 | `in.frq` | | `out.trn` | *prange trange srange* |
| REPITCH EXAG | 1 | `in.frq` | | `out.frq` | *meanpch range* |
| REPITCH EXAG | 2 | `in.frq` | | `out.trn` | *meanpch range* |
| REPITCH EXAG | 3 | `in.frq` | | `out.frq` | *meanpch contour* |
| REPITCH EXAG | 4 | `in.frq` | | `out.trn` | *meanpch contour* |
| REPITCH EXAG | 5 | `in.frq` | | `out.frq` | *meanpch range contour* |
| REPITCH EXAG | 6 | `in.frq` | | `out.trn` | *meanpch range contour* |
| REPITCH INVERT | 1 | `in.frq` | | `out.frq` | *map* (file) ... |
| REPITCH INVERT | 2 | `in.frq` | | `out.trn` | *map* (file) ... |
| REPITCH CUT | 1 | `in.frq` | | `out.frq` | *starttime* |
| REPITCH CUT | 2 | `in.frq` | | `out.frq` | *endtime* |
| REPITCH CUT | 3 | `in.frq` | | `out.frq` | *starttime endtime* |
| REPITCH FIX | 0 | `in.frq` | | `out.frq` | [several options] |
| REPITCH PCHSHIFT | 0 | `in.frq` | | `out.frq` | *transposition* (semitone constant) |
| REPITCH QUANTISE | 1 | `in.frq` | | `out.frq` | *q_set* (file of MIDI pitchvals) |
| REPITCH QUANTISE | 2 | `in.frq` | | `out.trn` | *q_set* (file of MIDI pitchvals) |
| REPITCH RANDOMISE | 1 | `in.frq` | | `out.frq` | *maxinterval timestep* |
| REPITCH RANDOMISE | 2 | `in.frq` | | `out.trn` | *maxinterval timestep* |
| REPITCH SMOOTH | 1 | `in.frq` | | `out.frq` | *timeframe* ... |
| REPITCH SMOOTH | 2 | `in.frq` | | `out.trn` | *timeframe* ... |
| REPITCH VIBRATO | 1 | `in.frq` | | `out.frq` | *vibfrq vibrange* |
| REPITCH VIBRATO | 2 | `in.frq` | | `out.trn` | *vibfreq vibrange* |
