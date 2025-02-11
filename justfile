# SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
# SPDX-FileContributor: Matthew Mark Ibbetson
#
# SPDX-License-Identifier: GPL-3.0-or-later

os := os_family()
home_dir := home_directory()
conf_dir := config_directory()
man_dir := "/usr/local/share/man/man1"
bin_dir := executable_directory()
bin_name := "dn"

# List all recipes by default.
_default:
    @just --list

# Build the project.
build: fix test _compile _doc extras
    cargo doc
    cargo build --release

# Enforce styling and linting.
fix:
    cargo fmt
    cargo clippy --fix

# Run tests.
test filter="":
    cargo test {{filter}}

# Build in release mode.
_compile:
    cargo build --release

# Compile documentation.
_doc:
    cargo doc

# Recipe with dependencies.
publish: test build
    cargo publish

# Clean build artifacts.
clean:
    cargo clean

# Install binary via cargo.
install:
    cargo install --path .

# Uninstall binary via cargo.
uninstall:
    cargo uninstall {{bin_name}}

# Install the cargo-xtask dev utility.
_install-xtask:
    cargo install --path xtask

# Generate manpage files.
_gen-manpages:
    cargo xtask manpages

# Generate shell completion files.
_gen-completions:
    cargo xtask completions

# Generate extra files like manpages and shell completions.
extras: _install-xtask _gen-manpages _gen-completions

# Install manpages.
manpages: _gen-manpages
    mkdir -p {{man_dir}}
    cp ./man/* {{man_dir}}

# Uninstall manpages.
uninstall-manpages:
    rm {{man_dir}}/dn.1
    rm {{man_dir}}/dn-rename.1
    rm {{man_dir}}/dn-new.1
