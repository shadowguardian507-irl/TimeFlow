import './styles/global.css';
import App from './App.svelte';

// Apply theme on load
function applyInitialTheme() {
  const savedTheme = localStorage.getItem('timeflow-theme') || 'system';
  if (savedTheme === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    document.documentElement.setAttribute('data-theme', prefersDark ? 'dark' : 'light');
  } else {
    document.documentElement.setAttribute('data-theme', savedTheme);
  }
}

applyInitialTheme();

const app = new App({
  target: document.getElementById('app')!,
});

export default app;
