# 陶土奶油系主题改造 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将现有琥珀色扁平主题升级为陶土奶油系温暖有机风格，包含全新色彩系统、微立体按钮、悬浮卡片、毛玻璃模态等视觉改进。

**Architecture:** 纯前端样式改造，不改动业务逻辑。通过替换 CSS 变量、调整组件 scoped 样式实现。所有改动集中在 `index.html` 的全局变量和四个 Vue 组件的样式区块。

**Tech Stack:** Vue 3 + Vite + Scoped CSS + CSS 变量

---

### Task 1: 全局色彩系统 — 更新 index.html CSS 变量

**Files:**
- Modify: `index.html:21-102`（dark/light 主题变量块）
- Modify: `index.html:89-101`（common 变量中的圆角）

**Step 1: 重写 Dark Theme 色彩变量**

将 `[data-theme="dark"]` 块替换为：

```css
[data-theme="dark"] {
  --bg-primary: #1C1917;
  --bg-secondary: #292524;
  --bg-tertiary: #44403C;
  --bg-elevated: #57534E;
  --bg-hover: #57534E;
  --bg-active: #78716C;

  --text-primary: #FAFAF9;
  --text-secondary: #A8A29E;
  --text-tertiary: #78716C;
  --text-muted: #57534E;

  --accent-primary: #B45309;
  --accent-secondary: #D97706;
  --accent-muted: rgba(180, 83, 9, 0.12);
  --accent-glow: rgba(180, 83, 9, 0.25);

  --border-subtle: rgba(255, 245, 235, 0.06);
  --border-default: rgba(255, 245, 235, 0.10);
  --border-strong: rgba(255, 245, 235, 0.15);

  --shadow-sm: 0 1px 3px rgba(28, 25, 23, 0.4);
  --shadow-md: 0 4px 16px rgba(28, 25, 23, 0.5);
  --shadow-lg: 0 12px 40px rgba(28, 25, 23, 0.6);

  --glow-accent: 0 0 30px rgba(180, 83, 9, 0.2);

  --surface-gradient: linear-gradient(180deg, rgba(255,255,255,0.03) 0%, rgba(255,255,255,0) 100%);
}
```

**Step 2: 重写 Light Theme 色彩变量**

将 `[data-theme="light"]` 块替换为：

```css
[data-theme="light"] {
  --bg-primary: #F5F0EB;
  --bg-secondary: #FFFCF8;
  --bg-tertiary: #FAF6F1;
  --bg-elevated: #FFFFFF;
  --bg-hover: #EDE8E2;
  --bg-active: #E7E2DC;

  --text-primary: #292524;
  --text-secondary: #57534E;
  --text-tertiary: #A8A29E;
  --text-muted: #D6D0C9;

  --accent-primary: #B45309;
  --accent-secondary: #D97706;
  --accent-muted: rgba(180, 83, 9, 0.10);
  --accent-glow: rgba(180, 83, 9, 0.15);

  --border-subtle: rgba(60, 40, 30, 0.06);
  --border-default: rgba(60, 40, 30, 0.10);
  --border-strong: rgba(60, 40, 30, 0.15);

  --shadow-sm: 0 1px 3px rgba(60, 40, 30, 0.06);
  --shadow-md: 0 4px 16px rgba(60, 40, 30, 0.08);
  --shadow-lg: 0 12px 40px rgba(60, 40, 30, 0.10);

  --glow-accent: 0 0 30px rgba(180, 83, 9, 0.12);

  --surface-gradient: linear-gradient(180deg, rgba(0,0,0,0.02) 0%, rgba(0,0,0,0) 100%);
}
```

**Step 3: 更新 Common 圆角变量**

将 `:root` 中的圆角变量替换为：

```css
:root {
  /* ... fonts unchanged ... */

  --radius-sm: 8px;
  --radius-md: 12px;
  --radius-lg: 16px;
  --radius-xl: 24px;

  /* ... transitions unchanged ... */
}
```

**Step 4: Commit**

```bash
git add index.html
git commit -m "style(theme): 陶土奶油系色彩系统与圆角变量"
```

---

### Task 2: 更新 App.vue 背景纹理

**Files:**
- Modify: `src/App.vue:57-69`（背景纹理伪元素）

**Step 1: 重写背景光斑效果**

将 `.app-container::before` 替换为：

```css
.app-container::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background:
    radial-gradient(ellipse 50% 35% at 85% 10%, rgba(180, 83, 9, 0.08), transparent),
    radial-gradient(ellipse 40% 25% at 15% 90%, rgba(253, 230, 138, 0.05), transparent),
    radial-gradient(ellipse 60% 40% at 50% 100%, rgba(68, 64, 60, 0.04), transparent);
  pointer-events: none;
  z-index: 0;
}
```

**Step 2: Commit**

```bash
git add src/App.vue
git commit -m "style(app): 三光斑环境背景纹理"
```

---

### Task 3: 更新 Sidebar.vue 侧边栏风格

**Files:**
- Modify: `src/components/Sidebar/Sidebar.vue:101-285`（全部样式）

**Step 1: 重写侧边栏容器样式**

`.sidebar` 修改为（加圆角）：

```css
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
```

移除旧的 `.sidebar::after` 渐变边框伪元素。

**Step 2: 重写分割线为渐变**

在 `.sidebar` 样式末尾添加：

```css
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
```

**Step 3: 调整 header padding 和 brand-mark**

- `.sidebar-header` padding 改为 `24px 20px 18px`
- `.brand-mark` 圆角改为 `var(--radius-md)`（即 12px）
- `.brand-name` letter-spacing 改为 `-0.01em`

