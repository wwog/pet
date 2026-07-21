# 小狗人生（Puppy Life OS）

AI 原生的宠物生活管理平台。Rust workspace + DDD 分层（`crates/domain`、`crates/db`、`crates/api`）。

- 产品需求：`degisn/产品原型.md`
- API 设计：`api_doc/`
- 架构与编码规范：`CLAUDE.md`

---

## 数据库迁移

数据库 schema 由 [toasty-cli](https://crates.io/crates/toasty-cli) 迁移机制管理。迁移文件存放在 `toasty/` 目录，由 `crates/api/src/bin/migrate.rs` 封装的 `migrate` bin 驱动。`Database::connect` **不会自动建表**——建表与演进完全依赖迁移文件，server 启动时仅执行 `migration apply`。

所有命令在**项目根目录**执行。

### 前置

```bash
cp .env.example .env   # 首次配置，DATABASE_URL 默认 sqlite:data.db
```

### 初始化（首次启动）

server 启动时会自动 `migration apply`，但前提是 `toasty/` 里已有迁移文件。新环境首次需先生成并应用初始迁移：

```bash
# 1. 生成初始迁移（基于 db crate 注册的全部模型，产出 toasty/migrations/0000_init.sql 等）
cargo run -p api --bin migrate -- migration generate --name init

# 2. 应用迁移（建表 + 写入 __toasty_migrations 历史）
cargo run -p api --bin migrate -- migration apply

# 3. 启动 server（会自动 seed 超级管理员与品种库）
cargo run -p api
```

### 更新（模型变更后）

当 `crates/domain` / `crates/db` 里的模型增删改字段后，schema 发生变化，需生成增量迁移：

```bash
# 1. 基于当前 schema 与最新 snapshot 的差异，生成增量迁移（如改了用户表）
cargo run -p api --bin migrate -- migration generate --name add_user_field

# 2. 应用新迁移（已应用的历史会跳过）
cargo run -p api --bin migrate -- migration apply
```

- 生成产物：`toasty/migrations/<序号>_<name>.sql`（可执行的 SQL）、`toasty/snapshots/<序号>_snapshot.toml`（schema 快照，供下次 diff）、更新 `toasty/history.toml`。
- **迁移文件与 snapshot 必须纳入版本控制**——新环境、CI、队友的本地库都依赖它们复现 schema。
- 生成后建议 review `toasty/migrations/<序号>_<name>.sql` 确认 SQL 符合预期再 apply。

### 删除 / 重置数据库

`migration reset` 会 drop 全部表（含数据），可选择是否顺带重新 apply：

```bash
# 完全重置：drop 全部表 → 重新 apply 全部迁移（回到干净初始化态，但表已重建）
cargo run -p api --bin migrate -- migration reset

# 仅清空：drop 全部表后不 apply（得到一个空库，下次启动 server 会重新 apply）
cargo run -p api --bin migrate -- migration reset --skip-migrations
```

#### 彻底从头开始（连迁移历史一起清掉）

若想完全回到项目首次 clone 后的状态（重新生成初始迁移）：

```bash
# 1. 删除本地数据库文件
rm -f data.db

# 2. （可选）若要重写迁移文件本身，删除 toasty/ 后重新生成
#    rm -rf toasty/

# 3. 重新生成并应用
cargo run -p api --bin migrate -- migration generate --name init
cargo run -p api --bin migrate -- migration apply
```

> 注意：删 `toasty/` 会丢失全部迁移历史记录，已部署的环境与新克隆的仓库 schema 将不一致，仅在你清楚后果时使用。日常重置数据只需 `migration reset`。

### 其他命令

```bash
cargo run -p api --bin migrate -- migration snapshot   # 打印当前 schema 快照
cargo run -p api --bin migrate -- migration drop        # 从历史中移除某个迁移
cargo run -p api --bin migrate -- help                  # 查看全部子命令
```
