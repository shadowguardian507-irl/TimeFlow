<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import TemplateList from './TemplateList.svelte';
  import TemplateForm from './TemplateForm.svelte';
  import { templatesStore, templates } from '../stores/templates';
  import { categoriesStore } from '../stores/categories';
  import type { Template } from '../api/types';

  const dispatch = createEventDispatcher<{
    applyTemplate: Template;
  }>();

  let showForm = false;
  let editingTemplate: Template | null = null;

  onMount(async () => {
    await templatesStore.load();
    await categoriesStore.load();
  });

  function handleAdd() {
    editingTemplate = null;
    showForm = true;
  }

  function handleEdit(event: CustomEvent<Template>) {
    editingTemplate = event.detail;
    showForm = true;
  }

  function handleApply(event: CustomEvent<Template>) {
    dispatch('applyTemplate', event.detail);
  }

  async function handleDelete(event: CustomEvent<Template>) {
    const template = event.detail;
    if (confirm(`Delete template "${template.name}"?`)) {
      await templatesStore.delete(template.id);
    }
  }

  function handleFormSave() {
    showForm = false;
    editingTemplate = null;
  }

  function handleFormCancel() {
    showForm = false;
    editingTemplate = null;
  }
</script>

<div class="templates-view" data-testid="templates-view">
  <div class="view-header">
    <div>
      <h2>Templates</h2>
      <p class="subtitle">Create templates for recurring tasks</p>
    </div>
    <button class="add-btn" on:click={handleAdd}>
      + New Template
    </button>
  </div>

  {#if showForm}
    <TemplateForm 
      template={editingTemplate}
      on:save={handleFormSave}
      on:cancel={handleFormCancel}
    />
  {/if}

  <TemplateList 
    templates={$templates}
    on:apply={handleApply}
    on:edit={handleEdit}
    on:delete={handleDelete}
  />
</div>

<style>
  .templates-view {
    max-width: 800px;
  }

  .view-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
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

  .add-btn {
    padding: 0.625rem 1.25rem;
    background: var(--accent-color);
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .add-btn:hover {
    opacity: 0.9;
  }
</style>
