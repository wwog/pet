# 健康档案模块 API（Health）

> 对应界面：`tab-health.html`
> PRD 参考：第五章 5.5 健康档案·电子病历本

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取健康概览 | GET | `/pets/{petId}/health/overview` | P3 | 过敏警示+体重曲线+疫苗驱虫倒计时 |
| 2 | 获取体重记录列表 | GET | `/pets/{petId}/health/weights` | P3 | 体重历史+曲线数据 |
| 3 | 录入体重记录 | POST | `/pets/{petId}/health/weights` | P4 | 新增一条体重记录 |
| 4 | 获取过敏与用药禁忌 | GET | `/pets/{petId}/health/allergies` | P3 | 过敏原+禁用药物 |
| 5 | 新增/更新过敏记录 | POST | `/pets/{petId}/health/allergies` | P4 | 添加过敏原或用药禁忌 |
| 6 | 获取疫苗记录列表 | GET | `/pets/{petId}/health/vaccines` | P3 | 疫苗记录+加强针倒计时 |
| 7 | 新增疫苗记录 | POST | `/pets/{petId}/health/vaccines` | P4 | 添加疫苗记录 |
| 8 | 获取驱虫记录列表 | GET | `/pets/{petId}/health/dewormings` | P3 | 体内/体外驱虫记录 |
| 9 | 新增驱虫记录 | POST | `/pets/{petId}/health/dewormings` | P4 | 添加驱虫记录 |
| 10 | 获取病历列表 | GET | `/pets/{petId}/health/records` | P3 | 就诊病历 |
| 11 | 新增病历 | POST | `/pets/{petId}/health/records` | P4 | 添加就诊记录 |
| 12 | 上传基因检测报告并AI解读 | POST | `/pets/{petId}/health/gene-report` | P4 | 上传PDF+AI提取关键信息 |
| 13 | 获取AI健康建议 | GET | `/pets/{petId}/health/ai-advice` | P3 | AI基于体重趋势等的建议 |
| 14 | 删除健康记录 | DELETE | `/pets/{petId}/health/records/{recordId}` | P4 | 删除指定记录 |

---

## 1. 获取健康概览

**GET** `/pets/{petId}/health/overview`

对应 `tab-health.html` 整页数据。

### 响应 data

```json
{
  "petId": "pet_001",
  "allergyBanner": {
    "allergens": ["青霉素类"],
    "forbiddenDrugs": ["布洛芬", "对乙酰氨基酚"],
    "warning": "就医时请出示本档案"
  },
  "weightSummary": {
    "currentKg": 13.0,
    "previousKg": 12.5,
    "deltaKg": 0.5,
    "riskLevel": "warn",
    "riskText": "轻度超重 · +0.5kg",
    "standardRangeMin": 25.0,
    "standardRangeMax": 34.0,
    "curve": [
      { "month": "1月", "weightKg": 11.0 },
      { "month": "2月", "weightKg": 11.5 },
      { "month": "3月", "weightKg": 12.0 },
      { "month": "4月", "weightKg": 12.3 },
      { "month": "5月", "weightKg": 12.5 },
      { "month": "6月", "weightKg": 12.8 },
      { "month": "7月", "weightKg": 13.0 }
    ],
    "aiTip": "AI 评估：近 2 月体重上升偏快，建议每日喂量从 320g 降至 300g，增加 10 分钟遛弯。"
  },
  "vaccineCountdown": {
    "daysUntilDue": 186,
    "vaccineName": "狂犬",
    "dueDate": "2026-12-10"
  },
  "dewormingStatus": {
    "type": "external",
    "status": "overdue",
    "overdueDays": 2,
    "drugName": "福来恩",
    "lastDate": "2026-06-05"
  },
  "records": [
    {
      "recordId": "hr_001",
      "type": "vaccine",
      "name": "狂犬疫苗 · 加强",
      "hospital": "宠安诊所",
      "batchNo": "RV2024A",
      "date": "2026-01-10",
      "status": "completed",
      "packagingPhoto": "url"
    },
    {
      "recordId": "hr_002",
      "type": "deworming",
      "name": "体外驱虫 · 福来恩",
      "lastDate": "2026-06-05",
      "status": "overdue",
      "packagingPhoto": "url"
    },
    {
      "recordId": "hr_003",
      "type": "gene_test",
      "name": "基因检测报告",
      "fileUrl": "url",
      "aiExtracted": true,
      "uploadDate": "2026-05-20"
    }
  ]
}
```

