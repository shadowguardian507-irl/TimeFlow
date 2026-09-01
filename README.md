# TimeFlow

A desktop time tracking application built with Rust + Tauri for macOS, designed for seamless ActiTime integration.

## Features

- **Time Entry**: Manual and timer-based time tracking
- **Task Types**: Direct tasks (ActiTime-ready) and mergeable tasks (distributed to direct tasks)
- **Categories**: Hierarchical category management matching ActiTime structure
- **Templates**: Create templates for recurring tasks
- **Views**: Full view, ActiTime view (ready to copy), and week overview
- **Distribution Strategies**: Proportional, even, manual, or weighted time distribution
- **Data Export**: YAML backup and CSV export
- **Themes**: Light, dark, and system-following themes

## Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Node.js](https://nodejs.org/) 18+
- [pnpm](https://pnpm.io/) (recommended) or npm

## Development Setup

1. Install dependencies:

   ```bash
   pnpm install
   pnpm approve-builds # if needed
   # approve esbuild@0.21.5
   ```

2. Run in development mode:

   ```bash
   pnpm tauri dev
   ```

3. Build for production:

   ```bash
   pnpm tauri build
   ```

## Project Structure

```text
timeflow/
|-- AGENTS.md              # Agent bootstrap instructions
|-- CLAUDE.md              # Claude instruction passthrough to AGENTS.md
|-- README.md              # Project overview and setup guide
|-- package.json           # Frontend scripts and dependencies
|-- package-lock.json      # npm dependency lockfile
|-- index.html             # Vite entry document
|-- vite.config.ts         # Vite configuration
|-- svelte.config.js       # Svelte configuration
|-- tsconfig*.json         # TypeScript configuration
|-- .ai-rules/             # Core and AIDLC workflow rules
|   |-- core-workflow-load.md
|   |-- aidlc-workflow-load.md
|   |-- core-rules/
|   `-- aidlc-rule-details/
|-- aidlc-docs/            # AIDLC project documentation and audit trail
|   `-- 0.1.0/
|       |-- aidlc-state.md
|       |-- audit.md
|       |-- inception/
|       `-- construction/
|-- src/                   # Frontend (Svelte + TypeScript)
|   |-- App.svelte
|   |-- main.ts
|   |-- lib/
|   |   |-- api/           # Tauri command wrappers and shared types
|   |   |-- components/    # UI components
|   |   `-- stores/        # Svelte stores
|   `-- styles/            # Global CSS and design variables
|-- src-tauri/             # Backend (Rust + Tauri)
|   |-- Cargo.toml
|   |-- Cargo.lock
|   |-- tauri.conf.json
|   |-- build.rs
|   |-- icons/
|   |-- gen/schemas/       # Generated Tauri schemas
|   `-- src/
|       |-- commands/      # Tauri command handlers
|       |-- models/        # Domain entities
|       `-- services/      # Business logic and persistence
|-- .cspell/               # Spell-check word lists
|-- .cspell.yaml           # Spell-check configuration
|-- .vscode/               # Editor settings
`-- dist/                  # Generated frontend build output
```

## Data Storage

TimeFlow stores data in YAML files in the operating system's application data directory. The current storage namespace
is `uk.etheria-software/TimeFlow`.

When upgrading from an earlier alpha build, TimeFlow checks the legacy `com/timeflow/TimeFlow` namespace and offers to
copy the existing data into the current namespace.

The data includes:

- `tasks/YYYY-MM-DD.yaml` - Daily task files
- `categories.yaml` - Category hierarchy
- `templates.yaml` - Task templates
- `settings.yaml` - User preferences

## Usage

### Adding Tasks

1. Use the timer widget to track time in real-time
2. Or click "Add Task" to manually enter a task with duration

### Task Types

- **Direct**: Tasks that map directly to ActiTime entries
- **Mergeable**: Tasks whose time gets distributed to direct tasks

### Distribution Strategies

For mergeable tasks, choose how time is distributed:

- **Proportional**: Based on direct task durations
- **Even**: Split equally among direct tasks
- **Manual**: Specify exact percentages
- **Weighted**: Assign weights to direct tasks

### ActiTime View

The ActiTime view shows merged entries ready to copy to ActiTime, with category paths and total durations.

## Licence

[MIT Licence](LICENSE.md)
