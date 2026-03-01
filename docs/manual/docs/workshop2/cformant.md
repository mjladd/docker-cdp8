# CDP Get and Use FORMANTS (Command Line)

## <a name="formantlist"></a>Functions which Get and Use FORMANT Data

- [FORMANTS GET](#get) - Extract evolving formant envelope from an analysis file
- [FORMANTS GETSEE](#getsee) - Get formant data from an analysis file to a pseudo-soundfile to view in VIEWSF
- [FORMANTS PUT](#put) - Impose formants in a formant data file on data in an analysis file
- [FORMANTS SEE](#see) - Convert formant data in formant data file to a pseudo soundfile to view in VIEWSF
- [FORMANTS VOCODE](#vocode) - Impose spectral envelope of one sound on another
- [Technical Discussion](#formanttechnical) - Formants and Spectral Envelopes

---

## <a name="get"></a>FORMANTS GET — extract evolving formant envelope from an analysis file

### Usage

```
formants get infile outfile -fN | -pN
```

*outfile* is a binary formant file, which can be used in other formant applications

### Parameters

- *infile* - an analysis file
- *outfile* - a binary formant file
- **-f** - extract formant envelope 'linear frequency-wise', using 1 point for every *N* equally-spaced frequency-channels
- **-p** - extract formant envelope 'linear pitchwise', using *N* equally-spaced pitch-bands per octave

### Understanding the FORMANTS GET Process

In earlier CDP systems this process was called SPECFGET. It is a straightforward utility to extract and store the evolving spectral envelope of a sound.

#### <a name="extractformants"></a>On Extracting Formants

The two **extraction options** (**-f***N* | **-p***N*) relate to resolution. Essential information: the logarithmic nature of the pitch scale means that as one goes higher, an increasingly wider band of frequencies is encompassed by the 'octave'. The octave of a tone is double that tone's frequency. Thus the octave between 220Hz and 440Hz encompasses 220Hz, while the octave between 2200Hz and 4400Hz encompasses 2200Hz.

- **-f***N* — 'linear frequency-wise' is an 'equal spacing' comprising *equal differences*.
- **-p***N* — 'linear pitchwise' is an 'equal spacing' comprising *equal ratios*.

With 'linear frequency-wise', the analysis channels define frequency bands of identical bandwidth: each band will encompass the same frequency range.

With 'linear pitchwise', the frequency bands actually get wider as one goes higher. For example, if linear pitchwise is set at *N* = 12, the octave between 220Hz and 440 Hz is divided into 12 semitones; if *N* = 3, the octave is divided into 4 minor thirds (each of slightly increasing bandwidth). In the octave between 2200Hz and 4400Hz these same divisions will consist in considerably wider bandwidths. In other words, linear pitchwise is working via ratios with the logarithmic relationship of pitches.

**Linear frequency-wise** should therefore be chosen if the main formant area of interest is in the *higher frequencies*, because there will be better resolution here when extracting the spectral envelope. But if the main interest in the sound is in the *lower frequencies*, then the **linear pitchwise** option will be suitable: the frequency bandwidths relate to the octave bandwidths. You are also able more easily to think in terms of normal musical terminology for intervals. If in doubt, you'll need to experiment.

#### Linear frequency-wise and linear pitchwise

|  | **Linear frequency-wise** | **Linear pitchwise** |
|---|---------------------------|----------------------|
| **'Equal' means:** | frequency spacing is the same *N* = **2** | octave divisor is the same *N* = **12** |
| **E.g., band is 220Hz** | 86Hz (e.g,) encompassed by every **2** channels | **12** semitones within that octave |
| **E.g., band is 2200Hz** | exactly the same bandwidth | **12** semitones within that octave |
| **Recommended application** | better resolution with higher frequencies | effective with lower frequencies |

### Musical applications

This process is essential when wanting to preserve the timbral qualities of a sound. They can be transferred to another sound, or they may be re-imposed on the original sound after it has undergone some other transformations. The re-imposing is done with [FORMANTS PUT](#put).

Also see [FORMANTS PUT](#put) and [FORMANTS GETSEE](#getsee).

[Return to formant function list](#formantlist)

---

## <a name="getsee"></a>FORMANTS GETSEE — extract formants from analfile and write as 'soundfile' for viewing

### Usage

```
formants getsee infile outsndfile -fN -pN [-s]
```

### Parameters

- *infile* - an analysis file
- *outsndfile* - a pseudo-soundfile file ready for viewing; do not resynthesize or play (an error message will be returned)
- **-f***N* - extract formant envelope linear frequency-wise, using 1 point for every *N* equally-spaced frequency-channels
- **-p***N* - extract formant envelope linear pitchwise, using *N* equally-spaced pitch-bands per octave
- **-s** - semitone bands for display (Default: equal Hz bands)

The resulting logarithmically scaled display indicates formant shapes, but NOT the absolute amplitude values.

### Understanding the FORMANTS GETSEE Process

See a detailed discussion of the two [extraction options](#extractformants) in the text for FORMANTS GET for further information on this choice.

Previously a part of SPECFSEE, this process produces a pseudo soundfile which can be viewed in the same way as a normal soundfile. In the display we see a series of spectral windows, separated by zeroes. The horizontal display represents the spectral frequencies and this may be displayed in a manner which increases regularly with frequency (linear frequency-wise) or in a manner which increases regularly with pitch (linear pitch-wise). The vertical display represents the amplitude of each spectral band. The range of amplitudes is very large indeed, and in order to display them in the soundfile format, the log of the amplitude values is used and scaled to utilise fully the range of sound-sample values (-32768 to 32767).

The display therefore indicates only the shape of the spectrum, and how that shape changes through time - no absolute amplitude values can be read off from it.

FORMANTS GETSEE uses linear interpolation in determining the spectral contour. The output pseudo-soundfile is viewed by loading it into a soundfile display program, such as CDP's VIEWSF.

**SPECIAL NOTE** — It is important to realise that output 'soundfile' is in this case a 'pseudo-soundfile'. All pseudo-soundfiles and binary files are displayed by DIRSF, and on PC are automatically given a .wav extension. It is therefore recommended that you adopt a naming convention that indicates which type of file it is, such as 'fmnt...' for the formant file produced by FORMANTS GET, 'psnd...' for the output of GETSEE, and 'bpdf...' for the binary pitch data file produced by PITCH. The different types of file all have their own 'property strings' written in the header, so the system will know what they are and prevent inappropriate use; the naming conventions are for your own convenience.

When a program produces a textfile, it is NOT given a .wav extension and will not be displayed by DIRSF. Look for textfiles in the current directory, with the command 'dir'.

Also see [FORMANTS GET](#get) and [FORMANTS PUT](#put).

[Return to formant function list](#formantlist)

---

## <a name="put"></a>FORMANTS PUT — impose spectral envelope in a formant file on spectrum in a pvoc analysis file

### Usage

```
formants put 1 infile fmntfile outfile [-i] [-llof] [-hhif] [-ggain]
formants put 2 infile fmntfile outfile [-llof] [-hhif] [-ggain]
```

- *infile* and *outfile* - PVOC analysis files
- *fmntfile* - a formant data file made with FORMANTS GET
- *outfile* - an analysis file ready for resynthesis

### MODES

- **1** - New formant envelope REPLACES the sound's own formant envelope
- **2** - New formant envelope is IMPOSED ON TOP OF the sound's own formant envelope

### Parameters

- **-l***lof* - *lof* = low frequency, below which spectrum is set to zero
- **-h***hif* - *hif* = high frequency, above which spectrum is set to zero
- **-g***gain* - *gain* = adjustment to spectrum loudness (normally < 1.0)
- **-i** - quicksearch for formants (less accurate)

### Understanding the FORMANTS PUT Process

Previously SPECFPUT, FORMANTS PUT reads a spectral trajectory as stored in *formant file* and imposes it on *infile*. In many sounds, the amplitude peaks are focused around certain frequencies; these are called formant areas, and are a key factor in creating the timbral colouration of a sound. These formants may themselves change over time, forming a spectral trajectory. The spectral trajectory is extracted (prior to running FORMANTS PUT) with [FORMANTS GET](#get). This trajectory (i.e., spectral envelope) is the rising and falling amplitude pattern of the partials (frequencies).

When this pattern is imposed on another file, the precise spectral envelope changes — and colourations — are introduced into the receiving file.

### Musical applications

The presence of the spectral trajectory data in the *fmntfile* makes it possible to impose the same data on more than one file.

There are several musical purposes which can be achieved with this process:

- simply as a means to link/unify musical data
- to re-colour a sound with the timbral characteristics of another
- to give one sound the meanings associated with another sound (this applies especially to recognisable sounds from the world around us)

[Return to formant function list](#formantlist)

---

## <a name="see"></a>FORMANTS SEE — convert formant data in a binary formant data file to a 'soundfile' for viewing

### Usage

```
formants see infile outsndfile [-v]
```

### Parameters

- *infile* - a binary formant data file created with [FORMANTS GET](#get)
- *outsndfile* - a 'pseudo-soundfile' suitable for display in a graphic soundfile editor (do not resynthesize or play (an error message will be returned)
- **-v** - display data about formant-band parameters

### Understanding the FORMANTS SEE Process

The resulting logarithmically scaled display indicates formant shapes, but *not* the absolute amplitude values. This process used to form a part of the CDP program SPECFSEE. In the display we see a series of spectral windows, separated by zeroes. The horizontal display represents the spectral frequencies and this may be displayed in a manner which increases regularly with frequency (if linear frequency-wise was selected when using FORMANTS GET) or in a manner which increases regularly with pitch (if linear pitch-wise was selected).

The vertical display represents the amplitude of each spectral band. The range of amplitudes is very large indeed, and in order to display them in the soundfile format, the log of the amplitude value is used and scaled to utilise fully the range of sound-sample values (-32768 to 32767).

[Return to formant function list](#formantlist)

---

## <a name="vocode"></a>FORMANTS VOCODE — impose spectral envelope of 2<sup>nd</sup> sound onto 1<sup>st</sup> sound

### Usage

```
formants vocode infile infile2 outfile -fN | -pN [-llof] [-hhif] [-ggain]
```

### Parameters

- *infile* and *infile2* - analysis files
- *outfile* - resultant analysis file
- **-f***N* - extract formant envelope linear frequency-wise, using 1 point for every *N* equally spaced frequency channels
- **-p***N* - extract formant envelope linear pitchwise, using *N* equally-spaced pitch-bands per octave
- **-l***lof* - *lof* is low frequency, below which data is filtered out
- **-h***hif* - *hif* is high frequency, above which data is filtered out
- **-g***gain* - *gain* is the amplitude adjustment to the signal (normally < 1.0)

### Understanding the FORMANTS VOCODE Process

FORMANTS VOCODE achieves 'cross-synthesis'; it was formerly called SPECVOCO, but here has been re-written extensively.

Cross-synthesis takes the time-evolving spectral envelope of one sound and imposes it on another sound. For example, vocal shouts will have a strong spectral profile. If this is imposed onto the sound of running water, we should hear the water shout.

Strictly speaking, this is not 'morphing', which is more of a gradual transition from one sound to another. But in a broader sense, it certainly is the reshaping of one sound with the characteristics of another.

### Musical applications

The difference between frequency-wise and pitch-wise could be important. As discussed in [FORMANTS GET](#extractformants) above, linear frequency-wise divides into absolutely equal frequency bands, while linear pitchwise divides octaves (of varying frequency widths) into equal-ratio bands, but these bands will vary in size from octave to octave, wider as one goes higher. It was recommended above to use frequency-wise if the sound is concentrated in the higher frequency areas (to get better resolution), and pitchwise for lower frequency areas. Experimentation is still the name of the game.

Frequencies below *lof* and above *hif* are simply ignored when the new spectrum is reconstructed.

*Gain* is not predictable because it depends on the level of the signals you put in, their relative levels, and the way those levels are distributed over the spectrum. A process that boosts level a great deal at the low end of the spectrum can cause distortion. Try it with no gain first.

[Return to formant function list](#formantlist)

---

## <a name="formanttechnical"></a>Technical Discussion of formants and spectral envelopes

Cover things like formants, spectral envelope (comparing with the amplitude envelope), resolution, frequency bands, more on linear frequency-wise and linear pitchwise ...technical issues dealing with using formants, leaving the definitions of these terms to the glossary. Maybe a diagram illustrating freqwise & pitchwise and more detail on assessing what needs to be done with different types of sound.

[Return to formant function list](#formantlist)

---

*Last Updated: 22 February 2000*
*All observations & ideas for improvement appreciated*
*Documentation: Archer Endrich*
*Composers Desktop Project*
*© Copyright 1998-2000 CDP*
