use crate::config::FilenameConfig;
use chrono::{DateTime, Local};

#[derive(Clone, Default)]
pub struct FilenameDetails {
	pub creation_time: DateTime<Local>,
	pub identifier_arg: Option<String>,
	pub signature_arg: Option<String>,
	pub title_arg: Option<String>,
	pub keywords_arg: Option<String>,
	pub extension_arg: Option<String>,
}

#[derive(PartialEq)]
pub enum Segment {
	Identifier,
	Signature,
	Title,
	Keywords,
	Extension,
}

pub const DN_IDENTIFIER_FORMAT: &str = "%Y%m%dT%H%M%S";

/// TODO: Documentation.
pub fn get_filename(filename_details: &FilenameDetails, config: &FilenameConfig) -> String {
	config
		.segment_order
		.iter()
		.map(|segment| process_segment(segment, filename_details, config))
		.collect::<Vec<_>>()
		.concat()
}

fn process_segment(
	segment: &Segment,
	filename_details: &FilenameDetails,
	config: &FilenameConfig,
) -> String {
	let arg = match segment {
		Segment::Identifier => &filename_details.identifier_arg,
		Segment::Signature => &filename_details.signature_arg,
		Segment::Title => &filename_details.title_arg,
		Segment::Keywords => &filename_details.keywords_arg,
		Segment::Extension => &filename_details.extension_arg,
	};
	let prefix = segment_prefix(&segment);

	match segment {
		Segment::Identifier => format_identifier(
			filename_details.creation_time,
			config.segment_order[0] == *segment,
		),
		Segment::Extension => format_segment(
			arg.as_deref().unwrap_or(&config.default_file_extension),
			prefix,
			&config.illegal_characters,
		),
		_ => format_optional(arg, prefix, &config.illegal_characters),
	}
}

fn segment_prefix(segment: &Segment) -> &'static str {
	match segment {
		Segment::Identifier => "@@",
		Segment::Signature => "==",
		Segment::Title => "--",
		Segment::Keywords => "__",
		Segment::Extension => ".",
	}
}

fn format_identifier(creation_time: DateTime<Local>, is_first: bool) -> String {
	let time = creation_time.format(DN_IDENTIFIER_FORMAT).to_string();

	match is_first {
		true => time,
		false => format!("{}{}", segment_prefix(&Segment::Identifier), time),
	}
}

fn format_optional(
	segment: &Option<String>,
	prefix: &str,
	illegal_characters: &Vec<char>,
) -> String {
	segment.as_deref().map_or(String::new(), |seg| {
		format_segment(seg, prefix, illegal_characters)
	})
}

fn format_segment(segment: &str, prefix: &str, illegal_characters: &Vec<char>) -> String {
	let out = segment
		.to_lowercase()
		.split([prefix.chars().nth(0).unwrap(), ' '].as_ref())
		.filter(|sub| !sub.is_empty())
		.map(|sub| sanitise_segment(sub, illegal_characters))
		.collect::<Vec<_>>()
		.join(&prefix[..1]);

	format!("{}{}", prefix, out)
}

