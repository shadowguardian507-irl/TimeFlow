# Frontend Components

## Overview

UI component structure for the TimeFlow Tauri application.

---

## Component Hierarchy

```text
App
+-- Layout
    +-- Sidebar
    |   +-- NavigationMenu
    |   +-- QuickActions
    +-- MainContent
        +-- Header
        |   +-- DateSelector
        |   +-- ViewToggle
        +-- TimeEntryView (default)
        |   +-- TimerWidget
        |   +-- TaskList
        |   |   +-- TaskItem
        |   +-- TaskEntryForm
        |   +-- DailySummary
        +-- ActiTimeView
        |   +-- MergedTaskTable
        |   +-- CopyButton
        +-- WeekView
        |   +-- WeekCalendar
        |   +-- DaySummaryCard
        +-- TemplatesView
        |   +-- TemplateList
        |   +-- TemplateForm
        +-- CategoriesView
        |   +-- CategoryTree
        |   +-- CategoryForm
        +-- SettingsView
            +-- ThemeSelector
            +-- PreferencesForm
            +-- ExportImportSection
```

---

## Core Components

### App

Root component managing global state and routing.

**State:**

- currentView: string
- currentDate: Date
- settings: Settings
- timerState: TimerState | null

**Backend Commands:**

- `get_settings()` on mount
- `get_timer_state()` on mount

---

### TimerWidget

Displays and controls the running timer.

**Props:**

| Prop       | Type       | Description          |
| ---------- | ---------- | -------------------- |
| timerState | TimerState | Current timer state  |
| onStart    | () => void | Start timer callback |
| onStop     | () => void | Stop timer callback  |

**State:**

- elapsedSeconds: number (updated via polling)
- taskName: string
- categoryPath: string

**Backend Commands:**

- `start_timer(name, category, type)`
- `stop_timer()`
- `get_timer_state()` (polling every 1 second)

**UI Elements:**

- Timer display (HH:MM:SS)
- Task name input
- Category picker (optional)
- Start/Stop button
- Task type toggle (Direct/Mergeable)

---

### TaskList

Displays tasks for the selected date.

**Props:**

| Prop     | Type             | Description          |
| -------- | ---------------- | -------------------- |
| date     | Date             | Selected date        |
| tasks    | Task[]           | Tasks for the date   |
| onEdit   | (task) => void   | Edit task callback   |
| onDelete | (taskId) => void | Delete task callback |

**Backend Commands:**

- `get_tasks_for_date(date)`
- `delete_task(taskId)`

---

### TaskItem

Single task row in the task list.

**Props:**

| Prop     | Type       | Description     |
| -------- | ---------- | --------------- |
| task     | Task       | Task data       |
| onEdit   | () => void | Edit callback   |
| onDelete | () => void | Delete callback |

**UI Elements:**

- Task name
- Duration (minutes)
- Category path (truncated with tooltip)
- Type badge (Direct/Mergeable)
- Edit/Delete buttons

---

### TaskEntryForm

Form for creating/editing tasks.

**Props:**

| Prop       | Type           | Description                 |
| ---------- | -------------- | --------------------------- |
| task       | Task           | Task to edit (null for new) |
| templates  | Template[]     | Available templates         |
| categories | CategoryNode   | Category tree               |
| onSave     | (task) => void | Save callback               |
| onCancel   | () => void     | Cancel callback             |

**State:**

- name: string
- duration: number
- categoryPath: string
- taskType: TaskType
- distributionStrategy: DistributionStrategy
- manualAllocations: ManualAllocation[]
- weights: WeightedAllocation[]

**Backend Commands:**

- `create_task(...)`
- `update_task(id, ...)`
- `apply_template(templateId, overrides)`

**Validation:**

- Duration >= minimum
- Category selected
- Strategy selected if mergeable
- Allocations valid if manual/weighted

---

### CategoryPicker

Hierarchical category selector.

**Props:**

| Prop       | Type           | Description            |
| ---------- | -------------- | ---------------------- |
| categories | CategoryNode   | Category tree          |
| value      | string         | Selected path          |
| onChange   | (path) => void | Selection callback     |
| showHidden | boolean        | Show hidden categories |

**UI Elements:**

- Expandable tree view
- Search/filter input
- Selected path display
- Hidden indicator (if showHidden)

