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

//! Every AIFF file under `docs/manual/sounds` should open and decode
//! without error -- the same style of corpus scan `docs/migration/
//! STATUS.md` records for this crate's `.wav` files. 10 files are
//! additionally cross-checked field-by-field against `legacy`
//! `sndinfo props`, run via the `cdp8-postmerge` Docker image.

use cdp_sf::SoundFile;
use std::path::{Path, PathBuf};

fn find_aiff_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            find_aiff_files(&path, out);
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("aiff") || ext.eq_ignore_ascii_case("aif"))
        {
            out.push(path);
        }
    }
}

#[test]
fn every_aiff_file_in_the_manual_corpus_opens_and_decodes() {
    let sounds_dir = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/manual/sounds"
    ));
    let mut files = Vec::new();
    find_aiff_files(sounds_dir, &mut files);
    assert_eq!(
        files.len(),
        120,
        "expected the 120 AIFF files this corpus is known to have under {sounds_dir:?}"
    );

    for path in &files {
        let sf = SoundFile::open(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        sf.samples_f32()
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    }
}

/// `docs/manual/sounds/ws2/tsw1-2nd.aiff`'s exact field values are
/// covered byte-for-byte in `tests/oracle_fixtures.rs`; this is the
/// broader (channel count, sample rate, sample count) cross-check
/// against a live `legacy` `sndinfo props` run for 10 further corpus
/// files, matching `cdp-sf`'s WAVE-file verification style recorded
/// in `docs/migration/STATUS.md`. Requires the `cdp8-postmerge`
/// Docker image; skipped (not failed) when Docker is unavailable, the
/// same way a CI-only check would be, since this is a cross-check
/// against an external oracle rather than a property of this crate's
/// own code.
#[test]
fn ten_corpus_files_match_legacy_sndinfo_props() {
    let sounds_dir = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/manual/sounds"
    ));
    let mut files = Vec::new();
    find_aiff_files(sounds_dir, &mut files);
    files.sort();
    let sample: Vec<&PathBuf> = files.iter().step_by(files.len() / 10).take(10).collect();

    for path in sample {
        let sf = SoundFile::open(path).unwrap();
        let rel = path.strip_prefix(sounds_dir).unwrap();
        let output = std::process::Command::new("docker")
            .args([
                "run",
                "--rm",
                "-v",
                &format!("{}:/m:ro", sounds_dir.display()),
                "cdp8-postmerge:latest",
                "sndinfo",
                "props",
                &format!("/m/{}", rel.display()),
            ])
            .output();
        let Ok(output) = output else {
            eprintln!("docker unavailable, skipping live cross-check");
            return;
        };
        if !output.status.success() {
            eprintln!("docker run failed, skipping live cross-check");
            return;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);

        let legacy_channels: u16 = stdout
            .lines()
            .find_map(|l| l.strip_prefix("channels: ..........."))
            .unwrap_or_else(|| panic!("{}: no channels line in sndinfo output", path.display()))
            .trim()
            .parse()
            .unwrap();
        let legacy_rate: u32 = stdout
            .lines()
            .find_map(|l| l.strip_prefix("sample rate: ........"))
            .unwrap_or_else(|| panic!("{}: no sample rate line", path.display()))
            .trim()
            .parse()
            .unwrap();
        let legacy_samples: u64 = stdout
            .lines()
            .find_map(|l| l.strip_prefix("samples: ............"))
            .unwrap_or_else(|| panic!("{}: no samples line", path.display()))
            .trim()
            .parse()
            .unwrap();

        assert_eq!(sf.fmt.channels, legacy_channels, "{}", path.display());
        assert_eq!(sf.fmt.sample_rate, legacy_rate, "{}", path.display());
        assert_eq!(sf.sample_count(), legacy_samples, "{}", path.display());
    }
}
