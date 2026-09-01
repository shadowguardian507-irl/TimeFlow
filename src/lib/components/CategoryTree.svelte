<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { Category } from '../api/types';

  export let categories: Category | null = null;
  export let showHidden: boolean = true;

  const dispatch = createEventDispatcher<{
    add: string;
    hide: string;
    unhide: string;
  }>();

  let expanded: Set<string> = new Set();

  function toggleExpand(path: string) {
    if (expanded.has(path)) {
      expanded.delete(path);
    } else {
      expanded.add(path);
    }
    expanded = expanded;
  }

  function buildPath(parentPath: string, name: string): string {
    return parentPath ? `${parentPath} > ${name}` : name;
  }
</script>

<div class="category-tree" data-testid="category-tree">
  {#if categories}
    {#each categories.children as child}
      {@const childPath = child.name}
      {#if showHidden || !child.hidden}
        <div class="tree-node">
          <div class="node-row" class:hidden={child.hidden}>
            {#if child.children.length > 0}
              <button 
                class="expand-btn"
                on:click={() => toggleExpand(childPath)}
                aria-label={expanded.has(childPath) ? 'Collapse' : 'Expand'}
              >
                {expanded.has(childPath) ? '▼' : '▶'}
              </button>
            {:else}
              <span class="expand-placeholder"></span>
            {/if}
            
            <span class="node-name">
              {child.name}
              {#if child.hidden}<span class="hidden-badge">hidden</span>{/if}
            </span>

            <div class="node-actions">
              <button 
                class="action-btn"
                on:click={() => dispatch('add', childPath)}
                title="Add subcategory"
              >
                +
              </button>
              {#if child.hidden}
                <button 
                  class="action-btn"
                  on:click={() => dispatch('unhide', childPath)}
                  title="Show category"
                >
                  👁
                </button>
              {:else}
                <button 
                  class="action-btn"
                  on:click={() => dispatch('hide', childPath)}
                  title="Hide category"
                >
                  🙈
                </button>
              {/if}
            </div>
          </div>

          {#if expanded.has(childPath) && child.children.length > 0}
            <div class="children">
              {#each child.children as grandchild}
                {@const grandchildPath = buildPath(childPath, grandchild.name)}
                {#if showHidden || !grandchild.hidden}
                  <div class="tree-node">
                    <div class="node-row" class:hidden={grandchild.hidden}>
                      {#if grandchild.children.length > 0}
                        <button 
                          class="expand-btn"
                          on:click={() => toggleExpand(grandchildPath)}
                        >
                          {expanded.has(grandchildPath) ? '▼' : '▶'}
                        </button>
                      {:else}
                        <span class="expand-placeholder"></span>
                      {/if}
                      
                      <span class="node-name">
                        {grandchild.name}
                        {#if grandchild.hidden}<span class="hidden-badge">hidden</span>{/if}
                      </span>

                      <div class="node-actions">
                        <button 
                          class="action-btn"
                          on:click={() => dispatch('add', grandchildPath)}
                          title="Add subcategory"
                        >
                          +
                        </button>
                        {#if grandchild.hidden}
                          <button 
                            class="action-btn"
                            on:click={() => dispatch('unhide', grandchildPath)}
                          >
                            👁
                          </button>
                        {:else}
                          <button 
                            class="action-btn"
                            on:click={() => dispatch('hide', grandchildPath)}
                          >
                            🙈
                          </button>
                        {/if}
                      </div>
                    </div>

                    {#if expanded.has(grandchildPath) && grandchild.children.length > 0}
                      <div class="children">
                        {#each grandchild.children as leaf}
                          {@const leafPath = buildPath(grandchildPath, leaf.name)}
                          {#if showHidden || !leaf.hidden}
                            <div class="node-row leaf" class:hidden={leaf.hidden}>
                              <span class="expand-placeholder"></span>
                              <span class="node-name">
                                {leaf.name}
                                {#if leaf.hidden}<span class="hidden-badge">hidden</span>{/if}
                              </span>
                              <div class="node-actions">
                                {#if leaf.hidden}
                                  <button 
                                    class="action-btn"
                                    on:click={() => dispatch('unhide', leafPath)}
                                  >
                                    👁
                                  </button>
                                {:else}
                                  <button 
                                    class="action-btn"
                                    on:click={() => dispatch('hide', leafPath)}
                                  >
                                    🙈
                                  </button>
                                {/if}
                              </div>
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
    <p class="empty">No categories yet. Add your first category above.</p>
  {/if}
</div>

<style>
  .category-tree {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 1rem;
  }

  .tree-node {
    margin-left: 0;
  }

  .children {
    margin-left: 1.5rem;
    border-left: 1px solid var(--border-color);
    padding-left: 0.5rem;
  }

  .node-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem;
    border-radius: 4px;
    transition: background 0.15s ease;
  }

  .node-row:hover {
    background: var(--bg-hover);
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
    font-size: 0.75rem;
    width: 1.5rem;
  }

  .expand-placeholder {
    width: 1.5rem;
  }

  .node-name {
    flex: 1;
    color: var(--text-primary);
    font-size: 0.9rem;
  }

  .hidden-badge {
    font-size: 0.65rem;
    padding: 0.1rem 0.3rem;
    background: var(--warning-bg);
    color: var(--warning-color);
    border-radius: 3px;
    margin-left: 0.5rem;
  }

  .node-actions {
    display: flex;
    gap: 0.25rem;
    opacity: 0;
    transition: opacity 0.15s ease;
  }

  .node-row:hover .node-actions {
    opacity: 1;
  }

  .action-btn {
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    padding: 0.25rem 0.5rem;
    cursor: pointer;
    font-size: 0.75rem;
    color: var(--text-primary);
  }

  .action-btn:hover {
    background: var(--bg-hover);
  }

  .empty {
    text-align: center;
    color: var(--text-secondary);
    padding: 2rem;
  }
</style>
