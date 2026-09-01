# Business Rules

## Overview

Detailed business rules and validation logic for TimeFlow.

---

## Task Management Rules

### BR-TASK-01: Task Duration Validation

- Task duration must be >= configured minimum (default: 1 minute)
- Task duration must be > 0
- If duration < minimum, reject with error message

### BR-TASK-02: Category Validation

- Category path must exist in category tree
- Category must not be hidden (for new tasks)
- Historical tasks can reference hidden categories

### BR-TASK-03: Mergeable Task Requirements

- Mergeable tasks MUST have a distribution_strategy
- If strategy is Manual, manual_allocations MUST be provided
- If strategy is Weighted, weights MUST be provided

### BR-TASK-04: Task Date Rules

- Tasks can be created for any date (past, present, future)
- Default date is today when creating new task

### BR-TASK-05: Task Update Rules

- All task fields are editable
- Changing task_type from Direct to Mergeable requires distribution_strategy
- Changing task_type from Mergeable to Direct clears distribution data

---

## Time Distribution Rules

### BR-DIST-01: Proportional Distribution

```text
For each direct task:
  allocation = (direct_task_duration / total_direct_duration) * mergeable_time
```

- Rounding: Allocate whole minutes, remainder goes to longest task
- If all direct tasks have 0 duration, fall back to even distribution

### BR-DIST-02: Even Distribution

```text
For each direct task:
  allocation = mergeable_time / number_of_direct_tasks
```

- Rounding: Allocate whole minutes, remainder distributed round-robin

### BR-DIST-03: Manual Distribution

- User specifies percentage for each selected direct task
- Sum of percentages MUST equal 100%
- At least one direct task must be selected
- Unselected direct tasks receive 0 minutes

### BR-DIST-04: Weighted Distribution

```text
For each direct task with weight:
  allocation = (task_weight / total_weights) * mergeable_time
```

- Tasks without weights receive 0 minutes
- At least one task must have weight > 0

### BR-DIST-05: Distribution Validation

- Total distributed time MUST equal original mergeable time
- No negative allocations allowed
- Allocations are whole minutes only

---

## ActiTime View Rules

### BR-VIEW-01: Direct Task Requirement

- ActiTime view CANNOT be generated if no direct tasks exist
- If only mergeable tasks exist, show error: "At least one direct task is required to generate ActiTime view"

### BR-VIEW-02: Time Aggregation

- Multiple tasks with same category are aggregated
- Aggregated entry shows combined duration
- Task names are concatenated or shown as "Multiple tasks"

### BR-VIEW-03: Output Format

- Format: Table with columns [Category Path, Duration]
- Category path uses " > " separator
- Duration shown in exact minutes (no rounding)

### BR-VIEW-04: Merge Calculation Order

1. Group all tasks by date
2. Separate direct and mergeable tasks
3. For each mergeable task, calculate distribution
4. Add distributed time to direct tasks
5. Aggregate by category

---

## Daily Validation Rules

### BR-DAILY-01: Work Day Warning

- If total daily time > configured work_day_hours, show warning
- Default work_day_hours: 8
- Warning is non-blocking (user can proceed)

### BR-DAILY-02: Maximum Time Warning

- If total daily time > 24 hours, show error
- This is a blocking error (likely data entry mistake)

### BR-DAILY-03: Time Calculation

- Total time = sum of all task durations (direct + mergeable)
- Mergeable time is NOT double-counted in total

---

## Timer Rules

### BR-TIMER-01: Single Timer

- Only one timer can run at a time
- Starting new timer while one is running: prompt to save/discard current

### BR-TIMER-02: Timer Persistence

- Timer state persisted to temp file for crash recovery
- On app start, check for orphaned timer state
- If found, prompt user to recover or discard

### BR-TIMER-03: App Close Behavior

- If timer running when app closes, prompt user:
  - "Save" - stop timer and create task
  - "Discard" - stop timer without saving
  - "Cancel" - keep app open

### BR-TIMER-04: Timer Duration

- Duration calculated as: current_time - start_time
- Minimum duration rules apply when saving

---

## Category Rules

### BR-CAT-01: Path Format

- Levels separated by " > " (space-greater than-space)
- Example: "Overhead > People Management > People Care"

### BR-CAT-02: Adding Categories

- New category added as child of specified parent
- If parent doesn't exist, create parent hierarchy
- Category names cannot contain " > "

### BR-CAT-03: Hiding Categories

- Hidden categories not shown in picker for new tasks
- Hidden categories still visible on existing tasks
- Hidden categories can be unhidden

### BR-CAT-04: Category Deletion

- Categories cannot be deleted (only hidden)
- This preserves historical data integrity

---

## Template Rules

### BR-TMPL-01: Template Application

- Applying template creates new task with template defaults
- All template values can be overridden
- Template category must still be valid (not hidden)

### BR-TMPL-02: Template Validation

- Template name must be unique
- Template category must exist
- Template duration must meet minimum duration rule

---

## Export Rules

### BR-EXP-01: Backup Format

- Backup includes: tasks, categories, templates, settings
- Format: Single YAML file with all data
- Filename includes timestamp

### BR-EXP-02: CSV Export

- Columns: Date, Task Name, Category, Duration (minutes), Type
- One row per task (not aggregated)
- Includes both direct and mergeable tasks

### BR-EXP-03: Import Validation

- Validate backup file structure before import
- Check for category conflicts
- Prompt user for conflict resolution strategy
