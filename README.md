# Super Workspace - 工作空间管理应用

一个基于 Tauri + Vue 3 + TypeScript 开发的桌面工作空间管理应用，帮助开发者更好地管理和查看多个项目。

## 功能特性

- 🔍 **智能项目扫描** - 自动扫描指定目录，识别项目（包含 package.json）和分组
- 📁 **项目树形展示** - 左侧边栏以树形结构展示项目和分组
- 📖 **README 预览** - 点击项目可查看 README.md 内容（支持 Markdown 渲染）
- 🔧 **Git 信息展示** - 显示项目的分支、最后提交信息、工作区状态等
- 🚀 **快速打开** - 一键在文件管理器/Finder 或 VSCode 中打开项目
- ⚙️ **配置管理** - 支持自定义扫描路径、主题等配置

## 技术栈

- **桌面框架**：Tauri 2.x
- **前端框架**：Vue 3 + TypeScript + Composition API
- **UI 组件库**：Naive UI
- **状态管理**：Pinia
- **构建工具**：Vite
- **Markdown 渲染**：markdown-it
- **后端语言**：Rust

## 开发指南

### 环境要求

- Node.js 16+
- pnpm 8+
- Rust 1.70+
- Tauri CLI

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建应用

```bash
pnpm tauri build
```

## 项目结构

```
super-workspace/
├── src/                    # 前端源代码
│   ├── components/         # Vue 组件
│   │   ├── Sidebar/       # 侧边栏组件
│   │   ├── ProjectDetail/ # 项目详情组件
│   │   └── Common/        # 通用组件
│   ├── stores/            # Pinia 状态管理
│   ├── types/             # TypeScript 类型定义
│   └── utils/             # 工具函数
├── src-tauri/             # Tauri 后端（Rust）
│   ├── src/
│   │   ├── lib.rs        # 主要逻辑和命令
│   │   └── main.rs       # 入口文件
│   └── Cargo.toml        # Rust 依赖配置
└── package.json          # Node.js 依赖配置
```

## 使用说明

1. **启动应用** - 应用会自动扫描默认目录（~/Projects）下的项目
2. **浏览项目** - 左侧边栏显示所有项目和分组，点击可查看详情
3. **查看项目信息** - 右侧面板显示项目的 README、Git 信息等
4. **打开项目** - 点击"在Finder中打开"或"用Code打开"按钮快速操作
5. **配置设置** - 点击右上角设置图标可自定义扫描路径等

## Tauri 命令说明

后端提供了以下命令供前端调用：

- `scan_directory` - 扫描指定目录
- `read_readme` - 读取项目的 README 文件
- `get_git_info` - 获取项目的 Git 信息
- `open_in_finder` - 在文件管理器中打开项目
- `open_in_vscode` - 用 VSCode 打开项目
- `get_default_projects_path` - 获取默认项目路径

## 未来扩展计划

- [ ] 添加项目搜索功能
- [ ] 支持多级目录扫描
- [ ] 添加项目标签和分类
- [ ] 集成终端功能
- [ ] 添加项目统计和周报功能
- [ ] 支持自定义主题和布局
- [ ] 添加项目快速启动功能
- [ ] 集成 Git 操作功能

## 开发建议

### IDE 推荐配置

- [VS Code](https://code.visualstudio.com/)
- [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar)
- [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 许可证

MIT License
