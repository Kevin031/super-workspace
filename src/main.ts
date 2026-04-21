import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import { useWorkspaceStore } from './stores/workspace';

const app = createApp(App);
const pinia = createPinia();

app.use(pinia);

// 在挂载前初始化主题
const workspaceStore = useWorkspaceStore();
workspaceStore.loadConfig();

// 监听系统主题变化
const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
mediaQuery.addEventListener('change', () => {
  if (workspaceStore.config.theme === 'auto') {
    workspaceStore.applyTheme();
  }
});

app.mount('#app');
