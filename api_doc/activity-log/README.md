# 动态墙模块 API（Activity Log）

> 对应界面：`pet-profile.html`（动态墙Tab + 本月活跃成员）
> PRD 参考：第七章 操作留痕与家庭动态墙

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取动态墙列表 | GET | `/pets/{petId}/activity-feed` | P5 | 分页+筛选 |
| 2 | 获取本月活跃成员 | GET | `/pets/{petId}/activity-feed/monthly-stats` | P5 | 责任可视化 |
| 3 | 获取动态详情 | GET | `/pets/{petId}/activity-feed/{feedId}` | P5 | 展开详情 |
| 4 | 获取操作统计 | GET | `/pets/{petId}/activity-feed/statistics` | P5 | 按成员/类型统计 |
| 5 | 获取责任称号 | GET | `/pets/{petId}/activity-feed/titles` | P5 | 趣味称号 |

---

## 1. 获取动态墙列表

**GET** `/pets/{petId}/activity-feed`

对应 `pet-profile.html` 动态墙主体。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| filter | string | 否 | 筛选：`all`（全部）/ `medical`（医疗）/ `feeding`（喂养）/ `walk`（遛弯）/ `photo`（照片）/ `permission`（权限变更） |
| memberId | string | 否 | 按成员筛选 |
| page | int | 否 | 页码 |
| pageSize | int | 否 | 每页条数，默认 20 |

### 响应 data

```json
{
  "list": [
    {
      "feedId": "af_001",
      "type": "weight_update",
      "typeLabel": "医疗",
      "isMedical": true,
      "actor": {
        "memberId": "u_abc123",
        "nickname": "阿哲",
        "role": "奶爸",
        "avatar": "url",
        "avatarColor": "accent"
      },
      "action": "更新了豆豆的体重",
      "detail": "12.5kg → 13.0kg，并咨询了呕吐问题，AI 建议观察至明日。",
      "timestamp": "2026-07-08T09:40:00+08:00",
      "timeText": "9:40",
      "expandable": true,
      "expandContent": {
        "drugName": "福来恩",
        "aiConversationId": "cs_002"
      }
    },
    {
      "feedId": "af_002",
      "type": "event_record",
      "typeLabel": "事件日记",
      "isMedical": false,
      "actor": {
        "memberId": "u_def456",
        "nickname": "小棠",
        "role": "奶妈",
        "avatar": "url",
        "avatarColor": "mint"
      },
      "action": "记录了豆豆今天在公园学会了\"握手\"。",
      "timestamp": "2026-07-07T18:30:00+08:00",
      "timeText": "昨天 18:30"
    },
    {
      "feedId": "af_003",
      "type": "task_overdue",
      "typeLabel": "医疗",
      "isMedical": true,
      "actor": {
        "memberId": "u_grandpa",
        "nickname": "老张",
        "role": "爷爷",
        "avatar": "url",
        "avatarColor": "tan"
      },
      "action": "标记了\"体外驱虫\"任务为待补做（福来恩）。",
      "timestamp": "2026-07-07T17:00:00+08:00",
      "timeText": "昨天 17:00"
    },
    {
      "feedId": "af_004",
      "type": "photo_upload",
      "typeLabel": "照片",
      "isMedical": false,
      "actor": {
        "memberId": "u_grandpa",
        "nickname": "老张",
        "role": "爷爷",
        "avatar": "url",
        "avatarColor": "brown"
      },
      "action": "上传了 3 张新照片",
      "detail": "公园散步",
      "timestamp": "2026-07-06T10:00:00+08:00",
      "timeText": "7.06"
    },
    {
      "feedId": "af_005",
      "type": "permission_change",
      "typeLabel": "权限变更",
      "isMedical": false,
      "actor": {
        "memberId": "u_abc123",
        "nickname": "阿哲",
        "role": "首席监护人",
        "avatar": "url",
        "avatarColor": "accent"
      },
      "action": "调整了 小棠 的权限",
      "detail": "新增\"管理日程\"",
      "timestamp": "2026-07-05T14:00:00+08:00",
      "timeText": "7.05"
    }
  ],
  "total": 89,
  "page": 1,
  "pageSize": 20,
  "hasMore": true
}
```

