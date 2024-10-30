use chrono::{DateTime, Local};

use crate::{config::FrontmatterConfig, filename::{format_identifier, FilenameDetails}};

#[derive(Copy, Clone)]
pub enum FrontmatterFormat {
    Text,
    YAML,
    TOML,
    Org,
}

#[derive(Copy, Clone)]
pub enum FrontmatterSegment {
    Title,
    Date,
    Keywords,
    Identifier,
}

#[derive(Copy, Clone)]
enum TimeFormat {
    Hour12,
    Hour24,
    None,
}

pub fn get_frontmatter(filename_details: &FilenameDetails, config: &FrontmatterConfig) -> String {
    let open = match config.format {
        FrontmatterFormat::YAML => "---\n",
        FrontmatterFormat::TOML => "+++\n",
        _ => "",
    };

    let close = match config.format {
        FrontmatterFormat::Text => "---------------------------\n\n",
        FrontmatterFormat::YAML => "---\n\n",
        FrontmatterFormat::TOML => "+++\n\n",
        _ => "",
    };

    let title = filename_details.title_arg.clone().map_or(String::new(), |seg| {
        format_segment(seg.to_owned(), &config.format)
    });

    let date = filename_details.title_arg.clone().map_or(String::new(), |seg| {
        format_segment(seg.to_owned(), &config.format)
    });

    let keywords = filename_details.title_arg.clone().map_or(String::new(), |seg| {
        format_segment(seg.to_owned(), &config.format)
    });
    
    let identifier = filename_details.creation_time.clone().map_or(String::new(), |seg| {
        format_segment(seg.to_owned(), &config.format)
    });

    let content = config
        .segment_order
        .iter()
        .map(|seg| process_segment(seg, filename_details, config))
        .collect::<Vec<_>>()
        .concat();

    format!("{}{}{}", open, content, close)
}

fn process_segment(
    segment: &FrontmatterSegment,
    filename_details: &FilenameDetails,
    config: &FrontmatterConfig,
) -> String {
    let arg = match segment {
        &FrontmatterSegment::Title => &filename_details.identifier_arg,
        &FrontmatterSegment::Date => &filename_details.signature_arg,
        &FrontmatterSegment::Keywords => &filename_details.keywords_arg,
        &FrontmatterSegment::Identifier => &filename_details.extension_arg,
    };
    let prefix = segment_prefix(segment, &config.format);

    format_optional(arg, &config.format)
}

fn format_optional(segment: &Option<String>, format: &FrontmatterFormat) -> String {
    
}

fn format_segment(segment: String, format: &FrontmatterFormat) -> String {
    let 
}

fn format_title(title: String, format: &FrontmatterFormat) -> String {
    match format {
        FrontmatterFormat::YAML | FrontmatterFormat::TOML => format!("\"{}\"\n", title),
        _ => format!("{}\n", title),
    }
}

fn format_date(date: DateTime<Local>, format: &FrontmatterFormat, time_format: TimeFormat) -> String {
    let content = match format {
        FrontmatterFormat::Org => match time_format {
            TimeFormat::Hour24 => date.format("[%Y-%m-%d %a %H:%M]").to_string(),
            TimeFormat::Hour12 => date.format("[%Y-%m-%d %a %I:%M %p]").to_string(), // %I for 12-hour, %p for AM/PM
            TimeFormat::None => date.format("[%Y-%m-%d %a]").to_string(),
        },
        _ => match time_format {
            TimeFormat::Hour24 => date.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            TimeFormat::Hour12 => date.format("%Y-%m-%d %I:%M:%S %p %:z").to_string(), // Non-standard ISO but human readable
            TimeFormat::None => date.format("%Y-%m-%d").to_string(),
        }
    };

    format!("{}\n", content)
}

fn format_keywords() {}

fn segment_prefix(segment: &FrontmatterSegment, format: &FrontmatterFormat) -> &'static str {
    match format {
        FrontmatterFormat::Text => text_prefix(segment),
        FrontmatterFormat::YAML => yaml_prefix(segment),
        FrontmatterFormat::TOML => toml_prefix(segment),
        FrontmatterFormat::Org => org_prefix(segment),
    }
}

fn text_prefix(segment: &FrontmatterSegment) -> &'static str {
    match segment {
        FrontmatterSegment::Title =>      "title:      ",
        FrontmatterSegment::Date =>       "date:       ",
        FrontmatterSegment::Keywords =>   "keywords:   ",
        FrontmatterSegment::Identifier => "identifier: ",
    }
}

fn yaml_prefix(segment: &FrontmatterSegment) -> &'static str {
    match segment {
        FrontmatterSegment::Title =>      "title:      ",
        FrontmatterSegment::Date =>       "date:       ",
        FrontmatterSegment::Keywords =>   "keywords:   ",
        FrontmatterSegment::Identifier => "identifier: ",
    }
}

fn toml_prefix(segment: &FrontmatterSegment) -> &'static str {
    match segment {
        FrontmatterSegment::Title =>      "title      = ",
        FrontmatterSegment::Date =>       "date       = ",
        FrontmatterSegment::Keywords =>   "keywords   = ",
        FrontmatterSegment::Identifier => "identifier = ",
    }
}

fn org_prefix(segment: &FrontmatterSegment) -> &'static str {
    match segment {
        FrontmatterSegment::Title =>      "#+title:      ",
        FrontmatterSegment::Date =>       "#+date:       ",
        FrontmatterSegment::Keywords =>   "#+keywords:   ",
        FrontmatterSegment::Identifier => "#+identifier: ",
    }
}
