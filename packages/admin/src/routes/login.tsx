import { createFileRoute, useNavigate } from '@tanstack/react-router';
import { useState } from 'react';
import { login } from '@/services';
import { setAuthToken } from '@/lib/auth';

export const Route = createFileRoute('/login')({
  component: LoginPage,
});

interface FieldError {
  account?: string;
  password?: string;
  form?: string;
}

function LoginPage() {
  const navigate = useNavigate();
  const [account, setAccount] = useState('');
  const [password, setPassword] = useState('');
  const [submitting, setSubmitting] = useState(false);
  const [errors, setErrors] = useState<FieldError>({});

  async function handleSubmit(event: React.FormEvent) {
    event.preventDefault();
    if (submitting) return;

    const fieldErrors: FieldError = {};
    if (account.trim().length < 6) {
      fieldErrors.account = '账号至少 6 位';
    }
    if (password.length < 8) {
      fieldErrors.password = '密码至少 8 位';
    }
    if (Object.keys(fieldErrors).length > 0) {
      setErrors(fieldErrors);
      return;
    }

    setSubmitting(true);
    setErrors({});

    const result = await login({
      body: { account: account.trim(), password },
    });

    if (result.error || !result.data) {
      const message =
        result.error?.message ?? '登录失败，请稍后重试';
      setErrors({ form: message });
      setSubmitting(false);
      return;
    }

    setAuthToken(result.data.data.access_token);
    void navigate({ to: '/dashboard' });
  }

  return (
    <div className="flex min-h-screen items-center justify-center bg-slate-100">
      <div className="w-full max-w-sm rounded-xl bg-white p-8 shadow-sm ring-1 ring-slate-200">
        <h1 className="text-2xl font-semibold text-slate-900">小狗人生 · 管理后台</h1>
        <p className="mt-2 text-sm text-slate-500">请使用管理员账号登录。</p>

        <form onSubmit={handleSubmit} className="mt-6 space-y-4">
          <div>
            <label htmlFor="account" className="block text-sm font-medium text-slate-700">
              账号
            </label>
            <input
              id="account"
              type="text"
              autoComplete="username"
              value={account}
              onChange={(e) => setAccount(e.target.value)}
              className="mt-1 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm shadow-sm focus:border-slate-900 focus:outline-none focus:ring-1 focus:ring-slate-900"
              placeholder="请输入账号"
              disabled={submitting}
            />
            {errors.account && (
              <p className="mt-1 text-xs text-red-600">{errors.account}</p>
            )}
          </div>

          <div>
            <label htmlFor="password" className="block text-sm font-medium text-slate-700">
              密码
            </label>
            <input
              id="password"
              type="password"
              autoComplete="current-password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="mt-1 w-full rounded-lg border border-slate-300 px-3 py-2 text-sm shadow-sm focus:border-slate-900 focus:outline-none focus:ring-1 focus:ring-slate-900"
              placeholder="请输入密码"
              disabled={submitting}
            />
            {errors.password && (
              <p className="mt-1 text-xs text-red-600">{errors.password}</p>
            )}
          </div>

          {errors.form && (
            <p className="text-sm text-red-600">{errors.form}</p>
          )}

          <button
            type="submit"
            disabled={submitting}
            className="w-full rounded-lg bg-slate-900 px-4 py-2.5 text-sm font-medium text-white transition hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-60"
          >
            {submitting ? '登录中…' : '登录'}
          </button>
        </form>
      </div>
    </div>
  );
}
