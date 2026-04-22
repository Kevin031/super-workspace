<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { useWorkspaceStore } from '../../stores/workspace';
import { invoke } from '@tauri-apps/api/core';
import MarkdownIt from 'markdown-it';

const workspaceStore = useWorkspaceStore();
const readmeContent = ref('');
const gitInfo = ref<any>(null);
const isLoadingDetail = ref(false);

const md = new MarkdownIt();

watch(() => workspaceStore.selectedProject, async (project) => {
  if (project) {
    await loadProjectDetail(project);
  } else {
    readmeContent.value = '';
    gitInfo.value = null;
  }
});

async function loadProjectDetail(project: any) {
  isLoadingDetail.value = true;
  try {
    const readme = await invoke('read_readme', { path: project.path });
    readmeContent.value = (readme as any).content;

    const git = await invoke('get_git_info', { path: project.path });
    gitInfo.value = git;
  } catch (error) {
    console.error('加载项目详情失败:', error);
  } finally {
    isLoadingDetail.value = false;
  }
}

async function openInFinder() {
  if (workspaceStore.selectedProject) {
    await invoke('open_in_finder', { path: workspaceStore.selectedProject.path });
  }
}

async function openInVSCode() {
  if (workspaceStore.selectedProject) {
    await invoke('open_in_vscode', { path: workspaceStore.selectedProject.path });
  }
}

const renderedReadme = computed(() => {
  if (!readmeContent.value) return '';
  return md.render(readmeContent.value);
});

const gitStatusMeta = computed(() => {
  if (!gitInfo.value) return { type: 'default', text: '' };
  switch (gitInfo.value.status) {
    case 'clean': return { type: 'clean', text: '干净' };
    case 'modified': return { type: 'modified', text: '已修改' };
    case 'untracked': return { type: 'untracked', text: '未跟踪' };
    default: return { type: 'default', text: '未知' };
  }
});
</script>

<template>
  <main class="project-detail">
    <!-- Empty State -->
    <div v-if="!workspaceStore.selectedProject" class="empty-state">
      <div class="empty-visual">
        <div class="empty-icon-ring">
          <svg width="40" height="40" viewBox="0 0 24 24" fill="none">
            <path d="M12 5v14M5 12h14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </div>
      </div>
      <h2 class="empty-title">选择一个项目</h2>
      <p class="empty-desc">从左侧边栏选择一个项目，查看其详细信息和 README</p>
    </div>

    <!-- Detail Content -->
    <div v-else class="detail-scroll">
      <div class="detail-inner">
        <!-- Project Header -->
        <header class="project-header">
          <div class="project-meta">
            <div class="project-icon">
              <svg width="22" height="22" viewBox="0 0 24 24" fill="none">
                <path d="M4 4h6v6H4zM14 4h6v6h-6zM4 14h6v6H4zM14 14h6v6h-6z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <div class="project-info">
              <h1 class="project-name">{{ workspaceStore.selectedProject.name }}</h1>
              <p v-if="workspaceStore.selectedProject.description" class="project-desc">
                {{ workspaceStore.selectedProject.description }}
              </p>
            </div>
          </div>
          <div class="project-actions">
            <button class="btn btn-secondary" @click="openInFinder">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none">
                <path d="M22 19a2 2 0 01-2 2H4a2 2 0 01-2-2V5a2 2 0 012-2h5l2 3h9a2 2 0 012 2z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              Finder
            </button>
            <button class="btn btn-primary" @click="openInVSCode">
              <svg width="15" height="15" viewBox="0 0 24 24" fill="none">
                <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
              VSCode
            </button>
          </div>
        </header>

        <!-- Info Section -->
        <section class="section">
          <div class="section-header">
            <div class="section-line" />
            <span class="section-label">项目信息</span>
            <div class="section-line" />
          </div>
          <div class="info-grid">
            <div class="info-cell">
              <span class="info-cell-label">路径</span>
              <span class="info-cell-value mono">{{ workspaceStore.selectedProject.path }}</span>
            </div>
            <div v-if="workspaceStore.selectedProject.version" class="info-cell">
              <span class="info-cell-label">版本</span>
              <span class="info-cell-value mono">{{ workspaceStore.selectedProject.version }}</span>
            </div>
          </div>
        </section>

        <!-- Git Section -->
        <section v-if="gitInfo" class="section">
          <div class="section-header">
            <div class="section-line" />
            <span class="section-label">Git</span>
            <div class="section-line" />
          </div>
          <div class="git-grid">
            <div class="git-cell">
              <span class="git-cell-label">分支</span>
              <span class="git-cell-value mono">
                <span class="git-branch-badge">
                  <svg width="10" height="10" viewBox="0 0 24 24" fill="none">
                    <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="2"/>
                    <path d="M12 3v6m0 6v6M21 12h-6M9 12H3" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
                  </svg>
                  {{ gitInfo.branch || '—' }}
                </span>
              </span>
            </div>
            <div class="git-cell">
              <span class="git-cell-label">状态</span>
              <span class="git-cell-value">
                <span class="status-dot" :class="'status-' + gitStatusMeta.type">
                  <span class="status-dot-marker"></span>
                  {{ gitStatusMeta.text }}
                </span>
              </span>
            </div>
            <div v-if="gitInfo.last_commit" class="git-cell">
              <span class="git-cell-label">最后提交</span>
              <span class="git-cell-value mono">{{ gitInfo.last_commit }}</span>
            </div>
            <div v-if="gitInfo.last_commit_author" class="git-cell">
              <span class="git-cell-label">作者</span>
              <span class="git-cell-value">{{ gitInfo.last_commit_author }}</span>
            </div>
            <div v-if="gitInfo.last_commit_time" class="git-cell">
              <span class="git-cell-label">提交时间</span>
              <span class="git-cell-value">{{ gitInfo.last_commit_time }}</span>
            </div>
          </div>
        </section>

        <!-- README Section -->
        <section class="section section-readme">
          <div class="section-header">
            <div class="section-line" />
            <span class="section-label">README</span>
            <div class="section-line" />
          </div>
          <div v-if="isLoadingDetail" class="loading-state">
            <div class="spinner" />
          </div>
          <div v-else-if="readmeContent" class="readme-rendered" v-html="renderedReadme" />
          <div v-else class="readme-empty">
            <span class="empty-text">暂无 README 文件</span>
          </div>
        </section>
      </div>
    </div>
  </main>
