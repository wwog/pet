# 智能日程模块 API（Calendar）

> 对应界面：`tab-calendar.html`
> PRD 参考：第五章 5.3 日历代办·智能日程

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取日历视图 | GET | `/pets/{petId}/calendar` | P7 | 周日历+待办概要 |
| 2 | 获取任务列表 | GET | `/pets/{petId}/calendar/tasks` | P7 | 按日期/状态筛选 |
| 3 | 创建任务 | POST | `/pets/{petId}/calendar/tasks` | P8 | 新增待办或周期性日程 |
| 4 | 获取任务详情 | GET | `/pets/{petId}/calendar/tasks/{taskId}` | P7 | 单个任务详情 |
| 5 | 更新任务 | PUT | `/pets/{petId}/calendar/tasks/{taskId}` | P8 | 修改任务内容/时间 |
| 6 | 标记任务完成/取消 | PUT | `/pets/{petId}/calendar/tasks/{taskId}/complete` | P8 | 勾选完成/取消完成 |
| 7 | 删除任务 | DELETE | `/pets/{petId}/calendar/tasks/{taskId}` | P8 | 删除任务 |
| 8 | 获取智能提醒 | GET | `/pets/{petId}/calendar/smart-reminder` | P7 | 智能提醒引擎 |
| 9 | 获取附近合作医院 | GET | `/pets/{petId}/calendar/nearby-hospitals` | P7 | LBS附近医院 |
| 10 | 获取训练计划进度 | GET | `/pets/{petId}/calendar/training-plans` | P7 | 分离焦虑训练等打卡计划 |

---

## 1. 获取日历视图

**GET** `/pets/{petId}/calendar`

对应 `tab-calendar.html` 周日历视图。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| weekStart | string | 否 | 周起始日期 `2026-07-06`，默认本周 |

### 响应 data

```json
{
  "weekDays": [
    { "date": "2026-07-06", "weekday": "一", "day": 6, "hasTask": true, "isToday": false },
    { "date": "2026-07-07", "weekday": "二", "day": 7, "hasTask": true, "isToday": true },
    { "date": "2026-07-08", "weekday": "三", "day": 8, "hasTask": false, "isToday": false },
    { "date": "2026-07-09", "weekday": "四", "day": 9, "hasTask": true, "isToday": false },
    { "date": "2026-07-10", "weekday": "五", "day": 10, "hasTask": false, "isToday": false },
    { "date": "2026-07-11", "weekday": "六", "day": 11, "hasTask": true, "isToday": false },
    { "date": "2026-07-12", "weekday": "日", "day": 12, "hasTask": false, "isToday": false }
  ],
  "todaySummary": {
    "date": "2026-07-07",
    "totalTasks": 3,
    "overdueTasks": 1,
    "completedTasks": 1
  }
}
```

---

## 2. 获取任务列表

**GET** `/pets/{petId}/calendar/tasks`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| date | string | 否 | 指定日期 `2026-07-07` |
| status | string | 否 | `pending` / `completed` / `overdue` / `all` |
| assigneeId | string | 否 | 指定负责人 memberId |
| type | string | 否 | `once`（单次）/ `cycle`（周期）/ `training`（训练计划） |

### 响应 data

```json
{
  "list": [
    {
      "taskId": "tsk_001",
      "title": "体外驱虫 · 福来恩",
      "type": "cycle",
      "cycleType": "monthly",
      "priority": "high",
      "priorityLabel": "逾期",
      "status": "overdue",
      "assigneeId": "u_abc123",
      "assigneeName": "奶爸 阿哲",
      "assigneeAvatar": "url",
      "scheduledTime": null,
      "lastDoneDate": "2026-06-05",
      "overdueDays": 2,
      "note": "上次：6月5日 · 已逾期 2 天",
      "isMedical": true
    },
    {
      "taskId": "tsk_002",
      "title": "晚餐喂粮 320g",
      "type": "cycle",
      "cycleType": "daily",
      "priority": "mid",
      "priorityLabel": "18:30",
      "status": "pending",
      "assigneeId": "u_def456",
      "assigneeName": "奶妈 小棠",
      "assigneeAvatar": "url",
      "scheduledTime": "18:30"
    },
    {
      "taskId": "tsk_003",
      "title": "刷牙训练 5 分钟",
      "type": "training",
      "priority": "cycle",
      "priorityLabel": "21:00",
      "status": "pending",
      "assigneeId": "u_grandpa",
      "assigneeName": "爷爷 老张",
      "assigneeAvatar": "url",
      "scheduledTime": "21:00",
      "trainingPlan": {
        "planId": "tp_001",
        "planName": "14天刷牙训练",
        "currentDay": 7,
        "totalDays": 14
      }
    },
    {
      "taskId": "tsk_004",
      "title": "晨遛 25 分钟",
      "type": "cycle",
      "cycleType": "daily",
      "priority": "cycle",
      "priorityLabel": "已完成",
      "status": "completed",
      "assigneeId": "u_abc123",
      "assigneeName": "奶爸 阿哲",
      "assigneeAvatar": "url",
      "completedAt": "2026-07-07T07:15:00+08:00"
    }
  ]
}
```

