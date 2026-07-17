import { createFileRoute, Outlet, redirect, useNavigate } from '@tanstack/react-router';
import { isAuthenticated, clearAuthToken } from '@/lib/auth';
import { AppShell } from '@/components/layout/AppShell';

export const Route = createFileRoute('/_authed')({
  beforeLoad: () => {
    if (!isAuthenticated()) {
      throw redirect({ to: '/login' });
    }
  },
  component: AuthedLayout,
});

function AuthedLayout() {
  const navigate = useNavigate();
  function logout() {
    clearAuthToken();
    void navigate({ to: '/login' });
  }
  return (
    <AppShell onLogout={logout}>
      <Outlet />
    </AppShell>
  );
}
