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
mod file;
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
                .then(|| metadata.to_frontmatter(&config.frontmatter).to_string());

            let template = cli_template_path.map(|tmp| get_template(tmp, config.file));

            let path = cli_directory_path
                .map_or(get_path(config.file.directory), PathBuf::from)
                .join(filename);

            let content = get_content(frontmatter, template);

            fs::write(path, content);

            cli_print.then(|| print!("{}", path));
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
            // TODO: What happens if someone renames based on frontmatter AND regenerates frontmatter?
            // Will need many integration tests.

            let input_path = PathBuf::from(input);
            // WARN: Unwrapping may panic.
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

            cli_rename_from_frontmatter.then(|| {
                // NOTE: We take the first 6 lines only because at most that is how many lines
                // of dn frontmatter will be present (in the case of YAML and TOML with all fields)
                let existing_frontmatter = input_content.lines().take(6).collect().to_frontmatter();
                existing_frontmatter.title.map(|t| existing_filename.title);
                existing_frontmatter.datetime.map(|d| parse_time = d);
                existing_frontmatter
                    .keywords
                    .map(|k| existing_filename.keywords);
                existing_frontmatter
                    .identifier
                    .map(|i| existing_filename.identifier = i);
            });

            // WARN: Side-effecting mutation.
            cli_signature.map(|s| existing_filename.signature = Some(s));
            cli_title.map(|t| existing_filename.title = Some(t));
            cli_keywords.map(|k| existing_filename.keywords = Some(k));
            cli_add_keywords.map(|k| {
                // TODO: k needs to be deserialised and sanitised then concatenated onto existing_filename.keywords.
            });
            cli_remove_keywords.map(|k| {
                // TODO: k needs to be deserialised and sanitised then existing_filename.keywords.iter().filter(!k.contains)
            });
            cli_extension.map(|e| existing_filename.extension = e);

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
                .then(|| metadata.to_frontmatter(config.frontmatter).to_string());

            // WARN: Unwrap may panic.
            let output_path = input_path.parent().unwrap().join(new_filename);
            let output_content = get_rename_content(new_frontmatter, input_content);

            fs::write(input_path, output_content);
            fs::rename(input_path, output_path);

            // WARN: Unwrap may panic.
            cli_print.then(|| print!("{}", output_path.to_str().unwrap()));
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
            cli_generate_frontmatter.then(|| config.frontmatter.enabled = cli_generate_frontmatter);

            cli_directory_path.map(|d| config.file.directory = PathBuf::from(d));
            cli_extension.map(|e| config.file.default_extension = e);
            cli_template_path.map(|t| config.file.template_path = Some(PathBuf::from(t)));

            cli_frontmatter_format.map(|f| {
                config.frontmatter.format = match f.to_lowercase().as_str() {
                    "text" => config::FrontmatterFormat::Text,
                    "yaml" => config::FrontmatterFormat::YAML,
                    "toml" => config::FrontmatterFormat::TOML,
                    "org" => config::FrontmatterFormat::Org,
                    // WARN: Panicking. Maybe throw anyhow error alert invalid format, or something?
                    _ => panic!("Invalid frontmatter format provided, must be one of: text, yaml, toml, org"),
                };
            });
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
            cli_regenerate_identifier
                .then(|| config.file.regenerate_identifier = cli_regenerate_identifier);
            cli_generate_frontmatter.then(|| config.frontmatter.rewrite = cli_generate_frontmatter);

            cli_extension.map(|e| config.file.default_extension = e);

            cli_frontmatter_format.map(|f| {
                config.frontmatter.format = match f.to_lowercase().as_str() {
                    "text" => config::FrontmatterFormat::Text,
                    "yaml" => config::FrontmatterFormat::YAML,
                    "toml" => config::FrontmatterFormat::TOML,
                    "org" => config::FrontmatterFormat::Org,
                    // WARN: Panicking. Maybe throw anyhow error alert invalid format, or something?
                    _ => panic!("Invalid frontmatter format provided, must be one of: text, yaml, toml, org"),
                };
            });
        }
    };

    config
}

///////////
// Tests //
///////////

#[cfg(test)]
mod tests {}
