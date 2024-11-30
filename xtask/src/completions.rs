// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{fs, path::Path};

use dn::cli::Cli;

use clap::CommandFactory;
use clap_complete::{
    aot::{Bash, Elvish, Fish, PowerShell, Zsh},
    generate_to,
};
use clap_complete_nushell::Nushell;

/// Generate shell completion files and write them to the "completions" directory.
pub fn gen() {
    let base_dir = Path::new("");
    let completions_dir = base_dir.join("completions");

    fs::create_dir_all(&completions_dir).unwrap();
    println!("Generating shell completions...");

    let mut cmd = Cli::command();
    generate_to(Bash, &mut cmd, "dn", &completions_dir).unwrap();
    generate_to(Zsh, &mut cmd, "dn", &completions_dir).unwrap();
    generate_to(Fish, &mut cmd, "dn", &completions_dir).unwrap();
    generate_to(Elvish, &mut cmd, "dn", &completions_dir).unwrap();
    generate_to(Nushell, &mut cmd, "dn", &completions_dir).unwrap();
    generate_to(PowerShell, &mut cmd, "dn", &completions_dir).unwrap();
    println!("Finished!");
}
