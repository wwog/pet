# 家庭与权限模块 API（Family）

> 对应界面：`family-permissions.html`、`tool-invite.html`、首页家庭切换器
> PRD 参考：第二章 用户与家庭体系、第三章 权限系统（柔性RBAC）

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取家庭列表 | GET | `/families` | 登录 | 当前用户所属的所有家庭 |
| 2 | 切换当前家庭 | PUT | `/families/current` | 登录 | 设置当前活跃家庭 |
| 3 | 获取家庭详情 | GET | `/families/{familyId}` | 家庭成员 | 家庭信息+宠物+成员概览 |
| 4 | 更新家庭信息 | PUT | `/families/{familyId}` | P12 | 修改家庭名称、头像、城市 |
| 5 | 获取成员列表 | GET | `/families/{familyId}/members` | 家庭成员 | 含角色标签与权限概要 |
| 6 | 获取成员详情与权限 | GET | `/families/{familyId}/members/{memberId}` | P12 | 完整权限矩阵 |
| 7 | 更新成员角色标签 | PUT | `/families/{familyId}/members/{memberId}/role` | P12 | 修改角色标签（仅展示用） |
| 8 | 更新成员权限 | PUT | `/families/{familyId}/members/{memberId}/permissions` | P12 | 逐项开关 P1-P11 |
| 9 | 应用权限模板 | POST | `/families/{familyId}/members/{memberId}/permissions/template` | P12 | 快速套用权限模板 |
| 10 | 获取权限模板列表 | GET | `/families/permission-templates` | P12 | 仅查看/奶妈全选/管家/摄影师 |
| 11 | 移除家庭成员 | DELETE | `/families/{familyId}/members/{memberId}` | P12 | 移出家庭 |
| 12 | 转让首席监护人 | POST | `/families/{familyId}/transfer-guardian` | P12 | 转让最高管理者身份 |
| 13 | 生成邀请码 | POST | `/families/{familyId}/invite-code` | P12 | 生成6位邀请码 |
| 14 | 凭邀请码预览加入信息 | GET | `/families/invite/{code}` | 登录 | 预览家庭信息与默认权限 |
| 15 | 申请加入家庭 | POST | `/families/invite/{code}/apply` | 登录 | 提交加入申请，含自选角色 |
| 16 | 获取待审核申请列表 | GET | `/families/{familyId}/join-requests` | P12 | 待审核的加入申请 |
| 17 | 审核加入申请 | PUT | `/families/{familyId}/join-requests/{requestId}` | P12 | 通过/拒绝 |
| 18 | 获取角色标签库 | GET | `/families/role-tags` | 登录 | 预设角色标签+自定义 |

---

## 1. 获取家庭列表

**GET** `/families`

### 响应 data

```json
{
  "list": [
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
  ],
  "currentFamilyId": "fam_xyz789"
}
```

---

## 2. 切换当前家庭

**PUT** `/families/current`

切换后全站数据同步切换，前端设置 `X-Family-Id` 头。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| familyId | string | 是 | 目标家庭 ID |

### 响应 data

```json
{
  "familyId": "fam_xyz789",
  "familyName": "阿哲的家",
  "pets": [
    { "petId": "pet_001", "name": "豆豆", "breed": "金毛寻回犬", "avatar": "url" }
  ]
}
```

---

## 3. 获取家庭详情

**GET** `/families/{familyId}`

### 响应 data

```json
{
  "familyId": "fam_xyz789",
  "familyName": "阿哲的家",
  "familyAvatar": "url",
  "city": "杭州",
  "memberCount": 3,
  "maxMembers": 20,
  "petCount": 1,
  "maxPets": 5,
  "guardianId": "u_abc123",
  "guardianName": "阿哲",
  "pets": [
    {
      "petId": "pet_001",
      "name": "豆豆",
      "breed": "金毛寻回犬",
      "avatar": "url",
      "gender": "male",
      "ageText": "1岁2个月"
    }
  ],
  "members": [
    {
      "memberId": "u_abc123",
      "nickname": "阿哲",
      "avatar": "url",
      "role": "首席监护人",
      "isGuardian": true,
      "joinedAt": "2024-03-01T10:00:00+08:00",
      "monthlyActionCount": 21
    }
  ]
}
```

---

