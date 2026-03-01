# TEXTURE Note Data File Chart

![CDP Logo](../images/cdpcircs.jpg)

## Contents

- [NDF Configurations](#ndf-configurations)
- [Code Meanings](#codes)

---

## <a name="ndf-configurations"></a>TEXTURE Note Data File Configurations by Function

**MPV** = MIDI Pitch Value, **HF-S** = Harmonic Field-Set, **NS** = Nodal Substructure,
**T** = Timed Rhythm, **M/O** = Motif or Ornament

| Function | Mode | NDF Key<br>MPV/S1/S2/S3 | Description |
|----------|------|-------------------------|-------------|
| SIMPLE | **5** | **MPV/—/—/—** | Note events are selected at random from a pitch range at a *packing* rate and shaped by the other parameters |
| SIMPLE | **1-4** | **MPV/HF-S/—/—** | Note events are selected at random from a Harmonic Field/Set at a *packing* rate and shaped by the other parameters |
| GROUPED | **5** | **MPV/—/—/—** | Note events are selected at random from a pitch range in groups which have their own internal timing etc., and which repeat after *packing* seconds |
| GROUPED | **1-4** | **MPV/HF-S/—/—** | Note events are selected at random from a Harmonic Field/Set in groups which have their own internal timing etc., and which repeat after *packing* seconds |
| PREDECOR<br>DECORATED<br>POSTDECOR | **5** | **MPV/NS/—/—** | Note events shaped by internal grouping parameters are selected at random from a pitch range and placed on a Nodal Substructure |
| PREDECOR<br>DECORATED<br>POSTDECOR | **1-4** | **MPV/NS/HF-S/—** | Note events shaped by internal grouping parameters are selected at random from a Harmonic Field/Set and placed on a Nodal Substructure |
| MOTIFS<br>MOTIFSIN | **5** | **MPV/M/—/—** | Fully defined motifs are attached to pitches selected at random from a pitch range. MOTIFSIN does not have a Mode **5**. |
| MOTIFS<br>MOTIFSIN | **1-4** | **MPV/HF-S/M/—** | Fully defined motifs are attached to pitches selected at random from a Harmonic Field/Set |
| PREORNATE<br>ORNATE<br>POSTORNATE | **5** | **MPV/NS/O/—** | Fully defined ornaments are placed on a Nodal Substructure |
| PREORNATE<br>ORNATE<br>POSTORNATE | **1-4** | **MPV/NS/HF-S/O** | Fully defined ornaments are placed on a Nodal Substructure with pitches restricted to the Field definition |
| TIMED | **5** | **MPV/T/—/—** | Pitches randomly selected from a pitch range are fitted to a rhythmic template which repeats after *skiptime* seconds |
| TIMED | **1-4** | **MPV/T/HF-S/—** | Pitches randomly selected from a Harmonic Field/Set are fitted to a rhythmic template which repeats after *skiptime* seconds |
| TGROUPED | **5** | **MPV/T/—/—** | The onsets of groups of events are set by a rhythmic template; size, pitches and internal timing of groups are shaped by parameter ranges |
| TGROUPED | **1-4** | **MPV/T/HF-S/—** | The onsets of groups of events are set by a rhythmic template, and the pitches of these events are drawn from a Harmonic Field/Set |
| TMOTIFS<br>TMOTIFSIN | **5** | **MPV/T/M/—** | The onsets of fully defined motifs are fitted to a rhythmic template and attached to pitches drawn at random from a pitch range |
| TMOTIFS<br>TMOTIFSIN | **1-4** | **MPV/T/HF-S/M** | The onsets of fully defined motifs are fitted to a rhythmic template and attached (at random) or constrained to a Harmonic Field/Set. TMOTIFSIN does not have a Mode **5**. |

---

## <a name="codes"></a>The Codes and What They Mean

- **MPV** — **All** Note Data Files start with a mandatory statement of a real or arbitrary pitch level of the input soundfile(s), stated as a MIDI Pitch Value ('MPV'). This value is used as a reference level for the pitch transpositions. This may or may not be followed by as many as 3 Sections. So the overall form is: **MPV/Section 1/Section 2/Section 3**.

- **'—'** — The contents of the 3 Sections differ from function to function. This can be confusing, which is why this chart has been put together. If a Section is omitted, a dash '-' is placed in that Section.

- **#*N*** — The 3 Sections all begin with a mandatory statement of the number of lines of (note) events which that Section will contain: the format is #*N*, where *N* is the number of lines.

- **NS** — Nodal Substructure (= 'Line' / 'Melody'). These always have ascending times. The *amplitude* and *duration* fields are inactive.

- **T** — Timed Rhythm. Only the *time* field is active. In effect, this establishes a rhythmic template which defines the onset times of notes from a pitch range, a Harmonic Field/Set, a group, or a motif.

- **HF-S** — Harmonic Field/Set. A time of zero is given in Modes **1** and **3**, which may produce simultaneities (chords) if the time between note events is very short. **Changing times** (in ascending order) are given in Modes **2** and **4**, enabling the harmonies to change at defined time points along the duration of the output soundfile, creating either melodies (time changes at every field event) or chords (more than one event at the same time — and a very fast time between note events). The Harmonic Field/Set is found usually in Section 2, but sometimes in Section 1 of the ndf.

- **M** — Motif. The motif is fully defined, like the ornament, *but does NOT use a Nodal Substructure*. It may therefore be found in Section 1 of the ndf, or in Section 2 if a time grid is used, or in Section 2 or 3 if a Harmonic Field/Set is involved.

- **O** — Ornament. All fields are active, enabling you to create fully defined musical embellishments, with timing, pitch, amplitude and duration for every note event. Durations can be longer or shorter than the time until the start time of the next note event, enabling legato or staccato articulations. Ornaments are ALWAYS attached to a Nodal Substructure (which will always be in Section 1). The ornament's own internal, relative timing begins at the time of the Nodal note, and its pitch is transposed to that of the Node. Ornaments may or may not use a Harmonic Field/Set. They can therefore be found in Sections 2 (no HF-S) or 3 of the ndf (HF-S is in Section 2). There can be several ornaments.

---

*Archer Endrich*
*Last updated: 11 May 2002*
