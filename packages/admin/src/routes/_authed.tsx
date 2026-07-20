import { createFileRoute, Outlet, redirect, useNavigate } from '@tanstack/react-router';
import { isAuthenticated, getUserRole, clearAuth } from '@/lib/auth';
import { AppShell } from '@/components/layout/AppShell';

const SUPER_ADMIN_ROLE = 'super_admin';

export const Route = createFileRoute('/_authed')({
  beforeLoad: () => {
    if (!isAuthenticated()) {
      throw redirect({ to: '/login' });
    }
    // 仅超级管理员可访问管理后台
    if (getUserRole() !== SUPER_ADMIN_ROLE) {
      clearAuth();
      throw redirect({ to: '/login' });
    }
  },
  component: AuthedLayout,
});

function AuthedLayout() {
  const navigate = useNavigate();
  function logout() {
    clearAuth();
    void navigate({ to: '/login' });
  }
  return (
    <AppShell onLogout={logout}>
      <Outlet />
    </AppShell>
  );
}
