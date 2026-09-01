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

```
timeflow/
├── src/                    # Frontend (Svelte + TypeScript)
│   ├── lib/
│   │   ├── api/           # Tauri command wrappers
│   │   ├── components/    # UI components
│   │   └── stores/        # Svelte stores
│   └── styles/            # Global CSS
├── src-tauri/             # Backend (Rust)
│   └── src/
│       ├── commands/      # Tauri command handlers
│       ├── models/        # Domain entities
│       └── services/      # Business logic
└── aidlc-docs/            # Design documentation
```

## Data Storage

TimeFlow stores data in YAML files in the user's data directory:
- `~/.timeflow/tasks/YYYY-MM-DD.yaml` - Daily task files
- `~/.timeflow/categories.yaml` - Category hierarchy
- `~/.timeflow/templates.yaml` - Task templates
- `~/.timeflow/settings.yaml` - User preferences

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

## License

MIT
