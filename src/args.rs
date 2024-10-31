// When getting --add-keywords or --remove-keywords we want to modify the keywords_arg
// 1. Take existing keywords_arg (-k>filename_parse>None)
// 2. format!("{}_{}", keywords_arg, add_keywords_arg)
// 3. split words in remove_keywords_arg
// 4. iterate over formatted string, remove instances of remove keywords arg words from [3]
// 5. collect properly into nice single string, to be used as final keywords_arg value

// When renaming a file, be sure to parse the filename correctly out of the input path
// and rejoin if necessary when writing to disk or w/e.

// When --regenerate-identifier is false, check get_identifier() and if the file has an identifier
// we will use that to get the creation_time and identifier_arg. This way, frontmatter.rs can be
// agnostic wrt the identifier arg.

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
        #[arg(short = 'd', long)]
        directory: Option<String>,

        /// Order of elements
        #[arg(short = 'o', long)]
        order: Option<String>,

        /// Frontmatter order
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