---

## 2. 获取体重记录列表

**GET** `/pets/{petId}/health/weights`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| startDate | string | 否 | 起始日期 |
| endDate | string | 否 | 结束日期 |

### 响应 data

```json
{
  "list": [
    {
      "recordId": "wt_001",
      "weightKg": 13.0,
      "recordedAt": "2026-07-08T09:00:00+08:00",
      "recordedBy": "u_abc123",
      "recordedByName": "阿哲",
      "note": "体检称重"
    }
  ],
  "latestKg": 13.0,
  "trend": "increasing"
}
```

---

## 3. 录入体重记录

**POST** `/pets/{petId}/health/weights`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| weightKg | float | 是 | 体重（kg），保留1位小数 |
| note | string | 否 | 备注 |

### 响应 data

```json
{
  "recordId": "wt_002",
  "weightKg": 13.0,
  "recordedAt": "2026-07-08T09:41:00+08:00",
  "aiEvaluation": {
    "riskLevel": "warn",
    "tip": "近 2 月体重上升偏快，建议每日喂量从 320g 降至 300g。"
  }
}
```

### 说明

- 录入后 AI 自动评估肥胖风险并给出饮食建议。
- 操作记录写入动态墙："奶爸 阿哲 更新了豆豆的体重：12.5kg → 13.0kg"。

---

## 4. 获取过敏与用药禁忌

**GET** `/pets/{petId}/health/allergies`

### 响应 data

```json
{
  "allergens": [
    { "allergenId": "al_01", "name": "青霉素类", "severity": "high", "addedAt": "2026-01-10" }
  ],
  "forbiddenDrugs": [
    { "drugId": "fd_01", "name": "布洛芬", "reason": "犬类禁用，可导致肾损伤" },
    { "drugId": "fd_02", "name": "对乙酰氨基酚", "reason": "犬类禁用，可导致肝损伤" }
  ]
}
```

---

## 5. 新增/更新过敏记录

**POST** `/pets/{petId}/health/allergies`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| type | string | 是 | `allergen`（过敏原）/ `forbidden_drug`（用药禁忌） |
| name | string | 是 | 名称 |
| severity | string | 否 | `high` / `medium` / `low`（仅过敏原） |
| reason | string | 否 | 禁用原因（仅用药禁忌） |

### 响应 data

