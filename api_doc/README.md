# 小狗人生（Puppy Life OS）API 文档

> **版本**：v1.0.0  
> **基础URL**：`https://api.puppy-life.com/v1`  
> **生成日期**：2026-07-08  
> **设计依据**：《小狗人生产品需求文档（PRD）》+ 21 个高保真原型页面  

---

## 总览

| # | 模块目录 | 接口数 | 说明 | 对应界面 |
|---|----------|:------:|------|----------|
| 1 | [auth](./auth/) | 8 | 登录注册、Token管理 | 登录/注册页（PRD第二章） |
| 2 | [family](./family/) | 18 | 家庭管理、RBAC权限、邀请 | `family-permissions.html`、`tool-invite.html` |
| 3 | [pet](./pet/) | 10 | 宠物档案、品种库、性格标签 | `onboarding-step1~3.html`、`pet-profile.html` |
| 4 | [health](./health/) | 14 | 体重曲线、疫苗驱虫、过敏、病历 | `tab-health.html` |
| 5 | [album](./album/) | 12 | 云相册、AI标签、时光回忆录 | `tab-album.html` |
| 6 | [calendar](./calendar/) | 10 | 智能日程、任务、训练计划 | `tab-calendar.html` |
| 7 | [walk](./walk/) | 10 | 实时遛狗追踪、回顾、AI点评 | `walk-live.html`、`walk-summary.html` |
| 8 | [ai-chat](./ai-chat/) | 10 | AI宠物翻译官、分诊、行为分析 | `tab-ai-chat.html` |
| 9 | [briefing](./briefing/) | 6 | AI每日简报、天气、遛狗进度 | `tab-briefing.html`（首页） |
| 10 | [finance](./finance/) | 9 | 宠物财务管家、预算、年度报表 | `tool-finance.html` |
| 11 | [diet](./diet/) | 7 | 饮食计算器、热量换算 | `tool-diet.html` |
| 12 | [radar](./radar/) | 8 | LBS社群雷达、约玩互动 | `tool-radar.html` |
| 13 | [vault](./vault/) | 8 | 加密文件保险箱 | `tool-vault.html` |
| 14 | [guest](./guest/) | 6 | 访客临时模式、24h只读链接 | `tool-guest.html` |
| 15 | [activity-log](./activity-log/) | 5 | 动态墙、操作留痕、责任可视化 | `pet-profile.html`（动态墙Tab） |
| 16 | [export](./export/) | 6 | 数据导出（PDF/CSV） | `tool-export.html` |
| 17 | [notification](./notification/) | 7 | 站内通知、推送 | 系统通知+推送 |
| 18 | [upload](./upload/) | 4 | 统一文件上传 | 各模块上传入口 |
| 19 | [common](./common/) | — | 通用约定、错误码、分页格式 | 全站通用 |

**接口总计：约 158 个**

---

## 项目接口总数统计

```
  模块           接口数
  ─────────────────────
  auth              8
  family           18
  pet              10
  health           14
  album            12
  calendar         10
  walk             10
  ai-chat          10
  briefing          6
  finance           9
  diet              7
  radar             8
  vault             8
  guest             6
  activity-log      5
  export            6
  notification      7
  upload            4
  ─────────────────────
  TOTAL           158
```

---

## 核心业务流程

### 1. 注册 → 添加宠物
```
Auth(短信/微信) → 设置昵称 → 自动创建家庭 → Onboarding三步 → 进入首页
```

### 2. 邀请家人
```
生成邀请码 → 微信分享 → 对方登录输入 → 自选角色 → 首席审核 → 正式加入
```

### 3. RBAC权限
```
首席监护人 → 进入成员权限页 → 逐项开关P1-P11 → 实时生效 → 通知被修改人
```

### 4. AI宠物翻译官
```
选择模式(分诊/解读/拟人) → 发送消息/上传视频 → SSE流式返回 → AI分诊卡/行为分析
```

