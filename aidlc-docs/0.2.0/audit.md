
# AI-DLC Audit Log

## CONSTRUCTION - Legacy Storage Migration

**Timestamp**: 2026-08-31T00:00:00Z
**User Input**: "ok so the new id is "uk.etheria-software.timeflow" as I own the 'etheria-software.uk' domain
please can you do the update/add in the import/copy code"
**AI Response**: Updated the active storage namespace to `uk.etheria-software/TimeFlow`,
added detection and user-confirmed copying from the legacy `com/timeflow/TimeFlow` namespace,
wired the migration into the first-run dialog, and added a focused nested-copy test.
**Context**: Legacy alpha data is copied into the new namespace without overwriting existing current data.

---

## DEVELOPMENT - Startup Theme Application

**Timestamp**: 2026-08-31T00:00:00Z
**User Input**: "ok so odd thing, on linux the ui loads in light mode even if the setting is set
to dark and only switches to dark when it is clicked again"
**AI Response**: Centralised theme application, applied the loaded backend theme during
application startup, and removed the unused localStorage theme source.
**Context**: The backend setting was loaded successfully but was not applied until the user clicked a theme option.

---

## DEVELOPMENT - Dependency Security Audit Targets

**Timestamp**: 2026-08-31T00:00:00Z
**User Input**: "please can you add options to the make file to alow audit/security
checks to be run on the packages in use. separate ones for rust and js/ts"
**AI Response**: Added separate Makefile targets for Rust and JavaScript/TypeScript
dependency security audits, with combined `audit` and `security` targets plus help descriptions.
**Context**: Rust checks use `cargo audit` against `src-tauri/Cargo.lock`; JavaScript/TypeScript checks use `pnpm audit`
against `pnpm-lock.yaml`. `make help` and the aggregate dry run passed.
The offline Rust audit identified two existing high-severity `quick-xml` advisories;
the JavaScript/TypeScript audit could not contact the npm registry in the restricted environment.

---

## DEVELOPMENT - Tauri Package Version Alignment (rust)

**Timestamp**: 2026-09-01T00:00:00Z
**User Input**: "please can you remediated the following"
**AI Response**: Aligned the Tauri JavaScript packages with the Rust crates by
upgrading `@tauri-apps/api` to `2.11.1`, `@tauri-apps/plugin-dialog` to `2.7.2`,
`tauri` to `2.11.1`, and `tauri-plugin-dialog` to `2.7.2`. The matching shell versions remain `2.3.5`.
**Context**: The originally reported `tauri 2.10.3`/API `2.11.1` and dialog `2.6.0`/plugin `2.7.2`
minor-version mismatches are removed. Dependency versions are pinned in the manifests to prevent future minor-version drift.

---

## DEVELOPMENT - Tauri Package Version Alignment (js)

**Timestamp**: 2026-09-01T00:00:00Z
**User Input**: "please can you remediated the following
[nvh@Phoenix-01 timeflow]$ make dev-arch
$ tauri dev
Running BeforeDevCommand (`pnpm run dev`)
$ vite
Error Found version mismatched Tauri packages. Make sure the NPM package and Rust crate versions are on the same
major/minor releases:
tauri (v2.10.3) : @tauri-apps/api (v2.11.1)
tauri-plugin-dialog (v2.6.0) : @tauri-apps/plugin-dialog (v2.7.2)"
**AI Response**: Aligned the JavaScript Tauri runtime packages with the locked Rust crates and pinned the versions to
prevent minor-version drift: API `2.10.1` with Tauri `2.10.3`, dialog `2.6.0` with `tauri-plugin-dialog 2.6.0`,
and shell `2.3.5` with `tauri-plugin-shell 2.3.5`.
**Context**: The API package has no `2.10.3` release, so published API `2.10.1` is the matching `2.10` minor release.
`pnpm install --frozen-lockfile`, `cargo check --locked --offline`, and `pnpm run build` pass.

---

## DEVELOPMENT - JavaScript/TypeScript Dependency Advisory Remediation

**Timestamp**: 2026-09-01T00:00:00Z
**User Input**: "please can you remediate the attached"
**AI Response**: Upgraded Vite to `6.4.3`, Svelte to `5.57.0`, and `@sveltejs/vite-plugin-svelte` to `6.2.4`,
bringing `esbuild` to `0.25.12` and refreshing `pnpm-lock.yaml`.
**Context**: `pnpm audit --audit-level low` reports no known vulnerabilities, and `pnpm run build` passes.
Existing Svelte accessibility warnings remain outside the scope of this dependency remediation.

---

## DEVELOPMENT - Rust Dependency Advisory Remediation

**Timestamp**: 2026-09-01T00:00:00Z
**User Input**: "please can you remediate the attached"
**AI Response**: Updated the transitive Rust dependency chain from `plist 1.8.0`/`quick-xml 0.38.4` to
`plist 1.10.0`/`quick-xml 0.41.0`, and updated `anyhow` to `1.0.104` and `rand 0.8` to `0.8.7`.
**Context**: `make security-rust CARGO_AUDIT_ARGS=--no-fetch` now exits successfully with no vulnerability errors.
`cargo check --locked --offline` also passes. Nineteen allowed warnings remain
for upstream GTK3/GLib and related transitive crates.

---

## DEVELOPMENT - Date Selector Navigation

**Timestamp**: 2026-09-01T00:00:00Z
**User Input**: "please can you check what causes and fix the bug that makes the
`<- and ->` buttons next to the date interface not move the date forward and backward by one day"
**AI Response**: Changed date navigation to perform arithmetic and serialisation in UTC in `DateSelector.svelte`.
**Context**: Local-midnight parsing followed by `toISOString()` shifted dates
backwards in positive UTC offsets, causing the previous button to skip a day and the next button to appear ineffective.

---

## DEVELOPMENT - Svelte 5 Application Bootstrap

**Timestamp**: 2026-09-01T00:00:00Z
**User Input**: "the UI does not load, attached is the shell output from the terminal
that ran the app also attached is the console output from the apps frontend"
**AI Response**: Replaced the removed Svelte 4 `new App({ target })` bootstrap with
Svelte 5's `mount(App, { target })` API in `src/main.ts`.
**Context**: The frontend console reported `component_api_invalid_new`; the root
component was never mounted, leaving the Tauri window blank.

---

## DEVELOPMENT - Svelte Accessibility Warning Remediation

**Timestamp**: 2026-09-01T11:03:16Z
**User Input**: "please can you resolve the following [Svelte warnings for
unassociated labels, unused TimerWidget currentDate export, and clickable modal overlay div]"
**AI Response**: Replaced non-associated labels with labelled groups, converted the
task form overlay click target into a backdrop button, added an accessible label for
category search, and removed the unused TimerWidget date prop and parent binding.
**Context**: `pnpm run build` completes successfully without the reported Svelte warnings.

---
