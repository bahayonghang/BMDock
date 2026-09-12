# ADR-0001：BMDock 命名、架构边界与许可方向

- **状态**：已接受（进入 T05 的设计前提）
- **日期**：2026-09-12
- **范围**：P0/G0 验证工具与 G0 之后的 Tauri 产品骨架
- **关联验收**：AC07、AC54、AC60

## 背景

BMDock 当前交付物是开发者验证工具，产品目标是 Tauri 2 + React/TypeScript + Rust/rmcp
桌面工作台。release 与 main-preview 是两套不可合并的 Basic Memory 引擎 profile。
若不先冻结名称、所有权和许可方向，T05 可能把 P0 探针误当成生产 IPC，或让
renderer 形成第二套数据真源。

## 决策

### 命名

| 名称 | 固定含义 | 约束 |
|---|---|---|
| **BMDock** | 产品与仓库名称 | UI、文档和发行物使用此名称 |
| **bmdock-probe** | P0 开发者探针 Rust crate/二进制 | 只用于 G0 contract/dev 验证，不作为生产 IPC |
| **bmdock-app** | Tauri 应用 Rust crate 名 | 通过 typed commands/events 连接 renderer |
| **bmdock-desktop** | Tauri/React 应用目录与 npm 包名 | 路径为 `apps/bmdock-desktop`；该目录不是生产 IPC |
| **BMDock UI** | Tauri renderer 的对外名称 | 不直接调用 rmcp、文件系统或 raw `callTool` |
| **release** | Basic Memory v0.23.2，固定 commit `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048` | 21 个 MCP tools；契约独立维护 |
| **main-preview** | Basic Memory development snapshot，固定 commit `3452c821d76c083823d020984d71e06904a1ff1e` | 27 个 MCP tools；契约独立维护 |

名称只表达组件职责，不表示能力等价。T04 冻结这些名称，不把工作树中已有的
T05–T07 代码算作本任务验收。两个 engine profile 不得合并、混用或平均清单、
schema、工具计数或合约结果。T04 不把任一 profile 的能力写成另一 profile 的
已验证范围。

### 数据所有权与分层

```text
Tauri shell + BMDock UI
        │ typed commands/events
        ▼
bmdock-app Rust core (policy, DTO, supervisor, state)
        │ rmcp stdio
        ▼
官方 Basic Memory engine (selected profile)
        │
        ▼
用户显式选择的 project/vault
```

- 官方 Basic Memory 拥有 Markdown、metadata、wiki-link 及其私有数据库的权威语义。
- BMDock 拥有 UI 状态、草稿、会话状态、受控配置和恢复清单；这些都不是笔记内容的
  第二套权威索引。
- BMDock 不直接改写 Basic Memory 私有数据库，不把 renderer 路径当作写入授权。
- 所有写操作必须经过 project 路由、profile 选择、工具/参数策略、结果分类和物理
  文件观察。`accepted_unverified`、拒绝、传输错误、`timeout_unknown` 与已观察落盘
  必须分别表示。
- `bmdock-probe` 保持独立的 fixture-only 边界；T05–T08 不得复用其控制通道作为生产
  raw MCP 接口。

### 许可方向

- BMDock 原创代码的目标许可为 **AGPL-3.0-or-later**。这是方向记录，不是发行合规
  证明，也不等于 T36 已通过。
- 官方 Basic Memory、rmcp、Tauri、React/TypeScript 及其他依赖保留各自上游许可和
  归属；不因 BMDock 的目标许可而重新声明上游代码的许可。
- 工作区 `Cargo.toml` 的 `[workspace.package] license = "AGPL-3.0-or-later"` 只是
  crate 元数据方向，**不是** `LICENSE` 文件，也不能当作发行合规或依赖扫描结果。
