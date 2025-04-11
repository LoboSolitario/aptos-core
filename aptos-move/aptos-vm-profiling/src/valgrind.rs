// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::{bail, Result};
use std::{
    ffi::OsStr,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

pub fn profile_with_valgrind(
    command_args: impl IntoIterator<Item = impl AsRef<OsStr>>,
    stdin_data: &[u8],
    log_path: impl AsRef<Path>,
    annotation_path: impl AsRef<Path>,
) -> Result<()> {
    println!("Profiling with Valgrind...");
    let log_path = log_path.as_ref();
    let annotation_path = annotation_path.as_ref();

    println!("DEBUG: Starting valgrind profiling");
    // Run callgrind.
    let mut proc = Command::new("valgrind")
        .arg(format!(
            "--callgrind-out-file={}",
            log_path.to_string_lossy()
        ))
        .arg("--tool=callgrind")
        .arg("--dump-instr=yes")
        .arg("--collect-jumps=yes")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(command_args)
        .spawn()?;

    {   println!("DEBUG: Writing to stdin");
        let mut stdin = proc.stdin.take().unwrap();
        stdin.write_all(stdin_data)?;
    }
    println!("DEBUG: Waiting for valgrind process to complete");
    let output = proc.wait_with_output()?;
    println!("DEBUG: Valgrind process completed with status");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        bail!("Failed to run callgrind.")
    }

    // Run callgrind_annotate.
    println!("DEBUG: Running callgrind_annotate");
    let output = Command::new("callgrind_annotate")
        .arg("--threshold=100")
        .arg("--tree=both")
        .arg("--inclusive=yes")
        .arg(log_path)
        .output()?;

    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        bail!("Failed to run callgrind_annotate.")
    }

    fs::write(annotation_path, output.stdout)?;

    Ok(())
}
