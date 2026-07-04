import { useState } from "react";
import { cn } from "../utils/cn";

function Toggle({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <button
      onClick={() => onChange(!checked)}
      className={cn(
        "relative h-6 w-11 shrink-0 rounded-full transition-colors",
        checked ? "bg-blue-500" : "bg-zinc-300 dark:bg-zinc-600"
      )}
    >
      <span
        className={cn(
          "absolute top-0.5 left-0.5 h-5 w-5 rounded-full bg-white shadow transition-transform",
          checked && "translate-x-5"
        )}
      />
    </button>
  );
}

function SettingRow({
  title,
  desc,
  children,
}: {
  title: string;
  desc: string;
  children: React.ReactNode;
}) {
  return (
    <div className="flex items-center gap-4 px-5 py-4">
      <div className="min-w-0 flex-1">
        <p className="text-sm font-medium text-zinc-800 dark:text-zinc-200">{title}</p>
        <p className="mt-0.5 text-xs text-zinc-500 dark:text-zinc-400">{desc}</p>
      </div>
      {children}
    </div>
  );
}

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="mt-6">
      <h2 className="mb-2 text-sm font-semibold text-zinc-500 dark:text-zinc-400">{title}</h2>
      <div className="divide-y divide-zinc-100 rounded-md border border-zinc-200 bg-white dark:divide-zinc-700/50 dark:border-zinc-700/60 dark:bg-[#2b2b2b]">
        {children}
      </div>
    </div>
  );
}

const inputCls =
  "w-44 rounded-md border border-zinc-200 bg-transparent px-3 py-1.5 text-sm text-zinc-800 outline-none focus:border-blue-500 dark:border-zinc-700 dark:text-zinc-200";

interface SettingsPageProps {
  dark: boolean;
  toggleDark: () => void;
}

export default function SettingsPage({ dark, toggleDark }: SettingsPageProps) {
  const [port, setPort] = useState("8000");
  const [listen, setListen] = useState(false);
  const [autoBrowser, setAutoBrowser] = useState(true);
  const [whitelist, setWhitelist] = useState(true);
  const [basicAuth, setBasicAuth] = useState(false);
  const [autoUpdate, setAutoUpdate] = useState(true);
  const [proxy, setProxy] = useState("");
  const [lang, setLang] = useState("zh-CN");
  const [closeAction, setCloseAction] = useState<"tray" | "exit">("tray");
  const [saved, setSaved] = useState(false);

  const save = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 1800);
  };

  return (
    <div className="page-enter h-full overflow-y-auto p-6">
      <div className="flex items-center">
        <div>
          <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">设置</h1>
          <p className="mt-1 text-sm text-zinc-500 dark:text-zinc-400">
            配置启动器与 SillyTavern 服务的运行参数。
          </p>
        </div>
        <div className="flex-1" />
        <button
          onClick={save}
          className={cn(
            "rounded-md px-5 py-2 text-sm font-semibold text-white transition-colors",
            saved ? "bg-emerald-500" : "bg-blue-500 hover:bg-blue-600"
          )}
        >
          {saved ? "✓ 已保存" : "保存设置"}
        </button>
      </div>

      <Section title="网络">
        <SettingRow title="服务端口" desc="SillyTavern 监听的端口号，默认 8000">
          <input value={port} onChange={(e) => setPort(e.target.value)} className={inputCls} />
        </SettingRow>
        <SettingRow title="局域网监听" desc="开启后局域网内其他设备可通过本机 IP 访问酒馆">
          <Toggle checked={listen} onChange={setListen} />
        </SettingRow>
        <SettingRow title="IP 白名单" desc="仅允许白名单内的 IP 地址访问（whitelist.txt）">
          <Toggle checked={whitelist} onChange={setWhitelist} />
        </SettingRow>
        <SettingRow title="基础身份验证" desc="访问酒馆时需要输入用户名和密码（basicAuthMode）">
          <Toggle checked={basicAuth} onChange={setBasicAuth} />
        </SettingRow>
        <SettingRow title="网络代理" desc="为 git / npm 下载配置 HTTP 代理，留空则不使用">
          <input
            value={proxy}
            onChange={(e) => setProxy(e.target.value)}
            placeholder="http://127.0.0.1:7890"
            className={inputCls}
          />
        </SettingRow>
      </Section>

      <Section title="启动">
        <SettingRow title="启动后自动打开浏览器" desc="服务启动完成后自动在默认浏览器中打开酒馆页面">
          <Toggle checked={autoBrowser} onChange={setAutoBrowser} />
        </SettingRow>
        <SettingRow title="启动前自动检查更新" desc="每次启动前检查 SillyTavern 是否有新版本">
          <Toggle checked={autoUpdate} onChange={setAutoUpdate} />
        </SettingRow>
        <SettingRow title="关闭窗口时" desc="点击关闭按钮后的行为">
          <select
            value={closeAction}
            onChange={(e) => setCloseAction(e.target.value as "tray" | "exit")}
            className={cn(inputCls, "dark:bg-[#2b2b2b]")}
          >
            <option value="tray">最小化到托盘</option>
            <option value="exit">退出并停止服务</option>
          </select>
        </SettingRow>
      </Section>

      <Section title="外观">
        <SettingRow title="夜间模式" desc="切换启动器的明暗主题（与侧边栏按钮同步）">
          <Toggle checked={dark} onChange={toggleDark} />
        </SettingRow>
        <SettingRow title="界面语言" desc="启动器界面显示语言">
          <select
            value={lang}
            onChange={(e) => setLang(e.target.value)}
            className={cn(inputCls, "dark:bg-[#2b2b2b]")}
          >
            <option value="zh-CN">简体中文</option>
            <option value="zh-TW">繁體中文</option>
            <option value="en-US">English</option>
            <option value="ja-JP">日本語</option>
          </select>
        </SettingRow>
      </Section>

      <Section title="关于">
        <SettingRow title="SillyTavern 启动器" desc="版本 1.0.0 Build 128 · 基于 MIT 协议开源，完全免费">
          <button className="rounded-md border border-zinc-300 px-3 py-1.5 text-xs font-medium text-zinc-600 hover:bg-zinc-50 dark:border-zinc-600 dark:text-zinc-300 dark:hover:bg-zinc-800">
            检查启动器更新
          </button>
        </SettingRow>
        <SettingRow title="开源仓库" desc="github.com/SillyTavern/SillyTavern">
          <a
            href="https://github.com/SillyTavern/SillyTavern"
            target="_blank"
            rel="noreferrer"
            className="text-xs font-medium text-blue-600 hover:underline dark:text-blue-400"
          >
            前往 GitHub →
          </a>
        </SettingRow>
      </Section>
      <div className="h-4" />
    </div>
  );
}
