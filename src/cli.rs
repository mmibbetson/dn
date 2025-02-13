// SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! CLAP-derive struct definition specifying the command line interface for dn.

use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dn",
    version = "0.1.3",
    about = "A command to manage notes following the Denote naming scheme"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new note
    New {
        /// Print the absolute path of the created note
        #[arg(
            short = 'p',
            long = "print",
            action = ArgAction::SetTrue,
        )]
        cli_print: bool,

        /// Directory in which the note will be created
        #[arg(
            short = 'd',
            long = "directory",
            value_name = "PATH",
            action = ArgAction::Set,
        )]
        cli_directory_path: Option<String>,

        /// Configuration file path
        #[arg(
            short = 'c',
            long = "config",
            value_name = "PATH",
            action = ArgAction::Set,
        )]
        cli_config_path: Option<String>,

        /// Template file to add contents to new note
        #[arg(
            short = 'T',
            long = "template",
            value_name = "PATH",
            action = ArgAction::Set,
        )]
        cli_template_path: Option<String>,

        /// Signature for the note
        #[arg(
            short = 's',
            long = "signature",
            value_name = "SIGNATURE",
            action = ArgAction::Set,
        )]
        cli_signature: Option<String>,

        /// Title for the note
        #[arg(
            short = 't',
            long = "title",
            value_name = "TITLE",
            action = ArgAction::Set,
        )]
        cli_title: Option<String>,

        /// File extension for the note
        #[arg(
            short = 'e',
            long = "extension",
            value_name = "EXTENSION",
            action = ArgAction::Set,
        )]
        cli_extension: Option<String>,

        /// Keywords for the note
        #[arg(
            short = 'k',
            long = "keywords",
            value_name = "KEYWORD(S)",
            action = ArgAction::Set,
        )]
        cli_keywords: Option<String>,
    },

    /// Rename an existing note
    Rename {
        /// Path to the input file to be renamed
        input: String,

        /// Print the absolute path of the created file
        #[arg(
            short = 'p',
            long = "print",
            action = ArgAction::SetTrue,
        )]
        cli_print: bool,

        /// Generate an identifier even if there is an existing one
        #[arg(
            short = 'I',
            long = "regenerate-identifier",
            action = ArgAction::SetTrue,
        )]
        cli_regenerate_identifier: bool,

        /// Configuration file path
        #[arg(
            short = 'c',
            long = "config",
            value_name = "PATH",
            action = ArgAction::Set,
        )]
        cli_config_path: Option<String>,

        /// New signature for the note
        #[arg(
            short = 's',
            long = "signature",
            value_name = "SIGNATURE",
            action = ArgAction::Set,
        )]
        cli_signature: Option<String>,

        /// New title for the note
        #[arg(
            short = 't',
            long = "title",
            value_name = "TITLE",
            action = ArgAction::Set,
        )]
        cli_title: Option<String>,

        /// New keywords for the note
        #[arg(
            short = 'k',
            long = "keywords",
            value_name = "KEYWORDS",
            action = ArgAction::Set,
        )]
        cli_keywords: Option<String>,

        /// Add keywords to the current or new keywords
        #[arg(
            short = 'A',
            long = "add-keywords",
            value_name = "KEYWORDS",
            action = ArgAction::Set,
        )]
        cli_add_keywords: Option<String>,

        /// Remove keywords from the current or new keywords
        #[arg(
            short = 'R',
            long = "remove-keywords",
            value_name = "KEYWORDS",
            action = ArgAction::Set,
        )]
        cli_remove_keywords: Option<String>,

        /// New file extension for the note
        #[arg(
            short = 'e',
            long = "extension",
            value_name = "EXTENSION",
            action = ArgAction::Set,
        )]
        cli_extension: Option<String>,
    },
}
