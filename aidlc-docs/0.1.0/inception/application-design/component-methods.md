# Component Methods

## Overview

Method signatures for each component. Detailed business rules will be defined in Functional Design.

---

## TaskManager Methods

### create_task

```rust
fn create_task(name: String, duration_minutes: u32, category_path: String, task_type: TaskType, distribution_strategy: Option<DistributionStrategy>) -> Result<Task, Error>
```

Creates a new time entry with the specified details.

### update_task

```rust
fn update_task(task_id: Uuid, updates: TaskUpdate) -> Result<Task, Error>
```

Updates an existing task's properties.

### delete_task

```rust
fn delete_task(task_id: Uuid) -> Result<(), Error>
```

Removes a task from the day's entries.

### get_tasks_for_date

```rust
fn get_tasks_for_date(date: NaiveDate) -> Result<Vec<Task>, Error>
```

Retrieves all tasks for a specific date.

### get_tasks_for_range

```rust
fn get_tasks_for_range(start: NaiveDate, end: NaiveDate) -> Result<Vec<Task>, Error>
```

Retrieves tasks for a date range (week view).

### start_timer

```rust
fn start_timer(task_name: String, category_path: Option<String>, task_type: TaskType) -> Result<TimerState, Error>
```

Starts a new timer for a task.

### stop_timer

```rust
fn stop_timer() -> Result<Task, Error>
```

Stops the running timer and creates a task entry.

### get_timer_state

```rust
fn get_timer_state() -> Result<Option<TimerState>, Error>
```

Returns current timer state for frontend polling.

---

## CategoryManager Methods

### get_category_tree

```rust
fn get_category_tree() -> Result<CategoryNode, Error>
```

Returns the full category hierarchy as a tree.

### add_category

```rust
fn add_category(path: String) -> Result<CategoryNode, Error>
```

Adds a new category at the specified path.

### hide_category

```rust
fn hide_category(path: String) -> Result<(), Error>
```

Hides a category from the picker (keeps for historical entries).

### unhide_category

```rust
fn unhide_category(path: String) -> Result<(), Error>
```

Restores a hidden category to the picker.

### validate_category_path

```rust
fn validate_category_path(path: String) -> Result<bool, Error>
```

Checks if a category path exists and is active.

---

## TemplateManager Methods

### create_template

```rust
fn create_template(name: String, default_duration: u32, category_path: String, is_mergeable: bool, distribution_strategy: Option<DistributionStrategy>) -> Result<Template, Error>
```

Creates a new task template.

### update_template

```rust
fn update_template(template_id: Uuid, updates: TemplateUpdate) -> Result<Template, Error>
```

Updates an existing template.

### delete_template

```rust
fn delete_template(template_id: Uuid) -> Result<(), Error>
```

Removes a template.

### get_templates

```rust
fn get_templates() -> Result<Vec<Template>, Error>
```

Returns all templates.

### apply_template

```rust
fn apply_template(template_id: Uuid, overrides: Option<TaskOverrides>) -> Result<Task, Error>
```

Creates a task from a template with optional overrides.

---

## ViewGenerator Methods

### get_full_view

```rust
fn get_full_view(date: NaiveDate) -> Result<FullView, Error>
```

Returns all tasks for a date with actual times.

### get_actitime_view

```rust
fn get_actitime_view(date: NaiveDate) -> Result<ActiTimeView, Error>
```

Returns ActiTime-formatted view with merged times.

### get_week_view

```rust
fn get_week_view(week_start: NaiveDate) -> Result<WeekView, Error>
```

Returns summary view for a week.

---

## ExportManager Methods

### export_backup

```rust
fn export_backup(path: PathBuf) -> Result<(), Error>
```

Exports all data to a backup file.

### import_backup

```rust
fn import_backup(path: PathBuf) -> Result<ImportResult, Error>
```

Imports data from a backup file.

### export_csv

```rust
fn export_csv(date_range: DateRange, path: PathBuf) -> Result<(), Error>
```

Exports time data to CSV format.

---

## SettingsManager Methods

### get_settings

```rust
fn get_settings() -> Result<Settings, Error>
```

Returns current application settings.

### update_settings

```rust
fn update_settings(updates: SettingsUpdate) -> Result<Settings, Error>
```

Updates application settings.

### is_first_run

```rust
fn is_first_run() -> Result<bool, Error>
```

Checks if this is the first application launch.

### complete_first_run

```rust
fn complete_first_run(initial_settings: Settings) -> Result<(), Error>
```

Marks first run as complete with initial settings.

---

## StorageService Methods

### save_tasks

```rust
fn save_tasks(date: NaiveDate, tasks: Vec<Task>) -> Result<(), Error>
```

Persists tasks for a date to YAML.

### load_tasks

```rust
fn load_tasks(date: NaiveDate) -> Result<Vec<Task>, Error>
```

Loads tasks for a date from YAML.

### save_categories

```rust
fn save_categories(categories: CategoryNode) -> Result<(), Error>
```

Persists category tree to YAML.

### load_categories

```rust
fn load_categories() -> Result<CategoryNode, Error>
```

Loads category tree from YAML.

### save_templates

```rust
fn save_templates(templates: Vec<Template>) -> Result<(), Error>
```

Persists templates to YAML.

### load_templates

```rust
fn load_templates() -> Result<Vec<Template>, Error>
```

Loads templates from YAML.

### save_settings

```rust
fn save_settings(settings: Settings) -> Result<(), Error>
```

Persists settings to YAML.

### load_settings

```rust
fn load_settings() -> Result<Settings, Error>
```

Loads settings from YAML.
