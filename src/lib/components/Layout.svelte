<script lang="ts">
  import Sidebar from './Sidebar.svelte';
  import Header from './Header.svelte';
  import TimeEntryView from './TimeEntryView.svelte';
  import ActiTimeView from './ActiTimeView.svelte';
  import WeekView from './WeekView.svelte';
  import TemplatesView from './TemplatesView.svelte';
  import CategoriesView from './CategoriesView.svelte';
  import SettingsView from './SettingsView.svelte';
  import type { Template } from '../api/types';

  let currentView = 'time-entry';
  let currentDate = new Date().toISOString().split('T')[0];
  let selectedTemplate: Template | null = null;

  function handleViewChange(event: CustomEvent<string>) {
    currentView = event.detail;
  }

  function handleDateChange(event: CustomEvent<string>) {
    currentDate = event.detail;
  }

  function handleApplyTemplate(event: CustomEvent<Template>) {
    selectedTemplate = event.detail;
    currentView = 'time-entry';
  }

  function handleTemplateUsed() {
    selectedTemplate = null;
  }
</script>

<div class="layout" data-testid="app-layout">
  <Sidebar {currentView} on:viewChange={handleViewChange} />
  
  <main class="main-content">
    <Header 
      {currentView} 
      {currentDate} 
      on:dateChange={handleDateChange}
      on:viewChange={handleViewChange}
    />
    
    <div class="content">
      {#if currentView === 'time-entry'}
        <TimeEntryView 
          {currentDate} 
          {selectedTemplate}
          on:templateUsed={handleTemplateUsed}
        />
      {:else if currentView === 'actitime'}
        <ActiTimeView {currentDate} />
      {:else if currentView === 'week'}
        <WeekView {currentDate} />
      {:else if currentView === 'templates'}
        <TemplatesView on:applyTemplate={handleApplyTemplate} />
      {:else if currentView === 'categories'}
        <CategoriesView />
      {:else if currentView === 'settings'}
        <SettingsView />
      {/if}
    </div>
  </main>
</div>

<style>
  .layout {
    display: flex;
    height: 100vh;
    background: var(--bg-primary);
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 1.5rem;
  }
</style>
