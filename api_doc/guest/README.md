# 访客临时模式 API（Guest）

> 对应界面：`tool-guest.html`
> PRD 参考：第六章 访客临时模式

## 接口清单

| # | 接口 | 方法 | 路径 | 说明 |
|---|------|:----:|------|------|
| 1 | 生成临时访问链接 | POST | `/pets/{petId}/guest/generate-link` | 生成24h临时链接 |
| 2 | 获取临时访问链接状态 | GET | `/pets/{petId}/guest/links/{linkId}` | 查看状态+倒计时 |
| 3 | 更新访客可见范围 | PUT | `/pets/{petId}/guest/links/{linkId}/visibility` | 调整可见内容 |
| 4 | 撤销访客链接 | DELETE | `/pets/{petId}/guest/links/{linkId}` | 撤销访问权限 |
| 5 | 获取访客历史 | GET | `/pets/{petId}/guest/links` | 历史访客链接 |
| 6 | 通过链接访问（公开） | GET | `/guest/access/{shortCode}` | 访客端只读视图 |

权限要求：`P12`（仅首席监护人可生成和管理访客链接）

---

## 1. 生成临时访问链接

**POST** `/pets/{petId}/guest/generate-link`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| scene | string | 是 | 访客场景：`foster`（寄养）/ `feeder`（上门喂养） |
| durationHours | int | 否 | 有效期（小时），默认 24 |
| guestName | string | 否 | 访客名称 |

### 响应 data

```json
{
  "linkId": "gl_001",
  "shortCode": "k7m3x9",
  "url": "https://puppy.life/g/k7m3x9",
  "expireAt": "2026-07-09T09:41:00+08:00",
  "durationHours": 24,
  "visibility": {
    "basicProfile": true,
    "scheduleAndReminders": true,
    "basicHealth": true,
    "album": false
  },
  "scene": "foster",
  "sceneLabel": "寄养人"
}
```

---

## 2. 获取临时访问链接状态

**GET** `/pets/{petId}/guest/links/{linkId}`

### 响应 data

```json
{
  "linkId": "gl_001",
  "shortCode": "k7m3x9",
  "url": "https://puppy.life/g/k7m3x9",
  "status": "active",
  "expireAt": "2026-07-09T09:41:00+08:00",
  "remainingSec": 83408,
  "remainingText": "23:14:08",
  "visitCount": 3,
  "lastVisitAt": "2026-07-08T11:00:00+08:00",
  "visibility": {
    "basicProfile": true,
    "scheduleAndReminders": true,
    "basicHealth": true,
    "album": false
  }
}
```

---

## 3. 更新访客可见范围

**PUT** `/pets/{petId}/guest/links/{linkId}/visibility`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| basicProfile | boolean | 否 | 基础档案（姓名、品种、饮食禁忌） |
| scheduleAndReminders | boolean | 否 | 日程与提醒（喂饭、遛弯安排） |
| basicHealth | boolean | 否 | 基础健康（过敏、用药禁忌、疫苗状态） |
| album | boolean | 否 | 相册（默认关闭，避免隐私泄露） |

### 响应 data

返回更新后的可见范围。

### 说明

- 所有内容只读，不可修改。
- 默认相册关闭，避免隐私泄露。

---

## 4. 撤销访客链接

**DELETE** `/pets/{petId}/guest/links/{linkId}`

立即撤销访问权限。

### 响应 data

```json
{
  "linkId": "gl_001",
  "revokedAt": "2026-07-08T09:41:00+08:00",
  "status": "revoked"
}
```

---

## 5. 获取访客历史

**GET** `/pets/{petId}/guest/links`

### 响应 data

```json
{
  "list": [
    {
      "linkId": "gl_002",
      "guestName": "王阿姨",
      "scene": "feeder",
      "sceneLabel": "上门喂养",
      "duration": "6.15-6.18",
      "visitCount": 3,
      "status": "expired",
      "createdAt": "2026-06-15T10:00:00+08:00"
    },
    {
      "linkId": "gl_003",
      "guestName": "李哥",
      "scene": "foster",
      "sceneLabel": "寄养",
      "duration": "5.20 · 寄养 2 天",
      "status": "expired",
      "createdAt": "2026-05-20T08:00:00+08:00"
    }
  ]
}
```

---

## 6. 通过链接访问（公开）

**GET** `/guest/access/{shortCode}`

访客无需登录即可访问只读视图。

### 响应 data

```json
{
  "shortCode": "k7m3x9",
  "petName": "豆豆",
  "breed": "金毛寻回犬",
  "expireAt": "2026-07-09T09:41:00+08:00",
  "viewData": {
    "basicProfile": {
      "name": "豆豆",
      "breed": "金毛寻回犬",
      "gender": "男孩",
      "ageText": "1岁2个月",
      "dietRestrictions": "禁用巧克力、葡萄、洋葱等"
    },
    "scheduleAndReminders": [
      { "title": "晚餐喂粮 320g", "time": "18:30", "assignee": "奶妈" },
      { "title": "晚遛 25 分钟", "time": "20:00", "assignee": "奶爸" }
    ],
    "basicHealth": {
      "allergens": ["青霉素类"],
      "forbiddenDrugs": ["布洛芬", "对乙酰氨基酚"],
      "vaccineStatus": "狂犬 · 已接种 · 12月加强"
    }
  },
  "note": "仅可查看 · 无写入权限 · 24h 后自动失效"
}
```
