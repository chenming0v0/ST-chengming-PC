import { useEffect, useRef, useState } from "react";
import { cn } from "../utils/cn";

interface InstallPageProps {
  installed: boolean;
  onInstalled: () => void;
}

interface Step {
  name: string;
  desc: string;
  duration: number; // 模拟耗时（毫秒）
}

const steps: Step[] = [
  { name: "环境检测", desc: "检测系统环境与磁盘空间", duration: 1200 },
  { name: "安装 Node.js", desc: "下载并配置 Node.js v20 LTS 运行时", duration: 2600 },
  { name: "安装 Git", desc: "下载并配置 Git 版本控制工具", duration: 2000 },
  { name: "克隆 SillyTavern", desc: "git clone SillyTavern 仓库 (release 分支)", duration: 3000 },
  { name: "安装依赖", desc: "npm install 安装项目依赖包", duration: 3200 },
  { name: "完成配置", desc: "生成默认配置文件 config.yaml", duration: 1000 },
];

type StepState = "pending" | "active" | "done";

export default function InstallPage({ installed, onInstalled }: InstallPageProps) {
  const [installing, setInstalling] = useState(false);
  const [currentStep, setCurrentStep] = useState(-1);
  const [stepProgress, setStepProgress] = useState(0);
  const [branch, setBranch] = useState<"release" | "staging">("release");
  const [source, setSource] = useState<"github" | "mirror">("mirror");
  const [path, setPath] = useState("D:\\SillyTavern");
  const timers = useRef<number[]>([]);

  useEffect(() => () => timers.current.forEach(clearTimeout), []);

  const startInstall = () => {
    setInstalling(true);
    setCurrentStep(0);
    setStepProgress(0);

    let delay = 0;
    steps.forEach((step, i) => {
      // 每一步内部的进度动画
      const ticks = 20;
      for (let t = 1; t <= ticks; t++) {
        timers.current.push(
          window.setTimeout(() => {
            setCurrentStep(i);
            setStepProgress((t / ticks) * 100);
          }, delay + (step.duration * t) / ticks)
        );
      }
      delay += step.duration;
    });

    timers.current.push(
      window.setTimeout(() => {
        setInstalling(false);
        setCurrentStep(steps.length);
        onInstalled();
      }, delay + 300)
    );
  };

  const overall =
    currentStep < 0
      ? 0
      : Math.min(100, ((currentStep + stepProgress / 100) / steps.length) * 100);

  const stateOf = (i: number): StepState => {
    if (currentStep > i) return "done";
    if (currentStep === i && installing) return "active";
    if (currentStep >= steps.length) return "done";
    return "pending";
  };

  return (
    <div className="page-enter h-full overflow-y-auto p-6">
      <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">安装 SillyTavern</h1>
      <p className="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
        一键部署运行环境，自动完成 Node.js、Git 与 SillyTavern 本体的安装。
      </p>

      {/* 安装选项 */}
      <div className="mt-6 grid grid-cols-1 gap-4 lg:grid-cols-3">
        <div className="rounded-md border border-zinc-200 bg-white p-4 dark:border-zinc-700/60 dark:bg-[#2b2b2b]">
          <p className="text-sm font-semibold text-zinc-800 dark:text-zinc-200">安装分支</p>
          <div className="mt-3 flex gap-2">
            {(["release", "staging"] as const).map((b) => (
              <button
                key={b}
                disabled={installing}
                onClick={() => setBranch(b)}
                className={cn(
                  "flex-1 rounded-md border px-3 py-2 text-sm transition-colors",
                  branch === b
                    ? "border-blue-500 bg-blue-50 text-blue-600 dark:bg-blue-500/10 dark:text-blue-400"
                    : "border-zinc-200 text-zinc-600 hover:bg-zinc-50 dark:border-zinc-700 dark:text-zinc-400 dark:hover:bg-zinc-800"
                )}
              >
                {b === "release" ? "release（稳定版）" : "staging（测试版）"}
              </button>
            ))}
          </div>
        </div>

        <div className="rounded-md border border-zinc-200 bg-white p-4 dark:border-zinc-700/60 dark:bg-[#2b2b2b]">
          <p className="text-sm font-semibold text-zinc-800 dark:text-zinc-200">下载源</p>
          <div className="mt-3 flex gap-2">
            {(["mirror", "github"] as const).map((s) => (
              <button
                key={s}
                disabled={installing}
                onClick={() => setSource(s)}
                className={cn(
                  "flex-1 rounded-md border px-3 py-2 text-sm transition-colors",
                  source === s
                    ? "border-blue-500 bg-blue-50 text-blue-600 dark:bg-blue-500/10 dark:text-blue-400"
                    : "border-zinc-200 text-zinc-600 hover:bg-zinc-50 dark:border-zinc-700 dark:text-zinc-400 dark:hover:bg-zinc-800"
                )}
              >
                {s === "mirror" ? "国内镜像加速" : "GitHub 官方源"}
              </button>
            ))}
          </div>
        </div>

        <div className="rounded-md border border-zinc-200 bg-white p-4 dark:border-zinc-700/60 dark:bg-[#2b2b2b]">
          <p className="text-sm font-semibold text-zinc-800 dark:text-zinc-200">安装路径</p>
          <div className="mt-3 flex gap-2">
            <input
              value={path}
              disabled={installing}
              onChange={(e) => setPath(e.target.value)}
              className="min-w-0 flex-1 rounded-md border border-zinc-200 bg-transparent px-3 py-2 text-sm text-zinc-800 outline-none focus:border-blue-500 dark:border-zinc-700 dark:text-zinc-200"
            />
            <button
              disabled={installing}
              className="rounded-md border border-zinc-200 px-3 py-2 text-sm text-zinc-600 hover:bg-zinc-50 dark:border-zinc-700 dark:text-zinc-400 dark:hover:bg-zinc-800"
            >
              浏览…
            </button>
          </div>
        </div>
      </div>

      {/* 安装步骤 */}
      <div className="mt-6 rounded-md border border-zinc-200 bg-white p-5 dark:border-zinc-700/60 dark:bg-[#2b2b2b]">
        <div className="flex items-center justify-between">
          <p className="text-sm font-semibold text-zinc-800 dark:text-zinc-200">安装进度</p>
          <span className="text-sm font-medium text-blue-600 dark:text-blue-400">
            {Math.round(overall)}%
          </span>
        </div>
        <div className="mt-3 h-2 overflow-hidden rounded-full bg-zinc-100 dark:bg-zinc-700">
          <div
            className="h-full rounded-full bg-blue-500 transition-all duration-150"
            style={{ width: `${overall}%` }}
          />
        </div>

        <ul className="mt-5 space-y-1">
          {steps.map((step, i) => {
            const st = stateOf(i);
            return (
              <li
                key={step.name}
                className={cn(
                  "flex items-center gap-3 rounded-md px-3 py-2.5",
                  st === "active" && "bg-blue-50 dark:bg-blue-500/10"
                )}
              >
                <span
                  className={cn(
                    "flex h-6 w-6 shrink-0 items-center justify-center rounded-full text-xs font-semibold",
                    st === "done" && "bg-emerald-500 text-white",
                    st === "active" && "bg-blue-500 text-white",
                    st === "pending" && "bg-zinc-200 text-zinc-500 dark:bg-zinc-700 dark:text-zinc-400"
                  )}
                >
                  {st === "done" ? (
                    <svg className="h-3.5 w-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={3} strokeLinecap="round" strokeLinejoin="round">
                      <path d="m5 13 4 4L19 7" />
                    </svg>
                  ) : st === "active" ? (
                    <svg className="h-3.5 w-3.5 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={3} strokeLinecap="round">
                      <path d="M21 12a9 9 0 1 1-6.2-8.56" />
                    </svg>
                  ) : (
                    i + 1
                  )}
                </span>
                <div className="min-w-0 flex-1">
                  <p
                    className={cn(
                      "text-sm font-medium",
                      st === "pending"
                        ? "text-zinc-400 dark:text-zinc-500"
                        : "text-zinc-800 dark:text-zinc-200"
                    )}
                  >
                    {step.name}
                  </p>
                  <p className="truncate text-xs text-zinc-400 dark:text-zinc-500">{step.desc}</p>
                </div>
                {st === "active" && (
                  <span className="text-xs font-medium text-blue-600 dark:text-blue-400">
                    {Math.round(stepProgress)}%
                  </span>
                )}
              </li>
            );
          })}
        </ul>
      </div>

      {/* 操作按钮 */}
      <div className="mt-5 flex items-center gap-3 pb-4">
        {installed && !installing ? (
          <>
            <span className="flex items-center gap-1.5 text-sm font-medium text-emerald-600 dark:text-emerald-400">
              <svg className="h-4 w-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2.5} strokeLinecap="round" strokeLinejoin="round">
                <path d="m5 13 4 4L19 7" />
              </svg>
              已安装 SillyTavern 1.13.4（{branch} 分支）
            </span>
            <div className="flex-1" />
            <button className="rounded-md border border-zinc-300 px-4 py-2 text-sm font-medium text-zinc-700 hover:bg-zinc-50 dark:border-zinc-600 dark:text-zinc-300 dark:hover:bg-zinc-800">
              检查更新
            </button>
            <button
              onClick={startInstall}
              className="rounded-md bg-blue-500 px-4 py-2 text-sm font-medium text-white hover:bg-blue-600"
            >
              重新安装
            </button>
          </>
        ) : (
          <button
            onClick={startInstall}
            disabled={installing}
            className={cn(
              "flex items-center gap-2 rounded-md px-6 py-2.5 text-sm font-semibold text-white transition-colors",
              installing
                ? "cursor-not-allowed bg-zinc-400 dark:bg-zinc-600"
                : "bg-blue-500 shadow-md shadow-blue-500/30 hover:bg-blue-600"
            )}
          >
            {installing ? "正在安装，请稍候…" : "开始安装"}
          </button>
        )}
      </div>
    </div>
  );
}
