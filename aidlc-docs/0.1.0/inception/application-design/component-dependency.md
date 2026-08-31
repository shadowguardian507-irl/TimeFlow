# Component Dependencies

## Dependency Matrix

| Component | Depends On |
|-----------|------------|
| TaskManager | StorageService, CategoryManager, TimerService |
| CategoryManager | StorageService |
| TemplateManager | StorageService, CategoryManager |
| ViewGenerator | TaskManager, TimeMergeService |
| ExportManager | StorageService, TaskManager, CategoryManager, TemplateManager |
| SettingsManager | StorageService |
| StorageService | DataIntegrityService |
| TimeMergeService | (none - pure logic) |
| TimerService | (none - self-contained) |
| DataIntegrityService | (none - utility) |

---

## Communication Patterns

### Frontend to Backend

```text
+------------------+          +------------------+
|                  |  Tauri   |                  |
|    Frontend      | Commands |    Managers      |
|    (TypeScript)  +--------->+    (Rust)        |
|                  |          |                  |
+------------------+          +------------------+
```

All frontend-to-backend communication uses Tauri commands:

- Frontend invokes command (e.g., `invoke('create_task', {...})`)
- Backend manager handles command
- Result returned to frontend

### Manager to Service

```text
+------------------+          +------------------+
|                  |  Direct  |                  |
|    Managers      |   Call   |    Services      |
|                  +--------->+                  |
|                  |          |                  |
+------------------+          +------------------+
```

Managers call services directly for:

- Business logic (TimeMergeService)
- Timer operations (TimerService)
- Data integrity (DataIntegrityService)

### Manager to Storage

```text
+------------------+          +------------------+
|                  |  Direct  |                  |
|    Managers      |   Call   |  StorageService  |
|                  +--------->+                  |
|                  |          |                  |
+------------------+          +------------------+
```

All persistence goes through StorageService:

- Managers never access files directly
- StorageService handles YAML serialization
- Atomic writes via DataIntegrityService

---

## Data Flow Diagrams

### Time Entry Flow

```text
User Input --> Frontend --> TaskManager --> StorageService --> YAML File
                              |
                              +--> CategoryManager (validate)
                              |
                              +--> TimerService (if timer-based)
```

### View Generation Flow

```text
Frontend Request --> ViewGenerator --> TaskManager (get tasks)
                          |
                          +--> TimeMergeService (calculate distribution)
                          |
                          +--> Return formatted view
```

### Export Flow

```text
Frontend Request --> ExportManager --> TaskManager (get tasks)
                          |
                          +--> CategoryManager (get categories)
                          |
                          +--> TemplateManager (get templates)
                          |
                          +--> Write to file
```

---

## Initialization Order

Application startup initializes components in this order:

1. DataIntegrityService (utility, no deps)
2. StorageService (depends on DataIntegrityService)
3. SettingsManager (depends on StorageService)
4. CategoryManager (depends on StorageService)
5. TemplateManager (depends on StorageService, CategoryManager)
6. TimerService (no deps, but needs settings)
7. TimeMergeService (no deps)
8. TaskManager (depends on StorageService, CategoryManager, TimerService)
9. ViewGenerator (depends on TaskManager, TimeMergeService)
10. ExportManager (depends on all managers)

---

## Error Propagation

Errors flow upward through the dependency chain:

- StorageService errors → Manager errors → Tauri command errors → Frontend
- All errors are typed and include context for debugging
- Frontend displays user-friendly error messages
