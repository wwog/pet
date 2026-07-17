import { Link } from '@tanstack/react-router';

interface NavItem {
  label: string;
  to: string;
  available?: boolean;
}

const MODULES: NavItem[] = [
  { label: '仪表盘', to: '/dashboard', available: true },
  { label: '认证', to: '/auth' },
  { label: '家庭', to: '/family' },
  { label: '宠物档案', to: '/pet' },
  { label: '健康记录', to: '/health' },
  { label: '云相册', to: '/album' },
  { label: '智能日程', to: '/calendar' },
  { label: '遛狗追踪', to: '/walk' },
  { label: 'AI 翻译官', to: '/ai-chat' },
  { label: '每日简报', to: '/briefing' },
  { label: '财务管家', to: '/finance' },
  { label: '饮食计算', to: '/diet' },
  { label: 'LBS 雷达', to: '/radar' },
  { label: '文件保险箱', to: '/vault' },
  { label: '访客模式', to: '/guest' },
  { label: '动态墙', to: '/activity-log' },
  { label: '数据导出', to: '/export' },
  { label: '通知中心', to: '/notification' },
  { label: '文件上传', to: '/upload' },
];

export function Sidebar() {
  return (
    <aside className="flex w-60 shrink-0 flex-col border-r border-slate-200 bg-white">
      <div className="flex h-14 items-center px-5 text-base font-semibold text-slate-900">
        小狗人生 OS
      </div>
      <nav className="flex-1 space-y-0.5 overflow-y-auto px-3 py-2">
        {MODULES.map((item) => {
          const content = (
            <span className="flex items-center justify-between">
              <span>{item.label}</span>
              {!item.available && (
                <span className="rounded bg-slate-100 px-1.5 py-0.5 text-[10px] text-slate-400">
                  待接入
                </span>
              )}
            </span>
          );
          if (!item.available) {
            return (
              <span
                key={item.to}
                className="block cursor-not-allowed rounded-md px-3 py-2 text-sm text-slate-400"
              >
                {content}
              </span>
            );
          }
          return (
            <Link
              key={item.to}
              to={item.to}
              activeProps={{ className: 'bg-slate-100 text-slate-900' }}
              className="block rounded-md px-3 py-2 text-sm text-slate-600 transition hover:bg-slate-50 hover:text-slate-900"
            >
              {content}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
