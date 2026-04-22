<script setup lang="ts">
import { ref, watch } from 'vue';
import { useWorkspaceStore } from '../../stores/workspace';
import { invoke } from '@tauri-apps/api/core';

const workspaceStore = useWorkspaceStore();
const showSettings = ref(false);
const scanPath = ref(workspaceStore.config.scanPath);

watch(() => workspaceStore.config.scanPath, (newPath) => {
  scanPath.value = newPath;
});

async function saveSettings() {
  workspaceStore.updateConfig({
    scanPath: scanPath.value
  });
  showSettings.value = false;
}

function openSettings() {
  scanPath.value = workspaceStore.config.scanPath;
  showSettings.value = true;
}

function cancelSettings() {
  scanPath.value = workspaceStore.config.scanPath;
  showSettings.value = false;
}

function setTheme(theme: 'light' | 'dark' | 'auto') {
  workspaceStore.updateConfig({ theme });
}

async function browsePath() {
  // 尝试使用 Tauri 的文件选择（如果可用），否则保持手动输入
  const defaultPath = await invoke<string>('get_default_projects_path');
  if (!scanPath.value) {
    scanPath.value = defaultPath;
  }
}
</script>

<template>
  <div class="settings-panel">
    <!-- Trigger Button -->
    <button class="settings-trigger" @click="openSettings" title="设置">
      <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
        <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="1.5"/>
        <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 01-2.83 2.83l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 11-2.83-2.83l.06-.06A1.65 1.65 0 004.67 15 1.65 1.65 0 003 15v-.09a2 2 0 010-4 1.65 1.65 0 001.67-1 1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 112.83-2.83l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 112.83 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>

    <!-- Modal -->
    <div v-if="showSettings" class="modal">
      <div class="modal-backdrop" @click="cancelSettings"></div>
      <div class="modal-card">
        <!-- Header -->
        <div class="modal-header">
          <div class="modal-title-group">
            <div class="modal-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
                <circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="1.5"/>
                <path d="M19.4 15a1.65 1.65 0 00.33 1.82l.06.06a2 2 0 01-2.83 2.83l-.06-.06a1.65 1.65 0 00-1.82-.33 1.65 1.65 0 00-1 1.51V21a2 2 0 01-4 0v-.09A1.65 1.65 0 009 19.4a1.65 1.65 0 00-1.82.33l-.06.06a2 2 0 11-2.83-2.83l.06-.06A1.65 1.65 0 004.67 15 1.65 1.65 0 003 15v-.09a2 2 0 010-4 1.65 1.65 0 001.67-1 1.65 1.65 0 00-.33-1.82l-.06-.06a2 2 0 112.83-2.83l.06.06a1.65 1.65 0 001.82.33H9a1.65 1.65 0 001-1.51V3a2 2 0 014 0v.09a1.65 1.65 0 001 1.51 1.65 1.65 0 001.82-.33l.06-.06a2 2 0 112.83 2.83l-.06.06a1.65 1.65 0 00-.33 1.82V9a1.65 1.65 0 001.51 1H21a2 2 0 010 4h-.09a1.65 1.65 0 00-1.51 1z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
              </svg>
            </div>
            <h2 class="modal-title">设置</h2>
          </div>
          <button class="modal-close" @click="cancelSettings">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
              <path d="M18 6L6 18M6 6l12 12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
            </svg>
          </button>
        </div>

        <!-- Body -->
        <div class="modal-body">
          <!-- Theme Section -->
          <div class="form-section">
            <label class="form-label">外观</label>
            <div class="theme-options">
              <button
                class="theme-option"
                :class="{ active: workspaceStore.config.theme === 'light' }"
                @click="setTheme('light')"
              >
                <div class="theme-preview theme-preview-light">
                  <div class="theme-preview-bar"></div>
                  <div class="theme-preview-content">
                    <div class="theme-preview-line"></div>
                    <div class="theme-preview-line short"></div>
                  </div>
                </div>
                <span class="theme-name">浅色</span>
                <div v-if="workspaceStore.config.theme === 'light'" class="theme-check">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
                    <path d="M20 6L9 17l-5-5" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </div>
              </button>
              <button
                class="theme-option"
                :class="{ active: workspaceStore.config.theme === 'dark' }"
                @click="setTheme('dark')"
              >
                <div class="theme-preview theme-preview-dark">
                  <div class="theme-preview-bar"></div>
                  <div class="theme-preview-content">
                    <div class="theme-preview-line"></div>
                    <div class="theme-preview-line short"></div>
                  </div>
                </div>
                <span class="theme-name">深色</span>
                <div v-if="workspaceStore.config.theme === 'dark'" class="theme-check">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
                    <path d="M20 6L9 17l-5-5" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </div>
              </button>
              <button
                class="theme-option"
                :class="{ active: workspaceStore.config.theme === 'auto' }"
                @click="setTheme('auto')"
              >
                <div class="theme-preview theme-preview-auto">
                  <div class="theme-preview-half light"></div>
                  <div class="theme-preview-half dark"></div>
                </div>
                <span class="theme-name">跟随系统</span>
                <div v-if="workspaceStore.config.theme === 'auto'" class="theme-check">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
                    <path d="M20 6L9 17l-5-5" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                  </svg>
                </div>
              </button>
            </div>
          </div>

          <!-- Path Section -->
          <div class="form-section">
            <label class="form-label">项目扫描路径</label>
            <div class="path-input-group">
              <input
                v-model="scanPath"
                type="text"
                class="path-input"
                placeholder="请输入项目目录路径"
              />
              <button class="path-browse" @click="browsePath" title="使用默认路径">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
                  <path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
              </button>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="modal-footer">
          <button class="btn btn-secondary" @click="cancelSettings">取消</button>
          <button class="btn btn-primary" @click="saveSettings">保存</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-panel {
  position: relative;
}

