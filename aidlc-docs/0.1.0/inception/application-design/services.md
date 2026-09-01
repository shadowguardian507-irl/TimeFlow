# Services

## Overview

Service layer components that orchestrate business logic across multiple managers.

---

## Service: TimeMergeService

### Purpose

Handles the core business logic for distributing mergeable task time to direct tasks.

### Responsibilities

- Calculate time distribution based on strategy (proportional, even, manual, weighted)
- Apply distribution to generate ActiTime-ready entries
- Validate distribution results (total time matches)
- Handle edge cases (no direct tasks, single direct task)

### Methods

#### calculate_distribution

```rust
fn calculate_distribution(mergeable_tasks: Vec<Task>, direct_tasks: Vec<Task>) -> Result<Vec<MergedEntry>, Error>
```

Calculates how mergeable task time should be distributed to direct tasks.

#### apply_proportional_distribution

```rust
fn apply_proportional_distribution(mergeable_time: u32, direct_tasks: &[Task]) -> Vec<TimeAllocation>
```

Distributes time proportionally based on direct task durations.

#### apply_even_distribution

```rust
fn apply_even_distribution(mergeable_time: u32, direct_tasks: &[Task]) -> Vec<TimeAllocation>
```

Distributes time evenly across all direct tasks.

#### apply_manual_distribution

```rust
fn apply_manual_distribution(mergeable_time: u32, allocations: &[ManualAllocation]) -> Vec<TimeAllocation>
```

Distributes time according to user-specified allocations.

#### apply_weighted_distribution

```rust
fn apply_weighted_distribution(mergeable_time: u32, weights: &[WeightedTask]) -> Vec<TimeAllocation>
```

Distributes time according to user-assigned weights.

### Orchestration

- Called by ViewGenerator when generating ActiTime view
- Uses task data from TaskManager
- Returns merged entries for display

---

## Service: TimerService

### Purpose

Manages the backend timer state and elapsed time tracking.

### Responsibilities

- Track timer start time
- Calculate elapsed time on poll
- Handle timer pause/resume (future)
- Persist timer state for crash recovery

### Methods

#### start

```rust
fn start(task_info: TimerTaskInfo) -> Result<TimerState, Error>
```

Starts a new timer with task information.

#### stop

```rust
fn stop() -> Result<ElapsedTime, Error>
```

Stops the timer and returns elapsed time.

#### get_state

```rust
fn get_state() -> Option<TimerState>
```

Returns current timer state (for polling).

#### is_running

```rust
fn is_running() -> bool
```

Checks if a timer is currently active.

### State

- Holds in-memory timer state
- Persists to temp file for crash recovery

---

## Service: DataIntegrityService

### Purpose

Ensures data consistency and handles recovery scenarios.

### Responsibilities

- Validate data on load
- Handle corrupted file recovery
- Ensure atomic writes
- Manage backup before destructive operations

### Methods

#### validate_task_data

```rust
fn validate_task_data(tasks: &[Task]) -> Result<ValidationResult, Error>
```

Validates task data integrity.

#### validate_category_tree

```rust
fn validate_category_tree(tree: &CategoryNode) -> Result<ValidationResult, Error>
```

Validates category tree structure.

#### create_recovery_backup

```rust
fn create_recovery_backup() -> Result<PathBuf, Error>
```

Creates a backup before potentially destructive operations.

#### atomic_write

```rust
fn atomic_write(path: &Path, content: &str) -> Result<(), Error>
```

Writes file atomically (write to temp, then rename).
