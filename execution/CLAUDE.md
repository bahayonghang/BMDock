# execution

> [仓库根](../CLAUDE.md) › `execution`
> 上次扫描：2026-09-09 13:16 +08:00

产品门禁与任务状态的机器可读真源，外加已复核的 CI 证据摘要。`just gate` 与 `scripts.tasks.check_source` 读 `status.json`。

## 文件

| 路径 | 角色 |
|---|---|
| `status.json` | 阶段、G0–G7、T01–T40、限制说明 |
| `evidence/g0-smoke-03c2839.json` | 双平台 smoke 数量与 SHA256 复核 |
| `evidence/profile-comparison.json` | release vs main-preview 的工具/schema 观察差异 |
| `evidence/t01-capability-baseline.json` | T01 双 profile 能力清单与范围限制 |
| `evidence/t02-rmcp-interoperability.json` | T02 rmcp 互操作与错误分类 |
| `evidence/t03-markdown-concurrency-recovery.json` | T03 Markdown 往返、并发与 UNVERIFIED 恢复边界 |
| `evidence/t04-architecture-adr-licensing.json` | T04 命名、所有权、许可方向和 AC07/54/60 映射 |

运行时 contract 报告写到仓库根 `artifacts/<profile>.contract.json`（gitignore），不在本目录。

## `status.json` 契约

- `schema_version`: 1
- `stage`: `"P0 verification tooling; no product gate passed"`
- `plan_sha256`: 原规划 ZIP 指纹 `a68f995f…`
- `gates.G0` … `G7`：当前仅 G0 为 `in_progress`，其余 `not_started`
- `tasks[]`：`id`（T01–T40）、`title`、`phase`（P0–P7）、`depends_on`、`acceptance_ids`、`status`
- `limitations`：作者环境缺 Rust/just、DNS 限制、单测不得推断真实引擎结果

`check_source`：若 `gates.G0.status != "passed"`，且存在 `id` 数字 ≥ 5 且 `status == "completed"` 的任务，则失败。

`just gate`：收集所有 `status != "passed"` 的门禁名，打印 JSON，有未通过则退出码 2。

## 任务阶段（摘要）

| 阶段 | 任务 | 含义 |
|---|---|---|
| P0 | T01–T04 | 版本清单、rmcp 互操作、Markdown/并发、命名许可 ADR |
| P1 | T05–T08 | Tauri 骨架、typed IPC、Supervisor、桌面壳 |
| P2 | T09–T13 | 预检、项目路由、文件树、备份、干净 Windows 运行时 |
| P3 | T14–T18 | 草稿、完整笔记 CRUD、冲突、退出恢复、编辑器安全 |
| P4 | T19–T24 | 观察/关系、图谱、检索、活动、中文召回 |
| P5 | T25–T30 | Schema、MCP 资源/提示、高级工具、导入、API 审计 |
| P6 | T31–T35 | Cloud、Agent 集成、Provider |
| P7 | T36–T40 | 安全 SBOM、安装升级、Release Gate |

工作树中 T04 已标 `completed`，对应 ADR-0001 与 `t04-architecture-adr-licensing.json`；G0 仍为 `in_progress`。HEAD 在合入前 T04 为 `planned`，不要把未提交文档写成已发布门禁，也不要把 T01–T04 写成 HEAD 已全部完成。T05 为 `in_progress`，不属于 T04 交付。T06/T07 保持 `completed`，因此 `check_source` / `test_repository_phase_order` 会失败；T04 不得回退这些状态。更新 status 须与真实验收同步，禁止因文档交接而把 G0 标 `passed`。LICENSE/NOTICE/SBOM 仍为 T36 `UNVERIFIED`。

## 证据文件用法

- `g0-smoke-03c2839.json`：`tested_code_commit`、`run_id`、两 OS 的 tools/prompts/resources/CLI/API 计数、7 项 checks、artifact SHA256。`product_gate` 字段明确 G0 未通过。
- `profile-comparison.json`：`added_tools` 六项 Unix 风格工具；`changed_tool_input_schemas` 含 `read_note`/`search_notes`/`write_note` 等。注释写明描述/默认值变化不一定是破坏性变更。
- `t04-architecture-adr-licensing.json`：AC07/AC54/AC60 文档映射。AGPL 是方向；`Cargo.toml` `license` 字段不是 LICENSE。`LICENSE`/`NOTICE`/SBOM 为 `UNVERIFIED_absent`。`product_gate` 保持 G0 `in_progress`。

改门禁或任务状态时同步 README / docs/VERIFICATION.md / 根 `CLAUDE.md` 中的阶段描述。

## 关联模块

- 读取方：[scripts/tasks.py](../scripts/tasks.py)
- 叙事：[docs](../docs/CLAUDE.md)
