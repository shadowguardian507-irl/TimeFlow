<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { TimerInfo } from '../api/types';

  export let timerState: TimerInfo;

  const dispatch = createEventDispatcher<{
    save: void;
    discard: void;
    cancel: void;
  }>();

  function formatElapsed(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    const parts = [];
    if (hours > 0) parts.push(`${hours}h`);
    if (mins > 0) parts.push(`${mins}m`);
    parts.push(`${secs}s`);
    
    return parts.join(' ');
  }
</script>

<div class="dialog-overlay" data-testid="close-confirm-dialog">
  <div class="dialog">
    <div class="warning-icon">⚠️</div>
    <h2>Timer Running</h2>
    <p>You have an active timer that hasn't been saved.</p>

    <div class="timer-info">
      <div class="info-row">
        <span class="label">Task:</span>
        <span class="value">{timerState.task_name}</span>
      </div>
      <div class="info-row">
        <span class="label">Elapsed:</span>
        <span class="value time">{formatElapsed(timerState.elapsed_seconds)}</span>
      </div>
      {#if timerState.category_path}
        <div class="info-row">
          <span class="label">Category:</span>
          <span class="value">{timerState.category_path}</span>
        </div>
      {/if}
    </div>

    <p class="question">What would you like to do?</p>

    <div class="actions">
      <button 
        class="save-btn"
        on:click={() => dispatch('save')}
        data-testid="close-save-btn"
      >
        💾 Save & Close
      </button>
      <button 
        class="discard-btn"
        on:click={() => dispatch('discard')}
        data-testid="close-discard-btn"
      >
        🗑️ Discard & Close
      </button>
      <button 
        class="cancel-btn"
        on:click={() => dispatch('cancel')}
        data-testid="close-cancel-btn"
      >
        ← Keep Working
      </button>
    </div>
  </div>
</div>

<style>
  .dialog-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 300;
  }

  .dialog {
    background: var(--bg-primary);
    border-radius: 16px;
    padding: 2rem;
    width: 100%;
    max-width: 400px;
    text-align: center;
  }

  .warning-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  h2 {
    margin: 0 0 0.5rem;
    color: var(--text-primary);
  }

  p {
    color: var(--text-secondary);
    margin: 0 0 1rem;
  }

  .timer-info {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 1rem;
    margin: 1rem 0;
    text-align: left;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    padding: 0.25rem 0;
  }

  .info-row .label {
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .info-row .value {
    color: var(--text-primary);
    font-weight: 500;
  }

  .info-row .value.time {
    color: var(--accent-color);
    font-family: monospace;
  }

  .question {
    font-weight: 500;
    color: var(--text-primary);
    margin-top: 1rem !important;
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .save-btn, .discard-btn, .cancel-btn {
    padding: 0.75rem 1rem;
    border-radius: 8px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .save-btn {
    background: var(--success-color);
    color: white;
    border: none;
  }

  .discard-btn {
    background: var(--error-color);
    color: white;
    border: none;
  }

  .cancel-btn {
    background: var(--bg-secondary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
  }

  .save-btn:hover, .discard-btn:hover, .cancel-btn:hover {
    opacity: 0.9;
  }
</style>
