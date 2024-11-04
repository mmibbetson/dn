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

// TODO: Reconsider clone.
#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Create a new note
    New {
        /// Print the absolute path of the created file
        #[arg(short = 'p', long = "print")]
        print_path: bool,

        /// Generate or regenerate frontmatter
        #[arg(short = 'G', long = "generate-frontmatter")]
        generate_frontmatter: bool,

        /// Directory to save the note
        #[arg(short = 'd', long = "directory", value_name = "PATH")]
        directory: Option<String>,

        /// Config file location
        #[arg(short = 'c', long = "config", value_name = "PATH")]
        config: Option<String>,

        /// Template file
        #[arg(short = 'T', long = "template", value_name = "PATH")]
        template: Option<String>,

        /// Frontmatter format
        #[arg(
            short = 'F',
            long = "frontmatter-format",
            value_name = "Text|YAML|TOML|Org"
        )]
        frontmatter_format: Option<String>,

        /// Signature to use
        #[arg(short = 's', long = "signature", value_name = "SIGNATURE")]
        signature: Option<String>,

        /// Title for the note
        #[arg(short = 't', long = "title", value_name = "TITLE")]
        title: Option<String>,

        /// File extension
        #[arg(short = 'e', long = "extension", value_name = "EXTENSION")]
        extension: Option<String>,

        /// Keywords for the note
        #[arg(short = 'k', long = "keywords", value_name = "KEYWORD(S)")]
        keywords: Option<String>,
    },

    /// Rename an existing note
    Rename {
        /// Path to the input file
        input: String,

        /// Print the absolute path of the created file
        #[arg(short = 'p', long = "print")]
        print_path: bool,

        /// Generate identifier even if there is an existing one
        #[arg(short = 'I', long = "regenerate-identifier")]
        regenerate_identifier: bool,

        /// Rename based on frontmatter values
        #[arg(short = 'f', long = "frontmatter")]
        frontmatter: bool,

        /// Generate or regenerate frontmatter
        #[arg(short = 'G', long = "generate-frontmatter")]
        generate_frontmatter: bool,

        /// Config file location
        #[arg(short = 'c', long = "config", value_name = "PATH")]
        config: Option<String>,

        /// Frontmatter format
        #[arg(
            short = 'F',
            long = "frontmatter-format",
            value_name = "Text|YAML|TOML|Org"
        )]
        frontmatter_format: Option<String>,

        /// Signature to use
        #[arg(short = 's', long = "signature", value_name = "SIGNATURE")]
        signature: Option<String>,

        /// Title for the note
        #[arg(short = 't', long = "title", value_name = "TITLE")]
        title: Option<String>,

        /// Keywords for the note
        #[arg(short = 'k', long = "keywords", value_name = "KEYWORDS")]
        keywords: Option<String>,

        /// Add keywords
        #[arg(short = 'A', long = "add-keywords", value_name = "KEYWORDS")]
        add_keywords: Option<String>,

        /// Remove keywords
        #[arg(short = 'R', long = "remove-keywords", value_name = "KEYWORDS")]
        remove_keywords: Option<String>,

        /// File extension
        #[arg(short = 'e', long = "extension", value_name = "EXTENSION")]
        extension: Option<String>,
    },
}
