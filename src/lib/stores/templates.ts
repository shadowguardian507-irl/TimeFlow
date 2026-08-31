import { writable } from 'svelte/store';
import type { Template, TemplateUpdate, TaskOverrides, DistributionStrategy } from '../api/types';
import * as api from '../api/commands';

const _templates = writable<Template[]>([]);

// Export the readable store
export const templates = { subscribe: _templates.subscribe };

function createTemplatesStore() {
  return {
    subscribe: _templates.subscribe,
    async load() {
      try {
        const templates = await api.getTemplates();
        _templates.set(templates);
        return templates;
      } catch (e) {
        console.error('Failed to load templates:', e);
        _templates.set([]);
        return [];
      }
    },
    async create(
      name: string,
      defaultDuration: number,
      categoryPath: string,
      isMergeable: boolean,
      distributionStrategy?: DistributionStrategy
    ) {
      const template = await api.createTemplate(
        name,
        defaultDuration,
        categoryPath,
        isMergeable,
        distributionStrategy
      );
      _templates.update((templates) => [...templates, template]);
      return template;
    },
    async update(templateId: string, updates: TemplateUpdate) {
      const updatedTemplate = await api.updateTemplate(templateId, updates);
      _templates.update((templates) =>
        templates.map((t) => (t.id === templateId ? updatedTemplate : t))
      );
      return updatedTemplate;
    },
    async delete(templateId: string) {
      await api.deleteTemplate(templateId);
      _templates.update((templates) => templates.filter((t) => t.id !== templateId));
    },
    async apply(templateId: string, overrides?: TaskOverrides) {
      return api.applyTemplate(templateId, overrides);
    },
  };
}

export const templatesStore = createTemplatesStore();
