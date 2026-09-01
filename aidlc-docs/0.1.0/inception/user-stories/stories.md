# User Stories

## Story Organization

Stories are organized by feature area with mixed granularity (coarse for simple features, fine for complex ones).

Priority: High / Medium / Low

---

## Epic: Time Entry (FR-01)

### Story TE-01: Manual Time Entry

**Priority**: High

As a Time Tracker, I want to manually enter a task with its duration so that I can log time for work I've already completed.

**Acceptance Criteria**:

- Given I am on the time entry screen
- When I enter a task name and duration
- Then the task is added to today's time log
- And the task appears in the full view

### Story TE-02: Timer-Based Time Entry

**Priority**: High

As a Time Tracker, I want to start and stop a timer for a task so that I can track time as I work without remembering durations.

**Acceptance Criteria**:

- Given I am on the time entry screen
- When I start a timer for a task
- Then the timer begins counting
- And I can see the elapsed time

- Given a timer is running
- When I stop the timer
- Then the task is added to today's time log with the elapsed duration

### Story TE-03: Classify Task as Direct

**Priority**: High

As a Time Tracker, I want to mark a task as "direct" so that it maps directly to an ActiTime entry.

**Acceptance Criteria**:

- Given I am creating or editing a task
- When I set the task type to "direct"
- Then the task will appear in the ActiTime-tuned view
- And the task's time is not distributed to other tasks

### Story TE-04: Classify Task as Mergeable

**Priority**: High

As a Time Tracker, I want to mark a task as "mergeable" so that its time gets distributed to direct tasks.

**Acceptance Criteria**:

- Given I am creating or editing a task
- When I set the task type to "mergeable"
- Then the task appears in the full view but not directly in ActiTime-tuned view
- And the task's time is distributed to direct tasks

### Story TE-05: Proportional Time Distribution

**Priority**: High

As a Time Tracker, I want mergeable task time distributed proportionally to direct tasks so that longer tasks receive more of the overhead time.

**Acceptance Criteria**:

- Given I have a mergeable task with proportional distribution
- And I have direct tasks with durations of 2h, 1h, and 1h
- When the ActiTime-tuned view is generated
- Then the mergeable time is split 50%, 25%, 25% respectively

### Story TE-06: Even Time Distribution

**Priority**: Medium

As a Time Tracker, I want mergeable task time distributed evenly to direct tasks so that each task receives an equal share.

**Acceptance Criteria**:

- Given I have a mergeable task with even distribution
- And I have 3 direct tasks
- When the ActiTime-tuned view is generated
- Then each direct task receives 1/3 of the mergeable time

### Story TE-07: Manual Time Distribution

**Priority**: Medium

As a Time Tracker, I want to manually specify which direct tasks receive mergeable time so that I have full control over distribution.

**Acceptance Criteria**:

- Given I have a mergeable task with manual distribution
- When I select specific direct tasks to receive the time
- Then only those tasks receive the distributed time
- And I can specify the proportion for each

### Story TE-08: Weighted Time Distribution

**Priority**: Low

As a Time Tracker, I want to assign weights to direct tasks for distribution so that I can customize how overhead is allocated.

**Acceptance Criteria**:

- Given I have a mergeable task with weighted distribution
- And I have assigned weights to direct tasks
- When the ActiTime-tuned view is generated
- Then time is distributed according to the weights

### Story TE-09: Default Distribution Strategy per Task Type

**Priority**: Medium

As a Time Tracker, I want to set a default distribution strategy for task types so that common overhead tasks automatically use my preferred method.

**Acceptance Criteria**:

- Given I have defined a task type with a default distribution strategy
- When I create a new mergeable task of that type
- Then the default strategy is pre-selected
- And I can override it if needed

---

## Epic: Category Management (FR-02)

### Story CM-01: Hierarchical Category Picker

**Priority**: High

As a Time Tracker, I want to select categories from a hierarchical picker so that I can quickly find the right ActiTime category.

**Acceptance Criteria**:

- Given I am assigning a category to a task
- When I open the category picker
- Then I see categories organized hierarchically (e.g., Overhead > People Management > People Care)
- And I can drill down through levels

### Story CM-02: Add New Category

**Priority**: Medium

As a Time Tracker, I want to add new categories so that I can match new ActiTime categories as they're created.

**Acceptance Criteria**:

- Given I am in category management
- When I add a new category with a hierarchical path
- Then the category is available in the picker
- And it appears in the correct position in the hierarchy

### Story CM-03: Hide/Archive Category

**Priority**: Medium

As a Time Tracker, I want to hide categories so that obsolete categories don't clutter the picker.

**Acceptance Criteria**:

- Given I have a category that's no longer used
- When I hide/archive the category
- Then it no longer appears in the category picker for new tasks
- And it still appears on historical tasks that used it

---

## Epic: Common Tasks/Templates (FR-03)

### Story CT-01: Create Task Template

**Priority**: High

As a Time Tracker, I want to create task templates so that I can quickly add recurring tasks.

**Acceptance Criteria**:

- Given I am in template management
- When I create a template with name, default duration, category, and mergeable flag
- Then the template is saved
- And it appears in my template list

### Story CT-02: Quick-Add from Template

**Priority**: High

As a Time Tracker, I want to add a task from a template so that I don't have to re-enter common task details.

**Acceptance Criteria**:

- Given I have saved templates
- When I select a template to add
- Then a new task is created with the template's default values
- And I can modify any values before saving

