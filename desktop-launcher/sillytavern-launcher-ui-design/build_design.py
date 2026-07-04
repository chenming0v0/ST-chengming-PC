# -*- coding: utf-8 -*-
from pathlib import Path
import json, time

ROOT = Path(__file__).resolve().parent

APP_CSS = (ROOT / "partials" / "app.css").read_text(encoding="utf-8") if (ROOT / "partials" / "app.css").exists() else ""

def write_app_css():
    css = r"""
:root { --bg: #1f1f1f; --sidebar: #181818; --card: #2b2b2b; --border: rgba(63,63,70,.6); --muted: #a1a1aa; --text: #f4f4f5; --blue: #3b82f6; }
* { box-sizing: border-box; }
html, body { margin: 0; height: 100%; font-family: "Segoe UI", "Microsoft YaHei", system-ui, sans-serif; background: var(--bg); color: var(--text); }
.app { display: flex; flex-direction: column; height: 100vh; overflow: hidden; }
.row { display: flex; flex: 1; min-height: 0; }
.sidebar { width: 72px; flex-shrink: 0; background: var(--sidebar); border-right: 1px solid #27272a; display: flex; flex-direction: column; align-items: center; padding: 8px 0; }
.sb-logo { width: 56px; height: 56px; margin: 6px 8px 14px; flex-shrink: 0; display: flex; align-items: center; justify-content: center; pointer-events: none; }
.sb-logo img { width: 100%; height: 100%; object-fit: contain; border-radius: 8px; }
.sidebar .sb-nav#nav-launch { margin-top: 4px; }
.sb-nav { position: relative; width: 100%; padding: 10px 0; border: none; background: transparent; cursor: pointer; display: flex; flex-direction: column; align-items: center; gap: 4px; color: var(--muted); font-size: 11px; }
.sb-nav:hover { color: var(--text); }
.sb-nav.on { color: #60a5fa; }
.sb-nav.on::before { content: ""; position: absolute; left: 0; top: 50%; transform: translateY(-50%); width: 3px; height: 32px; background: var(--blue); border-radius: 0 4px 4px 0; }
.sb-nav svg { width: 20px; height: 20px; }
.sb-grow { flex: 1; }
.titlebar { height: 36px; flex-shrink: 0; display: flex; align-items: center; justify-content: space-between; padding: 0 8px 0 12px; background: var(--sidebar); }
.titlebar .brand { display: flex; align-items: center; gap: 8px; font-size: 13px; font-weight: 500; }
.titlebar .titlebar-logo { width: 24px; height: 24px; border-radius: 6px; object-fit: contain; flex-shrink: 0; display: block; }
.wc { display: flex; height: 36px; }
.wc button { width: 44px; border: none; background: transparent; color: var(--muted); }
.wc button:hover { background: #3f3f46; }
.wc .x:hover { background: #ef4444; color: #fff; }
.main { flex: 1; min-width: 0; overflow: hidden; }
.page { height: 100%; overflow: auto; padding: 24px; animation: fadeIn .25s ease-out; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(8px); } to { opacity: 1; transform: none; } }
.h1 { font-size: 20px; font-weight: 700; margin: 0; }
.sub { margin: 4px 0 0; font-size: 14px; color: var(--muted); }
.card { border: 1px solid var(--border); border-radius: 6px; background: var(--card); padding: 16px; }
.btn { border-radius: 6px; border: 1px solid #52525b; padding: 8px 14px; font-size: 13px; cursor: pointer; background: #27272a; color: var(--text); }
.btn-p { background: var(--blue); border-color: var(--blue); color: #fff; font-weight: 600; }
.btn-a { background: #f59e0b; border-color: #f59e0b; color: #fff; font-weight: 600; }
.btn-r { background: #ef4444; border-color: #ef4444; color: #fff; }
.btn-pill { flex: 1; padding: 10px; border-radius: 6px; border: 1px solid var(--border); font-size: 14px; background: transparent; color: var(--muted); cursor: pointer; }
.btn-pill.on { border-color: var(--blue); background: rgba(59,130,246,.1); color: #60a5fa; }
.banner { position: relative; height: 224px; border-radius: 8px; overflow: hidden; }
.banner img { width: 100%; height: 100%; object-fit: cover; }
.banner::after { content: ""; position: absolute; inset: 0; background: linear-gradient(90deg,rgba(0,0,0,.6),transparent); }
.banner-t { position: absolute; left: 32px; top: 50%; transform: translateY(-50%); color: #fff; z-index: 1; }
.banner-t h1 { margin: 8px 0 0; font-size: 28px; }
.grid-f { display: grid; grid-template-columns: repeat(2,1fr); gap: 12px; margin-top: 12px; }
@media (min-width: 1200px) { .grid-f { grid-template-columns: repeat(3,1fr); } }
.fitem { display: flex; align-items: center; gap: 12px; padding: 14px 16px; border: 1px solid var(--border); border-radius: 6px; background: var(--card); text-align: left; }
.fitem strong { display: block; font-size: 14px; }
.fitem span { font-size: 12px; color: var(--muted); }
.split { display: flex; gap: 24px; margin-top: 24px; }
.split-side { width: 288px; flex-shrink: 0; }
.page-launch { display: flex; flex-direction: column; height: 100%; min-height: 0; box-sizing: border-box; }
.page-launch .banner { flex-shrink: 0; }
.page-launch .split { flex: 1; min-height: 0; align-items: stretch; }
.launch-main { flex: 1; min-width: 0; display: flex; flex-direction: column; min-height: 0; }
.launch-main .launch-grow { flex: 1; min-height: 16px; }
.launch-meta { flex-shrink: 0; margin-top: auto; padding-top: 24px; font-size: 13px; color: var(--muted); }
.launch-meta div + div { margin-top: 4px; }
.launch-meta span.val { color: var(--text); }
.launch-side { width: 288px; flex-shrink: 0; display: flex; flex-direction: column; min-height: 0; }
.launch-side .ann { flex: 1; min-height: 120px; overflow-y: auto; margin-top: 12px; }
.launch-side-bottom { flex-shrink: 0; margin-top: auto; padding-top: 16px; }
.launch-side-bottom .cta { margin-top: 0; }
.ann { margin-top: 12px; max-height: 288px; overflow-y: auto; padding: 16px; border: 1px solid var(--border); border-radius: 6px; background: var(--card); font-size: 13px; line-height: 1.65; color: #d4d4d8; }
.ann p { margin: 0 0 12px; }
.cta { width: 100%; margin-top: 16px; padding: 14px; font-size: 16px; border: none; border-radius: 6px; cursor: pointer; display: flex; align-items: center; justify-content: center; gap: 8px; }
.grid3 { display: grid; grid-template-columns: repeat(3,1fr); gap: 16px; margin-top: 24px; }
.steps { margin-top: 24px; border: 1px solid var(--border); border-radius: 6px; background: var(--card); overflow: hidden; }
.step { display: flex; justify-content: space-between; padding: 14px 16px; border-bottom: 1px solid var(--border); font-size: 14px; }
.bar { height: 8px; background: #3f3f46; border-radius: 4px; margin-top: 16px; overflow: hidden; }
.bar > i { display: block; height: 100%; width: 35%; background: var(--blue); }
.term { margin-top: 16px; flex: 1; min-height: 360px; display: flex; flex-direction: column; border: 1px solid #52525b; border-radius: 8px; overflow: hidden; background: #1e1e1e; }
.term-h { padding: 8px 16px; background: #252526; border-bottom: 1px solid #3f3f46; font-size: 12px; color: var(--muted); display: flex; gap: 8px; align-items: center; }
.term-h i { width: 12px; height: 12px; border-radius: 50%; display: inline-block; }
.term-b { flex: 1; overflow: auto; padding: 16px; font-family: Consolas, monospace; font-size: 13px; line-height: 1.55; color: #d4d4d8; }
.c-cyan { color: #22d3ee; } .c-grn { color: #4ade80; } .c-amb { color: #fbbf24; }
.sec { margin-top: 24px; }
.sec h2 { font-size: 13px; font-weight: 600; color: var(--muted); margin: 0 0 8px; }
.sec-box { border: 1px solid var(--border); border-radius: 6px; background: var(--card); overflow: hidden; }
.srow { display: flex; align-items: center; justify-content: space-between; gap: 16px; padding: 16px 20px; border-bottom: 1px solid rgba(63,63,70,.4); width: 100%; }
.srow:last-child { border-bottom: none; }
.srow > div:first-child { flex: 1; min-width: 0; }
.srow .t { font-size: 14px; font-weight: 500; }
.srow .d { font-size: 12px; color: var(--muted); margin-top: 2px; }
input, select { background: transparent; border: 1px solid #52525b; border-radius: 6px; padding: 6px 12px; color: var(--text); font-size: 13px; }
.badge { display: inline-flex; align-items: center; gap: 6px; padding: 4px 10px; border-radius: 999px; font-size: 12px; background: rgba(34,197,94,.15); color: #4ade80; }
.toolbar { display: flex; align-items: center; gap: 12px; flex-wrap: wrap; }
.toolbar .sp { flex: 1; }

/* Settings page — match SettingsPage.tsx demo */
.page-settings { padding: 24px; }
.settings-head { display: flex; align-items: center; gap: 16px; width: 100%; margin-bottom: 0; }
.settings-head > div:first-child { flex: 1; min-width: 0; }
.settings-head .sp { flex: 1; }
.settings-head .h1 { font-size: 20px; font-weight: 700; color: #f4f4f5; }
.settings-head .sub { margin-top: 4px; font-size: 14px; color: #a1a1aa; }
.btn-save { flex-shrink: 0; border: none; border-radius: 6px; padding: 8px 20px; font-size: 14px; font-weight: 600; color: #fff; background: #3b82f6; cursor: pointer; transition: background .15s; }
.btn-save:hover { background: #2563eb; }
.btn-save.saved { background: #22c55e; }
.page-settings .sec { margin-top: 24px; }
.page-settings .sec h2 { margin: 0 0 8px; font-size: 14px; font-weight: 600; color: #a1a1aa; }
.page-settings .sec-box { border: 1px solid rgba(63,63,70,.6); border-radius: 6px; background: #2b2b2b; overflow: hidden; }
.page-settings .srow { display: flex; align-items: center; gap: 16px; padding: 16px 20px; border-bottom: 1px solid rgba(63,63,70,.5); }
.page-settings .srow:last-child { border-bottom: none; }
.page-settings .srow > div:first-child { flex: 1; min-width: 0; }
.page-settings .srow .t { font-size: 14px; font-weight: 500; color: #e4e4e7; margin: 0; }
.page-settings .srow .d { margin-top: 2px; font-size: 12px; color: #a1a1aa; }
.page-settings .srow .ctrl { flex-shrink: 0; display: flex; align-items: center; justify-content: flex-end; }
.page-settings .input-w { width: 11rem; max-width: 11rem; box-sizing: border-box; padding: 6px 12px; font-size: 14px; border: 1px solid #3f3f46; border-radius: 6px; background: transparent; color: #e4e4e7; outline: none; }
.page-settings .input-w:focus { border-color: #3b82f6; }
.page-settings select.input-w { background: #2b2b2b; cursor: pointer; }
.page-settings .toggle { position: relative; width: 44px; height: 24px; flex-shrink: 0; border: none; border-radius: 999px; background: #52525b; cursor: pointer; padding: 0; transition: background .2s; }
.page-settings .toggle.on { background: #3b82f6; }
.page-settings .toggle::after { content: ""; position: absolute; top: 2px; left: 2px; width: 20px; height: 20px; border-radius: 50%; background: #fff; box-shadow: 0 1px 2px rgba(0,0,0,.2); transition: transform .2s; }
.page-settings .toggle.on::after { transform: translateX(20px); }
.page-settings .btn-outline { border: 1px solid #52525b; border-radius: 6px; padding: 6px 12px; font-size: 12px; font-weight: 500; color: #d4d4d8; background: transparent; cursor: pointer; }
.page-settings .btn-outline:hover { background: #3f3f46; }
.page-settings .link-btn { font-size: 12px; font-weight: 500; color: #60a5fa; text-decoration: none; }
.page-settings .link-btn:hover { text-decoration: underline; }

"""
    (ROOT / "partials" / "app.css").write_text(css.strip(), encoding="utf-8")

