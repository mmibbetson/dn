use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Error};
use clap::Parser;
use cli::Cli;
use config::read_config;
use config::Config;
use config::FrontmatterFormat;
use directory::default_config_directory_from_environment;
use directory::get_default_config_dir;
use filename::ToFilename;
use metadata::FileMetadata;

mod cli;
mod config;
mod directory;
mod filename;
mod format;
mod metadata;

fn main() {
    let cli = Cli::parse();

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
            // TODO: Consider creating a ConfigBuilder and then allowing a more natural mutation interface.
            let config = {
                // TODO: If this fails because the config file doesn't exist, fall back to defaults.
                // TODO: If it fails for any other reason, crash with error message.
                let config_path = cli_config_path
                    .map_or(default_config_directory_from_environment(), PathBuf::from);
                // WARN: Unwrap may panic. Do we want to alert the user of a misconfiguration?
                let mut config_content = read_config(config_path).unwrap();

                if *cli_generate_frontmatter {
                    config_content.frontmatter.enabled = *cli_generate_frontmatter;
                };
                if let Some(directory) = cli_directory_path {
                    config_content.file.directory = PathBuf::from(directory);
                };
                if let Some(extension) = cli_extension {
                    config_content.file.default_extension = extension.to_string();
                };
                if let Some(template) = cli_template_path {
                    config_content.file.template_path = Some(PathBuf::from(template));
                };
                if let Some(format) = cli_frontmatter_format {
                    config_content.frontmatter.format = determine_frontmatter_format(&format);
                };

                config_content
            };

            let metadata = FileMetadata::builder()
                .with_signature(cli_signature)
                .with_title(cli_title)
                .with_keywords(cli_keywords)
                .with_extension(cli_extension)
                .build(&config.file);

            let filename = metadata.to_filename(&config.file).to_string();
            let frontmatter = cli_generate_frontmatter
                .then(|| metadata.to_frontmatter(&config.frontmatter).to_string());
            let template = cli_template_path.map(fs::read);

            let path = cli_directory_path
                .map_or(PathBuf::from(config.file.directory), PathBuf::from)
                .join(filename);
            let content = concatenate_file_content(frontmatter, template);

            fs::write(path, content);

            if *cli_print {
                // WARN: Unwrap may panic. Do we want to alert the user of a problem converting the output path to a string?
                print!("{}", path.to_str().unwrap())
            };
        }
        cli::Commands::Rename {
            input,
            cli_print,
            cli_regenerate_identifier,
            cli_rename_from_frontmatter,
            cli_generate_frontmatter,
            cli_config_path,
            cli_frontmatter_format,
            cli_signature,
            cli_title,
            cli_extension,
            cli_keywords,
            cli_add_keywords,
            cli_remove_keywords,
        } => {
            let input_path = PathBuf::from(input);
            // WARN: Unwrap may panic. Do we want to alert the user of an invalid input?
            let input_content = fs::read_to_string(input_path).unwrap();

            let config = {
                let config_builder = Config::builder(cli_config_path.clone());

                if *cli_regenerate_identifier {
                    config_builder =
                        config_builder.with_file_regenerate_identifier(cli_regenerate_identifier);
                };

                if *cli_generate_frontmatter {
                    config_builder =
                        config_builder.with_frontmatter_enabled(cli_generate_frontmatter);
                };

                if let Some(extension) = cli_extension {
                    config_builder = config_builder.file_default_extension(extension.to_string());
                };

                if let Some(format) = cli_frontmatter_format {
                    let fmt = match determine_frontmatter_format(&format) {
                        Ok(fmt) => fmt,
                        Err(error) => {
                            eprintln!("Error determining frontmatter format: {}", error);
                            std::process::exit(1);
                        }
                    };

                    config_builder = config_builder.with_frontmatter_format(fmt);
                };

                config_builder.build()
            };

            let old_file_name = PathBuf::from(input)
                .file_name()
                .and_then(|o| o.to_str())
                // WARN: Unwrap may panic. Do we want to alert the user of a parsing error in the filename?
                .unwrap()
                .to_string()
                .to_filename(&config.file);

            let mut metadata_builder = FileMetadata::builder()
                .with_identifier(&Some(old_file_name.identifier))
                .with_signature(&old_file_name.signature)
                .with_title(&old_file_name.title)
                .with_keywords(&old_file_name.keywords)
                .with_extension(&Some(old_file_name.extension));

            if *cli_rename_from_frontmatter {
                let existing_frontmatter = input_content.to_frontmatter();

                if let Some(title) = existing_frontmatter.title {
                    metadata_builder = metadata_builder.with_title(title);
                }

                if let Some(keywords) = existing_frontmatter.keywords {
                    metadata_builder = metadata_builder.with_keywords(keywords);
                }

                if let Some(identifier) = existing_frontmatter.identifier {
                    metadata_builder = metadata_builder.with_identifier(identifier);
                }
            };

            if cli_signature.is_some() {
                metadata_builder = metadata_builder.with_signature(cli_signature);
            };

            if cli_title.is_some() {
                metadata_builder = metadata_builder.with_title(cli_title);
            };

            if cli_keywords.is_some() {
                metadata_builder = metadata_builder.with_keywords(cli_keywords);
            };

            if cli_add_keywords.is_some() {
                // TODO: Deserialised and sanitise then concatenated onto existing_filename.keywords
            };

            if cli_remove_keywords.is_some() {
                // TODO: Deserialise and sanitise then existing_filename.keywords.iter().filter(!k.contains)
            };

            if cli_extension.is_some() {
                metadata_builder = metadata_builder.with_extension(cli_extension);
            };

            // WARN: Possible code smell. Why does metadata take a &FileConfig specifically?
            let metadata = metadata_builder.build(&config.file);

            let new_filename = metadata.to_filename(&config.file).to_string();
            let new_frontmatter = cli_generate_frontmatter
                .then(|| metadata.to_frontmatter(config.frontmatter).to_string());

            // WARN: Unwrap may panic. Do we want to alert a user of a problem getting the parent path?
            // This may be impossible assuming previous path operations succeeded?
            let output_path = input_path.parent().unwrap().join(new_filename);
            // TODO: concatenate_rename_content : (Option<AsRef<[u8]>>, Option<AsRef<[u8]>>) -> Option<AsRef<[u8]>>
            let output_content = concatenate_rename_content(new_frontmatter, input_content);

            fs::write(input_path, output_content);
            fs::rename(input_path, output_path);

            if *cli_print {
                // WARN: Unwrap may panic. Do we want to alert the user of a problem converting the output path to a string?
                print!("{}", output_path.to_str().unwrap())
            };
        }
    }
}

fn determine_frontmatter_format(format_arg: &str) -> Result<FrontmatterFormat, Error> {
    match format_arg.to_lowercase().as_str() {
        "text" => Ok(FrontmatterFormat::Text),
        "yaml" => Ok(FrontmatterFormat::YAML),
        "toml" => Ok(FrontmatterFormat::TOML),
        "org" => Ok(FrontmatterFormat::Org),
        _ => Err(anyhow!(
            "Invalid frontmatter format provided, must be one of: text, yaml, toml, org"
        )),
    }
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
