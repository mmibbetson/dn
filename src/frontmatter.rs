use crate::{config::FrontmatterConfig, filename::FilenameDetails};

pub enum FrontmatterFormat {
    Text,
    YAML,
    TOML,
    Org,
}

pub enum FrontmatterSegment {
    Title,
    Date,
    Keywords,
    Identifier,
}

pub fn get_frontmatter(filename_details: &FilenameDetails, config: &FrontmatterConfig) -> String {
    let open = match format {
        FrontmatterFormat::YAML => "---",
        FrontmatterFormat::TOML => "+++",
        _ => "",
    }

    let close = match format {
        FrontmatterFormat::Text => "------------------------"
        FrontmatterFormat::YAML => "---",
        FrontmatterFormat::TOML => "+++",
        _ => "",
    }

    todo!()
}

fn format_optional(segment: &Option<String>, format: &FrontmatterFormat) -> &str {
    segment.as_deref().map_or(String::new(), |seg| {
        format_segment(seg, format)
    })
}

fn format_segment(segment: &str, format: &FrontmatterFormat) -> &str {
    let seg = segment
        .
}

// TODO: Lookup e.g. "#+title:" or "date =" depending on format and segment.
fn segment_key_lookup(segment: &FrontmatterSegment, format: &FrontmatterFormat) -> &'static str {}

