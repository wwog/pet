import { getAuthToken, clearAuthToken } from './auth';

export class ApiError extends Error {
  constructor(
    public readonly status: number,
    public readonly code?: string,
    message?: string,
    public readonly data?: unknown,
  ) {
    super(message ?? `API error ${status}`);
    this.name = 'ApiError';
  }
}

export interface ApiResponse<T> {
  data: T;
  status: number;
}

interface RequestOptions {
  method?: string;
  body?: unknown;
  headers?: Record<string, string>;
  signal?: AbortSignal;
}

const BASE_URL = '/api';

export async function request<T>(path: string, options: RequestOptions = {}): Promise<ApiResponse<T>> {
  const token = getAuthToken();
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...options.headers,
  };
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const res = await fetch(`${BASE_URL}${path}`, {
    method: options.method ?? 'GET',
    headers,
    body: options.body === undefined ? undefined : JSON.stringify(options.body),
    signal: options.signal,
  });

  if (res.status === 401) {
    clearAuthToken();
    if (typeof window !== 'undefined' && window.location.pathname !== '/login') {
      window.location.assign('/login');
    }
    throw new ApiError(401, 'UNAUTHORIZED', '未授权，请重新登录');
  }

  const isJson = res.headers.get('content-type')?.includes('application/json');
  const payload = isJson ? await res.json().catch(() => undefined) : undefined;

  if (!res.ok) {
    const code = typeof payload === 'object' && payload !== null ? (payload as { code?: string }).code : undefined;
    const message = typeof payload === 'object' && payload !== null ? (payload as { message?: string }).message : undefined;
    throw new ApiError(res.status, code, message, payload);
  }

  return { data: (payload ?? null) as T, status: res.status };
}
