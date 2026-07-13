# 财务管家模块 API（Finance）

> 对应界面：`tool-finance.html`
> PRD 参考：第六章 宠物财务管家

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取月度总览 | GET | `/pets/{petId}/finance/overview` | P5 | 月支出+预算+趋势 |
| 2 | 获取分类占比 | GET | `/pets/{petId}/finance/breakdown` | P5 | 饼图数据 |
| 3 | 获取趋势数据 | GET | `/pets/{petId}/finance/trend` | P5 | 近6月趋势 |
| 4 | 获取交易流水 | GET | `/pets/{petId}/finance/transactions` | P5 | 分页流水 |
| 5 | 新增交易记录 | POST | `/pets/{petId}/finance/transactions` | P6 | 记一笔 |
| 6 | 更新交易记录 | PUT | `/pets/{petId}/finance/transactions/{txId}` | P6 | 修改 |
| 7 | 删除交易记录 | DELETE | `/pets/{petId}/finance/transactions/{txId}` | P6 | 删除 |
| 8 | 设置预算 | PUT | `/pets/{petId}/finance/budget` | P6 | 月度预算 |
| 9 | 获取年度报表 | GET | `/pets/{petId}/finance/yearly-report` | P5 | 年度养宠成本 |

---

## 1. 获取月度总览

**GET** `/pets/{petId}/finance/overview`

对应 `tool-finance.html` 顶部月度英雄卡。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| month | string | 否 | 月份 `2026-07`，默认当月 |

### 响应 data

```json
{
  "month": "2026-07",
  "monthLabel": "7月支出",
  "petName": "豆豆",
  "totalAmount": 124000,
  "totalAmountYuan": 1240.00,
  "currency": "CNY",
  "deltaAmount": -18000,
  "deltaText": "↓ 比上月省 ¥180",
  "budget": 150000,
  "budgetYuan": 1500.00,
  "remaining": 26000,
  "remainingYuan": 260.00
}
```

---

## 2. 获取分类占比

**GET** `/pets/{petId}/finance/breakdown`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| month | string | 否 | 月份，默认当月 |

### 响应 data

```json
{
  "month": "2026-07",
  "totalAmount": 124000,
  "categories": [
    { "categoryId": "medical", "name": "医疗", "amount": 64500, "amountYuan": 645.00, "percent": 52, "color": "danger" },
    { "categoryId": "food", "name": "饮食", "amount": 37200, "amountYuan": 372.00, "percent": 30, "color": "accent" },
    { "categoryId": "toy", "name": "玩具", "amount": 15000, "amountYuan": 150.00, "percent": 12, "color": "mint" },
    { "categoryId": "insurance", "name": "保险", "amount": 7300, "amountYuan": 73.00, "percent": 6, "color": "info" }
  ]
}
```

---

## 3. 获取趋势数据

**GET** `/pets/{petId}/finance/trend`

### 响应 data

```json
{
  "avgAmount": 134800,
  "avgAmountYuan": 1348.00,
  "months": [
    { "month": "2026-02", "amount": 142000, "amountYuan": 1420.00 },
    { "month": "2026-03", "amount": 156000, "amountYuan": 1560.00 },
    { "month": "2026-04", "amount": 178000, "amountYuan": 1780.00 },
    { "month": "2026-05", "amount": 145000, "amountYuan": 1450.00 },
    { "month": "2026-06", "amount": 160000, "amountYuan": 1600.00 },
    { "month": "2026-07", "amount": 124000, "amountYuan": 1240.00, "isCurrent": true }
  ]
}
```

---

## 4. 获取交易流水

**GET** `/pets/{petId}/finance/transactions`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| month | string | 否 | 月份筛选 |
| category | string | 否 | 分类：medical/food/toy/insurance/other |
| page | int | 否 | 页码 |
| pageSize | int | 否 | 每页条数，默认 20 |

### 响应 data

```json
{
  "list": [
    {
      "txId": "tx_001",
      "category": "medical",
      "categoryName": "医疗",
      "title": "福来恩体外驱虫",
      "amount": -12800,
      "amountYuan": -128.00,
      "merchant": "宠安诊所",
      "date": "2026-07-05",
      "note": "",
      "createdBy": "u_abc123",
      "createdByName": "阿哲"
    },
    {
      "txId": "tx_002",
      "category": "food",
      "categoryName": "饮食",
      "title": "皇家成犬粮 12kg",
      "amount": -28900,
      "amountYuan": -289.00,
      "merchant": "京东自营",
      "date": "2026-07-03"
    },
    {
      "txId": "tx_003",
      "category": "toy",
      "categoryName": "玩具",
      "title": "飞盘 + 啸叫球",
      "amount": -6800,
      "amountYuan": -68.00,
      "merchant": "淘宝",
      "date": "2026-06-28"
    },
    {
      "txId": "tx_004",
      "category": "insurance",
      "categoryName": "保险",
      "title": "宠物医疗险 · 月供",
      "amount": -7300,
      "amountYuan": -73.00,
      "merchant": "众安保险",
      "date": "2026-06-20"
    }
  ],
  "total": 8,
  "page": 1,
  "pageSize": 20,
  "hasMore": false
}
```

---

## 5. 新增交易记录

**POST** `/pets/{petId}/finance/transactions`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| title | string | 是 | 记录名称 |
| amount | int | 是 | 金额（分），正为收入，负为支出 |
| category | string | 是 | medical/food/toy/insurance/other |
| merchant | string | 否 | 商户/渠道 |
| date | string | 是 | 日期 |
| note | string | 否 | 备注 |

### 响应 data

返回新增的交易记录，同 [4. 获取交易流水](#4-获取交易流水) 中单个对象。

---

## 6. 更新交易记录

**PUT** `/pets/{petId}/finance/transactions/{txId}`

### 请求参数

可更新 `title`、`amount`、`category`、`merchant`、`date`、`note`。

### 响应 data

返回更新后的交易记录。

---

## 7. 删除交易记录

**DELETE** `/pets/{petId}/finance/transactions/{txId}`

### 响应 data

```json
{
  "txId": "tx_001",
  "deletedAt": "2026-07-08T09:41:00+08:00"
}
```

---

## 8. 设置预算

**PUT** `/pets/{petId}/finance/budget`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| monthlyBudget | int | 是 | 月度预算（分） |

### 响应 data

```json
{
  "petId": "pet_001",
  "monthlyBudget": 150000,
  "monthlyBudgetYuan": 1500.00
}
```

---

## 9. 获取年度报表

**GET** `/pets/{petId}/finance/yearly-report`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| year | int | 否 | 年份，默认当前年 |

### 响应 data

```json
{
  "year": 2026,
  "totalAmount": 935800,
  "totalAmountYuan": 9358.00,
  "monthlyAvg": 133700,
  "monthlyAvgYuan": 1337.00,
  "categoryBreakdown": [
    { "categoryId": "medical", "name": "医疗", "amount": 480000, "percent": 51 },
    { "categoryId": "food", "name": "饮食", "amount": 280000, "percent": 30 },
    { "categoryId": "toy", "name": "玩具", "amount": 100000, "percent": 11 },
    { "categoryId": "insurance", "name": "保险", "amount": 75800, "percent": 8 }
  ],
  "monthlyData": [
    { "month": "2026-01", "amount": 120000 },
    { "month": "2026-02", "amount": 142000 },
    { "month": "2026-03", "amount": 156000 }
  ]
}
```