SVG_PLAY = '<svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5.14v13.72c0 .8.87 1.3 1.56.88l11.14-6.86c.66-.4.66-1.36 0-1.76L9.56 4.26A1.03 1.03 0 0 0 8 5.14Z"/></svg>'
SVG_DL = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><path d="M12 3v12"/><path d="m7 10 5 5 5-5"/><path d="M4 19h16"/></svg>'
SVG_TERM = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="2" y="4" width="20" height="16" rx="2"/><path d="m7 9 3 3-3 3"/><path d="M13 15h4"/></svg>'
SVG_SET = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1Z"/></svg>'
SVG_MOON = '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79Z"/></svg>'
SVG_LOGO = '<svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor"><path d="M9.5 8.2v7.6c0 .62.68 1 1.2.68l6.1-3.8a.8.8 0 0 0 0-1.36l-6.1-3.8a.8.8 0 0 0-1.2.68Z"/></svg>'

def sidebar(active: str) -> str:
    def nav(key, label, svg, dom):
        cls = "sb-nav on" if active == key else "sb-nav"
        return f'<button type="button" class="{cls}" id="{dom}" data-dom-id="{dom}">{svg}{label}</button>'
    return f"""<aside class="sidebar">
<div class="sb-logo"><img src="../assets/chengming.png" alt="辰林"></div>
{nav("launch","启动",SVG_PLAY,"nav-launch")}
{nav("install","安装",SVG_DL,"nav-install")}
{nav("terminal","终端",SVG_TERM,"nav-terminal")}
<div class="sb-grow"></div>
<button type="button" class="sb-nav" id="nav-theme-toggle" title="夜间模式（原型）">{SVG_MOON}夜间</button>
{nav("settings","设置",SVG_SET,"nav-settings")}
</aside>"""

