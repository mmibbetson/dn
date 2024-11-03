use std::fs;
use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use config::read_config;
use config::Config;
use directory::get_default_config_dir;
use metadata::FileMetadataBuilder;

mod cli;
mod config;
mod directory;
mod file;
mod format;
mod metadata;

// Top-down draft using api
fn main() {
    let cli = Cli::parse();
    let instance_time = chrono::Local::now();

    match &cli.command {
        cli::Commands::New {
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
            let config_path = match config {
                Some(path) => PathBuf::from(path),
                None => get_default_config_dir(),
            };

            let config_content = match read_config(config_path) {
                Ok(content) => content,
                Err(_) => Config::default(),
            };

            // WARN: This clones the command struct and then matches on it again...
            let config_final = update_config_with_cli_args(cli.command.clone(), &config_content);

            let metadata = FileMetadataBuilder::new(instance_time)
                .with_signature(signature)
                .with_title(title)
                .with_keywords(keywords)
                .with_extension(extension)
                .build(&config_final.file);

            // let filename = get_filename(metadata, config_content.file);

            // // let frontmatter = get_frontmatter(metadata, config.frontmatter_config); // optional
            // // let template = get_template(template_path, config.template_config); // optional

            // let path = get_path(config_content.directory_config);
            // let content = get_content(frontmatter, template);

            // fs::write(path, content);
        }
        cli::Commands::Rename {
            input,
            regenerate_identifier,
            frontmatter,
            generate_frontmatter,
            config,
            frontmatter_format,
            signature,
            extension,
            keywords,
            add_keywords,
            remove_keywords,
        } => {}
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
            config: _,
            signature: _,
            title: _,
            keywords: _,
        } => {
            if generate_frontmatter {
                config.frontmatter.enabled = generate_frontmatter;
            };

            if let Some(dir) = directory {
                config.file.directory = PathBuf::from(dir);
            };

            if let Some(ext) = extension {
                config.file.default_extension = ext;
            };

            if let Some(tmp) = template {
                config.file.template_path = Some(PathBuf::from(tmp));
            }

            if let Some(fmt) = frontmatter_format {
                config.frontmatter.format = match fmt.to_lowercase().as_str() {
                    "text" => config::FrontmatterFormat::Text,
                    "yaml" => config::FrontmatterFormat::YAML,
                    "toml" => config::FrontmatterFormat::TOML,
                    "org" => config::FrontmatterFormat::Org,
                    // WARN: Panicking.
                    // TODO: Maybe throw anyhow error alert invalid format, or something?
                    _ => panic!("Invalid frontmatter format provided, must be one of: text, yaml, toml, org"), 
                };
            }
        }
        cli::Commands::Rename {
            regenerate_identifier,
            generate_frontmatter,
            frontmatter_format,
            extension,
            input: _,
            frontmatter: _,
            config: _,
            signature: _,
            keywords: _,
            add_keywords: _,
            remove_keywords: _,
        } => {
            if regenerate_identifier {
                config.file.regenerate_identifier = regenerate_identifier;
            };

            if generate_frontmatter {
                config.frontmatter.rewrite = generate_frontmatter;
            };

            if let Some(ext) = extension {
                config.file.default_extension = ext;
            };

            if let Some(fmt) = frontmatter_format {
                config.frontmatter.format = match fmt.to_lowercase().as_str() {
                    "text" => config::FrontmatterFormat::Text,
                    "yaml" => config::FrontmatterFormat::YAML,
                    "toml" => config::FrontmatterFormat::TOML,
                    "org" => config::FrontmatterFormat::Org,
                    _ => panic!("Invalid frontmatter format provided, must be one of: text, yaml, toml, org"), // TODO: throw anyhow error alert invalid format, or something
                };
            }
        }
    };

    config
}

// When getting --add-keywords or --remove-keywords we want to modify the keywords_arg
// 1. Take existing keywords_arg (-k>filename_parse>None)
// 2. format!("{}_{}", keywords_arg, add_keywords_arg)
// 3. split words in remove_keywords_arg
// 4. iterate over formatted string, remove instances of remove keywords arg words from [3]
// 5. collect properly into nice single string, to be used as final keywords_arg value

// When renaming a file, be sure to parse the filename correctly out of the input path
// and rejoin if necessary when writing to disk or w/e.

// When --regenerate-identifier is false, check get_identifier() and if the file has an identifier
// we will use that to get the creation_time and identifier_arg. This way, frontmatter.rs can be
// agnostic wrt the identifier arg.

// let example = FileMetadata {
//     identifier: "20241031T232930",
//     signature: "ggl210",
//     title_raw: "Sprint Goals! - 210",
//     title: "sprint-goals-210",
//     keywords: vec!["ggl", "client-ADmtars!!", "admin"],
//     extension: "md",
//     datetime: chrono::Local::now(),
// }

// 20241031T232930==GGL210--sprint-goals-210__ggl_client_admin.md

// NOTE: It's important that tags/filetags are named as such rather than keywords
// This is due to the way various existing programs parse them.

// ---
// title:      "Sprint Goals - 210"
// date:       "2024-10-31 23:34:30 +2:00"
// tags:       ["ggl", "client", "admin"]
// identifier: "20241031T232930"
// ---

// +++
// title =      "Sprint Goals - 210"
// date =       "2024-10-31 23:34:30 +2:00"
// tags =       ["ggl", "client", "admin"]
// identifier = "20241031T232930"
// +++

// title:      Sprint Goals - 210
// date:       2024-10-31 23:34:30 +2:00
// tags:       ggl  client  admin
// identifier: "20241031T232930"
// -----------------------------

// #+title:      Sprint Goals - 210
// #+date:       [2024-10-31 23:34:30 +2:00]
// #+filetags:   :ggl:client:admin:
// #+identifier: 20241031T232930
