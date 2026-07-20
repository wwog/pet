// 移动端判定：与 Tailwind md 断点对齐（<768px 视为移动端）。
// 抽屉式导航等交互需要 JS 状态驱动 open/close、路由切换后自动关闭、遮罩点击关闭，
// 纯 CSS md:hidden 无法实现这些行为，故用 matchMedia 提供响应式状态。
import { useEffect, useState } from 'react';

const MOBILE_QUERY = '(max-width: 767px)';

export function useIsMobile(): boolean {
  const [isMobile, setIsMobile] = useState<boolean>(() => {
    if (typeof window === 'undefined') return false;
    return window.matchMedia(MOBILE_QUERY).matches;
  });

  useEffect(() => {
    if (typeof window === 'undefined') return;
    const mql = window.matchMedia(MOBILE_QUERY);
    function handleChange(event: MediaQueryListEvent) {
      setIsMobile(event.matches);
    }
    mql.addEventListener('change', handleChange);
    return () => mql.removeEventListener('change', handleChange);
  }, []);

  return isMobile;
}
