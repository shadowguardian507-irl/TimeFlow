<script lang="ts">
  import { onMount } from 'svelte';
  import * as api from '../api/commands';
  import type { ActiTimeView as ActiTimeViewType, ActiTimeEntry } from '../api/types';

  export let currentDate: string;

  let view: ActiTimeViewType | null = null;
  let loading = true;
  let copied = false;

  $: {
    loadView(currentDate);
  }

  async function loadView(date: string) {
    loading = true;
    try {
      view = await api.getActiTimeView(date);
    } catch (e) {
      console.error('Failed to load ActiTime view:', e);
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

  async function copyToClipboard() {
    if (!view) return;

    const lines = view.entries.map(e =>
      `${e.category_path}\t${e.duration_minutes}`
    );
    lines.push(`Total\t${view.total_minutes}`);

    try {
      await navigator.clipboard.writeText(lines.join('\n'));
      copied = true;
      setTimeout(() => copied = false, 2000);
    } catch (e) {
      console.error('Failed to copy:', e);
    }
  }
</script>

<div class="actitime-view" data-testid="actitime-view">
  <div class="view-header">
    <h2>ActiTime View</h2>
    <p class="subtitle">Ready to copy to ActiTime</p>
  </div>

  {#if loading}
    <div class="loading">Loading...</div>
  {:else if !view || view.entries.length === 0}
    <div class="empty-state">
      <p>No direct tasks for this day</p>
      <p class="hint">Add direct tasks to see them here</p>
    </div>
  {:else}
    <div class="table-container">
      <table>
        <thead>
          <tr>
            <th>Category</th>
            <th class="duration-col">Duration</th>
            <th class="tasks-col">Tasks</th>
          </tr>
        </thead>
        <tbody>
          {#each view.entries as entry}
            <tr>
              <td class="category-cell">{entry.category_path}</td>
              <td class="duration-cell">{formatDuration(entry.duration_minutes)}</td>
              <td class="tasks-cell">
                {entry.task_names.join(', ')}
              </td>
            </tr>
          {/each}
        </tbody>
        <tfoot>
          <tr>
            <td>Total</td>
            <td class="duration-cell total">{formatDuration(view.total_minutes)}</td>
            <td></td>
          </tr>
        </tfoot>
      </table>
    </div>

    <div class="actions">
      <button
        class="copy-btn"
        on:click={copyToClipboard}
        data-testid="copy-actitime-btn"
      >
        {copied ? '✓ Copied!' : '📋 Copy to Clipboard'}
      </button>
    </div>
  {/if}
</div>

<style>
  .actitime-view {
    max-width: 900px;
  }

  .view-header {
    margin-bottom: 1.5rem;
  }

  .view-header h2 {
    margin: 0;
    color: var(--text-primary);
  }

  .subtitle {
    margin: 0.25rem 0 0;
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .loading, .empty-state {
    text-align: center;
    padding: 3rem;
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

  .table-container {
    background: var(--bg-secondary);
    border-radius: 8px;
    overflow: hidden;
  }

  table {
    width: 100%;
    border-collapse: collapse;
  }

  th, td {
    padding: 0.75rem 1rem;
    text-align: left;
    border-bottom: 1px solid var(--border-color);
  }

  th {
    background: var(--bg-primary);
    font-weight: 600;
    font-size: 0.8rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-secondary);
  }

  .duration-col {
    width: 100px;
    text-align: right;
  }

  .tasks-col {
    width: 200px;
  }

  .category-cell {
    font-family: monospace;
    font-size: 0.9rem;
  }

  .duration-cell {
    text-align: right;
    font-weight: 500;
    color: var(--accent-color);
  }

  .tasks-cell {
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  tfoot td {
    font-weight: 600;
    background: var(--bg-primary);
    border-bottom: none;
  }

  .total {
    font-size: 1.1rem;
  }

  .actions {
    margin-top: 1rem;
    display: flex;
    justify-content: flex-end;
  }

  .copy-btn {
    padding: 0.75rem 1.5rem;
    background: var(--accent-color);
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .copy-btn:hover {
    opacity: 0.9;
  }
</style>
