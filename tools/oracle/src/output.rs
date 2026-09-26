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

//! Output-file comparison (section 6.1 of `docs/migration/PLAN-V2.md`).
//!
//! Gate 3 began by comparing the exit code, standard output and standard
//! error of a run. That is enough to catch a command that rejects its own
//! command line, which is what five of the seven known defects do. It is
//! not enough for a command whose real product is a file.
//!
//! This module records what a run left in the sandbox and compares it.
//! Files are found by taking a snapshot before the run and another after,
//! so a case never lists its outputs. That matters because several legacy
//! commands choose their own names (`housekeep chans` writes
//! `marimba_c1.wav`, `housekeep copy` mode 2 writes `in_001.wav`), and
//! because a command that writes a file it should not write is itself a
//! defect worth failing on. Deletions are recorded too, since
//! `housekeep remove` exists to delete files.
//!
//! ## What is deliberately not recorded
//!
//! Two fields in a legacy sound file hold the wall-clock time of the run:
//! the `DATE` property and the `PEAK` chunk timestamp. Both change between
//! two runs of the same command, so neither is recorded. Property *names*
//! are recorded, because presence is deterministic and meaningful:
//! `housekeep copy` is confirmed to drop `marimba.wav`'s own `maxamp`
//! properties and to add a fresh `DATE`.
//!
//! ## Why a fingerprint rather than the samples
//!
//! Storing every sample would make the repository large and the diffs
//! unreadable. A SHA-256 of the sample data answers "is this identical",
//! which is the common case. When it is not identical, the tolerance rules
//! of PLAN.md decision D4 apply, because the legacy build uses
//! `-ffast-math` and bit-exactness is not a goal. Those rules need
//! numbers, so the fingerprint keeps a fixed-size summary: the first and
//! last samples, the extremes with their positions, and the
//! root-mean-square value of each of 64 equal blocks. All of it stays
//! small enough to read in a diff.

use cdp_sf::SoundFile;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

/// How many leading and trailing samples the fingerprint keeps.
const EDGE_SAMPLES: usize = 8;
/// How many equal blocks the fingerprint measures.
const RMS_BLOCKS: usize = 64;
/// Text output stored in full up to this size. Larger text is summarised,
/// because `sndinfo prntsnd`'s own usage text warns "CARE!!! large
/// quantities of data."
const TEXT_VERBATIM_LIMIT: usize = 4096;

/// The result of comparing two runs' output files.
///
/// Failures and notes are kept apart because a tolerated difference is
/// worth printing and not worth failing on.
#[derive(Debug, Default)]
pub struct Comparison {
    pub failures: Vec<String>,
    pub notes: Vec<String>,
}

/// What one run left behind, keyed by path inside the sandbox.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Outputs {
    /// Files the run created.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<Output>,
    /// Input files the run changed in place.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<Output>,
    /// Input files the run deleted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<String>,
}

/// One file a run produced or changed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Output {
    pub name: String,
    /// `sound`, `text` or `binary`.
    pub kind: String,
    pub byte_count: u64,
    /// SHA-256 of the whole file for `text` and `binary`, and of the
    /// decoded sample data for `sound`. Hashing samples rather than bytes
    /// for a sound file keeps the hash independent of the header's
    /// timestamp fields.
    pub sha256: String,

    // Sound fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<u64>,
    /// Property names only. See this module's doc for why not values.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub property_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peak_values: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peak_positions: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub first_samples: Vec<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub last_samples: Vec<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_index: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_index: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub block_rms: Vec<f64>,

    // Text fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_count: Option<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub first_lines: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub last_lines: Vec<String>,
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn hash_bytes(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    hex(&h.finalize())
}

