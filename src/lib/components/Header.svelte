<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import DateSelector from './DateSelector.svelte';

  export let currentView: string;
  export let currentDate: string;

  const dispatch = createEventDispatcher<{ 
    dateChange: string;
    viewChange: string;
  }>();

  const viewTitles: Record<string, string> = {
    'time-entry': 'Time Entry',
    'actitime': 'ActiTime View',
    'week': 'Week View',
    'templates': 'Templates',
    'categories': 'Categories',
    'settings': 'Settings',
  };

  function handleDateChange(event: CustomEvent<string>) {
    dispatch('dateChange', event.detail);
  }

  function toggleView() {
    if (currentView === 'time-entry') {
      dispatch('viewChange', 'actitime');
    } else if (currentView === 'actitime') {
      dispatch('viewChange', 'time-entry');
    }
  }
</script>

<header class="header" data-testid="header">
  <div class="header-left">
    <h2>{viewTitles[currentView] || 'TimeFlow'}</h2>
  </div>
  
  <div class="header-center">
    {#if currentView === 'time-entry' || currentView === 'actitime'}
      <DateSelector value={currentDate} on:change={handleDateChange} />
    {/if}
  </div>
  
  <div class="header-right">
    {#if currentView === 'time-entry' || currentView === 'actitime'}
      <button 
        class="view-toggle"
        on:click={toggleView}
        data-testid="header-view-toggle"
      >
        {currentView === 'time-entry' ? 'Show ActiTime View' : 'Show Full View'}
      </button>
    {/if}
  </div>
</header>

<style>
  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 1rem 1.5rem;
    background: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .header-left h2 {
    margin: 0;
    font-size: 1.25rem;
    color: var(--text-primary);
  }

  .header-center {
    flex: 1;
    display: flex;
    justify-content: center;
  }

  .header-right {
    display: flex;
    gap: 0.5rem;
  }

  .view-toggle {
    padding: 0.5rem 1rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-primary);
    color: var(--text-primary);
    font-size: 0.875rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .view-toggle:hover {
    background: var(--bg-hover);
    border-color: var(--accent-color);
  }
</style>