返回更新后的过敏与用药禁忌列表，同 [4. 获取过敏与用药禁忌](#4-获取过敏与用药禁忌)。

---

## 6. 获取疫苗记录列表

**GET** `/pets/{petId}/health/vaccines`

### 响应 data

```json
{
  "list": [
    {
      "recordId": "vc_001",
      "vaccineName": "狂犬疫苗 · 加强",
      "hospital": "宠安诊所",
      "batchNo": "RV2024A",
      "vaccinatedAt": "2026-01-10",
      "nextBoosterDate": "2026-12-10",
      "daysUntilDue": 186,
      "status": "completed",
      "veterinarian": "李医生",
      "packagingPhoto": "url"
    }
  ],
  "nextDue": {
    "vaccineName": "狂犬",
    "dueDate": "2026-12-10",
    "daysUntilDue": 186
  }
}
```

---

## 7. 新增疫苗记录

**POST** `/pets/{petId}/health/vaccines`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| vaccineName | string | 是 | 疫苗名称 |
| hospital | string | 否 | 接种医院 |
| batchNo | string | 否 | 批号 |
| vaccinatedAt | string | 是 | 接种日期 |
| nextBoosterDate | string | 否 | 下次加强针日期（系统据此自动生成日程提醒） |
| veterinarian | string | 否 | 兽医姓名 |
| packagingPhoto | string | 否 | 药品外包装照片 URL |

### 响应 data

返回新增的疫苗记录。

### 说明

- 若填写了 `nextBoosterDate`，系统自动在日程中创建加强针提醒任务。

---

## 8. 获取驱虫记录列表

**GET** `/pets/{petId}/health/dewormings`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| type | string | 否 | `internal`（体内）/ `external`（体外） |

### 响应 data

```json
{
  "list": [
    {
      "recordId": "dw_001",
      "type": "external",
      "drugName": "福来恩",
      "date": "2026-06-05",
      "packagingPhoto": "url",
      "nextDueDate": "2026-07-05",
      "status": "overdue",
      "overdueDays": 2
    }
  ]
}
```

---

## 9. 新增驱虫记录

**POST** `/pets/{petId}/health/dewormings`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| type | string | 是 | `internal` / `external` |
| drugName | string | 是 | 药品名称 |
| date | string | 是 | 驱虫日期 |
| packagingPhoto | string | 否 | 药品外包装照片 URL |

### 响应 data

返回新增的驱虫记录，并自动生成下次驱虫提醒日程。

---

## 10. 获取病历列表

**GET** `/pets/{petId}/health/records`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| type | string | 否 | `vaccine` / `deworming` / `gene_test` / `diagnosis` |

### 响应 data

```json
{
  "list": [
    {
      "recordId": "hr_003",
      "type": "gene_test",
      "name": "基因检测报告",
      "fileUrl": "url",
      "fileName": "基因检测报告.pdf",
      "fileSize": 3984576,
      "aiExtracted": true,
      "aiSummary": {
        "breedConfirmation": "纯种金毛寻回犬",
        "geneticRisks": ["髋关节发育不良（低风险）"],
        "traits": ["乳糖耐受"]
      },
      "uploadDate": "2026-05-20",
      "uploadedBy": "u_abc123"
    }
  ]
}
```

---

## 11. 新增病历

**POST** `/pets/{petId}/health/records`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| type | string | 是 | `diagnosis`（就诊病历） |
| title | string | 是 | 病历标题 |
| hospital | string | 否 | 就诊医院 |
| veterinarian | string | 否 | 兽医 |
| visitDate | string | 是 | 就诊日期 |
| diagnosis | string | 否 | 诊断结果 |
| prescription | string | 否 | 处方 |
| fileUrl | string | 否 | 附件文件 URL |

### 响应 data

返回新增的病历记录。

---

## 12. 上传基因检测报告并AI解读

**POST** `/pets/{petId}/health/gene-report`

上传 PDF 基因检测报告，AI 自动提取关键信息并解读。

### 请求参数（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| file | file | 是 | PDF 文件 |

### 响应 data

```json
{
  "recordId": "hr_003",
  "fileName": "基因检测报告.pdf",
  "fileSize": 3984576,
  "aiExtracted": true,
  "aiSummary": {
    "breedConfirmation": "纯种金毛寻回犬",
    "geneticRisks": [
      { "name": "髋关节发育不良", "riskLevel": "low", "advice": "控制体重，适度运动" }
    ],
    "traits": ["乳糖耐受", "高运动耐力"],
    "extractedAt": "2026-07-08T09:41:00+08:00"
  },
  "status": "analyzed"
}
```

### 说明

- 上传后 AI 异步处理，处理中返回 `status: "processing"`，完成后通过推送通知用户。
- 处理完成后 `tab-health.html` 中显示"基因检测报告 · PDF 已上传 · AI 已提取关键项"。

---

## 13. 获取AI健康建议

**GET** `/pets/{petId}/health/ai-advice`

AI 基于体重趋势、品种标准、年龄等综合给出健康建议。

### 响应 data

```json
{
  "advice": "近 2 月体重上升偏快，建议每日喂量从 320g 降至 300g，增加 10 分钟遛弯。",
  "riskLevel": "warn",
  "suggestions": [
    { "type": "diet", "description": "每日喂量降至 300g", "priority": "high" },
    { "type": "exercise", "description": "增加 10 分钟遛弯", "priority": "medium" }
  ]
}
```

---

## 14. 删除健康记录

**DELETE** `/pets/{petId}/health/records/{recordId}`

### 响应 data

```json
{
  "recordId": "hr_003",
  "deletedAt": "2026-07-08T09:41:00+08:00"
}
```
