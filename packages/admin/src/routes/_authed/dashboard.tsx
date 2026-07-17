import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/_authed/dashboard')({
  component: DashboardPage,
});

function DashboardPage() {
  return (
    <div className="space-y-4">
      <h1 className="text-2xl font-semibold text-slate-900">仪表盘</h1>
      <div className="grid grid-cols-1 gap-4 md:grid-cols-3">
        {[
          { label: '家庭成员', value: '—' },
          { label: '宠物档案', value: '—' },
          { label: '今日活跃', value: '—' },
        ].map((card) => (
          <div key={card.label} className="rounded-xl bg-white p-5 ring-1 ring-slate-200">
            <p className="text-sm text-slate-500">{card.label}</p>
            <p className="mt-2 text-3xl font-semibold text-slate-900">{card.value}</p>
          </div>
        ))}
      </div>
      <p className="text-sm text-slate-400">
        后台管理系统脚手架已就绪。接入真实 admin 登录接口与各业务模块后，此处将展示真实数据。
      </p>
    </div>
  );
}