</template>

<style scoped>
.project-detail {
  flex: 1;
  overflow: hidden;
  background: var(--bg-primary);
  position: relative;
}

/* ===== Empty State ===== */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 20px;
  padding: 40px;
}

.empty-visual {
  position: relative;
}

.empty-icon-ring {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  border: 1.5px dashed var(--border-strong);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  animation: slow-rotate 8s linear infinite;
}

@keyframes slow-rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.empty-title {
  font-family: var(--font-display);
  font-size: 22px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  letter-spacing: -0.02em;
}

.empty-desc {
  font-size: 14px;
  color: var(--text-tertiary);
  margin: 0;
  text-align: center;
  max-width: 280px;
  line-height: 1.5;
}

/* ===== Scroll Area ===== */
.detail-scroll {
  height: 100%;
  overflow-y: auto;
  padding: 32px 36px 48px;
}

.detail-inner {
  max-width: 800px;
  margin: 0 auto;
}

/* ===== Project Header ===== */
.project-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 20px;
  margin-bottom: 36px;
  padding-bottom: 24px;
  border-bottom: 1px solid var(--border-subtle);
}

.project-meta {
  display: flex;
  align-items: flex-start;
  gap: 14px;
  min-width: 0;
}

.project-icon {
  width: 44px;
  height: 44px;
  flex-shrink: 0;
  border-radius: var(--radius-md);
  background: var(--surface-gradient), var(--accent-muted);
  color: var(--accent-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  border: 1px solid var(--border-subtle);
}

.project-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.project-name {
  font-family: var(--font-display);
  font-size: 24px;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  letter-spacing: -0.01em;
  line-height: 1.2;
  word-break: break-word;
}

.project-desc {
  font-size: 14px;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

.project-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

/* ===== Buttons ===== */
.btn {
  display: inline-flex;
  align-items: center;
  gap: 7px;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-family: var(--font-body);
  font-size: 13px;
  font-weight: 550;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid transparent;
  white-space: nowrap;
  position: relative;
  overflow: hidden;
}

.btn-secondary {
  background: transparent;
  color: var(--text-secondary);
  border-color: var(--border-default);
}

.btn-secondary:hover {
  background: #F5F0EB;
  color: var(--text-primary);
  border-color: var(--border-strong);
  transform: translateY(-1px);
}

[data-theme="dark"] .btn-secondary:hover {
  background: #44403C;
}

.btn-primary {
  background: linear-gradient(180deg, rgba(255,255,255,0.12) 0%, transparent 40%), var(--accent-primary);
  color: #fff;
  border-color: #92400E;
  box-shadow: 0 2px 0 #92400E, 0 4px 8px rgba(180, 83, 9, 0.2);
}

.btn-primary:hover {
  background: linear-gradient(180deg, rgba(255,255,255,0.18) 0%, transparent 40%), var(--accent-secondary);
  transform: translateY(-1px);
  box-shadow: 0 3px 0 #92400E, 0 6px 12px rgba(180, 83, 9, 0.25);
}

.btn-primary:active {
  transform: translateY(1px);
  box-shadow: 0 0 0 #92400E, 0 2px 4px rgba(180, 83, 9, 0.2);
}

/* ===== Sections ===== */
.section {
  margin-bottom: 32px;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 18px;
}

.section-line {
  flex: 1;
  height: 1px;
  background: var(--border-subtle);
}

.section-label {
  font-family: var(--font-display);
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.1em;
  white-space: nowrap;
}

/* ===== Info Grid ===== */
.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 12px;
}

.info-cell {
  display: flex;
  flex-direction: column;
  gap: 5px;
  padding: 14px 16px;
  background: var(--bg-secondary);
  border: none;
  border-radius: var(--radius-md);
  box-shadow: 0 2px 8px rgba(60, 40, 30, 0.04);
  transition: all var(--transition-fast);
}

[data-theme="dark"] .info-cell {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.info-cell:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(60, 40, 30, 0.08);
}

[data-theme="dark"] .info-cell:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
}