TITLEBAR = """<header class="titlebar"><div class="brand"><img class="titlebar-logo" src="../assets/chengming.png" alt="辰林">SillyTavern 启动器 1.0.0</div>
<div class="wc"><button type="button">?</button><button type="button">—</button><button type="button">□</button><button type="button" class="x">×</button></div></header>"""

def page(active, title, body, page_extra_class=""):
    extra = f" {page_extra_class}" if page_extra_class else ""
    return f"""<!DOCTYPE html>
<html lang="zh-CN" class="dark">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title}</title>
<link rel="stylesheet" href="../partials/app.css">
</head>
<body>
<div class="app">
{TITLEBAR}
<div class="row">
{sidebar(active)}
<main class="main"><div class="page{extra}">{body}</div></main>
</div>
</div>
</body>
</html>"""

LAUNCH = """
<div class="banner"><img src="../assets/bg.webp" alt=""><div class="banner-t"><p style="margin:0;font-size:14px;opacity:.9">SillyTavern</p><h1>酒馆 - 启动器</h1><p style="margin:8px 0 0;opacity:.9">与 AI 角色畅聊，让故事随心所欲！</p></div></div>
<div class="split"><div class="launch-main"><h2 class="h1" style="font-size:18px">文件夹</h2>
<div class="grid-f">
<div class="fitem"><div><strong>根目录</strong><span>.</span></div></div>
<div class="fitem"><div><strong>角色卡</strong><span>data/default-user/characters</span></div></div>
<div class="fitem"><div><strong>聊天记录</strong><span>data/default-user/chats</span></div></div>
<div class="fitem"><div><strong>世界书</strong><span>data/default-user/worlds</span></div></div>
<div class="fitem"><div><strong>扩展插件</strong><span>data/default-user/extensions</span></div></div>
</div>
<div class="launch-grow"></div>
<div class="launch-meta">
<div>启动器版本：<span class="val">1.0.0 Build 128</span></div>
<div>Node.js：<span class="val">v20.18.1 LTS</span></div>
<div>SillyTavern：<span class="val">1.13.4 release</span></div>
</div></div>
<div class="launch-side"><h2 class="h1" style="font-size:18px">公告</h2>
<div class="ann"><p>公告栏可滚动，请知晓以下全部内容。</p><p>SillyTavern 1.13.x 已发布，建议在「安装」页面检查更新后再启动。</p><p>首次使用请先前往「安装」完成环境部署。</p><p>启动后默认监听 http://127.0.0.1:8000。</p><p>请勿购买本软件教程，SillyTavern 完全免费开源。</p></div>
<div class="launch-side-bottom">
<button class="cta btn-p" type="button">一键启动</button>
<p style="text-align:center;font-size:12px;color:#4ade80;margin-top:8px">● 服务运行中 — http://127.0.0.1:8000（示意）</p>
<button class="btn" id="go-install" data-dom-id="go-install" type="button" style="width:100%;margin-top:8px">前往安装（未安装态入口）</button>
</div></div></div>"""

