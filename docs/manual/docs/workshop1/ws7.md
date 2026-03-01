# Workshop 7: Envelope Transfers

*by Dr Archer Endrich*

The locations of the processing functions are shown under the program name.

- **SS** = the Soundshaper GUI
- **SL** = the Sound Loom GUI

Input sounds for this Worksheet: [trcdt.wav](../../sounds/trcdt.wav) and [frogs3cdt.wav](../../sounds/WS07SF01frogs3cdt.wav).

## A. Time Domain: Envelope Replace

- **SS:** Soundfiles > Envelope > Replace
- **SL:** ENVELOPE > replace > env from other sndfile

The Time Domain envelope is that of the amplitude (loudness) contour of the samples, as shown in the conventional display of a soundfile.

In ENVELOPE REPLACE we take the contour from the second input sound and put it onto the first input sound. For this to work effectively, the second sound needs to have a lively envelope and the first sound a more steady-state envelope. In this example, we take the envelope contour of chirping frogs (second input) and put it onto the sound of an idling tractor engine (first input).

Set up Soundshaper to use your own named directory. Ensure that these two soundfiles are present: [trcdt.wav](../../sounds/trcdt.wav) and [frogs3cdt.wav](../../sounds/WS07SF01frogs3cdt.wav).

### ST1.

