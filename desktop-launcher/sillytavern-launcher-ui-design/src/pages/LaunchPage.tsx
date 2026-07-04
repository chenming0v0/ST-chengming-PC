import type { Page, ServerStatus } from "../types";
import { cn } from "../utils/cn";

interface LaunchPageProps {
  status: ServerStatus;
  installed: boolean;
  onLaunch: () => void;
  onStop: () => void;
  setPage: (p: Page) => void;
}

const folders = [
  {
    name: "根目录",
    path: ".",
    icon: (
      <svg className="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.6} strokeLinecap="round" strokeLinejoin="round">
        <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z" />
      </svg>
    ),
  },
  {
    name: "角色卡",
    path: "data/default-user/characters",
    icon: (
      <svg className="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.6} strokeLinecap="round" strokeLinejoin="round">
        <circle cx="12" cy="8" r="4" />
        <path d="M4 21c0-4 3.6-6.5 8-6.5s8 2.5 8 6.5" />
      </svg>
    ),
  },
  {
    name: "聊天记录",
    path: "data/default-user/chats",
    icon: (
      <svg className="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.6} strokeLinecap="round" strokeLinejoin="round">
        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2Z" />
      </svg>
    ),
  },
  {
    name: "世界书",
    path: "data/default-user/worlds",
    icon: (
      <svg className="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.6} strokeLinecap="round" strokeLinejoin="round">
        <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20" />
        <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2Z" />
      </svg>
    ),
  },
  {
    name: "扩展插件",
    path: "data/default-user/extensions",
    icon: (
      <svg className="h-6 w-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.6} strokeLinecap="round" strokeLinejoin="round">
        <path d="M20.5 11H19V7a2 2 0 0 0-2-2h-4V3.5a2.5 2.5 0 0 0-5 0V5H4a2 2 0 0 0-2 2v3.8h1.5a2.7 2.7 0 0 1 0 5.4H2V20a2 2 0 0 0 2 2h3.8v-1.5a2.7 2.7 0 0 1 5.4 0V22H17a2 2 0 0 0 2-2v-4h1.5a2.5 2.5 0 0 0 0-5Z" />
      </svg>
    ),
  },
];

const announcements = [
  "公告栏可滚动，请知晓以下全部内容。",
  "SillyTavern 1.13.x 已发布，建议在「安装」页面检查更新后再启动。",
  "首次使用请先前往「安装」页面完成环境部署（Node.js + Git + SillyTavern 本体）。",
  "启动后默认监听 http://127.0.0.1:8000，可在「设置」页面修改端口与监听地址。",
  "请勿从任何渠道购买本软件与教程，SillyTavern 是完全免费的开源项目。",
  "遇到问题请先查看「终端」页面的日志输出，大部分报错都能从日志中找到原因。",
];

