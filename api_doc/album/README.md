# 云相册模块 API（Album）

> 对应界面：`tab-album.html`
> PRD 参考：第五章 5.2 云相册·时光机

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取相册概览 | GET | `/pets/{petId}/album/overview` | P9 | 总数+AI标签统计+精选 |
| 2 | 获取照片列表 | GET | `/pets/{petId}/album/photos` | P9 | 分页+AI标签筛选 |
| 3 | 获取单张照片详情 | GET | `/pets/{petId}/album/photos/{photoId}` | P9 | 详情+AI标签+上传者 |
| 4 | 上传照片/视频 | POST | `/pets/{petId}/album/photos` | P10 | 批量上传+AI自动打标 |
| 5 | 删除照片 | DELETE | `/pets/{petId}/album/photos/{photoId}` | P10 | 删除单张 |
| 6 | 编辑照片信息 | PUT | `/pets/{petId}/album/photos/{photoId}` | P10 | 修改标签/备注 |
| 7 | AI语义搜索照片 | POST | `/pets/{petId}/album/search` | P9 | 自然语言搜索 |
| 8 | 获取时光回忆录列表 | GET | `/pets/{petId}/album/memories` | P9 | 月度高光短视频 |
| 9 | 生成时光回忆录 | POST | `/pets/{petId}/album/memories` | P10 | 手动触发生成 |
| 10 | 获取回忆录详情 | GET | `/pets/{petId}/album/memories/{memoryId}` | P9 | 视频URL+瞬间列表 |
| 11 | 获取相册时间线 | GET | `/pets/{petId}/album/timeline` | P9 | 上传/事件时间线 |
| 12 | 获取AI去重统计 | GET | `/pets/{petId}/album/dedup-stats` | P9 | AI已剔除的模糊/重复数 |

---

## 1. 获取相册概览

**GET** `/pets/{petId}/album/overview`

### 响应 data

```json
{
  "totalCount": 1284,
  "dedupStats": {
    "removedCount": 23,
    "reason": "AI 已剔除 23 张模糊/重复"
  },
  "aiTags": [
    { "tag": "飞盘", "count": 45 },
    { "tag": "睡觉", "count": 128 },
    { "tag": "开心", "count": 203 },
    { "tag": "委屈", "count": 12 },
    { "tag": "呕吐", "count": 3 }
  ],
  "weeklyPicks": [
    { "photoId": "ph_001", "thumbnail": "url", "aiTag": "接飞盘", "isVideo": false },
    { "photoId": "ph_002", "thumbnail": "url", "aiTag": "开心", "isVideo": false },
    { "photoId": "ph_003", "thumbnail": "url", "aiTag": "睡觉", "isVideo": false }
  ]
}
```

---

## 2. 获取照片列表

**GET** `/pets/{petId}/album/photos`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| tag | string | 否 | AI标签筛选（飞盘/睡觉/开心/委屈/呕吐等） |
| startDate | string | 否 | 起始日期 |
| endDate | string | 否 | 结束日期 |
| uploadedBy | string | 否 | 上传者 memberId |
| page | int | 否 | 页码 |
| pageSize | int | 否 | 每页条数，默认 30 |

### 响应 data

```json
{
  "list": [
    {
      "photoId": "ph_001",
      "url": "https://cdn.puppy-life.com/photos/ph_001.jpg",
      "thumbnail": "https://cdn.puppy-life.com/photos/ph_001_t.jpg",
      "isVideo": false,
      "aiTags": ["接飞盘", "开心"],
      "uploadDate": "2026-07-06",
      "uploadedBy": "u_grandpa",
      "uploadedByName": "爷爷 老张",
      "location": "滨江公园"
    }
  ],
  "total": 1284,
  "page": 1,
  "pageSize": 30,
  "hasMore": true
}
```

---

## 3. 获取单张照片详情

**GET** `/pets/{petId}/album/photos/{photoId}`

### 响应 data

```json
{
  "photoId": "ph_001",
  "url": "https://cdn.puppy-life.com/photos/ph_001.jpg",
  "thumbnail": "url",
  "isVideo": false,
  "width": 4032,
  "height": 3024,
  "fileSize": 2456789,
  "aiTags": ["接飞盘", "开心", "公园"],
  "aiDescription": "金毛在公园草地上接飞盘，表情开心",
  "uploadDate": "2026-07-06T17:00:00+08:00",
  "uploadedBy": "u_grandpa",
  "uploadedByName": "爷爷 老张",
  "uploadedByRole": "爷爷",
  "location": "滨江公园",
  "note": "豆豆第一次成功接住飞盘！"
}
```

---

## 4. 上传照片/视频

**POST** `/pets/{petId}/album/photos`

支持批量上传，上传后 AI 自动打标（动作识别、表情识别）并剔除模糊/重复。

### 请求参数（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| files | file[] | 是 | 照片/视频文件（支持多文件） |
| location | string | 否 | 拍摄地点 |

### 响应 data

```json
{
  "uploaded": [
    {
      "photoId": "ph_002",
      "url": "url",
      "aiTags": ["散步", "夜间"],
      "aiStatus": "tagged"
    }
  ],
  "deduped": [
    { "fileName": "IMG_0232.jpg", "reason": "与已有照片重复" }
  ],
  "totalCount": 1287,
  "aiProcessing": true
}
```

