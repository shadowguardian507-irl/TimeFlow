# Requirements Document: TimeFlow 0.11.0 Maintenance Fixes

## Intent Analysis Summary

- **User Request**: Document the fixes made in release 0.11.0 using lightweight AI-DLC documentation.
- **Request Type**: Brownfield maintenance documentation.
- **Scope Estimate**: Single desktop application, existing frontend and Tauri backend.
- **Complexity Estimate**: Simple to moderate.
  The release contains several narrow fixes across dependencies, startup, storage, date navigation, build tooling, and accessibility.

## Release Goals

- Preserve existing TimeFlow behaviour while removing known startup, build, and accessibility defects.
- Improve maintainability by pinning dependency versions that previously drifted into incompatible minor releases.
- Improve developer confidence with explicit security audit targets for Rust and JavaScript/TypeScript dependencies.
- Preserve local user data during the application identifier migration.

## Functional Requirements

### FR-01: Legacy Data Migration

- The active application data namespace must use the `uk.etheria-software.timeflow` application identifier.
- Existing alpha data in the legacy `com/timeflow/TimeFlow` namespace must be detected.
- Users must be offered a first-run path to import legacy data.
- Legacy import must copy nested user data without overwriting existing current data.

### FR-02: Startup Theme Application

- The UI must apply the stored backend theme setting during application startup.
- The app must no longer require the user to click the theme selector before dark mode takes effect.
- Theme application logic must remain centralised so startup and manual selection use the same behaviour.

### FR-03: Date Navigation

- The previous-day and next-day buttons beside the date selector must move exactly one calendar day.
- Date arithmetic must avoid local timezone shifts that cause skipped dates or apparently ineffective navigation.

### FR-04: Svelte 5 Bootstrap

- The frontend must mount correctly with Svelte 5.
- The removed Svelte 4 `new App({ target })` component API must not be used.

### FR-05: Accessibility and Compiler Warning Remediation

- Form labels must be associated with controls, or replaced by labelled groups when labelling a component group.
- Clickable static elements must not be used for modal backdrop interaction.
- `TimerWidget` must not export unused component props.
- The sidebar must display the actual app version from application metadata rather than a hard-coded fallback string.

### FR-06: Dependency and Audit Maintenance

- Rust dependency advisories must be remediated where compatible updates are available.
- JavaScript/TypeScript dependency advisories must be remediated where compatible updates are available.
- Tauri JavaScript packages and Rust crates must stay on matching major/minor releases.
- Makefile targets must allow Rust and JavaScript/TypeScript audits to run separately and together.
- Makefile targets must verify and apply calculated release versions across app manifests.

## Non-Functional Requirements

- Build output should be free of the Svelte warnings addressed in this release.
- Default Linux release packaging should avoid known AppImage tooling failures on rolling-release Arch hosts.
- Dependency versions should be pinned tightly enough to prevent avoidable Tauri minor-version drift.
- Security audit commands should be discoverable through `make help`.
- Changes should remain local to the existing app architecture.
- No new cloud, network, account, or multi-user behaviour should be introduced.

## Acceptance Criteria

- `pnpm run build` completes successfully.
- Rust dependency checks complete successfully in the configured offline/audit flow.
- JavaScript/TypeScript audit reports no known vulnerabilities at the configured audit level.
- Date navigation works consistently in positive and negative UTC offsets.
- Existing alpha data can be copied into the current app data namespace when no current data exists.
- Svelte accessibility/compiler warnings reported during the 0.11.0 work are resolved.
- Sidebar version display follows the app version configured in Tauri metadata.
- `make version-check` confirms `package.json`, `src-tauri/Cargo.toml`,
  `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.lock` are in sync.
- `pnpm tauri build` completes successfully and emits versioned `.deb` and `.rpm` packages.
