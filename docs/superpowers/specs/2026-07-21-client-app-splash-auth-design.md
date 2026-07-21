# client_app 首屏完善 + 模块拆分 + 登录注册 设计

- 日期:2026-07-21
- 范围:`crates/client_app`(Dioxus 0.7 web/desktop)
- 目标:完善首屏视觉(宠物头部上移 + 插画背景)、拆分 `main.rs` 为按层模块、实现登录注册与鉴权流并对接已有后端。

## 1. 背景与现状

### 1.1 现状
- `crates/client_app/src/main.rs` 单文件 717 行,包含 `App`、`AppLayout`、`Briefing`、`AiChat`、`Me` 四个组件 + `Route` enum + `TabId`/`ModeId`。
- 路由仅 3 条(`/`、`/ai`、`/me`),无鉴权机制,无登录注册。
- `assets/header.svg`(viewBox `1007×197`,内嵌 `@keyframes` 动画)**未被 main.rs 引用**。
- Briefing 顶部:`状态栏(44px)` -> `top-row(家庭+成员)` -> `pet-switch(宠物头像 40px)`,宠物头像视觉中心约距顶 90-100px,视觉上"太靠下"。

### 1.2 后端能力(已实现,可直接对接)
- `POST /app/auth/register`:`{ account, password, nickname }` -> `{ user_id, account, nickname, role, created_at }`
- `POST /app/auth/login`:`{ account, password }` -> `{ access_token, refresh_token, expires_in: 7200, user: {...} }`
- `POST /common/auth/token/refresh`:`{ refresh_token }` -> `{ access_token, expires_in: 7200 }`
- `GET /common/auth/me`(Bearer) -> `{ user_id, account, nickname?, avatar?, role, created_at }`
- 鉴权:纯 Bearer Token,JWT(HS256,access 2h)+ DB 持久化 session(refresh 30 天,UUID)。
- **后端无 CORS 配置**,需前端 dev proxy 解决跨域。

### 1.3 校验规则(后端 handler.rs)
- account:≥6 位,仅字母数字下划线
- password:≥8 位,必须含字母 + 数字
- nickname:1-20 字符

## 2. 目录结构(按层拆分)

遵循 AGENTS.md "Never create files with `mod.rs` paths",用同名 `.rs` 文件替代 `mod.rs`。

```
crates/client_app/src/
├── main.rs                  // 仅 dioxus::launch(App)
├── app.rs                   // App 组件 + Route enum + Router 装配
├── routes.rs                // 声明 pub mod briefing/ai_chat/me/login/register/splash
├── routes/
│   ├── briefing.rs          // Briefing 页
│   ├── ai_chat.rs           // AI 翻译官页
│   ├── me.rs                // 我的页
│   ├── login.rs             // 登录页
│   ├── register.rs          // 注册页
│   └── splash.rs            // 启动页(跳板)
├── components.rs            // 声明 pub mod app_layout/tab_bar/status_bar/pet_header
├── components/
│   ├── app_layout.rs        // AppLayout(状态栏 + Outlet + TabBar)
│   ├── tab_bar.rs           // 底部 TabBar(灵动指示器)
│   ├── status_bar.rs        // 顶部状态栏
│   └── pet_header.rs        // Briefing 顶部宠物头部(含插画背景)
├── auth.rs                  // 声明 pub mod token/session
├── auth/
│   ├── token.rs             // localStorage 封装
│   └── session.rs           // 全局 Session(Signal<AuthState>)
├── api.rs                   // 声明 pub mod client + 共享类型
├── api/
│   └── client.rs            // reqwest 封装 + 401 拦截/refresh
└── state.rs                 // 全局 Context provide
```

### 2.1 路由设计

```rust
#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Splash {},
    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},

    #[layout(AppLayout)]
    #[route("/briefing")]
    Briefing {},
    #[route("/ai")]
    AiChat {},
    #[route("/me")]
    Me {},
}
```

- `AppLayout` 仅包裹已登录的三个 Tab 页(状态栏 + Outlet + TabBar)。
- Splash / Login / Register **不进** AppLayout,无 TabBar。
- `/` 原本是 Briefing,改为 Splash 跳板;Briefing 迁至 `/briefing`。

## 3. 首屏视觉(Briefing 宠物头部上移)

### 3.1 pet_header 组件
抽出 `components/pet_header.rs`,把"家庭切换 + 宠物头部 + 插画背景"合并为约 150px 高的 hero header,紧贴状态栏下方:

```
┌─────────────────────────────────────┐
│ 9:41                    📶 🔋        │ ← 状态栏 44px
├─────────────────────────────────────┤
│  [header.svg 插画背景,绝对定位 cover]│
│  阿哲的家 ▾          ●●●  成员头像   │ ← 家庭行,叠在插画顶部
│                                      │
│   ╭──╮  豆豆                         │
│   │🐶│  金毛 · 1岁2个月 · 男孩  + │   ← 宠物头部,头像 56px
│   ╰──╯                               │
└─────────────────────────────────────┘
```

