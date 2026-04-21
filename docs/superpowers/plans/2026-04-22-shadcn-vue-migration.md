# shadcn-vue 全面改版实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将 Super Workspace 从 Naive UI 完全迁移到 shadcn-vue + Tailwind CSS，同时保持所有现有功能不变。

**Architecture:** 保持 Sidebar + ProjectDetail 双栏布局和 Pinia store 数据结构不变，仅替换 UI 层。引入 Tailwind CSS v4 和 shadcn-vue CLI 管理组件，手写递归 Tree 替代 NTree。

**Tech Stack:** Tauri 2 + Vue 3 + TypeScript + Vite + Tailwind CSS v4 + shadcn-vue + lucide-vue-next

---

## 文件结构总览

| 文件 | 动作 | 说明 |
|---|---|---|
| `package.json` | 修改 | 移除 naive-ui、@vicons/ionicons5；新增 tailwindcss、@tailwindcss/vite、tailwindcss-animate、lucide-vue-next |
| `vite.config.ts` | 修改 | 追加 `@/` 路径别名，保留所有 Tauri server 配置 |
| `tsconfig.json` | 修改 | 追加 `paths` 和 `baseUrl` |
| `components.json` | 新建 | shadcn-vue 配置文件 |
| `src/assets/globals.css` | 新建 | Tailwind + shadcn-vue 全局样式，替代 index.html 内联样式 |
| `src/lib/utils.ts` | 新建 | cn 工具函数（shadcn-vue 标准） |
| `index.html` | 修改 | 移除所有内联 CSS 和脚本，保留基本 HTML 结构 |
| `src/main.ts` | 修改 | 移除 naive-ui，引入 globals.css |
| `src/stores/workspace.ts` | 修改 | 主题切换从 `data-theme` 改为 `class="dark"` |
| `src/App.vue` | 修改 | 移除所有自定义 scoped CSS，改用 Tailwind 工具类 |
| `src/components/ui/tree/Tree.vue` | 新建 | 顶层树组件 |
| `src/components/ui/tree/TreeNode.vue` | 新建 | 递归树节点组件 |
| `src/components/Sidebar/Sidebar.vue` | 修改 | 全面重写，使用 Tree + shadcn-vue 组件 |
| `src/components/ProjectDetail/ProjectDetail.vue` | 修改 | 全面重写，使用 shadcn-vue 组件 |
| `src/components/Common/SettingsPanel.vue` | 修改 | 用 shadcn-vue Dialog 重写 |

---

### Task 1: git 快照保护

**Files:**
- 仓库根目录

- [ ] **Step 1: 初始化 git 仓库并提交当前状态**

```bash
git init
git add -A
git commit -m "chore: snapshot before shadcn-vue migration"
```

Run: `git log --oneline -1`
Expected: `chore: snapshot before shadcn-vue migration`

---

### Task 2: 运行 shadcn-vue init

**Files:**
- `components.json` (新建)
- `vite.config.ts` (修改)
- `tsconfig.json` (修改)
- `src/assets/globals.css` (新建)
- `src/lib/utils.ts` (新建)

- [ ] **Step 1: 运行 init 命令**

```bash
npx shadcn-vue@latest init
```

在交互提示中选择：
- **Style:** Default
- **Base color:** Neutral (或 Stone，看喜好)
- **CSS variables:** Yes
- **TypeScript:** Yes

- [ ] **Step 2: 检查 vite.config.ts 是否被错误覆盖**

```bash
git diff vite.config.ts
```

Expected: 只应看到 `resolve.alias` 的新增，server 配置（port 1420、hmr、watch）必须保持原样。

如果 server 配置被覆盖，恢复并手动添加 alias：

```typescript
// vite.config.ts
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? { protocol: "ws", host, port: 1421 }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
```

- [ ] **Step 3: 检查 tsconfig.json 是否被错误覆盖**

```bash
git diff tsconfig.json
```

Expected: 只应看到 `baseUrl` 和 `paths` 的追加。如果原有 compilerOptions 被覆盖，手动合并：

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "preserve",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*.ts", "src/**/*.tsx", "src/**/*.vue"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

- [ ] **Step 4: 提交 init 结果**

```bash
git add -A
git commit -m "chore: init shadcn-vue"
```

---

### Task 3: 安装额外依赖并移除旧依赖

**Files:**
- `package.json`
- `pnpm-lock.yaml`

- [ ] **Step 1: 安装 lucide-vue-next 和 tailwindcss-animate**