.info-cell-label {
  font-size: 11px;
  font-weight: 500;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

.info-cell-value {
  font-size: 13.5px;
  color: var(--text-primary);
  font-weight: 500;
  line-height: 1.4;
  word-break: break-all;
}

.info-cell-value.mono {
  font-family: var(--font-mono);
  font-size: 12.5px;
  color: var(--accent-primary);
}

/* ===== Git Grid ===== */
.git-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 12px;
}

.git-cell {
  display: flex;
  flex-direction: column;
  gap: 5px;
  padding: 14px 16px;
  background: var(--bg-secondary);
  border: none;
  border-radius: var(--radius-md);
  box-shadow: 0 2px 8px rgba(60, 40, 30, 0.04);
  transition: all var(--transition-fast);
}

[data-theme="dark"] .git-cell {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.git-cell:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(60, 40, 30, 0.08);
}

[data-theme="dark"] .git-cell:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
}

.git-cell-label {
  font-size: 11px;
  font-weight: 500;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

.git-cell-value {
  font-size: 13.5px;
  color: var(--text-primary);
  font-weight: 500;
  line-height: 1.4;
  word-break: break-word;
}

.git-cell-value.mono {
  font-family: var(--font-mono);
  font-size: 12.5px;
  color: var(--accent-primary);
}

.git-branch-badge {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 3px 10px;
  background: var(--accent-muted);
  border: 1px solid var(--accent-glow);
  border-radius: 20px;
  font-size: 12px;
  font-weight: 500;
  color: var(--accent-primary);
}

/* Status dots */
.status-dot {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.status-dot-marker {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  flex-shrink: 0;
}

.status-clean .status-dot-marker {
  background: #65A30D;
}

.status-modified .status-dot-marker {
  background: #B45309;
}

.status-untracked .status-dot-marker {
  background: #DC2626;
}

.status-default .status-dot-marker {
  background: var(--text-muted);
}

/* ===== README ===== */
.section-readme {
  margin-bottom: 0;
}

.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 48px 0;
}

.spinner {
  width: 24px;
  height: 24px;
  border: 2px solid var(--border-default);
  border-top-color: var(--accent-primary);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.readme-rendered {
  padding: 20px 24px;
  background: #FAFAF9;
  border: none;
  border-left: 3px solid var(--accent-primary);
  border-radius: 0 var(--radius-lg) var(--radius-lg) 0;
}

[data-theme="dark"] .readme-rendered {
  background: #1C1917;
}

.readme-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 48px 0;
}

.empty-text {
  color: var(--text-muted);
  font-size: 13px;
}

/* Markdown Styles */
.readme-rendered :deep(h1),
.readme-rendered :deep(h2),
.readme-rendered :deep(h3),
.readme-rendered :deep(h4) {
  font-family: var(--font-display);
  font-weight: 650;
  color: var(--text-primary);
  line-height: 1.25;
  margin-top: 28px;
  margin-bottom: 14px;
  letter-spacing: -0.02em;
}

.readme-rendered :deep(h1) {
  font-size: 1.75rem;
  margin-top: 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--border-subtle);
}

