import './styles/global.css';
import App from './App.svelte';
import { applyTheme } from './lib/theme';

// Apply theme on load
applyTheme('system');

const app = new App({
  target: document.getElementById('app')!,
});

export default app;
