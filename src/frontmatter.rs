use chrono::{DateTime, Local};

use crate::{
    config::{FilenameConfig, FrontmatterConfig, FrontmatterDateTimeFormat},
    filename::FilenameDetails,
};

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

pub fn get_frontmatter(
    filename_details: &FilenameDetails,
    frontmatter_config: &FrontmatterConfig,
    filename_config: &FilenameConfig,
) -> String {
    let (open, close) = get_format_delimiters(&frontmatter_config.format);

    let content = frontmatter_config
        .segment_order
        .iter()
        .map(|seg| process_segment(seg, filename_details, frontmatter_config, filename_config))
        .collect::<Vec<_>>()
        .concat();

    format!("{}{}{}", open, content, close)
}

fn get_format_delimiters(format: &FrontmatterFormat) -> (&'static str, &'static str) {
    match format {
        FrontmatterFormat::Text => ("", "---------------------------\n\n"),
        FrontmatterFormat::YAML => ("---\n", "---\n\n"),
        FrontmatterFormat::TOML => ("+++\n", "+++\n\n"),
        FrontmatterFormat::Org => ("", ""),
    }
}

fn get_segment_prefix(format: &FrontmatterFormat, segment: &FrontmatterSegment) -> &'static str {
    match (format, segment) {
        (FrontmatterFormat::Text | FrontmatterFormat::YAML, FrontmatterSegment::Title) => {
            "title:      "
        }
        (FrontmatterFormat::Text | FrontmatterFormat::YAML, FrontmatterSegment::Date) => {
            "date:       "
        }
        (FrontmatterFormat::Text | FrontmatterFormat::YAML, FrontmatterSegment::Keywords) => {
            "keywords:   "
        }
        (FrontmatterFormat::Text | FrontmatterFormat::YAML, FrontmatterSegment::Identifier) => {
            "identifier: "
        }
        (FrontmatterFormat::TOML, FrontmatterSegment::Title) => "title =      ",
        (FrontmatterFormat::TOML, FrontmatterSegment::Date) => "date =       ",
        (FrontmatterFormat::TOML, FrontmatterSegment::Keywords) => "keywords =   ",
        (FrontmatterFormat::TOML, FrontmatterSegment::Identifier) => "identifier = ",
        (FrontmatterFormat::Org, FrontmatterSegment::Title) => "#+title:      ",
        (FrontmatterFormat::Org, FrontmatterSegment::Date) => "#+date:       ",
        (FrontmatterFormat::Org, FrontmatterSegment::Keywords) => "#+keywords:   ",
        (FrontmatterFormat::Org, FrontmatterSegment::Identifier) => "#+identifier: ",
    }
}

fn process_segment(
    segment: &FrontmatterSegment,
    filename_details: &FilenameDetails,
    frontmatter_config: &FrontmatterConfig,
    filename_config: &FilenameConfig,
) -> String {
    let arg = match segment {
        &FrontmatterSegment::Title => &filename_details.identifier_arg,
        &FrontmatterSegment::Date => &None,
        &FrontmatterSegment::Keywords => &filename_details.keywords_arg,
        &FrontmatterSegment::Identifier => &filename_details.extension_arg,
    };

    let prefix = get_segment_prefix(&frontmatter_config.format, segment);

    arg.as_deref().map_or(String::new(), |seg| match segment {
        FrontmatterSegment::Title | FrontmatterSegment::Identifier => {
            format_title_or_identifier(seg.to_string(), &frontmatter_config.format)
        }
        FrontmatterSegment::Date => format_date(
            filename_details.creation_time,
            &frontmatter_config.format,
            &frontmatter_config.date_time_format,
        ),
        FrontmatterSegment::Keywords => format_keywords(
            seg.to_string(),
            frontmatter_config.format,
            filename_config.illegal_characters.clone(),
        ),
    })
}

fn format_title_or_identifier(segment: String, format: &FrontmatterFormat) -> String {
    match format {
        FrontmatterFormat::Text | FrontmatterFormat::Org => format!("{}\n", segment),
        FrontmatterFormat::YAML | FrontmatterFormat::TOML => format!("\"{}\"\n", segment),
    }
}

fn format_date(
    date: DateTime<Local>,
    format: &FrontmatterFormat,
    time_format: &FrontmatterDateTimeFormat,
) -> String {
    let content = match format {
        FrontmatterFormat::Org => match time_format {
            &FrontmatterDateTimeFormat::TwentyFourHour => {
                date.format("[%Y-%m-%d %a %H:%M]").to_string()
            }
            &FrontmatterDateTimeFormat::TwelveHour => {
                date.format("[%Y-%m-%d %a %I:%M %p]").to_string()
            } // %I for 12-hour, %p for AM/PM
            &FrontmatterDateTimeFormat::None => date.format("[%Y-%m-%d %a]").to_string(),
        },
        _ => match time_format {
            &FrontmatterDateTimeFormat::TwentyFourHour => {
                date.format("%Y-%m-%dT%H:%M:%S%:z").to_string()
            }
            &FrontmatterDateTimeFormat::TwelveHour => {
                date.format("%Y-%m-%d %I:%M:%S %p %:z").to_string()
            } // Non-standard ISO but human readable
            &FrontmatterDateTimeFormat::None => date.format("%Y-%m-%d").to_string(),
        },
    };

    format!("{}\n", content)
}

fn format_keywords(
    keywords: String,
    format: FrontmatterFormat,
    illegal_characters: Vec<char>,
) -> String {
    // TODO: Reconsider - This is repeated logic from filename processing.
    let processed = keywords
        .to_lowercase()
        .split(['_', ' '].as_ref())
        .filter(|sub| !sub.is_empty())
        .map(|sub| sanitise_segment(sub, &illegal_characters))
        .collect::<Vec<_>>();

    match format {
        FrontmatterFormat::Text => processed.join("  "),
        FrontmatterFormat::YAML | FrontmatterFormat::TOML => {
            let formatted = processed
                .iter()
                .map(|word| format!("\"{}\"", word))
                .collect::<Vec<String>>()
                .join(", ");

            format!("[{}]", formatted)
        }
        FrontmatterFormat::Org => format!(":{}:", processed.join(":")),
    }
}

// WARN: This is straight copy-pasted from the filename module. I didn't import to make it more obvious.
fn sanitise_segment(segment: &str, illegal_characters: &Vec<char>) -> String {
    const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

    segment
        .chars()
        .filter(|c| !SEGMENT_SEPARATORS.contains(c))
        .filter(|c| !illegal_characters.contains(c))
        .collect()
}
