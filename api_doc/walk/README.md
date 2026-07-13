# 遛狗记录模块 API（Walk）

> 对应界面：`walk-live.html`（实时追踪）、`walk-summary.html`（回顾）
> PRD 参考：核心功能 + 多家庭协作

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 开始遛狗 | POST | `/pets/{petId}/walks/start` | P6 | 创建遛狗会话 |
| 2 | 上传实时轨迹点 | POST | `/pets/{petId}/walks/{walkId}/track` | P6 | 实时上报GPS轨迹 |
| 3 | 暂停/继续遛狗 | PUT | `/pets/{petId}/walks/{walkId}/pause` | P6 | 暂停或继续 |
| 4 | 事件打卡 | POST | `/pets/{petId}/walks/{walkId}/events` | P6 | 排便/排尿/嗅闻/社交等 |
| 5 | 结束遛狗 | POST | `/pets/{petId}/walks/{walkId}/end` | P6 | 结束并生成回顾 |
| 6 | 获取实时遛狗状态 | GET | `/pets/{petId}/walks/{walkId}/live` | P5 | 实时数据+轨迹+事件 |
| 7 | 获取遛狗回顾 | GET | `/pets/{petId}/walks/{walkId}/summary` | P5 | 轨迹回放+数据+AI点评 |
| 8 | 获取遛狗历史列表 | GET | `/pets/{petId}/walks` | P5 | 分页历史 |
| 9 | 关联照片到遛狗记录 | POST | `/pets/{petId}/walks/{walkId}/photos` | P10 | 回顾页配照片 |
| 10 | 分享遛狗回顾 | POST | `/pets/{petId}/walks/{walkId}/share` | P5 | 生成分享链接 |

---

## 1. 开始遛狗

**POST** `/pets/{petId}/walks/start`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| startLat | float | 是 | 起点纬度 |
| startLng | float | 是 | 起点经度 |
| startLocation | string | 否 | 起点地名 |
| session | string | 否 | `morning`（晨遛）/ `evening`（晚遛）/ `other` |

### 响应 data

```json
{
  "walkId": "wk_001",
  "petId": "pet_001",
  "startTime": "2026-07-08T09:13:00+08:00",
  "startLocation": "滨江公园",
  "status": "active"
}
```

---

## 2. 上传实时轨迹点

**POST** `/pets/{petId}/walks/{walkId}/track`

客户端每隔 3-5 秒上报一次 GPS 轨迹点。

### 请求参数

```json
{
  "points": [
    {
      "lat": 30.1234,
      "lng": 120.5678,
      "timestamp": "2026-07-08T09:13:05+08:00",
      "speed": 5.9,
      "accuracy": 3.0
    },
    {
      "lat": 30.1235,
      "lng": 120.5679,
      "timestamp": "2026-07-08T09:13:08+08:00",
      "speed": 6.2,
      "accuracy": 2.5
    }
  ]
}
```

### 响应 data

```json
{
  "walkId": "wk_001",
  "receivedPoints": 2,
  "totalPoints": 42,
  "currentStats": {
    "distanceM": 1840,
    "durationSec": 1122,
    "currentSpeedKmh": 5.9,
    "caloriesKcal": 68,
    "avgSpeedKmh": 5.9,
    "peakSpeedKmh": 8.2
  }
}
```

### 说明

- 后端实时计算距离、速度、卡路里消耗并返回最新统计。

---

## 3. 暂停/继续遛狗

**PUT** `/pets/{petId}/walks/{walkId}/pause`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| paused | boolean | 是 | `true` 暂停 / `false` 继续 |

### 响应 data

```json
{
  "walkId": "wk_001",
  "status": "paused",
  "pausedAt": "2026-07-08T09:30:00+08:00"
}
```

---

## 4. 事件打卡

**POST** `/pets/{petId}/walks/{walkId}/events`

