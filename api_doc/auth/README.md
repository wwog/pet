# 认证模块 API（Auth）

> 对应界面：登录页、注册页（设计中未独立绘制，PRD 第二章 2.1 明确要求"手机号+验证码"或"微信一键登录"，不设独立密码）

## 接口清单

| # | 接口 | 方法 | 路径 | 说明 |
|---|------|:----:|------|------|
| 1 | 发送短信验证码 | POST | `/auth/sms/send` | 手机号注册/登录前获取验证码 |
| 2 | 手机号验证码登录/注册 | POST | `/auth/sms/login` | 验证码校验，新用户自动注册 |
| 3 | 微信一键登录 | POST | `/auth/wechat/login` | 微信 OAuth 授权码换 token |
| 4 | 刷新 Token | POST | `/auth/token/refresh` | access_token 过期后用 refresh_token 换新 |
| 5 | 退出登录 | POST | `/auth/logout` | 注销当前会话 |
| 6 | 获取当前用户信息 | GET | `/auth/me` | 获取登录用户基本信息 + 所属家庭列表 |
| 7 | 更新用户昵称 | PUT | `/auth/me/nickname` | 注册后填写昵称（触发默认家庭创建） |
| 8 | 注销账号 | DELETE | `/auth/account` | 注销账号，脱敏保留操作记录 |

---

## 1. 发送短信验证码

**POST** `/auth/sms/send`

发送短信验证码到指定手机号，用于注册或登录。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| phone | string | 是 | 手机号，含国际区号如 `+8613800006629` |
| scene | string | 是 | 场景：`login` / `bind_wechat` |

### 响应 data

