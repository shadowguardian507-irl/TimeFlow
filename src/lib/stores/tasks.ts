import { writable, derived } from 'svelte/store';
import type { Task, TaskType, DistributionStrategy, TaskUpdate } from '../api/types';
import * as api from '../api/commands';

export const currentDate = writable<string>(new Date().toISOString().split('T')[0]);

// Single source of truth for tasks
const _tasks = writable<Task[]>([]);
export const tasks = { subscribe: _tasks.subscribe };

function createTasksStore() {
  return {
    subscribe: _tasks.subscribe,
    async loadForDate(date: string) {
      try {
        const loadedTasks = await api.getTasksForDate(date);
        _tasks.set(loadedTasks);
        return loadedTasks;
      } catch (e) {
        console.error('Failed to load tasks:', e);
        _tasks.set([]);
        return [];
      }
    },
    async create(
      name: string,
      durationMinutes: number,
      categoryPath: string,
      taskType: TaskType,
      distributionStrategy?: DistributionStrategy,
      date?: string
    ) {
      const task = await api.createTask(
        name,
        durationMinutes,
        categoryPath,
        taskType,
        distributionStrategy,
        date
      );
      _tasks.update((tasks) => [...tasks, task]);
      return task;
    },
    async update(taskId: string, date: string, updates: TaskUpdate) {
      const updatedTask = await api.updateTask(taskId, date, updates);
      _tasks.update((tasks) =>
        tasks.map((t) => (t.id === taskId ? updatedTask : t))
      );
      return updatedTask;
    },
    async delete(taskId: string, date: string) {
      await api.deleteTask(taskId, date);
      _tasks.update((tasks) => tasks.filter((t) => t.id !== taskId));
    },
    async validateDaily(date: string) {
      return api.validateDailyTime(date);
    },
  };
}

export const tasksStore = createTasksStore();

// Derived stores - use _tasks directly
export const directTasks = derived(_tasks, ($tasks) =>
  $tasks.filter((t) => t.task_type === 'direct')
);

export const mergeableTasks = derived(_tasks, ($tasks) =>
  $tasks.filter((t) => t.task_type === 'mergeable')
);

export const totalMinutes = derived(_tasks, ($tasks) =>
  $tasks.reduce((sum, t) => sum + t.duration_minutes, 0)
);