### Story CT-03: Edit Task Template

**Priority**: Medium

As a Time Tracker, I want to edit existing templates so that I can update defaults as my work changes.

**Acceptance Criteria**:

- Given I have an existing template
- When I edit the template's properties
- Then the changes are saved
- And future tasks from this template use the new defaults

### Story CT-04: Delete Task Template

**Priority**: Low

As a Time Tracker, I want to delete templates I no longer use so that my template list stays manageable.

**Acceptance Criteria**:

- Given I have a template I no longer need
- When I delete the template
- Then it is removed from my template list
- And existing tasks created from it are not affected

---

## Epic: Time Views (FR-04)

### Story TV-01: Full View of All Tasks

**Priority**: High

As a Time Tracker, I want to see all tasks I've logged so that I have a complete picture of my day.

**Acceptance Criteria**:

- Given I have logged tasks for today
- When I view the full view
- Then I see all tasks (both direct and mergeable)
- And I see the actual time spent on each

### Story TV-02: ActiTime-Tuned View

**Priority**: High

As a Time Tracker, I want to see an ActiTime-ready view so that I can easily copy my time to ActiTime.

**Acceptance Criteria**:

- Given I have logged tasks with merged time calculated
- When I view the ActiTime-tuned view
- Then I see only direct tasks
- And each task shows total time including distributed mergeable time
- And the format matches ActiTime's input format

### Story TV-03: Calendar Day Navigation

**Priority**: Medium

As a Time Tracker, I want to navigate between days using a calendar so that I can view or edit past entries.

**Acceptance Criteria**:

- Given I am viewing time entries
- When I select a different date from the calendar
- Then I see the tasks for that date
- And I can add or edit tasks for that date

### Story TV-04: Week View

**Priority**: Medium

As a Time Tracker, I want to see a week view so that I can see my time across multiple days at once.

**Acceptance Criteria**:

- Given I am viewing time entries
- When I switch to week view
- Then I see all days of the current week
- And I can see task summaries for each day

---

## Epic: Data Storage & Export (FR-05)

### Story DS-01: Local YAML Storage

**Priority**: High

As a Time Tracker, I want my data stored locally as YAML files so that I can inspect and backup my data easily.

**Acceptance Criteria**:

- Given I have logged tasks
- When the app saves data
- Then data is stored as human-readable YAML files
- And I can find the files on my local disk

### Story DS-02: Export Data for Backup

**Priority**: Medium

As a Time Tracker, I want to export my data so that I can create backups.

**Acceptance Criteria**:

- Given I have data in the app
- When I export data
- Then a backup file is created
- And it contains all my tasks, categories, and templates

### Story DS-03: Import Data from Backup

**Priority**: Medium

As a Time Tracker, I want to import data from a backup so that I can restore my data if needed.

**Acceptance Criteria**:

- Given I have a backup file
- When I import the backup
- Then my data is restored
- And I can see my tasks, categories, and templates

### Story DS-04: Export to CSV/Excel

**Priority**: Medium

As a Time Tracker, I want to export my time data to CSV so that I can analyze it in spreadsheet software.

**Acceptance Criteria**:

- Given I have time entries
- When I export to CSV
- Then a CSV file is created with my time data
- And it can be opened in Excel or similar software

---

## Epic: Jira Integration (FR-06) - Phase 2

### Story JI-01: Connect to Jira

**Priority**: Low

As a Time Tracker, I want to connect to my Jira instance so that I can pull ticket information.

**Acceptance Criteria**:

- Given I have Jira credentials
- When I configure the Jira connection
- Then the app connects to my Jira instance
- And I can verify the connection is working

### Story JI-02: Pull Assigned Tickets

**Priority**: Low

As a Time Tracker, I want to see my assigned Jira tickets so that I can select them as tasks.

**Acceptance Criteria**:

- Given I am connected to Jira
- When I request my tickets
- Then I see a list of tickets assigned to me
- And I can select a ticket to create a task

---

## Epic: UI/Theme (NFR-02)

### Story UI-01: System Theme Default

**Priority**: Medium

As a Time Tracker, I want the app to follow my system theme by default so that it matches my other apps.

**Acceptance Criteria**:

- Given I have not set a theme preference
- When I open the app
- Then it uses my system's light/dark theme setting

### Story UI-02: First-Run Theme Prompt

**Priority**: Low

As a Time Tracker, I want to be asked my theme preference on first launch so that I can customize immediately.

**Acceptance Criteria**:

- Given this is my first time opening the app
- When the app launches
- Then I am prompted to choose light, dark, or system theme
- And my choice is saved

### Story UI-03: Change Theme Setting

**Priority**: Low

As a Time Tracker, I want to change my theme setting so that I can switch between light and dark modes.

**Acceptance Criteria**:

- Given I am in settings
- When I change the theme setting
- Then the app immediately updates to the new theme
- And my preference is saved

---

## Story Summary

| Epic | Stories | Priority Distribution |
|------|---------|----------------------|
| Time Entry | 9 | 4 High, 3 Medium, 2 Low |
| Category Management | 3 | 1 High, 2 Medium |
| Common Tasks | 4 | 2 High, 1 Medium, 1 Low |
| Time Views | 4 | 2 High, 2 Medium |
| Data Storage | 4 | 1 High, 3 Medium |
| Jira Integration | 2 | 2 Low |
| UI/Theme | 3 | 1 Medium, 2 Low |
| **Total** | **29** | **10 High, 12 Medium, 7 Low** |
