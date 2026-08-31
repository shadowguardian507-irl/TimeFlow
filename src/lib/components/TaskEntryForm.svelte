<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import CategoryPicker from './CategoryPicker.svelte';
  import { tasksStore } from '../stores/tasks';
  import { categoriesStore, categories } from '../stores/categories';
  import { templatesStore, templates } from '../stores/templates';
  import { settingsStore, settings } from '../stores/settings';
  import type { Task, TaskType, DistributionStrategy, Template } from '../api/types';

  export let task: Task | null = null;
  export let template: Template | null = null;
  export let date: string;

  const dispatch = createEventDispatcher();

  // Initialize from template if provided, otherwise from task or defaults
  let name = template?.name ?? task?.name ?? '';
  let durationMinutes = template?.default_duration ?? task?.duration_minutes ?? 30;
  let categoryPath = template?.category_path ?? task?.category_path ?? '';
  let taskType: TaskType = template ? (template.is_mergeable ? 'mergeable' : 'direct') : (task?.task_type ?? 'direct');
  let distributionStrategy: DistributionStrategy = template?.distribution_strategy ?? task?.distribution_strategy ?? 'proportional';
  let selectedTemplate: string = '';
  let saving = false;
  let error = '';

  $: isEdit = task !== null;
  $: minDuration = $settings?.minimum_task_duration ?? 1;

  async function handleSubmit() {
    if (!name.trim()) {
      error = 'Task name is required';
      return;
    }
    if (durationMinutes < minDuration) {
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
      if (isEdit && task) {
        await tasksStore.update(task.id, date, {
          name,
          duration_minutes: durationMinutes,
          category_path: categoryPath,
          task_type: taskType,
          distribution_strategy: taskType === 'mergeable' ? distributionStrategy : undefined,
        });
      } else {
        await tasksStore.create(
          name,
          durationMinutes,
          categoryPath,
          taskType,
          taskType === 'mergeable' ? distributionStrategy : undefined,
          date
        );
      }
      dispatch('save');
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to save task';
    } finally {
      saving = false;
    }
  }

  function handleTemplateSelect() {
    if (!selectedTemplate) return;
    const template = $templates.find(t => t.id === selectedTemplate);
    if (template) {
      name = template.name;
      durationMinutes = template.default_duration;
      categoryPath = template.category_path;
      taskType = template.is_mergeable ? 'mergeable' : 'direct';
      if (template.distribution_strategy) {
        distributionStrategy = template.distribution_strategy;
      }
    }
  }

  function handleCancel() {
    dispatch('close');
  }
</script>

<div class="modal-overlay" on:click|self={handleCancel} data-testid="task-form-overlay">
  <div class="task-form" data-testid="task-entry-form">
    <h3>{isEdit ? 'Edit Task' : 'Add Task'}</h3>

    {#if error}
      <div class="error-message" role="alert">{error}</div>
    {/if}

    <form on:submit|preventDefault={handleSubmit}>
      {#if !isEdit && $templates.length > 0}
        <div class="form-group">
          <label for="template">From Template</label>
          <select 
            id="template" 
            bind:value={selectedTemplate}
            on:change={handleTemplateSelect}
          >
            <option value="">-- Select template --</option>
            {#each $templates as template}
              <option value={template.id}>{template.name}</option>
            {/each}
          </select>
        </div>
      {/if}

      <div class="form-group">
        <label for="name">Task Name *</label>
        <input 
          id="name"
          type="text"
          bind:value={name}
          placeholder="What did you work on?"
          data-testid="task-name-input"
        />
      </div>

      <div class="form-row">
        <div class="form-group">
          <label for="duration">Duration (minutes) *</label>
          <input 
            id="duration"
            type="number"
            bind:value={durationMinutes}
            min={minDuration}
            data-testid="task-duration-input"
          />
        </div>

        <div class="form-group">
          <label for="taskType">Task Type</label>
          <select id="taskType" bind:value={taskType} data-testid="task-type-select">
            <option value="direct">Direct (ActiTime)</option>
            <option value="mergeable">Mergeable</option>
          </select>
        </div>
      </div>

      <div class="form-group">
        <label>Category *</label>
        <CategoryPicker 
          categories={$categories}
          bind:value={categoryPath}
        />
      </div>

      {#if taskType === 'mergeable'}
        <div class="form-group">
          <label for="strategy">Distribution Strategy</label>
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
          {saving ? 'Saving...' : (isEdit ? 'Update' : 'Add Task')}
        </button>
      </div>
    </form>
  </div>
</div>

<style>
  .modal-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 100;
  }

  .task-form {
    background: var(--bg-primary);
    border-radius: 12px;
    padding: 1.5rem;
    width: 100%;
    max-width: 480px;
    max-height: 90vh;
    overflow-y: auto;
  }

  h3 {
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

  label {
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
    transition: opacity 0.15s ease;
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

  .cancel-btn:hover, .save-btn:hover:not(:disabled) {
    opacity: 0.9;
  }
</style>