.settings-trigger {
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.settings-trigger:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ===== Modal ===== */
.modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: modal-in var(--transition-normal);
}

@keyframes modal-in {
  from { opacity: 0; }
  to { opacity: 1; }
}

.modal-backdrop {
  position: absolute;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(6px);
}

.modal-card {
  position: relative;
  width: 440px;
  max-width: 90vw;
  background: var(--bg-secondary);
  border: 1px solid rgba(255, 250, 245, 0.08);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-lg);
  overflow: hidden;
  backdrop-filter: blur(20px) saturate(1.2);
  animation: card-in var(--transition-slow) cubic-bezier(0.16, 1, 0.3, 1);
}

[data-theme="light"] .modal-card {
  border-color: rgba(60, 40, 30, 0.08);
}

@keyframes card-in {
  from {
    opacity: 0;
    transform: translateY(12px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

/* ===== Header ===== */
.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px;
  border-bottom: 1px solid var(--border-subtle);
}

.modal-title-group {
  display: flex;
  align-items: center;
  gap: 12px;
}

.modal-icon {
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  background: var(--accent-muted);
  color: var(--accent-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.modal-title {
  font-family: var(--font-display);
  font-size: 17px;
  font-weight: 650;
  color: var(--text-primary);
  margin: 0;
  letter-spacing: -0.01em;
}

.modal-close {
  width: 28px;
  height: 28px;
  border-radius: var(--radius-sm);
  border: none;
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.modal-close:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* ===== Body ===== */
.modal-body {
  padding: 20px 24px;
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.form-label {
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.1em;
}

/* ===== Theme Options ===== */
.theme-options {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.theme-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px 8px;
  border-radius: var(--radius-md);
  border: 1.5px solid var(--border-subtle);
  background: var(--bg-tertiary);
  cursor: pointer;
  transition: all var(--transition-fast);
  position: relative;
}

.theme-option:hover {
  border-color: var(--border-default);
  background: var(--bg-hover);
}

.theme-option.active {
  border-color: var(--accent-primary);
  background: var(--accent-muted);
}

.theme-preview {
  width: 56px;
  height: 40px;
  border-radius: var(--radius-sm);
  overflow: hidden;
  border: 1px solid var(--border-default);
  position: relative;
}

.theme-preview-light {
  background: #F5F0EB;
}

.theme-preview-light .theme-preview-bar {
  height: 10px;
  background: #E7E2DC;
}

.theme-preview-light .theme-preview-content {
  padding: 5px 6px;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.theme-preview-light .theme-preview-line {
  height: 3px;
  background: #D6D0C9;
  border-radius: 1px;
}

.theme-preview-light .theme-preview-line.short {
  width: 60%;
}

.theme-preview-dark {
  background: #1C1917;
}

.theme-preview-dark .theme-preview-bar {
  height: 10px;
  background: #292524;
}

.theme-preview-dark .theme-preview-content {
  padding: 5px 6px;
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.theme-preview-dark .theme-preview-line {
  height: 3px;
  background: #44403C;
  border-radius: 1px;
}

.theme-preview-dark .theme-preview-line.short {
  width: 60%;
}

.theme-preview-auto {
  display: flex;
}

.theme-preview-auto .theme-preview-half {
  flex: 1;
}

.theme-preview-auto .light {
  background: #F5F0EB;
}

.theme-preview-auto .dark {
  background: #1C1917;
}

.theme-name {
  font-size: 12px;
  font-weight: 500;
  color: var(--text-secondary);
}

.theme-option.active .theme-name {
  color: var(--accent-primary);
  font-weight: 600;
}

.theme-check {
  position: absolute;
  top: 6px;
  right: 6px;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  background: var(--accent-primary);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* ===== Path Input ===== */
.path-input-group {
  display: flex;
  gap: 6px;
}

.path-input {
  flex: 1;
  padding: 9px 14px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: 13px;
  outline: none;
  transition: all var(--transition-fast);
}

.path-input::placeholder {
  color: var(--text-muted);
}

.path-input:hover {
  border-color: var(--border-default);
}

.path-input:focus {
  border-color: var(--accent-primary);
  box-shadow: 0 0 0 3px var(--accent-muted);
}

.path-browse {
  width: 38px;
  height: 38px;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-subtle);
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  flex-shrink: 0;
}

.path-browse:hover {
  background: var(--bg-hover);
  border-color: var(--border-default);
  color: var(--text-primary);
}

/* ===== Footer ===== */
.modal-footer {
  display: flex;
  gap: 8px;
  justify-content: flex-end;
  padding: 16px 24px;
  border-top: 1px solid var(--border-subtle);
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 18px;
  border-radius: var(--radius-md);
  font-family: var(--font-body);
  font-size: 13px;
  font-weight: 550;
  cursor: pointer;
  transition: all var(--transition-fast);
  border: 1px solid transparent;
}

.btn-secondary {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  border-color: var(--border-default);
}

.btn-secondary:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  border-color: var(--border-strong);
}

.btn-primary {
  background: var(--accent-muted);
  color: var(--accent-primary);
  border-color: var(--accent-glow);
}

.btn-primary:hover {
  background: var(--accent-glow);
  border-color: var(--accent-primary);
  box-shadow: var(--glow-accent);
}
</style>
