# SPDX-FileCopyrightText: 2025 Matthew Mark Ibbetson
# SPDX-FileContributor: Matthew Mark Ibbetson
#
# SPDX-License-Identifier: GPL-3.0-or-later

.PHONY: all completions manpages install uninstall clean

# Detect OS
UNAME_S := $(shell uname -s)
USER_HOME := $(shell echo $$HOME)

# Default installation directories
PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
MANDIR ?= $(PREFIX)/share/man/man1

# OS-specific paths and commands
ifeq ($(UNAME_S),Darwin)    # macOS
    COMPLETION_DIRS = \
        $(USER_HOME)/.bash_completion.d \
        $(USER_HOME)/.zsh/site-functions \
        $(USER_HOME)/.config/fish/completions \
        $(USER_HOME)/.config/nushell/completions \
        $(USER_HOME)/.elvish/lib
else ifeq ($(OS),Windows_NT)    # Windows
    POWERSHELL_DIR = $(PROGRAMFILES)/WindowsPowerShell/Modules
else    # Linux
    COMPLETION_DIRS = \
        /usr/share/bash-completion/completions \
        /usr/share/zsh/site-functions \
        /usr/share/fish/vendor_completions.d \
        $(USER_HOME)/.config/nushell/completions \
        $(USER_HOME)/.elvish/lib
endif

all: completions manpages build install

# Create necessary directories
$(COMPLETION_DIRS):
	mkdir -p $@

# Build completions
completions: | $(COMPLETION_DIRS)
	@echo "Building completions..."
	cargo install --path ./xtask
	cargo xtask completions
	@echo "Installing completions..."
ifeq ($(OS),Windows_NT)
	@if exist "$(POWERSHELL_DIR)" ( \
		copy /Y ".\completions\_dn.ps1" "$(POWERSHELL_DIR)\_dn.ps1" \
	) else ( \
		echo "PowerShell modules directory not found" \
	)
else
	@if [ -d "$(USER_HOME)/.bash_completion.d" ]; then \
		cp ./completions/dn.bash "$(USER_HOME)/.bash_completion.d/"; \
	fi
	@if [ -d "$(USER_HOME)/.zsh/site-functions" ]; then \
		cp ./completions/_dn "$(USER_HOME)/.zsh/site-functions/"; \
	fi
	@if [ -d "$(USER_HOME)/.config/fish/completions" ]; then \
		cp ./completions/dn.fish "$(USER_HOME)/.config/fish/completions/"; \
	fi
	@if [ -d "$(USER_HOME)/.config/nushell/completions" ]; then \
		cp ./completions/dn.nu "$(USER_HOME)/.config/nushell/completions/"; \
	fi
	@if [ -d "$(USER_HOME)/.elvish/lib" ]; then \
		cp ./completions/dn.elv "$(USER_HOME)/.elvish/lib/"; \
	fi
endif

# Build and install manpages
manpages:
	@echo "Building manpages..."
	cargo install --path ./xtask
	cargo xtask manpages
	mkdir -p $(MANDIR)
	cp ./man/* $(MANDIR)/
	@echo "Installed manpages to $(MANDIR)"

# Build the binary
build:
	@echo "Building dn..."
	cargo build --release

# Install the binary
install: build
	@echo "Installing dn..."
	mkdir -p $(BINDIR)
	cp ./target/release/dn $(BINDIR)/dn
	@echo "Installed dn to $(BINDIR)/dn"

# Uninstall everything
uninstall:
	@echo "Uninstalling dn..."
	rm -f $(BINDIR)/dn
	rm -f $(MANDIR)/dn.1
ifeq ($(OS),Windows_NT)
	@if exist "$(POWERSHELL_DIR)\_dn.ps1" del "$(POWERSHELL_DIR)\_dn.ps1"
else
	rm -f $(USER_HOME)/.bash_completion.d/dn.bash
	rm -f $(USER_HOME)/.zsh/site-functions/_dn
	rm -f $(USER_HOME)/.config/fish/completions/dn.fish
	rm -f $(USER_HOME)/.config/nushell/completions/dn.nu
	rm -f $(USER_HOME)/.elvish/lib/dn.elv
endif

# Clean build artifacts
clean:
	cargo clean
