<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import * as api from '../api/commands';
  import type { WeekView as WeekViewType, DaySummary } from '../api/types';

  export let currentDate: string;

  const dispatch = createEventDispatcher<{ dateChange: string }>();

  let view: WeekViewType | null = null;
  let loading = true;

  $: weekStart = getWeekStart(currentDate);
  $: {
    loadView(weekStart);
  }

  function getWeekStart(date: string): string {
    const d = new Date(date);
    const day = d.getDay();
    const diff = d.getDate() - day + (day === 0 ? -6 : 1);
    d.setDate(diff);
    return d.toISOString().split('T')[0];
  }

  async function loadView(start: string) {
    loading = true;
    try {
      view = await api.getWeekView(start);
    } catch (e) {
      console.error('Failed to load week view:', e);
    } finally {
      loading = false;
    }
  }

  function formatDuration(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    if (hours > 0) {
      return `${hours}h ${mins}m`;
    }
    return `${mins}m`;
  }

  function formatDayName(dateStr: string): string {
    const date = new Date(dateStr);
    return date.toLocaleDateString('en-US', { weekday: 'short' });
  }

  function formatDayNumber(dateStr: string): string {
    const date = new Date(dateStr);
    return date.getDate().toString();
  }

  function isToday(dateStr: string): boolean {
    return dateStr === new Date().toISOString().split('T')[0];
  }

  function isSelected(dateStr: string): boolean {
    return dateStr === currentDate;
  }

  function selectDay(dateStr: string) {
    dispatch('dateChange', dateStr);
  }

  function previousWeek() {
    const d = new Date(weekStart);
    d.setDate(d.getDate() - 7);
    dispatch('dateChange', d.toISOString().split('T')[0]);
  }

  function nextWeek() {
    const d = new Date(weekStart);
    d.setDate(d.getDate() + 7);
    dispatch('dateChange', d.toISOString().split('T')[0]);
  }
</script>

<div class="week-view" data-testid="week-view">
  <div class="view-header">
    <h2>Week View</h2>
    <div class="week-nav">
      <button on:click={previousWeek} aria-label="Previous week">←</button>
      <span class="week-label">
        {#if view}
          Week of {new Date(view.week_start).toLocaleDateString('en-US', { month: 'short', day: 'numeric' })}
        {/if}
      </span>
      <button on:click={nextWeek} aria-label="Next week">→</button>
    </div>
  </div>

  {#if loading}
    <div class="loading">Loading...</div>
  {:else if view}
    <div class="week-grid">
      {#each view.days as day}
        <button 
          class="day-card"
          class:today={isToday(day.date)}
          class:selected={isSelected(day.date)}
          on:click={() => selectDay(day.date)}
          data-testid="week-day-{day.date}"
        >
          <span class="day-name">{formatDayName(day.date)}</span>
          <span class="day-number">{formatDayNumber(day.date)}</span>
          <span class="day-time">{formatDuration(day.total_minutes)}</span>
          <span class="task-count">{day.task_count} tasks</span>
        </button>
      {/each}
    </div>

    <div class="week-summary">
      <div class="summary-item">
        <span class="label">Week Total</span>
        <span class="value">{formatDuration(view.total_minutes)}</span>
      </div>
    </div>
  {/if}
</div>

<style>
  .week-view {
    max-width: 900px;
  }

  .view-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  .view-header h2 {
    margin: 0;
    color: var(--text-primary);
  }

  .week-nav {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .week-nav button {
    padding: 0.5rem 0.75rem;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    cursor: pointer;
    color: var(--text-primary);
  }

  .week-nav button:hover {
    background: var(--bg-hover);
  }

  .week-label {
    font-weight: 500;
    color: var(--text-primary);
  }

  .loading {
    text-align: center;
    padding: 3rem;
    color: var(--text-secondary);
  }

  .week-grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 0.75rem;
  }

  .day-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 1rem;
    background: var(--bg-secondary);
    border: 2px solid transparent;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .day-card:hover {
    background: var(--bg-hover);
  }

  .day-card.today {
    border-color: var(--accent-color);
  }

  .day-card.selected {
    background: var(--accent-bg);
    border-color: var(--accent-color);
  }

  .day-name {
    font-size: 0.75rem;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .day-number {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0.25rem 0;
  }

  .day-time {
    font-size: 0.9rem;
    font-weight: 500;
    color: var(--accent-color);
  }

  .task-count {
    font-size: 0.75rem;
    color: var(--text-secondary);
    margin-top: 0.25rem;
  }

  .week-summary {
    margin-top: 1.5rem;
    padding: 1rem;
    background: var(--bg-secondary);
    border-radius: 8px;
    display: flex;
    justify-content: flex-end;
  }

  .summary-item {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .summary-item .label {
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .summary-item .value {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--accent-color);
  }
</style>
