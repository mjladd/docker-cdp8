# The CDP Texture Set: Key Components

*by Dr Archer Endrich*

I am constantly trying to find ways to unlock the treasures of the TEXTURE Set of programs. One important thing to remember is that all of these programs start the infile(s) from the beginning for *every note event*, unlike BRASSAGE / *GrainMill* which will create grains as it gradually works its way through the infile. The nature of the onset transient is therefore crucial, as is the duration, i.e., how far into the infile each note event will go – this can be a constant, time-varying, or the 'whole file' (**-w** flag on the command line). There are (potentially time-varying) controls for the use of input soundfiles, density ('packing'), duration, pitch, velocity and tempo. TEXTURE is in effect a semi-algorithmic mixer.

The required Note Data File ('ndf') has to be hand-written. In my own work, I have tried to take things a little further by developing a script that will write algorithmic ndf and brk files for use with TEXTURE. Intricate patterns can be designed and then written as TEXTURE-compatible files. I currently use *Tabula Vigilans* for this (based on a TV script to write *Csound* score files made a long time ago by Nick Fells), but other programming languages such as PYTHON will work just as well, and in some respects, better.

Here I am trying to summarise the key components of the TEXTURE Set of programs. The idea is to get an overview of the salient features of TEXTURE as a guide to what its programs can do, such as:

- washes of note events – multi-event textures – possibly shaped by time-varying upwards/downwards values
- infile repeated with a specified rhythmic pattern
- motifs repeated with a specified rhythmic pattern
- motif textures on randomised pitches
- motif textures that are linked to an unfolding (melodic) line
- pitched note events or motifs mapped to a Harmonic 'Set' or 'Field'
- rhythmic events that overlap and interact (such as Example 20 in *Tutorial Workshop 2*

Grasping the possibilities and how the key components make them possible is a challenge. First, note that:

- 'Randomised' pitches means pitches selected at random from a *minpitch - maxpitch* pitch range or from a Harmonic Field or Set.

- A Harmonic Field or Set define a chord – or more generally a 'harmonic configuration' – i.e., simultaneous notes, although like any chord, its notes may start at different times. A 'Harmonic Field' replicates a set of pitches: specified in one octave but appearing in several octaves. A 'Harmonic Set' uses only the pitches specified. The 5 Modes handle harmony differently:
  - **No** Harmonic Field or Set is used: Mode 5 ('Neutral')
  - With a **Harmonic Field** (Modes 1 & 2: pitches appear in different octaves)
  - With a **Harmonic Set** (Modes 3 & 4: restricted to the pitches defined).

  I often use Mode 3 to achieve a tightly focused harmonic effect. For example, a typical compositional application for a harmonic set is to have long tones belonging to the same chord-set start at different times and overlap.

- Different chords starting at different times can be defined in the same Note Data File by making the start times of all the notes belonging to each chord the same: e.g., the 5 notes of one chord (a Harmonic Set) all start at time 0, and the 7 notes of another chord start at time 11. In this case, time 11 seconds is when the second chord starts to come into play. However, there will be some overlap between the two chords when the note events have started at different times and will therefore end at different times. Thus notes from the first chord-set may still be sounding when notes of the second chord-set begin to play. This overlapping is compositionally useful because it creates a subtle transition between the two chord-sets.

- In DECORATED and ORNATE, the motifs are **centred** around the times; in their PRE- versions, they **end** on the times, and in their POST-versions they **begin** on the times. The latter (POST-) is recommended as the best one to explore first. The other two variants create flexible time-placements that are more unpredictable but ideal when more suppleness is a design feature.

The table below identifies the key components that can form the focus for a passage of music, i.e., a rhythm, or a motif, or a motif that starts with defined rhythmic timing, or a melodic line (canons are possible!). It is important to think about these components and the kind of musical passage that they can create – both separately and in combination.

| Components | Program | Timing Control | Description |
|---|---|---|---|
| defined motifs only | MOTIFS / MOTIFSIN | *packing* | motifs are placed on randomised pitches MOTIFSIN: of the defined harmony |
| rhythmic template only | TIMED | *skiptime* | the same rhythm repeats |
| rhythmic template + defined motifs | TMOTIFS / TMOTIFSIN | *skiptime* | defined motifs are started at the template times, on randomised pitches TMOTIFSIN: of the defined harmony |
| 'line' and randomised motifs | DECORATED, PRE- and POST- | *skiptime* | randomised motifs are attached to the line |
| 'line' and defined motifs | ORNATE, PRE- and POST- | *skiptime* | defined motifs are attached to the line |

The *packing* and *skiptime* parameters are particularly important but a bit tricky to understand, so another document attempts to explain their use more fully: [Getting a Grip on Packing and Skiptime](getting-a-grip-on-packing-and-skiptime.md).

In the TEXTURE Set, the modes are numbered in the Reference Documentation, but not in the GUIs. To put these two items of information together, they are:

- 1. Single harmonic field (meaning all octaves)
- 2. Changing harmonic field
- 3. Single harmonic set (meaning only the exact pitches defined)
- 4. Changing harmonic sets
- 5. None ('Neutral') – therefore only the reference pitch value(s) go in the note data file, and pitch instructions come only from the pitch parameters (on the command line), or the pitch range on the command line has to accommodate the pitch-span of the motif(s).

[I don't actually know what Modes 2 and 4 mean! I have for example been able to create changing harmonic sets as described above in Mode 3.]

**Hints:**

- Also see the two **TEXTURE Charts**: *Texture Note Data File Chart* and *Two Useful Reference Charts for TEXTURE*

- *Tutorial Workshop 2* is devoted entirely to TEXTURE. The Play page comprises a grid with the TEXTURE programs SIMPLE, TIMED, MOTIFS and TMOTIFS across the top, and downwards different types of action: one pitch, pitch range, harmonic set, time-varying, and then a line for nodal substructure which involves the POSTORNATE program. The 20th example illustrates rhythmic interaction of motifs by using the nodal substructure to time their entries such that they overlap.

- When re-running a Texture routine in *Soundshaper* with the 'Repeat last process' two quaver icon, it is necessary to re-open the note data file each time. Also, if you edit the note data file within the process window, *you also need both to save the edits and then re-open the note data file so that the program uses the new version*. In *Sound Loom* when a text file is edited and `Edited Version` is pressed, the edited file is saved and automatically in place ready for use when returned to the processing dialogue.

---

Last updated: 02 August 2021
