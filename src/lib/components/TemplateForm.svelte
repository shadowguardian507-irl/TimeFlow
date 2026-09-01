<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import CategoryPicker from './CategoryPicker.svelte';
  import { templatesStore } from '../stores/templates';
  import { categories } from '../stores/categories';
  import { settings } from '../stores/settings';
  import type { Template, DistributionStrategy } from '../api/types';

  export let template: Template | null = null;

  const dispatch = createEventDispatcher();

  let name = template?.name ?? '';
  let defaultDuration = template?.default_duration ?? 30;
  let categoryPath = template?.category_path ?? '';
  let isMergeable = template?.is_mergeable ?? false;
  let distributionStrategy: DistributionStrategy = template?.distribution_strategy ?? 'proportional';
  let saving = false;
  let error = '';

  $: isEdit = template !== null;
  $: minDuration = $settings?.minimum_task_duration ?? 1;

  async function handleSubmit() {
    if (!name.trim()) {
      error = 'Template name is required';
      return;
    }
    if (defaultDuration < minDuration) {
      error = `Duration must be at least ${minDuration} minute(s)`;
      return;
    }
    if (!categoryPath) {
      error = 'Category is required';
      return;
    }

    saving = true;
    error = '';

    try {
      if (isEdit && template) {
        await templatesStore.update(template.id, {
          name,
          default_duration: defaultDuration,
          category_path: categoryPath,
          is_mergeable: isMergeable,
          distribution_strategy: isMergeable ? distributionStrategy : undefined,
        });
      } else {
        await templatesStore.create(
          name,
          defaultDuration,
          categoryPath,
          isMergeable,
          isMergeable ? distributionStrategy : undefined
        );
      }
      dispatch('save');
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save template';
    } finally {
      saving = false;
    }
  }

  function handleCancel() {
    dispatch('cancel');
  }
</script>

<div class="template-form" data-testid="template-form">
  <h4>{isEdit ? 'Edit Template' : 'Create Template'}</h4>

  {#if error}
    <div class="error-message" role="alert">{error}</div>
  {/if}

  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="templateName">Template Name *</label>
      <input
        id="templateName"
        type="text"
        bind:value={name}
        placeholder="e.g., Daily Standup"
        data-testid="template-name-input"
      />
    </div>

    <div class="form-row">
      <div class="form-group">
        <label for="duration">Default Duration (min) *</label>
        <input
          id="duration"
          type="number"
          bind:value={defaultDuration}
          min={minDuration}
        />
      </div>

      <div class="form-group">
        <label for="taskType">Task Type</label>
        <select id="taskType" bind:value={isMergeable}>
          <option value={false}>Direct (ActiTime)</option>
          <option value={true}>Mergeable</option>
        </select>
      </div>
    </div>

    <div class="form-group">
      <span id="template-category-label" class="field-label">Category *</span>
      <div role="group" aria-labelledby="template-category-label">
        <CategoryPicker
          categories={$categories}
          bind:value={categoryPath}
          ariaLabel="Template category"
        />
      </div>
    </div>

    {#if isMergeable}
      <div class="form-group">
        <label for="strategy">Default Distribution Strategy</label>
        <select id="strategy" bind:value={distributionStrategy}>
          <option value="proportional">Proportional</option>
          <option value="even">Even</option>
          <option value="manual">Manual</option>
          <option value="weighted">Weighted</option>
        </select>
      </div>
    {/if}

    <div class="form-actions">
      <button type="button" class="cancel-btn" on:click={handleCancel}>
        Cancel
      </button>
      <button type="submit" class="save-btn" disabled={saving}>
        {saving ? 'Saving...' : (isEdit ? 'Update' : 'Create Template')}
      </button>
    </div>
  </form>
</div>

<style>
  .template-form {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 1rem;
  }

  h4 {
    margin: 0 0 1rem;
    color: var(--text-primary);
  }

  .error-message {
    background: var(--error-bg);
    color: var(--error-color);
    padding: 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  label,
  .field-label {
    display: block;
    margin-bottom: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  input, select {
    width: 100%;
    padding: 0.625rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  input:focus, select:focus {
    outline: none;
    border-color: var(--accent-color);
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
    margin-top: 1.5rem;
  }

  .cancel-btn, .save-btn {
    padding: 0.625rem 1.25rem;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
  }

  .cancel-btn {
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    color: var(--text-primary);
  }

  .save-btn {
    background: var(--accent-color);
    border: none;
    color: white;
  }

  .save-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
