use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dn", version = "0.1", about = "A command to manage notes following the Denote naming scheme.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Display help information
    #[arg(short, long)]
    help: bool,

    /// Display version information
    #[arg(short, long)]
    version: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new note
    New {
        /// Generate or regenerate frontmatter
        #[arg(short = 'G', long)]
        generate_frontmatter: bool,

        /// Directory to save the note
        #[arg(short = 'd', long, value_name = "DIRECTORY")]
        directory: Option<String>,

        /// Order of filename segments, provided as a comma-delimited string
        #[arg(short = 'o', long, value_name = "LIST")]
        order: Option<String>,

        /// Frontmatter order of metadata, provided as a comma-delimited string
        #[arg(short = 'O', long)]
        frontmatter_order: Option<String>,

        /// Config file location
        #[arg(short = 'c', long)]
        config: Option<String>,

        /// Template file
        #[arg(short = 'T', long)]
        template: Option<String>,

        /// Frontmatter format
        #[arg(short = 'F', long)]
        frontmatter_format: Option<String>,

        /// Signature to use
        #[arg(short = 's', long)]
        signature: Option<String>,

        /// Title for the note
        #[arg(short = 't', long)]
        title: Option<String>,

        /// File extension
        #[arg(short = 'e', long)]
        extension: Option<String>,

        /// Keywords for the note
        #[arg(short = 'k', long)]
        keywords: Option<String>,
    },
    
    /// Rename an existing note
    Rename {
        /// Path to the input file
        input: String,

        /// Generate identifier even if there is an existing one
        #[arg(short = 'I', long)]
        regenerate_identifier: bool,

        /// Rename based on frontmatter values
        #[arg(short = 'f', long)]
        frontmatter: bool,

        /// Generate or regenerate frontmatter
        #[arg(short = 'G', long)]
        generate_frontmatter: bool,

        /// Order of elements
        #[arg(short = 'o', long)]
        order: Option<String>,

        /// Frontmatter order
        #[arg(short = 'O', long)]
        frontmatter_order: Option<String>,

        /// Config file location
        #[arg(short = 'c', long)]
        config: Option<String>,

        /// Frontmatter format
        #[arg(short = 'F', long)]
        frontmatter_format: Option<String>,

        /// Signature to use
        #[arg(short = 's', long)]
        signature: Option<String>,

        /// File extension
        #[arg(short = 'e', long)]
        extension: Option<String>,

        /// Keywords for the note
        #[arg(short = 'k', long)]
        keywords: Option<String>,

        /// Add keywords
        #[arg(short = 'A', long)]
        add_keywords: Option<String>,

        /// Remove keywords
        #[arg(short = 'R', long)]
        remove_keywords: Option<String>,
    },
}

