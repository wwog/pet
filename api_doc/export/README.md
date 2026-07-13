# 数据导出模块 API（Export）

> 对应界面：`tool-export.html`
> PRD 参考：第八章 数据安全与隐私（数据导出）

## 接口清单

| # | 接口 | 方法 | 路径 | 说明 |
|---|------|:----:|------|------|
| 1 | 获取导出选项 | GET | `/pets/{petId}/export/options` | 可导出范围+预览统计 |
| 2 | 提交导出任务 | POST | `/pets/{petId}/export` | 异步导出 |
| 3 | 获取导出任务状态 | GET | `/pets/{petId}/export/tasks/{taskId}` | 进度查询 |
| 4 | 下载导出文件 | GET | `/pets/{petId}/export/tasks/{taskId}/download` | 下载文件 |
| 5 | 获取导出历史 | GET | `/pets/{petId}/export/tasks` | 历史列表 |
| 6 | 删除导出文件 | DELETE | `/pets/{petId}/export/tasks/{taskId}` | 删除服务器端文件 |

权限要求：`P12`（仅首席监护人可导出数据）

---

## 1. 获取导出选项

**GET** `/pets/{petId}/export/options`

### 响应 data

```json
{
  "scopes": [
    { "scopeId": "profile", "name": "宠物档案", "description": "基础信息 + 性格", "photoCount": 0, "recordCount": 1 },
    { "scopeId": "health", "name": "健康记录", "description": "疫苗 + 病历", "photoCount": 0, "recordCount": 47 },
    { "scopeId": "album", "name": "相册", "description": "原图备份", "photoCount": 1284, "recordCount": 0 },
    { "scopeId": "finance", "name": "财务记录", "description": "开销流水", "photoCount": 0, "recordCount": 89 }
  ],
  "formats": [
    { "formatId": "pdf", "name": "PDF 报告", "description": "排版精美 · 适合打印", "maxPhotos": 500 },
    { "formatId": "csv", "name": "CSV 数据", "description": "表格 · 可导入Excel", "maxPhotos": 0 }
  ],
  "privacyNotes": [
    "导出文件本地生成，不上传服务器",
    "相册原图采用端到端加密包，需密码解压",
    "AI 对话记录默认不导出，需单独授权",
    "注销账号后导出功能保留 30 天"
  ]
}
```

---

## 2. 提交导出任务

**POST** `/pets/{petId}/export`

异步导出任务。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| scopes | string[] | 是 | 导出范围：profile/health/album/finance |
| format | string | 是 | `pdf` / `csv` |
| includeAiConversations | boolean | 否 | 是否包含AI对话记录，默认 false |
| encryptPhotos | boolean | 否 | 相册是否加密打包，默认 true |

### 响应 data

```json
{
  "taskId": "et_001",
  "status": "processing",
  "estimatedSize": "2.4 GB",
  "estimatedSeconds": 120,
  "createdAt": "2026-07-08T09:41:00+08:00",
  "preview": {
    "photoCount": 1284,
    "healthRecordCount": 47,
    "financeRecordCount": 89,
    "companionDays": 287,
    "aiConversationCount": 12
  }
}
```

---

## 3. 获取导出任务状态

**GET** `/pets/{petId}/export/tasks/{taskId}`

轮询进度。

### 响应 data

```json
{
  "taskId": "et_001",
  "status": "processing",
  "progressPercent": 47,
  "estimatedSeconds": 60,
  "createdAt": "2026-07-08T09:41:00+08:00"
}
```

### 状态说明

| status | 说明 |
|--------|------|
| `processing` | 导出进行中 |
| `completed` | 导出完成，可下载 |
| `failed` | 导出失败 |
| `expired` | 下载链接已过期 |

---

## 4. 下载导出文件

**GET** `/pets/{petId}/export/tasks/{taskId}/download`

### 响应

- Content-Type: `application/zip` 或 `application/pdf` 或 `text/csv`
- Content-Disposition: `attachment; filename="豆豆全档案.pdf"`

### 说明

- PDF 格式时相册不包含原图，仅含缩略图索引。
- 若需下载原图，使用 `format=csv` 导出清单 + 单独下载。

---

## 5. 获取导出历史

**GET** `/pets/{petId}/export/tasks`

### 响应 data

```json
{
  "list": [
    {
      "taskId": "et_002",
      "scopeSummary": "豆豆全档案.pdf",
      "fileSizeText": "2.1 GB",
      "format": "pdf",
      "status": "completed",
      "createdAt": "2026-06-30T10:00:00+08:00",
      "downloadExpireAt": "2026-07-30T10:00:00+08:00",
      "canDownload": true
    },
    {
      "taskId": "et_003",
      "fileSizeText": "38 KB",
      "scopeSummary": "健康记录.csv",
      "format": "csv",
      "status": "completed",
      "createdAt": "2026-05-15T08:00:00+08:00",
      "downloadExpireAt": "2026-06-15T08:00:00+08:00",
      "canDownload": false
    }
  ]
}
```

---

## 6. 删除导出文件

**DELETE** `/pets/{petId}/export/tasks/{taskId}`

删除服务器端已生成的导出文件。

### 响应 data

```json
{
  "taskId": "et_002",
  "deletedAt": "2026-07-08T09:41:00+08:00"
}
```
