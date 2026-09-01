import { invoke } from '@tauri-apps/api/core';
import type {
  Task,
  TaskType,
  TaskUpdate,
  DistributionStrategy,
  Category,
  Template,
  TemplateUpdate,
  TaskOverrides,
  TimerInfo,
  Settings,
  SettingsUpdate,
  FullView,
  ActiTimeView,
  WeekView,
  DailyValidation,
  ImportResult,
} from './types';

// Task commands
export async function createTask(
  name: string,
  durationMinutes: number,
  categoryPath: string,
  taskType: TaskType,
  distributionStrategy?: DistributionStrategy,
  date?: string
): Promise<Task> {
  return invoke('create_task', {
    name,
    durationMinutes,
    categoryPath,
    taskType,
    distributionStrategy,
    date,
  });
}

export async function updateTask(
  taskId: string,
  date: string,
  updates: TaskUpdate
): Promise<Task> {
  return invoke('update_task', { taskId, date, updates });
}

export async function deleteTask(taskId: string, date: string): Promise<void> {
  return invoke('delete_task', { taskId, date });
}

export async function getTasksForDate(date: string): Promise<Task[]> {
  return invoke('get_tasks_for_date', { date });
}

export async function getTasksForRange(start: string, end: string): Promise<Task[]> {
  return invoke('get_tasks_for_range', { start, end });
}

export async function validateDailyTime(date: string): Promise<DailyValidation> {
  return invoke('validate_daily_time', { date });
}

// Timer commands
export async function startTimer(
  taskName: string,
  categoryPath: string | null,
  taskType: TaskType,
  distributionStrategy?: DistributionStrategy
): Promise<TimerInfo> {
  return invoke('start_timer', {
    taskName,
    categoryPath,
    taskType,
    distributionStrategy,
  });
}

export async function stopTimer(): Promise<Task> {
  return invoke('stop_timer');
}

export async function getTimerState(): Promise<TimerInfo | null> {
  return invoke('get_timer_state');
}

export async function discardTimer(): Promise<void> {
  return invoke('discard_timer');
}

export async function isTimerRunning(): Promise<boolean> {
  return invoke('is_timer_running');
}

// Category commands
export async function getCategoryTree(): Promise<Category> {
  return invoke('get_category_tree');
}

export async function getVisibleCategoryPaths(): Promise<string[]> {
  return invoke('get_visible_category_paths');
}

export async function addCategory(path: string): Promise<Category> {
  return invoke('add_category', { path });
}

export async function hideCategory(path: string): Promise<void> {
  return invoke('hide_category', { path });
}

export async function unhideCategory(path: string): Promise<void> {
  return invoke('unhide_category', { path });
}

export async function validateCategoryPath(path: string): Promise<boolean> {
  return invoke('validate_category_path', { path });
}

// Template commands
export async function createTemplate(
  name: string,
  defaultDuration: number,
  categoryPath: string,
  isMergeable: boolean,
  distributionStrategy?: DistributionStrategy
): Promise<Template> {
  return invoke('create_template', {
    name,
    defaultDuration,
    categoryPath,
    isMergeable,
    distributionStrategy,
  });
}

export async function updateTemplate(
  templateId: string,
  updates: TemplateUpdate
): Promise<Template> {
  return invoke('update_template', { templateId, updates });
}

export async function deleteTemplate(templateId: string): Promise<void> {
  return invoke('delete_template', { templateId });
}

export async function getTemplates(): Promise<Template[]> {
  return invoke('get_templates');
}

export async function applyTemplate(
  templateId: string,
  overrides?: TaskOverrides
): Promise<Task> {
  return invoke('apply_template', { templateId, overrides });
}

// View commands
export async function getFullView(date: string): Promise<FullView> {
  return invoke('get_full_view', { date });
}

export async function getActiTimeView(date: string): Promise<ActiTimeView> {
  return invoke('get_actitime_view', { date });
}

export async function getWeekView(weekStart: string): Promise<WeekView> {
  return invoke('get_week_view', { weekStart });
}

// Settings commands
export async function getSettings(): Promise<Settings> {
  return invoke('get_settings');
}

export async function updateSettings(updates: SettingsUpdate): Promise<Settings> {
  return invoke('update_settings', { updates });
}

export async function isFirstRun(): Promise<boolean> {
  return invoke('is_first_run');
}

export async function completeFirstRun(initialSettings: Settings): Promise<Settings> {
  return invoke('complete_first_run', { initialSettings });
}

// Export commands
export async function exportBackup(): Promise<string> {
  // Generate default backup path
  const date = new Date().toISOString().split('T')[0];
  const path = `~/Downloads/timeflow-backup-${date}.yaml`;
  await invoke('export_backup', { path });
  return path;
}

export async function importBackup(path: string): Promise<ImportResult> {
  return invoke('import_backup', { path });
}

export async function exportCsv(start: string, end: string): Promise<string> {
  const path = `~/Downloads/timeflow-export-${start}-to-${end}.csv`;
  await invoke('export_csv', { start, end, path });
  return path;
}
