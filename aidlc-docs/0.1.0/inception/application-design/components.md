# Components

## Overview

TimeFlow is organized into domain-separated components with a backend-centric architecture. The Rust backend holds all application state, and the frontend communicates via Tauri commands.

---

## Component: TaskManager

### Purpose

Manages time entries (tasks) including creation, modification, and retrieval.

### Responsibilities

- Create new time entries (manual and timer-based)
- Update existing time entries
- Delete time entries
- Retrieve entries by date/date range
- Classify tasks as direct or mergeable
- Manage timer state (start, stop, pause)

### Interfaces

- Exposes Tauri commands for frontend interaction
- Depends on StorageService for persistence
- Depends on CategoryManager for category validation

---

## Component: CategoryManager

### Purpose

Manages the hierarchical category structure matching ActiTime categories.

### Responsibilities

- Maintain category tree structure
- Add new categories to hierarchy
- Hide/archive categories
- Validate category paths
- Provide category picker data to frontend

### Interfaces

- Exposes Tauri commands for category operations
- Depends on StorageService for persistence

---

## Component: TemplateManager

### Purpose

Manages task templates for quick entry of recurring tasks.

### Responsibilities

- Create task templates with default values
- Update existing templates
- Delete templates
- Retrieve template list
- Apply template to create new task entry

### Interfaces

- Exposes Tauri commands for template operations
- Depends on StorageService for persistence
- Depends on CategoryManager for category validation

---

## Component: ViewGenerator

### Purpose

Generates different views of time data (full view, ActiTime-tuned view).

### Responsibilities

- Generate full view showing all tasks
- Generate ActiTime-tuned view with merged time
- Calculate time distribution for mergeable tasks
- Format output for ActiTime compatibility
- Generate week view summaries

### Interfaces

- Exposes Tauri commands for view generation
- Depends on TaskManager for task data
- Depends on TimeMergeService for distribution calculations

---

## Component: ExportManager

### Purpose

Handles data export and import operations.

### Responsibilities

- Export data to backup format
- Import data from backup
- Export time data to CSV format
- Validate import data integrity

### Interfaces

- Exposes Tauri commands for export/import
- Depends on StorageService for data access
- Depends on TaskManager, CategoryManager, TemplateManager for data

---

## Component: SettingsManager

### Purpose

Manages application settings including theme preferences.

### Responsibilities

- Store and retrieve user preferences
- Manage theme settings (light/dark/system)
- Handle first-run setup
- Persist settings to storage

### Interfaces

- Exposes Tauri commands for settings operations
- Depends on StorageService for persistence

---

## Component: StorageService

### Purpose

Handles all YAML file persistence operations.

### Responsibilities

- Read/write task data to YAML files
- Read/write category data to YAML files
- Read/write template data to YAML files
- Read/write settings to YAML files
- Ensure data integrity on save
- Handle file system operations

### Interfaces

- Internal service used by all managers
- No direct frontend exposure
