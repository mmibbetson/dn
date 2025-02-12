// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{fs, path::Path};

use dn_lib::cli::Cli;

use clap::CommandFactory;
use clap_mangen::generate_to;

/// Generate man page files and write them to the "completions" directory.
pub fn gen() {
    let man_dir = Path::new("").join("man");

    fs::create_dir_all(&man_dir).unwrap();
    println!("Generating manpages...");

    generate_to(Cli::command(), &man_dir).unwrap();
    println!("Finished!");
}
