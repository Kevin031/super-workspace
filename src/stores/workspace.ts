import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { TreeItem, ProjectConfig } from '../types';

export const useWorkspaceStore = defineStore('workspace', () => {
  // 状态
  const projects = ref<TreeItem[]>([]);
  const selectedProject = ref<TreeItem | null>(null);
  const isLoading = ref(false);
  const config = ref<ProjectConfig>({
    scanPath: '',
    theme: 'auto',
    sidebarWidth: 300
  });

  // 计算属性
  const projectCount = computed(() => {
    const countProjects = (items: TreeItem[]): number => {
      let count = 0;
      items.forEach(item => {
        if (item.isProject) count++;
        if (item.children) count += countProjects(item.children);
      });
      return count;
    };
    return countProjects(projects.value);
  });

  const groupCount = computed(() => {
    return projects.value.filter(p => !p.isProject).length;
  });

  const effectiveTheme = computed(() => {
    const t = config.value.theme;
    if (t === 'auto') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return t;
  });

  // 方法
  function setProjects(items: TreeItem[]) {
    projects.value = items;
  }

  function setSelectedProject(item: TreeItem | null) {
    selectedProject.value = item;
  }

  function setLoading(loading: boolean) {
    isLoading.value = loading;
  }

  function applyTheme() {
    const theme = effectiveTheme.value;
    document.documentElement.setAttribute('data-theme', theme);
  }

  function updateConfig(newConfig: Partial<ProjectConfig>) {
    config.value = { ...config.value, ...newConfig };
    // 保存到本地存储
    localStorage.setItem('workspace-config', JSON.stringify(config.value));
    // 如果主题变更，立即应用
    if ('theme' in newConfig) {
      applyTheme();
    }
  }

  function loadConfig() {
    const saved = localStorage.getItem('workspace-config');
    if (saved) {
      try {
        config.value = JSON.parse(saved);
      } catch (e) {
        console.error('Failed to parse config:', e);
      }
    }
    applyTheme();
  }

  function toggleExpanded(item: TreeItem) {
    if (item.children) {
      item.isExpanded = !item.isExpanded;
    }
  }

  function findProjectByPath(path: string): TreeItem | null {
    const find = (items: TreeItem[]): TreeItem | null => {
      for (const item of items) {
        if (item.path === path) return item;
        if (item.children) {
          const found = find(item.children);
          if (found) return found;
        }
      }
      return null;
    };
    return find(projects.value);
  }

  return {
    projects,
    selectedProject,
    isLoading,
    config,
    effectiveTheme,
    projectCount,
    groupCount,
    setProjects,
    setSelectedProject,
    setLoading,
    updateConfig,
    loadConfig,
    applyTheme,
    toggleExpanded,
    findProjectByPath
  };
});