/// Every regular file under `dir`, as a relative path mapped to a hash of
/// its bytes. Used to tell created, modified and deleted files apart.
pub fn snapshot(dir: &Path) -> Result<BTreeMap<String, String>, String> {
    let mut found = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(current) = stack.pop() {
        let entries = std::fs::read_dir(&current)
            .map_err(|e| format!("cannot read {}: {e}", current.display()))?;
        for entry in entries {
            let path = entry
                .map_err(|e| format!("cannot read an entry of {}: {e}", current.display()))?
                .path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let relative = path
                .strip_prefix(dir)
                .map_err(|e| format!("{} is not under {}: {e}", path.display(), dir.display()))?
                .to_string_lossy()
                .replace('\\', "/");
            let bytes =
                std::fs::read(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            found.insert(relative, hash_bytes(&bytes));
        }
    }
    Ok(found)
}

/// Compares two snapshots of one sandbox and describes what changed.
pub fn collect(
    dir: &Path,
    before: &BTreeMap<String, String>,
    after: &BTreeMap<String, String>,
) -> Result<Outputs, String> {
    let mut outputs = Outputs::default();
    for (name, hash) in after {
        match before.get(name) {
            None => outputs.created.push(describe(dir, name)?),
            Some(old) if old != hash => outputs.modified.push(describe(dir, name)?),
            Some(_) => {}
        }
    }
    for name in before.keys() {
        if !after.contains_key(name) {
            outputs.deleted.push(name.clone());
        }
    }
    Ok(outputs)
}

/// Reads one produced file and records it. A file that `cdp-sf` can open
/// is recorded as sound, valid UTF-8 as text, and anything else as binary.
fn describe(dir: &Path, name: &str) -> Result<Output, String> {
    let path = dir.join(name);
    let bytes = std::fs::read(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;

    if let Ok(sound) = SoundFile::open(&path)
        && let Ok(samples) = sound.samples_f32()
    {
        return Ok(describe_sound(name, bytes.len() as u64, &sound, &samples));
    }
    match String::from_utf8(bytes.clone()) {
        Ok(text) => Ok(describe_text(name, &text)),
        Err(_) => Ok(Output {
            name: name.to_string(),
            kind: "binary".to_string(),
            byte_count: bytes.len() as u64,
            sha256: hash_bytes(&bytes),
            ..blank()
        }),
    }
}

fn describe_sound(name: &str, byte_count: u64, sound: &SoundFile, samples: &[f32]) -> Output {
    // Hash the decoded samples, not the file bytes, so that the header's
    // DATE property and PEAK timestamp cannot affect the result.
    let mut hasher = Sha256::new();
    for sample in samples {
        hasher.update(sample.to_le_bytes());
    }

    let (mut min_value, mut min_index) = (f64::INFINITY, 0u64);
    let (mut max_value, mut max_index) = (f64::NEG_INFINITY, 0u64);
    for (i, &sample) in samples.iter().enumerate() {
        let value = f64::from(sample);
        if value < min_value {
            min_value = value;
            min_index = i as u64;
        }
        if value > max_value {
            max_value = value;
            max_index = i as u64;
        }
    }

    let edge = EDGE_SAMPLES.min(samples.len());
    Output {
        name: name.to_string(),
        kind: "sound".to_string(),
        byte_count,
        sha256: hex(&hasher.finalize()),
        channels: Some(sound.fmt.channels),
        sample_rate: Some(sound.fmt.sample_rate),
        sample_type: Some(format!("{:?}", sound.fmt.sample_type)),
        file_kind: Some(format!("{:?}", sound.file_kind)),
        sample_count: Some(samples.len() as u64),
        property_names: sound
            .properties
            .names()
            .into_iter()
            .map(str::to_string)
            .collect(),
        peak_values: sound.peaks.iter().map(|p| f64::from(p.value)).collect(),
        peak_positions: sound.peaks.iter().map(|p| p.position).collect(),
        first_samples: samples[..edge].iter().map(|&s| f64::from(s)).collect(),
        last_samples: samples[samples.len() - edge..]
            .iter()
            .map(|&s| f64::from(s))
            .collect(),
        min_value: samples.is_empty().then_some(0.0).or(Some(min_value)),
        min_index: Some(min_index),
        max_value: samples.is_empty().then_some(0.0).or(Some(max_value)),
        max_index: Some(max_index),
        block_rms: block_rms(samples),
        ..blank()
    }
}

/// The root-mean-square value of each of [`RMS_BLOCKS`] equal blocks. A
/// fixed block count keeps the fingerprint the same size whatever the
/// file's length. A file with fewer samples than blocks yields one value
/// per sample and no more.
fn block_rms(samples: &[f32]) -> Vec<f64> {
    if samples.is_empty() {
        return Vec::new();
    }
    let blocks = RMS_BLOCKS.min(samples.len());
    (0..blocks)
        .map(|b| {
            let start = b * samples.len() / blocks;
            let end = ((b + 1) * samples.len() / blocks).max(start + 1);
            let block = &samples[start..end.min(samples.len())];
            let sum: f64 = block.iter().map(|&s| f64::from(s) * f64::from(s)).sum();
            (sum / block.len() as f64).sqrt()
        })
        .collect()
}

fn describe_text(name: &str, text: &str) -> Output {
    let lines: Vec<&str> = text.lines().collect();
    let small = text.len() <= TEXT_VERBATIM_LIMIT;
    Output {
        name: name.to_string(),
        kind: "text".to_string(),
        byte_count: text.len() as u64,
        sha256: hash_bytes(text.as_bytes()),
        content: small.then(|| text.to_string()),
        line_count: Some(lines.len()),
        first_lines: if small {
            Vec::new()
        } else {
            lines.iter().take(5).map(|l| l.to_string()).collect()
        },
        last_lines: if small {
            Vec::new()
        } else {
            lines
                .iter()
                .rev()
                .take(5)
                .rev()
                .map(|l| l.to_string())
                .collect()
        },
        ..blank()
    }
}

fn blank() -> Output {
    Output {
        name: String::new(),
        kind: String::new(),
        byte_count: 0,
        sha256: String::new(),
        channels: None,
        sample_rate: None,
        sample_type: None,
        file_kind: None,
        sample_count: None,
        property_names: Vec::new(),
        peak_values: Vec::new(),
        peak_positions: Vec::new(),
        first_samples: Vec::new(),
        last_samples: Vec::new(),
        min_value: None,
        min_index: None,
        max_value: None,
        max_index: None,
        block_rms: Vec::new(),
        content: None,
        line_count: None,
        first_lines: Vec::new(),
        last_lines: Vec::new(),
    }
}

/// Compares what legacy produced against what the Rust build produced.
///
/// Returns one message per disagreement. An empty result means the two
/// runs left the same files behind.
pub fn compare(expected: &Outputs, observed: &Outputs, tolerance: Option<f64>) -> Comparison {
    let mut out = Comparison::default();
    compare_set(
        "created",
        &expected.created,
        &observed.created,
        tolerance,
        &mut out,
    );
    compare_set(
        "modified",
        &expected.modified,
        &observed.modified,
        tolerance,
        &mut out,
    );

    if expected.deleted != observed.deleted {
        out.failures.push(format!(
            "deleted files differ: legacy deleted {:?}, this build deleted {:?}",
            expected.deleted, observed.deleted
        ));
    }
    out
}

fn compare_set(
    label: &str,
    expected: &[Output],
    observed: &[Output],
    tolerance: Option<f64>,
    out: &mut Comparison,
) {
    let names = |files: &[Output]| files.iter().map(|f| f.name.clone()).collect::<Vec<_>>();
    let (want, got) = (names(expected), names(observed));
    if want != got {
        out.failures.push(format!(
            "{label} files differ: legacy left {want:?}, this build left {got:?}"
        ));
        return;
    }
    for (want, got) in expected.iter().zip(observed) {
        compare_one(want, got, tolerance, out);
    }
}

fn compare_one(want: &Output, got: &Output, tolerance: Option<f64>, out: &mut Comparison) {
    let name = &want.name;
    if want.kind != got.kind {
        out.failures.push(format!(
            "{name}: legacy wrote a {} file, this build wrote a {} file",
            want.kind, got.kind
        ));
        return;
    }

    match want.kind.as_str() {
        "sound" => compare_sound(want, got, tolerance, out),
        "text" => compare_text(want, got, out),
        _ => {
            if want.sha256 != got.sha256 {
                out.failures.push(format!(
                    "{name}: contents differ ({} bytes against legacy's {})",
                    got.byte_count, want.byte_count
                ));
            }
        }
    }
}

fn compare_sound(want: &Output, got: &Output, tolerance: Option<f64>, out: &mut Comparison) {
    let name = &want.name;

    // Header fields must match exactly. A tolerance has no meaning here.
    let mut header = Vec::new();
    if want.channels != got.channels {
        header.push(format!(
            "channels {:?} against {:?}",
            got.channels, want.channels
        ));
    }
    if want.sample_rate != got.sample_rate {
        header.push(format!(
            "sample rate {:?} against {:?}",
            got.sample_rate, want.sample_rate
        ));
    }
    if want.sample_type != got.sample_type {
        header.push(format!(
            "sample type {:?} against {:?}",
            got.sample_type, want.sample_type
        ));
    }
    if want.file_kind != got.file_kind {
        header.push(format!(
            "file kind {:?} against {:?}",
            got.file_kind, want.file_kind
        ));
    }
    if want.sample_count != got.sample_count {
        header.push(format!(
            "sample count {:?} against {:?}",
            got.sample_count, want.sample_count
        ));
    }
    if !header.is_empty() {
        out.failures
            .push(format!("{name}: header differs: {}", header.join(", ")));
    }
    if want.property_names != got.property_names {
        out.failures.push(format!(
            "{name}: property names are {:?}, legacy wrote {:?}",
            got.property_names, want.property_names
        ));
    }

    // Identical sample data needs no further checking.
    if want.sha256 == got.sha256 {
        return;
    }

    // Sample data differs. Exact is the default, and a case must opt in to
    // a tolerance and say why, which is what PLAN.md section 5 requires:
    // "Tolerance rules are per case and can be tightened. A case that only
    // passes at a wider tolerance must say why in its notes."
    //
    // Exact by default is not merely plan-compliance, it is necessary.
    // PLAN.md decision D4 suggests 1e-4 for 16-bit output, but one
    // least-significant bit of a 16-bit sample is 1/32767, about 3.05e-5,
    // so 1e-4 permits roughly three bits of error. The rounding bug that
    // WP-2.2's fifth slice found in `cdp_sf::writer::encode_pcm16` moved a
    // sample by exactly one bit. A blanket D4 tolerance would have hidden
    // it. Confirmed by reintroducing that bug against these cases: the
    // difference came out at 3.0518509447574615e-5, inside D4 and plainly
    // a real defect.
    let mut differences = Vec::new();
    let mut worst = 0.0f64;

    let mut check = |label: &str, a: &[f64], b: &[f64], into: &mut Vec<String>| {
        if a.len() != b.len() {
            into.push(format!(
                "{label} has {} values against {}",
                b.len(),
                a.len()
            ));
            return;
        }
        for (i, (x, y)) in a.iter().zip(b).enumerate() {
            let difference = (x - y).abs();
            if difference > worst {
                worst = difference;
            }
            if difference > 0.0 {
                into.push(format!(
                    "{label}[{i}] is {y} against legacy's {x}, a difference of {difference:e}"
                ));
            }
        }
    };
    check(
        "first_samples",
        &want.first_samples,
        &got.first_samples,
        &mut differences,
    );
    check(
        "last_samples",
        &want.last_samples,
        &got.last_samples,
        &mut differences,
    );
    check(
        "block_rms",
        &want.block_rms,
        &got.block_rms,
        &mut differences,
    );

    for (label, a, b) in [
        ("min_value", want.min_value, got.min_value),
        ("max_value", want.max_value, got.max_value),
    ] {
        if let (Some(x), Some(y)) = (a, b) {
            let difference = (x - y).abs();
            if difference > worst {
                worst = difference;
            }
            if difference > 0.0 {
                differences.push(format!(
                    "{label} is {y} against legacy's {x}, a difference of {difference:e}"
                ));
            }
        }
    }

    // Peak positions and extreme positions are indices, so they match or
    // they do not. A moved peak means the samples really moved, whatever
    // the tolerance says.
    let mut moved = Vec::new();
    if want.peak_positions != got.peak_positions {
        moved.push(format!(
            "peak positions are {:?}, legacy wrote {:?}",
            got.peak_positions, want.peak_positions
        ));
    }
    if want.max_index != got.max_index {
        moved.push(format!(
            "the loudest sample is at {:?}, legacy put it at {:?}",
            got.max_index, want.max_index
        ));
    }

    match tolerance {
        Some(limit) if worst <= limit && moved.is_empty() => out.notes.push(format!(
            "{name}: sample data is not identical but stays inside the case's \
             declared tolerance of {limit:e} (largest difference {worst:e})"
        )),
        Some(limit) => out.failures.push(format!(
            "{name}: sample data differs beyond the case's declared tolerance of \
             {limit:e} (largest difference {worst:e}):\n    {}",
            moved
                .into_iter()
                .chain(differences.into_iter().take(6))
                .collect::<Vec<_>>()
                .join("\n    ")
        )),
        None => out.failures.push(format!(
            "{name}: sample data differs (largest difference {worst:e}). The case \
             declares no tolerance, so the samples must match exactly. If this \
             difference is acceptable, set sample_tolerance and tolerance_reason \
             on the case.\n    {}",
            moved
                .into_iter()
                .chain(differences.into_iter().take(6))
                .collect::<Vec<_>>()
                .join("\n    ")
        )),
    }
}

fn compare_text(want: &Output, got: &Output, out: &mut Comparison) {
    let name = &want.name;
    if want.sha256 == got.sha256 {
        return;
    }
    match (&want.content, &got.content) {
        (Some(a), Some(b)) => {
            let (al, bl): (Vec<_>, Vec<_>) = (a.lines().collect(), b.lines().collect());
            for i in 0..al.len().max(bl.len()) {
                if al.get(i) != bl.get(i) {
                    out.failures.push(format!(
                        "{name}: first difference at line {}:\n      actual  : {:?}\n      expected: {:?}\n      ({} lines against legacy's {})",
                        i + 1,
                        bl.get(i).unwrap_or(&"<end of file>"),
                        al.get(i).unwrap_or(&"<end of file>"),
                        bl.len(),
                        al.len()
                    ));
                    return;
                }
            }
            out.failures.push(format!(
                "{name}: lines match but bytes differ ({} against legacy's {})",
                got.byte_count, want.byte_count
            ));
        }
        _ => out.failures.push(format!(
            "{name}: contents differ, {} bytes and {:?} lines against legacy's {} bytes and {:?} lines",
            got.byte_count, got.line_count, want.byte_count, want.line_count
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::run::Sandbox;

    #[test]
    fn block_rms_returns_one_value_per_block() {
        let samples = vec![0.5f32; 1024];
        let rms = block_rms(&samples);
        assert_eq!(rms.len(), RMS_BLOCKS);
        for value in rms {
            assert!(
                (value - 0.5).abs() < 1e-6,
                "constant signal gives its own level"
            );
        }
    }

    #[test]
    fn block_rms_never_exceeds_the_sample_count() {
        // A file shorter than the block count yields one value per sample.
        assert_eq!(block_rms(&[1.0, 1.0, 1.0]).len(), 3);
        assert!(block_rms(&[]).is_empty());
    }

    #[test]
    fn block_rms_separates_a_loud_half_from_a_silent_half() {
        let mut samples = vec![0.0f32; 512];
        samples.extend(std::iter::repeat_n(1.0f32, 512));
        let rms = block_rms(&samples);
        assert_eq!(rms.len(), RMS_BLOCKS);
        assert!(rms[0] < 1e-6, "first block is silent");
        assert!(
            (rms[RMS_BLOCKS - 1] - 1.0).abs() < 1e-6,
            "last block is loud"
        );
    }

    #[test]
    fn a_small_text_file_is_stored_in_full() {
        let out = describe_text("o.txt", "one\ntwo\n");
        assert_eq!(out.kind, "text");
        assert_eq!(out.content.as_deref(), Some("one\ntwo\n"));
        assert_eq!(out.line_count, Some(2));
        assert!(out.first_lines.is_empty(), "no summary is needed");
    }

    #[test]
    fn a_large_text_file_is_summarised_rather_than_stored() {
        let big = (0..2000).map(|i| format!("line {i}\n")).collect::<String>();
        let out = describe_text("o.txt", &big);
        assert!(out.byte_count > TEXT_VERBATIM_LIMIT as u64);
        assert!(out.content.is_none(), "the content is not stored");
        assert_eq!(out.first_lines.len(), 5);
        assert_eq!(out.last_lines.len(), 5);
        assert_eq!(out.first_lines[0], "line 0");
        assert_eq!(out.last_lines[4], "line 1999");
    }

    #[test]
    fn collect_tells_created_modified_and_deleted_apart() {
        let sandbox = Sandbox::new("output-test").expect("sandbox");
        let dir = sandbox.path();
        std::fs::write(dir.join("kept.txt"), "same").unwrap();
        std::fs::write(dir.join("changed.txt"), "before").unwrap();
        std::fs::write(dir.join("gone.txt"), "doomed").unwrap();
        let before = snapshot(dir).expect("before");

        std::fs::write(dir.join("changed.txt"), "after").unwrap();
        std::fs::remove_file(dir.join("gone.txt")).unwrap();
        std::fs::write(dir.join("new.txt"), "fresh").unwrap();
        let after = snapshot(dir).expect("after");

        let outputs = collect(dir, &before, &after).expect("collect");
        assert_eq!(
            outputs.created.iter().map(|o| &o.name).collect::<Vec<_>>(),
            ["new.txt"]
        );
        assert_eq!(
            outputs.modified.iter().map(|o| &o.name).collect::<Vec<_>>(),
            ["changed.txt"]
        );
        assert_eq!(outputs.deleted, ["gone.txt"]);
    }

    /// Exact comparison is the default. A case must opt in to a tolerance,
    /// because one bit of a 16-bit sample is about 3.05e-5 and PLAN.md
    /// decision D4's 1e-4 would hide a real rounding defect.
    #[test]
    fn a_one_bit_sample_difference_fails_without_a_declared_tolerance() {
        let legacy = sound_fixture(0.5);
        let ours = sound_fixture(0.5 + 1.0 / 32767.0);
        let result = compare(&legacy, &ours, None);
        assert!(
            result
                .failures
                .iter()
                .any(|f| f.contains("must match exactly")),
            "expected an exact-match failure, got {:?}",
            result.failures
        );
        assert!(result.notes.is_empty());
    }

    #[test]
    fn the_same_difference_is_a_note_when_the_case_declares_a_tolerance() {
        let legacy = sound_fixture(0.5);
        let ours = sound_fixture(0.5 + 1.0 / 32767.0);
        let result = compare(&legacy, &ours, Some(1e-4));
        assert!(result.failures.is_empty(), "got {:?}", result.failures);
        assert_eq!(result.notes.len(), 1);
        assert!(result.notes[0].contains("declared tolerance"));
    }

    #[test]
    fn a_difference_beyond_the_declared_tolerance_still_fails() {
        let legacy = sound_fixture(0.5);
        let ours = sound_fixture(0.6);
        let result = compare(&legacy, &ours, Some(1e-4));
        assert!(
            result.failures.iter().any(|f| f.contains("beyond")),
            "got {:?}",
            result.failures
        );
    }

    #[test]
    fn a_header_difference_fails_whatever_the_tolerance() {
        let legacy = sound_fixture(0.5);
        let mut ours = sound_fixture(0.5);
        ours.created[0].channels = Some(2);
        let result = compare(&legacy, &ours, Some(1.0));
        assert!(
            result.failures.iter().any(|f| f.contains("header differs")),
            "got {:?}",
            result.failures
        );
    }

    /// One recorded sound output holding a constant sample value, enough
    /// to exercise the comparison without touching the filesystem.
    fn sound_fixture(level: f64) -> Outputs {
        Outputs {
            created: vec![Output {
                name: "out.wav".to_string(),
                kind: "sound".to_string(),
                byte_count: 100,
                sha256: format!("hash-of-{level}"),
                channels: Some(1),
                sample_rate: Some(44100),
                sample_type: Some("Short16".to_string()),
                file_kind: Some("Wave".to_string()),
                sample_count: Some(64),
                first_samples: vec![level; 8],
                last_samples: vec![level; 8],
                min_value: Some(level),
                min_index: Some(0),
                max_value: Some(level),
                max_index: Some(0),
                block_rms: vec![level; 64],
                ..blank()
            }],
            ..Outputs::default()
        }
    }
}