## 4. 更新家庭信息

**PUT** `/families/{familyId}`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| familyName | string | 否 | 家庭名称 |
| familyAvatar | string | 否 | 头像 URL（先通过 upload 接口获取） |
| city | string | 否 | 所在城市 |

### 响应 data

返回更新后的家庭信息，同 [3. 获取家庭详情](#3-获取家庭详情)。

---

## 5. 获取成员列表

**GET** `/families/{familyId}/members`

### 响应 data

```json
{
  "list": [
    {
      "memberId": "u_abc123",
      "nickname": "阿哲",
      "avatar": "url",
      "phone": "138****6629",
      "role": "首席监护人",
      "isGuardian": true,
      "isMe": true,
      "permissionSummary": "P1-P12（全部）",
      "joinedAt": "2024-03-01T10:00:00+08:00",
      "monthlyActionCount": 21,
      "monthlyTitle": "🏆 最佳铲屎官"
    },
    {
      "memberId": "u_def456",
      "nickname": "小棠",
      "avatar": "url",
      "phone": "138****6629",
      "role": "奶妈",
      "isGuardian": false,
      "isMe": false,
      "permissionSummary": "P1-P10（10项）",
      "joinedAt": "2024-03-05T14:00:00+08:00",
      "monthlyActionCount": 16,
      "monthlyTitle": "🦴 遛弯冠军"
    }
  ]
}
```

---

## 6. 获取成员详情与权限

**GET** `/families/{familyId}/members/{memberId}`

返回完整的 12 项权限矩阵，用于权限编辑页。

### 响应 data

```json
{
  "memberId": "u_def456",
  "nickname": "小棠",
  "avatar": "url",
  "role": "奶妈",
  "isGuardian": false,
  "joinedAt": "2024-03-05T14:00:00+08:00",
  "monthlyActionCount": 16,
  "permissions": {
    "P1": { "name": "查看宠物基础档案", "enabled": true, "editable": true },
    "P2": { "name": "编辑宠物基础档案", "enabled": false, "editable": true },
    "P3": { "name": "查看健康记录", "enabled": true, "editable": true },
    "P4": { "name": "编辑/新增健康记录", "enabled": false, "editable": true },
    "P5": { "name": "查看每日事件/日记", "enabled": true, "editable": true },
    "P6": { "name": "编辑/新增每日事件", "enabled": true, "editable": true },
    "P7": { "name": "查看日程列表", "enabled": true, "editable": true },
    "P8": { "name": "管理日程", "enabled": true, "editable": true },
    "P9": { "name": "查看相册", "enabled": true, "editable": true },
    "P10": { "name": "管理相册", "enabled": true, "editable": true },
    "P11": { "name": "使用AI对话", "enabled": false, "editable": true },
    "P12": { "name": "管理家庭成员", "enabled": false, "editable": false, "locked": true, "reason": "仅首席监护人专属" }
  }
}
```

### 说明

- `P12` 的 `editable` 始终为 `false`，`locked` 为 `true`，前端展示为锁定状态。
- 若被查看人为首席监护人，P1-P11 全部 `enabled: true` 且 `editable: false`（不可被修改）。

---

## 7. 更新成员角色标签

**PUT** `/families/{familyId}/members/{memberId}/role`

角色仅用于操作留痕展示，与权限完全解耦。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| role | string | 是 | 角色标签名称，如"奶妈"、"管家"或自定义 |

### 响应 data

```json
{
  "memberId": "u_def456",
  "role": "管家"
}
```

### 说明

- 角色变更实时生效，被修改人将收到站内通知。
- 操作记录写入动态墙："首席监护人 X 将 Y 的角色标签从'奶妈'改为'管家'"。
- 首席监护人的角色标签不可更改。

---

## 8. 更新成员权限

**PUT** `/families/{familyId}/members/{memberId}/permissions`

逐项开关权限，实时生效。

### 请求参数

```json
{
  "permissions": {
    "P1": true,
    "P2": false,
    "P3": true,
    "P4": true,
    "P5": true,
    "P6": true,
    "P7": true,
    "P8": true,
    "P9": true,
    "P10": true,
    "P11": false
  }
}
```

### 响应 data

```json
{
  "memberId": "u_def456",
  "permissions": {
    "P1": true,
    "P2": false,
    "P3": true,
    "P4": true,
    "P5": true,
    "P6": true,
    "P7": true,
    "P8": true,
    "P9": true,
    "P10": true,
    "P11": false,
    "P12": false
  },
  "updatedFields": ["P4", "P11"],
  "notifiedAt": "2026-07-08T09:41:00+08:00"
}
```

### 说明

- 请求中只包含 P1-P11，P12 不可通过此接口修改。
- 权限变更实时生效，被修改人收到站内通知。
- 若被修改人正在执行被移除权限的操作，系统强制中断并提示刷新。
- 操作记录写入动态墙："首席监护人 X 调整了 Y 的权限：新增'管理日程'"。

---

## 9. 应用权限模板

**POST** `/families/{familyId}/members/{memberId}/permissions/template`

快速套用权限模板，套用后可继续手动微调。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| templateId | string | 是 | 模板 ID |

### 响应 data

返回套用后的完整权限矩阵，同 [6. 获取成员详情与权限](#6-获取成员详情与权限) 中的 `permissions`。

---

## 10. 获取权限模板列表

**GET** `/families/permission-templates`

### 响应 data

```json
{
  "list": [
    {
      "templateId": "tpl_view",
      "name": "仅查看",
      "description": "P1, P3, P5, P7, P9（只读）",
      "permissions": { "P1": true, "P2": false, "P3": true, "P4": false, "P5": true, "P6": false, "P7": true, "P8": false, "P9": true, "P10": false, "P11": false }
    },
    {
      "templateId": "tpl_mom",
      "name": "奶妈全选",
      "description": "P1-P10 全选",
      "permissions": { "P1": true, "P2": true, "P3": true, "P4": true, "P5": true, "P6": true, "P7": true, "P8": true, "P9": true, "P10": true, "P11": false }
    },
    {
      "templateId": "tpl_manager",
      "name": "管家模板",
      "description": "全档案管理 + 日程管理",
      "permissions": { "P1": true, "P2": true, "P3": true, "P4": true, "P5": true, "P6": true, "P7": true, "P8": true, "P9": true, "P10": false, "P11": false }
    },
    {
      "templateId": "tpl_photo",
      "name": "摄影师",
      "description": "P9, P10 + P5",
      "permissions": { "P1": true, "P2": false, "P3": false, "P4": false, "P5": true, "P6": false, "P7": false, "P8": false, "P9": true, "P10": true, "P11": false }
    }
  ]
}
```

---

## 11. 移除家庭成员

**DELETE** `/families/{familyId}/members/{memberId}`

### 响应 data

```json
{
  "removedMemberId": "u_def456",
  "removedAt": "2026-07-08T09:41:00+08:00"
}
```

### 说明

- 首席监护人不可移除自己（需先转让身份），返回 `2001` 错误。
- 被移除用户在其他家庭的数据不受影响。
- 操作记录写入动态墙。

---

## 12. 转让首席监护人

**POST** `/families/{familyId}/transfer-guardian`

将首席监护人身份转让给指定成员。转让后自己降级为普通成员，失去 P12 权限，其余权限保留不变。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| targetMemberId | string | 是 | 接收首席监护人身份的成员 ID |

### 响应 data

```json
{
  "familyId": "fam_xyz789",
  "newGuardianId": "u_def456",
  "newGuardianName": "小棠",
  "previousGuardianId": "u_abc123",
  "transferredAt": "2026-07-08T09:41:00+08:00"
}
```

### 说明

- 转让后原首席监护人自动获得 P1-P11 全部权限（保留不变），仅失去 P12。
- 全家庭成员收到通知。
- 操作记录写入动态墙。

---

## 13. 生成邀请码

**POST** `/families/{familyId}/invite-code`

生成 6 位数字邀请码，有效期 7 天。

### 请求参数

无

### 响应 data

```json
{
  "code": "739KM2",
  "expireAt": "2026-07-15T09:41:00+08:00",
  "shareUrl": "https://puppy.life/invite/739KM2",
  "wxShareTitle": "阿哲邀请你加入「阿哲的家」",
  "wxShareDesc": "一起照顾豆豆吧！"
}
```

---

## 14. 凭邀请码预览加入信息

**GET** `/families/invite/{code}`

被邀请人输入邀请码后，预览家庭信息与默认权限。

### 响应 data

```json
{
  "familyId": "fam_xyz789",
  "familyName": "阿哲的家",
  "familyAvatar": "url",
  "memberCount": 3,
  "petCount": 1,
  "pets": [
    { "petId": "pet_001", "name": "豆豆", "breed": "金毛寻回犬" }
  ],
  "defaultRole": "亲友",
  "previewPermissions": {
    "P1": { "name": "查看宠物基础档案", "enabled": true },
    "P2": { "name": "编辑宠物基础档案", "enabled": false },
    "P5": { "name": "查看每日事件/日记", "enabled": true },
    "P7": { "name": "查看日程列表", "enabled": true },
    "P9": { "name": "查看相册", "enabled": true }
  }
}
```

### 错误码

| code | 说明 |
|------|------|
| 4001 | 邀请码无效或已过期 |
| 4002 | 邀请码已被使用 |

---

## 15. 申请加入家庭

**POST** `/families/invite/{code}/apply`

被邀请人自选角色标签并提交加入申请，需首席监护人审核通过后正式生效。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| role | string | 是 | 自选角色标签，如"奶妈" |
| note | string | 否 | 申请备注 |

### 响应 data

```json
{
  "requestId": "req_abc123",
  "familyId": "fam_xyz789",
  "familyName": "阿哲的家",
  "status": "pending",
  "submittedAt": "2026-07-08T09:41:00+08:00"
}
```

### 说明

- 申请状态：`pending`（待审核）→ `approved`（通过）/ `rejected`（拒绝）。

---

## 16. 获取待审核申请列表

**GET** `/families/{familyId}/join-requests`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| status | string | 否 | `pending` / `approved` / `rejected`，默认 `pending` |

### 响应 data

```json
{
  "list": [
    {
      "requestId": "req_abc123",
      "applicantId": "u_new789",
      "applicantNickname": "小棠",
      "applicantPhone": "138****6629",
      "applicantAvatar": "url",
      "selectedRole": "奶妈",
      "previewPermissions": ["P1", "P5", "P6", "P7", "P8", "P9", "P10"],
      "note": "我是阿哲的伴侣",
      "submittedAt": "2026-07-08T09:30:00+08:00",
      "status": "pending"
    }
  ]
}
```

---

## 17. 审核加入申请

**PUT** `/families/{familyId}/join-requests/{requestId}`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| action | string | 是 | `approve` / `reject` |
| permissions | object | 否 | 通过时可调整权限覆盖（同 [8. 更新成员权限](#8-更新成员权限) 格式） |

### 响应 data

```json
{
  "requestId": "req_abc123",
  "status": "approved",
  "newMemberId": "u_new789",
  "notifiedAt": "2026-07-08T09:41:00+08:00"
}
```

### 说明

- 通过后申请人正式成为家庭成员，权限实时生效。
- 拒绝后通知申请人，邀请码不变。
- 操作记录写入动态墙。

---

## 18. 获取角色标签库

**GET** `/families/role-tags`

返回预设角色标签列表，支持自定义。

### 响应 data

```json
{
  "presetTags": [
    { "tagId": "rt_guardian", "name": "首席监护人", "emoji": "👑", "editable": false, "note": "特殊标识，不可更改" },
    { "tagId": "rt_dad", "name": "奶爸", "emoji": "👨" },
    { "tagId": "rt_mom", "name": "奶妈", "emoji": "🍼" },
    { "tagId": "rt_grandpa", "name": "爷爷", "emoji": "👴" },
    { "tagId": "rt_grandma", "name": "奶奶", "emoji": "👵" },
    { "tagId": "rt_manager", "name": "管家", "emoji": "📋" },
    { "tagId": "rt_assistant", "name": "助理", "emoji": "📎" },
    { "tagId": "rt_health", "name": "健康顾问", "emoji": "🩺" },
    { "tagId": "rt_photo", "name": "家庭摄影师", "emoji": "📷" },
    { "tagId": "rt_friend", "name": "亲友", "emoji": "👋" },
    { "tagId": "rt_temp", "name": "临时照料人", "emoji": "⏳" }
  ],
  "customTags": [
    { "tagId": "rt_custom_001", "name": "御用铲屎官", "emoji": null }
  ],
  "allowCustom": true
}
```
