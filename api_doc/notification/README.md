# 通知模块 API（Notification）

> 对应：站内通知（权限变更提醒、日程提醒、审核通知等）
> PRD 参考：第三章（权限变更通知）、第七章（动态墙通知）

## 接口清单

| # | 接口 | 方法 | 路径 | 说明 |
|---|------|:----:|------|------|
| 1 | 获取通知列表 | GET | `/notifications` | 分页+分类 |
| 2 | 获取未读通知数 | GET | `/notifications/unread-count` | 红点数字 |
| 3 | 标记已读 | PUT | `/notifications/{notificationId}/read` | 单条已读 |
| 4 | 全部标记已读 | PUT | `/notifications/read-all` | 批量 |
| 5 | 获取通知设置 | GET | `/notifications/settings` | 推送偏好 |
| 6 | 更新通知设置 | PUT | `/notifications/settings` | 开关推送渠道 |
| 7 | 注册推送设备 | POST | `/notifications/devices` | 极光/个推 token |

---

## 1. 获取通知列表

**GET** `/notifications`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| type | string | 否 | 类型：`permission`（权限）/ `schedule`（日程）/ `approval`（审核）/ `system`（系统） |
| isRead | boolean | 否 | 是否已读 |
| page | int | 否 | 页码 |
| pageSize | int | 否 | 每页条数 |

### 响应 data

```json
{
  "list": [
    {
      "notificationId": "nt_001",
      "type": "permission",
      "typeLabel": "权限变更",
      "title": "阿哲 调整了你的权限",
      "body": "新增"管理日程"权限",
      "isRead": false,
      "createdAt": "2026-07-08T09:41:00+08:00",
      "relatedFamilyId": "fam_xyz789",
      "relatedMemberId": "u_abc123"
    },
    {
      "notificationId": "nt_002",
      "type": "schedule",
      "typeLabel": "日程提醒",
      "title": "体外驱虫已逾期 2 天",
      "body": "福来恩 · 上次 6月5日，请尽快补做",
      "isRead": false,
      "createdAt": "2026-07-08T08:00:00+08:00",
      "relatedPetId": "pet_001",
      "relatedTaskId": "tsk_001"
    },
    {
      "notificationId": "nt_003",
      "type": "approval",
      "typeLabel": "审核通知",
      "title": "小棠 申请加入阿哲的家",
      "body": "自选角色：奶妈 · 10 分钟前申请",
      "isRead": true,
      "createdAt": "2026-07-08T09:30:00+08:00",
      "relatedFamilyId": "fam_xyz789",
      "relatedRequestId": "req_abc123"
    }
  ],
  "total": 8,
  "page": 1,
  "pageSize": 20,
  "hasMore": false
}
```

---

## 2. 获取未读通知数

**GET** `/notifications/unread-count`

### 响应 data

```json
{
  "totalUnread": 3,
  "byType": {
    "permission": 1,
    "schedule": 2,
    "approval": 0,
    "system": 0
  }
}
```

---

## 3. 标记已读

**PUT** `/notifications/{notificationId}/read`

### 响应 data

```json
{
  "notificationId": "nt_001",
  "isRead": true
}
```

---

## 4. 全部标记已读

**PUT** `/notifications/read-all`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| type | string | 否 | 限定类型 |

### 响应 data

```json
{
  "readCount": 5
}
```

---

## 5. 获取通知设置

**GET** `/notifications/settings`

### 响应 data

```json
{
  "pushEnabled": true,
  "channels": {
    "inApp": {
      "enabled": true,
      "types": {
        "schedule_reminder": true,
        "permission_change": true,
        "approval_request": true,
        "ai_complete": true,
        "memory_generated": true,
        "system_notice": true
      }
    },
    "push": {
      "enabled": true,
      "types": {
        "schedule_reminder": true,
        "permission_change": true,
        "approval_request": true,
        "ai_complete": false,
        "memory_generated": true,
        "system_notice": false
      }
    }
  }
}
```

---

## 6. 更新通知设置

**PUT** `/notifications/settings`

### 请求参数

按 [5. 获取通知设置](#5-获取通知设置) 的格式提交部分更新。

---

## 7. 注册推送设备

**POST** `/notifications/devices`

用于极光/个推推送通道注册。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| platform | string | 是 | `ios` / `android` |
| pushToken | string | 是 | 厂商推送 token |
| provider | string | 是 | `jpush`（极光）/ `getui`（个推） |

### 响应 data

```json
{
  "deviceId": "dv_001",
  "registered": true
}
```