一键打卡遛狗过程中的事件。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| eventType | string | 是 | `poop`（排便）/ `pee`（排尿）/ `sniff`（嗅闻）/ `greet`（社交）/ `run`（奔跑）/ `rest`（休息）/ `drink`（喝水）/ `mark`（标记） |
| lat | float | 否 | 事件位置纬度 |
| lng | float | 否 | 事件位置经度 |
| location | string | 否 | 位置描述 |
| note | string | 否 | 备注 |
| durationSec | int | 否 | 持续时间（秒），如社交持续2分钟 |

### 响应 data

```json
{
  "eventId": "ev_001",
  "walkId": "wk_001",
  "eventType": "poop",
  "eventName": "排便",
  "location": "滨江路 · 草坪东侧",
  "timestamp": "2026-07-08T09:32:00+08:00"
}
```

### 说明

- 支持的事件类型与图标：排便、排尿、嗅闻、社交、奔跑、休息、喝水、标记。
- `greet` 类型可附带对方宠物信息（品种、持续时长）。

---

## 5. 结束遛狗

**POST** `/pets/{petId}/walks/{walkId}/end`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| endLat | float | 否 | 终点纬度 |
| endLng | float | 否 | 终点经度 |
| endLocation | string | 否 | 终点地名 |

### 响应 data

```json
{
  "walkId": "wk_001",
  "status": "completed",
  "endTime": "2026-07-08T09:41:00+08:00",
  "summaryId": "ws_001",
  "redirect": "/pets/pet_001/walks/wk_001/summary"
}
```

### 说明

- 结束后自动生成遛狗回顾，AI 异步生成点评。
- 操作记录写入动态墙。
- 若今日遛狗任务存在，自动标记完成。

---

## 6. 获取实时遛狗状态

**GET** `/pets/{petId}/walks/{walkId}/live`

对应 `walk-live.html` 全部数据。

### 响应 data

```json
{
  "walkId": "wk_001",
  "status": "active",
  "startTime": "2026-07-08T09:13:00+08:00",
  "elapsedSec": 1122,
  "location": "滨江公园环线",
  "gpsStrength": "strong",
  "liveStats": {
    "distanceKm": 1.84,
    "currentSpeedKmh": 5.9,
    "caloriesKcal": 68,
    "avgSpeedKmh": 5.9,
    "peakSpeedKmh": 8.2
  },
  "paceChart": [
    { "timestamp": "2026-07-08T09:13:00+08:00", "speedKmh": 4.0 },
    { "timestamp": "2026-07-08T09:15:00+08:00", "speedKmh": 5.5 },
    { "timestamp": "2026-07-08T09:20:00+08:00", "speedKmh": 8.2 }
  ],
  "trackPath": [
    { "lat": 30.1234, "lng": 120.5678 },
    { "lat": 30.1235, "lng": 120.5679 }
  ],
  "startPoint": { "lat": 30.1234, "lng": 120.5678 },
  "currentPoint": { "lat": 30.1250, "lng": 120.5685 },
  "events": [
    {
      "eventId": "ev_001",
      "eventType": "poop",
      "eventName": "排便",
      "timestamp": "2026-07-08T09:32:00+08:00",
      "location": "滨江路 · 草坪东侧"
    },
    {
      "eventId": "ev_002",
      "eventType": "sniff",
      "eventName": "嗅闻标记",
      "timestamp": "2026-07-08T09:28:00+08:00",
      "location": "公园北门 · 第 3 次标记"
    },
    {
      "eventId": "ev_003",
      "eventType": "greet",
      "eventName": "与金毛社交",
      "timestamp": "2026-07-08T09:24:00+08:00",
      "location": "公园中央 · 持续 2 分钟",
      "durationSec": 120
    }
  ]
}
```

---

## 7. 获取遛狗回顾

**GET** `/pets/{petId}/walks/{walkId}/summary`

对应 `walk-summary.html` 全部数据。

### 响应 data

