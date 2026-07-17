# 后台管理系统（packages/admin）设计

> 日期：2026-07-17
> 状态：已与用户确认设计方向，待审阅后进入实现计划

## 目标

在仓库根新增 `packages/admin` 后台管理系统，基于 Ant Design ProComponents + Vite 自建，通过 OpenAPI 契约全自动生成请求层，接入现有 auth 模块实现登录与权限。

## 范围

### 本期包含
- pnpm workspace 配置（仅含前端，不沾 Rust workspace）
- `packages/admin` 脚手架：React 18 + TS + Vite + ProComponents
- 后端 `crates/api` 增加 `--export-openapi` 导出能力（一次性改造）
- OpenAPI → TanStack Query hooks 自动生成链路（@hey-api/openapi-ts）
- 登录页（auth 模块：手机号验证码）+ 权限路由守卫 + ProLayout 动态菜单
- 一个示例业务模块：pet 宠物档案 ProTable，走通"列表 → 详情 → 增删改"全链路
- 根 `package.json` 代理脚本：`admin:dev`、`admin:build`、`gen:openapi`、`gen:api`

### 本期不包含
- 其余 150+ 接口的业务页面（由后续按相同模式扩展）
- i18n、主题切换
- 对现有 `crates/client_app`（Dioxus 桌面端）的任何改动
- Mock 数据层（直接对接真实 API；开发期可借助后端真实数据）

## 架构

### 仓库结构

```
pet/
├── Cargo.toml                      # Rust workspace（不动）
├── crates/                         # 不动
├── pnpm-workspace.yaml             # 新增：packages/*
├── package.json                    # 新增：根代理脚本
├── .gitignore                      # 追加：node_modules、openapi.json、services 生成产物
├── packages/
│   └── admin/
│       ├── package.json
│       ├── vite.config.ts
│       ├── tsconfig.json
│       ├── tsconfig.node.json
│       ├── openapi.json            # 生成物，gitignore
│       ├── .hey-api.ts            # 生成器配置
│       ├── index.html
│       └── src/
│           ├── main.tsx
│           ├── App.tsx             # 路由表 + QueryClientProvider
│           ├── access.ts          # 权限判断
│           ├── request.ts         # fetch 拦截器
│           ├── client.ts          # @hey-api client 配置
│           ├── layouts/
│           │   └── BasicLayout.tsx   # ProLayout + 动态菜单
│           ├── pages/
│           │   ├── User/Login.tsx
│           │   └── Pet/List.tsx       # 示例模块
│           ├── services/          # 生成物，gitignore
│           ├── routes/
│           │   └── RequireAuth.tsx   # 路由守卫
│           └── stores/
│               └── auth.ts        # zustand：token + user
└── api_doc/                        # 不动（Markdown 保留为人读）
```

### 后端改造（crates/api）

在 `main.rs` 检测 `--export-openapi` 参数：若存在，则序列化 `ApiDoc::openapi()` 为 JSON 写入 `packages/admin/openapi.json` 后 `return`，不启动服务器。约 15 行代码，使用 `std::env::args` 判断（不引入 clap）。

### 前端技术栈

| 层 | 选型 |
|----|------|
| 框架 | React 18 + TypeScript 5 + Vite 5 |
| UI | antd 5 + @ant-design/pro-components（ProLayout/ProTable/ProForm） |
| 数据层 | @tanstack/react-query v5 |
| 请求生成 | @hey-api/openapi-ts（生成 TanStack Query hooks） |
| 路由 | react-router-dom v6 |
| 状态 | zustand（token/user） |
| HTTP | @hey-api/client-fetch + 自定义拦截器 |

### 数据流

```
cargo run -p api -- --export-openapi
  → packages/admin/openapi.json
  → pnpm --filter admin gen:api
  → src/services/*.ts （queryKey + useXxx hooks）

ProTable → useListPetsQuery() → TanStack Query → client-fetch 拦截器注入 JWT
  → 后端 /v1/pets → 返回 → 缓存 → 渲染

401 → 拦截器清 token + 跳 /user/login
```

## 权限与登录

- 登录页调用 `POST /auth/sms/send` + `POST /auth/sms/login`，返回 `access_token` + `refresh_token`
- token 存于 zustand store，持久化到 localStorage
- `request.ts`（client-fetch 配置）：每个请求自动注入 `Authorization: Bearer <token>`
- `GET /auth/me` 获取当前用户 + 家庭列表，存入 store
- `access.ts`：暴露 `hasRole(role)` / `isPrimaryGuardian()`，基于家庭 RBAC
- `BasicLayout` 读取用户权限渲染动态菜单
- `RequireAuth` 路由守卫：无 token → 跳 `/user/login`
- 首期两级权限：首席监护人 / 普通成员（避免过度设计）

## 工作流脚本（根 package.json）

```json
{
  "scripts": {
    "admin:dev": "pnpm --filter admin dev",
    "admin:build": "pnpm --filter admin build",
    "gen:openapi": "cargo run -p api -- --export-openapi",
    "gen:api": "pnpm run gen:openapi && pnpm --filter admin gen:api"
  }
}
```

## 测试与验证

- `pnpm --filter admin build` 构建通过
- `cargo run -p api -- --export-openapi` 成功写出 openapi.json
- `pnpm run gen:api` 成功生成 services
- 登录页可走通验证码登录 → 跳转首页
- Pet List 页面可展示后端真实数据

## 关键约束

- 不引入 Rust workspace 依赖；前端独立 Node 生态
- 不修改 `crates/client_app`
- 不写组织性注释；不滥用 `unwrap`
- openapi.json 与 src/services 均为生成物，加入 .gitignore
