<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import TaskItem from './TaskItem.svelte';
  import type { Task } from '../api/types';

  export let tasks: Task[] = [];

  const dispatch = createEventDispatcher<{
    edit: Task;
    delete: Task;
  }>();
</script>

<div class="task-list" data-testid="task-list">
  {#if tasks.length === 0}
    <div class="empty-state" data-testid="task-list-empty">
      <p>No tasks for this day</p>
      <p class="hint">Click "Add Task" to get started</p>
    </div>
  {:else}
    {#each tasks as task (task.id)}
      <TaskItem
        {task}
        on:edit={() => dispatch('edit', task)}
        on:delete={() => dispatch('delete', task)}
      />
    {/each}
  {/if}
</div>

<style>
  .task-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: var(--text-secondary);
  }

  .empty-state p {
    margin: 0;
  }

  .hint {
    font-size: 0.875rem;
    margin-top: 0.5rem !important;
    opacity: 0.7;
  }
</style>
