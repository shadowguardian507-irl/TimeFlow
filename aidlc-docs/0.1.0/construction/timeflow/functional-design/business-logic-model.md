# Business Logic Model

## Overview

Core business logic flows and algorithms for TimeFlow.

---

## Task Creation Flow

```text
Input: TaskInput (name, duration, category, type, strategy?)
                    |
                    v
        +---------------------+
        | Validate Duration   |
        | (>= minimum)        |
        +---------------------+
                    |
                    v
        +---------------------+
        | Validate Category   |
        | (exists, not hidden)|
        +---------------------+
                    |
                    v
        +---------------------+
        | If Mergeable:       |
        | Validate Strategy   |
        +---------------------+
                    |
                    v
        +---------------------+
        | Generate UUID       |
        | Set timestamps      |
        +---------------------+
                    |
                    v
        +---------------------+
        | Persist to Storage  |
        +---------------------+
                    |
                    v
Output: Task
```

---

## Timer Flow

```text
START TIMER:
                    |
                    v
        +---------------------+
        | Check existing      |
        | timer running?      |
        +---------------------+
                    |
            Yes     |     No
        +-----------+-----------+
        |                       |
        v                       v
+---------------+       +---------------+
| Prompt: Save/ |       | Create Timer  |
| Discard/Cancel|       | State         |
+---------------+       +---------------+
        |                       |
        v                       v
(handle choice)         +---------------+
                        | Persist to    |
                        | temp file     |
                        +---------------+
                                |
                                v
                        +---------------+
                        | Return Timer  |
                        | State         |
                        +---------------+

STOP TIMER:
                    |
                    v
        +---------------------+
        | Calculate elapsed   |
        | time                |
        +---------------------+
                    |
                    v
        +---------------------+
        | Create Task with    |
        | elapsed duration    |
        +---------------------+
                    |
                    v
        +---------------------+
        | Clear timer state   |
        | Delete temp file    |
        +---------------------+
                    |
                    v
Output: Task
```

---

## Time Merge Algorithm

```text
Input: tasks[] for a date
                    |
                    v
        +---------------------+
        | Separate:           |
        | direct_tasks[]      |
        | mergeable_tasks[]   |
        +---------------------+
                    |
                    v
        +---------------------+
        | If no direct tasks: |
        | Return ERROR        |
        +---------------------+
                    |
                    v
        +---------------------+
        | For each mergeable: |
        | Calculate allocation|
        +---------------------+
                    |
                    v
    +---------------+---------------+
    |               |               |
    v               v               v
Proportional    Even          Manual/Weighted
    |               |               |
    v               v               v
+----------+   +----------+   +----------+
|duration/ |   |time/     |   |user-     |
|total *   |   |count     |   |specified |
|merge_time|   |          |   |          |
+----------+   +----------+   +----------+
    |               |               |
    +---------------+---------------+
                    |
                    v
        +---------------------+
        | Apply allocations   |
        | to direct tasks     |
        +---------------------+
                    |
                    v
        +---------------------+
        | Aggregate by        |
        | category            |
        +---------------------+
                    |
                    v
Output: MergedEntry[]
```

---

## Proportional Distribution Algorithm

```
// Pseudocode - will be implemented in Rust

fn proportional_distribute(mergeable_time, direct_tasks):
    total_direct_time = sum of task.duration for all direct_tasks
    
    if total_direct_time == 0:
        // Fall back to even distribution
        return even_distribute(mergeable_time, direct_tasks)
    
    allocations = []
    allocated = 0
    
    // Sort by duration descending (longest first for remainder)
    sorted_tasks = sort direct_tasks by duration descending
    
    for i, task in sorted_tasks:
        if i == last task:
            // Last task gets remainder
            allocation = mergeable_time - allocated
        else:
            ratio = task.duration / total_direct_time
            allocation = floor(mergeable_time * ratio)
        
        allocations.push((task.id, allocation))
        allocated += allocation
    
    return allocations
```

---

## Even Distribution Algorithm

```
// Pseudocode - will be implemented in Rust

fn even_distribute(mergeable_time, direct_tasks):
    count = length of direct_tasks
    base_allocation = mergeable_time / count (integer division)
    remainder = mergeable_time % count
    
    allocations = []
    for i, task in direct_tasks:
        allocation = base_allocation
        if i < remainder:
            allocation += 1  // Distribute remainder round-robin
        allocations.push((task.id, allocation))
    
    return allocations
```

---

## Category Tree Operations

### Add Category

```python
def add_category(tree, path):
    parts = path.split(" > ")
    current = tree
    
    for i, part in enumerate(parts):
        partial_path = " > ".join(parts[:i+1])
        
        # Find or create child
        child = find_child(current, part)
        if child is None:
            child = Category(
                name=part,
                path=partial_path,
                hidden=False,
                children=[]
            )
            current.children.append(child)
        
        current = child
    
    return current
```

### Hide Category

```python
def hide_category(tree, path):
    category = find_by_path(tree, path)
    if category:
        category.hidden = True
        # Recursively hide children
        for child in category.children:
            hide_category(tree, child.path)
```

---

## View Generation Logic

### Full View

```python
def generate_full_view(date):
    tasks = load_tasks(date)
    
    return FullView(
        date=date,
        tasks=tasks,
        total_time=sum(t.duration for t in tasks),
        direct_count=count(t for t in tasks if t.type == Direct),
        mergeable_count=count(t for t in tasks if t.type == Mergeable)
    )
```

### ActiTime View

```python
def generate_actitime_view(date):
    tasks = load_tasks(date)
    direct = [t for t in tasks if t.type == Direct]
    mergeable = [t for t in tasks if t.type == Mergeable]
    
    if not direct and mergeable:
        raise Error("At least one direct task required")
    
    # Calculate merged entries
    merged = calculate_distribution(mergeable, direct)
    
    # Aggregate by category
    by_category = {}
    for entry in merged:
        key = entry.category_path
        if key in by_category:
            by_category[key].duration += entry.merged_duration
        else:
            by_category[key] = entry
    
    return ActiTimeView(
        date=date,
        entries=list(by_category.values()),
        total_time=sum(e.merged_duration for e in by_category.values())
    )
```

---

## Daily Validation Logic

```python
def validate_daily_time(date, settings):
    tasks = load_tasks(date)
    total_minutes = sum(t.duration for t in tasks)
    total_hours = total_minutes / 60
    
    warnings = []
    errors = []
    
    if total_hours > 24:
        errors.append("Total time exceeds 24 hours - please check entries")
    elif total_hours > settings.work_day_hours:
        warnings.append(f"Total time ({total_hours:.1f}h) exceeds expected work day ({settings.work_day_hours}h)")
    
    return ValidationResult(warnings=warnings, errors=errors)
```
