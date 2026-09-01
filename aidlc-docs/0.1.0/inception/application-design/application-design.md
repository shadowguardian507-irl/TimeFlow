# Application Design - TimeFlow

## Overview

TimeFlow is a macOS desktop application built with Rust and Tauri for time tracking and ActiTime integration. The
architecture follows a backend-centric approach where Rust holds all application state and the frontend communicates
via Tauri commands.

---

## Architecture Principles

### Backend-Centric State Management

- Rust backend holds all application state
- Frontend requests data via Tauri commands
- No duplicate state between frontend and backend
- Single source of truth in backend

### Domain-Separated Data Models

- Models grouped by domain: tasks, categories, templates
- Clear separation of concerns
- Each domain has its own manager component

### YAML File Storage

- Human-readable YAML format for all data
- Nested tree structure for category hierarchy
- Atomic writes for data integrity
- Crash recovery support

---

## Components

| Component | Purpose |
| ----------- | --------- |
| TaskManager | Manages time entries (create, update, delete, timer) |
| CategoryManager | Manages hierarchical category structure |
| TemplateManager | Manages task templates for quick entry |
| ViewGenerator | Generates full and ActiTime-tuned views |
| ExportManager | Handles backup and CSV export |
| SettingsManager | Manages user preferences and theme |
| StorageService | YAML file persistence |

---

## Services

| Service | Purpose |
| --------- | --------- |
| TimeMergeService | Calculates time distribution for mergeable tasks |
| TimerService | Backend timer state management |
| DataIntegrityService | Ensures data consistency and atomic writes |

---

## Data Models

### Task

```yaml
id: uuid
name: string
date: date
duration_minutes: u32
category_path: string
task_type: direct | mergeable
distribution_strategy: proportional | even | manual | weighted (optional)
created_at: datetime
```

### Category (Nested Tree)

```yaml
name: string
path: string
hidden: bool
children:
  - name: string
    path: string
    hidden: bool
    children: [...]
```

### Template

```yaml
id: uuid
name: string
default_duration: u32
category_path: string
is_mergeable: bool
distribution_strategy: string (optional)
```

### Settings

```yaml
theme: light | dark | system
first_run_complete: bool
```

---

## Communication Pattern

```text
+-------------+     Tauri      +-------------+     Direct     +-------------+
|             |    Commands    |             |     Calls      |             |
|  Frontend   +--------------->+  Managers   +--------------->+  Services   |
| (TypeScript)|                |   (Rust)    |                |   (Rust)    |
|             |<---------------+             |<---------------+             |
+-------------+    Results     +-------------+    Results     +-------------+
                                     |
                                     | Direct Calls
                                     v
                              +-------------+
                              |   Storage   |
                              |   Service   |
                              +-------------+
                                     |
                                     v
                              +-------------+
                              | YAML Files  |
                              +-------------+
```

---

## File Structure

```text
~/.timeflow/
  data/
    tasks/
      2026-03-05.yaml
      2026-03-06.yaml
    categories.yaml
    templates.yaml
  settings.yaml
  timer_state.yaml (temp, for crash recovery)
```

---

## Key Design Decisions

| Decision | Choice | Rationale |
| ---------- | -------- | ----------- |
| State Management | Backend-centric | Single source of truth, simpler sync |
| Data Organization | Domain-separated | Clear boundaries, easier maintenance |
| Category Storage | Nested tree YAML | Matches ActiTime hierarchy, human-readable |
| Timer | Backend with polling | Accurate timing, crash recovery |
| Persistence | YAML files | Human-readable, easy backup, no DB overhead |

---

## Distribution Strategies

The TimeMergeService supports four distribution strategies for mergeable tasks:

1. **Proportional**: Time distributed based on direct task durations
2. **Even**: Time split equally across all direct tasks
3. **Manual**: User specifies which tasks receive time and how much
4. **Weighted**: User assigns weights to direct tasks

Default strategy can be set per task type (template), with per-entry override capability.
