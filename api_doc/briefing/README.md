# AI简报模块 API（Briefing）

> 对应界面：`tab-briefing.html`
> PRD 参考：第五章 5.1 今日·AI简报（首页默认Tab）

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取今日AI简报 | GET | `/pets/{petId}/briefing/today` | P5/P7 | 首页完整简报数据 |
| 2 | 获取遛狗进度 | GET | `/pets/{petId}/briefing/walk-progress` | P5 | 今日遛狗目标进度 |
| 3 | 获取睡眠分析 | GET | `/pets/{petId}/briefing/sleep` | P5 | 昨日睡眠分析 |
| 4 | 获取情绪指数 | GET | `/pets/{petId}/briefing/mood` | P5 | 情绪指数 |
| 5 | 获取今日待办概要 | GET | `/pets/{petId}/briefing/todos` | P7 | 简报页待办卡片 |
| 6 | 获取天气信息 | GET | `/briefing/weather` | 登录 | 当前位置天气 |

---

## 1. 获取今日AI简报

**GET** `/pets/{petId}/briefing/today`

对应 `tab-briefing.html` 全部数据，AI 基于时间、天气、品种年龄生成每日摘要。

### 响应 data

```json
{
  "date": "2026-07-08",
  "weekday": "周二",
  "weather": {
    "temperature": 28,
    "condition": "多云",
    "icon": "cloudy"
  },
  "aiTitle": "今天适合多陪豆豆散步",
  "aiQuote": "傍晚 7 点气温将降至 26°，是豆豆最舒服的散步时间。昨天它学会了'握手'，今天可以尝试巩固一下。",
  "walkProgress": {
    "walkedMinutes": 41,
    "goalMinutes": 60,
    "remainingMinutes": 19,
    "assigneeName": "奶爸 阿哲",
    "assigneeLabel": "负责晚遛",
    "percent": 68
  },
  "sleepStats": {
    "hours": 11.2,
    "deltaText": "↑比平均多1.4h",
    "quality": "good"
  },
  "moodIndex": {
    "score": 92,
    "maxScore": 100,
    "deltaText": "↑开心·活跃",
    "moodLabel": "开心"
  },
  "todos": {
    "total": 3,
    "overdue": 1,
    "items": [
      {
        "taskId": "tsk_001",
        "title": "体外驱虫 · 福来恩",
        "priority": "high",
        "assigneeName": "奶爸 阿哲",
        "assigneeAvatar": "url",
        "scheduledTime": null,
        "isOverdue": true
      },
      {
        "taskId": "tsk_002",
        "title": "晚餐喂粮 320g",
        "priority": "mid",
        "assigneeName": "奶妈 小棠",
        "assigneeAvatar": "url",
        "scheduledTime": "18:30",
        "isOverdue": false
      },
      {
        "taskId": "tsk_003",
        "title": "刷牙训练 5 分钟",
        "priority": "low",
        "assigneeName": "爷爷 老张",
        "assigneeAvatar": "url",
        "scheduledTime": "21:00",
        "isOverdue": false
      }
    ]
  },
  "walkCta": {
    "title": "开始遛狗",
    "description": "记录轨迹·速度·事件打卡"
  }
}
```

### 说明

- AI 简报每日凌晨生成，白天根据遛狗完成情况实时更新 `walkProgress`。
- AI 暖心寄语基于宠物性格标签、最近事件、天气综合生成。

---

## 2. 获取遛狗进度

**GET** `/pets/{petId}/briefing/walk-progress`

### 响应 data

```json
{
  "walkedMinutes": 41,
  "goalMinutes": 60,
  "remainingMinutes": 19,
  "assigneeId": "u_abc123",
  "assigneeName": "奶爸 阿哲",
  "assigneeRole": "奶爸",
  "assigneeLabel": "负责晚遛",
  "percent": 68,
  "todayWalks": [
    {
      "walkId": "wk_001",
      "distanceKm": 2.34,
      "durationMin": 28,
      "session": "morning",
      "walkerName": "奶爸 阿哲"
    }
  ]
}
```

---

## 3. 获取睡眠分析

**GET** `/pets/{petId}/briefing/sleep`

### 响应 data

```json
{
  "date": "2026-07-07",
  "totalHours": 11.2,
  "averageHours": 9.8,
  "deltaHours": 1.4,
  "deltaText": "↑比平均多1.4h",
  "quality": "good",
  "qualityLabel": "良好",
  "deepSleepHours": 6.5,
  "lightSleepHours": 4.7,
  "source": "manual"
}
```

### 说明

- 若接入智能设备，`source` 为 `wearable`；否则为手动记录 `manual`。

---

## 4. 获取情绪指数

**GET** `/pets/{petId}/briefing/mood`

### 响应 data

```json
{
  "date": "2026-07-08",
  "score": 92,
  "maxScore": 100,
  "moodLabel": "开心",
  "deltaText": "↑开心·活跃",
  "trend": "improving",
  "factors": [
    { "factor": "运动充足", "impact": "positive" },
    { "factor": "社交互动", "impact": "positive" }
  ]
}
```

---

## 5. 获取今日待办概要

**GET** `/pets/{petId}/briefing/todos`

### 响应 data

```json
{
  "total": 3,
  "overdue": 1,
  "completed": 1,
  "items": [
    {
      "taskId": "tsk_001",
      "title": "体外驱虫 · 福来恩",
      "priority": "high",
      "assigneeName": "奶爸 阿哲",
      "assigneeAvatar": "url",
      "isOverdue": true
    }
  ]
}
```

---

## 6. 获取天气信息

**GET** `/briefing/weather`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| lat | float | 否 | 纬度（默认家庭所在城市） |
| lng | float | 否 | 经度 |

### 响应 data

```json
{
  "temperature": 28,
  "condition": "多云",
  "conditionCode": "cloudy",
  "icon": "cloudy",
  "humidity": 65,
  "windSpeed": 3.2,
  "uvIndex": 5,
  "suggestion": "傍晚气温适宜，适合遛狗",
  "city": "杭州"
}
```
