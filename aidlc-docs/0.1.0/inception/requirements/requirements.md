# Requirements Document: TimeFlow (Time Management App)

## Intent Analysis Summary

- **User Request**: Desktop application for time tracking that handles both direct ActiTime tasks and "mergeable" overhead tasks, with category management and export capabilities
- **Request Type**: New Project (Greenfield)
- **Scope Estimate**: Single Application (Desktop)
- **Complexity Estimate**: Moderate - multiple features with business logic for time merging

---

## Functional Requirements

### FR-01: Time Entry Management

#### FR-01.1: Dual Entry Modes

- Users can enter time via manual entry (task name + duration)
- Users can enter time via start/stop timer
- Both modes available simultaneously

#### FR-01.2: Task Classification

- Each task is classified as either:
  - **Direct Task**: Maps directly to an ActiTime entry
  - **Mergeable Task**: Time gets distributed to direct tasks

#### FR-01.3: Mergeable Task Distribution

- Mergeable tasks support configurable distribution strategies:
  - **Proportional**: Distribute based on duration of each direct task
  - **Even**: Split equally across all direct tasks
  - **Manual**: User specifies which direct tasks receive the time
  - **Weighted**: User assigns weights to direct tasks
- Default distribution strategy can be set per task type
- User can override default strategy on individual entries

### FR-02: Category Management

#### FR-02.1: Hierarchical Categories

- Categories follow ActiTime's hierarchical structure (e.g., "Overhead/People Management/People Care")
- Hierarchical picker UI for selecting categories

#### FR-02.2: Category Lifecycle

- Users can add new categories
- Users can hide/archive categories (not deleted, just hidden from new entries)
- Hidden categories still visible on historical entries

### FR-03: Common Tasks (Templates)

#### FR-03.1: Template Storage

- Common tasks store:
  - Task name
  - Default duration
  - Default category
  - Whether task is mergeable (and default distribution strategy)

#### FR-03.2: Template Usage

- Quick-add from template list
- All template values can be overridden on individual entries

### FR-04: Time Views

#### FR-04.1: Full View

- Shows all tasks entered (both direct and mergeable)
- Displays actual time spent on each task

#### FR-04.2: ActiTime-Tuned View

- Table format matching ActiTime's input format
- Shows only direct tasks with merged time included
- Optimized for copy/paste to ActiTime

#### FR-04.3: Multi-Day Navigation

- Calendar view for navigating between days
- Week view showing all days at once
- Both navigation methods available

### FR-05: Data Storage & Export

#### FR-05.1: Local Storage

- All data stored locally as YAML files
- Human-readable format for manual inspection/editing

#### FR-05.2: Export/Import

- Export data for backup purposes
- Import data from backup
- Export to CSV/Excel for external analysis

### FR-06: Jira Integration (Bonus Feature - Phase 2)

#### FR-06.1: Read-Only Integration

- Pull list of tickets user is working on from Jira
- Select tickets as task names/categories
- No write-back to Jira in initial version

#### FR-06.2: Future Consideration

- Architecture should allow for future time logging to Jira tickets

---

## Non-Functional Requirements

### NFR-01: Platform

- macOS desktop application
- Built with Rust backend + Tauri framework
- Native-feeling UI

### NFR-02: User Interface

#### NFR-02.1: Theming

- Follows system theme by default (light/dark)
- User can override to force light, dark, or system-follow
- First-run prompt asks user's theme preference

#### NFR-02.2: Interaction

- Mouse/trackpad primary interaction method
- Keyboard shortcuts not required for initial version
- Future shortcuts should have global disable option

### NFR-03: Data Integrity

- Local YAML storage must be resilient to app crashes
- No data loss on unexpected shutdown

### NFR-04: Performance

- App should launch quickly (< 2 seconds)
- UI should remain responsive during data operations

---

## Out of Scope (Initial Version)

- Cloud synchronization
- Multi-user support
- Jira write-back (time logging to tickets)
- Keyboard shortcuts
- Detailed reporting/analytics (beyond CSV export)
- Mobile companion app

---

## Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Backend Language | Rust | User requirement |
| UI Framework | Tauri | User requirement, native feel |
| Data Storage | YAML files | User preference, human-readable |
| Database | None (file-based) | Simplicity, portability |

---

## Extension Configuration

| Extension | Enabled | Reason |
|-----------|---------|--------|
| security-baseline | No | Personal productivity tool, local-only, no network exposure |
