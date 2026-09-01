<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import TimerWidget from './TimerWidget.svelte';
  import TaskList from './TaskList.svelte';
  import TaskEntryForm from './TaskEntryForm.svelte';
  import DailySummary from './DailySummary.svelte';
  import { tasksStore, tasks } from '../stores/tasks';
  import { categoriesStore } from '../stores/categories';
  import { templatesStore } from '../stores/templates';
  import type { Task, Template } from '../api/types';

  export let currentDate: string;
  export let selectedTemplate: Template | null = null;

  const dispatch = createEventDispatcher();

  let showForm = false;
  let editingTask: Task | null = null;
  let formTemplate: Template | null = null;

  $: {
    loadData(currentDate);
  }

  // Watch for selectedTemplate changes and open form
  $: if (selectedTemplate) {
    formTemplate = selectedTemplate;
    editingTask = null;
    showForm = true;
    dispatch('templateUsed');
  }

  async function loadData(date: string) {
    await tasksStore.loadForDate(date);
    await categoriesStore.load();
    await templatesStore.load();
  }

  function handleAddTask() {
    editingTask = null;
    formTemplate = null;
    showForm = true;
  }

  function handleEditTask(event: CustomEvent<Task>) {
    editingTask = event.detail;
    formTemplate = null;
    showForm = true;
  }

  function handleFormClose() {
    showForm = false;
    editingTask = null;
    formTemplate = null;
  }

  async function handleFormSave() {
    showForm = false;
    editingTask = null;
    formTemplate = null;
    await tasksStore.loadForDate(currentDate);
  }

  async function handleDeleteTask(event: CustomEvent<Task>) {
    const task = event.detail;
    if (confirm(`Delete task "${task.name}"?`)) {
      await tasksStore.delete(task.id, currentDate);
    }
  }
</script>

<div class="time-entry-view" data-testid="time-entry-view">
  <div class="top-section">
    <TimerWidget {currentDate} on:taskCreated={handleFormSave} />
    <DailySummary tasks={$tasks} />
  </div>

  <div class="task-section">
    <div class="section-header">
      <h3>Tasks</h3>
      <button
        class="add-btn"
        on:click={handleAddTask}
        data-testid="time-entry-add-task"
      >
        + Add Task
      </button>
    </div>

    <TaskList
      tasks={$tasks}
      on:edit={handleEditTask}
      on:delete={handleDeleteTask}
    />
  </div>

  {#if showForm}
    <TaskEntryForm
      task={editingTask}
      template={formTemplate}
      date={currentDate}
      on:close={handleFormClose}
      on:save={handleFormSave}
    />
  {/if}
</div>

<style>
  .time-entry-view {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .top-section {
    display: grid;
    grid-template-columns: 1fr 300px;
    gap: 1.5rem;
  }

  .task-section {
    background: var(--bg-secondary);
    border-radius: 8px;
    padding: 1.5rem;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .section-header h3 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .add-btn {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 6px;
    background: var(--accent-color);
    color: white;
    font-size: 0.875rem;
    cursor: pointer;
    transition: opacity 0.15s ease;
  }

  .add-btn:hover {
    opacity: 0.9;
  }
</style>
