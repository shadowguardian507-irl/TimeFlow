<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { timerStore } from '../stores/timer';
  import type { TaskType, DistributionStrategy } from '../api/types';

  export let currentDate: string;

  const dispatch = createEventDispatcher<{ taskCreated: void }>();

  let taskName = '';
  let categoryPath = '';
  let taskType: TaskType = 'direct';
  let distributionStrategy: DistributionStrategy = 'proportional';

  $: timerState = $timerStore;
  $: isRunning = timerState?.is_running ?? false;
  $: elapsedSeconds = timerState?.elapsed_seconds ?? 0;

  function formatTime(seconds: number): string {
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    return `${hrs.toString().padStart(2, '0')}:${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }

  async function handleStart() {
    if (!taskName.trim()) {
      alert('Please enter a task name');
      return;
    }

    const strategy = taskType === 'mergeable' ? distributionStrategy : undefined;
    await timerStore.start(
      taskName,
      categoryPath || null,
      taskType,
      strategy
    );
  }

  async function handleStop() {
    await timerStore.stop();
    taskName = '';
    categoryPath = '';
    taskType = 'direct';
    dispatch('taskCreated');
  }

  async function handleDiscard() {
    if (confirm('Discard this timer without saving?')) {
      await timerStore.discard();
      taskName = '';
      categoryPath = '';
      taskType = 'direct';
    }
  }
</script>

<div class="timer-widget" data-testid="timer-widget">
  <div class="timer-display">
    <span class="time" class:running={isRunning}>
      {formatTime(elapsedSeconds)}
    </span>
    {#if isRunning}
      <span class="task-name">{timerState?.task_name}</span>
    {/if}
  </div>

  {#if !isRunning}
    <div class="timer-form">
      <input
        type="text"
        placeholder="Task name"
        bind:value={taskName}
        data-testid="timer-task-name"
      />

      <input
        type="text"
        placeholder="Category (optional)"
        bind:value={categoryPath}
        data-testid="timer-category"
      />

      <div class="type-selector">
        <label>
          <input
            type="radio"
            bind:group={taskType}
            value="direct"
            data-testid="timer-type-direct"
          />
          Direct
        </label>
        <label>
          <input
            type="radio"
            bind:group={taskType}
            value="mergeable"
            data-testid="timer-type-mergeable"
          />
          Mergeable
        </label>
      </div>

      {#if taskType === 'mergeable'}
        <select
          bind:value={distributionStrategy}
          data-testid="timer-distribution"
        >
          <option value="proportional">Proportional</option>
          <option value="even">Even</option>
          <option value="manual">Manual</option>
          <option value="weighted">Weighted</option>
        </select>
      {/if}
    </div>
  {/if}

  <div class="timer-actions">
    {#if isRunning}
      <button
        class="stop-btn"
        on:click={handleStop}
        data-testid="timer-stop"
      >
        Stop & Save
      </button>
      <button
        class="discard-btn"
        on:click={handleDiscard}
        data-testid="timer-discard"
      >
        Discard
      </button>
    {:else}
      <button
        class="start-btn"
        on:click={handleStart}
        data-testid="timer-start"
      >
        Start Timer
      </button>
    {/if}
  </div>
</div>

<style>
  .timer-widget {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .timer-display {
    text-align: center;
    margin-bottom: 1rem;
  }

  .time {
    font-size: 3rem;
    font-weight: 600;
    font-family: monospace;
    color: var(--text-primary);
  }

  .time.running {
    color: var(--accent-color);
  }

  .task-name {
    display: block;
    margin-top: 0.5rem;
    font-size: 1rem;
    color: var(--text-secondary);
  }

  .timer-form {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .timer-form input[type="text"],
  .timer-form select {
    padding: 0.75rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-size: 0.95rem;
  }

  .type-selector {
    display: flex;
    gap: 1rem;
  }

  .type-selector label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .timer-actions {
    display: flex;
    gap: 0.5rem;
  }

  .start-btn,
  .stop-btn,
  .discard-btn {
    flex: 1;
    padding: 0.75rem;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .start-btn {
    background: var(--accent-color);
    color: white;
  }

  .stop-btn {
    background: #4caf50;
    color: white;
  }

  .discard-btn {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    color: var(--text-secondary);
  }

  .start-btn:hover,
  .stop-btn:hover {
    opacity: 0.9;
  }

  .discard-btn:hover {
    background: var(--bg-hover);
  }
</style>
