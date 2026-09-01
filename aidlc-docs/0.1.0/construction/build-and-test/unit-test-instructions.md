# Unit Test Execution - TimeFlow

## Overview

TimeFlow uses:

- **Rust tests**: For backend business logic (services, models)
- **Vitest**: For frontend component and store testing (optional, not yet configured)

## Backend Unit Tests (Rust)

### Run All Rust Tests

```bash
# Run all tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run with output
cargo test --manifest-path src-tauri/Cargo.toml -- --nocapture

# Run specific test module
cargo test --manifest-path src-tauri/Cargo.toml time_merge

# Run specific test
cargo test --manifest-path src-tauri/Cargo.toml test_proportional_distribution
```

### Expected Test Coverage

| Module             | Tests | Description                                                    |
| ------------------ | ----- | -------------------------------------------------------------- |
| `time_merge`       | 4+    | Distribution algorithms (proportional, even, manual, weighted) |
| `task_manager`     | 3+    | Task CRUD operations                                           |
| `category_manager` | 3+    | Category tree operations                                       |
| `template_manager` | 3+    | Template CRUD operations                                       |
| `view_generator`   | 3+    | View generation (full, actitime, week)                         |
| `storage`          | 2+    | YAML read/write operations                                     |

### Review Test Results

**Expected Output (All Pass)**:

```text
running X tests
test services::time_merge::tests::test_proportional_distribution ... ok
test services::time_merge::tests::test_even_distribution ... ok
test services::time_merge::tests::test_manual_distribution ... ok
test services::time_merge::tests::test_weighted_distribution ... ok
...

test result: ok. X passed; 0 failed; 0 ignored
```

### Adding Unit Tests

To add tests for a module, add a `tests` submodule:

```rust
// In src-tauri/src/services/example.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_function() {
        let result = example_function();
        assert_eq!(result, expected_value);
    }
}
```

## Frontend Unit Tests (Optional)

### Setup Vitest (If Needed)

```bash
# Install Vitest
pnpm add -D vitest @testing-library/svelte jsdom

# Add to package.json scripts
# "test": "vitest run",
# "test:watch": "vitest"
```

### Create Test File

```typescript
// src/lib/stores/tasks.test.ts
import { describe, it, expect, vi } from 'vitest';
import { tasksStore } from './tasks';

describe('tasksStore', () => {
  it('should initialize with empty tasks', () => {
    // Test implementation
  });
});
```

### Run Frontend Tests

```bash
# Run once
pnpm test

# Watch mode
pnpm test:watch
```

## Test Coverage Report

### Generate Rust Coverage (Optional)

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --manifest-path src-tauri/Cargo.toml --out Html

# View report
open tarpaulin-report.html
```

## Fix Failing Tests

If tests fail:

1. **Read the error message** - Identify which test failed and why
2. **Check the assertion** - Verify expected vs actual values
3. **Debug the code** - Add `println!` or use debugger
4. **Fix the issue** - Update code or test as needed
5. **Rerun tests** - Verify the fix

### Common Test Failures

**Assertion Failed**:

```text
assertion failed: `(left == right)`
  left: `10`,
 right: `15`
```

→ Check the calculation logic in the function being tested

**Panic in Test**:

```text
thread 'test_name' panicked at 'called `Option::unwrap()` on a `None` value'
```

→ Handle the `None` case or ensure test data is valid

**Timeout**:

```text
test test_name has been running for over 60 seconds
```

→ Check for infinite loops or blocking operations
