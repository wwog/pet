# 文件保险箱模块 API（Vault）

> 对应界面：`tool-vault.html`
> PRD 参考：第六章 重要文件云保险箱

## 接口清单

| # | 接口 | 方法 | 路径 | 说明 |
|---|------|:----:|------|------|
| 1 | 获取保险箱概览 | GET | `/pets/{petId}/vault/overview` | 分类统计+加密状态 |
| 2 | 获取文件列表 | GET | `/pets/{petId}/vault/documents` | 分页+分类筛选 |
| 3 | 上传文件 | POST | `/pets/{petId}/vault/documents` | 加密上传 |
| 4 | 下载文件 | GET | `/pets/{petId}/vault/documents/{docId}/download` | 解密下载 |
| 5 | 预览文件 | GET | `/pets/{petId}/vault/documents/{docId}/preview` | 在线预览 |
| 6 | 删除文件 | DELETE | `/pets/{petId}/vault/documents/{docId}` | 删除 |
| 7 | 分享给兽医 | POST | `/pets/{petId}/vault/documents/{docId}/share-vet` | 生成加密分享链接 |
| 8 | 扫描文件 | POST | `/pets/{petId}/vault/scan` | 调用摄像头扫描上传 |

权限要求：`P12`（仅首席监护人可访问，涉及芯片编号、血统证书等敏感文件）

---

## 1. 获取保险箱概览

**GET** `/pets/{petId}/vault/overview`

### 响应 data

```json
{
  "totalCount": 7,
  "encrypted": true,
  "encryptionType": "E2EE",
  "banner": "端到端加密 · E2EE · 一键分享给兽医无需解密下载",
  "categories": [
    { "categoryId": "chip", "name": "芯片编号", "count": 1, "icon": "chip" },
    { "categoryId": "pedigree", "name": "血统证书", "count": 2, "icon": "pedigree" },
    { "categoryId": "insurance", "name": "保险单", "count": 1, "icon": "insurance" },
    { "categoryId": "medical", "name": "病历", "count": 3, "icon": "medical" }
  ]
}
```

---

## 2. 获取文件列表

**GET** `/pets/{petId}/vault/documents`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| category | string | 否 | `chip`/`pedigree`/`insurance`/`medical` |

### 响应 data

```json
{
  "list": [
    {
      "docId": "vd_001",
      "fileName": "CKU 血统证书.pdf",
      "fileType": "pdf",
      "fileSize": 1258291,
      "fileSizeText": "1.2 MB",
      "category": "pedigree",
      "uploadedAt": "2024-03-01",
      "encrypted": true,
      "aiExtracted": false,
      "note": "CKU 认证"
    },
    {
      "docId": "vd_002",
      "fileName": "基因检测报告.pdf",
      "fileType": "pdf",
      "fileSize": 3984576,
      "fileSizeText": "3.8 MB",
      "category": "medical",
      "uploadedAt": "2024-05-01",
      "encrypted": true,
      "aiExtracted": true
    },
    {
      "docId": "vd_003",
      "fileName": "芯片登记照.jpg",
      "fileType": "image",
      "fileSize": 838860,
      "fileSizeText": "0.8 MB",
      "category": "chip",
      "uploadedAt": "2024-03-01",
      "encrypted": true,
      "chipNumber": "982100015012345"
    },
    {
      "docId": "vd_004",
      "fileName": "众安宠物医疗险.pdf",
      "fileType": "pdf",
      "fileSize": 943718,
      "fileSizeText": "0.9 MB",
      "category": "insurance",
      "uploadedAt": "2024-06-01",
      "encrypted": true,
      "expireDate": "2025-06-01"
    }
  ],
  "total": 7
}
```

---

## 3. 上传文件

**POST** `/pets/{petId}/vault/documents`

支持 PDF、图片（JPG/PNG）等格式。

### 请求参数（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| file | file | 是 | 文件 |
| category | string | 是 | chip/pedigree/insurance/medical |
| chipNumber | string | 否 | 芯片编号（category=chip 时） |
| note | string | 否 | 备注 |
| expireDate | string | 否 | 过期日期（保单等） |

### 响应 data

```json
{
  "docId": "vd_005",
  "fileName": "新血统证书.pdf",
  "encryption": "e2ee",
  "status": "encrypted",
  "uploadedAt": "2026-07-08T09:41:00+08:00"
}
```

---

## 4. 下载文件

**GET** `/pets/{petId}/vault/documents/{docId}/download`

E2EE 解密后返回文件流。

### 响应

- Content-Type: `application/octet-stream` 或 `application/pdf`
- 自动解密后传输

---

## 5. 预览文件

**GET** `/pets/{petId}/vault/documents/{docId}/preview`

在线预览（不解密下载到本地）。

### 响应 data

```json
{
  "docId": "vd_001",
  "fileName": "CKU 血统证书.pdf",
  "previewUrl": "https://cdn.puppy-life.com/preview/vd_001?token=xxx",
  "expireIn": 300
}
```

---

## 6. 删除文件

**DELETE** `/pets/{petId}/vault/documents/{docId}`

### 响应 data

```json
{
  "docId": "vd_005",
  "deletedAt": "2026-07-08T09:41:00+08:00"
}
```

---

## 7. 分享给兽医

**POST** `/pets/{petId}/vault/documents/{docId}/share-vet`

生成加密分享链接，兽医无需解密下载即可查看。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| vetId | string | 否 | 兽医 ID |
| vetPhone | string | 否 | 兽医手机号 |
| expireHours | int | 否 | 有效期（小时），默认 24 |

### 响应 data

```json
{
  "shareId": "vs_001",
  "shareUrl": "https://puppy.life/vault/share/vs_001",
  "expireAt": "2026-07-09T09:41:00+08:00",
  "shortCode": "K7M3X9"
}
```

---

## 8. 扫描文件

**POST** `/pets/{petId}/vault/scan`

调用手机摄像头扫描纸质文件直接上传到保险箱。

### 请求参数（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| image | file | 是 | 扫描图片 |
| category | string | 是 | 分类 |
| note | string | 否 | 备注 |

### 响应 data

同 [3. 上传文件](#3-上传文件)。
