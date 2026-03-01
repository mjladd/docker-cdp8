# 2 Useful Reference Charts for TEXTURE

## Note Data File Field Definitions

| **MPV** | **HF-S** | **Line/NS** | **M/O** | **Timing** |
|---------|----------|-------------|---------|------------|
| Midi Pitch Value<br>(reference pitch<br>for the source) | Harmonic Field-Set<br>(if 'None', uses parameter<br>Pitch Range) | Linear Contour<br>with times for<br>the nodal points | 'Fully Defined'<br>Motif/Ornament | Rhythmic Template<br>('Sieve') |

## Illustrating the Format of Note Data File Components

The 5 fields are: *Times*, *Instr_no*, *(Midi)_Pitch*, *Vel*, *Dur*

('Y' and 'N' refer to Active and Inactive Fields — they are *not* part of the file —
the '1' for *instrument_number* must always be present, although inactive)

| MPV | Timing | Line (NS) | HF-S | Motif/Ornament |
|-----|--------|-----------|------|----------------|
| `60` | `#4`<br>`0.00 1 0 0 0`<br>`0.34 1 0 0 0`<br>`0.67 1 0 0 0`<br>`1.00 1 0 0 0`<br>`Y   N N N N` | `#4`<br>`0.0 1 60 0 0`<br>`1.0 1 65 0 0`<br>`2.5 1 67 0 0`<br>`3.5 1 72 0 0`<br>`Y  N Y  N N` | `#4 (M 1-3)`<br>`0 1 60 0 0`<br>`0 1 65 0 0`<br>`0 1 67 0 0`<br>`0 1 72 0 0`<br>`N N Y  N N`<br><br>`OR #4 (M 2-4)`<br>`0.0 1 60 0 0`<br>`1.5 1 65 0 0`<br>`2.0 1 67 0 0`<br>`2.0 1 72 0 0`<br>`Y  N Y  N N` | `#4`<br>`0.00 1 60 85 0.3`<br>`0.25 1 65 65 0.3`<br>`0.50 1 67 75 0.3`<br>`0.75 1 72 55 0.3`<br>`Y   N Y  Y   Y` |

## Note Data File Components Used by Each Function

| Program | MPV | Timing | Line (NS) | Modes1-4: HF-S<br>Mode 5: None | Motif/Ornament |
|---------|-----|--------|-----------|-------------------------------|----------------|
| **SIMPLE** | MPV | | | HF-S or None | |
| **GROUPED** | MPV | | | HF-S or None | |
| **DECORATED** | MPV | | Line | HF-S or None | |
| **MOTIFS** | MPV | | | HF-S or None | M/O |
| **MOTIFSIN** | MPV | | | HF-S only | M/O |
| **ORNATE** | MPV | | Line | HF-S or None | M/O |
| **TIMED** | MPV | Times | | HF-S or None | |
| **TGROUPED** | MPV | Times | | HF-S or None | |
| **TMOTIFS** | MPV | Times | | HF-S or None | M/O |
| **TMOTIFSIN** | MPV | Times | | HF-S only | M/O |

---

*Last updated: 11 May 2002*