### 3.2 关键改动
1. **插画背景层**:`header.svg` 作为 `pet_header` 绝对定位背景,`<img src={asset!("/assets/header.svg")}>`,`object-fit: cover`,宽度撑满,Z 轴最低。浏览器渲染 SVG 时播放内嵌 CSS 动画。
   - 备选:若 `<img>` 方式动画不播放,改用 `dangerous_inner_html` 内联 SVG(仅 19 行,体积可控)。
2. **宠物头像上移 + 放大**:40px -> 56px,视觉中心从距顶 ~90px 上提到 ~70px,加 `box-shadow` 浮出感。
3. **家庭行叠加**:家庭名 + 成员头像移到插画顶部,白色文字 + `text-shadow` 保证可读。
4. **消除冗余间距**:删除 `top-row`/`pet-switch` 各自的 `padding-bottom`,统一由 `pet_header` 的 `margin-bottom: 14px` 收尾。
5. **负 margin 撑满宽度**:`margin: -4px -16px 14px`,让插画突破 `screen-body` 的 16px padding。

### 3.3 CSS 要点
```css
.pet-header {
  position: relative;
  height: 150px;
  margin: -4px -16px 14px;
  overflow: hidden;
}
.pet-header .bg {
  position: absolute; inset: 0;
  width: 100%; height: 100%;
  object-fit: cover;
}
.pet-header::after {                 /* 顶部暗化遮罩,保证文字可读 */
  content: '';
  position: absolute; inset: 0;
  background: linear-gradient(180deg, oklch(26% 0.03 55 / 0.25) 0%, transparent 40%);
  pointer-events: none;
}
.pet-header .family-row {
  position: relative; z-index: 2;
  display: flex; justify-content: space-between;
  padding: 10px 16px 0;
  color: #fff;
}
.pet-header .pet-row {
  position: relative; z-index: 2;
  display: flex; align-items: center; gap: 12px;
  padding: 14px 16px 0;
}
.pet-header .pet-av {
  width: 56px; height: 56px;
  box-shadow: 0 4px 12px oklch(26% 0.03 55 / 0.2);
}
```

## 4. 鉴权流

### 4.1 状态模型

```rust
// auth/session.rs
#[derive(Clone, PartialEq)]
pub struct Session {
    pub state: Signal<AuthState>,
}

#[derive(Clone, PartialEq)]
pub enum AuthState {
    Loading,                                   // 启动时读 token 中
    Guest,                                     // 未登录
    Authenticated {
        access_token: String,
        user: UserInfo,
        refresh_token: String,
        expires_at: i64,                       // access_token 过期时间戳(秒)
    },
}

#[derive(Clone, PartialEq)]
pub struct UserInfo {
    pub user_id: String,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
    pub created_at: String,
}
```

`Session` 通过 `use_context_provider` 在 App 根注入,任意子组件 `use_context::<Session>()` 消费。

### 4.2 token 存取(localStorage)

```rust
// auth/token.rs
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,    // 秒级时间戳
}
pub fn save_tokens(access: &str, refresh: &str, expires_in: i64);
pub fn load_tokens() -> Option<StoredToken>;
pub fn clear_tokens();
```

- `expires_at = now + expires_in - 60`(提前 60 秒视为过期,避免边界竞态)。
- 所有 localStorage 访问必须在 `use_effect` / 事件回调里(hydration 安全,不在渲染期同步调用)。
- desktop 平台 localStorage 不可用时的兜底:函数返回 `None` / no-op,不 panic。

### 4.3 API client

```rust
// api/client.rs
const API_BASE: &str = "/api";   // 相对路径,走 dev proxy

pub async fn login(account, password) -> Result<LoginResponse, ApiError>;
pub async fn register(account, password, nickname) -> Result<RegisterResponse, ApiError>;
pub async fn me(access_token: &str) -> Result<MeResponse, ApiError>;
pub async fn refresh(refresh_token: &str) -> Result<RefreshResponse, ApiError>;
```

- 复用单例 `reqwest::Client`(lazy once_cell)。
- 请求体 serde 序列化为 snake_case(后端默认 snake_case)。
- 响应按后端 `ApiResponse<T>` 解析:`{ code: i32, message: String, data: Option<T> }`;`code != 0` 视为错误,返回 `ApiError::Api(code, message)`。
- Bearer token 在需要鉴权的请求(`me`)手动注入 header,client 层不全局注入(避免登录请求误带旧 token)。

### 4.4 跨域方案(dev proxy)

前端永远调相对路径 `/api/*`,由 dev server 转发到 `http://localhost:3000`。

- **首选**:验证 Dioxus 0.7 的 `Dioxus.toml` proxy 配置语法并启用。实现时先查 Dioxus 0.7 文档确认正确 key。
- **兜底**:若 Dioxus 0.7 dev server 不支持 proxy,后端 `crates/api` 临时加 `tower-http` `CorsLayer` 允许 `localhost:8080`(仅 dev)。
- 生产由反向代理(nginx)统一收口,前端仍调相对 `/api`。

### 4.5 Splash 启动页流程

