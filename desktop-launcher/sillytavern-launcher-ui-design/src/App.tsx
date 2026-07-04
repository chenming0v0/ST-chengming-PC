import { useEffect, useRef, useState } from "react";
import TitleBar from "./components/TitleBar";
import Sidebar from "./components/Sidebar";
import LaunchPage from "./pages/LaunchPage";
import InstallPage from "./pages/InstallPage";
import TerminalPage from "./pages/TerminalPage";
import SettingsPage from "./pages/SettingsPage";
import type { Page, ServerStatus } from "./types";

const bootLogs: { text: string; delay: number }[] = [
  { text: "> node server.js --port 8000", delay: 200 },
  { text: "[启动器] 正在启动 SillyTavern 服务…", delay: 500 },
  { text: "[INFO] SillyTavern 1.13.4 release", delay: 900 },
  { text: "[INFO] Node.js v20.18.1 | platform: win32 x64", delay: 1200 },
  { text: "[INFO] Loading config.yaml…", delay: 1600 },
  { text: "[OK] ✓ 配置文件加载完成", delay: 2000 },
  { text: "[INFO] Loading extensions…", delay: 2400 },
  { text: "[OK] ✓ 已加载 12 个扩展插件", delay: 2900 },
  { text: "[INFO] Loading characters… (37 found)", delay: 3300 },
  { text: "[WARN] 未检测到 API 密钥，请在酒馆网页中配置 AI 后端连接", delay: 3700 },
  { text: "[OK] Launching SillyTavern…", delay: 4200 },
  { text: "[OK] ✓ SillyTavern is listening on: http://127.0.0.1:8000", delay: 4600 },
  { text: "[启动器] 启动成功！已在默认浏览器中打开酒馆页面。", delay: 5000 },
];

export default function App() {
  const [page, setPage] = useState<Page>("launch");
  const [dark, setDark] = useState(true);
  const [installed, setInstalled] = useState(false);
  const [status, setStatus] = useState<ServerStatus>("stopped");
  const [logs, setLogs] = useState<string[]>([]);
  const timers = useRef<number[]>([]);
  const heartbeat = useRef<number | null>(null);

  useEffect(() => {
    document.documentElement.classList.toggle("dark", dark);
  }, [dark]);

  useEffect(
    () => () => {
      timers.current.forEach(clearTimeout);
      if (heartbeat.current) clearInterval(heartbeat.current);
    },
    []
  );

  const launch = () => {
    if (status !== "stopped") return;
    setStatus("starting");
    setPage("terminal");
    setLogs([]);

    bootLogs.forEach(({ text, delay }) => {
      timers.current.push(
        window.setTimeout(() => setLogs((prev) => [...prev, text]), delay)
      );
    });

    timers.current.push(
      window.setTimeout(() => {
        setStatus("running");
        // 运行中的心跳日志
        heartbeat.current = window.setInterval(() => {
          const now = new Date().toLocaleTimeString("zh-CN", { hour12: false });
          setLogs((prev) => [
            ...prev,
            `[INFO] ${now} 服务运行正常 · 内存占用 ${(180 + Math.random() * 60).toFixed(1)} MB`,
          ]);
        }, 8000);
      }, 5300)
    );
  };

  const stop = () => {
    timers.current.forEach(clearTimeout);
    timers.current = [];
    if (heartbeat.current) {
      clearInterval(heartbeat.current);
      heartbeat.current = null;
    }
    setLogs((prev) => [
      ...prev,
      "[启动器] 正在停止服务…",
      "[INFO] Server shutdown complete.",
      "[启动器] 服务已停止。",
    ]);
    setStatus("stopped");
  };

  return (
    <div className="flex h-full flex-col overflow-hidden bg-zinc-50 text-zinc-900 dark:bg-[#1f1f1f] dark:text-zinc-100">
      <TitleBar />
      <div className="flex min-h-0 flex-1">
        <Sidebar
          page={page}
          setPage={setPage}
          dark={dark}
          toggleDark={() => setDark((d) => !d)}
          status={status}
        />
        <main className="min-w-0 flex-1 overflow-hidden">
          {page === "launch" && (
            <LaunchPage
              status={status}
              installed={installed}
              onLaunch={launch}
              onStop={stop}
              setPage={setPage}
            />
          )}
          {page === "install" && (
            <InstallPage installed={installed} onInstalled={() => setInstalled(true)} />
          )}
          {page === "terminal" && (
            <TerminalPage
              logs={logs}
              status={status}
              onClear={() => setLogs([])}
              onStop={stop}
              onLaunch={launch}
              installed={installed}
            />
          )}
          {page === "settings" && (
            <SettingsPage dark={dark} toggleDark={() => setDark((d) => !d)} />
          )}
        </main>
      </div>
    </div>
  );
}
