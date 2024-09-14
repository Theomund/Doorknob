SHELL := /bin/sh

STYLE := .github/styles/RedHat

$(STYLE):
	vale sync

.PHONY: lint
lint: $(STYLE)
	vale README.md
