// SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{env, error::Error};

use clap::{Parser, Subcommand};

mod completions;
mod manpages;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completion files
    Completions,
    /// Generate man page files
    Manpages,
}

pub fn print_help() {
    println!(
        "
Usage: Run with `cargo xtask <task>`, eg. `cargo xtask manpages`.

Tasks:
    completions: Generate shell completion files.
    manpages: Generate manual pages.
"
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let task = env::args().nth(2);
    match task {
        None => print_help(),
        Some(t) => match t.as_str() {
            "completions" => completions::gen(),
            "manpages" => manpages::gen(),
            invalid => return Err(format!("Invalid task name: {}", invalid).into()),
        },
    };

    Ok(())
}