fn sanitise_segment(segment: &str, illegal_characters: &Vec<char>) -> String {
	const SEGMENT_SEPARATORS: [char; 4] = ['=', '-', '_', '.'];

	segment
		.chars()
		.filter(|c| !SEGMENT_SEPARATORS.contains(c))
		.filter(|c| !illegal_characters.contains(c))
		.collect()
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_filename_generation() {
		let details = FilenameDetails {
			title_arg: Some("My Document".to_string()),
			signature_arg: None,
			keywords_arg: None,
			extension_arg: None,
			..Default::default()
		};
		let config = FilenameConfig::default();
		let result = get_filename(&details, &config);

		assert!(result.contains("--my-document.txt"));
	}

	#[test]
	fn test_all_segments() {
		let details = FilenameDetails {
			title_arg: Some("Test Title".to_string()),
			signature_arg: Some("123".to_string()),
			keywords_arg: Some("key1_key2".to_string()),
			extension_arg: Some("md".to_string()),
			..Default::default()
		};
		let config = FilenameConfig::default();
		let result = get_filename(&details, &config);

		assert!(result.contains("==123--test-title__key1_key2.md"));
	}

	#[test]
	fn test_identifier_positioning() {
		let details = FilenameDetails {
			title_arg: Some("Test".to_string()),
			signature_arg: None,
			keywords_arg: None,
			extension_arg: None,
			..Default::default()
		};

		// Identifier first
		let config_1 = FilenameConfig::default();
		let result_1 = get_filename(&details, &config_1);

		// Identifier not first
		let config_2 = FilenameConfig {
			segment_order: [
				Segment::Title,
				Segment::Identifier,
				Segment::Signature,
				Segment::Keywords,
				Segment::Extension,
			],
			..Default::default()
		};
		let result2 = get_filename(&details, &config_2);

		assert!(!result_1.contains("@@"));
		assert!(result2.contains("@@"));
	}

	#[test]
	fn test_segment_reordering() {
		let details = FilenameDetails {
			title_arg: Some("my title".to_string()),
			signature_arg: Some("123".to_string()),
			keywords_arg: Some("key1_key2".to_string()),
			extension_arg: None,
			..Default::default()
		};
		let config = FilenameConfig {
			segment_order: [
				Segment::Identifier,
				Segment::Extension,
				Segment::Keywords,
				Segment::Title,
				Segment::Signature,
			],
			..Default::default()
		};
		let result = get_filename(&details, &config);

		assert!(result.contains(".txt__key1_key2--my-title==123"));
	}

	// TODO: test specific illegal character vec
	#[test]
	fn test_illegal_characters() {
		let details = FilenameDetails {
			title_arg: Some("Test! @#$ Title".to_string()),
			signature_arg: Some("Auth[or](Name)".to_string()),
			keywords_arg: Some("key1&&^key2".to_string()),
			extension_arg: Some("...org".to_string()),
			..Default::default()
		};
		let config = FilenameConfig::default();
		let result = get_filename(&details, &config);

		assert!(
			result
				.chars()
				.filter(|c| config.illegal_characters.contains(c))
				.collect::<String>()
				== "".to_string()
		);
	}

	#[test]
	fn test_empty_optional_segments() {
		let details = FilenameDetails {
			title_arg: None,
			signature_arg: None,
			keywords_arg: None,
			extension_arg: None,
			..Default::default()
		};
		let config = FilenameConfig::default();
		let result = get_filename(&details, &config);

		assert!(!result.contains("=="));
		assert!(!result.contains("--"));
		assert!(!result.contains("__"));
		assert!(result.ends_with(".txt"));
	}

	#[test]
	fn test_segment_separator_sanitisation() {
		let details = FilenameDetails {
			title_arg: Some("first.second-third_fourth".to_string()),
			signature_arg: None,
			keywords_arg: Some("_kwrd__check".to_string()),
			extension_arg: Some(".tar.gz".to_string()),
			..Default::default()
		};
		let config = FilenameConfig::default();
		let result = get_filename(&details, &config);

		assert!(result.contains("--firstsecond-thirdfourth__kwrd_check.tar.gz"));
	}

	#[test]
	fn test_case_sensitivity() {
		let details = FilenameDetails {
			title_arg: Some("UPPERCASE".to_string()),
			signature_arg: Some("MixedCase".to_string()),
			keywords_arg: Some("CamelCase".to_string()),
			extension_arg: Some("ORG".to_string()),
			..Default::default()
		};
		let config = FilenameConfig::default();
		let result = get_filename(&details, &config);

		assert!(result.contains("--uppercase"));
		assert!(result.contains("==mixedcase"));
		assert!(result.contains("__camelcase"));
		assert!(result.ends_with(".org"));
	}
}
