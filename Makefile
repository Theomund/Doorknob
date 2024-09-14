# Doorknob - Artificial intelligence program written in Rust.
# Copyright (C) 2024 Theomund
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

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
