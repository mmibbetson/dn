use std::fs;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use config::read_config;
use config::Config;
use directory::get_default_config_dir;
use filename::ToFilename;
use metadata::derive_creation_time;
use metadata::FileMetadataBuilder;

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
                let config_path = cli_config_path.map_or(get_default_config_dir(), PathBuf::from);
                let config_content = read_config(config_path).unwrap_or_default();

                update_config_with_cli_args(cli.command.clone(), &config_content)
            };

            let creation_time = chrono::Local::now();
            let metadata = FileMetadataBuilder::new(creation_time)
                .with_signature(cli_signature)
                .with_title(cli_title)
                .with_keywords(cli_keywords)
                .with_extension(cli_extension)
                .build(&config.file);

            let filename = metadata.to_filename(&config.file).to_string();
            let frontmatter = cli_generate_frontmatter
                .then(metadata.to_frontmatter(&config.frontmatter).to_string());
            let template = cli_template_path.map(fs::read);

            let path = cli_directory_path
                .map_or(get_path(config.file.directory), PathBuf::from)
                .join(filename);
            let content = get_content(frontmatter, template);

            fs::write(path, content);

            if *cli_print {
                print!("{}", path)
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
            // WARN: Unwrap may panic.
            let input_content = fs::read_to_string(input_path).unwrap();

            let config = {
                let config_path = cli_config_path.map_or(get_default_config_dir(), PathBuf::from);
                let config_content = read_config(config_path).unwrap_or_default();

                update_config_with_cli_args(cli.command.clone(), &config_content)
            };

            let file_name = PathBuf::from(input)
                .file_name()
                .and_then(|o| o.to_str())
                // WARN: Unwrap may panic.
                .unwrap()
                .to_string();

            let mut existing_filename = file_name.to_filename(&config.file);
            let mut parse_time = derive_creation_time(&existing_filename.identifier);

            if *cli_rename_from_frontmatter {
                // NOTE: We take the first 6 lines only because at most that is how many lines
                // of dn frontmatter will be present (in the case of YAML and TOML with all fields)
                let existing_frontmatter = input_content.lines().take(6).collect().to_frontmatter();
                if let Some(t) = existing_frontmatter.title {
                    existing_filename.title = t;
                }
                if let Some(d) = existing_frontmatter.datetime {
                    parse_time = d;
                }
                if let Some(k) = existing_frontmatter.keywords {
                    existing_filename.keywords = k;
                }
                if let Some(i) = existing_frontmatter.identifier {
                    existing_filename.identifier = i;
                }
            };

            if cli_signature.is_some() {
                existing_filename.signature = cli_signature.clone();
            };
            if cli_title.is_some() {
                existing_filename.title = cli_title.clone();
            };
            if cli_keywords.is_some() {
                existing_filename.keywords = cli_keywords.clone();
            };
            if cli_add_keywords.is_some() {
                // TODO: Deserialised and sanitise then concatenated onto existing_filename.keywords
            };
            if cli_remove_keywords.is_some() {
                // TODO: Deserialise and sanitise then existing_filename.keywords.iter().filter(!k.contains)
            };
            if let Some(e) = cli_extension {
                existing_filename.extension = e.to_owned();
            };

            let metadata = FileMetadataBuilder::new(parse_time)
                .with_identifier(&Some(existing_filename.identifier))
                .with_signature(&existing_filename.signature)
                .with_title(&existing_filename.title)
                .with_keywords(&existing_filename.keywords)
                .with_extension(&Some(existing_filename.extension))
                // WARN: Possible code smell. Why does metadata take a &FileConfig specifically?
                .build(&config.file);

            let new_filename = metadata.to_filename(&config.file).to_string();
            let new_frontmatter = cli_generate_frontmatter
                .then(metadata.to_frontmatter(config.frontmatter).to_string());

            // WARN: Unwrap may panic.
            let output_path = input_path.parent().unwrap().join(new_filename);
            let output_content = concatenate_rename_content(new_frontmatter, input_content);

            fs::write(input_path, output_content);
            fs::rename(input_path, output_path);

            if *cli_print {
                // WARN: Unwrap may panic.
                print!("{}", output_path.to_str().unwrap())
            };
        }
    }
}

fn update_config_with_cli_args(args: cli::Commands, original_config: &Config) -> Config {
    let mut config = original_config.clone();

    match args {
        cli::Commands::New {
            cli_generate_frontmatter,
            cli_directory_path,
            cli_template_path,
            cli_frontmatter_format,
            cli_extension,
            cli_print: _,
            cli_config_path: _,
            cli_signature: _,
            cli_title: _,
            cli_keywords: _,
        } => {
            if cli_generate_frontmatter {
                config.frontmatter.enabled = cli_generate_frontmatter;
            };
            if let Some(d) = cli_directory_path {
                config.file.directory = PathBuf::from(d);
            };
            if let Some(e) = cli_extension {
                config.file.default_extension = e;
            };
            if let Some(t) = cli_template_path {
                config.file.template_path = Some(PathBuf::from(t));
            };
            if let Some(f) = cli_frontmatter_format {
                config.frontmatter.format = match f.to_lowercase().as_str() {
                    "text" => config::FrontmatterFormat::Text,
                    "yaml" => config::FrontmatterFormat::YAML,
                    "toml" => config::FrontmatterFormat::TOML,
                    "org" => config::FrontmatterFormat::Org,
                    // WARN: Panicking. Maybe throw anyhow error alert invalid format, or something?
                    _ => panic!("Invalid frontmatter format provided, must be one of: text, yaml, toml, org"),
                };
            };
        }
        cli::Commands::Rename {
            cli_regenerate_identifier,
            cli_generate_frontmatter,
            cli_frontmatter_format,
            cli_extension,
            input: _,
            cli_print: _,
            cli_rename_from_frontmatter: _,
            cli_config_path: _,
            cli_signature: _,
            cli_title: _,
            cli_keywords: _,
            cli_add_keywords: _,
            cli_remove_keywords: _,
        } => {
            if cli_regenerate_identifier {
                config.file.regenerate_identifier = cli_regenerate_identifier;
            };
            if cli_generate_frontmatter {
                config.frontmatter.rewrite = cli_generate_frontmatter;
            };
            if let Some(e) = cli_extension {
                config.file.default_extension = e;
            };
            if let Some(f) = cli_frontmatter_format {
                config.frontmatter.format = match f.to_lowercase().as_str() {
                    "text" => config::FrontmatterFormat::Text,
                    "yaml" => config::FrontmatterFormat::YAML,
                    "toml" => config::FrontmatterFormat::TOML,
                    "org" => config::FrontmatterFormat::Org,
                    // WARN: Panicking. Maybe throw anyhow error alert invalid format, or something?
                    _ => panic!("Invalid frontmatter format provided, must be one of: text, yaml, toml, org"),
                };
            };
        }
    };

    config
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