---

## 2. 获取本月活跃成员

**GET** `/pets/{petId}/activity-feed/monthly-stats`

对应 `pet-profile.html` 顶部"本月活跃成员"卡片——责任可视化。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| month | string | 否 | 月份，默认当月 |

### 响应 data

```json
{
  "month": "2026-07",
  "monthLabel": "7月",
  "totalRecords": 47,
  "members": [
    {
      "memberId": "u_abc123",
      "nickname": "阿哲",
      "role": "奶爸",
      "avatar": "url",
      "avatarColor": "accent",
      "recordCount": 21,
      "title": "🏆 最佳铲屎官",
      "rank": 1
    },
    {
      "memberId": "u_def456",
      "nickname": "小棠",
      "role": "奶妈",
      "avatar": "url",
      "avatarColor": "mint",
      "recordCount": 16,
      "title": "🦴 遛弯冠军",
      "rank": 2
    }
  ]
}
```

---

## 3. 获取动态详情

**GET** `/pets/{petId}/activity-feed/{feedId}`

展开单条动态的完整详情。

### 响应 data

```json
{
  "feedId": "af_001",
  "type": "weight_update",
  "isMedical": true,
  "actor": {
    "memberId": "u_abc123",
    "nickname": "阿哲",
    "role": "奶爸",
    "avatar": "url"
  },
  "action": "更新了豆豆的体重",
  "detail": {
    "beforeWeight": 12.5,
    "afterWeight": 13.0,
    "deltaWeight": 0.5,
    "aiConsultationTopic": "呕吐问题",
    "aiSuggestion": "AI 建议观察至明日",
    "aiConversationId": "cs_002"
  },
  "linkedPhotos": [],
  "permissionsChanges": null,
  "timestamp": "2026-07-08T09:40:00+08:00"
}
```

---

## 4. 获取操作统计

**GET** `/pets/{petId}/activity-feed/statistics`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| month | string | 否 | 月份，默认当月 |
| memberId | string | 否 | 成员筛选 |

### 响应 data

```json
{
  "month": "2026-07",
  "totalRecords": 47,
  "byType": [
    { "type": "event_record", "label": "事件日记", "count": 15 },
    { "type": "photo_upload", "label": "照片上传", "count": 12 },
    { "type": "task_complete", "label": "完成任务", "count": 8 },
    { "type": "weight_update", "label": "体重更新", "count": 3 },
    { "type": "health_record", "label": "健康记录", "count": 5 },
    { "type": "walk_summary", "label": "遛狗记录", "count": 2 },
    { "type": "permission_change", "label": "权限变更", "count": 1 },
    { "type": "ai_consult", "label": "AI咨询", "count": 1 }
  ],
  "byMember": [
    { "memberId": "u_abc123", "nickname": "阿哲", "count": 21 },
    { "memberId": "u_def456", "nickname": "小棠", "count": 16 },
    { "memberId": "u_grandpa", "nickname": "老张", "count": 10 }
  ]
}
```

---

## 5. 获取责任称号

**GET** `/pets/{petId}/activity-feed/titles`

趣味称号系统。

### 响应数据

```json
{
  "availableTitles": [
    { "titleId": "t_best_cleaner", "name": "最佳铲屎官", "icon": "🏆", "condition": "当月记录次数最多" },
    { "titleId": "t_walk_champ", "name": "遛弯冠军", "icon": "🦴", "condition": "当月遛狗次数最多" },
    { "titleId": "t_feeding_master", "name": "喂饭达人", "icon": "🍖", "condition": "当月喂食记录最多" },
    { "titleId": "t_photo_king", "name": "摄影大师", "icon": "📸", "condition": "当月上传照片最多" },
    { "titleId": "t_health_guard", "name": "健康守护", "icon": "🩺", "condition": "当月健康记录最多" }
  ],
  "currentAssignments": {
    "u_abc123": "best_cleaner",
    "u_def456": "walk_champ"
  }
}
```
