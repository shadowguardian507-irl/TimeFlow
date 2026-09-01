# Project Agent Instructions

This repository uses AWS AIDLC-style workflow rules stored under `.ai-rules/`.

## Mandatory Rule Loading

Before performing software development work in this repository:

1. Read and follow `.ai-rules/core-workflow-load.md`.
2. Read and follow `.ai-rules/aidlc-workflow-load.md`.
3. Load any additional rule-detail files required by those workflow loader files before executing the relevant workflow phase.

## Rule Priority

Rules loaded from `.ai-rules/core-workflow-load.md` have the highest repository-level priority.
Rules loaded from `.ai-rules/aidlc-workflow-load.md` are next and apply to all software development requests unless a higher-priority instruction conflicts.

When a loader file references relative paths, resolve them from the repository root.
