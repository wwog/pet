// 临时 mock 认证：首版没有后台专属登录接口，本地用 localStorage 存一个开发 token。
// 真实 admin 登录接口就绪后，替换此模块的实现即可，调用方接口保持不变。

const TOKEN_KEY = 'pet_admin_dev_token';

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

export const DEV_MOCK_TOKEN = 'dev-mock-token';
