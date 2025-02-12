<!--
SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
SPDX-FileContributor: Matthew Mark Ibbetson

SPDX-License-Identifier: GPL-3.0-or-later
-->

# dn

dn is a CLI tool for creating and renaming plaintext notes with a predictable, timestamped naming format. It is inspired by the amazing Emacs package [Denote](https://protesilaos.com/emacs/denote) created by [Prot](https://protesilaos.com/). dn creates files that can be easily queried using standard tools like `find`, `sed`, `grep`, `awk`, `ripgrep`, `fzf`, etc.

dn aims to reproduce the file naming functionality of Denote, while being entirely editor agnostic and scriptable in any language. It deviates in several ways from the behaviour of Denote, both due to the reduced scope (much of Denote's functionality is best suited for editor integrations) and different concerns (Emacs & Org already have a complete, quality solution to practically all of the goals dn addresses, i.e. Denote).

## Installation

### Binary Download

The latest binaries are available [here]().

### Cargo Install

#### Crates.io

If you have a Rust environment set up, you can install the binary from [crates.io](https://crates.io/crates/dn-cli) with the following command:

```sh
cargo install dn-cli
```

### Build From Source

```sh
# clone repo and make install
git clone https://github.com/mmibbetson/dn
cd dn
just install
```

### Nix

#### Download From Nixpkgs

```sh

```

#### Build Derivation

```sh

```

## Quick Start

```bash
# Create a new note
dn new -k personal_ideas -t "My First Note"

# Rename an existing note
dn rename oldfile.txt -k converted -e md # produces something like 20241006T145030--oldfile__converted.md

# Remove the title and change extension of an existing note
dn rename 20241001T121314--oldtitle__foo.md -e dj # produces something like 20241001T121314--oldtitle__foo.dj

# Add and remove keywords on an existing file
dn rename 20241001T121314--oldtitle__foo.dj -A bar_baz -R baz # produces 20241001T121314--oldtitle__foo_bar.dj

# Search $DN_DIRECTORY for a note with fzf and then open it in helix
rg $DN_DIRECTORY --file | fzf | xargs hx
```

## Extras

Manpages and shell completions are available, they can be installed manually. The supported shells are:

- bash
- zsh
- fish
- powershell
- nushell
- elvish

## Editor Support

dn is designed with the intention that it will be integrated into text editors via extensions. When Helix's plugin system is implemented, the intention is to provide an ergonomic set of extensions as specified in the [integration docs](./docs/dev/integrations.md). A VSCode extension is also being considered.

- [ ] Helix
- [ ] Visual Studio Code

## Inspirations

- [Denote](https://protesilaos.com/emacs/denote)
- [Zettelkasten](https://zettelkasten.de/introduction/)
- [The Unix Philosophy](https://en.wikipedia.org/wiki/Unix_philosophy)
- [Cold-Blooded Software](https://dubroy.com/blog/cold-blooded-software/)

## Dependencies

Dependencies are relatively minimal. In time, this project will be feature-complete, and enter maintenance mode. A primary concern for dn is to minimise churn and maximise long-term stability. Eventually, all dependencies will be vendored and the program will be considered "finished", outside of necessary bug fixes and/or emergency patches.

## Development

- dn follows [Semantic Versioning](https://semver.org/) for version numbering.
- dn uses [Conventional Commits](https://www.conventionalcommits.org/) for commit messages.
- dn is [REUSE](https://reuse.software/) compliant.
