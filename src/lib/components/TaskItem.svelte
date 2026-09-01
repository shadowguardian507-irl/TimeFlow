<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Task } from '../api/types';

  export let task: Task;

  const dispatch = createEventDispatcher();

  function formatDuration(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    if (hours > 0) {
      return `${hours}h ${mins}m`;
    }
    return `${mins}m`;
  }

  function truncatePath(path: string, maxLen = 40): string {
    if (path.length <= maxLen) return path;
    const parts = path.split(' > ');
    if (parts.length <= 2) return path;
    return `${parts[0]} > ... > ${parts[parts.length - 1]}`;
  }
</script>

<div class="task-item" data-testid="task-item" data-task-id={task.id}>
  <div class="task-main">
    <span class="task-name">{task.name}</span>
    <span class="task-type {task.task_type}" title={task.task_type}>
      {task.task_type === 'direct' ? 'D' : 'M'}
    </span>
  </div>

  <div class="task-meta">
    <span class="category" title={task.category_path}>
      {truncatePath(task.category_path)}
    </span>
    <span class="duration">{formatDuration(task.duration_minutes)}</span>
  </div>

  <div class="task-actions">
    <button
      class="edit-btn"
      on:click={() => dispatch('edit')}
      data-testid="task-edit-btn"
      aria-label="Edit task"
    >
      ✏️
    </button>
    <button
      class="delete-btn"
      on:click={() => dispatch('delete')}
      data-testid="task-delete-btn"
      aria-label="Delete task"
    >
      🗑️
    </button>
  </div>
</div>

<style>
  .task-item {
    display: grid;
    grid-template-columns: 1fr auto auto;
    gap: 1rem;
    align-items: center;
    padding: 0.75rem 1rem;
    background: var(--bg-primary);
    border-radius: 6px;
    border: 1px solid var(--border-color);
  }

  .task-main {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .task-name {
    font-weight: 500;
    color: var(--text-primary);
  }

  .task-type {
    font-size: 0.7rem;
    padding: 0.15rem 0.4rem;
    border-radius: 3px;
    font-weight: 600;
  }

  .task-type.direct {
    background: var(--success-bg);
    color: var(--success-color);
  }

  .task-type.mergeable {
    background: var(--warning-bg);
    color: var(--warning-color);
  }

  .task-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.25rem;
  }

  .category {
    font-size: 0.8rem;
    color: var(--text-secondary);
  }

  .duration {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--accent-color);
  }

  .task-actions {
    display: flex;
    gap: 0.25rem;
  }

  .task-actions button {
    padding: 0.25rem 0.5rem;
    border: none;
    background: transparent;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.15s ease;
  }

  .task-actions button:hover {
    background: var(--bg-hover);
  }
</style>
