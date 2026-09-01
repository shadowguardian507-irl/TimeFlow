import './styles/global.css';
import App from './App.svelte';
import { mount } from 'svelte';
import { applyTheme } from './lib/theme';

// Apply theme on load
applyTheme('system');

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
