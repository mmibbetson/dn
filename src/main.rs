// SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
// SPDX-FileContributor: Matthew Mark Ibbetson
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! CLI tool for managing notes in a minimalistic, cross-platform, free, extensible manner.

use std::{fs, path::PathBuf};

use anyhow::{anyhow, Error};
use clap::Parser;
use cli::Cli;
use config::{load_config, Config};
use directory::safe_write;
use filename::ToFilename;
use metadata::FileMetadata;

mod cli;
mod config;
mod directory;
mod filename;
mod metadata;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        cli::Commands::New {
            cli_print,
            cli_directory_path,
            cli_config_path,
            cli_template_path,
            cli_signature,
            cli_title,
            cli_extension,
            cli_keywords,
        } => {
            let config = {
                let mut config_builder = Config::builder();

                let config_base = load_config(cli_config_path.as_deref())
                    .map_err(|e| anyhow!(e).context("Error loading configuration"))?;

                if let Some(base) = config_base {
                    config_builder = config_builder.with_base_config(base);
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

                config_builder.build()
            };

            let metadata = FileMetadata::builder()
                .with_signature(cli_signature.as_deref())
                .with_title(cli_title.as_deref())
                .with_keywords(cli_keywords.as_deref())
                .with_extension(cli_extension.as_deref())
                .build(&config);

            let filename = metadata.to_filename(&config).to_string();
            let template = cli_template_path.as_ref().map_or(Ok(Vec::new()), |p| {
                fs::read(p).map_err(|e| anyhow!(e).context("Error reading template file"))
            })?;

            let output_path = cli_directory_path
                .as_ref()
                .map_or(config.directory, PathBuf::from)
                .join(filename);

            safe_write(&output_path, &template)?;

            if *cli_print {
                print!(
                    "{}",
                    output_path
                        .to_str()
                        .ok_or_else(|| anyhow!("Error printing new file path"))?
                );
            };
        }
        cli::Commands::Rename {
            input,
            cli_print,
            cli_regenerate_identifier,
            cli_config_path,
            cli_signature,
            cli_title,
            cli_extension,
            cli_keywords,
            cli_add_keywords,
            cli_remove_keywords,
        } => {
            let config = {
                let mut config_builder = Config::builder();

                let config_base = load_config(cli_config_path.as_deref())
                    .map_err(|e| anyhow!(e).context("Error loading configuration"))?;

                if let Some(base) = config_base {
                    config_builder = config_builder.with_base_config(base.clone());
                }

                if *cli_regenerate_identifier {
                    config_builder = config_builder.with_file_regenerate_identifier(true);
                }

                if let Some(ext) = cli_extension {
                    config_builder = config_builder.with_file_default_extension(ext.to_owned());
                }

                config_builder.build()
            };

            let input_path = PathBuf::from(input);
            let input_content = fs::read_to_string(&input_path)
                .map_err(|e| anyhow!(e).context("Error reading input file"))?;

            let filename_old = PathBuf::from(input)
                .file_name()
                .ok_or_else(|| anyhow!("Error reading file name: Could not parse path"))?
                .to_str()
                .ok_or_else(|| {
                    anyhow!("Error reading file name: Filename is not in a valid format")
                })?
                .to_owned()
                .to_filename(&config);

            let mut metadata_builder = FileMetadata::builder()
                .with_identifier(Some(filename_old.identifier).as_deref())
                .with_signature(filename_old.signature.as_deref())
                .with_title(filename_old.title.as_deref())
                .with_keywords(filename_old.keywords.as_deref())
                .with_extension(Some(filename_old.extension).as_deref());

            if cli_signature.is_some() {
                metadata_builder = metadata_builder.with_signature(cli_signature.as_deref());
            }

            if cli_title.is_some() {
                metadata_builder = metadata_builder.with_title(cli_title.as_deref());
            }

            if cli_keywords.is_some() {
                metadata_builder = metadata_builder.with_keywords(cli_keywords.as_deref());
            }

            if cli_add_keywords.is_some() {
                metadata_builder =
                    metadata_builder.with_added_keywords(cli_add_keywords.as_deref());
            }

            if cli_remove_keywords.is_some() {
                metadata_builder =
                    metadata_builder.with_removed_keywords(cli_remove_keywords.as_deref());
            }

            if cli_extension.is_some() {
                metadata_builder = metadata_builder.with_extension(cli_extension.as_deref());
            }

            let metadata = metadata_builder.build(&config);

            let filename_new = metadata.to_filename(&config).to_string();

            let output_path = input_path
                .parent()
                .ok_or_else(|| {
                    anyhow!(
                        "Error reading file directory: Could not parse input file parent directory"
                    )
                })?
                .join(filename_new);

            fs::rename(&input_path, &output_path)?;
            safe_write(&output_path, &input_content)?;

            if *cli_print {
                print!(
                    "{}",
                    output_path
                        .to_str()
                        .ok_or_else(|| anyhow!("Error printing new file path"))?
                );
            };
        }
    }

    Ok(())
}
