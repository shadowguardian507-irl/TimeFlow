import { writable } from 'svelte/store';
import type { Category } from '../api/types';
import * as api from '../api/commands';

const _categories = writable<Category | null>(null);

// Export the readable store
export const categories = { subscribe: _categories.subscribe };

function createCategoriesStore() {
  return {
    subscribe: _categories.subscribe,
    async load() {
      try {
        const tree = await api.getCategoryTree();
        _categories.set(tree);
        return tree;
      } catch (e) {
        console.error('Failed to load categories:', e);
        _categories.set(null);
        return null;
      }
    },
    async getVisiblePaths() {
      return api.getVisibleCategoryPaths();
    },
    async add(path: string) {
      const tree = await api.addCategory(path);
      _categories.set(tree);
      return tree;
    },
    async hide(path: string) {
      await api.hideCategory(path);
      return this.load();
    },
    async unhide(path: string) {
      await api.unhideCategory(path);
      return this.load();
    },
  };
}

export const categoriesStore = createCategoriesStore();
