# 宠物档案模块 API（Pet）

> 对应界面：`onboarding-welcome.html`、`onboarding-step1.html`、`onboarding-step2.html`、`onboarding-step3.html`、`pet-profile.html`（档案Tab）、`profile-me.html`（宠物卡片）
> PRD 参考：第四章 新用户引导与宠物档案创建

## 接口清单

| # | 接口 | 方法 | 路径 | 所需权限 | 说明 |
|---|------|:----:|------|:--------:|------|
| 1 | 获取宠物列表 | GET | `/pets` | P1 | 当前家庭的所有宠物 |
| 2 | 获取宠物详情 | GET | `/pets/{petId}` | P1 | 完整基础档案 |
| 3 | 创建宠物（三步引导汇总） | POST | `/pets` | P2 | 提交完整宠物信息 |
| 4 | 更新宠物基础信息 | PUT | `/pets/{petId}` | P2 | 修改姓名、性别、生日等 |
| 5 | 更新宠物品种与外貌 | PUT | `/pets/{petId}/appearance` | P2 | 品种、毛色、花纹、绝育状态 |
| 6 | 更新宠物性格标签 | PUT | `/pets/{petId}/personality` | P2 | 性格标签+自定义标签 |
| 7 | 获取品种库 | GET | `/pets/breeds` | 登录 | AKC+FCI 全犬种数据库 |
| 8 | 获取性格标签库 | GET | `/pets/personality-tags` | 登录 | 20+预设标签，分4类 |
| 9 | 删除宠物 | DELETE | `/pets/{petId}` | P12 | 仅首席监护人可操作 |
| 10 | 获取陪伴天数统计 | GET | `/pets/{petId}/stats` | P1 | 陪伴天数等统计数据 |

---

## 1. 获取宠物列表

**GET** `/pets`

### 响应 data

```json
{
  "list": [
    {
      "petId": "pet_001",
      "name": "豆豆",
      "breed": "金毛寻回犬",
      "breedId": "brd_golden",
      "gender": "male",
      "birthDate": "2023-05-01",
      "birthApproximate": false,
      "neuterStatus": "neutered",
      "avatar": "https://cdn.puppy-life.com/pet/pet_001.jpg",
      "ageText": "1岁2个月",
      "companionDays": 287,
      "weightKg": 13.0
    }
  ]
}
```

---

## 2. 获取宠物详情

**GET** `/pets/{petId}`

### 响应 data

```json
{
  "petId": "pet_001",
  "familyId": "fam_xyz789",
  "name": "豆豆",
  "emoji": "🐾",
  "gender": "male",
  "birthDate": "2023-05-01",
  "birthApproximate": false,
  "birthYear": 2023,
  "birthMonth": 5,
  "breedId": "brd_golden",
  "breedName": "金毛寻回犬",
  "breedSize": "大型",
  "breedCoatType": "长毛",
  "standardWeightMin": 25.0,
  "standardWeightMax": 34.0,
  "lifeSpanMin": 10,
  "lifeSpanMax": 12,
  "exerciseNeeds": "high",
  "coatColor": "gold",
  "coatPattern": "纯色",
  "neuterStatus": "neutered",
  "avatar": "url",
  "ageText": "1岁2个月",
  "companionDays": 287,
  "createdAt": "2023-05-01T10:00:00+08:00",
  "personalityTags": [
    { "tagId": "pt_social_01", "name": "社牛", "category": "social" },
    { "tagId": "pt_social_03", "name": "粘人", "category": "social" },
    { "tagId": "pt_behavior_01", "name": "爱接飞盘", "category": "behavior" },
    { "tagId": "pt_emotion_01", "name": "爱被摸肚", "category": "emotion" },
    { "tagId": "pt_custom_01", "name": "爱晒太阳", "category": "custom" }
  ],
  "customTags": ["爱追球", "贪吃"]
}
```

---

## 3. 创建宠物

**POST** `/pets`