INSTALL = """
<h1 class="h1">安装 SillyTavern</h1>
<p class="sub">一键部署运行环境，自动完成 Node.js、Git 与 SillyTavern 本体的安装。</p>
<div class="grid3">
<div class="card"><strong>安装分支</strong><div style="display:flex;gap:8px;margin-top:12px"><button class="btn-pill on">release（稳定版）</button><button class="btn-pill">staging（测试版）</button></div></div>
<div class="card"><strong>下载源</strong><div style="display:flex;gap:8px;margin-top:12px"><button class="btn-pill on">国内镜像加速</button><button class="btn-pill">GitHub 官方源</button></div></div>
<div class="card"><strong>安装路径</strong><input type="text" value="D:\\SillyTavern" style="width:100%;margin-top:12px"></div>
</div>
<div class="card" style="margin-top:24px"><strong>安装进度</strong><div class="bar"><i></i></div>
<div class="steps" style="margin-top:16px;border:none">
<div class="step"><span>环境检测</span><span class="c-grn">完成</span></div>
<div class="step"><span>安装 Node.js</span><span class="c-amb">进行中</span></div>
<div class="step"><span>安装 Git</span><span style="color:var(--muted)">等待</span></div>
<div class="step"><span>克隆 SillyTavern</span><span style="color:var(--muted)">等待</span></div>
<div class="step"><span>安装依赖</span><span style="color:var(--muted)">等待</span></div>
<div class="step"><span>完成配置</span><span style="color:var(--muted)">等待</span></div>
</div>
<button class="btn btn-p" style="margin-top:16px">开始安装</button></div>"""

