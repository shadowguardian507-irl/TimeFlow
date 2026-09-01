# Execution Plan: TimeFlow 0.11.0

## Workflow Mode

This release used a lightweight AI-DLC workflow because it covered targeted
maintenance fixes rather than new user-facing product capabilities.

## Executed Stages

| Stage                           | Depth   | Outcome                                                                                                           |
| ------------------------------- | ------- | ----------------------------------------------------------------------------------------------------------------- |
| Requirements Analysis           | Minimal | Captured maintenance release goals and acceptance criteria.                                                       |
| Workflow Planning               | Minimal | Grouped fixes by risk area: data migration, startup, dependencies, date navigation, bootstrap, and accessibility. |
| Code Generation and Remediation | Focused | Modified existing files in place. No duplicate replacement files were created.                                    |
| Build and Test                  | Focused | Ran targeted build, audit, and check commands recorded in the release audit.                                      |

## Skipped Stages

| Stage                 | Reason                                                                                               |
| --------------------- | ---------------------------------------------------------------------------------------------------- |
| Reverse Engineering   | Existing 0.1.0 documentation and current code structure were sufficient for narrow maintenance work. |
| User Stories          | Fixes were defect and maintenance items with clear expected behaviour.                               |
| Application Design    | No new components, services, or architecture boundaries were introduced.                             |
| Units Generation      | Work stayed within the existing TimeFlow unit.                                                       |
| NFR Design            | Existing local desktop NFR posture was unchanged.                                                    |
| Infrastructure Design | No infrastructure was introduced or changed.                                                         |

## Release Work Items

| Work Item                       | Summary                                                                                           | Verification                                                                              |
| ------------------------------- | ------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| Storage namespace migration     | Move active app identity to `uk.etheria-software.timeflow` and support copying legacy alpha data. | Nested-copy test and existing Rust checks.                                                |
| Startup theme application       | Apply persisted theme during startup using shared theme logic.                                    | Frontend build and manual startup expectation.                                            |
| Dependency audit targets        | Add split and aggregate audit/security Makefile targets.                                          | `make help` and audit command execution recorded in audit log.                            |
| Rust advisory remediation       | Update advisory-affected transitive Rust dependency chain.                                        | `make security-rust CARGO_AUDIT_ARGS=--no-fetch` and `cargo check --locked --offline`.    |
| JavaScript advisory remediation | Update Vite/Svelte tooling stack and lockfile.                                                    | `pnpm audit --audit-level low` and `pnpm run build`.                                      |
| Tauri version alignment         | Align JavaScript Tauri packages and Rust crates on matching major/minor versions.                 | `pnpm install --frozen-lockfile`, `cargo check --locked --offline`, and `pnpm run build`. |
| Date selector navigation        | Use UTC arithmetic for previous/next day changes.                                                 | Behavioural validation and frontend build.                                                |
| Svelte 5 bootstrap              | Replace removed Svelte 4 bootstrap API with Svelte 5 `mount`.                                     | App root mounts and frontend build passes.                                                |
| Svelte warnings                 | Resolve reported accessibility and unused export warnings.                                        | `pnpm run build` passes without the reported warnings.                                    |
| Sidebar version display         | Use Tauri runtime metadata with a build-time app metadata fallback.                               | `pnpm run build` passes and no sidebar version literal remains.                           |
| Version sync tooling            | Add Makefile targets to check and apply calculated release versions across app manifests.         | `make version-set VERSION=0.10.3` and `make version-check` pass.                          |
| Linux packaging defaults        | Exclude AppImage from the default Tauri bundle targets on Arch due to linuxdeploy strip failures. | `pnpm tauri build` passes and emits `.deb` and `.rpm` packages for `0.11.0`.              |

## Risk Assessment

- **Overall Risk**: Low to medium.
- **Highest Risk Area**: Legacy data migration, because it touches user data locations.
- **Mitigation**: Import only when legacy data exists and current data does not already exist;
  keep copy behaviour covered by a focused nested-data test.
- **Residual Risk**: Manual verification is still useful for first-run migration and theme startup on each supported OS.
