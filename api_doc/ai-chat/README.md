# AI对话模块 API（AI Chat）

> 对应界面：`tab-ai-chat.html`
> PRD 参考：第五章 5.4 AI对话·宠物翻译官

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取对话会话列表 | GET | `/pets/{petId}/ai-chat/sessions` | P11 | 历史对话列表 |
| 2 | 创建对话会话 | POST | `/pets/{petId}/ai-chat/sessions` | P11 | 新建对话 |
| 3 | 获取对话消息列表 | GET | `/pets/{petId}/ai-chat/sessions/{sessionId}/messages` | P11 | 对话记录 |
| 4 | 发送消息（流式） | POST | `/pets/{petId}/ai-chat/sessions/{sessionId}/messages` | P11 | 发送文本/视频 |
| 5 | 切换对话模式 | PUT | `/pets/{petId}/ai-chat/sessions/{sessionId}/mode` | P11 | 紧急分诊/行为解读/拟人化 |
| 6 | 获取紧急分诊结果 | GET | `/pets/{petId}/ai-chat/sessions/{sessionId}/triage` | P11 | 红/黄/绿分诊+导航 |
| 7 | 上传行为视频分析 | POST | `/pets/{petId}/ai-chat/analyze-video` | P11 | 上传视频让AI分析行为 |
| 8 | 获取附近24h急诊医院 | GET | `/pets/{petId}/ai-chat/emergency-hospitals` | P11 | 紧急分诊导航 |
| 9 | 联系兽医 | POST | `/pets/{petId}/ai-chat/contact-vet` | P11 | 一键联系绑定兽医 |
| 10 | 删除对话会话 | DELETE | `/pets/{petId}/ai-chat/sessions/{sessionId}` | P11 | 删除历史对话 |

---

## 1. 获取对话会话列表

**GET** `/pets/{petId}/ai-chat/sessions`

### 响应 data

```json
{
  "list": [
    {
      "sessionId": "cs_001",
      "title": "呕吐问题咨询",
      "mode": "triage",
      "lastMessage": "AI 建议：观察至明日，若加重需就医",
      "lastMessageAt": "2026-07-08T09:40:00+08:00",
      "unreadCount": 0,
      "triageLevel": "yellow"
    }
  ]
}
```

---

## 2. 创建对话会话

**POST** `/pets/{petId}/ai-chat/sessions`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| mode | string | 否 | `triage`（紧急分诊）/ `behavior`（行为解读）/ `pet`（拟人化），默认 `triage` |

### 响应 data

```json
{
  "sessionId": "cs_002",
  "mode": "triage",
  "welcomeMessage": {
    "role": "ai",
    "content": "你好，豆豆的家长。描述一下豆豆现在的症状或行为，我会判断紧急程度并给出建议。也可以上传一段视频让我看看。",
    "timestamp": "2026-07-08T09:38:00+08:00"
  }
}
```

### 说明

- AI 上下文包含宠物档案（品种、年龄、性格标签、过敏史等），实现个性化对话。

---

## 3. 获取对话消息列表

**GET** `/pets/{petId}/ai-chat/sessions/{sessionId}/messages`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| page | int | 否 | 页码 |
| pageSize | int | 否 | 每页条数，默认 50 |

### 响应 data

```json
{
  "list": [
    {
      "messageId": "msg_001",
      "role": "ai",
      "roleLabel": "宠物翻译官",
      "content": "你好，豆豆的家长。描述一下豆豆现在的症状或行为...",
      "timestamp": "2026-07-08T09:38:00+08:00",
      "attachments": []
    },
    {
      "messageId": "msg_002",
      "role": "user",
      "roleLabel": "奶爸 阿哲",
      "content": "豆豆从半小时前开始反复干呕，吐了两次黄水，精神有点蔫，鼻子发干。",
      "timestamp": "2026-07-08T09:40:00+08:00",
      "attachments": []
    },
    {
      "messageId": "msg_003",
      "role": "ai",
      "roleLabel": "宠物翻译官",
      "content": "综合症状分析，建议尽快就医排查。",
      "timestamp": "2026-07-08T09:40:00+08:00",
      "triageCard": {
        "level": "red",
        "levelLabel": "红色 · 紧急",
        "title": "疑似胃扩张或异物阻塞",
        "description": "反复干呕 + 黄水 + 精神萎靡是高危信号。就医前：禁食禁水，避免剧烈活动，记录呕吐次数与内容。",
        "actions": [
          { "label": "导航至24h急诊", "type": "navigate", "hospitalId": "hs_001" },
          { "label": "联系兽医", "type": "call_vet", "vetId": "vt_001" }
        ]
      },
      "disclaimer": "AI 建议，非诊断"
    }
  ],
  "hasMore": false
}
```

---

## 4. 发送消息（流式）

**POST** `/pets/{petId}/ai-chat/sessions/{sessionId}/messages`

使用 Server-Sent Events (SSE) 流式返回，首字延时 < 2 秒。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| content | string | 是 | 用户消息文本 |
| attachments | array | 否 | 附件列表（视频/图片） |

### 响应（SSE 流）

