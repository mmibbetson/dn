# dn

**dn** is a CLI tool for creating and renaming plaintext notes with a predictable, timestamped naming format. It is inspired by the amazing Emacs package [Denote](https://protesilaos.com/emacs/denote) created by [Prot](https://protesilaos.com/). dn creates files that can be easily queried using standard tools like `find`, `sed`, `grep`, `awk`, `ripgrep`, `fzf`, etc.

dn aims to reproduce the file naming functionality of Denote, while being entirely editor agnostic and scriptable in any language. It will adhere to the defaults of Denote as much as possible unless there are justifiable reasons to deviate such as limitation of scope for a CLI or the rare strong opinion of the author.

## Summary

**@@IDENTIFIER==SIGNATURE--TITLE\_\_KEYWORDS.EXTENSION**

## Key Terms

| Term       | Definition                                                                                                                |
| :--------- | :------------------------------------------------------------------------------------------------------------------------ |
| Identifier | The first file name fragment by default, prefixed with `@@` if placed elsewhere; used as a unique identifier for the file |
| Signature  | The file name fragment from `==`, a single-part fragment used to uniquely distinguish past the identifier                 |
| Title      | The file name fragment from `--`, also used in original format in the frontmatter                                         |
| Keywords   | The file name fragment from `__`, one-word associations used to categorise files; files can have multiple keywords        |
| Extension  | The file type indicator following the last `.` in the file name                                                           |

## Installation

(Add installation instructions here, e.g., through package managers, building from source, etc.)

### Package Manager

| OS  | Version |
| :-- | :------ |

### Cargo

```sh
# presumably cargo install dn
```

### Binary Download

(Link to releases page)

### Build From Source

```sh
# clone repo and cargo build w/ some flags
```

## Quick Start

```bash
# Create a new note
dn -k personal_ideas "My First Note"

# Rename an existing note
dn -r oldfile.txt -k converted -e md # produces something like 20241006T145030--oldfile__converted.md

# Remove the title and change extension of an existing note
dn -r 20241001T121314--oldtitle__foo.md -e dj - # produces something like 20241001T121314__foo.dj

# Add and remove keywords on an existing file
dn -r 20241001T121314--oldtitle__foo.md -A bar_baz -R baz # produces 20241001T121314--oldtitle__foo_bar.md

# Search $DN_DIRECTORY for a note and then open it in neovim
# this is obviously wrong atm
echo "foo" | rg $DN_DIRECTORY/$1 --file | nvim
```

## CLI Structure

The dn command primarily creates a new file following the Denote naming scheme, provided a string to use for the title. By combining the various option flags, the details of this file can be adjusted along various axes.

### Base Command

```sh
dn
```

### Subcommands

```sh
rename
keywords
```

### Boolean Flags

```sh
-h/--help
-v/--version
-f/--frontmatter
```

### Option Flags

```sh
-d/--directory # Defaults to config file value if present, otherwise ~/dnotes
-o/--order # Defaults to identifier,signature,title,keywords
-O/--frontmatter-order # defaults to title,date,keywords,identifier
-c/--config # Defaults to $XDG_CONFIG_HOME on Unix-like and the equivalent on Windows

# !WARN! -R and -r are mutually exclusive
-r/--rename # accepts an input file to be renamed
-R/--frontmatter-rename # renames file from frontmatter values 

-t/--template # accepts an input file whose contents are to be inserted in the new file, below frontmatter if present
-f/--frontmatter # enable the addition of frontmatter to the created file
-F/--frontmatter-format # Defaults to txt, other valid options are yaml, toml, org
-s/--signature # Omitted if not specified
-e/--extension # Defaults to txt unless --modifying, then defaults to extension of modified file
-k/--keywords # String, can be separated with _ for multiple

# !WARN! -A and -D are ONLY able to be used with -r (NOT -R)
-A/--add-keywords # String, can be separated with _ for multiple
-D/--delete-keywords # String, can be separated with _ for multiple
```

## Configuration

dn looks for one environment variable, `DN_DIRECTORY`. This is the default directory dn will create files unless otherwise specified. If it is not set, it will default to `~/dnotes`. If a value is set in the `dn.toml` configuration file, it will take precedence over the environment variable and the default value.

```toml
[directory]
directory = "~/dnotes"

[file]
segment_order = ["Identifier", "Signature", "Title", "Keywords", "Extension"] # These are not optional, you must specify each segment.
default_extension = "txt" # default extension

[frontmatter]
enabled = false
rewrite = true
format = "txt" # txt, yaml, toml, org
date_time_style = "24h" # or "12h" or "none"
order = ["Title", "Date", "Keywords", "Identifier"] # These are all optional so you can leave some out?

[template]
enabled = false
default_path = "" # default path to a template file

```

## Inspirations

- [Denote](https://protesilaos.com/emacs/denote): The Emacs package that inspired this project
- [Zettelkasten](https://zettelkasten.de/introduction/): A method for personal knowledge management

## Dependencies

We try to keep dependencies minimal. Ideally, this will become a [cold-blooded]() project and the dependencies will be vendored if present.


## Development

- We follow [Semantic Versioning](https://semver.org/) for version numbering.
- We use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages.
- We aim for [POSIX Compliance](https://pubs.opengroup.org/onlinepubs/9699919799/) where possible.

## Contributing

(Add information about how others can contribute to the project, including guidelines for submitting issues, pull requests, etc.)

## License

dn is licensed under the [GPLv3 License](), available in this repository in the `LICENSE` file.