```
App 挂载
  ├─ use_context_provider(Session { state: Signal::new(Loading) })
  └─ Splash 组件:
       use_effect {
         let stored = load_tokens();
         match stored {
           None => { state.set(Guest); navigator.replace(Login) }
           Some(t) if expired(t) =>
             match refresh(t.refresh).await {
               Ok(r) => { save_tokens(r.access, t.refresh, r.expires_in);
                          match me(r.access).await {
                            Ok(u) => { state.set(Authenticated{...}); navigator.replace(Briefing) }
                            Err(_) => { clear_tokens(); state.set(Guest); navigator.replace(Login) }
                          }
             }
               Err(_) => { clear_tokens(); state.set(Guest); navigator.replace(Login) }
             }
           Some(t) =>
             match me(t.access).await {
               Ok(u) => { state.set(Authenticated{...}); navigator.replace(Briefing) }
               Err(_) => { /* 401 则尝试 refresh,见 4.6 */ }
             }
         }
       }
       rsx! { /* 闪屏:logo + 旋转 paw */ }
```

- 闪屏显示 logo + 加载动画,避免白屏。
- `navigator.replace` 确保返回键不回 Splash。

### 4.6 运行期自动 refresh(401 拦截)

简化策略:**不在每次调用前预检**,而是在 `api/client.rs` 统一封装里拦截 401:

```
请求 -> 收到 401 ->
  读 localStorage 的 refresh_token ->
  refresh() 成功 -> 保存新 token -> 用新 access_token 重试原请求(仅重试一次) ->
    成功:返回结果
    失败:返回错误
  refresh() 失败 -> clear_tokens + state.set(Guest) + navigator.replace(Login) -> 返回错误
```

- 仅重试一次,避免无限循环。
- 并发 refresh 竞态:用 `Mutex<Option<SharedRefreshFuture>>` 合并并发 refresh 请求(实现时按需,初版可串行)。
- 对调用方透明:`me()` 等函数内部已处理。

### 4.7 Login / Register UI

**共同布局:** 顶部 logo + slogan、中间表单卡、底部切换链接。

Login:
```
┌─────────────────────────┐
│        🐶 logo          │
│   小狗人生 · Puppy Life OS│
│  用科技的温度,延伸爱的刻度 │
│  ┌───────────────────┐  │
│  │ 账号              │  │
│  └───────────────────┘  │
│  ┌───────────────────┐  │
│  │ 密码           👁 │  │
│  └───────────────────┘  │
│  [    登录    ]         │
│  忘记密码?  |  注册账号  │
└─────────────────────────┘
```

Register:多一个 nickname 字段 + 确认密码字段。提交成功后自动登录(用返回的 token 直接 set Authenticated + 跳 Briefing)。

### 4.8 表单校验(与后端一致)

| 字段     | 规则                                  | 错误提示                       |
|----------|---------------------------------------|--------------------------------|
| account  | ≥6 位,仅字母数字下划线               | "账号至少6位字母数字下划线"    |
| password | ≥8 位,必须含字母 + 数字              | "密码至少8位,需含字母和数字"  |
| nickname | 1-20 字符                             | "昵称1-20字符"                 |
| confirm  | 与 password 一致                      | "两次密码不一致"               |

- `onblur` 失焦校验 + 提交时全量校验,错误显示在输入框下方红字。
- 后端返回 `code != 0`:能映射到具体字段的显示在字段下,否则顶部 toast。

### 4.9 退出登录

`Me` 页底部加"退出登录"按钮:`clear_tokens()` + `state.set(Guest)` + `navigator.replace(Login)`。

## 5. 依赖新增

`crates/client_app/Cargo.toml` 新增:
- `reqwest`(workspace.dependencies 新增,features `["json"]`,web 平台需 `["default-tls"]` 关闭或用 `rustls-tls`,实现时按 Dioxus 0.7 web 目标验证)
- `serde`(workspace 已有,features `["derive"]`)
- `serde_json`(workspace 已有)
- `chrono`(workspace 已有,用于 expires_at 计算)

web 平台 localStorage 通过 `dioxus::web::web_sys` 或 `web-sys` crate 访问(实现时确认 Dioxus 0.7 推荐方式)。

## 6. 验证清单

- [ ] `cargo check -p client_app --all-features` 通过
- [ ] `dx serve` 启动无报错
- [ ] Briefing 宠物头像视觉中心在距顶 ~70px,插画背景动画播放
- [ ] 未登录访问 `/briefing` 自动跳 `/login`
- [ ] 注册 -> 自动登录 -> 跳 Briefing 全链路通
- [ ] 登录 -> 刷新页面 -> 仍保持登录态(localStorage 恢复)
- [ ] token 过期后请求自动 refresh 并重试成功
- [ ] refresh 也失败时自动登出跳 Login
- [ ] 表单校验规则与后端一致(前端放过时后端也放过)
- [ ] 退出登录后无法返回受保护页面

## 7. 非目标(YAGNI)

- 忘记密码 / 短信验证码 / 微信登录(后端未实现)
- 登录页的第三方登录入口
- 多账号切换 UI
- token 的加密存储(localStorage 明文,够用)
- 完整的 RBAC 权限分级 UI(仅区分登录/未登录)