TERMINAL = """
<div class="toolbar"><h1 class="h1" style="margin:0">终端</h1><span class="badge"><span style="width:6px;height:6px;border-radius:50%;background:#22c55e"></span>运行中</span><span class="sp"></span>
<label style="font-size:12px;color:var(--muted)"><input type="checkbox" checked> 自动滚动</label>
<button class="btn">清空日志</button><button class="btn btn-r">停止服务</button></div>
<div class="term"><div class="term-h"><i style="background:#ef4444"></i><i style="background:#f59e0b"></i><i style="background:#22c55e"></i> SillyTavern — node server.js</div>
<div class="term-b">
<div class="c-cyan">&gt; node server.js --port 8000</div>
<div>[启动器] 正在启动 SillyTavern 服务…</div>
<div>[INFO] SillyTavern 1.13.4 release</div>
<div class="c-grn">[OK] ✓ 配置文件加载完成</div>
<div class="c-amb">[WARN] 未检测到 API 密钥，请在酒馆网页中配置 AI 后端连接</div>
<div class="c-grn">[OK] ✓ SillyTavern is listening on: http://127.0.0.1:8000</div>
<div>[启动器] 启动成功！已在默认浏览器中打开酒馆页面。</div>
</div></div>"""

SETTINGS = """
<div class="settings-head"><div><h1 class="h1">设置</h1><p class="sub">配置启动器与 SillyTavern 服务的运行参数。</p></div><span class="sp"></span><button type="button" class="btn-save" id="btn-save-settings">保存设置</button></div>
<div class="sec"><h2>网络</h2><div class="sec-box">
<div class="srow"><div><div class="t">服务端口</div><div class="d">SillyTavern 监听的端口号，默认 8000</div></div><div class="ctrl"><input type="text" class="input-w" value="8000"></div></div>
<div class="srow"><div><div class="t">局域网监听</div><div class="d">开启后局域网内其他设备可通过本机 IP 访问酒馆</div></div><div class="ctrl"><button type="button" class="toggle" aria-label="局域网监听" onclick="this.classList.toggle('on')"></button></div></div>
<div class="srow"><div><div class="t">IP 白名单</div><div class="d">仅允许白名单内的 IP 地址访问（whitelist.txt）</div></div><div class="ctrl"><button type="button" class="toggle on" aria-label="IP 白名单" onclick="this.classList.toggle('on')"></button></div></div>
<div class="srow"><div><div class="t">基础身份验证</div><div class="d">访问酒馆时需要输入用户名和密码（basicAuthMode）</div></div><div class="ctrl"><button type="button" class="toggle" aria-label="基础身份验证" onclick="this.classList.toggle('on')"></button></div></div>
<div class="srow"><div><div class="t">网络代理</div><div class="d">为 git / npm 下载配置 HTTP 代理，留空则不使用</div></div><div class="ctrl"><input type="text" class="input-w" placeholder="http://127.0.0.1:7890"></div></div>
</div></div>
<div class="sec"><h2>启动</h2><div class="sec-box">
<div class="srow"><div><div class="t">启动后自动打开浏览器</div><div class="d">服务启动完成后自动在默认浏览器中打开酒馆页面</div></div><div class="ctrl"><button type="button" class="toggle on" aria-label="自动打开浏览器" onclick="this.classList.toggle('on')"></button></div></div>
<div class="srow"><div><div class="t">启动前自动检查更新</div><div class="d">每次启动前检查 SillyTavern 是否有新版本</div></div><div class="ctrl"><button type="button" class="toggle on" aria-label="自动检查更新" onclick="this.classList.toggle('on')"></button></div></div>
<div class="srow"><div><div class="t">关闭窗口时</div><div class="d">点击关闭按钮后的行为</div></div><div class="ctrl"><select class="input-w"><option selected>最小化到托盘</option><option>退出并停止服务</option></select></div></div>
</div></div>
<div class="sec"><h2>外观</h2><div class="sec-box">
<div class="srow"><div><div class="t">夜间模式</div><div class="d">切换启动器的明暗主题（与侧边栏按钮同步）</div></div><div class="ctrl"><button type="button" class="toggle on" aria-label="夜间模式" onclick="this.classList.toggle('on')"></button></div></div>
<div class="srow"><div><div class="t">界面语言</div><div class="d">启动器界面显示语言</div></div><div class="ctrl"><select class="input-w"><option selected>简体中文</option><option>繁體中文</option><option>English</option><option>日本語</option></select></div></div>
</div></div>
<div class="sec"><h2>关于</h2><div class="sec-box">
<div class="srow"><div><div class="t">SillyTavern 启动器</div><div class="d">版本 1.0.0 Build 128 · 基于 MIT 协议开源，完全免费</div></div><div class="ctrl"><button type="button" class="btn-outline">检查启动器更新</button></div></div>
<div class="srow"><div><div class="t">开源仓库</div><div class="d">github.com/SillyTavern/SillyTavern</div></div><div class="ctrl"><a class="link-btn" href="https://github.com/SillyTavern/SillyTavern" target="_blank" rel="noreferrer">前往 GitHub →</a></div></div>
</div></div>
<div style="height:16px"></div>
<script>
(function(){
  var save = document.getElementById("btn-save-settings");
  if (save) save.addEventListener("click", function(){
    save.classList.add("saved");
    save.textContent = "✓ 已保存";
    setTimeout(function(){ save.classList.remove("saved"); save.textContent = "保存设置"; }, 1800);
  });
})();
</script>"""

