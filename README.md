# Ku Terminal

一个基于 [Tauri](https://v2.tauri.app/) + [Vue 3](https://vuejs.org/) 构建的现代终端应用程序，支持本地终端和 SSH 远程连接。

## 功能特性

- **多标签页终端** — 同时管理多个本地或远程终端会话
- **SSH 远程连接** — 支持密码和密钥认证，连接远程服务器
- **文件浏览器** — 浏览本地和远程文件系统，支持目录树导航
- **系统资源监控** — 实时显示 CPU、内存、磁盘使用情况（本地/远程）
- **会话管理** — 支持分组（Production / Staging / Development）、过滤、折叠
- **命令面板** — 快速执行操作的快捷入口
- **凭证管理** — 使用系统密钥链安全存储 SSH 密码
- **配置持久化** — 会话配置自动保存，支持版本迁移

## 技术栈

| 层 | 技术 |
|---|---|
| 前端 | Vue 3 + TypeScript + Pinia + UnoCSS |
| 终端 | [xterm.js](https://xtermjs.org/) |
| 后端 | Tauri v2 + Rust |
| SSH | [russh](https://github.com/aspect-build/russh) |
| 打包 | Vite + Tauri CLI |

## 快速开始

### 环境要求

- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) (最新稳定版)
- 系统依赖见 [Tauri 前置要求](https://v2.tauri.app/start/prerequisites/)

### 安装与运行

```bash
# 克隆项目
git clone https://github.com/your-username/ku-terminal.git
cd ku-terminal

# 安装依赖
pnpm install

# 启动开发模式
pnpm tauri dev
```

### 构建生产版本

```bash
pnpm tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`。

## 项目结构

```
ku-terminal/
├── src/                    # Vue 前端
│   ├── components/         # UI 组件
│   │   ├── TerminalView    # xterm.js 终端视图
│   │   ├── FileExplorer    # 文件浏览器
│   │   ├── ResourceMonitor # 系统监控面板
│   │   ├── SessionSidebar  # 会话侧边栏
│   │   ├── TabBar          # 标签页栏
│   │   └── CommandPalette  # 命令面板
│   ├── stores/             # Pinia 状态管理
│   └── lib/                # IPC 通信、主题等
├── src-tauri/              # Rust 后端
│   └── src/
│       ├── commands.rs     # Tauri 命令接口
│       ├── session.rs      # 会话管理器
│       ├── pty/            # 本地终端 PTY
│       ├── ssh/            # SSH 远程连接
│       ├── fs.rs           # 文件系统操作
│       ├── sysmon.rs       # 系统资源监控
│       ├── config.rs       # 配置管理
│       └── credentials.rs  # 凭证存储
└── package.json
```

## 开发

```bash
# 启动前端开发服务器（仅前端）
pnpm dev

# 启动 Tauri 开发模式（完整应用）
pnpm tauri dev

# 类型检查
pnpm build
```

## License

MIT
