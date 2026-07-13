# 文件上传模块 API（Upload）

> 通用文件上传接口，供各业务模块（相册、健康档案、保险箱等）统一使用。

## 接口清单

| # | 接口 | 方法 | 路径 | 说明 |
|---|------|:----:|------|------|
| 1 | 上传单个文件 | POST | `/upload/file` | 通用上传 |
| 2 | 批量上传文件 | POST | `/upload/files` | 多文件上传 |
| 3 | 上传前校验 | POST | `/upload/validate` | 校验文件大小/格式 |
| 4 | 获取预签名URL | POST | `/upload/presigned-url` | 客户端直传OSS |

---

## 1. 上传单个文件

**POST** `/upload/file`

### 请求参数（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| file | file | 是 | 文件 |
| module | string | 是 | 业务模块：`album`（相册）/ `health`（健康）/ `vault`（保险箱）/ `avatar`（头像） |
| petId | string | 否 | 关联宠物 ID |

### 文件限制

| 模块 | 支持格式 | 大小限制 | 说明 |
|------|----------|:--------:|------|
| album | JPG/PNG/HEIC/MP4/MOV | 50MB（图）/ 200MB（视频） | 原画质 |
| health | JPG/PNG/PDF | 20MB | 药品包装/基因报告 |
| vault | PDF/JPG/PNG | 50MB | 加密存储 |
| avatar | JPG/PNG | 5MB | 头像 |

### 响应 data

```json
{
  "fileId": "fl_001",
  "url": "https://cdn.puppy-life.com/album/pet_001/fl_001.jpg",
  "thumbnailUrl": "https://cdn.puppy-life.com/album/pet_001/fl_001_t.jpg",
  "fileName": "IMG_1234.jpg",
  "fileSize": 2456789,
  "mimeType": "image/jpeg",
  "width": 4032,
  "height": 3024,
  "module": "album",
  "uploadedAt": "2026-07-08T09:41:00+08:00"
}
```

---

## 2. 批量上传文件

**POST** `/upload/files`

### 请求参数（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| files | file[] | 是 | 多文件（最多 20 个） |
| module | string | 是 | 业务模块 |
| petId | string | 否 | 关联宠物 ID |

### 响应 data

```json
{
  "uploaded": [
    { "fileId": "fl_001", "url": "url", "fileName": "IMG_1234.jpg", "fileSize": 2456789 },
    { "fileId": "fl_002", "url": "url", "fileName": "IMG_1235.jpg", "fileSize": 3124567 }
  ],
  "failed": [],
  "totalCount": 2
}
```

---

## 3. 上传前校验

**POST** `/upload/validate`

前端在上传前校验文件是否合规，避免上传后才发现格式/大小不满足。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| fileName | string | 是 | 文件名 |
| fileSize | int | 是 | 文件大小（字节） |
| module | string | 是 | 业务模块 |

### 响应 data

```json
{
  "valid": true,
  "reason": null
}
```

### 校验不通过

```json
{
  "valid": false,
  "reason": "相册视频大小不能超过 200MB",
  "maxSizeBytes": 209715200
}
```

---

## 4. 获取预签名URL

**POST** `/upload/presigned-url`

生成 OSS 预签名 URL，客户端直传至对象存储（绕过服务器）。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| fileName | string | 是 | 原始文件名 |
| mimeType | string | 是 | MIME 类型 |
| module | string | 是 | 业务模块 |
| petId | string | 否 | 关联宠物 ID |

### 响应 data

```json
{
  "uploadUrl": "https://oss.puppy-life.com/upload?Expires=...&Signature=...",
  "fileId": "fl_001",
  "expireIn": 300,
  "callbackUrl": "https://api.puppy-life.com/v1/upload/callback"
}
```

### 说明

- 客户端使用 `uploadUrl` 直接 PUT 文件到 OSS，不上传经过服务器。
- OSS 上传完成后回调 `callbackUrl` 通知后端创建对应的业务记录（如照片记录、健康记录等）。