First, Open [trcdt.wav](../../sounds/trcdt.wav): the soundfile onto which you will impose the envelope, i.e., whose amplitude contour you will reshape with that of another soundfile. Now go to **SS: Soundfiles > Envelope > Replace** / **SL: ENVELOPE > replace** and you will find that Mode 1 (SS: 'From other sound' / SL: 'env from other sndfile') is already ticked / open. In Soundshaper, Open the second soundfile. This is done in the IN/OUT box where you will see INfile already filled in. Place the cursor into the Inf 2, 3 ... empty box and click on Select, and then choose `frogs3cdt`. In Sound Loom, you have to CHOOSE it before going to the processing function (if it's a 2-file function, you have to select the two to process first, or the function will remain greyed out).

The Window parameter means how many milliseconds between points where we check the amplitude values, i.e., the 'resolution' of the envelope shape. You can accept the default 5 for a fine resolution. Coarser values will tend to smear the frog's envelope, an effect that could be useful at times.

Name the Outfile `trcdtfrogenv` (i.e., tractor-with-frog-envelope). Now click OK and play the result. You should now hear the tractor sound pulsing with the same shape as the frog chirps.

We can check this by mixing the two sounds together synchronously (`frogs3cdt.wav` and `trcdtfrogenv.wav`). We can do this quickly with a simple mix of these two soundfiles.

- In Soundshaper, Open `frogs3cdt.wav` and go to **Edit/Mix > Mix > Two**, which calls up SUBMIX MERGE.
- Now open `trcdtfrogenv.wav` as the second input.
- The Stagger parameter is left at 0 (i.e., start both sounds together).
- Name the Outfile `f-tfstag0` (i.e., frogs + tractor-with-frog-envelope).
- Click on OK and when finished, play the result.

Now do this again, giving Stagger a value of 0.1, i.e., start the tractor sound 1/10th of a second after the frogs. Now the original `frogs3cdt.wav` will lead, and the 'tractor-with-frog-envelope' (Select `trcdtfrogenv.wav`) will follow just after, rather like a shadow effect in text graphics. Name the Outfile `f-tfstag&1`, click on OK and listen to the result.

## B. Spectral Domain: Formants Vocode

- **SS:** Spectral > Morph/Formants > Vocode
- **SL:** FORMANTS > vocode

The Spectral Domain envelope is the 'spectral envelope'. This is the overall, changing, amplitudes of the frequencies in the sound. Because we are raising and lowering frequencies, the timbre, the tone colour, of the sound is going to change. That is, the tone qualities of the second sound will now colour the first sound. With our two input sounds, we can therefore make the tractor chirp like a frog - not just with the pulsations, but with the actual sound of the frog mixed in as well. This is more than a mix as we explored above, because the sound of the frog actually replaces a good part (but not all) of the sound of the tractor: we hear the tractor chirping like a frog. This neat process is called vocoding. It works best with vocal sounds, so the frog's voice should be OK.

First we need to Convert both sounds to analysis files in the usual way: **SS: Spectral > Convert > Analyse** / **SL: PVOC > analysis** if we haven't already done so.

### ST2.

Now we Open `trcdt.ana` and go to **SS: Spectral > Morph/Formants > Vocode** / **SL: FORMANTS > vocode**. Open `frogs3cdt.ana` as the second input (from which the spectral envelope / tone colours will come).

Here, we are given a choice of spectral envelope analysis methods:

- **Mode 1: Formants by freq** = the channel spacing is logarithmic, i.e., higher octaves cover a much wider band of frequencies than lower octaves. For example, the band between 220 (A below Middle C, A-4 on our Chart) and 440 (A above Middle C, A-5 on our Chart) is 220 Hz. But the band between A-8 (3520 Hz) and A-9 (7040 Hz) is 3520 Hz. It's still double, but the band is much wider because each octave doubles the frequency: 220 - 440 - 880 - 1760 - 3520 - 7040 etc. This method is usually recommended for high and unpitched sounds.
- **Mode 2: Formants by pitch** = the band spacing is linear, i.e., there is the same distance between each band in the spectral envelope analysis. This method is usually recommended for lower and pitched sounds.

We can leave the frequency range as it is (100 to 10000 Hz).

Our frog chirps are low and somewhat pitched, so we'll use Method 2 (Formants by pitch) and give 12 as the number of bands. (I tried 4 as well, which was OK. I also tried Method 1 with 4 (which gave a similar result) and 12 (more balanced tractor & frog timbres). All of these settings worked reasonably well with these inputs.) We hear a steady tractor sound, but the machine seems to be inhabited by frogs. Name the output `t-fvocode` (tractor->frog-vocode).

### ST3.

Let's try it the other way round: first the frogs (`frogs3cdt.ana`) into which we put the sound of the tractor (`trcdt.ana`). Use Method 1 (Formants by frequency) and 4 channels. Now we hear the frogs chirping with the voice of a tractor. Name the output `f-tvocode` (frog-tractor-vocode).

You might want to try out these functions with steady state and vocal sounds of your own. Extracting envelopes from complex sounds with both clearly pitched and strong noise components doesn't work that well, as the software struggles with the frequency analysis.

You can Convert `t-fvocode.ana` and `f-tvocode.ana` to soundfiles (`t-fvocode.wav` and `f-tvocode.wav`) to save them along with the other Time Domain outputs.

Please delete the analysis files to clear space on your hard disk.

You can in fact do the envelope transfers in two separate stages, using either ENVELOPE EXTRACT and ENVELOPE IMPOSE in the Time Domain, or FORMANTS GET and FORMANTS PUT in the Spectral Domain. FORMANTS GET produces a formants file (`.for` extension) that is saved to the hard disk, so that it can be used in other functions. For example, with **Morph/Formants > Make spectrum** you can combine the pitch trace from one analysis file (`.frq` extension) with the formant structure (`.for`) of another analysis file to get a combination of these two different features from two different sounds. The pitch trace is acquired from an analysis file with **Pitch > Get Pitch** (note that this is another top-level menu in Soundshaper). This and other combinations form a super-advanced part of the CDP software.

## Worksheet 7 - Envelope Transfers - Summary and Main Outputs

### Extract & Replace Time Domain Amplitude Envelope

| Soundfile | Process |
|-----------|---------|
| `trcdtfrogenv.wav` | ENVEL REPLACE - take envelope of second input and impose it on the first input |
| `f-tfstag0.wav` | SUBMIX MERGE (Mix Two) - confirm synchronous envelope shapes |
| `f-tfstag&1.wav` | SUBMIX MERGE (Mix Two) - second file starts 0.1 sec later to create a 'shadow' effect |

### Extract & Replace Spectral Domain Spectral Envelope (= Vocoding, i.e., 'cross-synthesis')

| Soundfile | Process |
|-----------|---------|
| `t-fvocode.wav` | FORMANTS VOCODE - Mode 2 (pitch) with 12 bands and Mode 1 (frequency) with 4 channels give similar results: we hear the timbre of the frog voice IN the tractor sound. |
| `f-tvocode.wav` | FORMANTS VOCODE - Mode 1 (frequency) with 4 channels: we hear the chirping amplitude envelope shapes of the frogs with the sound of the tractor. |
