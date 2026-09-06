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

//! Every `.mix` file under `docs/manual/data` should parse, the same
//! corpus check `cdp-sf` runs over its `.wav` files (see
//! `docs/migration/STATUS.md`).

use cdp_data::MixFile;
use std::path::{Path, PathBuf};

fn find_mix_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            find_mix_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "mix") {
            out.push(path);
        }
    }
}

#[test]
fn every_mix_file_in_the_manual_corpus_parses() {
    let data_dir = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/manual/data"
    ));
    let mut mix_files = Vec::new();
    find_mix_files(data_dir, &mut mix_files);
    assert!(
        !mix_files.is_empty(),
        "expected to find .mix files under {data_dir:?}"
    );

    for path in &mix_files {
        let text = std::fs::read_to_string(path).unwrap();
        MixFile::parse(&text).unwrap_or_else(|e| panic!("{} failed to parse: {e}", path.display()));
    }
}

/// `docs/manual/data/simplemix.mix`, cross-checked by hand against
/// the file's own content (see the module doc in `cdp_data::mix` for
/// the same file verified end to end against `legacy` `submix mix`).
#[test]
fn simplemix_example_file_has_the_expected_events() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/manual/data/simplemix.mix"
    );
    let text = std::fs::read_to_string(path).unwrap();
    let mix = MixFile::parse(&text).unwrap();

    assert_eq!(mix.len(), 5);
    assert_eq!(mix.events()[0].filename, "capm.wav");
    assert_eq!(mix.events()[0].time, 0.0);
    assert_eq!(mix.events()[0].left_pan, 0.0); // 'C'
    assert_eq!(mix.events()[4].filename, "clashmx.wav");
    assert_eq!(mix.events()[4].time, 6.7);
    assert_eq!(mix.events()[4].left_pan, 0.5);
}

/// `docs/manual/data/ws4_data/arpmajmindt.mix`, the one file in the
/// corpus using the 7-word (full stereo level+pan) line form.
#[test]
fn arpmajmindt_example_file_uses_the_seven_word_stereo_form() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/manual/data/ws4_data/arpmajmindt.mix"
    );
    let text = std::fs::read_to_string(path).unwrap();
    let mix = MixFile::parse(&text).unwrap();

    assert_eq!(mix.len(), 4);
    for ev in mix.events() {
        assert_eq!(ev.chans, 2);
        assert!(!ev.right_level.is_nan());
        assert!(!ev.right_pan.is_nan());
    }
}
