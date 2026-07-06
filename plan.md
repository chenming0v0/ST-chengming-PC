# ST Launcher 实现计划

## 目标

将 ST Launcher 从静态 mock UI 改造为功能完整的 SillyTavern 桌面启动器：
- 内联终端（真实 PTY）
- 安装 SillyTavern（自动下载便携 Node.js 24 + Git，git clone）
- 更新 SillyTavern（git pull）
- 启动 SillyTavern（node server.js）
- 所有依赖和数据仅在安装目录内，不污染系统环境

## 依赖源

| 组件 | 来源 |
|------|------|
| SillyTavern | `https://gitcode.com/GitHub_Trending/si/SillyTavern.git` |
| Node.js 24 | `https://nodejs.org/dist/` (win-x64 zip，便携版) |
| Git | `https://github.com/git-for-windows/git/releases` (PortableGit zip) |
| npm | 随 Node.js 自带 |

## 安装目录结构

```
<安装目录>/
├── ST-Launcher.exe
├── runtime/
│   ├── node/           # 便携 Node.js 24 (解压即用)
│   └── git/            # PortableGit (解压即用)
└── SillyTavern/        # git clone 的酒馆源码
```

## 实现步骤

### Phase 1: Tauri 后端核心功能

1. **路径管理模块** (`src-tauri/src/paths.rs`)
   - 获取应用安装目录（exe 所在目录）
   - 构建 runtime/node、runtime/git、SillyTavern 的绝对路径
   - 确保无论 exe 在哪里执行，路径都正确

2. **下载模块** (`src-tauri/src/download.rs`)
   - HTTP 下载 + 进度回调（通过 Tauri event 推送到前端）
   - 支持 zip 解压到目标目录
   - 断点续传（可选，后续优化）

3. **运行时管理** (`src-tauri/src/runtime.rs`)
   - 检测 Node.js/Git 是否已安装（检查 runtime/ 目录）
   - 下载并解压 Node.js 便携版
   - 下载并解压 PortableGit
   - 返回各工具的可执行文件路径

4. **SillyTavern 管理** (`src-tauri/src/tavern.rs`)
   - 安装：使用便携 git clone 酒馆仓库
   - 更新：使用便携 git pull
   - 启动：使用便携 node 运行 server.js
   - 状态查询：是否已安装、当前版本等

5. **PTY 终端** (`src-tauri/src/terminal.rs`)
   - 使用 `portable-pty` crate 创建伪终端
   - 支持多会话管理
   - 通过 Tauri event 双向通信（输入/输出）
   - 终端环境变量只包含本地 runtime 路径（PATH 只指向 runtime/node、runtime/git）

### Phase 2: Tauri Commands 注册

在 `src-tauri/src/lib.rs` 注册所有 command：
- `check_install_status` - 检查安装状态
- `install_runtime` - 下载安装 Node.js + Git
- `install_tavern` - clone SillyTavern
- `update_tavern` - git pull 更新
- `launch_tavern` - 启动酒馆
- `stop_tavern` - 停止酒馆
- `terminal_create` - 创建终端会话
- `terminal_write` - 向终端写入
- `terminal_resize` - 调整终端大小
- `terminal_close` - 关闭终端会话

### Phase 3: 前端改造

1. **启动器页面改造**
   - 安装状态检测和显示
   - "安装酒馆"按钮 → 触发完整安装流程
   - "更新酒馆"按钮 → git pull
   - "启动酒馆"按钮 → node server.js
   - 下载进度条

2. **终端页面改造**
   - 接入真实 PTY 输出（通过 Tauri event 监听）
   - 支持键盘输入
   - 支持 ANSI 转义序列渲染（使用 xterm.js 或纯 Rust 方案）

### Phase 4: 打包配置

- Tauri bundle 配置为 MSI
- MSI 安装后目录即为工作目录
- 不写注册表（除安装记录外）
- 不修改系统 PATH

## 关键依赖 (Cargo)

```toml
# src-tauri/Cargo.toml 新增
reqwest = { version = "0.12", features = ["stream"] }
tokio = { version = "1", features = ["full"] }
zip = "2"
portable-pty = "0.8"
```

## 注意事项

- 所有子进程的 PATH 环境变量只设置为 `runtime/node` 和 `runtime/git/bin`，不继承系统 PATH
- git clone/pull 使用 `runtime/git/bin/git.exe`
- node 使用 `runtime/node/node.exe`
- npm 使用 `runtime/node/npm.cmd`
- 终端会话的工作目录默认为 SillyTavern 目录
