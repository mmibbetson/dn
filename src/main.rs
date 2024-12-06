// SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! CLI tool for managing notes in a minimalistic, cross-platform, free, extensible manner.

use std::{fs, ops::Deref, path::PathBuf};

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
    let cli = Cli::parse();

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

    //             let config_base = load_config(cli_config_path.as_deref()).unwrap_or_else(|e| {
    //                 eprintln!("Error loading configuration: {e:#?}");
    //                 std::process::exit(1);
    //             });

    //             if let Some(base) = config_base {
    //                 config_builder = config_builder.with_base_config(&base);
    //             }

    //             if *cli_generate_frontmatter {
    //                 config_builder = config_builder.with_frontmatter_enabled(true);
    //             }

    //             if let Some(path) = cli_directory_path {
    //                 config_builder = config_builder.with_file_directory(path);
    //             }

    //             if let Some(ext) = cli_extension {
    //                 config_builder = config_builder.with_file_default_extension(ext);
    //             }

    //             if let Some(path) = cli_template_path {
    //                 config_builder = config_builder.with_file_template_path(&PathBuf::from(path));
    //             }

    //             if let Some(format) = cli_frontmatter_format {
    //                 config_builder = config_builder.with_frontmatter_format(format);
    //             }

    //             config_builder.build().unwrap_or_else(|e| {
    //                 // ERROR
    //                 eprintln!("Error buildig configuration: {e:#?}");
    //                 std::process::exit(1);
    //             })
    //         };

    //         let metadata = FileMetadata::builder()
    //             .with_signature(cli_signature.as_deref())
    //             .with_title(cli_title.as_deref())
    //             .with_keywords(cli_keywords.as_deref())
    //             .with_extension(cli_extension.as_deref())
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

    //             let config_base = load_config(cli_config_path.as_deref()).unwrap_or_else(|e| {
    //                 eprintln!("Error loading configuration: {e:#?}");
    //                 std::process::exit(1);
    //             });

    //             if let Some(base) = config_base {
    //                 config_builder = config_builder.with_base_config(&base);
    //             }

    //             if *cli_regenerate_identifier {
    //                 config_builder = config_builder.with_file_regenerate_identifier(true);
    //             }

    //             if *cli_generate_frontmatter {
    //                 config_builder = config_builder.with_frontmatter_enabled(true);
    //             }

    //             if let Some(ext) = cli_extension {
    //                 config_builder = config_builder.with_file_default_extension(ext);
    //             }

    //             if let Some(format) = cli_frontmatter_format {
    //                 config_builder = config_builder.with_frontmatter_format(format);
    //             }

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
    //             .with_identifier(Some(filename_old.identifier).as_deref())
    //             .with_signature(filename_old.signature.as_deref())
    //             .with_title(filename_old.title.as_deref())
    //             .with_keywords(filename_old.keywords.as_deref())
    //             .with_extension(Some(filename_old.extension).as_deref());

    //         if *cli_rename_from_frontmatter {
    //             let frontmatter_old = frontmatter_old_str.to_frontmatter();

    //             if let Some(title) = frontmatter_old.title {
    //                 metadata_builder = metadata_builder.with_title(title);
    //             }

    //             if let Some(keywords) = frontmatter_old.keywords {
    //                 metadata_builder = metadata_builder.with_keywords(keywords);
    //             }

    //             if let Some(identifier) = frontmatter_old.identifier {
    //                 metadata_builder = metadata_builder.with_identifier(identifier);
    //             }
    //         };

    //         if cli_signature.is_some() {
    //             metadata_builder = metadata_builder.with_signature(cli_signature.as_deref())
    //         }

    //         if cli_title.is_some() {
    //             metadata_builder = metadata_builder.with_title(cli_title.as_deref())
    //         }

    //         if cli_keywords.is_some() {
    //             metadata_builder = metadata_builder.with_keywords(cli_keywords.as_deref())
    //         }

    //         if cli_add_keywords.is_some() {
    //             metadata_builder = metadata_builder.with_added_keywords(cli_add_keywords.as_deref())
    //         }

    //         if cli_remove_keywords.is_some() {
    //             metadata_builder =
    //                 metadata_builder.with_removed_keywords(cli_remove_keywords.as_deref())
    //         }

    //         if cli_extension.is_some() {
    //             metadata_builder = metadata_builder.with_extension(cli_extension.as_deref())
    //         }

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

    stopgap(&cli);

    std::process::exit(0);
}

fn stopgap(cli: &Cli) {
    match &cli.command {
        cli::Commands::New {
            cli_print,
            cli_generate_frontmatter,
            cli_directory_path,
            cli_config_path,
            cli_template_path,
            cli_frontmatter_format,
            cli_signature,
            cli_title,
            cli_extension,
            cli_keywords,
        } => {
            let config = {
                let mut config_builder = Config::builder();

                let config_base = load_config(cli_config_path.as_deref()).unwrap_or_else(|e| {
                    eprintln!("Error loading configuration: {e:#?}");
                    std::process::exit(1);
                });

                if let Some(base) = config_base {
                    config_builder = config_builder.with_base_config(base);
                }

                if *cli_generate_frontmatter {
                    config_builder = config_builder.with_frontmatter_enabled(true);
                }

                if let Some(path) = cli_directory_path {
                    config_builder = config_builder.with_file_directory(path.to_owned());
                }

                if let Some(ext) = cli_extension {
                    config_builder = config_builder.with_file_default_extension(ext.to_owned());
                }

                if let Some(path) = cli_template_path {
                    config_builder = config_builder.with_file_template_path(PathBuf::from(path));
                }

                if let Some(format) = cli_frontmatter_format {
                    config_builder = config_builder.with_frontmatter_format(format.to_owned());
                }

                config_builder.build().unwrap_or_else(|e| {
                    // ERROR
                    eprintln!("Error buildig configuration: {e:#?}");
                    std::process::exit(1);
                })
            };

            let metadata = FileMetadata::builder()
                .with_signature(cli_signature.as_deref())
                .with_title(cli_title.as_deref())
                .with_keywords(cli_keywords.as_deref())
                .with_extension(cli_extension.as_deref())
                // WARN: Possible code smell. Why does metadata take a &FileConfig specifically?
                .build(&config.file);

            let filename = metadata.to_filename(&config.file).to_string();

            let output_path = cli_directory_path
                .clone()
                .map_or(config.file.directory, PathBuf::from)
                .join(filename);

            let _ = fs::write(output_path.clone(), "");

            if *cli_print {
                print!(
                    "{}",
                    output_path.to_str().unwrap_or_else(|| {
                        // ERROR
                        eprintln!("Error printing new file path");
                        std::process::exit(1);
                    })
                )
            };
        }
        _ => (),
    }
}