```bash
pnpm add lucide-vue-next tailwindcss-animate
```

- [ ] **Step 2: 移除 naive-ui 和 @vicons/ionicons5**

```bash
pnpm remove naive-ui @vicons/ionicons5
```

- [ ] **Step 3: 验证 package.json**

检查 `dependencies` 中不应再有 `naive-ui` 和 `@vicons/ionicons5`，应有 `lucide-vue-next` 和 `tailwindcss-animate`。

```bash
cat package.json
```

- [ ] **Step 4: 提交**

```bash
git add package.json pnpm-lock.yaml
git commit -m "chore: replace naive-ui with shadcn-vue dependencies"
```

---

### Task 4: 配置全局样式和主题系统

**Files:**
- `index.html`
- `src/assets/globals.css`
- `src/stores/workspace.ts`

- [ ] **Step 1: 替换 index.html 为干净版本**

删除所有内联 `<style>` 和 `<script>`，保留字体加载（可选，也可移入 globals.css）。

```html
<!-- index.html -->
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Super Workspace</title>
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

- [ ] **Step 2: 更新 globals.css**

shadcn-vue init 已生成基础文件，需要补充：
1. 字体变量
2. 滚动条样式
3. 暗色模式的默认激活（因为当前默认暗色）
4. body 基础样式

```css
/* src/assets/globals.css */
@import "tailwindcss";

@plugin 'tailwindcss-animate';

@custom-variant dark (&:is(.dark *));

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --color-card: var(--card);
  --color-card-foreground: var(--card-foreground);
  --color-popover: var(--popover);
  --color-popover-foreground: var(--popover-foreground);
  --color-primary: var(--primary);
  --color-primary-foreground: var(--primary-foreground);
  --color-secondary: var(--secondary);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-muted: var(--muted);
  --color-muted-foreground: var(--muted-foreground);
  --color-accent: var(--accent);
  --color-accent-foreground: var(--accent-foreground);
  --color-destructive: var(--destructive);
  --color-destructive-foreground: var(--destructive-foreground);
  --color-border: var(--border);
  --color-input: var(--input);
  --color-ring: var(--ring);
  --color-radius: var(--radius);

  --font-sans: 'DM Sans', -apple-system, BlinkMacSystemFont, 'PingFang SC', 'Microsoft YaHei', sans-serif;
  --font-mono: 'JetBrains Mono', 'SF Mono', 'Fira Code', monospace;
}

@layer base {
  :root {
    --background: 0 0% 100%;
    --foreground: 240 10% 3.9%;
    --card: 0 0% 100%;
    --card-foreground: 240 10% 3.9%;
    --popover: 0 0% 100%;
    --popover-foreground: 240 10% 3.9%;
    --primary: 240 5.9% 10%;
    --primary-foreground: 0 0% 98%;
    --secondary: 240 4.8% 95.9%;
    --secondary-foreground: 240 5.9% 10%;
    --muted: 240 4.8% 95.9%;
    --muted-foreground: 240 3.8% 46.1%;
    --accent: 240 4.8% 95.9%;
    --accent-foreground: 240 5.9% 10%;
    --destructive: 0 84.2% 60.2%;
    --destructive-foreground: 0 0% 98%;
    --border: 240 5.9% 90%;
    --input: 240 5.9% 90%;
    --ring: 240 5.9% 10%;
    --radius: 0.5rem;
  }

  .dark {
    --background: 240 10% 3.9%;
    --foreground: 0 0% 98%;
    --card: 240 10% 3.9%;
    --card-foreground: 0 0% 98%;
    --popover: 240 10% 3.9%;
    --popover-foreground: 0 0% 98%;
    --primary: 0 0% 98%;
    --primary-foreground: 240 5.9% 10%;
    --secondary: 240 3.7% 15.9%;
    --secondary-foreground: 0 0% 98%;
    --muted: 240 3.7% 15.9%;
    --muted-foreground: 240 5% 64.9%;
    --accent: 240 3.7% 15.9%;
    --accent-foreground: 0 0% 98%;
    --destructive: 0 62.8% 30.6%;
    --destructive-foreground: 0 0% 98%;
    --border: 240 3.7% 15.9%;
    --input: 240 3.7% 15.9%;
    --ring: 240 4.9% 83.9%;
  }

  * {
    @apply border-border;
  }

  body {
    @apply bg-background text-foreground antialiased;
    font-family: var(--font-sans);
    overflow: hidden;
  }

  #app {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
  }

  ::-webkit-scrollbar {
    width: 5px;
    height: 5px;
  }

  ::-webkit-scrollbar-track {
    background: transparent;
  }

  ::-webkit-scrollbar-thumb {
    background: hsl(var(--border));
    border-radius: 10px;
  }

  ::-webkit-scrollbar-thumb:hover {
    background: hsl(var(--muted-foreground));
  }

  ::selection {
    background: hsl(var(--accent));
    color: hsl(var(--accent-foreground));
  }
}
```

- [ ] **Step 3: 更新 workspace store 主题逻辑**

```typescript
// src/stores/workspace.ts
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { TreeItem, ProjectConfig } from '../types';

