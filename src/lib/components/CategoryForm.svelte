<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { categoriesStore } from '../stores/categories';

  export let parentPath: string = '';

  const dispatch = createEventDispatcher();

  let name = '';
  let saving = false;
  let error = '';

  $: fullPath = parentPath ? `${parentPath} > ${name}` : name;

  async function handleSubmit() {
    if (!name.trim()) {
      error = 'Category name is required';
      return;
    }

    saving = true;
    error = '';

    try {
      await categoriesStore.add(fullPath);
      dispatch('save');
      name = '';
    } catch (e) {
      error = e instanceof Error ? e.message : 'Failed to add category';
    } finally {
      saving = false;
    }
  }

  function handleCancel() {
    dispatch('cancel');
  }
</script>

<div class="category-form" data-testid="category-form">
  <h4>{parentPath ? `Add subcategory to "${parentPath}"` : 'Add Category'}</h4>

  {#if error}
    <div class="error-message" role="alert">{error}</div>
  {/if}

  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="categoryName">Category Name</label>
      <input 
        id="categoryName"
        type="text"
        bind:value={name}
        placeholder="Enter category name"
        data-testid="category-name-input"
      />
    </div>

    {#if name}
      <div class="preview">
        <span class="label">Full path:</span>
        <span class="path">{fullPath}</span>
      </div>
    {/if}

    <div class="form-actions">
      <button type="button" class="cancel-btn" on:click={handleCancel}>
        Cancel
      </button>
      <button type="submit" class="save-btn" disabled={saving}>
        {saving ? 'Adding...' : 'Add Category'}
      </button>
    </div>
  </form>
</div>

<style>
  .category-form {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    padding: 1rem;
    margin-bottom: 1rem;
  }

  h4 {
    margin: 0 0 1rem;
    color: var(--text-primary);
    font-size: 1rem;
  }

  .error-message {
    background: var(--error-bg);
    color: var(--error-color);
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.875rem;
  }

  .form-group {
    margin-bottom: 1rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-size: 0.875rem;
    color: var(--text-secondary);
  }

  input {
    width: 100%;
    padding: 0.625rem;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-secondary);
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  input:focus {
    outline: none;
    border-color: var(--accent-color);
  }

  .preview {
    background: var(--bg-secondary);
    padding: 0.5rem 0.75rem;
    border-radius: 6px;
    margin-bottom: 1rem;
    font-size: 0.8rem;
  }

  .preview .label {
    color: var(--text-secondary);
    margin-right: 0.5rem;
  }

  .preview .path {
    color: var(--accent-color);
    font-family: monospace;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 0.75rem;
  }

  .cancel-btn, .save-btn {
    padding: 0.5rem 1rem;
    border-radius: 6px;
    font-size: 0.875rem;
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
