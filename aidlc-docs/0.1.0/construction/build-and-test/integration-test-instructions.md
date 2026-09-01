# Integration Test Instructions - TimeFlow

## Overview

Integration tests verify that TimeFlow's components work together correctly:

- Frontend ↔ Backend communication via Tauri commands
- Service layer interactions
- Data persistence across operations

## Test Scenarios

### Scenario 1: Task Creation Flow

**Description**: Verify complete task creation from UI to storage

**Setup**:

1. Start the application in development mode
2. Ensure clean data directory (backup and remove `~/.timeflow/`)

**Test Steps**:

1. Click "Add Task" button
2. Fill in task details:
   - Name: "Test Task"
   - Duration: 30 minutes
   - Category: Select any category
   - Type: Direct
3. Click "Add Task" to save
4. Verify task appears in task list

**Expected Results**:

- Task appears in the list immediately
- Task persists after app restart
- YAML file created at `~/.timeflow/tasks/YYYY-MM-DD.yaml`

**Cleanup**:

- Delete test task or restore backup

---

### Scenario 2: Timer Flow

**Description**: Verify timer start, tracking, and task creation

**Test Steps**:

1. Enter task name in timer widget
2. Select category (optional)
3. Click "Start" button
4. Wait 10+ seconds
5. Click "Stop" button
6. Verify task created with correct duration

**Expected Results**:

- Timer displays elapsed time
- Task created with duration matching elapsed time
- Timer state persists if app crashes (check `~/.timeflow/timer.yaml`)

---

### Scenario 3: Category Management

**Description**: Verify category CRUD operations

**Test Steps**:

1. Navigate to Categories view
2. Add new category: "Test > Subcategory"
3. Verify category appears in tree
4. Hide the category
5. Verify category shows "hidden" badge
6. Unhide the category
7. Verify category is visible again

**Expected Results**:

- Category tree updates immediately
- Changes persist in `~/.timeflow/categories.yaml`
- Hidden categories don't appear in task form picker (unless "show hidden" enabled)

---

### Scenario 4: ActiTime View Generation

**Description**: Verify mergeable task distribution and ActiTime view

**Setup**:

1. Create 2 direct tasks with different durations
2. Create 1 mergeable task with "proportional" strategy

**Test Steps**:

1. Navigate to ActiTime view
2. Verify merged entries show correct totals
3. Click "Copy to Clipboard"
4. Paste and verify format

**Expected Results**:

- Mergeable task time distributed to direct tasks proportionally
- Total time matches sum of all tasks
- Clipboard contains tab-separated category and duration

---

### Scenario 5: Template Application

**Description**: Verify template creation and application

**Test Steps**:

1. Navigate to Templates view
2. Create new template:
   - Name: "Daily Standup"
   - Duration: 15 minutes
   - Category: Select category
   - Type: Direct
3. Navigate to Time Entry view
4. Click "Add Task"
5. Select template from dropdown
6. Verify form populated with template values
7. Save task

**Expected Results**:

- Template saved to `~/.timeflow/templates.yaml`
- Form fields populated from template
- Task created with template values

---

### Scenario 6: Settings Persistence

**Description**: Verify settings save and load

**Test Steps**:

1. Navigate to Settings view
2. Change theme to "Dark"
3. Change minimum task duration to 5 minutes
4. Click "Save Settings"
5. Restart application
6. Verify settings persisted

**Expected Results**:

- Theme changes immediately
- Settings persist in `~/.timeflow/settings.yaml`
- Settings loaded on app restart

---

### Scenario 7: Data Export/Import

**Description**: Verify backup and restore functionality

**Test Steps**:

1. Create some test data (tasks, categories, templates)
2. Navigate to Settings view
3. Click "Export Backup"
4. Note the backup file path
5. Delete all data (remove `~/.timeflow/` directory)
6. Restart application
7. Complete first-run wizard
8. Import the backup file
9. Verify data restored

**Expected Results**:

- Backup file created as YAML
- Import restores categories, templates, and settings
- Tasks are NOT included in backup (date-specific)

---

## Running Integration Tests

### Manual Testing Checklist

Run through each scenario manually:

```text
[ ] Scenario 1: Task Creation Flow
[ ] Scenario 2: Timer Flow
[ ] Scenario 3: Category Management
[ ] Scenario 4: ActiTime View Generation
[ ] Scenario 5: Template Application
[ ] Scenario 6: Settings Persistence
[ ] Scenario 7: Data Export/Import
```

### Automated Integration Tests (Future)

For automated testing, consider:

- **Tauri's test utilities** for command testing
- **Playwright** for E2E UI testing
- **WebDriver** for cross-platform testing

## Troubleshooting

### Frontend Not Communicating with Backend

**Symptoms**: Actions don't persist, errors in console

**Check**:

1. Open DevTools (Cmd+Option+I)
2. Check Console for errors
3. Verify Tauri commands are being called

### Data Not Persisting

**Symptoms**: Data lost after restart

**Check**:

1. Verify `~/.timeflow/` directory exists
2. Check file permissions
3. Look for error messages in terminal

### Timer Not Updating

**Symptoms**: Timer display frozen

**Check**:

1. Verify polling is active (check Network tab)
2. Check for JavaScript errors
3. Restart the application