---

### MergedTaskTable

ActiTime-ready table view.

**Props:**

| Prop      | Type          | Description    |
| --------- | ------------- | -------------- |
| entries   | MergedEntry[] | Merged entries |
| totalTime | number        | Total minutes  |

**UI Elements:**

- Table with columns: Category, Duration
- Category path with " > " separator
- Duration in minutes
- Total row at bottom
- Copy to clipboard button

---

### DateSelector

Date navigation component.

**Props:**

| Prop     | Type           | Description          |
| -------- | -------------- | -------------------- |
| value    | Date           | Selected date        |
| onChange | (date) => void | Date change callback |

**UI Elements:**

- Previous/Next day buttons
- Date display
- Calendar popup picker
- "Today" quick button

---

### WeekCalendar

Week view with day summaries.

**Props:**

| Prop        | Type           | Description            |
| ----------- | -------------- | ---------------------- |
| weekStart   | Date           | Start of week          |
| onDaySelect | (date) => void | Day selection callback |

**Backend Commands:**

- `get_tasks_for_range(start, end)`

**UI Elements:**

- 7 day columns
- Daily total for each day
- Visual indicator for current day
- Click to navigate to day

---

### DailySummary

Summary statistics for current day.

**Props:**

| Prop     | Type     | Description   |
| -------- | -------- | ------------- |
| tasks    | Task[]   | Day's tasks   |
| settings | Settings | User settings |

**UI Elements:**

- Total time
- Direct vs Mergeable breakdown
- Warning if exceeds work day
- Error if exceeds 24 hours

---

### TemplateList

List of task templates.

**Props:**

| Prop      | Type                 | Description     |
| --------- | -------------------- | --------------- |
| templates | Template[]           | All templates   |
| onApply   | (template) => void   | Apply template  |
| onEdit    | (template) => void   | Edit template   |
| onDelete  | (templateId) => void | Delete template |

**Backend Commands:**

- `get_templates()`
- `delete_template(id)`

---

### TemplateForm

Form for creating/editing templates.

**Props:**

| Prop       | Type               | Description      |
| ---------- | ------------------ | ---------------- |
| template   | Template           | Template to edit |
| categories | CategoryNode       | Category tree    |
| onSave     | (template) => void | Save callback    |
| onCancel   | () => void         | Cancel callback  |

**Backend Commands:**

- `create_template(...)`
- `update_template(id, ...)`

---

### CategoryTree

Category management tree view.

**Props:**

| Prop       | Type                 | Description     |
| ---------- | -------------------- | --------------- |
| categories | CategoryNode         | Category tree   |
| onAdd      | (parentPath) => void | Add category    |
| onHide     | (path) => void       | Hide category   |
| onUnhide   | (path) => void       | Unhide category |

**Backend Commands:**

- `get_category_tree()`
- `add_category(path)`
- `hide_category(path)`
- `unhide_category(path)`

---

### SettingsView

Application settings panel.

**Props:**

| Prop     | Type               | Description      |
| -------- | ------------------ | ---------------- |
| settings | Settings           | Current settings |
| onSave   | (settings) => void | Save callback    |

**Sections:**

- Theme selection (Light/Dark/System)
- Minimum task duration
- Work day hours
- Default distribution strategy

**Backend Commands:**

- `get_settings()`
- `update_settings(...)`
- `export_backup(path)`
- `import_backup(path)`
- `export_csv(dateRange, path)`

---

## First Run Flow

### FirstRunDialog

Modal shown on first application launch.

**State:**

- step: number (1-3)
- selectedTheme: Theme

**Steps:**

1. Welcome message
2. Theme selection
3. Optional: Import existing data

**Backend Commands:**

- `is_first_run()`
- `complete_first_run(settings)`

---

## App Close Handler

### CloseConfirmDialog

Modal shown when closing with active timer.

**Props:**

| Prop       | Type       | Description       |
| ---------- | ---------- | ----------------- |
| timerState | TimerState | Active timer      |
| onSave     | () => void | Save and close    |
| onDiscard  | () => void | Discard and close |
| onCancel   | () => void | Cancel close      |

**UI Elements:**

- Warning message
- Timer info (name, elapsed time)
- Save / Discard / Cancel buttons