```json
{
  "expireSeconds": 300,
  "nextSendSeconds": 60
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| expireSeconds | int | 验证码有效期（秒） |
| nextSendSeconds | int | 下次可发送的间隔（秒），用于前端倒计时 |

### 错误码

| code | 说明 |
|------|------|
| 1004 | 手机号格式错误 |
| 1006 | 发送过于频繁，请等待倒计时结束 |

---

## 2. 手机号验证码登录/注册

**POST** `/auth/sms/login`

验证码校验通过后，若手机号已存在则登录，不存在则自动注册。注册成功后系统自动创建以"XX（用户昵称）的家"命名的默认家庭，用户成为该家庭的**首席监护人**。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| phone | string | 是 | 手机号 `+8613800006629` |
| code | string | 是 | 6 位短信验证码 |
| nickname | string | 否 | 首次注册时的昵称；新用户未填则返回 `isNewUser: true` 引导填写 |

### 响应 data

```json
{
  "accessToken": "eyJhbGciOi...",
  "refreshToken": "eyJhbGciOi...",
  "expiresIn": 7200,
  "isNewUser": false,
  "user": {
    "userId": "u_abc123",
    "phone": "138****6629",
    "nickname": "阿哲",
    "avatar": "https://cdn.puppy-life.com/avatar/u_abc123.jpg",
    "hasPet": true
  },
  "defaultFamilyId": "fam_xyz789",
  "families": [
    {
      "familyId": "fam_xyz789",
      "familyName": "阿哲的家",
      "role": "首席监护人",
      "isGuardian": true,
      "petCount": 1,
      "memberCount": 3
    }
  ]
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| accessToken | string | 访问令牌，有效期 2h |
| refreshToken | string | 刷新令牌，有效期 30d |
| expiresIn | int | accessToken 有效期（秒） |
| isNewUser | boolean | 是否为新注册用户 |
| user.userId | string | 用户唯一 ID |
| user.phone | string | 脱敏手机号 |
| user.nickname | string? | 昵称，新用户可能为 null |
| user.avatar | string? | 头像 URL |
| user.hasPet | boolean | 是否已添加宠物 |
| defaultFamilyId | string | 默认家庭 ID |
| families | array | 用户所属的所有家庭列表 |
| families[].role | string | 在该家庭的角色标签 |
| families[].isGuardian | boolean | 是否为首席监护人 |

### 错误码

| code | 说明 |
|------|------|
| 1004 | 验证码错误或已过期 |

---

## 3. 微信一键登录

**POST** `/auth/wechat/login`

使用微信 OAuth 授权码登录，自动注册或绑定已有账号。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| code | string | 是 | 微信 OAuth 授权码 |
| phone | string | 否 | 首次登录需绑定手机号（配合验证码） |
| smsCode | string | 否 | 手机号绑定时的验证码 |

### 响应 data

同 [2. 手机号验证码登录/注册](#2-手机号验证码登录注册) 响应结构。

### 错误码

| code | 说明 |
|------|------|
| 1004 | 微信授权码无效 |
| 1005 | 微信 openId 已绑定其他账号 |

---

## 4. 刷新 Token

**POST** `/auth/token/refresh`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| refreshToken | string | 是 | 刷新令牌 |

### 响应 data

```json
{
  "accessToken": "eyJhbGciOi...",
  "expiresIn": 7200
}
```

### 错误码

| code | 说明 |
|------|------|
| 1001 | refreshToken 无效或已过期，需重新登录 |

---

## 5. 退出登录

**POST** `/auth/logout`

注销当前设备会话，使 accessToken 和 refreshToken 失效。

### 请求参数

无（通过 Authorization 头识别用户）

### 响应 data

```json
null
```

---

## 6. 获取当前用户信息

**GET** `/auth/me`

获取登录用户基本信息及所属家庭列表，用于 App 启动初始化。

### 响应 data

```json
{
  "userId": "u_abc123",
  "phone": "138****6629",
  "nickname": "阿哲",
  "avatar": "https://cdn.puppy-life.com/avatar/u_abc123.jpg",
  "wechatBound": true,
  "families": [
    {
      "familyId": "fam_xyz789",
      "familyName": "阿哲的家",
      "familyAvatar": "https://cdn.puppy-life.com/family/fam_xyz789.jpg",
      "city": "杭州",
      "role": "首席监护人",
      "isGuardian": true,
      "petCount": 1,
      "memberCount": 3,
      "memberAvatars": ["url1", "url2", "url3"]
    }
  ]
}
```

---

## 7. 更新用户昵称

**PUT** `/auth/me/nickname`

新用户注册后首次填写昵称，触发默认家庭自动创建。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| nickname | string | 是 | 昵称，1-20 字符 |

### 响应 data

```json
{
  "userId": "u_abc123",
  "nickname": "阿哲",
  "defaultFamilyId": "fam_xyz789"
}
```

### 说明

- 若为首次设置昵称（`isNewUser`），后端自动创建默认家庭，命名为"{nickname}的家"，用户自动成为首席监护人。
- 默认家庭创建后返回 `defaultFamilyId`。

---

## 8. 注销账号

**DELETE** `/auth/account`

注销账号后清除所有个人身份信息，但家庭内操作记录脱敏保留（显示为"已注销用户"）。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| reason | string | 否 | 注销原因 |
| smsCode | string | 是 | 短信验证码（二次确认） |

### 响应 data

```json
{
  "scheduledAt": "2026-07-08T09:41:00+08:00",
  "retentionDays": 30,
  "message": "账号将于 30 天后永久注销，期间可恢复。导出功能保留 30 天。"
}
```

| 字段 | 类型 | 说明 |
|------|------|------|
| scheduledAt | string | 计划注销时间 |
| retentionDays | int | 数据保留天数（可恢复期） |
| message | string | 提示文案 |

### 说明

- 注销非即时执行，设置 30 天宽限期，期间用户可登录恢复。
- 若用户为首席监护人，需先转让首席监护人身份，否则返回 `2001` 错误。
- 宽限期结束后，个人信息永久删除，操作记录中的用户名替换为"已注销用户"。
