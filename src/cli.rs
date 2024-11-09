//! TODO

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "dn",
    version = "0.1.1",
    about = "A command to manage notes following the Denote naming scheme."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Display help information
    #[arg(short, long)]
    pub help: bool,

    /// Display version information
    #[arg(short, long)]
    pub version: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new note
    New {
        /// Print the absolute path of the created note
        #[arg(short = 'p', long = "print")]
        cli_print: bool,

        /// Generate frontmatter
        #[arg(short = 'G', long = "generate-frontmatter")]
        cli_generate_frontmatter: bool,

        /// Directory in which the note will be created
        #[arg(short = 'd', long = "directory", value_name = "PATH")]
        cli_directory_path: Option<String>,

        /// Configuration file path
        #[arg(short = 'c', long = "config", value_name = "PATH")]
        cli_config_path: Option<String>,

        /// Template file to add contents to new note
        #[arg(short = 'T', long = "template", value_name = "PATH")]
        cli_template_path: Option<String>,

        /// Frontmatter format
        #[arg(
            short = 'F',
            long = "frontmatter-format",
            value_name = "text|yaml|toml|org"
        )]
        cli_frontmatter_format: Option<String>,

        /// Signature for the note
        #[arg(short = 's', long = "signature", value_name = "SIGNATURE")]
        cli_signature: Option<String>,

        /// Title for the note
        #[arg(short = 't', long = "title", value_name = "TITLE")]
        cli_title: Option<String>,

        /// File extension for the note
        #[arg(short = 'e', long = "extension", value_name = "EXTENSION")]
        cli_extension: Option<String>,

        /// Keywords for the note
        #[arg(short = 'k', long = "keywords", value_name = "KEYWORD(S)")]
        cli_keywords: Option<String>,
    },

    /// Rename an existing note
    Rename {
        /// Path to the input file to be renamed
        input: String,

        /// Print the absolute path of the created file
        #[arg(short = 'p', long = "print")]
        cli_print: bool,

        /// Generate an identifier even if there is an existing one
        #[arg(short = 'I', long = "regenerate-identifier")]
        cli_regenerate_identifier: bool,

        /// Rename based on frontmatter values
        #[arg(short = 'f', long = "frontmatter")]
        cli_rename_from_frontmatter: bool,

        /// Generate or regenerate frontmatter
        #[arg(short = 'G', long = "generate-frontmatter")]
        cli_generate_frontmatter: bool,

        /// Configuration file path
        #[arg(short = 'c', long = "config", value_name = "PATH")]
        cli_config_path: Option<String>,

        /// Frontmatter format
        #[arg(
            short = 'F',
            long = "frontmatter-format",
            value_name = "text|yaml|toml|org"
        )]
        cli_frontmatter_format: Option<String>,

        /// New signature for the note
        #[arg(short = 's', long = "signature", value_name = "SIGNATURE")]
        cli_signature: Option<String>,

        /// New title for the note
        #[arg(short = 't', long = "title", value_name = "TITLE")]
        cli_title: Option<String>,

        /// New keywords for the note
        #[arg(short = 'k', long = "keywords", value_name = "KEYWORDS")]
        cli_keywords: Option<String>,

        /// Add keywords to the current or new keywords
        #[arg(short = 'A', long = "add-keywords", value_name = "KEYWORDS")]
        cli_add_keywords: Option<String>,

        /// Remove keywords from the current or new keywords
        #[arg(short = 'R', long = "remove-keywords", value_name = "KEYWORDS")]
        cli_remove_keywords: Option<String>,

        /// New file extension for the note
        #[arg(short = 'e', long = "extension", value_name = "EXTENSION")]
        cli_extension: Option<String>,
    },
}
