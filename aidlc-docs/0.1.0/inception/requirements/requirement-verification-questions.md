# Requirements Clarification Questions

Please answer the following questions to help clarify the requirements for your time management/logging application.

---

## Question 1

What is the primary way you want to track time during the day?

A) Manual entry - I'll enter tasks and durations when I remember
B) Timer-based - Start/stop a timer for each task as I work
C) Both manual entry and timer-based tracking
D) Other (please describe after [Answer] tag below)

[Answer] C

---

## Question 2

How should "mergeable" tasks (like desk setup time) be distributed to "real" tasks?

A) Proportionally based on duration of each real task
B) Evenly split across all real tasks
C) Manually specify which real tasks receive the merged time
D) Weighted distribution (I assign weights to real tasks)
E) Other (please describe after [Answer] tag below)

[Answer] E
having the option to define any of the options per task (A -> D) would be really helpful, especially if you can assign
the behaviour to a type of task as a default ie desk setup gets option A by default but on some occasions I may want to
use option B

---

## Question 3

How do you want to define ActiTime categories?

A) Manually type them in as needed
B) Import from a configuration file (JSON/YAML)
C) Hierarchical picker (e.g., Overhead > People Management > People Care)
D) Both manual entry and hierarchical picker
E) Other (please describe after [Answer] tag below)

[Answer] E
mostly would be option C on a day to day basis, but I would need to have a way of adding categories and hiding them from
being added to future items as time goes on (our actitime admins sometimes add and remove categories ie when a big
project is started and finished)

---

## Question 4

For "common tasks" (repeated tasks), what information should be stored?

A) Just the task name
B) Task name + default duration
C) Task name + default duration + default category
D) Task name + default duration + default category + whether it's mergeable
E) Other (please describe after [Answer] tag below)

[Answer] D

---

## Question 5

How should the "ActiTime-tuned view" present the merged/adjusted times?

A) Simple list with task name, category, and total time
B) Grouped by category with subtotals
C) Table format matching ActiTime's input format
D) Both grouped view and table format
E) Other (please describe after [Answer] tag below)

[Answer] C

---

## Question 6

How do you want to handle multiple days of time tracking?

A) Single day view only - start fresh each day
B) Calendar view to navigate between days
C) Week view showing all days at once
D) Both calendar navigation and week view
E) Other (please describe after [Answer] tag below)

[Answer] D

---

## Question 7

Where should the application store its data?

A) Local file storage only (JSON/SQLite on disk)
B) Cloud sync (specify service if you have a preference)
C) Local with optional export/import for backup
D) Other (please describe after [Answer] tag below)

[Answer] C
local storage should be as yaml files

---

## Question 8

For the Jira/Atlassian integration (bonus feature), what level of integration do you need?

A) Read-only - just pull ticket list to select from
B) Read + update - pull tickets and log time back to Jira
C) Full sync - bidirectional time tracking with Jira
D) Skip this feature for now - focus on core functionality first
E) Other (please describe after [Answer] tag below)

[Answer] A
readonly for now, maybe in future it would be nice to have an option to push the time to the ticket but for now that's
not needed

---

## Question 9

What visual style/theme do you prefer for the application?

A) Native macOS look and feel (follows system theme)
B) Custom dark theme
C) Custom light theme
D) User-selectable theme (light/dark/system)
E) Other (please describe after [Answer] tag below)

[Answer] D
the app should follow the system theme by default 'out of the box' but should let the user choose after that, it would
be nice if it asked the initial preference on first startup

---

## Question 10

How important is keyboard navigation and shortcuts?

A) Essential - I want to do everything without touching the mouse
B) Nice to have - some shortcuts for common actions
C) Not important - mouse/trackpad is fine
D) Other (please describe after [Answer] tag below)

[Answer] C
for now there is no need for shortcuts, maybe we can add some in future but if so there should always be an option for
the user to turn them off completely

---

## Question 11: Security Extensions

Should security extension rules be enforced for this project?

A) Yes — enforce all SECURITY rules as blocking constraints (recommended for production-grade applications)
B) No — skip all SECURITY rules (suitable for PoCs, prototypes, and experimental projects)
C) Other (please describe after [Answer] tag below)

[Answer] B

---

## Question 12

Do you need any reporting or analytics features?

A) No - just the daily time views are sufficient
B) Basic weekly/monthly summaries
C) Detailed reports with charts and trends
D) Export to CSV/Excel for external analysis
E) Other (please describe after [Answer] tag below)

[Answer] D

---