三步引导完成后一次性提交完整宠物信息。

### 请求参数

```json
{
  "name": "豆豆",
  "emoji": "🐾",
  "gender": "male",
  "birthYear": 2023,
  "birthMonth": 5,
  "birthApproximate": false,
  "breedId": "brd_golden",
  "coatColor": "gold",
  "coatPattern": "纯色",
  "neuterStatus": "neutered",
  "personalityTagIds": ["pt_social_01", "pt_social_03", "pt_behavior_01", "pt_emotion_01"],
  "customTags": ["爱追球", "贪吃"]
}
```

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| name | string | 是 | 爱称，1-10 字符 |
| emoji | string | 否 | 推荐emoji |
| gender | string | 是 | `male` / `female` |
| birthYear | int | 是 | 出生年份 |
| birthMonth | int | 否 | 出生月份（大概模式可不填） |
| birthApproximate | boolean | 否 | 是否为大概日期（领养未知情况） |
| breedId | string | 是 | 品种 ID |
| coatColor | string | 是 | 毛色：cream/tan/brown/black/white/gray/gold/red/choco/merle/fawn/pearl |
| coatPattern | string | 否 | 花纹：纯色/虎斑/陨石/双色/三色/花斑 |
| neuterStatus | string | 是 | `neutered`（已绝育）/ `intact`（未绝育）/ `planned`（计划中） |
| personalityTagIds | string[] | 否 | 性格标签 ID 列表 |
| customTags | string[] | 否 | 自定义标签 |

### 响应 data

```json
{
  "petId": "pet_001",
  "name": "豆豆",
  "createdAt": "2026-07-08T09:41:00+08:00"
}
```

### 错误码

| code | 说明 |
|------|------|
| 3003 | 宠物数量已达上限（5只/家庭） |

---

## 4. 更新宠物基础信息

**PUT** `/pets/{petId}`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| name | string | 否 | 爱称 |
| gender | string | 否 | 性别 |
| birthYear | int | 否 | 出生年份 |
| birthMonth | int | 否 | 出生月份 |
| birthApproximate | boolean | 否 | 大概日期标记 |
| neuterStatus | string | 否 | 绝育状态 |

### 响应 data

