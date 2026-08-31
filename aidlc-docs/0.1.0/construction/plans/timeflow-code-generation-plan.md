# Code Generation Plan - TimeFlow

## Overview

This plan outlines the code generation steps for the TimeFlow time management application.

## Project Context

- **Project Type**: Greenfield (new application)
- **Technology Stack**: Rust + Tauri
- **Target Platform**: macOS desktop
- **Code Location**: Workspace root

## Project Structure

```text
timeflow/
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs              # Tauri entry point
│   │   ├── lib.rs               # Library exports
│   │   ├── commands/            # Tauri commands (API layer)
│   │   │   ├── mod.rs
│   │   │   ├── tasks.rs
│   │   │   ├── categories.rs
│   │   │   ├── templates.rs
│   │   │   ├── views.rs
│   │   │   ├── settings.rs
│   │   │   └── export.rs
│   │   ├── models/              # Domain entities
│   │   │   ├── mod.rs
│   │   │   ├── task.rs
│   │   │   ├── category.rs
│   │   │   ├── template.rs
│   │   │   ├── timer.rs
│   │   │   └── settings.rs
│   │   ├── services/            # Business logic
│   │   │   ├── mod.rs
│   │   │   ├── task_manager.rs
│   │   │   ├── category_manager.rs
│   │   │   ├── template_manager.rs
│   │   │   ├── view_generator.rs
│   │   │   ├── export_manager.rs
│   │   │   ├── settings_manager.rs
│   │   │   ├── time_merge.rs
│   │   │   ├── timer_service.rs
│   │   │   └── storage.rs
│   │   └── error.rs             # Error types
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                          # Frontend (TypeScript/HTML/CSS)
│   ├── main.ts                  # Entry point
│   ├── App.svelte               # Root component (using Svelte)
│   ├── lib/
│   │   ├── components/          # UI components
│   │   ├── stores/              # State management
│   │   └── api/                 # Backend command wrappers
│   └── styles/
├── package.json
├── tsconfig.json
├── vite.config.ts
└── README.md
```

## Story Coverage

This code generation implements all 29 user stories:

- Time Entry (TE-01 to TE-09): 9 stories
- Category Management (CM-01 to CM-03): 3 stories
- Common Tasks (CT-01 to CT-04): 4 stories
- Time Views (TV-01 to TV-04): 4 stories
- Data Storage (DS-01 to DS-04): 4 stories
- Jira Integration (JI-01 to JI-02): 2 stories (Phase 2 - stubs only)
- UI/Theme (UI-01 to UI-03): 3 stories

---

## Code Generation Steps

### Phase 1: Project Setup

- [x] Step 1: Initialize Tauri project structure
  - Create Cargo.toml with dependencies
  - Create tauri.conf.json configuration
  - Create package.json for frontend
  - Create vite.config.ts
  - Create tsconfig.json

- [x] Step 2: Create error handling module
  - Create src-tauri/src/error.rs with custom error types

### Phase 2: Domain Models

- [x] Step 3: Create task models
  - Create src-tauri/src/models/task.rs (Task, TaskType, DistributionStrategy)
  - Implements: TE-03, TE-04

- [x] Step 4: Create category models
  - Create src-tauri/src/models/category.rs (Category tree structure)
  - Implements: CM-01

- [x] Step 5: Create template models
  - Create src-tauri/src/models/template.rs (Template entity)
  - Implements: CT-01

- [x] Step 6: Create timer and settings models
  - Create src-tauri/src/models/timer.rs (TimerState)
  - Create src-tauri/src/models/settings.rs (Settings, Theme)
  - Implements: TE-02, UI-01

- [x] Step 7: Create models module exports
  - Create src-tauri/src/models/mod.rs

### Phase 3: Storage Service

- [x] Step 8: Create storage service
  - Create src-tauri/src/services/storage.rs
  - YAML file read/write operations
  - Atomic writes for data integrity
  - Implements: DS-01

### Phase 4: Business Logic Services

- [x] Step 9: Create time merge service
  - Create src-tauri/src/services/time_merge.rs
  - Proportional, even, manual, weighted distribution algorithms
  - Implements: TE-05, TE-06, TE-07, TE-08

- [x] Step 10: Create timer service
  - Create src-tauri/src/services/timer_service.rs
  - Timer state management, crash recovery
  - Implements: TE-02

- [x] Step 11: Create task manager
  - Create src-tauri/src/services/task_manager.rs
  - Task CRUD operations, timer integration
  - Implements: TE-01, TE-02, TE-09

- [x] Step 12: Create category manager
  - Create src-tauri/src/services/category_manager.rs
  - Category tree operations, add/hide
  - Implements: CM-01, CM-02, CM-03

- [x] Step 13: Create template manager
  - Create src-tauri/src/services/template_manager.rs
  - Template CRUD, apply template
  - Implements: CT-01, CT-02, CT-03, CT-04