export const useWorkspaceStore = defineStore('workspace', () => {
  const projects = ref<TreeItem[]>([]);
  const selectedProject = ref<TreeItem | null>(null);
  const isLoading = ref(false);
  const config = ref<ProjectConfig>({
    scanPath: '',
    theme: 'auto',
    sidebarWidth: 300
  });

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
    const html = document.documentElement;
    if (theme === 'dark') {
      html.classList.add('dark');
    } else {
      html.classList.remove('dark');
    }
  }

  function updateConfig(newConfig: Partial<ProjectConfig>) {
    config.value = { ...config.value, ...newConfig };
    localStorage.setItem('workspace-config', JSON.stringify(config.value));
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
    findProjectByPath
  };
});
```

- [ ] **Step 4: 更新 main.ts**

```typescript
// src/main.ts
import { createApp } from 'vue';
import { createPinia } from 'pinia';
import App from './App.vue';
import { useWorkspaceStore } from './stores/workspace';
import './assets/globals.css';

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
```

- [ ] **Step 5: 提交**

```bash
git add -A
git commit -m "feat: configure tailwind, shadcn-vue theme and globals"
```

---

### Task 5: 安装所需的 shadcn-vue 组件

**Files:**
- `src/components/ui/*` (多个新建)

- [ ] **Step 1: 逐个添加组件**

```bash
npx shadcn-vue@latest add button card badge dialog input label separator skeleton empty collapsible
```

- [ ] **Step 2: 验证组件已安装**

```bash
ls src/components/ui/
```

Expected: 目录中包含 button、card、badge、dialog、input、label、separator、skeleton、empty、collapsible 等子目录。

- [ ] **Step 3: 提交**

```bash
git add -A
git commit -m "chore: add shadcn-vue components"
```

---

### Task 6: 创建手写 Tree 组件

**Files:**
- `src/components/ui/tree/TreeNode.vue` (新建)
- `src/components/ui/tree/Tree.vue` (新建)

- [ ] **Step 1: 创建 TreeNode.vue**

```vue
<!-- src/components/ui/tree/TreeNode.vue -->
<script setup lang="ts">
import { ref, computed } from 'vue';
import { ChevronRight, Folder, FolderOpen, Code } from 'lucide-vue-next';

interface TreeNodeData {
  key: string;
  label: string;
  isProject: boolean;
  children?: TreeNodeData[];
}

const props = defineProps<{
  node: TreeNodeData;
  selectedKey?: string;
}>();

const emit = defineEmits<{
  select: [key: string];
}>();

const isExpanded = ref(true);
const hasChildren = computed(() => props.node.children && props.node.children.length > 0);
const isSelected = computed(() => props.selectedKey === props.node.key);

function toggleExpand() {
  if (hasChildren.value) {
    isExpanded.value = !isExpanded.value;
  }
}

function handleSelect() {
  emit('select', props.node.key);
}

function onChildSelect(key: string) {
  emit('select', key);
}
</script>

<template>
  <div>
    <div
      class="flex items-center gap-1 rounded-sm px-2 py-1.5 text-sm transition-colors cursor-pointer select-none"
      :class="[
        isSelected
          ? 'bg-accent text-accent-foreground font-medium'
          : 'text-foreground hover:bg-accent/50'
      ]"
      @click="handleSelect"
    >
      <!-- Expand toggle -->
      <button
        v-if="hasChildren"
        class="flex h-4 w-4 shrink-0 items-center justify-center text-muted-foreground transition-transform"
        :class="{ 'rotate-90': isExpanded }"
        @click.stop="toggleExpand"
      >
        <ChevronRight class="h-3.5 w-3.5" />
      </button>
      <span v-else class="h-4 w-4 shrink-0" />

      <!-- Icon -->
      <component
        :is="node.isProject ? Code : (isExpanded && hasChildren ? FolderOpen : Folder)"
        class="h-4 w-4 shrink-0 text-muted-foreground"
        :class="{ 'text-primary': isSelected }"
      />

      <!-- Label -->
      <span class="truncate">{{ node.label }}</span>
    </div>

    <!-- Children -->
    <div
      v-if="hasChildren && isExpanded"
      class="ml-4 border-l border-border pl-2"
    >
      <TreeNode
        v-for="child in node.children"
        :key="child.key"
        :node="child"
        :selected-key="selectedKey"
        @select="onChildSelect"
      />
    </div>
  </div>
</template>
```

- [ ] **Step 2: 创建 Tree.vue**

```vue
<!-- src/components/ui/tree/Tree.vue -->
<script setup lang="ts">
import TreeNode from './TreeNode.vue';

interface TreeNodeData {
  key: string;
  label: string;
  isProject: boolean;
  children?: TreeNodeData[];
}

const props = defineProps<{
  data: TreeNodeData[];
  selectedKey?: string;
}>();

const emit = defineEmits<{
  select: [key: string];
}>();

function handleSelect(key: string) {
  emit('select', key);
}
</script>

<template>
  <div class="space-y-0.5">
    <TreeNode
      v-for="node in data"
      :key="node.key"
      :node="node"
      :selected-key="selectedKey"
      @select="handleSelect"
    />
  </div>
</template>
```

- [ ] **Step 3: 提交**

```bash
git add -A
git commit -m "feat: add custom recursive Tree component"
```

---

### Task 7: 重写 Sidebar.vue

**Files:**
- `src/components/Sidebar/Sidebar.vue` (修改)

- [ ] **Step 1: 重写 Sidebar 组件**

```vue
<!-- src/components/Sidebar/Sidebar.vue -->
<script setup lang="ts">
import { computed } from 'vue';
import { useWorkspaceStore } from '../../stores/workspace';
import type { TreeItem } from '../../types';
import { LayoutGrid, Settings } from 'lucide-vue-next';
import { Skeleton } from '@/components/ui/skeleton';
import { Button } from '@/components/ui/button';
import Tree from '@/components/ui/tree/Tree.vue';
import SettingsPanel from '../Common/SettingsPanel.vue';

const workspaceStore = useWorkspaceStore();

interface TreeNodeData {
  key: string;
  label: string;
  isProject: boolean;
  children?: TreeNodeData[];
}

const treeData = computed(() => {
  const transform = (items: TreeItem[]): TreeNodeData[] => {
    return items.map(item => ({
      key: item.path,
      label: item.name,
      isProject: item.isProject,
      children: item.children ? transform(item.children) : undefined,
    }));
  };
  return transform(workspaceStore.projects);
});

const selectedKey = computed(() => workspaceStore.selectedProject?.path);

function handleSelect(key: string) {
  const selected = workspaceStore.findProjectByPath(key);
  if (selected) {
    workspaceStore.setSelectedProject(selected);
  }
}
</script>

<template>
  <aside class="flex w-[280px] shrink-0 flex-col border-r bg-card">
    <!-- Header -->
    <div class="flex items-center justify-between border-b p-4">
      <div class="flex items-center gap-3">
        <div class="flex h-9 w-9 items-center justify-center rounded-lg bg-accent text-accent-foreground">
          <LayoutGrid class="h-5 w-5" />
        </div>
        <div>
          <h1 class="text-sm font-semibold leading-tight">Super Workspace</h1>
          <p class="text-xs text-muted-foreground">工作空间管理</p>
        </div>
      </div>
      <SettingsPanel />
    </div>

    <!-- Stats -->
    <div class="flex items-center gap-4 border-b px-4 py-3 text-xs text-muted-foreground">
      <div class="flex items-baseline gap-1">
        <span class="text-sm font-semibold text-foreground">{{ workspaceStore.projectCount }}</span>
        <span>项目</span>
      </div>
      <div class="h-3 w-px bg-border" />
      <div class="flex items-baseline gap-1">
        <span class="text-sm font-semibold text-foreground">{{ workspaceStore.groupCount }}</span>
        <span>分组</span>
      </div>
    </div>

    <!-- Tree -->
    <div class="flex-1 overflow-y-auto p-2">
      <div v-if="workspaceStore.isLoading" class="space-y-2 p-2">
        <Skeleton v-for="i in 6" :key="i" class="h-6 w-full" />
      </div>
      <Tree
        v-else-if="treeData.length > 0"
        :data="treeData"
        :selected-key="selectedKey"
        @select="handleSelect"
      />
      <div v-else class="flex flex-col items-center justify-center py-10 text-muted-foreground">
        <LayoutGrid class="mb-2 h-8 w-8 opacity-50" />
        <p class="text-sm">暂无项目</p>
      </div>
    </div>

    <!-- Footer -->
    <div class="border-t px-4 py-3 text-xs text-muted-foreground">
      选择项目查看详情
    </div>
  </aside>
</template>
```

- [ ] **Step 2: 提交**

```bash
git add -A
git commit -m "feat: rewrite Sidebar with shadcn-vue and custom Tree"
```

---

### Task 8: 重写 SettingsPanel.vue

**Files:**
- `src/components/Common/SettingsPanel.vue` (修改)

- [ ] **Step 1: 重写 SettingsPanel 组件**

```vue
<!-- src/components/Common/SettingsPanel.vue -->
<script setup lang="ts">
import { ref, watch } from 'vue';
import { useWorkspaceStore } from '../../stores/workspace';
import { invoke } from '@tauri-apps/api/core';
import { Settings, X, FolderOpen, Sun, Moon, Monitor } from 'lucide-vue-next';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

const workspaceStore = useWorkspaceStore();
const open = ref(false);
const scanPath = ref(workspaceStore.config.scanPath);

watch(() => workspaceStore.config.scanPath, (newPath) => {
  scanPath.value = newPath;
});

async function saveSettings() {
  workspaceStore.updateConfig({
    scanPath: scanPath.value
  });
  open.value = false;
}

function setTheme(theme: 'light' | 'dark' | 'auto') {
  workspaceStore.updateConfig({ theme });
}

async function browsePath() {
  const defaultPath = await invoke<string>('get_default_projects_path');
  if (!scanPath.value) {
    scanPath.value = defaultPath;
  }
}

function onOpenChange(isOpen: boolean) {
  if (isOpen) {
    scanPath.value = workspaceStore.config.scanPath;
  }
  open.value = isOpen;
}
</script>

<template>
  <Dialog :open="open" @update:open="onOpenChange">
    <DialogTrigger as-child>
      <Button variant="ghost" size="icon" class="h-8 w-8">
        <Settings class="h-4 w-4" />
      </Button>
    </DialogTrigger>
    <DialogContent class="sm:max-w-[440px]">
      <DialogHeader>
        <DialogTitle class="flex items-center gap-2">
          <Settings class="h-5 w-5" />
          设置
        </DialogTitle>
        <DialogDescription>
          自定义应用的外观和扫描路径
        </DialogDescription>
      </DialogHeader>

      <div class="space-y-6 py-4">
        <!-- Theme -->
        <div class="space-y-3">
          <Label>外观</Label>
          <div class="grid grid-cols-3 gap-2">
            <Button
              variant="outline"
              :class="workspaceStore.config.theme === 'light' ? 'border-primary bg-accent' : ''"
              @click="setTheme('light')"
            >
              <Sun class="mr-2 h-4 w-4" />
              浅色
            </Button>
            <Button
              variant="outline"
              :class="workspaceStore.config.theme === 'dark' ? 'border-primary bg-accent' : ''"
              @click="setTheme('dark')"
            >
              <Moon class="mr-2 h-4 w-4" />
              深色
            </Button>
            <Button
              variant="outline"
              :class="workspaceStore.config.theme === 'auto' ? 'border-primary bg-accent' : ''"
              @click="setTheme('auto')"
            >
              <Monitor class="mr-2 h-4 w-4" />
              自动
            </Button>
          </div>
        </div>

        <!-- Path -->
        <div class="space-y-3">
          <Label for="scan-path">项目扫描路径</Label>
          <div class="flex gap-2">
            <Input
              id="scan-path"
              v-model="scanPath"
              placeholder="请输入项目目录路径"
              class="font-mono text-sm"
            />
            <Button variant="outline" size="icon" @click="browsePath">
              <FolderOpen class="h-4 w-4" />
            </Button>
          </div>
        </div>
      </div>

      <div class="flex justify-end gap-2">
        <Button variant="outline" @click="open = false">取消</Button>
        <Button @click="saveSettings">保存</Button>
      </div>
    </DialogContent>
  </Dialog>
</template>
```

- [ ] **Step 2: 提交**

```bash
git add -A
git commit -m "feat: rewrite SettingsPanel with shadcn-vue Dialog"
```

---

### Task 9: 重写 ProjectDetail.vue

**Files:**
- `src/components/ProjectDetail/ProjectDetail.vue` (修改)

- [ ] **Step 1: 重写 ProjectDetail 组件**

```vue
<!-- src/components/ProjectDetail/ProjectDetail.vue -->
<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { useWorkspaceStore } from '../../stores/workspace';
import { invoke } from '@tauri-apps/api/core';
import MarkdownIt from 'markdown-it';
import {
  FolderOpen,
  ExternalLink,
  GitBranch,
  Clock,
  User,
  LayoutGrid
} from 'lucide-vue-next';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Skeleton } from '@/components/ui/skeleton';

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

const gitStatusVariant = computed(() => {
  if (!gitInfo.value) return 'default';
  switch (gitInfo.value.status) {
    case 'clean': return 'default';
    case 'modified': return 'warning';
    case 'untracked': return 'destructive';
    default: return 'secondary';
  }
});

const gitStatusText = computed(() => {
  if (!gitInfo.value) return '';
  switch (gitInfo.value.status) {
    case 'clean': return '干净';
    case 'modified': return '已修改';
    case 'untracked': return '未跟踪';
    default: return '未知';
  }
});
</script>

<template>
  <main class="flex-1 overflow-hidden bg-background">
    <!-- Empty State -->
    <div
      v-if="!workspaceStore.selectedProject"
      class="flex h-full flex-col items-center justify-center gap-4 p-10"
    >
      <div class="flex h-20 w-20 items-center justify-center rounded-full border-2 border-dashed border-muted-foreground/30">
        <LayoutGrid class="h-8 w-8 text-muted-foreground/50" />
      </div>
      <h2 class="text-xl font-semibold">选择一个项目</h2>
      <p class="max-w-xs text-center text-sm text-muted-foreground">
        从左侧边栏选择一个项目，查看其详细信息和 README
      </p>
    </div>

    <!-- Detail Content -->
    <div v-else class="h-full overflow-y-auto px-8 py-8">
      <div class="mx-auto max-w-3xl space-y-8">
        <!-- Header -->
        <div class="flex items-start justify-between gap-4 border-b pb-6">
          <div class="flex items-start gap-4">
            <div class="flex h-11 w-11 shrink-0 items-center justify-center rounded-lg border bg-accent">
              <LayoutGrid class="h-5 w-5 text-accent-foreground" />
            </div>
            <div>
              <h1 class="text-2xl font-bold tracking-tight">
                {{ workspaceStore.selectedProject.name }}
              </h1>
              <p
                v-if="workspaceStore.selectedProject.description"
                class="mt-1 text-sm text-muted-foreground"
              >
                {{ workspaceStore.selectedProject.description }}
              </p>
            </div>
          </div>
          <div class="flex gap-2 shrink-0">
            <Button variant="outline" size="sm" @click="openInFinder">
              <FolderOpen class="mr-2 h-4 w-4" />
              Finder
            </Button>
            <Button size="sm" @click="openInVSCode">
              <ExternalLink class="mr-2 h-4 w-4" />
              VSCode
            </Button>
          </div>
        </div>

        <!-- Project Info -->
        <section>
          <h3 class="mb-4 text-sm font-semibold uppercase tracking-wider text-muted-foreground">
            项目信息
          </h3>
          <div class="grid grid-cols-1 gap-3 sm:grid-cols-2">
            <Card>
              <CardContent class="p-4">
                <p class="text-xs font-medium uppercase text-muted-foreground">路径</p>
                <p class="mt-1 break-all font-mono text-sm">{{ workspaceStore.selectedProject.path }}</p>
              </CardContent>
            </Card>
            <Card v-if="workspaceStore.selectedProject.version">
              <CardContent class="p-4">
                <p class="text-xs font-medium uppercase text-muted-foreground">版本</p>
                <p class="mt-1 font-mono text-sm">{{ workspaceStore.selectedProject.version }}</p>
              </CardContent>
            </Card>
          </div>
        </section>

        <!-- Git Info -->
        <section v-if="gitInfo">
          <h3 class="mb-4 text-sm font-semibold uppercase tracking-wider text-muted-foreground">
            Git
          </h3>
          <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
            <Card>
              <CardContent class="flex items-center gap-3 p-4">
                <GitBranch class="h-4 w-4 text-muted-foreground" />
                <div>
                  <p class="text-xs font-medium uppercase text-muted-foreground">分支</p>
                  <Badge variant="outline" class="mt-1 font-mono">
                    {{ gitInfo.branch || '—' }}
                  </Badge>
                </div>
              </CardContent>
            </Card>
            <Card>
              <CardContent class="flex items-center gap-3 p-4">
                <div class="h-4 w-4 rounded-full" :class="{
                  'bg-green-500': gitInfo.status === 'clean',
                  'bg-yellow-500': gitInfo.status === 'modified',
                  'bg-red-500': gitInfo.status === 'untracked'
                }" />
                <div>
                  <p class="text-xs font-medium uppercase text-muted-foreground">状态</p>
                  <Badge :variant="gitStatusVariant" class="mt-1">
                    {{ gitStatusText }}
                  </Badge>
                </div>
              </CardContent>
            </Card>
            <Card v-if="gitInfo.last_commit">
              <CardContent class="p-4">
                <p class="text-xs font-medium uppercase text-muted-foreground">最后提交</p>
                <p class="mt-1 font-mono text-sm truncate">{{ gitInfo.last_commit }}</p>
              </CardContent>
            </Card>
            <Card v-if="gitInfo.last_commit_author">
              <CardContent class="flex items-center gap-3 p-4">
                <User class="h-4 w-4 text-muted-foreground" />
                <div>
                  <p class="text-xs font-medium uppercase text-muted-foreground">作者</p>
                  <p class="mt-1 text-sm">{{ gitInfo.last_commit_author }}</p>
                </div>
              </CardContent>
            </Card>
            <Card v-if="gitInfo.last_commit_time" class="sm:col-span-2 lg:col-span-1">
              <CardContent class="flex items-center gap-3 p-4">
                <Clock class="h-4 w-4 text-muted-foreground" />
                <div>
                  <p class="text-xs font-medium uppercase text-muted-foreground">提交时间</p>
                  <p class="mt-1 text-sm">{{ gitInfo.last_commit_time }}</p>
                </div>
              </CardContent>
            </Card>
          </div>
        </section>

        <!-- README -->
        <section>
          <h3 class="mb-4 text-sm font-semibold uppercase tracking-wider text-muted-foreground">
            README
          </h3>
          <div v-if="isLoadingDetail" class="space-y-2">
            <Skeleton class="h-4 w-full" />
            <Skeleton class="h-4 w-5/6" />
            <Skeleton class="h-4 w-4/6" />
          </div>
          <Card v-else-if="readmeContent">
            <CardContent class="prose prose-sm dark:prose-invert max-w-none p-6">
              <div v-html="renderedReadme" />
            </CardContent>
          </Card>
          <div v-else class="flex flex-col items-center py-10 text-muted-foreground">
            <LayoutGrid class="mb-2 h-8 w-8 opacity-50" />
            <p class="text-sm">暂无 README 文件</p>
          </div>
        </section>
      </div>
    </div>
  </main>
</template>

<style scoped>
:deep(.prose h1) {
  @apply text-2xl font-bold tracking-tight mt-0 mb-4 pb-2 border-b;
}
:deep(.prose h2) {
  @apply text-xl font-semibold tracking-tight mt-6 mb-3 pb-1 border-b;
}
:deep(.prose h3) {
  @apply text-lg font-semibold mt-4 mb-2;
}
:deep(.prose p) {
  @apply text-sm text-muted-foreground leading-relaxed mb-4;
}
:deep(.prose code) {
  @apply font-mono text-xs bg-muted px-1.5 py-0.5 rounded;
}
:deep(.prose pre) {
  @apply bg-muted p-4 rounded-lg overflow-x-auto mb-4;
}
:deep(.prose pre code) {
  @apply bg-transparent p-0;
}
:deep(.prose ul, .prose ol) {
  @apply ml-5 mb-4 text-sm text-muted-foreground;
}
:deep(.prose li) {
  @apply mb-1;
}
:deep(.prose a) {
  @apply text-primary underline-offset-4 hover:underline;
}
:deep(.prose blockquote) {
  @apply border-l-2 border-primary pl-4 italic text-muted-foreground my-4;
}
:deep(.prose table) {
  @apply w-full text-sm border-collapse mb-4;
}
:deep(.prose th, .prose td) {
  @apply border px-3 py-2 text-left;
}
:deep(.prose th) {
  @apply bg-muted font-semibold;
}
:deep(.prose img) {
  @apply rounded-lg max-w-full my-4;
}
:deep(.prose hr) {
  @apply my-6 border-border;
}
</style>
```

- [ ] **Step 2: 提交**

```bash
git add -A
git commit -m "feat: rewrite ProjectDetail with shadcn-vue components"
```

---

### Task 10: 重写 App.vue

**Files:**
- `src/App.vue` (修改)

- [ ] **Step 1: 重写 App.vue**

```vue
<!-- src/App.vue -->
<script setup lang="ts">
import { onMounted } from 'vue';
import { useWorkspaceStore } from './stores/workspace';
import Sidebar from './components/Sidebar/Sidebar.vue';
import ProjectDetail from './components/ProjectDetail/ProjectDetail.vue';
import { invoke } from '@tauri-apps/api/core';
import type { TreeItem } from './types';

const workspaceStore = useWorkspaceStore();

onMounted(async () => {
  workspaceStore.loadConfig();

  if (!workspaceStore.config.scanPath) {
    const defaultPath = await invoke<string>('get_default_projects_path');
    workspaceStore.updateConfig({ scanPath: defaultPath });
  }

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
  <div class="flex h-screen overflow-hidden bg-background">
    <Sidebar />
    <ProjectDetail />
  </div>
</template>
```

- [ ] **Step 2: 提交**

```bash
git add -A
git commit -m "feat: simplify App.vue with tailwind utilities"
```

---

### Task 11: 构建验证

**Files:**
- 全部

- [ ] **Step 1: 运行构建验证**

```bash
pnpm run build
```

Expected: 无 TypeScript 错误，构建成功。

- [ ] **Step 2: 运行开发服务器快速验证**

```bash
pnpm run dev
```

在浏览器中打开 `http://localhost:1420`，检查：
- 侧边栏是否正确渲染项目树
- 点击项目是否能正确显示详情
- 设置弹窗是否正常打开
- 主题切换是否生效

- [ ] **Step 3: 提交最终版本**

```bash
git add -A
git commit -m "feat: complete shadcn-vue migration"
```

---

## 自检

### Spec 覆盖检查

| Spec 要求 | 实现 Task |
|---|---|
| 移除 naive-ui、@vicons | Task 3 |
| 引入 tailwindcss、lucide-vue-next | Task 2、3 |
| 配置 vite alias | Task 2 |
| 配置 tsconfig paths | Task 2 |
| 替换 index.html 全局样式 | Task 4 |
| 更新 main.ts | Task 4 |
| 更新 store 主题逻辑 | Task 4 |
| 手写 Tree 组件 | Task 6 |
| 重写 Sidebar | Task 7 |
| 重写 ProjectDetail | Task 9 |
| 重写 SettingsPanel (Dialog) | Task 8 |
| 重写 App.vue | Task 10 |
| 安装 shadcn-vue 组件 | Task 5 |

**无遗漏。**

### Placeholder 扫描

- 无 "TBD"、"TODO"、"implement later"
- 无 "Add appropriate error handling" 等模糊描述
- 所有代码步骤都包含完整代码
- 无 "Similar to Task N" 引用

### 类型一致性检查

- `TreeNodeData` 接口在 Task 6、7 中定义一致
- `TreeItem` 类型未改动，与原有类型兼容
- `gitInfo` 的字段名与后端返回一致（`last_commit`、`last_commit_author`、`last_commit_time`、`status`、`branch`）
- Badge variant 名称使用 shadcn-vue 标准：`default`、`secondary`、`destructive`、`outline`、`warning`

---

## 可能遇到的问题及解决

### 问题 1: shadcn-vue init 覆盖了 vite.config.ts 的 server 配置
**解决:** Task 2 Step 2 中通过 git diff 检查并手动恢复。

### 问题 2: Badge 没有 `warning` variant
**解决:** 如果 shadcn-vue 的 Badge 没有 warning variant，改用 `default` 并自定义 class（`class="bg-yellow-500/10 text-yellow-500"`）。

### 问题 3: Tree 组件递归导致性能问题
**解决:** 当前项目数量通常不多（<100），递归渲染不会造成性能问题。如后续扩展，可添加虚拟滚动。

### 问题 4: 主题切换时闪烁
**解决:** index.html 中移除了内联脚本，首次加载时主题由 main.ts 中的 `loadConfig()` 设置。如需避免闪烁，可在 index.html 中保留一个极简的内联脚本（只设置 dark class，不设置完整样式）。