---

## 3. 创建任务

**POST** `/pets/{petId}/calendar/tasks`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| title | string | 是 | 任务标题 |
| type | string | 是 | `once` / `cycle` / `training` |
| cycleType | string | 否 | `daily` / `weekly` / `monthly` / `quarterly`（type=cycle 时必填） |
| priority | string | 否 | `high` / `mid` / `low` |
| assigneeId | string | 否 | 负责人 memberId，默认创建者 |
| scheduledDate | string | 否 | 计划日期 |
| scheduledTime | string | 否 | 计划时间 HH:mm |
| note | string | 否 | 备注 |
| isMedical | boolean | 否 | 是否医疗操作（影响动态墙红色角标） |

### 响应 data

```json
{
  "taskId": "tsk_005",
  "title": "体内驱虫 · 拜宠清",
  "type": "cycle",
  "cycleType": "quarterly",
  "priority": "high",
  "status": "pending",
  "assigneeId": "u_abc123",
  "assigneeName": "奶爸 阿哲",
  "createdAt": "2026-07-08T09:41:00+08:00"
}
```

### 说明

- 周期性任务完成后，系统自动根据 `cycleType` 计算下次提醒日并生成新任务。

---

## 4. 获取任务详情

**GET** `/pets/{petId}/calendar/tasks/{taskId}`

### 响应 data

同 [2. 获取任务列表](#2-获取任务列表) 中单个任务对象。

---

## 5. 更新任务

**PUT** `/pets/{petId}/calendar/tasks/{taskId}`

### 请求参数

可更新 `title`、`type`、`cycleType`、`priority`、`assigneeId`、`scheduledDate`、`scheduledTime`、`note`。

### 响应 data

返回更新后的任务详情。

---

## 6. 标记任务完成/取消

**PUT** `/pets/{petId}/calendar/tasks/{taskId}/complete`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| completed | boolean | 是 | `true` 完成 / `false` 取消完成 |
| note | string | 否 | 完成备注 |

### 响应 data

```json
{
  "taskId": "tsk_001",
  "status": "completed",
  "completedAt": "2026-07-08T09:41:00+08:00",
  "completedBy": "u_grandpa",
  "nextCycleDate": "2026-08-08",
  "nextTaskId": "tsk_006"
}
```

### 说明

- 周期性任务完成时，自动生成下一周期任务，返回 `nextCycleDate` 和 `nextTaskId`。
- 操作记录写入动态墙："管家 王五 标记了'体内驱虫'任务为已完成"。

---

## 7. 删除任务

**DELETE** `/pets/{petId}/calendar/tasks/{taskId}`

### 响应 data

```json
{
  "taskId": "tsk_005",
  "deletedAt": "2026-07-08T09:41:00+08:00"
}
```

---

## 8. 获取智能提醒

**GET** `/pets/{petId}/calendar/smart-reminder`

智能提醒引擎：根据上次记录自动计算下次提醒日，推荐附近合作医院或线上药房。

### 响应 data

```json
{
  "reminders": [
    {
      "reminderId": "sr_001",
      "type": "deworming",
      "title": "距上次体外驱虫已 32 天",
      "body": "建议今日补做。附近合作医院 宠安诊所 距你 1.2km，线上药房福来恩现货。",
      "daysSinceLast": 32,
      "lastDate": "2026-06-05",
      "actions": [
        { "label": "导航至医院", "type": "navigate", "hospitalId": "hs_001" },
        { "label": "去线上药房", "type": "pharmacy", "url": "url" }
      ]
    }
  ]
}
```

---

## 9. 获取附近合作医院

**GET** `/pets/{petId}/calendar/nearby-hospitals`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| lat | float | 是 | 纬度 |
| lng | float | 是 | 经度 |
| radius | int | 否 | 搜索半径（米），默认 3000 |
| is24h | boolean | 否 | 是否仅24小时急诊 |

### 响应 data

```json
{
  "list": [
    {
      "hospitalId": "hs_001",
      "name": "宠安诊所",
      "address": "杭州市滨江区江南大道123号",
      "distanceM": 1200,
      "is24h": true,
      "phone": "0571-88888888",
      "lat": 30.1234,
      "lng": 120.5678,
      "rating": 4.8
    }
  ]
}
```

---

## 10. 获取训练计划进度

**GET** `/pets/{petId}/calendar/training-plans`

分离焦虑训练等渐进式课程。

### 响应 data

```json
{
  "list": [
    {
      "planId": "tp_001",
      "planName": "14天刷牙训练",
      "description": "渐进式脱敏课程，配合每日打卡任务",
      "currentDay": 7,
      "totalDays": 14,
      "status": "in_progress",
      "completedDays": 7,
      "todayTaskId": "tsk_003",
      "progressPercent": 50
    }
  ]
}
```
