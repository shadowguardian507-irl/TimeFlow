# End-to-End Test Instructions - TimeFlow

## Overview

E2E tests validate complete user workflows from start to finish, simulating real user interactions with the application.

## Test Environment Setup

### Prerequisites

- TimeFlow built and running
- Clean data directory (fresh install state)

### Start Application

```bash
# Development mode
pnpm tauri dev

# Or production build
open src-tauri/target/release/bundle/macos/TimeFlow.app
```

## E2E Test Scenarios

### Workflow 1: First-Time User Setup

**Objective**: Verify new user can complete initial setup

**Steps**:

1. Launch application for first time
2. Verify First Run Dialog appears
3. Click "Get Started"
4. Select theme preference
5. Click "Continue"
6. Skip import (click "Start Fresh")
7. Verify main application loads

**Pass Criteria**:

- [ ] First Run Dialog displays correctly
- [ ] Theme selection works
- [ ] Main layout loads after completion
- [ ] Settings saved with selected theme

---

### Workflow 2: Daily Time Tracking

**Objective**: Simulate a typical day of time tracking

**Steps**:

1. **Morning**: Start timer for "Email Review"
   - Enter task name
   - Select category "Overhead > Communication"
   - Click Start
   - Wait 2 minutes
   - Click Stop

2. **Add manual task**: "Team Meeting"
   - Click "Add Task"
   - Name: "Team Meeting"
   - Duration: 60 minutes
   - Category: "Overhead > People Management"
   - Type: Direct
   - Save

3. **Add mergeable task**: "Desk Setup"
   - Click "Add Task"
   - Name: "Desk Setup"
   - Duration: 10 minutes
   - Category: "Overhead > General"
   - Type: Mergeable
   - Strategy: Proportional
   - Save

4. **Review ActiTime View**:
   - Navigate to ActiTime view
   - Verify merged totals
   - Copy to clipboard

**Pass Criteria**:

- [ ] Timer tracks time correctly
- [ ] Tasks appear in list
- [ ] ActiTime view shows merged entries
- [ ] Copy produces valid format

---

### Workflow 3: Category Setup

**Objective**: Set up category hierarchy matching ActiTime

**Steps**:

1. Navigate to Categories view
2. Add root category: "Project A"
3. Add subcategory: "Project A > Development"
4. Add subcategory: "Project A > Development > Backend"
5. Add subcategory: "Project A > Testing"
6. Hide "Project A > Testing"
7. Verify hidden category not in task form picker

**Pass Criteria**:

- [ ] Categories created with correct hierarchy
- [ ] Path separator is " > "
- [ ] Hidden categories excluded from picker
- [ ] Categories persist after restart

---

### Workflow 4: Template Workflow

**Objective**: Create and use task templates

**Steps**:

1. Navigate to Templates view
2. Create template:
   - Name: "Code Review"
   - Duration: 30 minutes
   - Category: "Project A > Development"
   - Type: Direct
3. Navigate to Time Entry view
4. Click "Add Task"
5. Select "Code Review" template
6. Verify form populated
7. Adjust duration to 45 minutes
8. Save task

**Pass Criteria**:

- [ ] Template created successfully
- [ ] Template populates form fields
- [ ] Can override template values
- [ ] Task saved with modified values

---

### Workflow 5: Week Review

**Objective**: Review weekly time summary

**Steps**:

1. Add tasks for multiple days (use date selector)
2. Navigate to Week View
3. Verify daily totals displayed
4. Click on a specific day
5. Verify navigation to that day's Time Entry view

**Pass Criteria**:

- [ ] Week view shows all 7 days
- [ ] Daily totals calculated correctly
- [ ] Week total is sum of daily totals
- [ ] Day click navigates correctly

---

### Workflow 6: Data Backup and Restore

**Objective**: Verify data can be backed up and restored

**Steps**:

1. Create test data:
   - 3 categories
   - 2 templates
   - Change settings
2. Navigate to Settings
3. Click "Export Backup"
4. Note backup path
5. Quit application
6. Delete `~/.timeflow/` directory
7. Launch application
8. Complete first-run
9. Go to Settings
10. Click "Import Backup"
11. Enter backup path
12. Verify data restored

**Pass Criteria**:

- [ ] Backup file created
- [ ] Categories restored
- [ ] Templates restored
- [ ] Settings restored

---

### Workflow 7: App Close with Active Timer

**Objective**: Verify timer save prompt on close

**Steps**:

1. Start a timer
2. Wait 30+ seconds
3. Attempt to close application (Cmd+Q)
4. Verify Close Confirm Dialog appears
5. Click "Save & Close"
6. Relaunch application
7. Verify task was saved

**Pass Criteria**:

- [ ] Dialog appears when timer active
- [ ] Shows timer info (name, elapsed time)
- [ ] "Save & Close" creates task
- [ ] "Discard & Close" doesn't create task
- [ ] "Keep Working" cancels close

---

## Test Execution Checklist

```text
Date: ___________
Tester: ___________

Workflow Results:
[ ] Workflow 1: First-Time User Setup     - PASS / FAIL
[ ] Workflow 2: Daily Time Tracking       - PASS / FAIL
[ ] Workflow 3: Category Setup            - PASS / FAIL
[ ] Workflow 4: Template Workflow         - PASS / FAIL
[ ] Workflow 5: Week Review               - PASS / FAIL
[ ] Workflow 6: Data Backup and Restore   - PASS / FAIL
[ ] Workflow 7: App Close with Active Timer - PASS / FAIL

Overall: _____ / 7 workflows passed

Notes:
_________________________________
_________________________________
```

## Automated E2E Testing (Future)

For automated E2E testing, consider:

### Playwright Setup

```bash
pnpm add -D @playwright/test
```

```typescript
// e2e/time-entry.spec.ts
import { test, expect } from '@playwright/test';

test('can add a task', async ({ page }) => {
  await page.goto('tauri://localhost');
  await page.click('[data-testid="time-entry-add-task"]');
  await page.fill('[data-testid="task-name-input"]', 'Test Task');
  await page.fill('[data-testid="task-duration-input"]', '30');
  // ... continue test
});
```

### WebdriverIO Setup

For cross-platform testing with Tauri's WebDriver support.
