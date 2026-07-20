// 认证 token 与角色管理：登录成功后由 login.tsx 写入，http/services 拦截器读取注入请求头。

const TOKEN_KEY = 'pet_admin_dev_token';
const ROLE_KEY = 'pet_admin_dev_role';

export function getAuthToken(): string | null {
  if (typeof window === 'undefined') return null;
  return window.localStorage.getItem(TOKEN_KEY);
}

export function setAuthToken(token: string): void {
  window.localStorage.setItem(TOKEN_KEY, token);
}

export function clearAuthToken(): void {
  window.localStorage.removeItem(TOKEN_KEY);
}

export function isAuthenticated(): boolean {
  return getAuthToken() !== null;
}

export function getUserRole(): string | null {
  if (typeof window === 'undefined') return null;
  return window.localStorage.getItem(ROLE_KEY);
}

export function setUserRole(role: string): void {
  window.localStorage.setItem(ROLE_KEY, role);
}

export function clearUserRole(): void {
  window.localStorage.removeItem(ROLE_KEY);
}

export function clearAuth(): void {
  clearAuthToken();
  clearUserRole();
}
