// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clap::Parser;
use once_cell::sync::Lazy;
use std::path::Path;

mod profile_aptos_vm;
mod profile_move_vm;
mod valgrind;

const BUILD_PROFILE: &str = "release";
static PATH_CRATE_ROOT: Lazy<&Path> = Lazy::new(|| Path::new(env!("CARGO_MANIFEST_DIR")));

#[derive(Parser, Debug)]
struct Args {
    #[clap(short = 'r', long)]
    regenerate_all: bool,
    
    #[clap(short = 'p', long, help = "Path to the Move code directory")]
    move_dir: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    profile_move_vm::run(args.regenerate_all, args.move_dir.as_deref())?;
    profile_aptos_vm::run()?;

    Ok(())
}

#[test]
fn verify_tool() {
    use clap::CommandFactory;
    Args::command().debug_assert()
}
