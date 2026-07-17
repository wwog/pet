interface TopbarProps {
  onLogout: () => void;
}

export function Topbar({ onLogout }: TopbarProps) {
  return (
    <header className="flex h-14 items-center justify-between border-b border-slate-200 bg-white px-6">
      <div className="flex items-center gap-2">
        <span className="rounded bg-amber-100 px-2 py-0.5 text-xs font-medium text-amber-700">
          DEV MODE
        </span>
        <span className="text-sm text-slate-500">开发模式（未接入真实 admin 登录）</span>
      </div>
      <button
        onClick={onLogout}
        className="rounded-md px-3 py-1.5 text-sm text-slate-600 transition hover:bg-slate-100 hover:text-slate-900"
      >
        退出登录
      </button>
    </header>
  );
}
