<script lang="ts">
  import { settings } from '../stores/settings';
  import type { Task } from '../api/types';

  export let tasks: Task[] = [];

  $: totalMinutes = tasks.reduce((sum, t) => sum + t.duration_minutes, 0);
  $: directMinutes = tasks
    .filter(t => t.task_type === 'direct')
    .reduce((sum, t) => sum + t.duration_minutes, 0);
  $: mergeableMinutes = tasks
    .filter(t => t.task_type === 'mergeable')
    .reduce((sum, t) => sum + t.duration_minutes, 0);

  $: totalHours = totalMinutes / 60;
  $: workDayHours = $settings?.work_day_hours ?? 8;
  $: exceedsWorkDay = totalHours > workDayHours;
  $: exceeds24Hours = totalHours > 24;

  function formatTime(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    if (hours > 0) {
      return `${hours}h ${mins}m`;
    }
    return `${mins}m`;
  }
</script>

<div class="daily-summary" data-testid="daily-summary">
  <h4>Today's Summary</h4>

  <div class="stat total">
    <span class="label">Total Time</span>
    <span class="value">{formatTime(totalMinutes)}</span>
  </div>

  <div class="breakdown">
    <div class="stat">
      <span class="label">Direct</span>
      <span class="value direct">{formatTime(directMinutes)}</span>
    </div>
    <div class="stat">
      <span class="label">Mergeable</span>
      <span class="value mergeable">{formatTime(mergeableMinutes)}</span>
    </div>
  </div>

  {#if exceeds24Hours}
    <div class="alert error" role="alert">
      ⛔ Total exceeds 24 hours!
    </div>
  {:else if exceedsWorkDay}
    <div class="alert warning" role="alert">
      ⚠️ Exceeds {workDayHours}h work day
    </div>
  {/if}
</div>

<style>
  .daily-summary {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 1rem;
  }

  h4 {
    margin: 0 0 1rem;
    font-size: 0.9rem;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .stat {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
  }

  .stat.total {
    border-bottom: 1px solid var(--border-color);
    margin-bottom: 0.5rem;
  }

  .label {
    color: var(--text-secondary);
    font-size: 0.875rem;
  }

  .value {
    font-weight: 600;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .value.direct {
    color: var(--success-color);
  }

  .value.mergeable {
    color: var(--warning-color);
  }

  .breakdown {
    display: flex;
    flex-direction: column;
  }

  .breakdown .stat {
    padding: 0.25rem 0;
  }

  .breakdown .value {
    font-size: 0.9rem;
  }

  .alert {
    margin-top: 1rem;
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 500;
  }

  .alert.warning {
    background: var(--warning-bg);
    color: var(--warning-color);
  }

  .alert.error {
    background: var(--error-bg);
    color: var(--error-color);
  }
</style>
