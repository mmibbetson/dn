use std::fs;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use config::Config;
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
            let config = {
                let config_builder = Config::builder(cli_config_path.clone());

                config_builder = cli_config_path
                    .map(|p| config_builder.with_config_path(p))
                    .unwrap_or(config_builder);

                config_builder = cli_generate_frontmatter
                    .then(|| config_builder.with_frontmatter_enabled(true))
                    .unwrap_or(config_builder);

                config_builder = cli_directory_path
                    .map(|p| config_builder.with_file_directory(p))
                    .unwrap_or(config_builder);

                config_builder = cli_extension
                    .map(|e| config_builder.with_file_default_extension(e))
                    .unwrap_or(config_builder);

                config_builder = cli_template_path
                    .map(|p| config_builder.with_file_template_path(p))
                    .unwrap_or(config_builder);

                config_builder = cli_frontmatter_format
                    .map(|f| config_builder.with_frontmatter_format(f))
                    .unwrap_or(config_builder);

                config_builder.build()
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
            let input_content = match fs::read_to_string(input_path) {
                Ok(path) => path,
                Err(error) => {
                    eprintln!("Error reading input file: {}", error);
                    std::process::exit(1);
                }
            };

            let config = {
                let config_builder = Config::builder(cli_config_path.clone());

                config_builder = cli_regenerate_identifier
                    .then(|| config_builder.with_file_regenerate_identifier(true))
                    .unwrap_or(config_builder);

                config_builder = cli_generate_frontmatter
                    .then(|| config_builder.with_frontmatter_enabled(true))
                    .unwrap_or(config_builder);

                config_builder = cli_extension
                    .map(|e| config_builder.with_file_default_extension(e))
                    .unwrap_or(config_builder);

                config_builder = cli_frontmatter_format
                    .map(|f| config_builder.with_frontmatter_format(f))
                    .unwrap_or(config_builder);

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

                metadata_builder = existing_frontmatter
                    .title
                    .map(|t| metadata_builder.with_title(t))
                    .unwrap_or(metadata_builder);

                metadata_builder = existing_frontmatter
                    .keywords
                    .map(|k| metadata_builder.with_keywords(k))
                    .unwrap_or(metadata_builder);

                metadata_builder = existing_frontmatter
                    .identifier
                    .map(|i| metadata_builder.with_identifier(i))
                    .unwrap_or(metadata_builder);
            };

            metadata_builder = cli_signature
                .map(|s| metadata_builder.with_signature(&Some(s)))
                .unwrap_or(metadata_builder);

            metadata_builder = cli_title
                .map(|t| metadata_builder.with_title(&Some(t)))
                .unwrap_or(metadata_builder);

            metadata_builder = cli_keywords
                .map(|k| metadata_builder.with_keywords(&Some(k)))
                .unwrap_or(metadata_builder);

            metadata_builder = cli_add_keywords
                .map(|k| {
                    // TODO: Deserialised and sanitise then concatenated onto existing_filename.keywords
                    todo!()
                })
                .unwrap_or(metadata_builder);

            metadata_builder = cli_remove_keywords
                .map(|k| {
                    // TODO: Deserialise and sanitise then existing_filename.keywords.iter().filter(!k.contains)
                    todo!()
                })
                .unwrap_or(metadata_builder);

            metadata_builder = cli_extension
                .map(|e| metadata_builder.with_extension(&Some(e)))
                .unwrap_or(metadata_builder);

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
