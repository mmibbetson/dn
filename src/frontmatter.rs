use crate::{config::FrontmatterConfig, filename::FilenameDetails};

pub fn get_frontmatter(filename_details: &FilenameDetails, config: &FrontmatterConfig) -> String {
    config
        .segment_order
        .iter()
        .map(|segment| process_segment(segment, filename_details, config))
        .collect::<Vec<_>>()
        .concat()
}
