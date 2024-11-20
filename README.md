<!--
SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
SPDX-FileContributor: Matthew Mark Ibbetson

SPDX-License-Identifier: GPL-3.0-or-later
-->

# dn

**dn** is a CLI tool for creating and renaming plaintext notes with a predictable, timestamped naming format. It is inspired by the amazing Emacs package [Denote](https://protesilaos.com/emacs/denote) created by [Prot](https://protesilaos.com/). dn creates files that can be easily queried using standard tools like `find`, `sed`, `grep`, `awk`, `ripgrep`, `fzf`, etc.

dn aims to reproduce the file naming functionality of Denote, while being entirely editor agnostic and scriptable in any language. It will adhere to the defaults of Denote as much as possible unless there are justifiable reasons to deviate such as limitation of scope for a CLI or the rare strong opinion of the author.

## Summary

`@@[identifier]==[signature]--[title]__[keywords].[extension]`

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
| :-- | ------: |
|     |         |

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
dn new -k personal_ideas -t "My First Note"

# Rename an existing note
dn rename oldfile.txt -k converted -e md # produces something like 20241006T145030--oldfile__converted.md

# Remove the title and change extension of an existing note
dn rename 20241001T121314--oldtitle__foo.md -e dj - # produces something like 20241001T121314__foo.dj

# Add and remove keywords on an existing file
dn rename 20241001T121314--oldtitle__foo.md -A bar_baz -R baz # produces 20241001T121314--oldtitle__foo_bar.md

# Search $DN_DIRECTORY for a note and then open it in neovim
# this is obviously wrong atm
echo "foo" | rg $DN_DIRECTORY/$1 --file | nvim
```

## Inspirations

- [Denote](https://protesilaos.com/emacs/denote): The Emacs package that inspired this project
- [Zettelkasten](https://zettelkasten.de/introduction/): A method for personal knowledge management
- The Unix Philosophy

## Dependencies

We try to keep dependencies minimal. Ideally, this will become a [cold-blooded]() project and the dependencies will be vendored.

## Development

- We follow [Semantic Versioning](https://semver.org/) for version numbering.
- We use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages.
- We are [REUSE](https://reuse.software/) complaint.
- We aim for [POSIX Compliance](https://pubs.opengroup.org/onlinepubs/9699919799/) where possible.

## Contributing

(Add information about how others can contribute to the project, including guidelines for submitting issues, pull requests, etc.)

## License

dn is licensed under the [GPLv3 License](https://www.gnu.org/licenses/gpl-3.0.en.html), available in this repository in the `LICENSE` file.
