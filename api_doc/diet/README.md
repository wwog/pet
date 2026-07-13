# 饮食计算器模块 API（Diet）

> 对应界面：`tool-diet.html`
> PRD 参考：第六章 智能饮食计算器

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取今日饮食概览 | GET | `/pets/{petId}/diet/today` | P5 | 热量环+已喂列表 |
| 2 | 搜索狗粮品牌 | GET | `/diet/food-brands/search` | P5 | 品牌+热量数据 |
| 3 | 换算热量 | POST | `/pets/{petId}/diet/calculate` | P5 | 克重→热量换算 |
| 4 | 添加喂食记录 | POST | `/pets/{petId}/diet/records` | P6 | 记一顿 |
| 5 | 获取喂食记录列表 | GET | `/pets/{petId}/diet/records` | P5 | 今日已喂 |
| 6 | 删除喂食记录 | DELETE | `/pets/{petId}/diet/records/{recordId}` | P6 | 删除一条 |
| 7 | 获取每日建议热量 | GET | `/pets/{petId}/diet/recommendation` | P5 | AI基于体重+运动量 |

---

## 1. 获取今日饮食概览

**GET** `/pets/{petId}/diet/today`

对应 `tool-diet.html` 全部数据。

### 响应 data

```json
{
  "date": "2026-07-08",
  "calorieSummary": {
    "consumedKcal": 842,
    "recommendedKcal": 1100,
    "remainingKcal": 258,
    "percent": 77,
    "status": "ok",
    "statusText": "未超标 · 还可吃 258 kcal"
  },
  "recommendation": {
    "dailyKcal": 1100,
    "weightKg": 13.0,
    "breed": "金毛寻回犬",
    "todayWalkKm": 2.34,
    "note": "13kg 金毛 · 已结合今日遛弯 2.34km 消耗"
  },
  "todayRecords": [
    {
      "recordId": "fd_001",
      "mealName": "早餐 · 皇家粮 170g",
      "foodBrand": "皇家金毛专用粮",
      "grams": 170,
      "caloriesKcal": 612,
      "fedAt": "2026-07-08T07:15:00+08:00",
      "fedBy": "u_def456",
      fedByName: "奶妈 小棠"
    },
    {
      "recordId": "fd_002",
      "mealName": "训练零食 · 鸡肉干 3 根",
      "foodBrand": "鸡肉干",
      "grams": 15,
      "caloriesKcal": 230,
      "fedAt": "2026-07-08T14:30:00+08:00",
      "fedBy": "u_abc123",
      "fedByName": "奶爸 阿哲"
    }
  ],
  "totalMeals": 2,
  "totalCalories": 842
}
```

---

## 2. 搜索狗粮品牌

**GET** `/diet/food-brands/search`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| keyword | string | 是 | 搜索关键词 |

### 响应 data

```json
{
  "list": [
    {
      "brandId": "fb_001",
      "brandName": "皇家金毛专用粮",
      "brandLabel": "皇家 · 成犬",
      "kcalPer100g": 360,
      "lifeStage": "adult",
      "breedSpecific": "金毛"
    }
  ]
}
```

---

## 3. 换算热量

**POST** `/pets/{petId}/diet/calculate`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| brandId | string | 否 | 狗粮品牌 ID |
| kcalPer100g | float | 否 | 每100g热量（无品牌时手动输入） |
| grams | int | 是 | 喂食克重 |

### 响应 data

```json
{
  "grams": 150,
  "caloriesKcal": 540,
  "percentOfDaily": 49,
  "remainingKcal": 258,
  "afterThisKcal": 1382,
  "isOverLimit": true,
  "overLimitKcal": 282,
  "suggestion": "150g 皇家金毛粮 = 540kcal。豆豆今日已摄入 842kcal，加这顿后达 1382kcal，将超标 282kcal。建议减量至 90g。",
  "suggestedGrams": 90
}
```

---

## 4. 添加喂食记录

**POST** `/pets/{petId}/diet/records`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| mealName | string | 是 | 餐次名称，如"晚餐 150g" |
| foodBrand | string | 否 | 狗粮品牌 |
| brandId | string | 否 | 品牌ID |
| grams | int | 是 | 克重 |
| caloriesKcal | int | 否 | 热量（可由换算接口计算） |
| fedAt | string | 否 | 喂食时间，默认当前 |

### 响应 data

```json
{
  "recordId": "fd_003",
  "mealName": "晚餐 150g",
  "grams": 150,
  "caloriesKcal": 540,
  "fedAt": "2026-07-08T18:00:00+08:00",
  "todayTotalKcal": 1382,
  "isOverLimit": true
}
```

---

## 5. 获取喂食记录列表

**GET** `/pets/{petId}/diet/records`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| date | string | 否 | 日期，默认今日 |

### 响应 data

```json
{
  "list": [
    {
      "recordId": "fd_001",
      "mealName": "早餐 · 皇家粮 170g",
      "grams": 170,
      "caloriesKcal": 612,
      "fedAt": "2026-07-08T07:15:00+08:00",
      "fedBy": "u_def456",
      "fedByName": "奶妈 小棠"
    }
  ],
  "totalMeals": 2,
  "totalCalories": 842
}
```

---

## 6. 删除喂食记录

**DELETE** `/pets/{petId}/diet/records/{recordId}`

### 响应 data

```json
{
  "recordId": "fd_003",
  "deletedAt": "2026-07-08T09:41:00+08:00",
  "todayTotalKcal": 842
}
```

---

## 7. 获取每日建议热量

**GET** `/pets/{petId}/diet/recommendation`

AI 基于体重、品种、运动量综合计算每日建议热量。

### 响应 data

```json
{
  "dailyKcal": 1100,
  "weightKg": 13.0,
  "breed": "金毛寻回犬",
  "standardWeightMin": 25.0,
  "standardWeightMax": 34.0,
  "todayWalkKm": 2.34,
  "todayWalkCalories": 92,
  "factors": [
    { "factor": "体重 13kg", "baseKcal": 950 },
    { "factor": "今日运动 2.34km", "extraKcal": 150 }
  ],
  "note": "13kg 金毛 · 已结合今日遛弯 2.34km 消耗"
}
```
