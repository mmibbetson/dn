use chrono::{DateTime, Local};

use crate::{config::FileConfig, format::DN_IDENTIFIER_FORMAT};

pub struct FileMetadata {
    identifier: String,
    signature: Option<String>,
    title: Option<String>,
    title_raw: Option<String>,
    keywords: Option<Vec<String>>,
    extension: String,
    datetime: DateTime<Local>,
}

pub fn get_metadata(
    instance_time: &DateTime<Local>,
    identifier_arg: &Option<String>,
    signature_arg: &Option<String>,
    title_arg: &Option<String>,
    keywords_arg: &Option<String>,
    extension_arg: &Option<String>,
    config: &FileConfig,
) -> FileMetadata {
    let identifier = derive_identifier(instance_time, identifier_arg);
    let signature = signature_arg
        .as_ref()
        .and_then(|sig| parse_signature(&sig, &config.illegal_characters));
    let title_raw = title_arg.as_ref().map(String::from);
    let title = title_arg
        .as_ref()
        .and_then(|ttl| parse_title(&ttl, &config.illegal_characters));
    let keywords = keywords_arg
        .as_ref()
        .and_then(|keys| parse_keywords(&keys, &config.illegal_characters));
    let extension = extension_arg
        .as_ref()
        .and_then(|ext| parse_extension(&ext, &config.illegal_characters))
        .unwrap_or(config.default_extension.clone());
    let datetime = *instance_time;

    FileMetadata {
        identifier,
        signature,
        title,
        title_raw,
        keywords,
        extension,
        datetime,
    }
}

fn derive_identifier(instance_time: &DateTime<Local>, signature_arg: &Option<String>) -> String {
    match signature_arg {
        Some(id) => id.to_string(),
        None => instance_time.format(DN_IDENTIFIER_FORMAT).to_string(),
    }
}

fn parse_signature(signature_arg: &str, illegal_characters: &[char]) -> Option<String> {
    let out = signature_arg
        .to_lowercase()
        .chars()
        .filter(|&c| !illegal_characters.contains(&c))
        .collect::<String>();

    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn parse_title(title_arg: &str, illegal_characters: &[char]) -> Option<String> {
    let out = title_arg
        .to_lowercase()
        .split(['-', ' '])
        .map(|w| sanitise(w, illegal_characters))
        .collect::<Vec<_>>()
        .join("-");

    // NOTE: (!out.is_empty()).then_some(out) is equivalent; pretty cool.
    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn parse_keywords(keywords_arg: &str, illegal_characters: &[char]) -> Option<Vec<String>> {
    let out = keywords_arg
        .to_lowercase()
        .split(['_', ' '])
        .map(|w| sanitise(w, illegal_characters))
        .collect::<Vec<_>>();

    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn parse_extension(extension_arg: &str, illegal_characters: &[char]) -> Option<String> {
    let out = extension_arg
        .to_lowercase()
        .split(['.', ' '])
        .map(|w| sanitise(w, illegal_characters))
        .collect::<Vec<_>>()
        .join(".");

    match out.is_empty() {
        true => None,
        false => Some(out),
    }
}

fn sanitise(dirty: &str, illegal_characters: &[char]) -> String {
    dirty
        .chars()
        .filter(|&c| !SEGMENT_SEPARATORS.contains(&c))
        .filter(|&c| !illegal_characters.contains(&c))
        .collect::<String>()
}

const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];
