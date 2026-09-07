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

//! Every CDP-derived analysis-family file (`.ana`, `.evl`, `.frq`,
//! `.trn`) under `docs/manual` should open and be classified as
//! something other than plain [`FileKind::Wave`] -- these all carry
//! the `float32`-plus-property-block layout `crate::props::
//! detect_file_kind` decodes.

use cdp_sf::{FileKind, SoundFile};
use std::path::{Path, PathBuf};

fn find_analysis_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            find_analysis_files(&path, out);
        } else if path.extension().is_some_and(|ext| {
            ["ana", "evl", "frq", "trn"]
                .iter()
                .any(|e| ext.eq_ignore_ascii_case(e))
        }) {
            out.push(path);
        }
    }
}

#[test]
fn every_analysis_family_file_in_the_manual_corpus_is_classified() {
    let manual_dir = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/manual"));
    let mut files = Vec::new();
    find_analysis_files(manual_dir, &mut files);
    assert_eq!(
        files.len(),
        91,
        "expected the 91 analysis-family files this corpus is known to have under {manual_dir:?}"
    );

    for path in &files {
        let sf = SoundFile::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert_ne!(
            sf.file_kind,
            FileKind::Wave,
            "{}: expected an analysis-family FileKind, not Wave",
            path.display()
        );
    }
}
