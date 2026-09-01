<script lang="ts">
  import { onMount } from 'svelte';
  import CategoryTree from './CategoryTree.svelte';
  import CategoryForm from './CategoryForm.svelte';
  import { categoriesStore, categories } from '../stores/categories';

  let showForm = false;
  let parentPath = '';
  let showHidden = true;

  onMount(async () => {
    await categoriesStore.load();
  });

  function handleAddRoot() {
    parentPath = '';
    showForm = true;
  }

  function handleAddChild(event: CustomEvent<string>) {
    parentPath = event.detail;
    showForm = true;
  }

  async function handleHide(event: CustomEvent<string>) {
    await categoriesStore.hide(event.detail);
  }

  async function handleUnhide(event: CustomEvent<string>) {
    await categoriesStore.unhide(event.detail);
  }

  function handleFormSave() {
    showForm = false;
    parentPath = '';
  }

  function handleFormCancel() {
    showForm = false;
    parentPath = '';
  }
</script>

<div class="categories-view" data-testid="categories-view">
  <div class="view-header">
    <div>
      <h2>Categories</h2>
      <p class="subtitle">Manage your ActiTime category hierarchy</p>
    </div>
    <div class="header-actions">
      <label class="toggle">
        <input type="checkbox" bind:checked={showHidden} />
        Show hidden
      </label>
      <button class="add-btn" on:click={handleAddRoot}>
        + Add Category
      </button>
    </div>
  </div>

  {#if showForm}
    <CategoryForm 
      {parentPath}
      on:save={handleFormSave}
      on:cancel={handleFormCancel}
    />
  {/if}

  <CategoryTree 
    categories={$categories}
    {showHidden}
    on:add={handleAddChild}
    on:hide={handleHide}
    on:unhide={handleUnhide}
  />
</div>

<style>
  .categories-view {
    max-width: 700px;
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

  .header-actions {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
    cursor: pointer;
  }

  .toggle input {
    cursor: pointer;
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
