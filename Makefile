.DEFAULT_GOAL := help

CSPELL ?= cspell
SPELL_SOURCES ?= README.md AGENTS.md CLAUDE.md .ai-rules aidlc-docs src src-tauri package.json svelte.config.js vite.config.ts tsconfig.json tsconfig.node.json

.PHONY: help check-spelling spell-check check-cspell sort-cspell-dictionaries sort-dictionaries

help: ## Show this help list.
	@awk 'BEGIN {FS = ":.*## "; printf "Available targets:\n"} /^[a-zA-Z0-9_-]+:.*## / {printf "  %-28s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

check-spelling: check-cspell ## Check repository text for spelling errors with cspell.
	$(CSPELL) --config .cspell.yaml $(SPELL_SOURCES)

spell-check: check-spelling ## Alias for check-spelling.

check-cspell:
	@command -v $(CSPELL) >/dev/null 2>&1 || { \
		echo "cspell is not installed."; \
		echo "Install it with: pnpm add -g cspell@latest"; \
		exit 127; \
	}

sort-cspell-dictionaries: ## Alphabetically sort dictionary files under .cspell/.
	@find .cspell -type f -name '*.txt' -print0 | xargs -0 -r -n 1 sh -c 'LC_ALL=C sort -f -u "$$1" -o "$$1"' sh
	@echo "Sorted .cspell dictionary files."

sort-dictionaries: sort-cspell-dictionaries ## Alias for sort-cspell-dictionaries.