返回更新后的宠物详情，同 [2. 获取宠物详情](#2-获取宠物详情)。

---

## 5. 更新宠物品种与外貌

**PUT** `/pets/{petId}/appearance`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| breedId | string | 否 | 品种 ID |
| coatColor | string | 否 | 毛色 |
| coatPattern | string | 否 | 花纹 |

### 响应 data

返回更新后的宠物详情。

---

## 6. 更新宠物性格标签

**PUT** `/pets/{petId}/personality`

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| personalityTagIds | string[] | 是 | 性格标签 ID 列表（全量覆盖） |
| customTags | string[] | 否 | 自定义标签列表（全量覆盖） |

### 响应 data

```json
{
  "petId": "pet_001",
  "personalityTags": [ ... ],
  "customTags": [ ... ]
}
```

### 说明

- 性格标签作为 AI 对话的个性化基础上下文。

---

## 7. 获取品种库

**GET** `/pets/breeds`

AKC + FCI 全犬种数据库，支持中文拼音首字母检索。

### 请求参数

| 字段 | 类型 | 必填 | 说明 |
|------|------|:----:|------|
| keyword | string | 否 | 搜索关键词，支持中文或拼音首字母（如 "J" 显示金毛、吉娃娃） |
| size | string | 否 | 体型筛选：`small` / `medium` / `large` |
| page | int | 否 | 页码，默认 1 |
| pageSize | int | 否 | 每页条数，默认 20 |

### 响应 data

```json
{
  "list": [
    {
      "breedId": "brd_golden",
      "name": "金毛寻回犬",
      "pinyin": "jinmao",
      "initial": "J",
      "sizeCategory": "大型",
      "coatType": "长毛",
      "standardWeightMin": 25.0,
      "standardWeightMax": 34.0,
      "lifeSpanMin": 10,
      "lifeSpanMax": 12,
      "exerciseNeeds": "high",
      "icon": "🐕",
      "origin": "英国"
    },
    {
      "breedId": "brd_chihuahua",
      "name": "吉娃娃",
      "pinyin": "jiwawa",
      "initial": "J",
      "sizeCategory": "小型",
      "coatType": "短毛",
      "standardWeightMin": 1.5,
      "standardWeightMax": 3.0,
      "lifeSpanMin": 14,
      "lifeSpanMax": 16,
      "exerciseNeeds": "low",
      "icon": "🐶"
    }
  ],
  "total": 197,
  "page": 1,
  "pageSize": 20,
  "hasMore": true
}
```

---

## 8. 获取性格标签库

**GET** `/pets/personality-tags`

20+ 预设标签，分 4 类。

### 响应 data

```json
{
  "categories": [
    {
      "categoryId": "social",
      "categoryName": "社交性格",
      "tags": [
        { "tagId": "pt_social_01", "name": "社牛" },
        { "tagId": "pt_social_02", "name": "社恐" },
        { "tagId": "pt_social_03", "name": "粘人" },
        { "tagId": "pt_social_04", "name": "独立" },
        { "tagId": "pt_social_05", "name": "对人友好" },
        { "tagId": "pt_social_06", "name": "怕生" },
        { "tagId": "pt_social_07", "name": "护主" }
      ]
    },
    {
      "categoryId": "behavior",
      "categoryName": "行为习惯",
      "tags": [
        { "tagId": "pt_behavior_01", "name": "护食" },
        { "tagId": "pt_behavior_02", "name": "爱游泳" },
        { "tagId": "pt_behavior_03", "name": "爱叫" },
        { "tagId": "pt_behavior_04", "name": "拆家" },
        { "tagId": "pt_behavior_05", "name": "扑人" },
        { "tagId": "pt_behavior_06", "name": "挑食" },
        { "tagId": "pt_behavior_07", "name": "爱接飞盘" },
        { "tagId": "pt_behavior_08", "name": "追尾巴" }
      ]
    },
    {
      "categoryId": "emotion",
      "categoryName": "情绪偏好",
      "tags": [
        { "tagId": "pt_emotion_01", "name": "爱被摸肚" },
        { "tagId": "pt_emotion_02", "name": "怕雷声" },
        { "tagId": "pt_emotion_03", "name": "怕鞭炮" },
        { "tagId": "pt_emotion_04", "name": "怕吹风机" },
        { "tagId": "pt_emotion_05", "name": "晕车" },
        { "tagId": "pt_emotion_06", "name": "怕独处" }
      ]
    },
    {
      "categoryId": "custom",
      "categoryName": "更多·自定义",
      "tags": [
        { "tagId": "pt_custom_01", "name": "爱晒太阳" },
        { "tagId": "pt_custom_02", "name": "爱追球" },
        { "tagId": "pt_custom_03", "name": "贪吃" }
      ],
      "allowCustom": true,
      "customPlaceholder": "自定义角色，如 御用铲屎官"
    }
  ]
}
```

---

## 9. 删除宠物

**DELETE** `/pets/{petId}`

### 说明

- 仅首席监护人（P12）可操作。
- 删除后宠物所有关联数据（健康记录、相册、日程等）归档保留 30 天后清除。

### 响应 data

```json
{
  "petId": "pet_001",
  "deletedAt": "2026-07-08T09:41:00+08:00",
  "archiveRetentionDays": 30
}
```

---

## 10. 获取陪伴天数统计

**GET** `/pets/{petId}/stats`

### 响应 data

```json
{
  "petId": "pet_001",
  "companionDays": 287,
  "totalWalks": 156,
  "totalWalkDistanceKm": 342.5,
  "totalPhotos": 1284,
  "totalHealthRecords": 47,
  "totalDiaryEntries": 89
}
```
