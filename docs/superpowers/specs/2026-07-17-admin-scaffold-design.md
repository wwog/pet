# Admin 后台管理系统 - 首版脚手架设计

> 日期：2026-07-17
> 范围：搭建 `packages/admin/` 前端工程的脚手架、登录页、布局壳，打通 OpenAPI → 自动生成 API 请求器的工作流。

## 1. 目标与非目标

### 目标
- 在 root 下建立 `packages/admin/` 目录，承载后台管理系统前端工程。
- 基于 Vite（最新版）+ React + TypeScript，不使用 React 框架（Next/Remix 等），自写路由与布局。
- 引入 TailwindCSS 作为样式工具。
- 引入 TanStack 全家桶：Query（数据缓存）、Router（类型安全路由）、Table（后续表格）、Store（状态）。
- 自动生成 API 请求器：使用 `@hey-api/openapi-ts`，消费 `packages/admin/openapi.json`（由 Rust 后端 `cargo run -p api -- --export-openapi` 产出），生成到 `packages/admin/src/services/`。
- 提供最小可运行的登录页 + 主布局（侧边栏 + 顶栏 + 内容区），路由权限守卫预留位置。
- 提供 dev 模式跳过登录的 mock token 机制，方便后续接入真实 admin 登录接口。

### 非目标（首版不涉及）
- 真实 admin 登录接口实现（Rust 侧不改动）。
- auth/family/pet 等业务模块的实际 CRUD 页面。
- 表格页的实际渲染（TanStack Table 只做依赖引入与示例）。
- 生产部署、CI、docker。

## 2. 目录结构

```
packages/admin/
├── index.html
├── package.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── tailwind.config.js
├── postcss.config.js
├── .gitignore (子级，可选)
├── openapi.json            # 由 Rust 后端导出，.gitignore 忽略
└── src/
    ├── main.tsx            # 应用入口，挂载 RouterProvider + QueryClientProvider
    ├── App.tsx             # 根布局
    ├── routes/             # TanStack Router 路由树
    │   ├── __root.tsx      # 根路由 + 全局布局
    │   ├── login.tsx       # 登录页
    │   └── _authed.tsx     # 受保护布局（守卫）
    │       └── dashboard.tsx
    ├── components/
    │   ├── layout/
    │   │   ├── Sidebar.tsx
    │   │   ├── Topbar.tsx
    │   │   └── AppShell.tsx
    │   └── ui/             # 通用小组件（Button、Input 等，按需）
    ├── lib/
    │   ├── query-client.ts # QueryClient 单例
    │   ├── http.ts         # fetch 包装，注入 token / base URL
    │   └── auth.ts         # mock token 读写（localStorage）
    ├── services/           # 自动生成，.gitignore 忽略
    │   ├── gen/            # @hey-api/openapi-ts 产物
    │   └── index.ts       # 重新导出 + 注入 http 客户端
    └── styles/
        └── globals.css     # Tailwind 入口
```

> `src/services/` 在 `.gitignore` 中已规划为忽略，每次 dev 启动或手动脚本重新生成。

## 3. 关键技术选型与版本

- **Vite**：最新版（>= 5.x，使用 `node scripts/getCrateVersion.js` 的 JS 对等脚本 `npm view` 取最新）
- **React 18 / 19**：取最新稳定
- **TypeScript**：最新 5.x
- **TailwindCSS**：最新版（v3 stable，不使用 v4 alpha 以保稳）
- **TanStack**：
  - `@tanstack/react-query` 最新
  - `@tanstack/react-router` 最新
  - `@tanstack/react-table` 最新
  - `@tanstack/react-store` 最新
- **@hey-api/openapi-ts**：最新，CLI 形式
- **pnpm** 作为包管理（与根 `.gitignore` 中 `node_modules/` 规划一致）

具体版本号在实现阶段使用 `npm view <pkg> version` 取最新写入 package.json。

## 4. OpenAPI → 请求器 生成链路

1. Rust 侧默认导出路径已是 `packages/admin/openapi.json`（`crates/api/src/main.rs:30`）。
2. 在 `packages/admin/package.json` 增加脚本：
   - `gen:api`: 调用 `openapi-ts -i openapi.json -o src/services/gen`
   - `watch:api`: 用 `vite-node` / `chokidar` 监听 `openapi.json` 变化重新生成（或简单用 `concurrently` + `nodemon`）
   - `dev`: 同时跑 vite + openapi watch
3. `src/services/index.ts` 中把生成的 client 与本地 `lib/http.ts` 的 fetch 包装绑定（注入 base URL + token）。
4. 后端默认导出路径与 `.gitignore` 中的 `packages/admin/openapi.json` 一致，无需改 Rust 代码。

## 5. 登录与权限守卫

- `src/lib/auth.ts` 提供 `getDevToken()` / `setDevToken()` / `clearDevToken()`，使用 `localStorage` 存一个 mock token。
- 登录页提供一个"以开发模式进入"按钮，写入一个假 token 后跳转到 dashboard。
- `_authed.tsx` 作为受保护路由的 layout，在 `beforeLoad` 中检查 token，无则重定向 `/login`。
- 真实接入时只需替换 `lib/auth.ts` 实现和登录页表单即可。

## 6. 布局

- `AppShell`：flex 布局，左侧 Sidebar、右侧上方 Topbar、下方 `<Outlet />`。
- Sidebar：模块导航占位（auth/family/pet/health/album/... 等 19 模块），首版仅 dashboard 可点，其余展示但标记"开发中"。
- Topbar：显示当前模式（DEV MODE）、mock 退出登录按钮。
- 配色：浅色主题，Tailwind 默认 slate 色板，后续再做暗色。

## 7. 错误处理与可观察性

- `lib/http.ts` 统一拦截 401 → 清 token 跳 login；其他非 2xx 抛出包含 status、message 的 `ApiError`。
- QueryClient 默认配置：重试 1 次，staleTime 30s，错误以 toast 形式提示（首版可先 console.error，后续再接 UI）。

## 8. 测试

首版不强制单测。后续在 services 与 lib 层引入 vitest，对 auth/http 做 1-2 个 smoke test 即可。本次不写测试。

## 9. 验收标准

- `pnpm -F admin install && pnpm -F admin run dev` 能启动 Vite，访问 `http://localhost:5173`。
- 登录页可见，点击"开发模式进入"后跳到 dashboard，左侧导航、顶栏布局正常。
- 运行 `cargo run -p api -- --export-openapi` 后再 `pnpm -F admin run gen:api`，能从 openapi.json 生成 `src/services/gen/` 下类型与 client。
- 生成的 services 在 dashboard 页能被 import 并调用（即使接口可能 401，只要 import 通、类型不报错即可）。
- TypeScript 严格模式（`strict: true`）下无类型错误，`pnpm -F admin run build` 成功。