write_app_css()
(ROOT / "partials" / "sidebar.html").write_text("<!-- 由 build_design.py 内联到各页；Tab domId: nav-launch nav-install nav-terminal nav-settings -->", encoding="utf-8")
(ROOT / "partials" / "titlebar.html").write_text(TITLEBAR, encoding="utf-8")

pages = [
    ("launch.html", "launch", "启动 - SillyTavern 启动器", LAUNCH, "page-launch"),
    ("install.html", "install", "安装 - SillyTavern 启动器", INSTALL),
    ("terminal.html", "terminal", "终端 - SillyTavern 启动器", TERMINAL),
    ("settings.html", "settings", "设置 - SillyTavern 启动器", SETTINGS, "page-settings"),
]
for item in pages:
    if len(item) == 5:
        fname, active, title, body, extra = item
        (ROOT / "pages" / fname).write_text(page(active, title, body, extra), encoding="utf-8")
    else:
        fname, active, title, body = item
        (ROOT / "pages" / fname).write_text(page(active, title, body), encoding="utf-8")

nav_dom = ["nav-launch", "nav-install", "nav-terminal", "nav-settings"]
pid_map = {d: "page-" + d.replace("nav-", "") for d in nav_dom}
ts = int(time.time() * 1000)
coords = [(0, 0), (640, 0), (640, 420), (640, 840)]
titles = {"page-launch": "启动器 - 启动", "page-install": "启动器 - 安装", "page-terminal": "启动器 - 终端", "page-settings": "启动器 - 设置"}
data = []
for i, pid in enumerate(["page-launch", "page-install", "page-terminal", "page-settings"]):
    inter = []
    for dom in nav_dom:
        tgt = pid_map[dom]
        if tgt != pid:
            inter.append({"domId": dom, "targetPageId": tgt, "hideEdge": False, "transitionLabel": dom})
    if pid == "page-launch":
        inter.append({"domId": "go-install", "targetPageId": "page-install", "hideEdge": False})
    data.append({
        "id": pid,
        "title": titles[pid],
        "type": "page",
        "version": 1,
        "createdAt": ts + i,
        "canvasData": {"x": coords[i][0], "y": coords[i][1], "group": 0},
        "devMetadata": {"htmlSrc": "pages/" + pid.replace("page-", "") + ".html", "interactions": inter},
    })

