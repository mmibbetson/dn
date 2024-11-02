use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use cli::Cli;

mod cli;
mod directory;
mod config;
mod format;
mod file;

// Top-down draft using api
fn main() {
    let cli = Cli::parse();
    let instance_time = chrono::Local::now();

    match &cli.command {
        cli::Commands::New {
            generate_frontmatter,
            directory,
            order,
            frontmatter_order,
            config,
            template,
            frontmatter_format,
            signature,
            title,
            extension,
            keywords,
        } => {
            let config = build_config(); // coordinate available config
            let metadata = get_metadata();
            let filename = get_filename(metadata, config.filename_config);
            // let frontmatter = get_frontmatter(metadata, config.frontmatter_config); // optional
            // let template = get_template(template_path, config.template_config); // optional
            let path = get_path(config.directory_config);
            let content = get_content(frontmatter, template);

            file.write(path, filecontent)
        }
        cli::Commands::Rename {
            input,
            regenerate_identifier,
            frontmatter,
            generate_frontmatter,
            order,
            frontmatter_order,
            config,
            frontmatter_format,
            signature,
            extension,
            keywords,
            add_keywords,
            remove_keywords,
        } => {}
    }
}

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

pub struct FileMetadata {
    identifier: String,
    signature: Option<String>,
    title: Option<String>,
    keywords: Option<Vec<String>>,
    extension: String,
    datetime: DateTime<Local>,
}

pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

// let example = FileMetadata {
//     identifier: "20241031T232930",
//     signature: "ggl210",
//     title_raw: "Sprint Goals! - 210",
//     title: "sprint-goals-210",
//     keywords: vec!["ggl", "client-ADmtars!!", "admin"],
//     extension: "md",
//     datetime: chrono::Local::now(),
// }

// 20241031T232930==GGL210--sprint-goals-210__ggl_client_admin.md

// ---
// title:      "Sprint Goals - 210"
// date:       "2024-10-31 23:34:30 +2:00"
// tags:       ["ggl", "client", "admin"]
// identifier: "20241031T232930"
// ---

// +++
// title =      "Sprint Goals - 210"
// date =       "2024-10-31 23:34:30 +2:00"
// tags =       ["ggl", "client", "admin"]
// identifier = "20241031T232930"
// +++

// title:      Sprint Goals - 210
// date:       2024-10-31 23:34:30 +2:00
// tags:       ggl  client  admin
// identifier: "20241031T232930"
// -----------------------------

// #+title:      Sprint Goals - 210
// #+date:       [2024-10-31 23:34:30 +2:00]
// #+tags:       :ggl:client:admin:
// #+identifier: 20241031T232930
