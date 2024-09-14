SHELL := /bin/sh

BINARY := target/debug/doorknob
STYLE := .github/styles/RedHat

$(BINARY):
	cargo build

$(STYLE):
	vale sync

.PHONY: all
all: $(BINARY)

.PHONY: clean
clean:
	cargo clean

.PHONY: debug
debug:
	rust-gdb -ex "file $(BINARY)"

.PHONY: distclean
distclean:
	rm -rf $(STYLE)

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint: $(STYLE)
	cargo clippy
	vale README.md

.PHONY: run
run:
	cargo run

.PHONY: test
test:
	cargo test