### 5. 遛狗追踪
```
开始遛狗 → 实时GPS上报 → 事件打卡 → 结束遛狗 → 生成回顾 → AI点评
```

### 6. 云相册
```
上传照片 → AI自动打标/去重 → 语义搜索 → 月度回忆录生成
```

---

## 外部服务集成

| 服务 | 用途 | 相关模块 |
|------|------|----------|
| **AI/LLM** | 每日简报生成、宠物翻译官对话、行为分析、AI分诊、遛狗点评、图片语义搜索、回忆录生成、基因报告解读 | briefing, ai-chat, walk, album, health |
| **微信OAuth** | 微信一键登录 | auth |
| **短信服务** | 验证码登录 | auth |
| **天气API** | 日报天气、遛狗建议 | briefing |
| **LBS/地图** | 遛狗轨迹、附近狗狗雷达、急诊医院导航 | walk, radar, ai-chat |
| **极光/个推** | 推送通知 | notification |
| **OSS/CDN** | 相册原画质备份、文件存储 | album, upload, vault |
| **视频生成** | 月度高光回忆录自动生成 | album |
| **PDF解析（AI）** | 基因检测报告解读 | health |

---

## 权限点与模块对应

| 权限点 | 名称 | 涉及模块 |
|--------|------|----------|
| P1 | 查看宠物基础档案 | pet, diet（档案部分） |
| P2 | 编辑宠物基础档案 | pet |
| P3 | 查看健康记录 | health, briefing（健康部分） |
| P4 | 编辑/新增健康记录 | health |
| P5 | 查看每日事件/日记 | walk, briefing, activity-log, album（时间线） |
| P6 | 编辑/新增每日事件 | walk, calendar（任务标记）, diet（喂食记录） |
| P7 | 查看日程列表 | calendar, briefing（待办） |
| P8 | 管理日程 | calendar |
| P9 | 查看相册 | album |
| P10 | 管理相册 | album, walk（关联照片） |
| P11 | 使用AI对话 | ai-chat |
| P12 | 管理家庭成员（专属） | family, export, vault, guest |

---

## 目录结构

```
api_doc/
├── README.md                   # 本文件（总览索引）
├── common/
│   └── README.md               # 通用约定（基础URL、响应格式、错误码、权限校验）
├── auth/
│   └── README.md               # 8 个接口：登录注册、Token管理
├── family/
│   └── README.md               # 18 个接口：家庭管理、RBAC权限、邀请审核
├── pet/
│   └── README.md               # 10 个接口：宠物档案、品种库、性格标签
├── health/
│   └── README.md               # 14 个接口：体重、疫苗、驱虫、过敏、病历
├── album/
│   └── README.md               # 12 个接口：云相册、AI标签、时光回忆录
├── calendar/
│   └── README.md               # 10 个接口：日程、任务、智能提醒
├── walk/
│   └── README.md               # 10 个接口：实时遛狗、轨迹、回顾
├── ai-chat/
│   └── README.md               # 10 个接口：AI对话、分诊、行为分析
├── briefing/
│   └── README.md               # 6 个接口：AI简报、天气、遛狗进度
├── finance/
│   └── README.md               # 9 个接口：财务管家、交易流水
├── diet/
│   └── README.md               # 7 个接口：饮食计算器、热量换算
├── radar/
│   └── README.md               # 8 个接口：LBS社群雷达、约玩
├── vault/
│   └── README.md               # 8 个接口：加密文件保险箱
├── guest/
│   └── README.md               # 6 个接口：访客临时模式
├── activity-log/
│   └── README.md               # 5 个接口：动态墙、操作留痕
├── export/
│   └── README.md               # 6 个接口：数据导出
├── notification/
│   └── README.md               # 7 个接口：站内通知、推送
└── upload/
    └── README.md               # 4 个接口：统一文件上传
```
