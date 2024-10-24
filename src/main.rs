use crate::filename::get_filename;
use crate::filename::Segment;

mod filename;
mod frontmatter;
mod config;
mod args;


// Top-down draft using api
fn main() {
    let default_segment_order = vec![Segment::Identifier, Segment::Signature, Segment::Title, Segment::Keywords, Segment::Extension];

    // get_args();
    // get_config();
    let filename = get_filename(default_segment_order);
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
    order: Vec<filename::Segment>
}

fn create_file(details: FileDetails, config: Config) {
    todo!()
}

