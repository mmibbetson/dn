use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use cli::Cli;
use config::read_config;
use config::Config;
use directory::get_default_config_dir;
use filename::Filename;
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
            print_path,
            generate_frontmatter,
            directory,
            config,
            template,
            frontmatter_format,
            signature,
            title,
            extension,
            keywords,
        } => {
            let config_path = config.map_or(get_default_config_dir(), PathBuf::from);
            let config_content = read_config(config_path).unwrap_or_default();
            let config_final = update_config_with_cli_args(cli.command.clone(), &config_content);

            let creation_time = chrono::Local::now();
            let metadata = FileMetadataBuilder::new(creation_time)
                .with_signature(signature)
                .with_title(title)
                .with_keywords(keywords)
                .with_extension(extension)
                .build(&config_final.file);

            let filename = metadata.to_filename(&config_final.file).to_string();
            let frontmatter = generate_frontmatter.then(|| {
                metadata
                    .to_frontmatter(&config_final.frontmatter)
                    .to_string()
            });

            let template_content = template.map(|tmp| get_template(tmp, config_final.file));

            let path = directory
                .map_or(get_path(config_final.file.directory), PathBuf::from)
                .join(filename);

            let content = get_content(frontmatter, template_content);

            fs::write(path, content);

            print_path.then(|| print!("{}", path));
        }
        cli::Commands::Rename {
            input,
            print_path,
            regenerate_identifier,
            frontmatter,
            generate_frontmatter,
            config,
            frontmatter_format,
            signature,
            title,
            extension,
            keywords,
            add_keywords,
            remove_keywords,
        } => {
            let input_path = PathBuf::from(input);
            // WARN: Unwrapping may panic.
            let input_content = fs::read_to_string(input_path).unwrap();

            let config_path = config.map_or(get_default_config_dir(), PathBuf::from);
            let config_content = read_config(config_path).unwrap_or_default();
            let config_final = update_config_with_cli_args(cli.command.clone(), &config_content);

            let file_name = PathBuf::from(input)
                .file_name()
                .and_then(|o| o.to_str())
                // WARN: Unwrap may panic.
                .unwrap()
                .to_string();

            let mut existing_filename = file_name.to_filename(&config_final.file);
            let mut parse_time = derive_creation_time(&existing_filename.identifier);

            frontmatter.then(|| {
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
            signature.map(|s| existing_filename.signature = Some(s));
            title.map(|t| existing_filename.title = Some(t));
            keywords.map(|k| existing_filename.keywords = Some(k));
            add_keywords.map(|k| {
                // k needs to be deserialised and sanitised then concatenated onto existing_filename.keywords.
            });
            remove_keywords.map(|k| {
                // k needs to be deserialised and sanitised then existing_filename.keywords.iter().filter(!k.contains)
            });
            extension.map(|e| existing_filename.extension = e);

            let metadata = FileMetadataBuilder::new(parse_time)
                .with_identifier(&Some(existing_filename.identifier))
                .with_signature(&existing_filename.signature)
                .with_title(&existing_filename.title)
                .with_keywords(&existing_filename.keywords)
                .with_extension(&Some(existing_filename.extension))
                // WARN: Possible code smell. Why does metadata take a FILE config?
                .build(&config_final.file);

            let new_filename = metadata.to_filename(&config_final.file).to_string();
            let new_frontmatter = generate_frontmatter.then(|| {
                metadata
                    .to_frontmatter(config_final.frontmatter)
                    .to_string()
            });

            // WARN: Unwrap may panic.
            let output_path = input_path.parent().unwrap().join(new_filename);
            let output_content = get_rename_content(new_frontmatter, input_content);

            fs::write(input_path, output_content);
            fs::rename(input_path, output_path);

            // WARN: Unwrap may panic.
            print_path.then(|| print!("{}", output_path.to_str().unwrap()));
        }
    }
}

fn update_config_with_cli_args(args: cli::Commands, original_config: &Config) -> Config {
    let mut config = original_config.clone();

    match args {
        cli::Commands::New {
            generate_frontmatter,
            directory,
            template,
            frontmatter_format,
            extension,
            print_path: _,
            config: _,
            signature: _,
            title: _,
            keywords: _,
        } => {
            generate_frontmatter.then(|| config.frontmatter.enabled = generate_frontmatter);

            directory.map(|d| config.file.directory = PathBuf::from(d));
            extension.map(|e| config.file.default_extension = e);
            template.map(|t| config.file.template_path = Some(PathBuf::from(t)));

            frontmatter_format.map(|f| {
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
            regenerate_identifier,
            generate_frontmatter,
            frontmatter_format,
            extension,
            input: _,
            print_path: _,
            frontmatter: _,
            config: _,
            signature: _,
            title: _,
            keywords: _,
            add_keywords: _,
            remove_keywords: _,
        } => {
            regenerate_identifier
                .then(|| config.file.regenerate_identifier = regenerate_identifier);
            generate_frontmatter.then(|| config.frontmatter.rewrite = generate_frontmatter);

            extension.map(|e| config.file.default_extension = e);

            frontmatter_format.map(|f| {
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
