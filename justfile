# SPDX-FileCopyrightText: 2024-2025 Matthew Mark Ibbetson
# SPDX-FileContributor: Matthew Mark Ibbetson
#
# SPDX-License-Identifier: GPL-3.0-or-later

# List all recipes by default
default:
    @just --list

install-xtask:
    cargo install --path xtask

xtasks:
    cargo xtask completions
    cargo xtask manpages

# Build the project
build:
    cargo build

# Run tests with optional filter
test filter="":
    cargo test {{filter}}

# Clean build artifacts
clean:
    cargo clean

# Install project dependencies
install:
    npm install
    cargo install --path .

# Run multiple commands in sequence
setup: install build

# Recipe with arguments
say-hello name:
    echo "Hello, {{name}}!"

# Recipe with environment variable
deploy env="staging":
    echo "Deploying to {{env}}"
    
# Recipe with dependencies
publish: build test
    cargo publish

# Recipe with conditional
lint:
    #!/usr/bin/env sh
    if [ -f ".eslintrc" ]; then
        npm run lint
    else
        echo "No ESLint config found"
    fi
