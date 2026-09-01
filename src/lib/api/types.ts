// Domain types matching Rust backend

export type TaskType = 'direct' | 'mergeable';

export type DistributionStrategy = 'proportional' | 'even' | 'manual' | 'weighted';

export type Theme = 'light' | 'dark' | 'system';

export interface ManualAllocation {
  target_task_id: string;
  percentage: number;
}

export interface WeightedAllocation {
  target_task_id: string;
  weight: number;
}

export interface Task {
  id: string;
  name: string;
  date: string;
  duration_minutes: number;
  category_path: string;
  task_type: TaskType;
  distribution_strategy?: DistributionStrategy;
  manual_allocations: ManualAllocation[];
  weights: WeightedAllocation[];
  created_at: string;
  updated_at: string;
}

export interface TaskUpdate {
  name?: string;
  duration_minutes?: number;
  category_path?: string;
  task_type?: TaskType;
  distribution_strategy?: DistributionStrategy;
  manual_allocations?: ManualAllocation[];
  weights?: WeightedAllocation[];
}

export interface Category {
  name: string;
  hidden: boolean;
  children: Category[];
}

export interface Template {
  id: string;
  name: string;
  default_duration: number;
  category_path: string;
  is_mergeable: boolean;
  distribution_strategy?: DistributionStrategy;
  created_at: string;
}

export interface TemplateUpdate {
  name?: string;
  default_duration?: number;
  category_path?: string;
  is_mergeable?: boolean;
  distribution_strategy?: DistributionStrategy;
}

export interface TaskOverrides {
  name?: string;
  duration_minutes?: number;
  category_path?: string;
  is_mergeable?: boolean;
  distribution_strategy?: DistributionStrategy;
}

export interface TimerInfo {
  task_name: string;
  category_path?: string;
  task_type: TaskType;
  distribution_strategy?: DistributionStrategy;
  elapsed_seconds: number;
  is_running: boolean;
}

export interface Settings {
  theme: Theme;
  first_run_complete: boolean;
  minimum_task_duration: number;
  work_day_hours: number;
  default_distribution_strategy: DistributionStrategy;
}

export interface SettingsUpdate {
  theme?: Theme;
  minimum_task_duration?: number;
  work_day_hours?: number;
  default_distribution_strategy?: DistributionStrategy;
}

export interface FullView {
  date: string;
  tasks: Task[];
  total_minutes: number;
  direct_count: number;
  mergeable_count: number;
}

export interface ActiTimeEntry {
  category_path: string;
  duration_minutes: number;
  task_names: string[];
}

export interface ActiTimeView {
  date: string;
  entries: ActiTimeEntry[];
  total_minutes: number;
}

export interface DaySummary {
  date: string;
  total_minutes: number;
  task_count: number;
}

export interface WeekView {
  week_start: string;
  days: DaySummary[];
  total_minutes: number;
}

export interface DailyValidation {
  total_minutes: number;
  total_hours: number;
  warnings: string[];
  errors: string[];
}

export interface ImportResult {
  categories_imported: boolean;
  templates_count: number;
  settings_imported: boolean;
}
