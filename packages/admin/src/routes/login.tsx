import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { setAuthToken, DEV_MOCK_TOKEN } from '@/lib/auth';

export const Route = createFileRoute('/login')({
  component: LoginPage,
});

function LoginPage() {
  const navigate = useNavigate();
  function enterDevMode() {
    setAuthToken(DEV_MOCK_TOKEN);
    void navigate({ to: '/dashboard' });
  }
  return (
    <div className="flex min-h-screen items-center justify-center bg-slate-100">
      <div className="w-full max-w-sm rounded-xl bg-white p-8 shadow-sm ring-1 ring-slate-200">
        <h1 className="text-2xl font-semibold text-slate-900">小狗人生 · 管理后台</h1>
        <p className="mt-2 text-sm text-slate-500">后台专属登录接口尚未就绪，首版以开发模式进入。</p>
        <button
          onClick={enterDevMode}
          className="mt-6 w-full rounded-lg bg-slate-900 px-4 py-2.5 text-sm font-medium text-white transition hover:bg-slate-700"
        >
          以开发模式进入
        </button>
      </div>
    </div>
  );
}