- [x] Step 14: Create view generator
  - Create src-tauri/src/services/view_generator.rs
  - Full view, ActiTime view, week view
  - Implements: TV-01, TV-02, TV-03, TV-04

- [x] Step 15: Create export manager
  - Create src-tauri/src/services/export_manager.rs
  - Backup export/import, CSV export
  - Implements: DS-02, DS-03, DS-04

- [x] Step 16: Create settings manager
  - Create src-tauri/src/services/settings_manager.rs
  - Settings CRUD, first run detection
  - Implements: UI-01, UI-02, UI-03

- [x] Step 17: Create services module exports
  - Create src-tauri/src/services/mod.rs

### Phase 5: Tauri Commands (API Layer)

- [x] Step 18: Create task commands
  - Create src-tauri/src/commands/tasks.rs
  - Tauri command handlers for task operations

- [x] Step 19: Create category commands
  - Create src-tauri/src/commands/categories.rs
  - Tauri command handlers for category operations

- [x] Step 20: Create template commands
  - Create src-tauri/src/commands/templates.rs
  - Tauri command handlers for template operations

- [x] Step 21: Create view commands
  - Create src-tauri/src/commands/views.rs
  - Tauri command handlers for view generation

- [x] Step 22: Create settings commands
  - Create src-tauri/src/commands/settings.rs
  - Tauri command handlers for settings operations

- [x] Step 23: Create export commands
  - Create src-tauri/src/commands/export.rs
  - Tauri command handlers for export/import

- [x] Step 24: Create commands module exports and main entry
  - Create src-tauri/src/commands/mod.rs
  - Create src-tauri/src/main.rs with Tauri setup
  - Create src-tauri/src/lib.rs

### Phase 6: Frontend Foundation

- [x] Step 25: Create frontend entry and API wrappers
  - Create src/main.ts
  - Create src/lib/api/commands.ts (TypeScript wrappers for Tauri commands)
  - Create src/lib/api/types.ts (TypeScript type definitions)

- [x] Step 26: Create state management stores
  - Create src/lib/stores/tasks.ts
  - Create src/lib/stores/categories.ts
  - Create src/lib/stores/templates.ts
  - Create src/lib/stores/settings.ts
  - Create src/lib/stores/timer.ts

### Phase 7: Frontend Components

- [x] Step 27: Create layout components
  - Create src/App.svelte
  - Create src/lib/components/Layout.svelte
  - Create src/lib/components/Sidebar.svelte
  - Create src/lib/components/Header.svelte

- [x] Step 28: Create time entry components
  - Create src/lib/components/TimerWidget.svelte
  - Create src/lib/components/TaskList.svelte
  - Create src/lib/components/TaskItem.svelte
  - Create src/lib/components/TaskEntryForm.svelte
  - Implements: TE-01, TE-02

- [x] Step 29: Create category components
  - Create src/lib/components/CategoryPicker.svelte
  - Create src/lib/components/CategoryTree.svelte
  - Create src/lib/components/CategoryForm.svelte
  - Implements: CM-01, CM-02, CM-03

- [x] Step 30: Create template components
  - Create src/lib/components/TemplateList.svelte
  - Create src/lib/components/TemplateForm.svelte
  - Implements: CT-01, CT-02, CT-03, CT-04

- [x] Step 31: Create view components
  - Create src/lib/components/TimeEntryView.svelte
  - Create src/lib/components/ActiTimeView.svelte
  - Create src/lib/components/WeekView.svelte
  - Create src/lib/components/DateSelector.svelte
  - Create src/lib/components/DailySummary.svelte
  - Create src/lib/components/CategoriesView.svelte
  - Create src/lib/components/TemplatesView.svelte
  - Implements: TV-01, TV-02, TV-03, TV-04

- [x] Step 32: Create settings and dialog components
  - Create src/lib/components/SettingsView.svelte
  - Create src/lib/components/FirstRunDialog.svelte
  - Create src/lib/components/CloseConfirmDialog.svelte
  - Create src/lib/components/ThemeSelector.svelte
  - Implements: UI-01, UI-02, UI-03

### Phase 8: Styling

- [x] Step 33: Create styles
  - Create src/styles/global.css
  - Create src/styles/variables.css (theme variables)

### Phase 9: Documentation

- [x] Step 34: Create README and documentation
  - Create README.md with setup instructions
  - Create aidlc-docs/construction/timeflow/code/code-summary.md

---

## Dependencies

- Rust 1.70+
- Node.js 18+
- Tauri CLI 2.x
- Svelte 4.x
- Vite 5.x

## Notes

- Jira Integration (JI-01, JI-02) will be stubbed for Phase 2
- All interactive elements will include data-testid attributes for testing
- Frontend uses Svelte for reactive UI (lightweight, good Tauri integration)