```
event: start
data: {"messageId":"msg_004"}

event: chunk
data: {"delta":"综合"}

event: chunk
data: {"delta":"症状分析"}

event: chunk
data: {"delta":"，建议尽快就医排查。"}

event: triage
data: {"triageCard":{"level":"red","levelLabel":"红色 · 紧急","title":"疑似胃扩张或异物阻塞",...}}

event: done
data: {"messageId":"msg_004","timestamp":"2026-07-08T09:40:00+08:00"}
```

### 说明

- 流式返回 AI 回复内容，前端逐字渲染。
- 紧急分诊模式下，AI 回复完成后附带 `triageCard`。
- AI 对话记录仅用于当次咨询，默认不用于模型训练。

---

## 5. 切换对话模式

**PUT** `/pets/{petId}/ai-chat/sessions/{sessionId}/mode`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| mode | string | 是 | `triage` / `behavior` / `pet` |

### 响应 data

```json
{
  "sessionId": "cs_002",
  "mode": "pet",
  "modeLabel": "🐾 拟人化",
  "hint": "已切换至模拟宠物口吻模式，豆豆会用第一人称回应你。"
}
```

### 模式说明

| 模式 | 说明 |
|------|------|
| `triage` | 紧急分诊：描述症状后AI给出紧急等级（绿/黄/红）及就医前预处理建议 |
| `behavior` | 行为解读：上传视频或描述行为，AI分析背后原因（如分离焦虑、早期疼痛表现） |
| `pet` | 拟人化聊天：用狗狗第一人称回应用户，增加趣味性 |

---

## 6. 获取紧急分诊结果

**GET** `/pets/{petId}/ai-chat/sessions/{sessionId}/triage`

### 响应 data

```json
{
  "level": "red",
  "levelLabel": "红色 · 紧急",
  "title": "疑似胃扩张或异物阻塞",
  "description": "反复干呕 + 黄水 + 精神萎靡是高危信号。就医前：禁食禁水，避免剧烈活动，记录呕吐次数与内容。",
  "preActions": [
    "禁食禁水",
    "避免剧烈活动",
    "记录呕吐次数与内容"
  ],
  "actions": [
    { "label": "导航至24h急诊", "type": "navigate", "hospitalId": "hs_001" },
    { "label": "联系兽医", "type": "call_vet", "vetId": "vt_001" }
  ]
}
```

### 分诊等级

| 等级 | 颜色 | 说明 |
|------|------|------|
| `red` | 红色·紧急 | 需立即就医 |
| `yellow` | 黄色·注意 | 建议观察，若加重需就医 |
| `green` | 绿色·安全 | 正常现象，无需担心 |

---

## 7. 上传行为视频分析

**POST** `/pets/{petId}/ai-chat/analyze-video`

上传视频让 AI 分析宠物行为。

### 请求参数（multipart/form-data）

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| file | file | 是 | 视频文件 |
| description | string | 否 | 用户对行为的描述 |

### 响应 data

```json
{
  "analysisId": "an_001",
  "status": "processing",
  "estimatedSeconds": 15
}
```

处理完成后通过 SSE 或推送返回结果：

```json
{
  "analysisId": "an_001",
  "status": "completed",
  "behavior": "反复追逐尾巴",
  "interpretation": "这可能表示肛门腺问题或强迫性行为。建议检查肛门腺是否需要清理，若频繁出现建议咨询兽医。",
  "severity": "yellow",
  "suggestion": "观察频率，若每日超过3次建议就医检查"
}
```

---

## 8. 获取附近24h急诊医院

**GET** `/pets/{petId}/ai-chat/emergency-hospitals`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| lat | float | 是 | 纬度 |
| lng | float | 是 | 经度 |
| radius | int | 否 | 搜索半径（米），默认 5000 |

### 响应 data

```json
{
  "list": [
    {
      "hospitalId": "hs_001",
      "name": "宠安24h急诊",
      "address": "杭州市滨江区江南大道123号",
      "distanceM": 1200,
      "is24h": true,
      "phone": "0571-88888888",
      "lat": 30.1234,
      "lng": 120.5678,
      "navigateUrl": "https://maps.apple.com/?daddr=30.1234,120.5678"
    }
  ]
}
```

---

## 9. 联系兽医

**POST** `/pets/{petId}/ai-chat/contact-vet`

一键联系家庭绑定的兽医。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| vetId | string | 否 | 兽医 ID，默认家庭绑定兽医 |
| sessionId | string | 否 | 对话会话 ID，附带最近症状 |

### 响应 data

```json
{
  "vetId": "vt_001",
  "vetName": "李医生",
  "vetPhone": "0571-88888888",
  "vetHospital": "宠安诊所",
  "contextShared": true,
  "shareUrl": "https://puppy.life/vet/share/cs_002"
}
```

### 说明

- 联系兽医时自动分享宠物健康档案 + 当前对话的症状描述。
- 兽医可通过分享链接查看过敏禁忌、疫苗记录等。

---

## 10. 删除对话会话

**DELETE** `/pets/{petId}/ai-chat/sessions/{sessionId}`

### 响应 data

```json
{
  "sessionId": "cs_002",
  "deletedAt": "2026-07-08T09:41:00+08:00"
}
```

### 说明

- AI 对话记录默认不用于模型训练，删除后不可恢复。
- 若需用于模型优化，需单独弹窗授权。