**Step 4: Commit**

```bash
git add src/components/Sidebar/Sidebar.vue
git commit -m "style(sidebar): 圆角侧边栏与渐变分割线"
```

---

### Task 4: 更新 TreeView.vue 节点交互

**Files:**
- Modify: `src/components/Common/TreeView.vue:79-163`（全部样式）

**Step 1: 为节点添加左侧指示器**

`.tree-node-content` 添加 `position: relative;`，并新增选中态指示器：

```css
.tree-node-content::before {
  content: '';
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 3px;
  height: 0;
  background: var(--accent-primary);
  border-radius: 0 2px 2px 0;
  transition: height var(--transition-fast);
}

.tree-node-selected::before {
  height: 60%;
}

.tree-node-selected {
  background: var(--accent-muted);
  transform: translateX(2px);
}
```

**Step 2: Commit**

```bash
git add src/components/Common/TreeView.vue
git commit -m "style(tree): 左侧赭石指示器与选中位移"
```

---

### Task 5: 更新 ProjectDetail.vue 按钮、卡片与状态标签

**Files:**
- Modify: `src/components/ProjectDetail/ProjectDetail.vue:197-753`（全部样式，配合 HTML 结构调整）

**Step 5a: 更新按钮为微立体风格**

`.btn` 基础样式增加 `position: relative; overflow: hidden;`

`.btn-secondary` 改为幽灵样式：

```css
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
```

`.btn-primary` 改为微立体：

```css
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
```

**Step 5b: 重写信息卡片为悬浮纸张效果**

`.info-cell` 和 `.git-cell` 统一改为：

```css
.info-cell,
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

[data-theme="dark"] .info-cell,
[data-theme="dark"] .git-cell {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

.info-cell:hover,
.git-cell:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(60, 40, 30, 0.08);
}

[data-theme="dark"] .info-cell:hover,
[data-theme="dark"] .git-cell:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
}
```

移除原来的 `border: 1px solid var(--border-subtle)` 和 hover 的 `border-color` 变化。

**Step 5c: 重写 Git 状态标签为色点样式**

修改模板中状态 pill 的 HTML 结构，改为色点+文字。在 `<template>` 中，将状态标签部分：

```html
<span class="status-pill" :class="'status-' + gitStatusMeta.type">
  {{ gitStatusMeta.text }}
</span>
```

替换为：

```html
<span class="status-dot" :class="'status-' + gitStatusMeta.type">
  <span class="status-dot-marker"></span>
  {{ gitStatusMeta.text }}
</span>
```

并替换对应的样式：

```css
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
```

**Step 5d: 重写 README 区域样式**

`.readme-rendered` 改为左侧竖线指示器样式：

```css
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
```

代码块样式改为虚线边框：

```css
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
```

**Step 5e: 空状态圆环动画改为慢速旋转**

替换 `.empty-icon-ring` 的动画：

```css
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
```

移除旧的 `@keyframes pulse-ring`。

**Step 5f: Commit**

```bash
git add src/components/ProjectDetail/ProjectDetail.vue
git commit -m "style(detail): 微立体按钮、悬浮卡片、色点状态、README 竖线风格"
```

---

### Task 6: 更新 SettingsPanel.vue 毛玻璃模态与主题预览

**Files:**
- Modify: `src/components/Common/SettingsPanel.vue:193-552`（模态和主题预览样式）

**Step 6a: 模态卡片改为毛玻璃**

`.modal-card` 添加：

```css
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
```

**Step 6b: 更新主题预览配色**

`.theme-preview-light` 和子元素改为新亮色配色：

```css
.theme-preview-light {
  background: #F5F0EB;
}

.theme-preview-light .theme-preview-bar {
  height: 10px;
  background: #E7E2DC;
}

.theme-preview-light .theme-preview-line {
  height: 3px;
  background: #D6D0C9;
  border-radius: 1px;
}
```

`.theme-preview-dark` 和子元素改为新暗色配色：

```css
.theme-preview-dark {
  background: #1C1917;
}

.theme-preview-dark .theme-preview-bar {
  height: 10px;
  background: #292524;
}

.theme-preview-dark .theme-preview-line {
  height: 3px;
  background: #44403C;
  border-radius: 1px;
}

.theme-preview-auto .light {
  background: #F5F0EB;
}

.theme-preview-auto .dark {
  background: #1C1917;
}
```

**Step 6c: Commit**

```bash
git add src/components/Common/SettingsPanel.vue
git commit -m "style(settings): 毛玻璃模态与主题预览新配色"
```

---

### Task 7: 验证与收尾

**Step 1: 启动开发服务器**

```bash
pnpm dev
```

**Step 2: 浏览器验证清单**

- [ ] 暗色主题下背景呈现三光斑纹理
- [ ] 侧边栏右侧有渐变分割线
- [ ] TreeView 节点 hover 有背景色，选中有左侧赭石指示器和右移
- [ ] 主按钮有微立体高光和底部投影，点击有按压感
- [ ] 次按钮为幽灵样式，hover 有暖色背景
- [ ] 信息卡片无显式边框，有弥散阴影，hover 微抬
- [ ] Git 状态显示为色点+文字（橄榄绿/赭石/砖红）
- [ ] README 区域左侧有赭石竖线，代码块有虚线边框
- [ ] 空状态圆环慢速旋转
- [ ] 设置模态有毛玻璃效果
- [ ] 主题预览小卡片颜色正确
- [ ] 亮色主题切换后一切正常

**Step 3: 最终 Commit**

```bash
git add .
git commit -m "style: 陶土奶油系主题改造完成"
```
