# API Auth 模块设计规范

> 日期：2026-07-16
> 范围：api crate 企业级项目搭建 + 用户账号密码认证（4 个接口）

## 1. 概述

本次为 `crates/api` 搭建企业级项目骨架，并实现基于账号密码的用户认证链路。认证策略使用 **JWT 无状态 access_token + 数据库持久化 refresh_token**。

## 2. 领域实体改动

### 2.1 User（domain 层）

将 `phone` 改为 `account`，新增 `password_hash`：

| 字段 | 类型 | 变更 |
|------|------|------|
| `phone: String` | → `account: String` | 唯一，最小6位，仅含字母/数字/下划线 |
| 新增 `password_hash: String` | `String` | argon2id 哈希结果 |
| `wechat_open_id` | `Option<String>` | 保留，预留微信登录 |
| 其他字段 | — | 不变 |

`nickname` 在注册时必填（1–20 字符），不设默认值。

### 2.2 UserSession

不变。access_token 存 JWT 字符串，refresh_token 存 UUID v4。

### 2.3 UserRepository trait 变更

```rust
find_by_phone(&str)       → find_by_account(&str)
create(User)              → 不变
find_by_id(Uuid)          → 不变
find_by_wechat_open_id    → 不变（预留）
update_nickname           → 不变
```

### 2.4 AppError 新增

```rust
#[error("auth: {0}")]
Auth(String),
```

## 3. Token 策略

| 项目 | 值 |
|------|-----|
| access_token | JWT HS256，payload 含 `sub`(user_id) `exp`，2h 过期 |
| refresh_token | UUID v4，存储于 user_sessions 表，30d 过期 |
| 刷新逻辑 | 校验 refresh_token 存在且未过期 → 签发新 JWT → 返回 |

JWT secret 从环境变量 `JWT_SECRET` 读取。

## 4. 密码策略

- 哈希算法：argon2id（`argon2` crate）
- 注册校验：密码 ≥8 字符，必须同时含字母和数字
- 不存明文，不记录密码历史

## 5. API 接口

### 5.1 注册 — `POST /auth/register`

请求：
```json
{
  "account": "zhangsan",
  "password": "abc12345",
  "nickname": "张三"
}
```

校验：
- account ≥6 字符，仅字母数字下划线
- password ≥8 字符，含字母+数字
- nickname 1–20 字符
- account 不重复

成功 201，返回：
```json
{
  "user_id": "uuid",
  "account": "zhangsan",
  "nickname": "张三",
  "created_at": "ISO8601"
}
```

### 5.2 登录 — `POST /auth/login`

请求：
```json
{
  "account": "zhangsan",
  "password": "abc12345"
}
```

校验 account 存在 + 密码匹配，成功后签发 JWT。

成功 200，返回：
```json
{
  "access_token": "eyJ...",
  "refresh_token": "uuid",
  "expires_in": 7200,
  "user": {
    "user_id": "uuid",
    "account": "zhangsan",
    "nickname": "张三",
    "avatar": null
  }
}
```

### 5.3 刷新 Token — `POST /auth/token/refresh`

请求：
```json
{
  "refresh_token": "uuid"
}
```

成功 200，返回：
```json
{
  "access_token": "eyJ...",
  "expires_in": 7200
}
```

### 5.4 获取当前用户 — `GET /auth/me`

Header: `Authorization: Bearer <access_token>`

成功 200，返回：
```json
{
  "user_id": "uuid",
  "account": "zhangsan",
  "nickname": "张三",
  "avatar": null,
  "created_at": "ISO8601"
}
```

## 6. 错误码规范

统一响应格式：
```json
{
  "code": 1004,
  "message": "account already exists"
}
```

| code | 含义 |
|------|------|
| 1001 | 未认证 / token 无效 |
| 1002 | token 过期 |
| 1003 | 权限不足 |
| 1004 | 请求参数校验失败 |
| 1005 | 账号已存在 |
| 1006 | 账号不存在 |
| 1007 | 密码错误 |
| 9999 | 内部服务错误 |

## 7. api crate 目录结构

```
crates/api/src/
  main.rs          — tokio::main, 初始化 tracing/DB, 挂载路由, bind 端口
  config.rs        — 环境变量读取 (DATABASE_URL, JWT_SECRET, BIND_ADDR)
  error.rs         — AppError → (StatusCode, Json<ErrorResponse>) 
  app_state.rs     — Arc<Database> 共享状态
  auth/
    mod.rs         — Router::new() 组装子路由
    handler.rs     — register, login, refresh_token, get_me
    dto.rs         — 请求/响应结构体（Serialize/Deserialize）
    jwt.rs         — JWT 签发与验证
    middleware.rs  — Bearer token 提取（FromRequestParts）
```

## 8. 依赖

新增 workspace 依赖：
- `axum` 0.8.9（已存在）
- `jsonwebtoken` — JWT 签发验证
- `argon2` — 密码哈希
- `once_cell` — 懒加载静态配置
- `regex` — account 格式校验

## 9. 配置

环境变量（均通过 `config.rs` 读取，无默认值或提供合理默认值）：

| 变量 | 说明 | 默认值 |
|------|------|--------|
| `DATABASE_URL` | SQLite 路径 | `sqlite:data.db` |
| `JWT_SECRET` | JWT 签名密钥 | _必须设置_ |
| `BIND_ADDR` | 监听地址 | `0.0.0.0:3000` |
| `RUST_LOG` | tracing 日志级别 | `info` |

## 10. 自检清单

- [x] account 替代 phone，新增 password_hash
- [x] JWT 访问令牌 + UUID 刷新令牌
- [x] 4 个接口：register / login / refresh / me
- [x] 统一错误响应格式
- [x] 环境变量配置
- [x] 无 TODO / 占位符
