# Reverb and Amplitude

*by Dr Archer Endrich*

*some observations on a few topics*

| | |
|---|---|
| [**Reverb and Delay**](#REVERBDELAY) | [**Amplitude**](#AMPLITUDE) |

## About Reverberation and Delay {#REVERBDELAY}

There can be some ambiguity between 'reverb' and 'delay' because reverb is basically a delay process. Tiny delays and multiple, varied delays (varied sound reflections) cause a resonance effect in which the fact that a sound is repeating is hidden. With longer delays the repeat of the sound is clearly audible, becoming full-scale echoes. All sound packages contain a reverb module, so it can be matter of choosing the one that works best for what is needed. In the CDP software, REVERB (multi-channel reverb) seems to give the smoothest results and MODIFY REVECHO mode 3 (stadium echo) produces superb echo effects. MODIFY REVECHO mode 1 can be used to play with reverb / delay balance points. The REVERB group also contains ROOMRESP and ROOMVERB (reverb with configurable room reflections) and TAPDELAY (stereo multi-tapped delay line with feedback).

## Amplitude handling {#AMPLITUDE}

Amplitude has many facets. These are some considerations to bear in mind when designing sounds and putting them together in a passage of music.

- **Normalisation** means applying the same gain ratio to the whole signal in order to bring it up to a target level, usually at or very close to full amplitude (0dB). Because it is applied across the board, as it were, the signal-to-noise ratio and relative dynamics of the sound samples remain unchanged. There is a double caveat here when a sound is going be processed: the processes will need a good signal with which to work, especially if filtering is involved; on the other hand, they may need some amplitude 'headroom' to avoid resonant overload. The general rule is to leave some headroom for processing, and perhaps normalise at the end of the process chain.

- **Equalisation** means adjusting the relative amplitude of the partials in a sound, for example to make the sound 'brighter' (increase the amplitude of the high frequencies) or to reduce excessive bass resonance (decrease the low frequencies).

- **Compression** evens out the relative amplitudes of the sound samples by fitting them all into a smaller dynamic range. This reduces the higher amplitudes while increasing the lower amplitudes. The overall aim is mainly to bring up the lower amplitudes so that the whole sound has a strong signal. It is commonly used in broadcasting. It is sometimes inappropriate in electroacoustic composition because it limits variety in the dynamic range.

- **Digital noise** occurs when there are significant changes in amplitude from one sample to the next. Sample rates have been increasing steadily in order to minimise these changes by sampling the sound more frequently per second. Digital noise can become excessive if a sound with a fairly low signal level is pushed a lot higher with a large *gain factor*. If there is background noise on a recording, this will also be amplified by gain. So it is important to work with good signal and not try to compensate by applying gain.

---

Last updated: 28 October 2021
