// This file is part of a Rust reimplementation of the CDP System, a
// Composers Desktop Project (CDP) command-line sound-processing suite
// originally written by Trevor Wishart, Richard Dobson, Martin Atkins
// and others (see legacy/dev for the original C source and its
// per-file copyright notices).
//
// SPDX-License-Identifier: LGPL-2.1-or-later
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 2.1 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

//! Every texture note-data file under `docs/manual/data` should
//! parse. There is no dedicated file extension for this format (the
//! corpus uses plain `.txt`), so, unlike the `.mix`/`.wav` corpus
//! checks, the files are named explicitly here, each with the
//! `infilecnt` (sample-pitch count) its own first line actually
//! carries -- confirmed against `legacy` `texture simple`/`texture
//! motifs` (see the module doc in `cdp_data::notedata`).

use cdp_data::NoteDataFile;
use std::path::Path;

fn data_dir() -> &'static Path {
    Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/manual/data"
    ))
}

fn parse(name: &str, infilecnt: usize) -> NoteDataFile {
    let path = data_dir().join(name);
    NoteDataFile::from_file(&path, infilecnt)
        .unwrap_or_else(|e| panic!("{} failed to parse: {e}", path.display()))
}

#[test]
fn every_notedata_file_in_the_manual_corpus_parses() {
    const ONE_PITCH: &[&str] = &[
        "ndf50.txt",
        "ndf60.txt",
        "ndf62.txt",
        "ndf67.txt",
        "ndf62hs.txt",
        "ndfp_ornate1.txt",
        "ndfp_ornate2.txt",
        "ndfp_ornate2b.txt",
        "ndftim1.txt",
        "ndftimed.txt",
        "ndftmotifs.txt",
        "rhythtempl.txt",
        "tmotifsinhf.txt",
    ];
    const FOUR_PITCHES: &[&str] = &["ndfPO1.txt", "ndfPO2.txt", "ndfPO3.txt", "ndfPO4.txt"];

    for name in ONE_PITCH {
        let f = parse(name, 1);
        assert_eq!(f.sample_pitches().len(), 1, "{name}");
    }
    for name in FOUR_PITCHES {
        let f = parse(name, 4);
        assert_eq!(f.sample_pitches().len(), 4, "{name}");
    }
}

/// `docs/manual/data/ndf62hs.txt`: one pitch, one five-note harmonic
/// field motif, cross-checked against a live `legacy` `texture
/// simple` mode-1 (harmonic field) run reaching completion.
#[test]
fn ndf62hs_has_the_expected_structure() {
    let f = parse("ndf62hs.txt", 1);
    assert_eq!(f.sample_pitches(), &[62.0]);
    assert_eq!(f.motifs().len(), 1);
    assert_eq!(f.motifs()[0].len(), 5);
    assert_eq!(f.motifs()[0][2].pitch, 69.0);
}

/// `docs/manual/data/ndfPO1.txt`: four pitches, two motifs (`#2` then
/// `#7`) separated by a blank line.
#[test]
fn ndf_po1_has_the_expected_structure() {
    let f = parse("ndfPO1.txt", 4);
    assert_eq!(f.sample_pitches(), &[60.0, 60.0, 60.0, 60.0]);
    assert_eq!(f.motifs().len(), 2);
    assert_eq!(f.motifs()[0].len(), 2);
    assert_eq!(f.motifs()[1].len(), 7);
}

/// `docs/manual/data/tmotifsinhf.txt`: one pitch, three motifs (`#5`,
/// `#6`, `#11`) -- the `IS_ORN_OR_MTF` "at least" case, cross-checked
/// against a live `legacy` `texture motifs` run.
#[test]
fn tmotifsinhf_has_three_motifs() {
    let f = parse("tmotifsinhf.txt", 1);
    assert_eq!(f.motifs().len(), 3);
    assert_eq!(
        f.motifs().iter().map(|m| m.len()).collect::<Vec<_>>(),
        vec![5, 6, 11]
    );
    f.check_motif_count(1, true).unwrap();
}

/// `docs/manual/data/rhythtempl.txt`'s first note line has a trailing
/// `;quintuplet` comment after the five real fields, which legacy
/// silently ignores (see the module doc in `cdp_data::notedata`).
#[test]
fn rhythtempl_trailing_comment_after_duration_does_not_error() {
    let f = parse("rhythtempl.txt", 1);
    assert_eq!(f.motifs().len(), 1);
    assert_eq!(f.motifs()[0].len(), 11);
}
