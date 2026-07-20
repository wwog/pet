interface TopbarProps {
  onLogout: () => void;
  onMenuClick: () => void;
}

export function Topbar({ onLogout, onMenuClick }: TopbarProps) {
  return (
    <header className="flex h-14 items-center justify-between border-b border-slate-200 bg-white px-4 md:px-6">
      <div className="flex items-center gap-2">
        {/* 汉堡按钮：仅移动端可见，触发抽屉导航 */}
        <button
          type="button"
          onClick={onMenuClick}
          className="-ml-1 mr-1 inline-flex items-center justify-center rounded-md p-2 text-slate-600 transition hover:bg-slate-100 hover:text-slate-900 md:hidden"
          aria-label="打开导航菜单"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 20 20"
            fill="none"
            stroke="currentColor"
            strokeWidth="1.6"
            strokeLinecap="round"
          >
            <line x1="3" y1="6" x2="17" y2="6" />
            <line x1="3" y1="10" x2="17" y2="10" />
            <line x1="3" y1="14" x2="17" y2="14" />
          </svg>
        </button>
        <span className="rounded bg-slate-100 px-2 py-0.5 text-xs font-medium text-slate-600">
          ADMIN
        </span>
        {/* “管理后台”文案在最小屏隐藏，避免挤占退出按钮 */}
        <span className="hidden text-sm text-slate-500 sm:inline">管理后台</span>
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
