import { useEffect, useRef, useState } from "react";
import type { ServerStatus } from "../types";
import { cn } from "../utils/cn";

interface TerminalPageProps {
  logs: string[];
  status: ServerStatus;
  onClear: () => void;
  onStop: () => void;
  onLaunch: () => void;
  installed: boolean;
}

function colorize(line: string): string {
  if (line.includes("[ERROR]") || line.includes("error")) return "text-rose-400";
  if (line.includes("[WARN]")) return "text-amber-400";
  if (line.includes("[OK]") || line.includes("✓") || line.includes("Launching")) return "text-emerald-400";
  if (line.startsWith(">")) return "text-cyan-400";
  return "text-zinc-300";
}

export default function TerminalPage({
  logs,
  status,
  onClear,
  onStop,
  onLaunch,
  installed,
}: TerminalPageProps) {
  const bottomRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);

  useEffect(() => {
    if (autoScroll) bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [logs, autoScroll]);

  return (
    <div className="page-enter flex h-full flex-col p-6">
      <div className="flex shrink-0 items-center gap-3">
        <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">终端</h1>
        <span
          className={cn(
            "flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-xs font-medium",
            status === "running" && "bg-emerald-100 text-emerald-700 dark:bg-emerald-500/15 dark:text-emerald-400",
            status === "starting" && "bg-amber-100 text-amber-700 dark:bg-amber-500/15 dark:text-amber-400",
            status === "stopped" && "bg-zinc-100 text-zinc-500 dark:bg-zinc-700/50 dark:text-zinc-400"
          )}
        >
          <span
            className={cn(
              "h-1.5 w-1.5 rounded-full",
              status === "running" && "bg-emerald-500",
              status === "starting" && "animate-pulse bg-amber-500",
              status === "stopped" && "bg-zinc-400"
            )}
          />
          {status === "running" ? "运行中" : status === "starting" ? "启动中" : "已停止"}
        </span>

        <div className="flex-1" />

        <label className="flex cursor-pointer items-center gap-1.5 text-xs text-zinc-500 dark:text-zinc-400">
          <input
            type="checkbox"
            checked={autoScroll}
            onChange={(e) => setAutoScroll(e.target.checked)}
            className="accent-blue-500"
          />
          自动滚动
        </label>
        <button
          onClick={onClear}
          className="rounded-md border border-zinc-300 px-3 py-1.5 text-xs font-medium text-zinc-600 hover:bg-zinc-50 dark:border-zinc-600 dark:text-zinc-300 dark:hover:bg-zinc-800"
        >
          清空日志
        </button>
        {status === "stopped" ? (
          <button
            onClick={onLaunch}
            disabled={!installed}
            className={cn(
              "rounded-md px-3 py-1.5 text-xs font-medium text-white",
              installed ? "bg-blue-500 hover:bg-blue-600" : "cursor-not-allowed bg-zinc-400 dark:bg-zinc-600"
            )}
          >
            启动服务
          </button>
        ) : (
          <button
            onClick={onStop}
            className="rounded-md bg-rose-500 px-3 py-1.5 text-xs font-medium text-white hover:bg-rose-600"
          >
            停止服务
          </button>
        )}
      </div>

      {/* 终端窗口 */}
      <div className="mt-4 flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-zinc-300 bg-[#1e1e1e] shadow-inner dark:border-zinc-700">
        <div className="flex shrink-0 items-center gap-2 border-b border-zinc-700/70 bg-[#252526] px-4 py-2">
          <span className="h-3 w-3 rounded-full bg-rose-500/80" />
          <span className="h-3 w-3 rounded-full bg-amber-500/80" />
          <span className="h-3 w-3 rounded-full bg-emerald-500/80" />
          <span className="terminal-font ml-2 text-xs text-zinc-400">
            SillyTavern — node server.js
          </span>
        </div>
        <div className="terminal-font min-h-0 flex-1 overflow-y-auto p-4 text-[13px] leading-6">
          {logs.length === 0 ? (
            <p className="text-zinc-500">
              [启动器] 暂无日志输出。点击「启动服务」后，SillyTavern 的控制台输出将显示在这里。
            </p>
          ) : (
            logs.map((line, i) => (
              <p key={i} className={cn("whitespace-pre-wrap", colorize(line))}>
                {line}
              </p>
            ))
          )}
          {status !== "stopped" && (
            <p className="text-zinc-300">
              <span className="inline-block h-4 w-2 translate-y-0.5 animate-pulse bg-zinc-300" />
            </p>
          )}
          <div ref={bottomRef} />
        </div>
      </div>
    </div>
  );
}
