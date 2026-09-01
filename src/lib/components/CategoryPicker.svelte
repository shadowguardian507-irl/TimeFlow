<script lang="ts">
  import type { Category } from '../api/types';

  export let categories: Category | null = null;
  export let value: string = '';
  export let showHidden: boolean = false;

  let expanded: Set<string> = new Set();
  let searchQuery = '';

  function toggleExpand(path: string) {
    if (expanded.has(path)) {
      expanded.delete(path);
    } else {
      expanded.add(path);
    }
    expanded = expanded;
  }

  function selectCategory(path: string) {
    value = path;
  }

  function buildPath(parentPath: string, name: string): string {
    return parentPath ? `${parentPath} > ${name}` : name;
  }

  function matchesSearch(name: string): boolean {
    if (!searchQuery) return true;
    return name.toLowerCase().includes(searchQuery.toLowerCase());
  }

  function hasVisibleChildren(cat: Category, parentPath: string): boolean {
    if (!cat.children.length) return false;
    return cat.children.some(child => {
      if (!showHidden && child.hidden) return false;
      if (matchesSearch(child.name)) return true;
      return hasVisibleChildren(child, buildPath(parentPath, cat.name));
    });
  }
</script>

<div class="category-picker" data-testid="category-picker">
  <input
    type="text"
    class="search-input"
    placeholder="Search categories..."
    bind:value={searchQuery}
  />

  {#if value}
    <div class="selected-path">
      <span>Selected:</span> {value}
      <button type="button" on:click={() => value = ''}>×</button>
    </div>
  {/if}

  <div class="tree-container">
    {#if categories}
      {#each categories.children as child}
        {@const childPath = child.name}
        {#if (showHidden || !child.hidden) && (matchesSearch(child.name) || hasVisibleChildren(child, ''))}
          <div class="tree-node">
            <div
              class="node-row"
              class:selected={value === childPath}
              class:hidden={child.hidden}
            >
              {#if child.children.length > 0}
                <button
                  type="button"
                  class="expand-btn"
                  on:click={() => toggleExpand(childPath)}
                >
                  {expanded.has(childPath) ? '▼' : '▶'}
                </button>
              {:else}
                <span class="expand-placeholder"></span>
              {/if}
              <button
                type="button"
                class="node-name"
                on:click={() => selectCategory(childPath)}
              >
                {child.name}
                {#if child.hidden}<span class="hidden-badge">hidden</span>{/if}
              </button>
            </div>

            {#if expanded.has(childPath) && child.children.length > 0}
              <div class="children">
                {#each child.children as grandchild}
                  {@const grandchildPath = buildPath(childPath, grandchild.name)}
                  {#if (showHidden || !grandchild.hidden) && (matchesSearch(grandchild.name) || hasVisibleChildren(grandchild, childPath))}
                    <div class="tree-node">
                      <div
                        class="node-row"
                        class:selected={value === grandchildPath}
                        class:hidden={grandchild.hidden}
                      >
                        {#if grandchild.children.length > 0}
                          <button
                            type="button"
                            class="expand-btn"
                            on:click={() => toggleExpand(grandchildPath)}
                          >
                            {expanded.has(grandchildPath) ? '▼' : '▶'}
                          </button>
                        {:else}
                          <span class="expand-placeholder"></span>
                        {/if}
                        <button
                          type="button"
                          class="node-name"
                          on:click={() => selectCategory(grandchildPath)}
                        >
                          {grandchild.name}
                          {#if grandchild.hidden}<span class="hidden-badge">hidden</span>{/if}
                        </button>
                      </div>

                      {#if expanded.has(grandchildPath) && grandchild.children.length > 0}
                        <div class="children">
                          {#each grandchild.children as leaf}
                            {@const leafPath = buildPath(grandchildPath, leaf.name)}
                            {#if (showHidden || !leaf.hidden) && matchesSearch(leaf.name)}
                              <div
                                class="node-row leaf"
                                class:selected={value === leafPath}
                                class:hidden={leaf.hidden}
                              >
                                <span class="expand-placeholder"></span>
                                <button
                                  type="button"
                                  class="node-name"
                                  on:click={() => selectCategory(leafPath)}
                                >
                                  {leaf.name}
                                  {#if leaf.hidden}<span class="hidden-badge">hidden</span>{/if}
                                </button>
                              </div>
                            {/if}
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {/if}
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      {/each}
    {:else}
      <p class="empty">No categories available</p>
    {/if}
  </div>
</div>

<style>
  .category-picker {
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--bg-secondary);
    overflow: hidden;
  }

  .search-input {
    width: 100%;
    padding: 0.625rem;
    border: none;
    border-bottom: 1px solid var(--border-color);
    background: transparent;
    color: var(--text-primary);
    font-size: 0.875rem;
  }

  .search-input:focus {
    outline: none;
  }

  .selected-path {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.625rem;
    background: var(--accent-color);
    color: white;
    font-size: 0.8rem;
  }

  .selected-path span {
    opacity: 0.8;
  }

  .selected-path button {
    margin-left: auto;
    background: none;
    border: none;
    color: white;
    cursor: pointer;
    font-size: 1rem;
  }

  .tree-container {
    max-height: 200px;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .tree-node {
    margin-left: 0;
  }

  .children {
    margin-left: 1rem;
  }

  .node-row {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem;
    border-radius: 4px;
  }

  .node-row:hover {
    background: var(--bg-hover);
  }

  .node-row.selected {
    background: var(--accent-bg);
  }

  .node-row.hidden {
    opacity: 0.6;
  }

  .expand-btn {
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: var(--text-secondary);
    font-size: 0.7rem;
    width: 1.25rem;
  }

  .expand-placeholder {
    width: 1.25rem;
  }

  .node-name {
    flex: 1;
    text-align: left;
    background: none;
    border: none;
    padding: 0.25rem;
    cursor: pointer;
    color: var(--text-primary);
    font-size: 0.875rem;
  }

  .hidden-badge {
    font-size: 0.65rem;
    padding: 0.1rem 0.3rem;
    background: var(--warning-bg);
    color: var(--warning-color);
    border-radius: 3px;
    margin-left: 0.5rem;
  }

  .empty {
    text-align: center;
    color: var(--text-secondary);
    padding: 1rem;
    font-size: 0.875rem;
  }
</style>
