.DEFAULT_GOAL := help

CSPELL ?= cspell
SPELL_SOURCES ?= README.md AGENTS.md CLAUDE.md .ai-rules aidlc-docs src src-tauri package.json svelte.config.js vite.config.ts tsconfig.json tsconfig.node.json
CARGO_AUDIT ?= cargo audit
RUST_LOCKFILE ?= src-tauri/Cargo.lock
VERSION_SYNC ?= node scripts/sync-version.mjs
PNPM_AUDIT_LEVEL ?= low
PNPM_AUDIT_ARGS ?=
CARGO_AUDIT_ARGS ?=

.PHONY: help check-spelling spell-check check-cspell sort-cspell-dictionaries sort-dictionaries version-check version-set audit audit-rust audit-js security security-rust security-js check-cargo-audit clean dev-arch dev-mac

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

version-check: ## Verify app versions match across package.json, Cargo.toml, Tauri config, and Cargo.lock.
	$(VERSION_SYNC) check

version-set: ## Set app versions across manifests. Usage: make version-set VERSION=1.2.3
	@test -n "$(VERSION)" || { echo "Usage: make version-set VERSION=1.2.3"; exit 2; }
	$(VERSION_SYNC) set "$(VERSION)"

audit: audit-rust audit-js ## Run dependency security audits for Rust and JavaScript/TypeScript packages.

audit-rust: check-cargo-audit ## Audit Rust dependencies for known security advisories.
	$(CARGO_AUDIT) --file $(RUST_LOCKFILE) $(CARGO_AUDIT_ARGS)

audit-js: ## Audit JavaScript/TypeScript dependencies for known security advisories.
	pnpm audit --audit-level $(PNPM_AUDIT_LEVEL) $(PNPM_AUDIT_ARGS)

security: audit ## Alias for the complete dependency security audit.

security-rust: audit-rust ## Alias for the Rust dependency security audit.

security-js: audit-js ## Alias for the JavaScript/TypeScript dependency security audit.

check-cargo-audit:
	@$(CARGO_AUDIT) --version >/dev/null 2>&1 || { \
		echo "cargo-audit is not available."; \
		echo "Install it with: cargo install cargo-audit"; \
		exit 127; \
	}

clean: ## Remove installed frontend dependencies, Rust build output, and Vite output.
	@rm -rf node_modules
	@rm -rf src-tauri/target
	@rm -rf dist

dev-arch: ## Start the Tauri desktop app in development mode on Arch Linux.
	@if [ ! -d node_modules ]; then pnpm install; fi
	@exec env __NV_DISABLE_EXPLICIT_SYNC=1 pnpm run tauri dev

dev-mac: ## Start the Tauri desktop app in development mode on macOS.
	@if [ ! -d node_modules ]; then pnpm install; fi
	@exec env pnpm run tauri dev