export default function LaunchPage({ status, installed, onLaunch, onStop, setPage }: LaunchPageProps) {
  return (
    <div className="page-enter flex h-full flex-col overflow-y-auto p-6">
      {/* 横幅 */}
      <div className="relative h-56 shrink-0 overflow-hidden rounded-lg">
        <img src="/images/banner.jpg" alt="banner" className="h-full w-full object-cover" />
        <div className="absolute inset-0 bg-gradient-to-r from-black/60 via-black/20 to-transparent" />
        <div className="absolute top-1/2 left-8 -translate-y-1/2 text-white">
          <p className="text-sm font-medium opacity-90">SillyTavern</p>
          <h1 className="mt-1 text-3xl font-bold tracking-wide">酒馆 - 启动器</h1>
          <p className="mt-2 text-base opacity-90">与 AI 角色畅聊，让故事随心所欲！</p>
        </div>
        <span className="absolute right-3 bottom-2 text-[11px] text-white/60">
          SillyTavern Launcher
        </span>
      </div>

      <div className="mt-6 flex flex-1 gap-6">
        {/* 左侧：文件夹 */}
        <div className="flex min-w-0 flex-1 flex-col">
          <h2 className="text-lg font-bold text-zinc-900 dark:text-zinc-100">文件夹</h2>
          <div className="mt-3 grid grid-cols-2 gap-3 xl:grid-cols-3">
            {folders.map((f) => (
              <button
                key={f.name}
                className="group flex items-center gap-3 rounded-md border border-zinc-200 bg-white px-4 py-3.5 text-left transition-colors hover:border-zinc-300 hover:bg-zinc-50 dark:border-zinc-700/60 dark:bg-[#2b2b2b] dark:hover:border-zinc-600 dark:hover:bg-[#333]"
              >
                <span className="text-zinc-700 dark:text-zinc-300">{f.icon}</span>
                <span className="min-w-0 flex-1">
                  <span className="block text-sm font-medium text-zinc-900 dark:text-zinc-100">
                    {f.name}
                  </span>
                  <span className="block truncate text-xs text-zinc-500 dark:text-zinc-400">
                    {f.path}
                  </span>
                </span>
                <svg
                  className="h-4 w-4 shrink-0 text-zinc-400 transition-transform group-hover:translate-x-0.5"
                  viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round"
                >
                  <path d="m9 18 6-6-6-6" />
                </svg>
              </button>
            ))}
          </div>

          <div className="flex-1" />

          {/* 版本信息 */}
          <div className="mt-6 space-y-1 text-[13px] text-zinc-600 dark:text-zinc-400">
            <div className="flex gap-6">
              <span className="w-28 shrink-0">启动器版本：</span>
              <span className="text-zinc-800 dark:text-zinc-200">1.0.0 Build 128</span>
            </div>
            <div className="flex gap-6">
              <span className="w-28 shrink-0">Node.js 版本：</span>
              <span className="text-zinc-800 dark:text-zinc-200">
                {installed ? "v20.18.1 LTS" : "未检测到（请先安装）"}
              </span>
            </div>
            <div className="flex gap-6">
              <span className="w-28 shrink-0">SillyTavern 版本：</span>
              <span className="text-zinc-800 dark:text-zinc-200">
                {installed ? "1.13.4 release (2026-01-20 14:32:05)" : "未安装"}
              </span>
            </div>
          </div>
        </div>

        {/* 右侧：公告 + 启动按钮 */}
        <div className="flex w-72 shrink-0 flex-col">
          <h2 className="text-lg font-bold text-zinc-900 dark:text-zinc-100">公告</h2>
          <div className="mt-3 max-h-72 flex-1 overflow-y-auto rounded-md border border-zinc-200 bg-white p-4 text-[13px] leading-relaxed text-zinc-700 dark:border-zinc-700/60 dark:bg-[#2b2b2b] dark:text-zinc-300">
            {announcements.map((a, i) => (
              <p key={i} className={i > 0 ? "mt-3" : ""}>
                {a}
              </p>
            ))}
          </div>

          <div className="mt-4">
            {status === "stopped" ? (
              installed ? (
                <button
                  onClick={onLaunch}
                  className="flex w-full items-center justify-center gap-2 rounded-md bg-blue-500 py-3.5 text-base font-semibold text-white shadow-md shadow-blue-500/30 transition-colors hover:bg-blue-600 active:bg-blue-700"
                >
                  <svg className="h-4 w-4" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M8 5.14v13.72c0 .8.87 1.3 1.56.88l11.14-6.86c.66-.4.66-1.36 0-1.76L9.56 4.26A1.03 1.03 0 0 0 8 5.14Z" />
                  </svg>
                  一键启动
                </button>
              ) : (
                <button
                  onClick={() => setPage("install")}
                  className="flex w-full items-center justify-center gap-2 rounded-md bg-amber-500 py-3.5 text-base font-semibold text-white shadow-md shadow-amber-500/30 transition-colors hover:bg-amber-600"
                >
                  <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2.2} strokeLinecap="round" strokeLinejoin="round">
                    <path d="M12 3v12" />
                    <path d="m7 10 5 5 5-5" />
                    <path d="M4 19h16" />
                  </svg>
                  前往安装
                </button>
              )
            ) : (
              <button
                onClick={onStop}
                className={cn(
                  "flex w-full items-center justify-center gap-2 rounded-md py-3.5 text-base font-semibold text-white shadow-md transition-colors",
                  status === "starting"
                    ? "bg-zinc-400 shadow-zinc-400/30 dark:bg-zinc-600"
                    : "bg-rose-500 shadow-rose-500/30 hover:bg-rose-600"
                )}
              >
                {status === "starting" ? (
                  <>
                    <svg className="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2.5}>
                      <path d="M21 12a9 9 0 1 1-6.2-8.56" strokeLinecap="round" />
                    </svg>
                    正在启动…
                  </>
                ) : (
                  <>
                    <svg className="h-4 w-4" viewBox="0 0 24 24" fill="currentColor">
                      <rect x="6" y="6" width="12" height="12" rx="2" />
                    </svg>
                    停止运行
                  </>
                )}
              </button>
            )}
            {status === "running" && (
              <p className="mt-2 text-center text-xs text-emerald-600 dark:text-emerald-400">
                ● 服务运行中 — http://127.0.0.1:8000
              </p>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
