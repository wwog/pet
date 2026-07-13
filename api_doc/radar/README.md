# 社群雷达模块 API（Radar）

> 对应界面：`tool-radar.html`
> PRD 参考：第六章 附近遛狗社群雷达

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取附近狗狗列表 | GET | `/radar/nearby` | 登录 | LBS附近同品种/全部 |
| 2 | 更新我的位置 | PUT | `/radar/my-location` | 登录 | 上报位置供他人发现 |
| 3 | 获取狗狗详情 | GET | `/radar/pets/{petPublicId}` | 登录 | 公开信息 |
| 4 | 发起约玩/打招呼 | POST | `/radar/interactions` | 登录 | 发起约玩请求 |
| 5 | 获取互动请求列表 | GET | `/radar/interactions` | 登录 | 收到的/发出的 |
| 6 | 处理互动请求 | PUT | `/radar/interactions/{interactionId}` | 登录 | 接受/拒绝 |
| 7 | 设置雷达可见性 | PUT | `/radar/visibility` | 登录 | 是否对他人可见 |
| 8 | 更新雷达搜索范围 | PUT | `/radar/range` | 登录 | 搜索半径设置 |

---

## 1. 获取附近狗狗列表

**GET** `/radar/nearby`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| lat | float | 是 | 纬度 |
| lng | float | 是 | 经度 |
| range | int | 否 | 搜索半径（米），默认 3000 |
| filter | string | 否 | `all` / `same_breed`（同品种）/ `online` / `available`（可约玩） |

### 响应 data

```json
{
  "range": 3000,
  "location": "滨江公园",
  "totalCount": 5,
  "sameBreedCount": 3,
  "list": [
    {
      "petPublicId": "pp_001",
      "petName": "麦克斯",
      "breed": "金毛寻回犬",
      "isSameBreed": true,
      "age": 3,
      "ownerNickname": "阿哲的邻居",
      "distanceM": 400,
      "isOnline": true,
      "lastActiveAt": "2026-07-08T09:35:00+08:00",
      "tags": [],
      "avatar": "url",
      "canPlay": true
    },
    {
      "petPublicId": "pp_002",
      "petName": "奶昔",
      "breed": "金毛寻回犬",
      "isSameBreed": true,
      "age": 2,
      "distanceM": 800,
      "isOnline": false,
      "lastActiveAt": "2026-07-08T09:36:00+08:00",
      "tags": []
    },
    {
      "petPublicId": "pp_003",
      "petName": "豆包",
      "breed": "柯基",
      "isSameBreed": false,
      "distanceM": 1200,
      "lastActiveAt": "2026-07-08T09:10:00+08:00",
      "tags": ["爱接飞盘"]
    },
    {
      "petPublicId": "pp_004",
      "petName": "大福",
      "breed": "拉布拉多",
      "isSameBreed": false,
      "distanceM": 1500,
      "isOnline": true,
      "tags": ["可互助遛狗"]
    }
  ]
}
```

---

## 2. 更新我的位置

**PUT** `/radar/my-location`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| lat | float | 是 | 纬度 |
| lng | float | 是 | 经度 |
| location | string | 否 | 地名 |

### 响应 data

```json
{
  "updated": true,
  "nearbyCount": 5
}
```

---

## 3. 获取狗狗详情

**GET** `/radar/pets/{petPublicId}`

### 响应 data

```json
{
  "petPublicId": "pp_001",
  "petName": "麦克斯",
  "breed": "金毛寻回犬",
  "age": 3,
  "gender": "male",
  "avatar": "url",
  "ownerNickname": "阿哲的邻居",
  "distanceM": 400,
  "isOnline": true,
  "tags": [],
  "sharedInterests": ["爱接飞盘"]
}
```

### 说明

- 仅返回公开信息，不泄露联系方式。
- 联系方式需双方同意后解锁（通过互动请求）。

---

## 4. 发起约玩/打招呼

**POST** `/radar/interactions`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| targetPetPublicId | string | 是 | 目标狗狗公开 ID |
| type | string | 是 | `play_invite`（约玩）/ `greeting`（打招呼）/ `walk_help`（互助遛狗） |
| message | string | 否 | 消息 |

### 响应 data

```json
{
  "interactionId": "in_001",
  "status": "pending",
  "createdAt": "2026-07-08T09:41:00+08:00"
}
```

---

## 5. 获取互动请求列表

**GET** `/radar/interactions`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| direction | string | 否 | `incoming`（收到的）/ `outgoing`（发出的）/ `all`，默认 `all` |
| status | string | 否 | `pending` / `accepted` / `rejected` |

### 响应 data

```json
{
  "list": [
    {
      "interactionId": "in_001",
      "type": "play_invite",
      "typeLabel": "约玩",
      "fromPetPublicId": "pp_005",
      "fromPetName": "大福",
      "fromPetAvatar": "url",
      "toPetPublicId": "pp_me",
      "message": "下午去公园一起玩？",
      "status": "pending",
      "createdAt": "2026-07-08T09:30:00+08:00",
      "direction": "incoming"
    }
  ]
}
```

---

## 6. 处理互动请求

**PUT** `/radar/interactions/{interactionId}`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| action | string | 是 | `accept` / `reject` |

### 响应 data

```json
{
  "interactionId": "in_001",
  "status": "accepted",
  "contactUnlocked": true,
  "contactInfo": {
    "ownerNickname": "大福的主人",
    "wechat": "dofu_owner",
    "phone": "138****8888"
  }
}
```

### 说明

- 双方同意后解锁联系方式。

---

## 7. 设置雷达可见性

**PUT** `/radar/visibility`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| visible | boolean | 是 | 是否对他人可见 |

### 响应 data

```json
{
  "visible": true
}
```

---

## 8. 更新雷达搜索范围

**PUT** `/radar/range`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| range | int | 是 | 搜索半径（米），范围 500-10000 |

### 响应 data

```json
{
  "range": 3000
}
```
