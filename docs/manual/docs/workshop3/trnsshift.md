![](images/cdpcircs.jpg)

# Guide to Transposition & Frequency Shifting with CDP {#guide-to-transposition-frequency-shifting-with-cdp}

## \~ CDP Workshop 3 by Archer Endrich \~ {#cdp-workshop-3-by-archer-endrich}


+-----------------------------------------------------------------------------------------------+
| **Preparing to Use the Workshop**\                                                            |
| \                                                                                             |
| [**Objectives**](#PURPOSE)   \|   [**How to use the Workshop**](#OPERATION)   \|   [**List of |
| Files Supplied**](#FILELIST)   \|   [**Setup & System Test**](#SETUP)                         |
+================================+=================================+============================+
| [](#BGROUND)                   | [](#LISTENING)                  | [](#EXERCISES)             |
|                                |                                 |                            |
| ### I -- Background            | ### II -- Comparative Listening | ### III -- Exercises       |
+--------------------------------+---------------------------------+----------------------------+
| [**Key Terms**](#KEYTERMS)     | [**1. Simple to                 | [**Time-Domain:            |
|                                | Complex**](#SEQUENCE)           | Time-varying               |
|                                |                                 | Transposition**](#TDSPEED) |
+--------------------------------+---------------------------------+----------------------------+
| [**Formant                     | [**2. Simple                    | [**Time-Domain:            |
| Extraction**](#FORMANTX)       | Transposition**](#SIMTRANS)     | 'Waveshape'              |
|                                |                                 | Distortion**](#TDDSTORT)   |
+--------------------------------+---------------------------------+----------------------------+
| [**Overview of                 | [**3. Simple .frq               | [**Inharmonic              |
| Types**](#OTYPES)              | Input**](#SIMFRQIN)             | Shifts**](#INHARMONIC)     |
+--------------------------------+---------------------------------+----------------------------+
| [**Inputs                      | [**4. Spectral                  | [**With & Without          |
| &Outputs**](#INANDOUT)         | Movement**](#SPECMOVE)          | Preserving                 |
|                                |                                 | Formants**](#FORMANTS)     |
+--------------------------------+---------------------------------+----------------------------+
| [**Batch/Presets**](#BATFILES) | [**5. Processed .frq            | [**Sequence of             |
|                                | Input**](#PROCESSFRQ)           | Operations**](#SEQUENCE)   |
+--------------------------------+---------------------------------+----------------------------+
| [**Additional                  | [**6. Combinations**](#COMBOS)  | [**Combine .frq            |
| Info**](#XTRAINFO)             |                                 | files**](#COMBINE)         |
+--------------------------------+---------------------------------+----------------------------+

: []{#T&FSCONTENTS}
Contents

> **My thanks to Trevor Wishart and Richard Dobson for assistance with
> technical phraseology and with understanding the operation of the
> programs.**

------------------------------------------------------------------------

## Preparing to Use the Workshop

### Also see [Readtsws.txt](Readtsws.txt)

> > []{#PURPOSE}
> >
> > ### The Transposition & Shifting Workshop aims to achieve three things:
> >
> > 1.  to shed some light on a complex but powerful part of the CDP
> >     System by providing groups of examples which can be listened to
> >     and compared.
> >
> >
> > 2.  to illustrate a number of advanced processing functions, which
> >     can be used to create original, highly transformed sounds
> >
> >
> > 3.  to show how to handle the (batching) mechanisms for extracting
> >     (or using pre-designed) shapes and applying them to the same or
> >     another sound. These mechanisms are useful tools for a composer
> >     who likes to integrate material by working with design shapes in
> >     a variety of guises.
>
> > []{#OPERATION}
> >
> > ### The Workshop is designed for use with generic batch files, so a DOS (PC) or Terminal (MAC) setup needs to be in place along with this HTML document
> >
> > - **You will need at least 200Mb of free space to run all the
> >   examples and store them in your Workshop folder, where they can be
> >   played from this HTML document.** You can reduce this requirement
> >   by running one batch file at a time and then deleting its outputs
> >   when finished studying that section.
> >
> >
> > - The batch files will create the examples used in the Workshop.
> >   There are PC and MAC versions of these files. Each has a
> >   corresponding deletions batch file to remove the output soundfiles
> >   in preparation for another run of the creations batch file -- the
> >   CDP System will not allow soundfiles of the same name to be
> >   overwritten unless the environment variable 'set
> >   CDP_OVERWRITE_FILE' is set, *which is not recommended until you
> >   are a very experienced user*. You can edit the parameter values
> >   between runs to extend your explorations in ways particularly
> >   relevant to your own musical objectives. Note that there are PC
> >   and MAC versions of the main HTML document, *Trnsshft*, the PC
> >   version providing links to .wav files and the MAC version
> >   providing links to .wav files.
> >
> >
> > - These batch files have been revised to add 'pause' and 'play'
> >   commands to stop the screen so that you have time to see what it
> >   is doing and then listen to each result in turn. These can be
> >   disabled by adding 'rem' (PC) or '#' (MAC-UNIX) to
> >   the beginning of these lines.
> >
> >
> > - One could create *SoundShaper* presets or *Sound Loom* patches to
> >   make the examples. It is in fact more cumbersome to do this when
> >   so many examples are involved, but I (or you) could create presets
> >   and patches data files should you wish to work that way.
> >
> >
> > - Select/create a folder in which to place all the Workshop
> >   materials, e.g., d:\\T-Sworkshop. These materials include this
> >   HTML document (*trnsshft.htm*), the batch files, the breakpoint
> >   files, and 4 source soundfiles. The soundfiles have been reduced
> >   in amplitude because some of the complex combinations of pitch
> >   data files can easily produce amplitude overload.
> >
> >
> > - A path to the CDP .exe programs needs to be set in DOS, either in
> >   *autoexec.sh* or in the folder where you have placed the workshop,
> >   or on the Environment Variables page, or with Environment.plist,
> >   depending on your operating system.
> >
> >
> > - You will need 4 source soundfiles, preferably each of them 5 sec
> >   in duration so that the breakpoint files will work as intended
> >   with each of the sounds. These sounds should include, as noted in
> >   the batch files, a vocal, drumming, pitched instrumental (single)
> >   tone, and a bell or gong.
> >
> >
> > - To run the batch files with your own sounds, you just need to
> >   alter the input soundfile name(s) in the line(s) that copy the
> >   soundfile to the generic name used in the batch file. E.g., change
> >   `copysfx vocalsnd tsw1` to `copysfx `**`mysound`**` tsw1`.
> >
> >
> > - Although you can use this Workshop in the relatively early stages
> >   of learning about the potential of the CDP System, I would suggest
> >   that you become comfortable with the overall operation of the CDP
> >   System before using it. These functions, with their various
> >   (combinations of) inputs, can be very confusing.
> >
> >
> > - All of the outputs can be auditioned from this HTML document. It
> >   might be practical, however, to run one batch file at a time,
> >   study those examples, and then run the batch file to delete those
> >   ouputs. It depends how much hard disk space you have available and
> >   want to tie up with this material.
>
> > []{#FILELIST}
> >
> > ### List of files used in the Workshop
> >
> > - *trsnshft.htm* -- this HTML document
> >
> >
> > - *tswtest.sh* -- brief batchfile with which to test your setup. It
> >   creates, plays and deletes a soundfile transposed down an octave.
> >   Deletions: *tswtdels.sh*
> >
> >
> > - *tswpfopt.sh* -- exploring what happens when different values are
> >   given to the **-p \| -f** formant extraction method option. You
> >   need to run this with different types of input to observe how the
> >   results can differ. Deletions: *tswpfdel.sh*
> >
> >
> > - *tsw1.sh* -- this explores a sequence of operations from the
> >   relatively simple to more complex possibilities. Deletions:
> >   *tsw1dels.sh*. Other files:
> >   - *glideup.brk* transposition breakpoint file (semitones)
> >   - *harmon.txt* file containing a list of 'harmonics' and
> >     amplitude values
> >
> >
> > - *tsw2.sh* -- simple transpositions. Deletions: *tsw2dels.sh*.
> >   Other files:
> >   - *tsw2harm.txt* containing 'harmonics' and amplitude values
> >   - *tsw2brk.brk* transposition breakpoint file (semitones)
> >
> >
> > - *tsw3.sh* -- simple **.frq / .trn** as input file. Deletions:
> >   *tsw3dels.sh*
> >
> >
> > - *tsw4.sh* -- shifting (spectral movement). Deletions:
> >   *tsw4dels.sh*. Other files:
> >   - *frqshift.brk* time-varying frequency shifts
> >   - *frqdiv.brk* time-varying frequency split point
> >   - *frqlow.brk* time-varying low frequency split point
> >   - *frqhigh.brk* time-varying high frequency split point
> >   - *tsw4bits.txt* data file for setting the 4-bits used by HILITE
> >     BAND
> >
> >
> > - *tsw5.sh* -- various **.frq** outputs are used as inputs to other
> >   REPITCH processing functions. These functions are 'chained',
> >   i.e., each output is used as the next input, so that the entire
> >   processing sequence is present in the last file. If any process
> >   fails in this kind of batch file, the chain is broken and you have
> >   to fix the problem before you can complete the processing
> >   sequence. Deletions: *tsw5dels.sh*. Other files:
> >   - *tsw5qset.txt* quantisation file containing MIDI pitch values
> >
> >
> > - *tsw6cp.sh* -- .frq & .trn combinations. Deletions: *tsw6dels.sh*
>
> > []{#SETUP}
> >
> > ### Setting up Your Working Environment
> >
> > Here is what you need to do to check out the environment:
> >
> > - Ensure that a DOS (PC) or Terminal (MAC) environment for the CDP
> >   System is properly set up (by whatever method is appropriate for
> >   your operating system -- see the *Installation Notes* that came
> >   with your CDP System.
> >   - Check that the path to the CDP programs is set.
> >   - Check that the CDP soundfile extension is set.
> >   - Check that the CDP memory allocation is set to at least 300.
> >     Possibly 1000 might be safer.
> >   - Are you able to access the HTML reference documentation? You
> >     will be looking mainly at this document, and also accessing the
> >     HTML reference documentation at times for additional
> >     information.
> >
> >
> >
> > - Plan out which folder you will use and place all the Workshop
> >   files in it, along with the source sounds you will use. If you are
> >   using a 'special batch file in your working directory', this is
> >   where it needs to be so that the path is active from within this
> >   DOS window. The PC *autoexec.sh* or Environment Variables via the
> >   Control Panel, and the MAC Environment.plist mechanisms are
> >   generic for your whole system.
> >
> >
> > - **Ensure that you run the correct set of batch files for your
> >   computer: PC or MAC. Two different sets are provided. The MAC set
> >   has a .sh extension, or no extension.**
> >
> >
> > - Edit *tswtest.sh* so that one of your soundfiles is copied to the
> >   generic name *tswtest.wav*.
> >
> >
> > - Now run the *tswtest.sh* batch file, which transposes a [single
> >   soundfile](tswd12.wav) down an octave -- just type `tswtest` at
> >   the DOS prompt or on the Terminal and press return.
> >
> >
> > - Run *tswtdels.sh* to delete the test soundfile.
> >
> >
> > - Edit the breakpoint file to create some other transposition effect
> >   and re-run the batch file.
> >
> >
> > - Run *tswtdels.sh* again when ready to finish.
> >
> > You should now be ready to run the other batch files and work your
> > way through the various examples in the Transposition & Shifting
> > Workshop.

------------------------------------------------------------------------

## [I -- Background and Reference Information]{#BGROUND}

> ### [Questions & Answers about Key Terms]{#KEYTERMS}
>
> \[Also see the **Glossary of Technical Terms** accessible via the Main
> CDP Documentation Index (*ccdpndex.htm*). This glossary is more recent
> text than the descriptions below and may be expressed in more
> technically accurate language.\]
>
> > ### Q. What is a 'partial'?
> >
> > > **A.** A partial is a significant frequency component of a sound.
> > > Partial content and their amplitude strength determines the
> > > timbral character of the sound. The presence and strength of the
> > > partials in a sound change over time. The Fast Fourier Transform
> > > (FFT) analysis searches for partials within a series of
> > > (user-defined) frequency bandwidths referred to as 'channels'.
> > > How it conducts this search and what it actually picks up differs
> > > in various software packages and with the nature of the sound
> > > being analyzed.
> >
> > ### Q. What is the difference between a 'harmonic' and an 'inharmonic' relationship among partials?
> >
> > > **A.** A harmonic partial is an integer multiple of a given
> > > 'fundamental'. Sometimes this results in divisions within given
> > > node points which lock the waveforms of the higher frequencies
> > > firmly within the lower ones, so that all phases begin at the same
> > > zero point. This binds them together into one focused pitch.
> > > Frequently, however, the phases overlap, which may alter the
> > > colouration, but the fact that the partials are in
> > > integer-relationship predominates, giving a pitch-effect focused
> > > on the fundamental. Weighting the higher partials with more
> > > amplitude than the fundamental also introduces colourations which
> > > mitigate the focused-pitch effect.
> > >
> > > Inharmonic partials have non-integer and additive relationships.
> > > Thus one hears multiple frequency strands, as with bells and
> > > gongs.
> > >
> > > Richard Dobson has provided a more detailed technical discussion
> > > of [**phase, harmonicity & inharmonicity**](#PHASEETC), which is
> > > included below.
> >
> > ### Q. What does 'maintaining harmonicity' mean?
> >
> > > **A.** 'Maintaining harmonicity means keeping the harmonic
> > > relationship of the partials, i.e., using a multiplicative (rather
> > > than additive) process. The partials of a sound may or may not
> > > actually be in harmonic relationship (integer multiples of a
> > > fundamental) -- this depends on the sound itself. 'Pitched'
> > > sounds have mostly integer-multiple related partials -- but phase
> > > relationships and other inharmonic factors also contribute to the
> > > colouration of the sound.
> > >
> > > Another way to put this is that the **proportional spacing** (in
> > > Hz) between the partials is maintained. For example 880 and 440
> > > are in a 2:1 proportion; when both are doubled (i.e., multiplied
> > > by 2), the frequencies are 1760 and 880, which is also a 2:1
> > > proportion although the spacing, the distance in Hz) between them
> > > is now 880 Hz instead of 440 Hz.
> > >
> > > What the ear mainly hears is that the pitch-coherence, the mutual
> > > reinforcement of the partials, remains in place.
> >
> > ### Q. What is a 'transposition'?
> >
> > > **A.** A transposition is a change of frequency, whether higher or
> > > lower. In the time domain, this is done by removing or duplicating
> > > samples, which makes the sound faster when higher (removing), or
> > > slower when lower (duplicating). In the frequency domain, a
> > > transposition, properly speaking, is a change of frequency which
> > > retains the relationship of the partials, i.e., their proportional
> > > spacing along the (logarithmic) frequency scale. It is therefore
> > > the result of a multiplicative mathematical operation. This
> > > retains many of the features of the sound.
> > >
> > > Note the difference here between 'pitch' and 'frequency'.
> > > Pitch is a musical construct which is presented to the performing
> > > musician as equally spaced 'notes'. For example, the octaves on
> > > a keyboard are all equally far apart. However, the distance
> > > between the frequencies increases on a logarithmic scale, i.e., as
> > > a geometric progression (double the previous value): 55 - 110 -
> > > 220 - 440 - 880 etc. To transpose these pitches up an octave, we
> > > multiply by 2; thus they become 110 - 220 - 440 - 880 - 1760, etc.
> > > and the relationship between them stays the same (a doubling: all
> > > of them are 1:2).
> >
> > ### Q. What is a 'shift'?
> >
> > > **A.** A shift is also a change of frequency, whether higher or
> > > lower, but as an additive operation, with the result that the
> > > existing harmonicity of the sound is not preserved. This changes
> > > the proportional spacing of the partials, though this effect can
> > > be mitigated by retaining/preserving the formants of the sound.
> > > Using the example frequencies above and adding, say 47, to each of
> > > them, we get 102 - 157 - 267 - 487 - 927 etc., or, adding 1000, we
> > > get 1055 - 1110 - 1220 - 1440 - 1880 etc. The relationship of the
> > > last two (1440 and 1880) is 1:1.306, so we can see how a
> > > previously harmonic octave relationship has become inharmonic, and
> > > how the shift process has resulted in a squeezing together of the
> > > frequencies. This is heard as beat patterns and timbral colour.
> > >
> > > You might also be able to restore harmonic relationships by
> > > shifting back again. There can also be shifts between one
> > > inharmonic situation and another.
> > >
> > > What the ear hears is that the partials separate more and can be
> > > heard more independently of one another. This results in a
> > > 'richer', more complex and often 'brighter' sound.
> >
> > ### Q. What is a 'spectral envelope'?
> >
> > > **A.** The spectral envelope is the contour formed connecting up
> > > the amplitudes of the partials within each analysis window, and
> > > from window to window. Remember that analysis data has two items
> > > for every digital sample of the original sound: a frequency
> > > component and an amplitude component. The spectral envelope is a
> > > way of imaging the amplitude components of the different frequency
> > > components. The key to understanding this description is that the
> > > resulting envelope is *highly dependent on which frequencies are
> > > present* -- we are dealing really with timbral components, not
> > > simply with an sample-amplitude envelope as in the time domain.
> > > The sonogram displays the frequency components on a vertical
> > > scale; where there are many partials (indicating formant regions),
> > > the plot thickens and looks darker.
> >
> > ### Q. What is a 'formant'?
> >
> > > **A.** A formant is in principle the same as the spectral
> > > envelope, meaning that it is a way of imaging the amplitude
> > > component. However, its meaning is a little more specific in that
> > > a formant refers to a frequency region of the sound which
> > > comprises an area of peak amplitude. These regions often result
> > > from resonances in the sound source (such as the head cavities of
> > > vocal sounds). The pattern of these peaks can stay the same as the
> > > frequency level changes, such as the sound of the vowel 'ah'
> > > being sung on the pitches of a rising scale: i.e., the pitches are
> > > changing, but the vowel pattern is staying the same.
> >
> > ### Q. What's the difference between formants being 'preserved' or 'not preserved'?
> >
> > > **A.** 'Preserving formants' means that resonant frequency
> > > regions are kept at the same frequency level, even when the
> > > overall sound is being moved upwards or downwards. These affect
> > > tonal colouration and have particular relevance to the sounds of
> > > the vowels and most effect on vocal sounds.
> > >
> > > Formants are most important in vocal sounds, because the voice can
> > > change formants (it does so all the time during speech). However,
> > > they are important as STATIC entities for instruments. A violin
> > > will have a formant envelope defined mainly by its soundbox, etc.
> > >
> > > **Equivalent phrases:** *'preserving the spectral envelope',
> > > 'preserving the formants', 'preserving the formant
> > > characteristics of the original', 'retains original spectral
> > > envelope', 'without formant shift'*.
> > >
> > > On the other hand, when 'formants are not preserved', their
> > > location in the range of frequencies alters, moving up or down
> > > along with the rest of the sound's frequencies. A similar shift
> > > of the spectral envelope happens with tape-transposition-type
> > > (time domain) transposition -- that's why voices go 'Mickey
> > > Mouse': it's a side effect of both processes.
> > >
> > > **Equivalent phrase:** *'the spectral envelope also moves'*.
> >
> > ### Q. What happens to the partials when a spectrum is stretched?
> >
> > > **A.** In this case the vertical frequency spacing of the partials
> > > is **expanded** gradually from 1 (no change) to
> > > *maximum_stretch_factor*. Thus a series of gradually increasing
> > > multipliers are applied to the partials. These multipliers may
> > > progress in a linear manner (equal intervals) or exponentially,
> > > larger to smaller or smaller to larger. These differing
> > > multipliers (with a decimal portion) reconfigure the relationship
> > > between the partials in an inharmonic way.
> >
> > ### Q. What happens to the partials when an analysis file is time-stretched?
> >
> > > **A.** STRETCH TIME is given a *stretch_factor*. Suppose it is 3.
> > > The first result is that the output analysis file will have 3
> > > windows for every 1 of the original: thus windows 1-2 in the
> > > original are expanded to 1-1-1, 2-2-2, and the data in window 2 is
> > > now no longer adjacent to window 1, but 3 windows away. The new,
> > > intervening windows are neither empty nor duplicates; rather, they
> > > are filled by interpolating between windows 1 and 2. This process
> > > of interpolation results in a smooth sonic movement through the
> > > expanded set of windws.
>
> [**Return**](#T&FSCONTENTS) to Contents
>
> ### [Formant Extraction:]{#FORMANTX} the -p or -f option (EXPLORE TOOL: Batch file: *tswpfopt.sh*)
>
> > When formants are to preserved, they must first be identified. This
> > is done with REPITCH GETPITCH by extracting the pitch trace of a
> > sound and saving it to a binary **.frq** or breakpoint **.brk**
> > file. A binary file is easiest to use for further processing and for
> > using combinations. It is useful to look at (and audition) the pitch
> > trace to see what the software has made of the sound presented to
> > it.
> >
> > The two formant extraction options are **-p** (linear pitchwise) and
> > **-f** (linear frequencywise): i.e., equal proportions (pitch
> > logarithmic scale) or equal frequency bands. The
> > [meaning](cformant.htm#extractformants) of these two methods is
> > discussed in the reference documentation. (Put the CDP HTML
> > reference document *cformant.htm* in the current directory to
> > activate this link.) What we would like to do in this Workshop is to
> > *audition* the difference between the two methods using a variety of
> > values.
> >
> > The table below provides a starting point. These examples remain by
> > definition as close as possible to the original sound by maintaining
> > harmonicity and preserving formants (REPITCH TRANSPOSEF). Note that
> > in making 'equal divisions per octave' with the linear pitchwise
> > method, the actual number of frequencies contained in each 'equal
> > division' actually increases. The equal frequency bands, however,
> > each contain exactly the same span of frequencies.
> >
> > On the whole, the **-p** and **-f** results are fairly similar. The
> > **-p** linear pitchwise sequence may perhaps become *less* clear as
> > the values increase, whereas the **-f** linear frequencywise
> > sequence seems to become *more* precise. The results are reasonably
> > true to the original with vocal sounds, with the higher **-f**
> > values the clearest. With higher pitched instrumental sounds, the
> > results are reasonably clear if firmly pitched, but the more
> > variable pitch content of gong-like sounds makes the results
> > somewhat unpredictable, but not necessarily less interesting. With
> > the higher pitched gong-like sounds etc., the higher **-p** values
> > get notably 'worse' while the higher **-f** values get clearer and
> > clearer.
> >
> > However, you must experiment with your own sources, so the batch
> > file used for these examples can be a useful resource.
> >
> >
> >   Soundfile                  -p value (max: 12)        Soundfile                  -f value
> >   -------------------------- ------------------------- -------------------------- --------------------------------------------------------
> >   [fexp4.wav](fexp4.wav)     4 = divisions per 8^ve^   [fexf4.wav](fexf4.wav)     4 = number of analysis channels in one frequency band
> >   [fexp6.wav](fexp6.wav)     6 = divisions per 8^ve^   [fexf6.wav](fexf6.wav)     6 = number of analysis channels in one frequency band
> >   [fexp9.wav](fexp9.wav)     9 = divisions per 8^ve^   [fexf9.wav](fexf9.wav)     9 = number of analysis channels in one frequency band
> >   [fexp12.wav](fexp12.wav)   12 divisions per 8^ve^    [fexf12.wav](fexf12.wav)   12 = number of analysis channels in one frequency band
> >
> >   : -p and -f formant extraction (source is [tswpt.wav](tswpt.wav))
> >
> > :::
>
> ### [Overview of Types]{#OTYPES}
>
> > There are 3 types plus the possibility of intermediate processing
> > and combinations:
> >
> > - MAINTAIN HARMONICITY, PRESERVE (DO NOT MOVE) THE FORMANTS
> >   \[**mHpF**\]
> >   - REPITCH TRANSPOSEF: transpose, preserving formants
> >   - PITCH OCTMOVE: harmonic series integer transpositions
> >
> >
> >
> > - MAINTAIN HARMONICITY, MOVE (DO NOT PRESERVE) THE FORMANTS
> >   \[**mHmvF**\]
> >   - MODIFY SPEED: time domain transposition, acceleration and
> >     vibrato ('Mickey-Mousing' effect)
> >   - DISTORT MULTIPLY: raise pitch and speed by increasing the rate
> >     of waveshapes (time domain)
> >   - DISTORT HARMONIC: accumulate waveshapes (time domain)
> >   - PITCH TRANSP: transpose (part of) spectrum by a constant
> >   - REPITCH TRANSPOSE: transpose without preserving formants
> >
> >
> >
> > - DO NOT MAINTAIN HARMONICITY, MOVE (DO NOT PRESERVE) THE FORMANTS
> >   \[**noHmvF**\]
> >   - STRANGE SHIFT: linear (inharmonic) shift of the spectrum
> >
> >
> >
> > - INTERMEDIATE PROCESSING OF EXTRACTED PITCH TRACE (**.frq** binary
> >   pitch data file \[abbreviated below as 'bpdf'\])
> >   - REPITCH APPROX: approximate copy of bpdf
> >   - REPITCH EXAG: exaggerate pitch contour of bpdf
> >   - REPITCH INVERT: invert pitch contour (not the spectrum) of a
> >     bpdf
> >   - REPITCH CUT: utility to cut and keep a segment of a bpdf
> >   - REPITCH FIX: amend the pitch data in a bpdf in various ways
> >   - REPITCH PCHSHIFT: transpose a bpdf by fixed (fractional)
> >     semitones
> >   - REPITCH QUANTISE: quantise the pitches in a bpdf
> >   - REPITCH RANDOMISE: randomise the pitch line in a bpdf
> >   - REPITCH SMOOTH: smooth the pitch contour in a bpdf
> >   - REPITCH VIBRATO: add vibrato to pitch in a bpdf
> >
> >
> >
> > - COMBINATIONS OF **.frq** and **.trn** FILES
> >   - REPITCH COMBINE/B: various combinations of **.frq** and **.trn**
> >     files, from one or two sources. (The results are moved to
> >     **.ana** files with REPITCH TRANSPOSE/F.)
>
> ### [Summary of Inputs and Outputs]{#INANDOUT}
>
> > The input and output files for all the functions listed in the
> > previous sections vary quite a bit, so this can be one of the most
> > confusing parts of the CDP System. The table below lists these
> > inputs and outputs (omitting the other parameters) to provide a
> > simple visual summary for easy reference. You can print out this
> > chart for easy reference: **Rpchchrt.htm**.
> >
> > - **.ana** = analysis file
> > - **.frq** = binary pitch data file
> > - **.trn** = binary transposition data file
>
>
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | CDP         | Mode      | Input 1   | Input 2 OR      | Output    | Key                     |
> | FUNCTION    |           |           | Output          |           | Parameters/\[Comments\] |
> +=============+===========+===========+=================+===========+=========================+
> | REPITCH     | 1         | `in.ana`  | `outlisten.ana` | `out.frq` | \[binary pitch data     |
> | GETPITCH\   |           |           |                 |           | file\]                  |
> | (get pitch  |           |           |                 |           |                         |
> | trace)      |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.ana`  | `outlisten.ana` | `out.brk` | \[breakpoint (text)     |
> |             |           |           |                 |           | file\]                  |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1--3      | `in.ana`  | \               | `out.ana` | *transpose* (ratio,     |
> | TRANSPOSE\  |           |           |                 |           | 8^ves^, semitones)      |
> | (M4: route  |           |           |                 |           |                         |
> | for .trn to |           |           |                 |           |                         |
> | .ana)       |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 4         | `in.ana`  | `in.trn`        | `out.ana` | \[ready to synthesise\] |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1--3      | `in.ana`  | \               | `out.ana` | `-p` or `-f` (formant   |
> | TRANSPOSEF\ |           |           |                 |           | extraction)\            |
> | (M4: route  |           |           |                 |           | *transpose* (ratio,     |
> | for .trn to |           |           |                 |           | 8^ves^, semitones)      |
> | .ana)       |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 4         | `in.ana`  | `in.trn`        | `out.ana` | `-p` or `-f` (formant   |
> |             |           |           |                 |           | extraction)\            |
> |             |           |           |                 |           | \[ready to synthesise\] |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in1.frq` | `in2.frq`       | `out.trn` | \[can be input to       |
> | COMBINE\    |           |           |                 |           | REPITCH TRANSPOSE/F\]   |
> | NB: creates |           |           |                 |           |                         |
> | .trn        |           |           |                 |           |                         |
> | outputs     |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | `in.trn`        | `out.frq` | \[can be given to       |
> |             |           |           |                 |           | another .frq            |
> |             |           |           |                 |           | processor\]             |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 3         | `in1.trn` | `in2.trn`       | `out.trn` | \[can be input to       |
> |             |           |           |                 |           | REPITCH TRANSPOSE/F\]   |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | PITCH       | 1-3       | `in.ana`  | \               | `out.ana` | *frq_split* \[frequency |
> | TRANSP      |           |           |                 |           | split point\]           |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 4-5       | `in.ana`  | \               | `out.ana` | *frq_split* \[frequency |
> |             |           |           |                 |           | split point\]\          |
> |             |           |           |                 |           | *transpose*             |
> |             |           |           |                 |           | \[semitones\]           |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 6         | `in.ana`  | \               | `out.ana` | *frq_split* \[frequency |
> |             |           |           |                 |           | split point\]\          |
> |             |           |           |                 |           | *transpose1 transpose2* |
> |             |           |           |                 |           | \[semitones\]           |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | PITCH       | 1-2       | `in.ana`  | `in.frq`        | `out.ana` | *transposition*         |
> | OCTMOVE     |           |           |                 |           | (integer \>0:\          |
> |             |           |           |                 |           | follows harmonic        |
> |             |           |           |                 |           | series)\                |
> |             |           |           |                 |           | \[in.frq is derived     |
> |             |           |           |                 |           | from in.ana\]           |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 3         | `in.ana`  | `in.frq`        | `out.ana` | *transposition          |
> |             |           |           |                 |           | bassboost*              |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | STRANGE     | 1         | `in.ana`  | \               | `out.ana` | *frqshift* (in Hz)      |
> | SHIFT       |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2-3       | `in.ana`  | \               | `out.ana` | *frqshift frq_divide*   |
> |             |           |           |                 |           | (in Hz)                 |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 4-5       | `in.ana`  | \               | `out.ana` | *frqshift frqlo frqhi*  |
> |             |           |           |                 |           | (in Hz)                 |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in.frq`  | \               | `out.frq` | *prange trange srange*  |
> | APPROX      |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.trn` | *prange trange srange*  |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in.frq`  | \               | `out.frq` | *meanpch range*         |
> | EXAG        |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.trn` | *meanpch range*         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 3         | `in.frq`  | \               | `out.frq` | *meanpch contour*       |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 4         | `in.frq`  | \               | `out.trn` | *meanpch contour*       |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 5         | `in.frq`  | \               | `out.frq` | *meanpch range contour* |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 6         | `in.frq`  | \               | `out.trn` | *meanpch range contour* |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in.frq`  | \               | `out.frq` | *map* (file) \...       |
> | INVERT      |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.trn` | *map* (file) \...       |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH CUT | 1         | `in.frq`  | \               | `out.frq` | *starttime*             |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.frq` | *endtime*               |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 3         | `in.frq`  | \               | `out.frq` | *starttime endtime*     |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH FIX | 0         | `in.frq`  | \               | `out.frq` | \[several options\]     |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 0         | `in.frq`  | \               | `out.frq` | *transposition*         |
> | PCHSHIFT    |           |           |                 |           | (semitone constant)     |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in.frq`  | \               | `out.frq` | *q_set* (file of MIDI   |
> | QUANTISE    |           |           |                 |           | pitchvals)              |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.trn` | *q_set* (file of MIDI   |
> |             |           |           |                 |           | pitchvals)              |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in.frq`  | \               | `out.frq` | *maxinterval timestep*  |
> | RANDOMISE   |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.trn` | *maxinterval timestep*  |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in.frq`  | \               | `out.frq` | *timeframe* \...        |
> | SMOOTH      |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.trn` | *timeframe* \...        |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
> | REPITCH     | 1         | `in.frq`  | \               | `out.frq` | *vibfrq vibrange*       |
> | VIBRATO     |           |           |                 |           |                         |
> |             +-----------+-----------+-----------------+-----------+-------------------------+
> |             | 2         | `in.frq`  | \               | `out.trn` | *vibfreq vibrange*      |
> +-------------+-----------+-----------+-----------------+-----------+-------------------------+
>
> : Transposition & Shifting Inputs and Outputs Chart {border="1"
> cellpadding="3"}
> :::
>
> ### [Extra Information on Relevant Issues]{#XTRAINFO}
>
> 1.  ### Information about the frequency/partial/formant profile of a sound
>
>     See *hreports.sh* for a convenient way to get most of these
>     reports for a given sound. Just edit the batch file to enter the
>     name of your input analysis file. The sonogram display can be made
>     with most soundfile editors.
>
>     - FORMANTS SEE -- 'see' the shape of the formants by converting
>       a binary formant data file to a pseudo-soundfile. You can see
>       the shape, but frequency and time locations are not clearly
>       marked.
>     - FORMANTS GETSEE -- create a binary formant data file and convert
>       to a pseudo-soundfile for viewing, with the same result as
>       FORMANTS SEE.
>     - SPECINFO PEAK -- reports on the time location of energy bands
>     - SPECINFO REPORT Mode 1 -- time + formant center frequencies in
>       ascending frequency order
>     - SPECINFO REPORT Mode 2 -- time + formant center frequencies in
>       ascending loudness order
>     - SPECINFO REPORT Mode 3 -- formant center frequencies in
>       ascending frequency order
>     - SPECINFO REPORT Mode 4 -- formant center frequencies in
>       ascending loudness order
>     - SONOGRAM DISPLAY -- e.g., via SNDAN
>
>
>
> 2.  ### Considering split-point options:
>
>     - At the bottom of a sound, to shift virtually the whole spectrum
>       upwards
>     - At the top of the sound, to shift virtually the whole spectrum
>       downwards
>     - At some point towards the middle of the sound, to shift:
>       i.  up and down from that point
>       ii. upwards from that point, leaving the lower portion unchanged
>       iii. downwards from that point, leaving the upper portion
>            unchanged
>     - Within a defined range of frequencies
>     - Outside of a defined range of frequencies
>
>
>
> 3.  ### [**Richard Dobson on Phase, Harmonicity & Inharmonicity**]{#PHASEETC}
>
>     \"There are two main points to be made here:
>     **1. Harmonic partials can have any phase relationship**. Even in
>     a physical instrument, the phase of one component can, especially,
>     be opposite to that of another. But the colouration effects of a
>     soundbox, bridge, conical tube, tone-holes etc can result in a
>     mixture of phases. We will still hear this as a harmonic tone, as
>     the only technical requirement of that is that all components are
>     an \~integral\~ multiple of a single 'fundamental' frequency -
>     i.e x 2, x 3, x 4, x 7,etc.
>
>     \"Even if a sound starts off with all partials in identical phase,
>     as soon as that sound is passed through a filter (especially a
>     resonant one), the phases of some partials will be shifted more
>     than others (hence, you can have both 'linear-phase' and
>     'non-linear-phase' filters), but the harmonicity itself will not
>     be altered -- filters change phase, but not pitch. The ear is
>     largely insensitive to phase. The classical demonstration of this
>     is the example of changing the phases of harmonics of a square
>     wave; the waveform looks nothing like a square wave, but the sound
>     is the same to the ear.
>
>     **\"2. You can have inharmonic partials with are related
>     multiplicatively**, that is, if the ratio between partials and the
>     'fundamental' is not integral but something else, such as 1.618.
>     Adding a constant shift to each partial's frequency will induce
>     inharmonic relationships, but multiplying by a non-integral factor
>     will also do that. This is the principle behind FM synthesis --
>     integral C:M ratios \[Carrier:Modulator\] create harmonic
>     partials, non-integral ones produce inharmonic sounds.
>
>     \"There is also a certain latitude in the harmonic relationships
>     between partials before we really hear them as inharmonic. the
>     classic example is the piano, which we think of as harmonic, but
>     in fact is technically inharmonic, as the stiffness of the strings
>     is such that high partials are a little sharp relative to the
>     fundamental. This is the source of the so-called 'stretch'
>     tuning, in which piano tuners tune octaves a little wide, so that
>     the harmonics actually coincide better. So individual partials of
>     a tone can vary in frequency a small amount (up to the
>     'just-noticeable-difference' (JND) as measured by acousticians),
>     without the tone \~sounding\~ inharmonic.
>
>     \"There are various useful graphical ways of demonstrating
>     harmonicity with variable phase. For example, there are GEN
>     functions in *Csound* thatallow you to specify the phase of each
>     partial, so you can make various identical-sounding wavetables
>     with different internal phase relationships. The definition of
>     'harmonic' then becomes \"able to fit inside a wavetable without
>     a disconinutity at the ends\" -- so the wavetable can be looped
>     without glitches. The technical term for that is of course
>     'periodicity' -- the partials share a single common period
>     (string length, for example).
>
>     \"A final complication in all this is that 'musical' harmonicity
>     requires a predominance of ratios of small numbers (1:2,2:3,3:4,
>     etc). So the fundamental must also be in a 'musical' range,
>     directly audible, for the whole sonic image to be perceived as
>     'harmonic'. If you take a very low pitched harmonic sound, and
>     cut out all the low harmonics (and especially if you then remove
>     some of the higher ones too), what is left will very likely sound
>     inharmonic, as the intervals between the partials are remote from
>     the musical ones we expect. This is a known compositional
>     technique in computer music, Jonathan Harvey does something like
>     it in *Ritual Melodies*, and it is relevant to *Mortuos Plango* as
>     well.
>
>     \"The technical people argue incessantly about the meaning of
>     'pitch shift' *versus* 'frequency shift'. Since 'pitch' is a
>     cognitive term rather than a physical/analytical one, DSP experts
>     argue that 'pitch shift' preserves ratios between partials, and
>     is therefore transposition by definition, while 'frequency
>     shift' would be reserved for an additive shift that changes the
>     ratios.\"
>
> [**Return**](#T&FSCONTENTS) to Contents

## [II -- Listen & Compare]{#LISTENING}

> ### [1. An Overall Arrangement]{#SEQUENCE} from Simple to Complex Processes (EXPLORE TOOL: Batch File: *tsw1.sh*, Deletions File: *tsw1dels.sh*)
>
> This first section is meant to provide an overview of the range of
> transposition and shifting functions provided by the CDP System. The
> exampes are arranged in a sequence, a sequence that moves, generally,
> but not entirely, from the most similar to the most dissimilar, from
> the simple to the (potentially) more complex. Various parts of this
> range of possibilities are explored more thoroughly in the other
> sections of 'Comparative Listening'.
>
> > The source sound is copied to the generic name [tsw1.wav](tsw1.wav).
> > A vocal sound really needs to be used as an input in order to hear
> > the difference regarding the formants. The second sound used for the
> > combinations is copied to the generic name
> > [tsw1-2nd.wav](tsw1-2nd.wav).
> >
> > By copying different source sounds to these names, you can re-run
> > the batch file with different inputs. Please examine the *tsw1.sh*
> > file for a detailed look at the inputs, outputs and parameters. You
> > can run the deletions batch file *tsw1dels.sh* to clear the previous
> > outputs, change inputs and /or parameters and run *tsw1.sh* again.
>
> **Very close to the original: HARMONICITY MAINTAINED and FORMANTS
> PRESERVED**
>
> simple transposition with a constant or time-varying breakpoint file
>
> - REPITCH TRANSPOSEF Mode 3, 7 semitones higher
>   [tsw1-1a.wav](tsw1-1a.wav)
> - REPITCH TRANSPOSEF Mode 3, time-varying with *glideup.brk* (0.0 0,
>   0.49 0, 0.5 -12, 4.0 24, 4.8 -12) [tsw1-1b.wav](tsw1-1b.wav)
>
>
>
>
> using a .frq pitch trace *derived from the infile* (translated into a
> .trn); the shape of the pitch movement in the *infile* becomes the
> (time-varying) transposition data. When re-applied to the *infile*,
> the sound remains fairly true to the original. When applied to another
> file, we have a shape transfer.
>
> - TRANSPOSEF Mode 4 (and up 7 semitones via REPITCH PCHSHIFT)
>   [tsw1-2a.wav](tsw1-2a.wav)
> - PITCH OCTMOVE Mode 1 (and up a 12^th^: factor of 3 = 3^rd^ harmonic)
>   [tsw1-2b.wav](tsw1-2b.wav)
>
>
>
> **Formants alter ('Mickey Mouse' effect): HARMONICITY MAINTAINED and
> FORMANTS NOT PRESERVED**
>
> time domain sample transposition
>
> - MODIFY SPEED Mode 2, time-varying with *glideup.brk* as above
>   [tsw1-3.wav](tsw1-3.wav)
>
>
>
> time domain harmonic distortion
>
> - DISTORT HARMONIC with harmonics file *harmon.txt* (*harmonic_number
>   amplitude*): 2 0.6, 4 0.8 [tsw1-4.wav](tsw1-4.wav)
>
>
>
> simple transposition without preserving formants (compare the timbral
> qualities with those of 1b, where formants are preserved)
>
> - REPITCH TRANSPOSE Mode 3, time-varying with *glideup.brk* as above
>   [tsw1-5.wav](tsw1-5.wav)
>
>
> Also note the difference between this example and number 3: the
> duration remains the same in spectral transposition, so the sound does
> not speed up. It is therefore closer to the original, even though
> formants are not preserved. There is a 'Mickey Mouse', but much less
> pronounced.
>
>
>
>
> transpose part of the spectrum
>
> - PITCH TRANSP Mode 3, one octave above and below a frequency split
>   point (here, of 1000 Hz) [tsw1-6.wav](tsw1-6.wav)
>
>
>
> shift part of the spectrum
>
> - PITCH TRANSP Mode 6, above (+7 semitones) and below (-5 semitones) a
>   frequency split point of 1000 Hz [tsw1-7.wav](tsw1-7.wav)
>
>
> This is actually still a transposition -- it doesn't become an
> additive process. There is, however, more flexibility regarding
> transposition location and values.
>
>
>
> **Becoming very different: HARMONICITY NOT MAINTAINED, FORMANTS NOT
> PRESERVED**
>
> an inharmonic shift adds rather than multiplies the shift factor, thus
> effecting a much greater change to the sound
>
> - STRANGE SHIFT, Mode 5, to shift the spectrum a certain amount
>   *outside* a specified frequency range (band):
>   [tsw1-8.wav](tsw1-8.wav).
>
>
> Pre-processing the sound with SPEC BARE (to clarify the sound by
> removing everything which isn't a harmonic), or with PITCH ALTHARMS
> to remove either the odd or the even harmonics, provide additional
> ways to adjust the overall result.
>
>
>
> **Adding changes to changes: PROCESSED .FRQ INPUT AND COMBINATIONS**
>
> use a processed .frq input
>
> - REPITCH INVERT Mode 2. This time we will invert the [pitch trace of
>   the 1^st^ sound](tsw1ptrace.wav), saving directly to a .trn file
>   then used as the transposition input data in Mode 4 of TRANSPOSE and
>   applied to the 1^st^ sound.
>
>   We hear the *spectrum* (= timbral quality) of the original sound
>   altered by the shape of its inverted pitch trace moving through it,
>   as well as the pitch shape changing direction.
>   [tsw1-9.wav](tsw1-9.wav)
>
>
> Other great processing options in REPITCH include EXAG, PCHSHIFT,
> QUANTISE, RANDOMISE, SMOOTH and VIBRATO.
>
>
>
>
> combinations 1: use a .frq (to .trn) derived from another soundfile as
> input when not preserving formants
>
> - We analyse the 2^nd^ soundfile and extract its [pitch trace
>   2^nd^](tsw1-2ndptrace.wav) to a .frq. This we use as the second
>   input to REPITCH COMBINE Mode 1; the first input is the .frq [pitch
>   trace 1^st^](tsw1ptrace.wav) from the 1^st^ sound.
>
>   The output of COMBINE is a new .trn then used as the transposition
>   data in REPITCH TRANSPOSE Mode 4, here applied to the 1^st^ sound.
>   We now hear the 1^st^ sound moving up and down following the pitch
>   trace derived from the 2^nd^ soundfile. [tsw1-10.wav](tsw1-10.wav)
>
>
>
> combinations 2: use a *processed* .frq as an input to COMBINE
>
> - Number 9 INVERTed the [pitch trace 1^st^](tsw1ptrace.wav) of the
>   first sound. We can add VIBRATO to this inversion and then COMBINE
>   this new vibrato'd inversion with the [pitch trace
>   2^nd^](tsw1-2ndptrace.wav) of the second sound. This gives us a
>   pitch trace which combines features of both sounds, plus some
>   vibrato. Finally, we apply this new transposition data to *both* our
>   input sounds to hear what it does.
>
>   To achieve this, we re-do the INVERT process on the 1^st^ sound,
>   this time saving to a .frq instead of a .trn so that we can use
>   the.frq as an input to the VIBRATO process, again saving to .frq.
>
>   The .frq pitch trace of the 2^nd^ sound (this will be the first
>   input) is then COMBINEd (Mode 1) with the vibrato'd inversion .frq
>   (of the 1^st^ sound -- this will be the second input) to form a new
>   .trn.
>
>   This new .trn is then applied to the 1^st^ sound's .ana with
>   REPITCH TRANSPOSEF, Mode 4 to form [tsw1-11iva.wav](tsw1-11iva.wav),
>   and to the 2^nd^ sound's .ana to form
>   [tsw1-11ivb.wav](tsw1-11ivb.wav).
>
>
> These two sounds provide good examples of shape transfers. The
> potential for accumulated transformations is huge. Having an effective
> method for keeping track of your files and steps as change is added to
> change becomes very important. Identifying the various soundfile
> inputs and their .ana and .frq files derived from them with 1^st^,
> 2^nd^, 3^rd^ etc. in the name might be one solution.
>
> ### [2. Focus on Simple Transposition:]{#SIMTRANS}, maintain Harmonicity -- Formants Preserved/not Preserved (EXPLORE TOOL: Batch File: *tsw2.sh*, Deletions File: *tsw2dels.sh*)
>
> We first look at the Time-Domain, where sample data (rather than
> analysis data) is processed. A typical compositional goal is to input
> a [sound](tsw2.wav) and [raise it](tsw2u12.wav) an octave in pitch
> (MODIFY SPEED, Mode 2). When this is done in the Time-Domain, the
> harmonicity of the sound is maintained, but **the formants move** (=
> 'are not preserved' = 'spectral envelope also moves'). This
> creates what is now referred to as a 'Mickey-Mousing' effect: high
> squeaky voices, originally done by speeding up the playback of an
> analogue tape.
>
> We hear the same aural movement of the formants when we
> [transpose](tsw2tvtrn.wav) by a time-varying breakpoint file (MODIFY
> SPEED, Mode 2) or [accelerate](tsw2accel.wav) (MODIFY SPEED, Mode 5)
> the sound -- i.e., transpose to a certain level *transposition*) by a
> certain time (*goaltime*). The 'acceleration' transposition can go
> up or down, that is, it can also decelerate.
>
> The breakpoint file *tsw2brk.brk* for the time-varying transposition
> reads:
>
>
>     0    0
>     1   12
>     3   -24
>     4.5 12
>
> Note that the `-24` takes the sound down two octaves from the point it
> had reached after 1 second (up one octave). Thus, when it moves from
> `-24` up another `12` semitones, it returns to its original frequency
> level.
>
> [Rapid vibrato-like fluctuations](tsw2vib.wav) of the transpositions
> here in the Time-Domain also 'Mickey-Mouse' the sound, with some
> extraordinary side-effects as the *vibrato_rate* (here=12) and the
> amount of *transposition* (here=7) increases. MODIFY SPEED, Mode 6 is
> well worth further investivation.
>
> The next two examples come from the DISTORT Group, which manipulates
> **waveshapes**, which in this case means portions of sound files which
> lie between zero crossings. These are of irregular length, so the
> manipulations result in a special kind of distortion. There is nothing
> in the code which actually changes the harmonic relationship of the
> partials, so this is maintained, although, aurally, it doesn't much
> sound like it.
>
> The two examples from the DISTORT Group created here focus on
> functions which involve transposition. Multiplying the rate of the
> waveshapes (by removing samples) also [raises the
> (distorted)](tsw2mult.wav) sound in pitch (DISTORT MULTIPLY, using
> *multiplier* = 2). Accumulated multiplications following the harmonic
> series, produce a [similar result](tsw2harm.wav), but one which may
> retain more of the original sound if the lower harmonic transpositions
> are employed (DISTORT HARMONIC). In this case the harmonics-file
> contains 'harmonics' 4, 8, 16 and a squeaky 32.
>
> The last three sounds in this section enable us to compare the sound
> without and then with formants preserved.
>
> The first sound [tsw2transp.wav](tsw2transp.wav) is made by PITCH
> TRANSP, Mode 4 (shift above *freq_split*) with the split point at the
> bottom of the frequency range (10 Hz) so that virtually the whole
> spectrum is transposed. PITCH TRANSP does not preserve the formants,
> so that when raised by 12 semitones as in this example, we hear the
> voice become audibly 'thinner'. The effect however, is not as
> pronounced as in the Time-Domain transposition because the sound does
> not 'speed up' as it goes higher. PITCH TRANSP only allows the use
> of constants as transposition factors, i.e., it cannot be
> time-varying. You would ordinarily use this function with a split
> point somewhere in the middle of the sound in order to achieve timbral
> variants.
>
> The second sound [tsw2tpose.wav](tsw2tpose.wav) is made by REPITCH
> TRANSPOSE and also does not preserve formants, so we hear the timbre
> of the voice 'thin' towards the top of this time-varying example and
> 'thicken/growl' at the bottom. The third sound
> [tsw2tposef.wav](tsw2tposef.wav) is made by TRANSPOSEF which *does*
> preserve formants, and we hear a (marginally?) more consistent timbre
> throughout. The formants have been extracted with **-f***12* (linear
> frequencywise).
>
> REPITCH TRANSPOSE and TRANSPOSEF, Modes 1-3, are useful to achieve
> Spectral-Domain transpositions, time-varying or not, leaving the
> duration of the sound unchanged. TRANSPOSEF is preferred for vocal
> sounds meant to remain similar to the source sound. Mode 4 reads in a
> .trn binary transposition data file and can be used in more complex
> operations, as described elsewhere in this Workshop.
>
>
>   SOUND                              DESCRIPTION & TYPE                                      CDP FUNCTION         MODE & PARAMS
>   ---------------------------------- ------------------------------------------------------- -------------------- ---------------------------------------
>   [tsw2u12.wav](tsw2u12.wav)         transpose by a constant \[**mHmvF**\]                   MODIFY SPEED         **2** *semitone_tr_factor*
>   [tsw2tvtrn.wav](tsw2tvtrn.wav)     transpose by a breakpoint file \[**mHmvF**\]            MODIFY SPEED         **2** file: *time semitone_tr_factor*
>   [tsw2accel.wav](tsw2accel.wav)     acceleration towards a goaltime \[**mHmvF**\]           MODIFY SPEED         **5** *acceleration* *goaltime*
>   [tsw2vib.wav](tsw2vib.wav)         pitch vibrato \[**mHmvF**\]                             MODIFY SPEED         **6** *vib_rate semitone_tr_factor*
>   [tsw2mult.wav](tsw2mult.wav)       multiply waveshapes \[**mHmvF**\]                       DISTORT MULTIPLY     **None** *integer_multiplier (2-16)*
>   [tsw2harm.wav](tsw2harm.wav)       harmonic multiples of waveshapes \[**mHmvF**\]          DISTORT HARMONIC     **None** *harmonics amplitude* file
>   [tsw2transp.wav](tsw2transp.wav)   transpose spectrum by a constant \[**mHmvF**\]          PITCH TRANSP         **4** *frq_split semitone_tr_factor*
>   [tsw2tpose.wav](tsw2tpose.wav)     transpose spectrum by a breakpoint file \[**mHmvF**\]   REPITCH TRANSPOSE    **3** with bp file *tsw2brk.brk*
>   [tsw2tposef.wav](tsw2tposef.wav)   transpose spectrum by a breakpoint file \[**mHpF**\]    REPITCH TRANSPOSEF   **3** with bp file: *tsw2brk.brk*
>
>   : **COMPARATIVE LISTENING 2. QUICK-LISTEN SUMMARY (Source:
>   [tsw2.wav](tsw2.wav))**
> :::
>
> ### [3. Using a .frq]{#SIMFRQIN} (binary pitch data file) Input (EXPLORE TOOL: Batch File: *tsw3.sh*, Deletions File: *tsw3dels.sh*)
>
> There is only one program which takes a .frq input and goes on
> directly to produce an analysis file: PITCH OCTMOVE. Two other sets
> take a .frq input, but their output are .frq, .trn or .brk, not
> analysis files. These two sets are the REPITCH functions meant for
> 'intermediate .frq processing' and the the COMBINE/B pair. These two
> sets are presented in 'Listen & Compare' sections 5 and 6.
>
> In this section therefore, we shall only consider PITCH OCTMOVE. We
> have already seen this function in sections 1 and 2 of 'Listen &
> Compare', so there is not a lot more to say. It may be useful,
> however, to provide some examples by which we can compare the outputs
> of PITCH OCTMOVE, REPITCH TRANSPOSEF (both of these preserve formants)
> and REPITCH TRANSPOSE (which does not preserve formants).
>
> To do this we will create identical transpositions for each of these
> functions: at the octave (12 semitones), the twelfth (19 semitones),
> and at two octaves (24 semitones). PITCH OCTMOVE does this with an
> integer 'octave' transposition factor which in fact follows the
> harmonic series: 2 = one octave, 3 = a twelfth, 4 = two octaves, etc.
>
> To make these examples, please run ***tsw3.sh***. The deletions file
> for it is ***tsw3dels.sh***. As usual, you can edit the batch file to
> have it use whichever sound you wish.
>
>
>   ---------------------------------------------------------------------------------------------
>   SOUND                              DESCRIPTION &     CDP FUNCTION      MODE & PARAMS
>                                      TYPE
>   ---------------------------------- ----------------- ----------------- ----------------------
>   [tsw3om1.wav](tsw3om1.wav)         transpose by a    PITCH OCTMOVE     **1**
>                                      constant                            *'octave'\_factor* =
>                                      \[**mHpF**\]                        2
>
>   [tsw3rtfu12.wav](tsw3rtfu12.wav)   transpose         REPITCH           **3** *semitones* = 12
>                                      spectrum by a     TRANSPOSEF
>                                      semitone constant
>                                      \[**mHpF**\]
>
>   [tsw3rtu12.wav](tsw3rtu12.wav)     transpose         REPITCH TRANSPOSE **3** *semitones* = 12
>                                      spectrum by a
>                                      semitone constant
>                                      \[**mHmvF**\]
>
>   [tsw3om2.wav](tsw3om2.wav)         transpose by a    PITCH OCTMOVE     **1**
>                                      constant                            *'octave'\_factor* =
>                                      \[**mHpF**\]                        3
>
>   [tsw3rtfu19.wav](tsw3rtfu19.wav)   transpose         REPITCH           **3** *semitones* = 19
>                                      spectrum by a     TRANSPOSEF
>                                      semitone constant
>                                      \[**mHpF**\]
>
>   [tsw3rtu19.wav](tsw3rtu19.wav)     transpose         REPITCH TRANSPOSE **3** *semitones* = 19
>                                      spectrum by a
>                                      semitone constant
>                                      \[**mHmvF**\]
>
>   [tsw3om3.wav](tsw3om3.wav)         transpose         PITCH OCTMOVE     **1**
>                                      spectrum by a                       *'octave'\_factor* =
>                                      constant                            4
>                                      \[**mHpF**\]
>
>   [tsw3rtfu24.wav](tsw3rtfu24.wav)   transpose         REPITCH           **3** *semitones* = 24
>                                      spectrum by a     TRANSPOSEF
>                                      semitone constant
>                                      \[**mHpF**\]
>
>   [tsw3rtu24.wav](tsw3rtu24.wav)     transpose         REPITCH TRANSPOSE **3** *semitones* = 24
>                                      spectrum by
>                                      semitone constant
>                                      \[**mHmvF**\]
>
>   [tsw3om4.wav](tsw3om4.wav)         transpose         PITCH OCTMOVE     **3**
>                                      spectrum by a                       *'octave'\_factor* =
>                                      constant                            3
>                                      \[**mHpF**\]                        *bassboost* = 4
>   ---------------------------------------------------------------------------------------------
>
>   : **COMPARATIVE LISTENING 3. QUICK-LISTEN SUMMARY (Source:
>   [tsw3.wav](tsw3.wav))**
> :::
>
> ### [4. Moving]{#SPECMOVE} (part of) the Spectrum -- various types (EXPLORE TOOL: Batch File: *tsw4.sh*, Deletions File: *tsw4dels.sh*)
>
> Now we turn our attention to shifting frequencies. This can be done
> while maintaining harmonicity with PITCH TRANSP, and without
> maintaining harmonicity with STRANGE SHIFT and HILITE BAND. The
> presence or absence of harmonicity does make a big difference. The
> examples in this section are made with ***tsw4.sh***.
>
> The main role of PITCH TRANSP is to move only a part of the spectrum
> by specifying a frequency split point. Harmonicity is maintained, but
> the formants are not preserved. The combination of shifted and
> unshifted frequency components results in timbral variants akin to
> filtering techniques. Only the *depth* parameter (transposition effect
> on source) is time-varying, so the effects apply consistently to the
> whole sound. The following table provides six examples to listen to
> and compare, one for each mode. The *depth* parameter is not used.
>
>
>   SOUND                          DESCRIPTION & TYPE                                  CDP FUNCTION   MODE & PARAMS
>   ------------------------------ --------------------------------------------------- -------------- -----------------------------------------
>   [tsw4ptm1.wav](tsw4ptm1.wav)   shift 8^ve^ up, above split point \[**mHmvF**\]     PITCH TRANSP   **1** *frq_split*
>   [tsw4ptm2.wav](tsw4ptm2.wav)   shift 8^ve^ down, below split point \[**mHmvF**\]   PITCH TRANSP   **2** *frq_split*
>   [tsw4ptm3.wav](tsw4ptm3.wav)   8^ve^ shift up & down \[**mHmvF**\]                 PITCH TRANSP   **3** *frq_split*
>   [tsw4ptm4.wav](tsw4ptm4.wav)   pitch shift up, above split point \[**mHmvF**\]     PITCH TRANSP   **4** *frq_split transpose*
>   [tsw4ptm5.wav](tsw4ptm5.wav)   pitch shift down, below split point \[**mHmvF**\]   PITCH TRANSP   **5** *frq_split transpose*
>   [tsw4ptm6.wav](tsw4ptm6.wav)   pitch shift up & down \[**mHmvF**\]                 PITCH TRANSP   **6** *frq_split transpose1 transpose2*
>
>   : **COMPARATIVE LISTENING 4A. (PITCH TRANSP) QUICK-LISTEN SUMMARY
>   (Source: [tsw4pt.wav](tsw4pt.wav))**
> :::
>
> []{#HILITEBAND}
>
> Dividing up the frequency range into bands takes the split point idea
> considerably further. In the Time-Domain, we can create a file of
> user-defined filter bands. In the Spectral-Domain we can create a file
> to define frequency bands which can be amplified, transposed or
> shifted, replacing existing frequencies with the new information, or
> adding it to them. All this is done with HILITE BAND.
>
> The datafile created for HILITE BAND contains the following elements:
> *lowfreq highfreq bitflag \[amp1 amp2 \[+\]transpose\]*
> The first two define a frequency range. The bitflag has four digits
> which can be 0 or 1, with the following meaning:
>
> - **bit 1** -- **1**000 -- amplitude change multiplier: put in a value
>   for *amp1*
> - **bit 2** -- 0**1**00 -- second value to create an amplitude
>   gradient (cresc. or decresc.), also a multiplier: put in a vlaue for
>   *amp2*
> - **bit 3** -- 00**1**0 -- transposition multiplier for that band: put
>   in a value for *transpose*. If you want to ADD rather than multiply,
>   use a value in Hz and precede it with a + sign
> - **bit 4** -- 000**1** -- by default, the altered frequency bands
>   *replace* the existing frequencies. Fill in this flag if you want
>   the altered frequency bands to be *added* to the existing ones.
>
> Also see the HILITE BAND entry in *CDP File Formants & Codes* for
> additional explanation and a detailed example.
>
> Our single [soundbyte](tsw4hb1.wav) for this function uses the
> following datafile, *tsw4bits.txt*:
>
>
>       10     320    1000    0.0
>      320     390    1110    1.0 2.0 0.9
>      390     410    1000    0.0
>      410     452    1100    1.0 2.0
>      452     500    1000    0.0
>      500     550    1111    1.0 2.0 +7
>      550     800    1000    0.0
>      800     905    1100    0.5 3.0
>      905    1280    1000    0.0
>     1280    1810    1100    0.1 2.0
>     1810    2560    1000    0.0
>
> Comments on this datafile:
>
> - Bands are zeroed by setting bit one and assigning an amplitude of 0.
>   So much of the sound is zeroed out in this example that it ends up
>   rather soft and could use a bit of gain.
>
>
> - The fact that the amplitude value is a multiplier is unusual. Values
>   greater than 4 may cause overflow.
>
>
> - This is not the same as filtering, but you will find that if bands
>   are too wide in the register where most of the sound is
>   concentrated, too much of the original will come through and there
>   will be little audible difference.
>
>
> - Similarly, it can be useful to do something with all the most
>   salient frequencies, i.e., without leaving gaps between frequency
>   bands. In the above file, the wide bands are zeroed, and the narrow
>   ones are amplified.
>
>
> - The +7 in the 500-550 band with the 4^th^ bit flag has no noticeable
>   effect, but is included here to illustrate the use of these
>   features.
>
> STRANGE SHIFT, on the other hand, shifts the spectrum in an inharmonic
> way. This is a very important difference, as we shall hear with the
> following examples. Also, the key parameters are time-varying, so the
> possibilities for changing timbres and shape transfer are extensive.
> The following set of examples use all these features to illustrate the
> flexibility of this function.
>
> The breakpoint files are:
>
>
>     frqshift.brk frqdiv.brk  frqlow.brk  frqhigh.brk
>     0.0   10    0.0   300   0.0   500   0.0   900
>     4.5 3000    1.0  1000   1.0  1200   1.0  1500
>             2.0   600   4.5   200   4.5   200
>             3.5   900
>             4.5   200
>
>
>   SOUND                          DESCRIPTION & TYPE                                       CDP FUNCTION    MODE & PARAMS
>   ------------------------------ -------------------------------------------------------- --------------- ------------------------------
>   [tsw4ssm1.wav](tsw4ssm1.wav)   shift the whole spectrum \[**noHmvF**\]                  STRANGE SHIFT   **1** *frqshift* (.brk file)
>   [tsw4ssm2.wav](tsw4ssm2.wav)   shift the spectrum above the frqdivide \[**noHmvF**\]    STRANGE SHIFT   **2** *frqshift frqdivide*
>   [tsw4ssm3.wav](tsw4ssm3.wav)   shift the spectrum below the frqdivide \[**noHmvF**\]    STRANGE SHIFT   **3** *frqshift frqdivide*
>   [tsw4ssm4.wav](tsw4ssm4.wav)   shift only within the low-high frqband \[**noHmvF**\]    STRANGE SHIFT   **4** *frqshift frqlo frqhi*
>   [tsw4ssm5.wav](tsw4ssm5.wav)   shift only outside the low-high frqband \[**noHmvF**\]   STRANGE SHIFT   **5** *frqshift frqlo frqhi*
>
>   : **COMPARATIVE LISTENING 4B. (STRANGE SHIFT) QUICK-LISTEN SUMMARY
>   (Source: [tsw4ss.wav](tsw4ss.wav))**
> :::
>
> ### [5. Intermediate processing of a .frq]{#PROCESSFRQ} (binary pitch data file) Input (EXPLORE TOOL: Batch File: *tsw5.sh*, Deletions File: *tsw5dels.sh*)
>
> In this section we look at a number of REPITCH functions designed to
> take a .frq pitch trace input and process it further: EXAG, INVERT,
> QUANTISE, RANDOMISE, SMOOTH, PCHSHIFT, and VIBRATO. Rather than look
> at each of these individually, we will *accumulate* the changes by
> using the .frq output of the previous process as the intput to the
> next process.
>
> We also made a .trn output and use that as an input to TRANSPOSE (or
> TRANSPOSEF) so that we can produce an analysis file, resynthesise it
> and audition each stage of this little journey. PCHSHIFT does not
> produce a .trn output, so we use COMBINE to achieve this, combining
> the PCHSHIFT .frq output with *tsw5.frq*.
>
> (Bug fixes made on 10-11 September 2005 appear to have corrected
> errors in REPITCH that affected this batch file. It should now work OK
> as described.)
>
>
>   SOUND                                DESCRIPTION & TYPE                                  CDP FUNCTION                MODE & PARAMS
>   ------------------------------------ --------------------------------------------------- --------------------------- -------------------------------------------
>   [tsw5x.wav](tsw5x.wav)               exaggerate .frq pitch contour \[**mHmvF**\]         REPITCH EXAG                **5 & 6** *meanpch range contour*
>   [tsw5xi.wav](tsw5xi.wav)             & invert .frq pitch contour \[**mHmvF**\]           REPITCH INVERT              **1 & 2** *map* \[0\]
>   [tsw5xiq.wav](tsw5xiq.wav)           & quantise .frq pitch contour \[**mHmvF**\]         REPITCH QUANTISE            **1 & 2** *q-set*
>   [tsw5xiqr.wav](tsw5xiqr.wav)         & randomise .frq pitch contour \[**mHmvF**\]        REPITCH RANDOMISE           **1 & 2** *maxinterval timestep*
>   [tsw5xiqrs.wav](tsw5xiqrs.wav)       & smooth .frq pitch contour \[**mHmvF**\]           REPITCH SMOOTH              **1 & 2** *timeframe* \[millisec\]
>   [tsw5xiqrsp.wav](tsw5xiqrsp.wav)     & shift pitch of .frq pitch contour \[**mHmvF**\]   REPITCH PCHSHIFT, COMBINE   **None, 1** *transposition* \[semitones\]
>   [tsw5xiqrspv.wav](tsw5xiqrspv.wav)   & add vibrato to .frq pitch contour \[**mHmvF**\]   REPITCH VIBRATO             **2** *vibfrq vibrange*
>
>   : **COMPARATIVE LISTENING 5. (REPITCH Processing Functions)
>   QUICK-LISTEN SUMMARY
>   (Source: [tsw5.wav](tsw5.wav))**
> :::
>
>
>
>
> ### [6. Combinations]{#COMBOS} of .frq and .trn files (EXPLORE TOOL: Batch File: *tsw6cp.sh*, Deletions File: *tsw6dels.sh*)
>
> In the last section of 'Listen & Compare', we run through a number
> of possible combinations of .frq and .trn files. This is where the
> tremendous flexibility of this program set comes into its own, as a
> given input can be combined with simple or highly processed shapes
> which emerge either out of itself or from another file.
>
> Note the naming conventions adopted in order to minimise confusion.
> The alternative descriptions 'pass 1 through \...' etc. are only
> intended to provide another way to visualise what is happening. By
> repeated audition of the 3 pitch traces, one can gradually become
> aware of how the shapes are interacting. Some intermediate processing
> is thrown in to illustrate further how the various functions can be
> woven together to produce the desired (or a serendipitous) result. The
> details of parameter values, which you can alter, are contained in the
> batch file.
>
>
>   --------------------------------------------------------------------------------------------------------
>   SOUND                                              DESCRIPTION &     CDP FUNCTIONS     MODES & PARAMS
>                                                      TYPE
>   -------------------------------------------------- ----------------- ----------------- -----------------
>   [tsw6cp1by2.wav](tsw6cp1by2.wav)                   modify 1 by shape REPITCH COMBINE   **1 and 4**
>                                                      of 2 = 1by2\      and TRANSPOSE
>                                                      i.e., 'pass 1
>                                                      through the shape
>                                                      of 2'
>
>   [tsw6cp2by1.wav](tsw6cp2by1.wav)                   modify 2 by shape REPITCH COMBINE   **1 and 4**
>                                                      of 1 = 2by1\      and TRANSPOSE
>                                                      i.e., 'pass 2
>                                                      through the shape
>                                                      of 1'
>
>   [tsw6cp3by2.wav](tsw6cp3by2.wav)                   modify 3 by shape REPITCH COMBINE   **1 and 4**
>                                                      of 2 = 3by2\      and TRANSOSE
>                                                      i.e., 'pass 3
>                                                      through the shape
>                                                      of 2'
>
>   [tsw6cp1and1by2.wav](tsw6cp1and1by2.wav)           combine 1 & 1     REPITCH COMBINE,  **2, 2 and 4**
>                                                      modified by 2 =   VIBRATO and
>                                                      1and1by2, then    TRANSPOSE
>                                                      VIBRATO
>                                                      i.e., 'pass 1
>                                                      through the shape
>                                                      of 1 shaped by
>                                                      2'
>
>   [tsw6cp1and2by1.wav](tsw6cp1and2by1.wav)           combine 1 & 2     REPITCH COMBINE,  **2, 2 and 4**
>                                                      modified by 1 =   EXAG and
>                                                      1and2by1, then    TRANSPOSE
>                                                      EXAG
>                                                      i.e., 'pass 1
>                                                      through the shape
>                                                      of 2 shaped by
>                                                      1'
>
>   [tsw6cp2and1by2.wav](tsw6cp2and1by2.wav)           combine 2 & 1     REPITCH COMBINE,  **2, 2 and 4**
>                                                      modified by 2,    INVERT and
>                                                      then INVERT\      TRANSPOSE
>                                                      i.e., 'pass 2
>                                                      through the shape
>                                                      of 1 shaped by
>                                                      2'
>
>   [tsw6cp2and2by1.wav](tsw6cp2and2by1.wav)           combine 2 & 2     REPITCH COMBINE,  **2, 2 and 4**
>                                                      modified by 1 =   QUANTISE and
>                                                      2and2by1, then    TRANSPOSE
>                                                      QUANTISE
>                                                      i.e., 'pass 2
>                                                      through the shape
>                                                      of 2 shaped by
>                                                      1'
>
>   [tsw6cp1by2and3by2.wav](tsw6cp1by2and3by2gd.wav)   combine 1         REPITCH COMBINE   **3 and 4**
>                                                      modified by 2 & 3 and TRANSPOSE
>                                                      modified by 2 =
>                                                      1by2and3by2
>                                                      i.e., 'pass 1
>                                                      shaped by 2
>                                                      through the shape
>                                                      of 3 shaped by
>                                                      2'
>   --------------------------------------------------------------------------------------------------------
>
>   : **COMPARATIVE LISTENING 6. (REPITCH COMBINE)
>   QUICK-LISTEN SUMMARY
>   (Sources: [tsw6cp1.wav](tsw6cp1.wav) [tsw6cp2.wav](tsw6cp2.wav)
>   [tsw6cp3.wav](tsw6cp3.wav))**
>   (Pitch Traces: [tsw6cp1hearfrq.wav](tsw6cp1hearfrq.wav)
>   [tsw6cp2hearfrq.wav](tsw6cp2hearfrq.wav)
>   [tsw6cp3hearfrq.wav](tsw6cp3hearfrq.wav)) {border="1"
>   cellpadding="3"}
> :::




## [III -- Directed Study: Some Practical Exercise Designs]{#EXERCISES}

> **It is assumed that you will work on the following exercises with
> your *SoundShaper* or *Sound Loom* graphic interface. You may also
> want to try the batch file method to create more extended sets of your
> own trial outputs.**

1.  ### [Time-varying Transposition]{#TDSPEED}

    > Make sure *tswexer1.brk* is present in your current directory.
    >
    >
    >         0.0 -12
    >         1.0  -9
    >         2.5 -18
    >         3.5   0
    >         3.51     12
    >         4.9 -12
    >
    >
    > Run MODIFY SPEED Mode 2 (semitones) with this breakpoint file
    > naming the output [tswglisses.wav](tswglisses.wav) and audition
    > the resulting soundfile (the extension does not have to be put on
    > the name of the outfile).
    >
    > Audition it again a few times from within the GUI, and also by
    > calling `playsfx tswglisses` on the command line, (or MAC
    > terminal: `paplay tswglisses`) and (PC) repeating the command by
    > using the up-arrow (DOSkey must be installed).
    >
    > Explain exactly how the breakpoint file produces the result it
    > does: i.e, exactly where is the transposition taking the sound at
    > each time-point? **Rule of thumb: all semitone or ratio
    > transpositions are *relative to the original pitch level, not the
    > previous position***.
    >
    > Now create your own breakpoint file with the following features:
    >
    > - Start at 0 semitones or 1.0 ratio (no transposition)
    > - Glissando to a new transposition during the first second or so
    > - Hold that position for about 1 second
    > - Then move 'instantly' up or down
    > - Hold the new position
    > - Return to the original pitch level

2.  ### [Distortion Transposition]{#TDDSTORT}

    > The workshop covers two programs which involve waveshape
    > distortion and transposition: DISTORT MULTIPLY and DISTORT
    > HARMONIC.
    >
    > Run DISTORT MULTIPLY first with various integer constant
    > *multipliers* (range 2 to 16).
    >
    > Explore DISTORT HARMONIC with widely different constructions of
    > harmonics in the *harmonics-file*:
    >
    > - low values close together
    > - alternate harmonics (skipping the one inbetween)
    > - harmonics which rise geometrically (2-4-8-16 \...)
    > - very widely spaced harmonics (the range is 2 to 1024!)
    > - when the *amplitude* values for the harmonics increase as the
    >   harmonic number rises; when they decrease
    > - Use a large amount of *pre_attenuation* -- the lower the value
    >   (below 0), the more reduction there is

3.  ### [Explore inharmonic shifts]{#INHARMONIC}

    > Two programs can be used to achieve inharmonic shifts: STRANGE
    > SHIFT and HILITE BAND.
    >
    > 1.  Run [SPECINFO PEAK and/or SPECINFO REPORT](#XTRAINFO) (e.g.,
    >     both Modes 1 and 2) and examine the spectral information about
    >     your source sound which they provide.
    > 2.  STRANGE SHIFT
    >     - shift with a series of constants, getting higher and higher
    >     - design a time-varying shift to change the amount of shift in
    >       a time-varying way
    >     - see what happens with the above time-varying shift value
    >       when the *frq_divide* is also moving
    > 3.  HILITE BAND
    >     - re-examine the [*data-file*](#HILITEBAND) for HILITE BAND
    >       used as one of the Workshop examples. Note how it uses
    >       contiguous bands, and zeroes some of them.
    >     - now create your own *data-file* in various ways:
    >       - use only bit 1 and experiment with different amplitude
    >         patterns
    >       - zero the middle bands in order to hollow out the middle of
    >         the sound
    >       - use bits 1 and 2 for two sets of bands, such that one set
    >         crescendos and the other decresendos
    >       - use bits 1 and 3 and seek to make the two sets of bands
    >         diverge; compare MULTIPLY and ADD
    >       - use bits 1 and 3 and seek to create a sound with several
    >         clearly separated striations (stripes, like thin ribbons
    >         of sound)
    >       - use all four bands and see how dense you can make the
    >         sound become

4.  ### [With and without Preserving Formants]{#FORMANTS}

    > Formants preserved: PITCH OCTMOVE, PITCH TRANSP, REPITCH
    > TRANSPOSEF
    > Formants NOT preserved: REPITCH TRANSPOSE
    >
    > - line up a number of different source sounds and analyse them
    > - create frequency information files with SPECINFO REPORT, Mode 1
    >   (time + formant center frequencies in ascending order) for each
    >   sound and print them out
    > - run the Explore Tool *tswpfopt.sh* with each sound and audition
    >   the results while looking at the printouts, observing the
    >   difference between the outputs on the left side (REPITCH
    >   TRANSPOSE: formants not preserved) and the outputs on the right
    >   side (REPITCH TRANSPOSEF: formants preserved.
    > - compare the outputs of TRANSPOSE and TRANSPOSEF when run with a
    >   time-varying breakpoint file which holds the transposition for a
    >   couple of seconds first at a high point and then at a low point.

5.  ### [Sequenced Processing]{#SEQUENCE}

    > We can explore the unexpected by running the same data through a
    > series of processes sequentially, further transforming the
    > material at each step along the way. These use the various REPITCH
    > transformations which can be applied to a .frq file.
    >
    > - select a source sound, analyse it and extract its pitch trace
    >   with REPITCH GETPITCH, Mode 1, creating a .frq file
    > - select 3 REPITCH transformations, and run the .frq through each
    >   of them in turn, using the previous output as the input to the
    >   next process. Create a .frq output for the first two, and then a
    >   .trn output for the third.
    > - create .ana files using this .trn both with REPITCH TRANSPOSE
    >   and REPITCH TRANSPOSEF and resynthesise
    > - run the 3 transformations again *in a different order* and see
    >   if you get the same result

6.  ### [Combinations of .frq Pitch Trace Shapes]{#COMBINE}

    > Let's compare results when the two sources are combined first in
    > one order, and then in reverse order. Number them so it is easy to
    > identify what the order is.
    >
    > - analyse your sources and create .frq files, resynthesising the
    >   .ana versions of the pitch trace data so you can listen to them
    > - using REPITCH COMBINE, Mode 1, create a .trn with the order 1,
    >   then 2
    > - then create a .trn with the order 2, then 1
    > - create .ana files using this .trn both with REPITCH TRANSPOSE
    >   and REPITCH TRANSPOSEF and resynthesise
    > - describe the difference and see if you can work out how the two
    >   files are affecting each other -- it may be useful to audition
    >   the pitch traces inbetween audtioning the combination outfiles.

------------------------------------------------------------------------

Last updated: 17 November 2021 (wav extensions)

------------------------------------------------------------------------

**©2002-2021 Archer Endrich, HITHER GATE
MUSIC, Plymouth, Devon
PL4 7NL** -- **archerhgm@gmail.com**

------------------------------------------------------------------------