.readme-rendered :deep(h2) {
  font-size: 1.35rem;
  padding-bottom: 8px;
  border-bottom: 1px solid var(--border-subtle);
}

.readme-rendered :deep(h3) {
  font-size: 1.1rem;
}

.readme-rendered :deep(h4) {
  font-size: 0.95rem;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.readme-rendered :deep(p) {
  margin-bottom: 14px;
  line-height: 1.7;
  color: var(--text-secondary);
  font-size: 14.5px;
}

.readme-rendered :deep(code) {
  font-family: var(--font-mono);
  background: var(--bg-tertiary);
  color: var(--accent-primary);
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 12.5px;
  border: 1px solid var(--border-subtle);
  word-break: break-word;
}

.readme-rendered :deep(pre) {
  background: #F0EBE5;
  padding: 16px;
  border-radius: var(--radius-md);
  overflow-x: auto;
  margin-bottom: 16px;
  border: 1px dashed rgba(120, 80, 60, 0.15);
}

[data-theme="dark"] .readme-rendered :deep(pre) {
  background: #24201C;
  border-color: rgba(120, 80, 60, 0.20);
}

.readme-rendered :deep(pre code) {
  background: transparent;
  padding: 0;
  border: none;
  color: inherit;
  font-size: 13px;
}

.readme-rendered :deep(ul),
.readme-rendered :deep(ol) {
  margin-left: 20px;
  margin-bottom: 14px;
  color: var(--text-secondary);
}

.readme-rendered :deep(li) {
  margin-bottom: 6px;
  line-height: 1.6;
  font-size: 14px;
}

.readme-rendered :deep(a) {
  color: var(--accent-primary);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color var(--transition-fast);
}

.readme-rendered :deep(a:hover) {
  border-bottom-color: var(--accent-primary);
}

.readme-rendered :deep(img) {
  max-width: 100%;
  height: auto;
  border-radius: var(--radius-md);
  margin: 16px 0;
  border: 1px solid var(--border-subtle);
}

.readme-rendered :deep(table) {
  width: 100%;
  border-collapse: collapse;
  margin-bottom: 16px;
  font-size: 13px;
}

.readme-rendered :deep(th),
.readme-rendered :deep(td) {
  border: 1px solid var(--border-subtle);
  padding: 10px 14px;
  text-align: left;
}

.readme-rendered :deep(th) {
  background: var(--bg-tertiary);
  font-weight: 600;
  color: var(--text-primary);
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.readme-rendered :deep(td) {
  color: var(--text-secondary);
}

.readme-rendered :deep(blockquote) {
  border-left: 3px solid var(--accent-primary);
  padding-left: 16px;
  margin: 16px 0;
  color: var(--text-tertiary);
  font-style: italic;
  line-height: 1.6;
}

.readme-rendered :deep(hr) {
  border: none;
  border-top: 1px solid var(--border-subtle);
  margin: 24px 0;
}

/* Responsive */
@media (max-width: 700px) {
  .detail-scroll {
    padding: 20px 20px 32px;
  }

  .project-header {
    flex-direction: column;
    gap: 16px;
  }

  .info-grid,
  .git-grid {
    grid-template-columns: 1fr;
  }
}
</style>
