.PHONY: all completions manpages install

all: completions manpages build install

completions:
	cargo xtask completions
	cargo xtask install-completions

manpages:
	cargo xtask manpages
	cargo xtask install-manpages

build:
	cargo build --release

install: build
	cp ./target/release/dn /usr/local/bin/dn
	chmod +x /usr/local/bin/dn
	echo "Installed dn to /usr/local/bin/dn"

uninstall:
	rm /usr/local/bin/dn
