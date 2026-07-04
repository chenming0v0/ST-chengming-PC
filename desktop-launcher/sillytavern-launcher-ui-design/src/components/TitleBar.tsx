export default function TitleBar() {
  return (
    <div className="flex h-9 shrink-0 items-center justify-between bg-zinc-100 pl-2 select-none dark:bg-[#181818]">
      <div className="flex items-center gap-2">
        <div className="flex h-6 w-6 items-center justify-center rounded bg-gradient-to-br from-amber-400 to-rose-500 text-[13px]">
          🍺
        </div>
        <span className="text-[13px] font-medium text-zinc-800 dark:text-zinc-200">
          SillyTavern 启动器 1.0.0
        </span>
      </div>
      <div className="flex h-full items-center">
        <button
          className="flex h-full w-11 items-center justify-center text-zinc-600 hover:bg-zinc-200 dark:text-zinc-400 dark:hover:bg-zinc-700"
          title="帮助"
        >
          <svg className="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2}>
            <circle cx="12" cy="12" r="10" />
            <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" strokeLinecap="round" />
            <circle cx="12" cy="17" r="0.5" fill="currentColor" />
          </svg>
        </button>
        <button className="flex h-full w-11 items-center justify-center text-zinc-600 hover:bg-zinc-200 dark:text-zinc-400 dark:hover:bg-zinc-700">
          <svg className="h-3 w-3" viewBox="0 0 12 12" stroke="currentColor" strokeWidth={1.2}>
            <line x1="1" y1="6" x2="11" y2="6" />
          </svg>
        </button>
        <button className="flex h-full w-11 items-center justify-center text-zinc-600 hover:bg-zinc-200 dark:text-zinc-400 dark:hover:bg-zinc-700">
          <svg className="h-3 w-3" viewBox="0 0 12 12" fill="none" stroke="currentColor" strokeWidth={1.2}>
            <rect x="1.5" y="1.5" width="9" height="9" />
          </svg>
        </button>
        <button className="flex h-full w-11 items-center justify-center text-zinc-600 hover:bg-red-500 hover:text-white dark:text-zinc-400 dark:hover:bg-red-600">
          <svg className="h-3 w-3" viewBox="0 0 12 12" stroke="currentColor" strokeWidth={1.2}>
            <line x1="1" y1="1" x2="11" y2="11" />
            <line x1="11" y1="1" x2="1" y2="11" />
          </svg>
        </button>
      </div>
    </div>
  );
}
