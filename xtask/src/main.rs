// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    env,
    path::{Path, PathBuf},
};

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

fn main() {
    let Cli { command } = Cli::parse();

    env::set_current_dir(project_root()).unwrap();

    match command {
        Commands::Completions => completions::gen(),
        Commands::Manpages => manpages::gen(),
    }
}

fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}
