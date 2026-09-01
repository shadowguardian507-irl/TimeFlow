# Code Summary: TimeFlow 0.11.2

## Overview

Release 0.11.2 modified the existing TimeFlow application in place. The changes
were maintenance-focused and did not introduce new architectural layers.

## Storage and Identifier Fixes

- Updated the Tauri application identifier to `uk.etheria-software.timeflow`.
- Updated storage path resolution to use the current `uk/etheria-software/TimeFlow` namespace.
- Added legacy namespace detection for `com/timeflow/TimeFlow`.
- Added import support that copies legacy data into the current namespace only when current data does not already exist.
- Exposed legacy detection/import commands through the existing Tauri command layer.
- Wired the migration option into the first-run dialog.

Primary areas:

- `src-tauri/tauri.conf.json`
- `src-tauri/src/services/storage.rs`
- `src-tauri/src/services/settings_manager.rs`
- `src-tauri/src/commands/settings.rs`
- `src-tauri/src/main.rs`
- `src/lib/api/commands.ts`
- `src/lib/components/FirstRunDialog.svelte`

## Theme Startup Fixes

- Centralised frontend theme application.
- Applied the loaded backend theme on application startup.
- Removed reliance on stale local storage state for initial theme selection.

Primary areas:

- `src/main.ts`
- `src/lib/theme.ts`
- `src/App.svelte`
- `src/lib/components/ThemeSelector.svelte`

## Build, Audit, and Dependency Fixes

- Added Makefile targets for split Rust and JavaScript/TypeScript dependency audits.
- Added Makefile targets for checking and applying release versions across app manifests.
- Updated Rust dependency chain to remove the known `quick-xml` advisory path.
- Updated JavaScript/Svelte/Vite tooling to remediate known advisories.
- Pinned Tauri JavaScript packages and Rust crates to matching major/minor release lines.
- Kept shared Tauri bundle targets at `all` and added a Linux-specific Tauri
  config that limits Linux packaging to `deb` and `rpm`, so standard Linux
  builds do not fail in AppImage linuxdeploy on Arch while macOS can still
  produce `.dmg` bundles.

Primary areas:

- `Makefile`
- `package.json`
- `pnpm-lock.yaml`
- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/tauri.conf.json`
- `src-tauri/tauri.linux.conf.json`
- `scripts/sync-version.mjs`

Final relevant dependency state:

| Package Area | Final State |
| --- | --- |
| Tauri Rust crate | `tauri` `~2.11.1` |
| Tauri JS API | `@tauri-apps/api` `2.11.1` |
| Dialog Rust plugin | `tauri-plugin-dialog` `~2.7.2` |
| Dialog JS plugin | `@tauri-apps/plugin-dialog` `2.7.2` |
| Shell plugin pair | `2.3.5` release line |
| Vite | `^6.4.3` |
| Svelte | `^5.57.0` |
| Svelte Vite plugin | `^6.2.4` |

## Frontend Behaviour Fixes

- Replaced Svelte 4 `new App({ target })` startup with Svelte 5 `mount`.
- Changed date selector navigation to use UTC date arithmetic.
- Removed the unused `currentDate` prop from `TimerWidget` and its parent binding.
- Replaced non-associated form labels with labelled groups where custom component groups are used.
- Converted the clickable task-form overlay into an accessible backdrop button.
- Added an accessible label to the category picker search input.
- Changed the sidebar version display to use Tauri app metadata, with a build-time fallback read from `src-tauri/tauri.conf.json`.

Primary areas:

- `vite.config.ts`
- `src/app-version.d.ts`
- `src/main.ts`
- `src/lib/components/DateSelector.svelte`
- `src/lib/components/CategoryPicker.svelte`
- `src/lib/components/TaskEntryForm.svelte`
- `src/lib/components/TemplateForm.svelte`
- `src/lib/components/ThemeSelector.svelte`
- `src/lib/components/TimeEntryView.svelte`
- `src/lib/components/TimerWidget.svelte`
- `src/lib/components/Sidebar.svelte`

## Behaviour Preserved

- Existing task, category, template, timer, and export flows remain within the 0.1.0 application model.
- TimeFlow remains a local desktop application with YAML-backed data.
- No cloud synchronisation, remote service integration, or multi-user behaviour was added.
