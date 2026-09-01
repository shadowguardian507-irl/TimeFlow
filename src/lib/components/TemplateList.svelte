<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Template } from '../api/types';

  export let templates: Template[] = [];

  const dispatch = createEventDispatcher<{
    apply: Template;
    edit: Template;
    delete: Template;
  }>();

  function formatDuration(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    if (hours > 0) {
      return `${hours}h ${mins}m`;
    }
    return `${mins}m`;
  }
</script>

<div class="template-list" data-testid="template-list">
  {#if templates.length === 0}
    <div class="empty-state">
      <p>No templates yet</p>
      <p class="hint">Create templates for tasks you do regularly</p>
    </div>
  {:else}
    {#each templates as template (template.id)}
      <div class="template-item" data-testid="template-item">
        <div class="template-info">
          <span class="template-name">{template.name}</span>
          <span class="template-type {template.is_mergeable ? 'mergeable' : 'direct'}">
            {template.is_mergeable ? 'Mergeable' : 'Direct'}
          </span>
        </div>

        <div class="template-meta">
          <span class="category">{template.category_path}</span>
          <span class="duration">{formatDuration(template.default_duration)}</span>
        </div>

        <div class="template-actions">
          <button
            class="apply-btn"
            on:click={() => dispatch('apply', template)}
            title="Use this template"
          >
            Use
          </button>
          <button
            class="edit-btn"
            on:click={() => dispatch('edit', template)}
            title="Edit template"
          >
            ✏️
          </button>
          <button
            class="delete-btn"
            on:click={() => dispatch('delete', template)}
            title="Delete template"
          >
            🗑️
          </button>
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .template-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
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

  .template-item {
    display: grid;
    grid-template-columns: 1fr auto auto;
    gap: 1rem;
    align-items: center;
    padding: 1rem;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
  }

  .template-info {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .template-name {
    font-weight: 500;
    color: var(--text-primary);
  }

  .template-type {
    font-size: 0.7rem;
    padding: 0.2rem 0.5rem;
    border-radius: 4px;
    font-weight: 600;
  }

  .template-type.direct {
    background: var(--success-bg);
    color: var(--success-color);
  }

  .template-type.mergeable {
    background: var(--warning-bg);
    color: var(--warning-color);
  }

  .template-meta {
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

  .template-actions {
    display: flex;
    gap: 0.5rem;
  }

  .apply-btn {
    padding: 0.4rem 0.75rem;
    background: var(--accent-color);
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 0.8rem;
    cursor: pointer;
  }

  .edit-btn, .delete-btn {
    padding: 0.4rem 0.5rem;
    background: transparent;
    border: none;
    cursor: pointer;
    border-radius: 4px;
  }

  .edit-btn:hover, .delete-btn:hover {
    background: var(--bg-hover);
  }

  .apply-btn:hover {
    opacity: 0.9;
  }
</style>
