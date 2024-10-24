mod filename;
mod frontmatter;
mod config;
mod args;

// Top-down draft using api
fn main() {
    // get_args();
    // get_config();
    // get_filename();
    // get_frontmatter();
    // get_template();
    // create_file();

    todo!()
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

