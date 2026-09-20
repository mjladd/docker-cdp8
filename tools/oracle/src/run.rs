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

//! Running one case, either against the legacy image or against this
//! repository's Rust build.
//!
//! Both runs use a fresh sandbox directory as their working directory,
//! with the case's inputs copied in under the names the command uses. A
//! command that writes an output file therefore writes it inside the
//! sandbox, where the caller can inspect it, and never beside the
//! corpus.

use crate::case::{Case, Expected};
use std::path::{Path, PathBuf};
use std::process::Command;

/// A working directory for one run, removed when dropped.
pub struct Sandbox {
    path: PathBuf,
}

impl Sandbox {
    pub fn new(label: &str) -> Result<Self, String> {
        // A counter keeps two sandboxes in one process distinct, since
        // the process id alone does not.
        use std::sync::atomic::{AtomicU32, Ordering};
        static NEXT: AtomicU32 = AtomicU32::new(0);
        let unique = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "cdp-oracle-{}-{}-{unique}",
            std::process::id(),
            label.replace('/', "-")
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("cannot create sandbox {}: {e}", path.display()))?;
        Ok(Sandbox { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Copies a case's inputs in under the names the command uses.
    pub fn populate(&self, case: &Case, repo_root: &Path) -> Result<(), String> {
        for input in &case.inputs {
            let from = repo_root.join(&input.from);
            let to = self.path.join(&input.name);
            if let Some(parent) = to.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
            }
            std::fs::copy(&from, &to)
                .map_err(|e| format!("cannot copy {} to {}: {e}", from.display(), to.display()))?;
        }
        Ok(())
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

fn capture(output: std::process::Output) -> Expected {
    Expected {
        // A process killed by a signal has no code. -1 marks that, and
        // no legacy run is expected to produce it.
        exit_code: output.status.code().unwrap_or(-1),
        stdout: terminal_visible(&String::from_utf8_lossy(&output.stdout)),
        stderr: terminal_visible(&String::from_utf8_lossy(&output.stderr)),
    }
}

/// Reduces a captured stream to what a terminal would show, by keeping
/// only the text after the last carriage return on each line.
///
/// This is needed, not a convenience. legacy reports progress with
/// `display_virtual_time` (`legacy/dev/cdp2k/writedata.c`), which prints
/// `"\r%d min %5.2lf sec"` once per internal write buffer and no newline.
/// Each tick overwrites the one before it on screen, so only the last one
/// is ever visible.
///
/// The number of ticks is not reproducible. The buffer size comes from
/// `create_sndbufs`' `Malloc(-1)` (`legacy/dev/cdp2k/tklib3.c`), which
/// asks for the largest free block of memory at allocation time, so it
/// depends on the machine. This was found the hard way: two `housekeep
/// copy` cases recorded on a workstation drifted when the `golden-drift`
/// job re-recorded them on a continuous-integration runner, while all 17
/// cases with no progress output stayed identical.
///
/// Keeping the last tick also matches the porting decision already taken
/// for the Rust side, which prints one tick with the final sample count
/// rather than legacy's several (see `housekeep::print_virtual_time`).
fn terminal_visible(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for (i, line) in text.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        match line.rfind('\r') {
            Some(at) => out.push_str(&line[at + 1..]),
            None => out.push_str(line),
        }
    }
    out
}

/// Runs a case against the legacy image. Needs Docker.
///
/// `--user` keeps any file the command writes owned by the caller rather
/// than by root, so that the sandbox can be read and removed afterwards.
pub fn run_legacy(case: &Case, sandbox: &Sandbox, image: &str) -> Result<Expected, String> {
    let ids = user_and_group()?;
    let mut command = Command::new("docker");
    command
        .arg("run")
        .arg("--rm")
        .arg("--user")
        .arg(&ids)
        .arg("-v")
        .arg(format!("{}:/w", sandbox.path().display()))
        .arg("-w")
        .arg("/w")
        .arg(image)
        .arg(&case.program)
        .args(&case.argv);

    let output = command
        .output()
        .map_err(|e| format!("cannot run docker: {e}. Is Docker installed and running?"))?;
    Ok(capture(output))
}

fn user_and_group() -> Result<String, String> {
    let read = |flag: &str| -> Result<String, String> {
        let out = Command::new("id")
            .arg(flag)
            .output()
            .map_err(|e| format!("cannot run id {flag}: {e}"))?;
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    };
    Ok(format!("{}:{}", read("-u")?, read("-g")?))
}

/// Runs a case against this repository's Rust build. Needs no Docker.
///
/// A sub-command whose program has a launcher binary is invoked through
/// it, exactly as a user would. One without is invoked through `cdp`.
pub fn run_rust(case: &Case, sandbox: &Sandbox, bin_dir: &Path) -> Result<Expected, String> {
    let entry = case.registry_entry()?;
    let (binary, leading) = match entry.launcher {
        Some(name) => (bin_dir.join(name), Vec::new()),
        None => (bin_dir.join("cdp"), vec![case.program.clone()]),
    };

    if !binary.exists() {
        return Err(format!(
            "{} does not exist. Build it first: cargo build --workspace",
            binary.display()
        ));
    }

    let output = Command::new(&binary)
        .args(&leading)
        .args(&case.argv)
        .current_dir(sandbox.path())
        .output()
        .map_err(|e| format!("cannot run {}: {e}", binary.display()))?;
    Ok(capture(output))
}

#[cfg(test)]
mod tests {
    use super::terminal_visible;

    #[test]
    fn keeps_text_with_no_carriage_return_unchanged() {
        assert_eq!(terminal_visible("a\nb\n"), "a\nb\n");
    }

    #[test]
    fn keeps_only_the_last_tick_of_a_progress_line() {
        // What legacy prints for a file large enough for several buffers.
        let raw = "\r0 min  0.50 sec\r0 min  1.00 sec\n\n";
        assert_eq!(terminal_visible(raw), "0 min  1.00 sec\n\n");
    }

    #[test]
    fn strips_the_leading_carriage_return_of_a_single_tick() {
        assert_eq!(
            terminal_visible("\r0 min  1.00 sec\n\n"),
            "0 min  1.00 sec\n\n"
        );
    }

    #[test]
    fn treats_each_line_on_its_own_and_discards_overwritten_text() {
        // "keep" sits before a carriage return on its line, so a terminal
        // overwrites it. Only the text after the last return survives.
        assert_eq!(terminal_visible("\ra\nkeep\r\rb\n"), "a\nb\n");
    }
}
