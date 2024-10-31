use chrono::DateTime;
use chrono::Local;

use crate::filename::get_filename;
use crate::filename::FilenameSegment;

mod cli;
mod config;
mod filename;
mod filename_parse;
mod frontmatter;

// Top-down draft using api
fn main() {
    // get_args();
    // get_config();
    // get_filename();
    // get_frontmatter();
    // get_template();
    // create_file();
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

// let example = FileMetadata {
//     identifier: "20241031T232930",
//     signature: "GGL210",
//     title: "Sprint Goals - 210",
//     keywords: vec!["ggl", "client", "admin"],
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