- T36 已在仓库根加入 `LICENSE`（GNU AGPL-3.0 正文，BMDock 原创代码
  AGPL-3.0-or-later）、`NOTICE`（区分 BMDock 原创、官方 Basic Memory、第三方
  锁文件依赖）以及 `docs/sbom/lockfile-inventory.json`（离线从
  `Cargo.lock` 与 `apps/bmdock-desktop/package-lock.json` 导出的名称/版本清单）。
  漏洞扫描、人工法律复核、hosted CI 与 G7 仍为 `UNVERIFIED`。这些文件不是
  T04 的发行合规证明，也不把 G0/G7 标为通过。
- 官方 Basic Memory、rmcp、Tauri、React/TypeScript 及其他依赖保留各自上游许可和
  归属；不因 BMDock 的目标许可而重新声明上游代码的许可。
- 正式发行前仍须由发布负责人复核 `LICENSE`、NOTICE、打包内容与漏洞处置；
  T36 的 lockfile inventory 不是 hosted-CI 扫描，也不是人工法律签署。
- README 中的 AGPL 方向文字和 Cargo crate `license` 字段都不是法律结论，也不是
  `LICENSE` / NOTICE / SBOM 替代物。

### T05 进入条件

本 ADR 只冻结 T05 的设计前提，**不实现** T05 桌面代码，也不把后续工作树中的 T05–T07
改动算作 T04 验收。T05 可以开始建立桌面骨架，但必须满足以下边界：

1. 保留 `just contract`、`just contract-main` 和 `bmdock-probe` 构建路径。
2. `just dev` 切换到 Tauri 入口前，补充可回滚的 justfile 变更；G0 未通过时不得宣称
   产品安全可用。
3. 新增依赖写入真实锁文件，并在 T36 复核许可；不得用前端依赖替代 P0 验证。
4. renderer 只使用 typed IPC DTO，不暴露任意工具名、任意路径或 raw `callTool`。
5. 缺失 native GUI、真实 vault、故障注入或 hosted CI 证据继续标记为 `UNVERIFIED`。
6. 本任务不运行 `just contract`；合约 smoke 不能当作许可、native GUI 或真实 vault
   证据。

## 备选方案与否决理由

- **让 renderer 直接调用 MCP**：否决。会绕过策略、项目路由和结果分类边界。
- **复制一份本地笔记数据库作为产品真源**：否决。会与官方引擎的 Markdown/数据库
  语义产生冲突，并扩大恢复与并发风险。
- **把 release 与 main-preview 合成一套能力**：否决。T01/T02 已证明两者工具和
  schema 数量不同。
- **现在就承诺完整 AGPL 合规或 G7**：否决。T36 已加入 LICENSE/NOTICE/lockfile
  inventory，但漏洞扫描、人工法律复核、hosted CI 与 G7 仍未完成。

## 证据与限制

- 能力和固定 commit：`execution/evidence/t01-capability-baseline.json`。
- rmcp、资源/提示词和错误分类：`execution/evidence/t02-rmcp-interoperability.json`。
- Markdown 物化与并发边界：`execution/evidence/t03-markdown-concurrency-recovery.json`。
- T04 验收映射：`execution/evidence/t04-architecture-adr-licensing.json`。
- 当前仓库仍处于 G0 `in_progress`。取消、丢响应、强杀、磁盘故障、真实 vault、原生
  桌面、hosted CI、漏洞扫描和人工法律签署均不能由本 ADR 推断为已验证。
  T36 已加入 `LICENSE` / `NOTICE` / lockfile inventory；这不等于 G0 或 G7 通过。
- T04 不回退工作树中已 `completed` 的 T06/T07；这些状态会让 G0 未通过时的
  `check_source` 失败，这是已知限制，不是 T04 回归。

## 后果

T05–T08 可以按上述稳定接口开始实现；产品功能、云端、安装器和发布许可仍必须按
`execution/status.json` 的依赖推进。未来若变更产品名、数据真源或许可方向，应新增
ADR 并重新评估 AC07/AC54/AC60，而不是静默修改本文件。
