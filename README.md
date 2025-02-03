<!--
SPDX-FileCopyrightText: 2024 Matthew Mark Ibbetson
SPDX-FileContributor: Matthew Mark Ibbetson

SPDX-License-Identifier: GPL-3.0-or-later
-->

# dn

dn is a CLI tool for creating and renaming plaintext notes with a predictable, timestamped naming format. It is inspired by the amazing Emacs package [Denote](https://protesilaos.com/emacs/denote) created by [Prot](https://protesilaos.com/). dn creates files that can be easily queried using standard tools like `find`, `sed`, `grep`, `awk`, `ripgrep`, `fzf`, etc.

dn aims to reproduce the file naming functionality of Denote, while being entirely editor agnostic and scriptable in any language. It deviates in several ways from the behaviour of Denote, both due to the reduced scope (much of Denote's functionality is best suited for editor integrations) and different concerns (we feel that Emacs & Org already have a complete, quality solution to practically all of the goals dn addresses, i.e. Denote).

## Summary

TODO


## Installation

(Add installation instructions here, e.g., through package managers, building from source, etc.)

### Package Manager

TODO: Repology

### Container Image

TODO

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
rg "--dn-example__metanote" $DN_DIRECTORY/$1 --file | nvim
```

## Shell Completions

Shell completion files are available in the repo for various shells. In the future, we may provide a more convenient method of installing them.

## Man Pages

Manpages are available in the repo. In the future, we may provide a more convenient method of installing them.

## Inspirations

- [Denote](https://protesilaos.com/emacs/denote)
- [Zettelkasten](https://zettelkasten.de/introduction/)
- [GNU Core Utils]()
- [Cold-Blooded Software]()

## FAQ

TODO

## Dependencies

We try to keep dependencies relatively minimal. In time, this project will be feature-complete, and enter maintenance mode. A primary concern for dn is to minimise churn and maximise long-term stability. Eventually, all dependencies will be vendored and the program will be considered "finished", outside of necessary bug fixes and/or emergency patches.

## Development

- We follow [Semantic Versioning](https://semver.org/) for version numbering.
- We use [Conventional Commits](https://www.conventionalcommits.org/) for commit messages.
- We are [REUSE](https://reuse.software/) compliant.

TODO: Consider refactoring configuration setup to reflect a more modular approach - refrence [jujutsu](https://github.com/jj-vcs/jj) and [helix](https://github.com/helix-editor)
TODO: Add xtask command to install man pages and completions (as distinct from generating them)