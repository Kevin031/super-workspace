<script setup lang="ts">
import { onMounted } from 'vue';
import { useWorkspaceStore } from './stores/workspace';
import Sidebar from './components/Sidebar/Sidebar.vue';
import ProjectDetail from './components/ProjectDetail/ProjectDetail.vue';
import { invoke } from '@tauri-apps/api/core';
import type { TreeItem } from './types';

const workspaceStore = useWorkspaceStore();

onMounted(async () => {
  // 加载配置
  workspaceStore.loadConfig();

  // 如果没有配置扫描路径，使用默认路径
  if (!workspaceStore.config.scanPath) {
    const defaultPath = await invoke<string>('get_default_projects_path');
    workspaceStore.updateConfig({ scanPath: defaultPath });
  }

  // 扫描项目
  await scanProjects();
});

async function scanProjects() {
  workspaceStore.setLoading(true);
  try {
    const projects = await invoke<TreeItem[]>('scan_directory', {
      path: workspaceStore.config.scanPath
    });
    workspaceStore.setProjects(projects);
  } catch (error) {
    console.error('扫描项目失败:', error);
  } finally {
    workspaceStore.setLoading(false);
  }
}
</script>

<template>
  <div class="app-container">
    <Sidebar />
    <ProjectDetail />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
  background: var(--bg-primary);
  position: relative;
}

/* 微妙的背景纹理 */
.app-container::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background:
    radial-gradient(ellipse 60% 40% at 80% 0%, var(--accent-glow), transparent),
    radial-gradient(ellipse 40% 30% at 20% 100%, var(--accent-muted), transparent);
  pointer-events: none;
  z-index: 0;
}

.app-container > * {
  position: relative;
  z-index: 1;
}
</style>
