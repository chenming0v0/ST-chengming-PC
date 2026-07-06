# ST Launcher (Tauri + Leptos)

前端：**Leptos** + **Trunk**  
桌面：**Tauri 2**（Rust）

本项目**不是** Node/React 模板，根目录原先没有 `package.json`，所以 `npm install` / `npm run tauri dev` 会报 ENOENT。

## 正确启动方式（推荐）

在项目根目录执行：

```powershell
cargo tauri dev
```

`tauri.conf.json` 里配置了 `beforeDevCommand: trunk serve`，会自动编译前端并打开窗口。

## 若提示找不到 trunk

安装 Trunk（只需一次）：

```powershell
cargo install trunk
```

## 用 npm 的等价命令（可选）

已添加 `package.json`，仅用于转发到 Cargo：

```powershell
npm run dev
# 或
npm run tauri -- dev
```

**不需要**为了跑应用而 `npm install`（没有前端 npm 依赖）。

## 仅编译前端

```powershell
trunk serve
# 浏览器访问 http://localhost:1420
```
