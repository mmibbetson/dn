// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! CLI tool for managing notes in a minimalistic, cross-platform, free, extensible manner.

use std::{fs, path::PathBuf};

use clap::Parser;
use cli::Cli;
use config::{load_config, Config};
use content::concatenate_file_content;
use filename::ToFilename;
use format::get_first_paragraph;
use metadata::FileMetadata;

mod cli;
mod config;
mod content;
mod directory;
mod filename;
mod format;
mod metadata;

fn main() {
    // let cli = Cli::parse();

    // match &cli.command {
    //     cli::Commands::New {
    //         cli_print,
    //         cli_generate_frontmatter,
    //         cli_directory_path,
    //         cli_config_path,
    //         cli_template_path,
    //         cli_frontmatter_format,
    //         cli_signature,
    //         cli_title,
    //         cli_extension,
    //         cli_keywords,
    //     } => {
    //         let config = {
    //             let config_builder = Config::builder();

    //             let config_base = load_config(cli_config_path).unwrap_or_else(|e| {
    //                 eprintln!("Error loading configuration: {e:#?}");
    //                 std::process::exit(1);
    //             });

    //             config_builder = config_base
    //                 .map(|p| config_builder.with_base_config(&p))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_generate_frontmatter
    //                 .then(|| config_builder.with_frontmatter_enabled(true))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_directory_path
    //                 .map(|p| config_builder.with_file_directory(&p))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_extension
    //                 .map(|e| config_builder.with_file_default_extension(&e))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_template_path
    //                 .map(|p| config_builder.with_file_template_path(&p.into()))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_frontmatter_format
    //                 .map(|f| config_builder.with_frontmatter_format(&f))
    //                 .unwrap_or(config_builder);

    //             config_builder.build().unwrap_or_else(|e| {
    //                 // ERROR
    //                 eprintln!("Error buildig configuration: {e:#?}");
    //                 std::process::exit(1);
    //             })
    //         };

    //         let metadata = FileMetadata::builder()
    //             .with_signature(cli_signature)
    //             .with_title(cli_title)
    //             .with_keywords(cli_keywords)
    //             .with_extension(cli_extension)
    //             // WARN: Possible code smell. Why does metadata take a &FileConfig specifically?
    //             .build(&config.file);

    //         let filename = metadata.to_filename(&config.file).to_string();
    //         let frontmatter = cli_generate_frontmatter
    //             .then(|| metadata.to_frontmatter(&config.frontmatter).to_string());
    //         let template = cli_template_path.map(|p| {
    //             fs::read(p).unwrap_or_else(|e| {
    //                 // ERROR
    //                 eprintln!("Error reading template file: {e:#?}");
    //                 std::process::exit(1);
    //             })
    //         });

    //         let output_path = cli_directory_path
    //             .map_or(config.file.directory, PathBuf::from)
    //             .join(filename);
    //         let output_content = concatenate_file_content(frontmatter, template);

    //         fs::write(output_path, output_content);

    //         if *cli_print {
    //             print!(
    //                 "{}",
    //                 output_path.to_str().unwrap_or_else(|| {
    //                     // ERROR
    //                     eprintln!("Error printing new file path");
    //                     std::process::exit(1);
    //                 })
    //             )
    //         };
    //     }
    //     cli::Commands::Rename {
    //         input,
    //         cli_print,
    //         cli_regenerate_identifier,
    //         cli_rename_from_frontmatter,
    //         cli_generate_frontmatter,
    //         cli_config_path,
    //         cli_frontmatter_format,
    //         cli_signature,
    //         cli_title,
    //         cli_extension,
    //         cli_keywords,
    //         cli_add_keywords,
    //         cli_remove_keywords,
    //     } => {
    //         let config = {
    //             let config_builder = Config::builder();

    //             let config_base = load_config(cli_config_path).unwrap_or_else(|e| {
    //                 eprintln!("Error loading configuration: {e:#?}");
    //                 std::process::exit(1);
    //             });

    //             config_builder = config_base
    //                 .map(|p| config_builder.with_base_config(&p))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_regenerate_identifier
    //                 .then(|| config_builder.with_file_regenerate_identifier(true))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_generate_frontmatter
    //                 .then(|| config_builder.with_frontmatter_enabled(true))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_extension
    //                 .map(|e| config_builder.with_file_default_extension(&e))
    //                 .unwrap_or(config_builder);

    //             config_builder = cli_frontmatter_format
    //                 .map(|f| config_builder.with_frontmatter_format(&f))
    //                 .unwrap_or(config_builder);

    //             config_builder.build().unwrap_or_else(|e| {
    //                 // ERROR
    //                 eprintln!("Error building configuration: {e:#?}");
    //                 std::process::exit(1);
    //             })
    //         };

    //         let input_path = PathBuf::from(input);
    //         let input_content = fs::read_to_string(input_path).unwrap_or_else(|e| {
    //             // ERROR
    //             eprintln!("Error reading input file: {e:#?}");
    //             std::process::exit(1);
    //         });

    //         let (frontmatter_old_str, content_old_str) = separate_existing_content(&input_content);

    //         let filename_old = PathBuf::from(input)
    //             .file_name()
    //             .and_then(|o| o.to_str())
    //             .unwrap_or_else(|| {
    //                 // ERROR
    //                 eprintln!("Error reading file name: Could not parse path");
    //                 std::process::exit(1);
    //             })
    //             .to_string()
    //             .to_filename(&config.file);

    //         let metadata_builder = FileMetadata::builder()
    //             .with_identifier(&Some(filename_old.identifier))
    //             .with_signature(&filename_old.signature)
    //             .with_title(&filename_old.title)
    //             .with_keywords(&filename_old.keywords)
    //             .with_extension(&Some(filename_old.extension));

    //         if *cli_rename_from_frontmatter {
    //             let frontmatter_old = frontmatter_old_str.to_frontmatter();

    //             metadata_builder = frontmatter_old
    //                 .title
    //                 .map(|t| metadata_builder.with_title(t))
    //                 .unwrap_or(metadata_builder);

    //             metadata_builder = frontmatter_old
    //                 .keywords
    //                 .map(|k| metadata_builder.with_keywords(k))
    //                 .unwrap_or(metadata_builder);

    //             metadata_builder = frontmatter_old
    //                 .identifier
    //                 .map(|i| metadata_builder.with_identifier(i))
    //                 .unwrap_or(metadata_builder);
    //         };

    //         metadata_builder = cli_signature
    //             .map(|s| metadata_builder.with_signature(&Some(s)))
    //             .unwrap_or(metadata_builder);

    //         metadata_builder = cli_title
    //             .map(|t| metadata_builder.with_title(&Some(t)))
    //             .unwrap_or(metadata_builder);

    //         metadata_builder = cli_keywords
    //             .map(|k| metadata_builder.with_keywords(&Some(k)))
    //             .unwrap_or(metadata_builder);

    //         metadata_builder = cli_add_keywords
    //             .map(|k| metadata_builder.with_added_keywords(&Some(k)))
    //             .unwrap_or(metadata_builder);

    //         metadata_builder = cli_remove_keywords
    //             .map(|k| metadata_builder.with_removed_keywords(&Some(k)))
    //             .unwrap_or(metadata_builder);

    //         metadata_builder = cli_extension
    //             .map(|e| metadata_builder.with_extension(&Some(e)))
    //             .unwrap_or(metadata_builder);

    //         // WARN: Possible code smell. Why does metadata take a &FileConfig specifically?
    //         // Passing the full Config is more conceptually sound but would be passing unnecessary
    //         // information, currently.
    //         let metadata = metadata_builder.build(&config.file);

    //         let filename_new = metadata.to_filename(&config.file).to_string();
    //         let frontmatter_new = cli_generate_frontmatter
    //             .then(|| metadata.to_frontmatter(config.frontmatter).to_string());

    //         let output_path = input_path
    //             .parent()
    //             .unwrap_or_else(|| {
    //                 // ERROR
    //                 eprintln!(
    //                     "Error reading file directory: Could not parse input file parent directory"
    //                 );
    //                 std::process::exit(1);
    //             })
    //             .join(filename_new);
    //         let output_content = concatenate_file_content(frontmatter_new, content_old_str);

    //         fs::rename(input_path, output_path);
    //         fs::write(output_path, output_content);

    //         if *cli_print {
    //             print!(
    //                 "{}",
    //                 output_path.to_str().unwrap_or_else(|| {
    //                     // ERROR
    //                     eprintln!("Error printing new file path");
    //                     std::process::exit(1);
    //                 })
    //             )
    //         };
    //     }
    // }

    std::process::exit(0);
}
