use crate::filename::get_filename;
use crate::filename::Segment;

mod args;
mod config;
mod filename;
mod frontmatter;

const DEFAULT_SEGMENT_ORDER: [Segment; 5] = [
    Segment::Identifier,
    Segment::Signature,
    Segment::Title,
    Segment::Keywords,
    Segment::Extension,
];

// Top-down draft using api
fn main() {
    // get_args();
    // get_config();
    let filename = get_filename(DEFAULT_SEGMENT_ORDER);
    // get_frontmatter();
    // get_template();
    // create_file();

    println!("{}", filename);
}

// Struct to represent details pertinent to file being created
struct FileDetails {
    name: String,
    frontmatter: String,
    template: String,
}

// Config struct to represent various configuration parameters
struct Config {
    directory: String,
    order: Vec<filename::Segment>,
}

fn create_file(details: FileDetails, config: Config) {
    todo!()
}