### 说明

- 上传后 AI 异步处理打标，处理中返回 `aiStatus: "processing"`。
- AI 自动识别动作（飞盘/睡觉/呕吐）与表情（开心/委屈）。
- 模糊/重复照片自动剔除并返回 `deduped` 列表。
- 操作记录写入动态墙："爷爷 老张 上传了3张新照片"。

---

## 5. 删除照片

**DELETE** `/pets/{petId}/album/photos/{photoId}`

### 响应 data

```json
{
  "photoId": "ph_001",
  "deletedAt": "2026-07-08T09:41:00+08:00"
}
```

---

## 6. 编辑照片信息

**PUT** `/pets/{petId}/album/photos/{photoId}`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| note | string | 否 | 备注 |
| location | string | 否 | 拍摄地点 |
| aiTags | string[] | 否 | 手动修正 AI 标签 |

### 响应 data

返回更新后的照片详情，同 [3. 获取单张照片详情](#3-获取单张照片详情)。

---

## 7. AI语义搜索照片

**POST** `/pets/{petId}/album/search`

自然语言语义搜索，如"去年冬天在公园接飞盘"。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| query | string | 是 | 自然语言查询 |
| page | int | 否 | 页码 |
| pageSize | int | 否 | 每页条数 |

### 响应 data

```json
{
  "list": [
    {
      "photoId": "ph_045",
      "url": "url",
      "thumbnail": "url",
      "aiTags": ["接飞盘", "开心"],
      "uploadDate": "2025-12-15",
      "location": "滨江公园",
      "relevanceScore": 0.95
    }
  ],
  "total": 12,
  "query": "去年冬天在公园接飞盘",
  "page": 1,
  "pageSize": 20,
  "hasMore": false
}
```

### 说明

- AI 基于照片标签、描述、时间、地点进行语义匹配。
- 搜索建议词：开心的表情、睡觉、接飞盘、和爷爷、呕吐时刻。

---

## 8. 获取时光回忆录列表

**GET** `/pets/{petId}/album/memories`

每月自动生成 15 秒高光短视频。

### 响应 data

```json
{
  "list": [
    {
      "memoryId": "mm_006",
      "month": "2026-06",
      "title": "豆豆的高光15秒",
      "momentCount": 12,
      "music": "暖阳",
      "duration": 15,
      "videoUrl": "url",
      "thumbnail": "url",
      "createdAt": "2026-07-01T00:00:00+08:00",
      "autoGenerated": true
    }
  ]
}
```

---

## 9. 生成时光回忆录

**POST** `/pets/{petId}/album/memories`

手动触发当月回忆录生成，支持自定义配乐。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| month | string | 否 | 月份 `2026-07`，默认当月 |
| music | string | 否 | 配乐选择 |

### 响应 data

```json
{
  "memoryId": "mm_007",
  "status": "processing",
  "estimatedSeconds": 30
}
```

### 说明

- 视频生成异步处理，完成后通过推送通知用户。

---

## 10. 获取回忆录详情

**GET** `/pets/{petId}/album/memories/{memoryId}`

### 响应 data

```json
{
  "memoryId": "mm_006",
  "month": "2026-06",
  "title": "豆豆的高光15秒",
  "momentCount": 12,
  "music": "暖阳",
  "duration": 15,
  "videoUrl": "url",
  "thumbnail": "url",
  "moments": [
    { "photoId": "ph_010", "thumbnail": "url", "aiTag": "接飞盘", "date": "2026-06-03" },
    { "photoId": "ph_015", "thumbnail": "url", "aiTag": "睡觉", "date": "2026-06-08" }
  ],
  "createdAt": "2026-07-01T00:00:00+08:00"
}
```

---

## 11. 获取相册时间线

**GET** `/pets/{petId}/album/timeline`

### 响应 data

```json
{
  "list": [
    {
      "date": "2026-07-06",
      "events": [
        {
          "type": "upload",
          "description": "爷爷上传3张照片",
          "actorId": "u_grandpa",
          "actorName": "爷爷 老张",
          "actorRole": "爷爷",
          "photoCount": 3
        }
      ]
    },
    {
      "date": "2026-07-04",
      "events": [
        {
          "type": "health_sync",
          "description": "奶爸记录呕吐时刻（已同步健康档案·AI建议观察）",
          "actorId": "u_abc123",
          "actorName": "奶爸 阿哲",
          "actorRole": "奶爸"
        }
      ]
    },
    {
      "date": "2026-07-01",
      "events": [
        {
          "type": "memory_generated",
          "description": "6月回忆录已生成",
          "memoryId": "mm_006"
        }
      ]
    }
  ]
}
```

---

## 12. 获取AI去重统计

**GET** `/pets/{petId}/album/dedup-stats`

### 响应 data

```json
{
  "totalUploaded": 1307,
  "removedCount": 23,
  "blurryCount": 8,
  "duplicateCount": 15,
  "keptCount": 1284,
  "message": "AI 已剔除 23 张模糊/重复"
}
```