design = {
    "data": data,
    "config": {
        "autoLayout": True,
        "deviceType": "desktop",
        "projectName": "SillyTavern 启动器 UI",
        "designLibrary": {
            "name": "Vercel",
            "id": "dl_builtin_vercel",
            "version": None,
            "scope": "built-in-global",
            "path": r"c:\Users\HEI\.trae-cn\design_libraries\dl_builtin_vercel",
            "versionSource": "selected-context",
        },
        "showEdge": False,
    },
}
(ROOT / "sillytavern-launcher-ui-design.design").write_text(json.dumps(design, ensure_ascii=False, indent=2), encoding="utf-8")

(ROOT / "interactions.md").write_text("""# sillytavern-launcher-ui-design 交互

## 统一侧栏 Tab（各页一致）

| domId | targetPageId |
|-------|----------------|
| nav-launch | page-launch |
| nav-install | page-install |
| nav-terminal | page-terminal |
| nav-settings | page-settings |

## 启动页

| go-install | page-install |

`nav-theme-toggle` 仅原型展示，不写 interactions。
""", encoding="utf-8")

(ROOT / "PLAN.md").write_text("""# SillyTavern 启动器 UI 原型

- 源 demo：`src/` React（Launch / Install / Terminal / Settings）
- 交付：`assets/` `pages/` `partials/` + `sillytavern-launcher-ui-design.design`
- 重建：在本目录执行 `python build_design.py`
""", encoding="utf-8")

print("built", ROOT)
