<script setup lang="ts">
import { computed } from 'vue';
import { useWorkspaceStore } from '../../stores/workspace';
import type { TreeItem } from '../../types';
import { FolderOutline, CodeSlashOutline } from '@vicons/ionicons5';
import TreeView, { type TreeNodeData } from '../Common/TreeView.vue';
import SettingsPanel from '../Common/SettingsPanel.vue';

const workspaceStore = useWorkspaceStore();

const treeData = computed(() => {
  const transformToTreeFormat = (items: TreeItem[]): TreeNodeData[] => {
    return items.map(item => ({
      key: item.path,
      label: item.name,
      isProject: item.isProject,
      children: item.children ? transformToTreeFormat(item.children) : undefined,
    }));
  };
  return transformToTreeFormat(workspaceStore.projects);
});

function handleSelect(key: string) {
  const selected = workspaceStore.findProjectByPath(key);
  if (selected) {
    workspaceStore.setSelectedProject(selected);
  }
}
</script>

<template>
  <aside class="sidebar">
    <!-- Brand Header -->
    <div class="sidebar-header">
      <div class="brand">
        <div class="brand-mark">
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none">
            <path d="M4 4h6v6H4zM14 4h6v6h-6zM4 14h6v6H4zM14 14h6v6h-6z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M10 7h4M7 10v4M17 10v4M10 17h4" stroke="currentColor" stroke-width="1.5" opacity="0.5"/>
          </svg>
        </div>
        <div class="brand-text">
          <h1 class="brand-name">Super Workspace</h1>
          <span class="brand-tagline">工作空间管理</span>
        </div>
      </div>
      <SettingsPanel />
    </div>

    <!-- Stats -->
    <div class="stats-bar">
      <div class="stat">
        <span class="stat-number">{{ workspaceStore.projectCount }}</span>
        <span class="stat-name">项目</span>
      </div>
      <div class="stat-divider" />
      <div class="stat">
        <span class="stat-number">{{ workspaceStore.groupCount }}</span>
        <span class="stat-name">分组</span>
      </div>
    </div>

    <!-- Tree Content -->
    <div class="sidebar-body">
      <div v-if="workspaceStore.isLoading" class="loading-state">
        <div class="spinner" />
      </div>
      <div v-else-if="treeData.length === 0" class="empty-state">
        <span class="empty-text">暂无项目</span>
      </div>
      <div v-else class="tree-wrapper">
        <TreeView
          v-for="node in treeData"
          :key="node.key"
          :node="node"
          :selected-key="workspaceStore.selectedProject?.path"
          @select="handleSelect"
        >
          <template #prefix="{ option }">
            <component
              :is="option.isProject ? CodeSlashOutline : FolderOutline"
              class="tree-icon"
            />
          </template>
        </TreeView>
      </div>
    </div>

    <!-- Footer -->
    <div class="sidebar-footer">
      <div class="footer-hint">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
          <path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
        </svg>
        <span>选择项目查看详情</span>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.sidebar {
  width: 280px;
  min-width: 280px;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  border-right: none;
  border-radius: 0 20px 20px 0;
  position: relative;
  box-shadow: 2px 0 16px rgba(28, 25, 23, 0.08);
}

/* 右侧渐变分割线 */
.sidebar::after {
  content: '';
  position: absolute;
  top: 0;
  right: 0;
  width: 1px;
  height: 100%;
  background: linear-gradient(
    to bottom,
    transparent,
    rgba(120, 80, 60, 0.08) 15%,
    rgba(120, 80, 60, 0.08) 85%,
    transparent
  );
}

/* ===== Header ===== */
.sidebar-header {
  padding: 24px 20px 18px;
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.brand-mark {
  width: 36px;
  height: 36px;
  flex-shrink: 0;
  border-radius: var(--radius-md);
  background: var(--accent-muted);
  color: var(--accent-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-normal);
}

.brand-mark:hover {
  background: var(--accent-glow);
  transform: translateY(-1px);
}

.brand-text {
  display: flex;
  flex-direction: column;
  min-width: 0;
}

.brand-name {
  font-family: var(--font-display);
  font-size: 15px;
  font-weight: 650;
  color: var(--text-primary);
  letter-spacing: -0.01em;
  line-height: 1.3;
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.brand-tagline {
  font-size: 11px;
  font-weight: 400;
  color: var(--text-tertiary);
  letter-spacing: 0.02em;
  margin-top: 1px;
}

/* ===== Stats Bar ===== */
.stats-bar {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 0 20px 16px;
  margin-bottom: 4px;
}

.stat {
  display: flex;
  align-items: baseline;
  gap: 5px;
}

.stat-number {
  font-family: var(--font-mono);
  font-size: 15px;
  font-weight: 600;
  color: var(--accent-primary);
  line-height: 1;
}

.stat-name {
  font-size: 12px;
  font-weight: 400;
  color: var(--text-tertiary);
  letter-spacing: 0.02em;
}

.stat-divider {
  width: 1px;
  height: 14px;
  background: var(--border-default);
  border-radius: 1px;
}

/* ===== Body ===== */
.sidebar-body {
  flex: 1;
  overflow-y: auto;
  padding: 0 8px 8px;
}

.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px 0;
}

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--border-default);
  border-top-color: var(--accent-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 40px 0;
}

.empty-text {
  color: var(--text-muted);
  font-size: 13px;
}

/* ===== Footer ===== */
.sidebar-footer {
  padding: 10px 20px;
  border-top: 1px solid var(--border-subtle);
}

.footer-hint {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
  font-size: 11px;
  letter-spacing: 0.02em;
}

.footer-hint svg {
  flex-shrink: 0;
  opacity: 0.6;
}
</style>
