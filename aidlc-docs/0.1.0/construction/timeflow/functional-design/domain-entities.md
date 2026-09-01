# Domain Entities

## Overview

Core domain entities for the TimeFlow application.

---

## Entity: Task

Represents a single time entry for a specific date.

### Fields

| Field                 | Type                    | Required | Description                                               |
| --------------------- | ----------------------- | -------- | --------------------------------------------------------- |
| id                    | UUID                    | Yes      | Unique identifier                                         |
| name                  | String                  | Yes      | Task name/description                                     |
| date                  | Date                    | Yes      | Date the task was performed                               |
| duration_minutes      | u32                     | Yes      | Duration in minutes                                       |
| category_path         | String                  | Yes      | Full category path (e.g., "Overhead > People Management") |
| task_type             | TaskType                | Yes      | Direct or Mergeable                                       |
| distribution_strategy | DistributionStrategy    | No       | Only for mergeable tasks                                  |
| manual_allocations    | Vec<ManualAllocation>   | No       | Only for manual distribution                              |
| weights               | Vec<WeightedAllocation> | No       | Only for weighted distribution                            |
| created_at            | DateTime                | Yes      | When the task was created                                 |
| updated_at            | DateTime                | Yes      | When the task was last modified                           |

### Constraints

- duration_minutes >= configured minimum (default: 1)
- category_path must exist in category tree
- distribution_strategy required if task_type is Mergeable

---

## Enum: TaskType

```rust
enum TaskType {
    Direct,     // Maps directly to ActiTime entry
    Mergeable,  // Time distributed to direct tasks
}
```

---

## Enum: DistributionStrategy

```rust
enum DistributionStrategy {
    Proportional,  // Based on direct task durations
    Even,          // Split equally
    Manual,        // User-specified allocations
    Weighted,      // User-assigned weights
}
```

---

## Entity: ManualAllocation

For manual distribution strategy - specifies which tasks receive time.

### Fields

| Field          | Type | Required | Description                          |
| -------------- | ---- | -------- | ------------------------------------ |
| target_task_id | UUID | Yes      | Direct task to receive time          |
| percentage     | f32  | Yes      | Percentage of mergeable time (0-100) |

### Constraints

- Sum of all percentages must equal 100

---

## Entity: WeightedAllocation

For weighted distribution strategy.

### Fields

| Field          | Type | Required | Description                           |
| -------------- | ---- | -------- | ------------------------------------- |
| target_task_id | UUID | Yes      | Direct task to receive time           |
| weight         | f32  | Yes      | Relative weight (any positive number) |

---

## Entity: Category

Represents a node in the category hierarchy tree.

### Fields

| Field    | Type          | Required | Description                    |
| -------- | ------------- | -------- | ------------------------------ |
| name     | String        | Yes      | Category name (this node only) |
| hidden   | bool          | Yes      | Whether hidden from picker     |
| children | Vec<Category> | Yes      | Child categories               |

### Notes

- Full path is computed at runtime by traversing the tree hierarchy
- No redundant path storage - path derived from parent chain

### Example Structure (YAML)

```yaml
name: "Root"
hidden: false
children:
  - name: "Overhead"
    hidden: false
    children:
      - name: "People Management"
        hidden: false
        children:
          - name: "People Care"
            hidden: false
            children: []
```

### Path Computation

```rust
// Path computed at runtime: "Overhead > People Management > People Care"
fn get_path(node: &Category, ancestors: &[String]) -> String {
    let mut parts = ancestors.to_vec();
    parts.push(node.name.clone());
    parts.join(" > ")
}
```

---

## Entity: Template

Represents a reusable task template.

### Fields

| Field                 | Type                 | Required | Description                    |
| --------------------- | -------------------- | -------- | ------------------------------ |
| id                    | UUID                 | Yes      | Unique identifier              |
| name                  | String               | Yes      | Template name                  |
| default_duration      | u32                  | Yes      | Default duration in minutes    |
| category_path         | String               | Yes      | Default category path          |
| is_mergeable          | bool                 | Yes      | Whether tasks are mergeable    |
| distribution_strategy | DistributionStrategy | No       | Default strategy for mergeable |
| created_at            | DateTime             | Yes      | When template was created      |

---

## Entity: TimerState

Represents the current state of a running timer.

### Fields

| Field                 | Type                 | Required | Description                 |
| --------------------- | -------------------- | -------- | --------------------------- |
| task_name             | String               | Yes      | Name of task being timed    |
| category_path         | String               | No       | Category (can be set later) |
| task_type             | TaskType             | Yes      | Direct or Mergeable         |
| distribution_strategy | DistributionStrategy | No       | For mergeable tasks         |
| start_time            | DateTime             | Yes      | When timer was started      |
| is_running            | bool                 | Yes      | Whether timer is active     |

---

## Entity: Settings

Application settings and user preferences.

### Fields

| Field                         | Type                 | Required | Description                                 |
| ----------------------------- | -------------------- | -------- | ------------------------------------------- |
| theme                         | Theme                | Yes      | UI theme preference                         |
| first_run_complete            | bool                 | Yes      | Whether first run setup done                |
| minimum_task_duration         | u32                  | Yes      | Minimum task duration (default: 1)          |
| work_day_hours                | f32                  | Yes      | Expected work day for warnings (default: 8) |
| default_distribution_strategy | DistributionStrategy | Yes      | Default for new mergeable tasks             |

---

## Enum: Theme

```rust
enum Theme {
    Light,
    Dark,
    System,  // Follow OS setting
}
```

---

## Entity: MergedEntry

Output entity for ActiTime-tuned view.

### Fields

| Field             | Type             | Required | Description                       |
| ----------------- | ---------------- | -------- | --------------------------------- |
| task_id           | UUID             | Yes      | Original direct task ID           |
| task_name         | String           | Yes      | Task name                         |
| category_path     | String           | Yes      | Category path                     |
| original_duration | u32              | Yes      | Original task duration            |
| merged_duration   | u32              | Yes      | Duration after adding merged time |
| merged_from       | Vec<MergeSource> | Yes      | Sources of merged time            |

---

## Entity: MergeSource

Tracks where merged time came from.

### Fields

| Field               | Type   | Required | Description                     |
| ------------------- | ------ | -------- | ------------------------------- |
| source_task_id      | UUID   | Yes      | Mergeable task that contributed |
| source_task_name    | String | Yes      | Name of source task             |
| contributed_minutes | u32    | Yes      | Minutes contributed             |
