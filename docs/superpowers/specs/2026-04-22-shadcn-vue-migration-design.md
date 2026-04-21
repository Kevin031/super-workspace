# shadcn-vue 全面改版设计文档

## 背景

当前项目使用 Naive UI 作为 Vue 3 组件库，UI 风格为自定义设计（紫色强调色、渐变背景、CSS 变量主题系统）。目标是将 Naive UI 完全替换为 shadcn-vue，同时彻底清理所有自定义 CSS，全面切换到 Tailwind CSS + shadcn-vue 的设计系统。

## 技术栈

- Tauri 2.x + Vue 3 + TypeScript + Vite
- Tailwind CSS v4 + @tailwindcss/vite
- shadcn-vue 组件库
- lucide-vue-next 图标库
- Pinia 状态管理

## 架构概览

整体保持现有页面结构不变：左侧 `Sidebar` + 右侧 `ProjectDetail` 的双栏布局，`SettingsPanel` 改为 shadcn-vue Dialog。核心改动如下：

- **依赖层**：移除 `naive-ui`、`@vicons/ionicons5`；引入 `tailwindcss`（v4 + @tailwindcss/vite）、`tailwindcss-animate`、`lucide-vue-next`
- **样式层**：删除所有组件级自定义 CSS，全局样式迁移至 `src/assets/globals.css`，采用 shadcn-vue 的标准 CSS 变量系统
- **组件层**：Naive UI 组件逐一替换为 shadcn-vue 对应组件；Tree 手写递归实现
- **图标层**：全部替换为 `lucide-vue-next`
- **主题层**：从 `data-theme` 切换改为 Tailwind 的 `.dark` class 切换，与 shadcn-vue 保持一致

## 组件映射表

| 当前（Naive UI / 自定义） | 替换为（shadcn-vue / 手写） | 用途 |
|---|---|---|
| NTree | 手写 RecursiveTree | 侧边栏项目树 |
| NIcon + @vicons | lucide-vue-next | 所有图标 |
| NEmpty | shadcn-vue Empty | 空状态展示 |
| NSpin | shadcn-vue Skeleton | 加载态 |
| 手写 Modal | shadcn-vue Dialog | 设置弹窗 |
| 手写 Button | shadcn-vue Button | 所有按钮 |
| 手写 info/git 卡片 | shadcn-vue Card | 信息展示卡片 |
| 手写 status pill | shadcn-vue Badge | Git 状态标签 |
| 手写 section 分割线 | shadcn-vue Separator | 区块分隔 |
| 手写 input | shadcn-vue Input | 路径输入框 |
| 手写 label | shadcn-vue Label | 表单标签 |

## 新增文件

- `components.json`
- `tailwind.config.ts`
- `src/assets/globals.css`
- `src/lib/utils.ts`（cn 工具）
- `src/components/ui/tree/Tree.vue`
- `src/components/ui/tree/TreeNode.vue`
- shadcn-vue CLI 生成的所有 UI 组件（Button、Card、Badge、Dialog、Input、Label、Separator、Skeleton、Empty、Collapsible 等）

## 修改文件

- `package.json` — 依赖替换
- `vite.config.ts` — 追加 `@/` 路径别名（server 配置保持原样）
- `tsconfig.json` — 追加 `paths` 和 `baseUrl`
- `src/main.ts` — 移除 naive-ui 引入，引入 globals.css
- `src/App.vue` — 移除所有自定义 CSS，改用 Tailwind 工具类
- `src/components/Sidebar/Sidebar.vue` — 全面重写，NTree 替换为 RecursiveTree，NIcon 替换为 Lucide
- `src/components/ProjectDetail/ProjectDetail.vue` — 全面重写，所有自定义样式替换为 shadcn-vue 组件和 Tailwind
- `src/components/Common/SettingsPanel.vue` — 用 shadcn-vue Dialog 重写，按钮和输入框替换
- `src/stores/workspace.ts` — 主题切换逻辑改为 Tailwind dark class
- `index.html` — 移除全局内联样式（如存在）

## 主题系统迁移

当前使用自定义 CSS 变量（`--bg-primary`、`--accent-primary` 等），替换为 shadcn-vue 标准变量：

| 原变量 | 新变量 |
|---|---|
| `--bg-primary` | `--background` |
| `--bg-secondary` | `--card` |
| `--text-primary` | `--foreground` |
| `--text-secondary` | `--muted-foreground` |
| `--accent-primary` | `--primary` |
| `--border-subtle` | `--border` |

暗色模式从 `data-theme="dark"` 改为 Tailwind 标准 `class="dark"` 在 html 元素上。主题切换函数在 `workspace.ts` 中更新为通过 `document.documentElement.classList.toggle('dark')` 实现。

## 手写 Tree 组件设计

shadcn-vue 未提供 Tree 组件，因此手写递归实现。

### 接口定义

```typescript
interface TreeNode {
  key: string
  label: string
  isProject: boolean
  children?: TreeNode[]
}
```

### 实现方案

- `Tree.vue`：顶层组件，接收 `data` prop，渲染根节点列表
- `TreeNode.vue`：递归组件，每个节点一个实例
  - 有子节点的分组节点：左侧 ChevronRight 图标，点击切换展开/收起（通过内部状态或传入的 expandedKeys）
  - 项目节点：左侧 Code 图标
  - 选中状态通过传入的 `selectedKey` 控制，点击时触发 `select` 事件
  - 样式完全使用 Tailwind 工具类（`hover:bg-accent`、`rounded-sm`、`px-2 py-1.5` 等）
  - 展开动画使用 CSS transition，不使用外部动画库

## shadcn-vue init 安全检查清单

1. 运行 init 前执行 `git add -A && git commit -m "before shadcn-vue init"`
2. 运行 `npx shadcn-vue@latest init`
3. init 后立刻执行 `git diff` 检查以下文件是否被错误修改：
   - `vite.config.ts` — 确保 server 配置（port 1420、hmr、watch 忽略 src-tauri）未被覆盖，只保留 alias 改动
   - `index.html` — CSP 和 viewport 保持原样
   - `tsconfig.json` — 追加 paths 即可，不要覆盖原有配置
4. 如有错误覆盖，用 `git checkout -- <file>` 恢复后手动添加必要配置

## 边界与约束

- 不改 Tauri 后端 Rust 代码
- 不改 Pinia store 的核心数据结构（`projects`、`selectedProject`、`config` 等）
- 不改 `TreeItem` 类型定义
- Markdown 渲染继续使用 `markdown-it`，只改样式适配
- 当前只有一个主题（暗色为主，支持 light/dark/auto 切换），迁移后保持相同能力

## 不在本次范围内的内容

- 新增功能（搜索、标签分类、终端集成等）
- 后端 API 扩展
- 多语言支持
- 构建流程改动（保持 `pnpm tauri dev` / `pnpm tauri build` 不变）
