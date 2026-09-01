<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { getVersion } from '@tauri-apps/api/app';
  import { onMount } from 'svelte';

  export let currentView: string;

  const dispatch = createEventDispatcher<{ viewChange: string }>();

  let version = '';

  onMount(async () => {
    try {
      version = await getVersion();
    } catch {
      version = __APP_VERSION__;
    }
  });

  const menuItems = [
    { id: 'time-entry', label: 'Time Entry', icon: '⏱️' },
    { id: 'actitime', label: 'ActiTime View', icon: '📊' },
    { id: 'week', label: 'Week View', icon: '📅' },
    { id: 'templates', label: 'Templates', icon: '📋' },
    { id: 'categories', label: 'Categories', icon: '🏷️' },
    { id: 'settings', label: 'Settings', icon: '⚙️' },
  ];

  function selectView(viewId: string) {
    dispatch('viewChange', viewId);
  }
</script>

<aside class="sidebar" data-testid="sidebar">
  <div class="logo">
    <h1>TimeFlow</h1>
    {#if version}
      <span class="version">v{version}</span>
    {/if}
  </div>

  <nav class="nav">
    {#each menuItems as item}
      <button
        class="nav-item"
        class:active={currentView === item.id}
        on:click={() => selectView(item.id)}
        data-testid={`sidebar-nav-${item.id}`}
      >
        <span class="icon">{item.icon}</span>
        <span class="label">{item.label}</span>
      </button>
    {/each}
  </nav>
</aside>

<style>
  .sidebar {
    width: 220px;
    background: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
  }

  .logo {
    padding: 1.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .logo h1 {
    margin: 0;
    font-size: 1.5rem;
    color: var(--text-primary);
  }

  .version {
    display: block;
    font-size: 0.75rem;
    color: var(--text-secondary);
    margin-top: 0.25rem;
  }

  .nav {
    flex: 1;
    padding: 1rem 0;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.75rem 1.5rem;
    border: none;
    background: none;
    color: var(--text-secondary);
    font-size: 0.95rem;
    cursor: pointer;
    transition: all 0.15s ease;
    text-align: left;
  }

  .nav-item:hover {
    background: var(--bg-hover);
    color: var(--text-primary);
  }

  .nav-item.active {
    background: var(--bg-active);
    color: var(--accent-color);
  }

  .icon {
    font-size: 1.1rem;
  }
</style>
