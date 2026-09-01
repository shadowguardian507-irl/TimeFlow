# Functional Design Plan - TimeFlow

## Overview

This plan outlines the detailed business logic design for the TimeFlow application.

---

## Design Questions

Please answer the following questions to guide the functional design.

### Question 1: Time Rounding

Should task durations be rounded when displayed or exported?

A) No rounding - show exact minutes
B) Round to nearest 5 minutes
C) Round to nearest 15 minutes
D) User-configurable rounding preference
E) Other (please describe after [Answer] tag below)

[Answer] A

---

### Question 2: Minimum Task Duration

What is the minimum duration for a task entry?

A) 1 minute (no minimum)
B) 5 minutes minimum
C) 15 minutes minimum
D) User-configurable minimum
E) Other (please describe after [Answer] tag below)

[Answer] D
default is 1 minute

---

### Question 3: Daily Time Validation

Should the app validate that daily time totals make sense?

A) No validation - allow any total
B) Warn if total exceeds 24 hours
C) Warn if total exceeds configurable work day (e.g., 8-10 hours)
D) Other (please describe after [Answer] tag below)

[Answer] C

---

### Question 4: Handling Zero Direct Tasks

When generating ActiTime view, what happens if there are mergeable tasks but no direct tasks?

A) Show error - require at least one direct task
B) Show warning but allow viewing (mergeable time shown as unallocated)
C) Automatically create an "Unallocated" direct task to receive the time
D) Other (please describe after [Answer] tag below)

[Answer] A

---

### Question 5: Timer Behavior on App Close

What should happen if the app is closed while a timer is running?

A) Auto-stop timer and save the task
B) Pause timer and resume on next app open
C) Discard timer (user must manually save before closing)
D) Prompt user to save or discard before closing
E) Other (please describe after [Answer] tag below)

[Answer] D

---

### Question 6: Category Path Separator

What character should separate category hierarchy levels?

A) Forward slash: "Overhead/People Management/People Care"
B) Greater than: "Overhead > People Management > People Care"
C) Dot: "Overhead.People Management.People Care"
D) Other (please describe after [Answer] tag below)

[Answer] B

---

## Design Steps

Once questions are answered, the following steps will be executed:

- [x] Step 1: Define domain entities (Task, Category, Template, TimerState, etc.)
- [x] Step 2: Define business rules for task management
- [x] Step 3: Define business rules for time merging/distribution
- [x] Step 4: Define business rules for category management
- [x] Step 5: Define business rules for template management
- [x] Step 6: Define view generation logic
- [x] Step 7: Define export logic
- [x] Step 8: Create business-logic-model.md
- [x] Step 9: Create business-rules.md
- [x] Step 10: Create domain-entities.md
- [x] Step 11: Create frontend-components.md

---

## Mandatory Artifacts

The following artifacts will be generated:

- `aidlc-docs/construction/timeflow/functional-design/business-logic-model.md`
- `aidlc-docs/construction/timeflow/functional-design/business-rules.md`
- `aidlc-docs/construction/timeflow/functional-design/domain-entities.md`
- `aidlc-docs/construction/timeflow/functional-design/frontend-components.md`