```json
{
  "walkId": "wk_001",
  "date": "2026-07-08",
  "weekday": "周二",
  "session": "evening",
  "sessionLabel": "晚遛",
  "hero": {
    "distanceKm": 2.34,
    "durationText": "28 分 15 秒",
    "durationSec": 1695,
    "caloriesKcal": 92,
    "goalPercent": 78,
    "goalText": "满足今日目标的 78%"
  },
  "trackReplay": {
    "path": [{ "lat": 30.1234, "lng": 120.5678 }],
    "startPoint": { "lat": 30.1234, "lng": 120.5678 },
    "endPoint": { "lat": 30.1250, "lng": 120.5685 }
  },
  "stats": {
    "durationText": "28:15",
    "durationDelta": "↑ 比平均多 4 分",
    "avgSpeedKmh": 5.0,
    "avgSpeedDelta": "↑ 节奏稳定",
    "peakSpeedKmh": 8.2,
    "peakSpeedNote": "公园奔跑段",
    "caloriesKcal": 92,
    "caloriesDelta": "达标"
  },
  "speedSegments": {
    "avgKmh": 5.0,
    "peakKmh": 8.2,
    "bars": [
      { "minute": 1, "speedKmh": 4.0, "isPeak": false },
      { "minute": 9, "speedKmh": 8.2, "isPeak": true }
    ],
    "timeRange": { "start": "09:13", "mid": "09:27", "end": "09:41" }
  },
  "aiComment": {
    "title": "AI 遛狗点评",
    "body": "豆豆今天在 公园中央 有 3 分钟持续奔跑（8.2km/h），运动量充足。注意到 2 次标记行为集中在林荫道，建议下次避开该路段以减少应激。整体节奏理想，下次可延长至 35 分钟 以达每日目标。"
  },
  "eventHeat": {
    "totalCount": 8,
    "items": [
      { "eventType": "mark", "eventName": "标记行为", "count": 3, "percent": 75 },
      { "eventType": "sniff", "eventName": "嗅闻", "count": 2, "percent": 50 },
      { "eventType": "poop", "eventName": "排便", "count": 1, "percent": 25 },
      { "eventType": "greet", "eventName": "社交", "count": 1, "percent": 25 }
    ]
  },
  "linkedPhotos": []
}
```

---

## 8. 获取遛狗历史列表

**GET** `/pets/{petId}/walks`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| startDate | string | 否 | 起始日期 |
| endDate | string | 否 | 结束日期 |
| walkerId | string | 否 | 遛狗人 memberId |
| page | int | 否 | 页码 |
| pageSize | int | 否 | 每页条数 |

### 响应 data

```json
{
  "list": [
    {
      "walkId": "wk_001",
      "date": "2026-07-08",
      "distanceKm": 2.34,
      "durationSec": 1695,
      "caloriesKcal": 92,
      "eventCount": 8,
      "walkerId": "u_abc123",
      "walkerName": "奶爸 阿哲",
      "session": "evening",
      "sessionLabel": "晚遛",
      "thumbnail": "url"
    }
  ],
  "total": 156,
  "page": 1,
  "pageSize": 20,
  "hasMore": true
}
```

---

## 9. 关联照片到遛狗记录

**POST** `/pets/{petId}/walks/{walkId}/photos`

回顾页"配照片"功能。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| photoIds | string[] | 是 | 照片 ID 列表 |

### 响应 data

```json
{
  "walkId": "wk_001",
  "linkedPhotoIds": ["ph_010", "ph_011"],
  "totalLinked": 2
}
```

---

## 10. 分享遛狗回顾

**POST** `/pets/{petId}/walks/{walkId}/share`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| platform | string | 否 | `wechat` / `link`，默认 `link` |

### 响应 data

```json
{
  "shareUrl": "https://puppy.life/walk/wk_001",
  "wxShareTitle": "豆豆今日遛了 2.34km，8 个事件",
  "wxShareDesc": "AI 点评：运动量充足，节奏理想",
  "expireAt": "2026-07-15T09:41:00+08:00"
}
```
