import type { Page, ServerStatus } from "../types";
import { cn } from "../utils/cn";

interface SidebarProps {
  page: Page;
  setPage: (p: Page) => void;
  dark: boolean;
  toggleDark: () => void;
  status: ServerStatus;
}

interface NavItem {
  key: Page;
  label: string;
  icon: React.ReactNode;
}

const iconCls = "h-5 w-5";

const topItems: NavItem[] = [
  {
    key: "launch",
    label: "启动",
    icon: (
      <svg className={iconCls} viewBox="0 0 24 24" fill="currentColor">
        <path d="M8 5.14v13.72c0 .8.87 1.3 1.56.88l11.14-6.86c.66-.4.66-1.36 0-1.76L9.56 4.26A1.03 1.03 0 0 0 8 5.14Z" />
      </svg>
    ),
  },
  {
    key: "install",
    label: "安装",
    icon: (
      <svg className={iconCls} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8} strokeLinecap="round" strokeLinejoin="round">
        <path d="M12 3v12" />
        <path d="m7 10 5 5 5-5" />
        <path d="M4 19h16" />
      </svg>
    ),
  },
  {
    key: "terminal",
    label: "终端",
    icon: (
      <svg className={iconCls} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8} strokeLinecap="round" strokeLinejoin="round">
        <rect x="2" y="4" width="20" height="16" rx="2" />
        <path d="m7 9 3 3-3 3" />
        <path d="M13 15h4" />
      </svg>
    ),
  },
];

const bottomItems: NavItem[] = [
  {
    key: "settings",
    label: "设置",
    icon: (
      <svg className={iconCls} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8} strokeLinecap="round" strokeLinejoin="round">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1Z" />
      </svg>
    ),
  },
];

function NavButton({
  item,
  active,
  onClick,
  badge,
}: {
  item: NavItem;
  active: boolean;
  onClick: () => void;
  badge?: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className={cn(
        "relative flex w-full flex-col items-center gap-1 py-2.5 transition-colors",
        active
          ? "text-blue-600 dark:text-blue-400"
          : "text-zinc-500 hover:text-zinc-800 dark:text-zinc-400 dark:hover:text-zinc-100"
      )}
    >
      {active && (
        <span className="absolute top-1/2 left-0 h-8 w-[3px] -translate-y-1/2 rounded-r bg-blue-500" />
      )}
      <span className="relative">
        {item.icon}
        {badge}
      </span>
      <span className="text-[11px]">{item.label}</span>
    </button>
  );
}

export default function Sidebar({ page, setPage, dark, toggleDark, status }: SidebarProps) {
  return (
    <aside className="flex w-[72px] shrink-0 flex-col border-r border-zinc-200 bg-zinc-100 dark:border-zinc-800 dark:bg-[#181818]">
      {/* 顶部大启动按钮 */}
      <button
        onClick={() => setPage("launch")}
        className={cn(
          "mx-2.5 mt-3 mb-2 flex h-14 items-center justify-center rounded-lg border transition-all",
          page === "launch"
            ? "border-zinc-400 bg-white shadow-sm dark:border-zinc-500 dark:bg-zinc-800"
            : "border-transparent hover:bg-zinc-200 dark:hover:bg-zinc-800"
        )}
        title="启动页"
      >
        <svg className="h-7 w-7 text-blue-500" viewBox="0 0 24 24" fill="currentColor">
          <rect x="3" y="3" width="18" height="18" rx="3" className="text-blue-500/15" fill="currentColor" />
          <path d="M9.5 8.2v7.6c0 .62.68 1 1.2.68l6.1-3.8a.8.8 0 0 0 0-1.36l-6.1-3.8a.8.8 0 0 0-1.2.68Z" />
        </svg>
      </button>

      <nav className="flex flex-col">
        {topItems.map((item) => (
          <NavButton
            key={item.key}
            item={item}
            active={page === item.key}
            onClick={() => setPage(item.key)}
            badge={
              item.key === "terminal" && status === "running" ? (
                <span className="absolute -top-0.5 -right-1 h-2 w-2 rounded-full bg-emerald-500 ring-2 ring-zinc-100 dark:ring-[#181818]" />
              ) : undefined
            }
          />
        ))}
      </nav>

      <div className="flex-1" />

      {/* 夜间模式切换 */}
      <button
        onClick={toggleDark}
        className="flex w-full flex-col items-center gap-1 py-2.5 text-zinc-500 transition-colors hover:text-zinc-800 dark:text-zinc-400 dark:hover:text-zinc-100"
        title={dark ? "切换到日间模式" : "切换到夜间模式"}
      >
        {dark ? (
          <svg className={iconCls} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8} strokeLinecap="round" strokeLinejoin="round">
            <circle cx="12" cy="12" r="4" />
            <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" />
          </svg>
        ) : (
          <svg className={iconCls} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={1.8} strokeLinecap="round" strokeLinejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79Z" />
          </svg>
        )}
        <span className="text-[11px]">{dark ? "日间" : "夜间"}</span>
      </button>

      {bottomItems.map((item) => (
        <NavButton
          key={item.key}
          item={item}
          active={page === item.key}
          onClick={() => setPage(item.key)}
        />
      ))}
      <div className="h-2" />
    </aside>
  );
}
