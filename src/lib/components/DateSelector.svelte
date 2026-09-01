<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let value: string;

  const dispatch = createEventDispatcher<{ change: string }>();

  function formatDisplayDate(dateStr: string): string {
    const date = new Date(dateStr + 'T00:00:00');
    return date.toLocaleDateString('en-US', {
      weekday: 'short',
      month: 'short',
      day: 'numeric',
      year: 'numeric',
    });
  }

  function changeDate(days: number) {
    const date = new Date(value + 'T00:00:00Z');
    date.setUTCDate(date.getUTCDate() + days);
    const newDate = date.toISOString().split('T')[0];
    dispatch('change', newDate);
  }

  function goToToday() {
    const today = new Date().toISOString().split('T')[0];
    dispatch('change', today);
  }

  function handleInputChange(event: Event) {
    const input = event.target as HTMLInputElement;
    dispatch('change', input.value);
  }

  $: isToday = value === new Date().toISOString().split('T')[0];
</script>

<div class="date-selector" data-testid="date-selector">
  <button
    class="nav-btn"
    on:click={() => changeDate(-1)}
    data-testid="date-selector-prev"
    aria-label="Previous day"
  >
    ←
  </button>

  <div class="date-display">
    <input
      type="date"
      {value}
      on:change={handleInputChange}
      data-testid="date-selector-input"
    />
    <span class="date-text">{formatDisplayDate(value)}</span>
  </div>

  <button
    class="nav-btn"
    on:click={() => changeDate(1)}
    data-testid="date-selector-next"
    aria-label="Next day"
  >
    →
  </button>

  {#if !isToday}
    <button
      class="today-btn"
      on:click={goToToday}
      data-testid="date-selector-today"
    >
      Today
    </button>
  {/if}
</div>

<style>
  .date-selector {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .nav-btn {
    width: 32px;
    height: 32px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .nav-btn:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
  }

  .date-display {
    position: relative;
  }

  .date-display input {
    position: absolute;
    opacity: 0;
    width: 100%;
    height: 100%;
    cursor: pointer;
  }

  .date-text {
    padding: 0.5rem 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    font-size: 0.95rem;
    color: var(--text-primary);
    cursor: pointer;
  }

  .today-btn {
    padding: 0.5rem 0.75rem;
    border: 1px solid var(--accent-color);
    border-radius: 6px;
    background: var(--accent-color);
    color: white;
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .today-btn:hover {
    opacity: 0.9;
  }
</style>
