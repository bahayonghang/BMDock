# 验证记录：首个实施批次

## 最终复核结果

实际受测代码提交：`03c2839f3182e177a0c8cc60354eafb28487103e`。

GitHub Actions：[CI run 34309026376](https://github.com/bahayonghang/BMDock/actions/runs/34309026376)。**Windows 与 Ubuntu 两个 job 均 completed / success**；已下载两份原始 artifact，核验 ZIP SHA256、四份报告的 suite 状态、CLI 子命令数量和进程退出字段。随后提交的交接/证据文档不改变此受测代码。

| 验证层 | Ubuntu | Windows |
|---|---|---|
| 已提交 Cargo.lock 与 rustfmt 检查 | passed | passed |
| Python 单测（42 项） | passed | passed |
| Rust Clippy（warnings as errors）及单测（4 项） | passed | passed |
| release 真实 MCP smoke suite | passed | passed |
| main-preview 真实 MCP smoke suite | passed | passed |
| `just build` release 探针 | passed | passed |
| 构建后 tracked git diff | clean | clean |

命令实现见 `justfile:20–38`、`scripts/tasks.py:111–130`。原始数据摘要与校验值：[g0-smoke-03c2839.json](../execution/evidence/g0-smoke-03c2839.json)。

## 实际能力发现结果

下列数量已在两个操作系统的原始报告中核对一致。CLI 节点包含命令组；叶子数不包含命令组。数量不等于已逐项实现或成功执行全部功能。

T01 的逐 profile 固定版本、能力数量、验收映射和范围限制汇总见 [t01-capability-baseline.json](../execution/evidence/t01-capability-baseline.json)。T01 只把双平台发现数量和独立工具名基线当作能力清单证据；CLI/API 注册树不等于功能成功。桌面负向测试、UI 漂移标记、named CLI 叶子目录、已提交握手/分页页数字段，以及独立于工具名的 schema/命令叶子 CI 阻断，仍为 UNVERIFIED。

| 项目 | release / v0.23.2 | main-preview / 3452c821 |
|---|---:|---:|
| MCP tools | 21 | 27 |
| prompts | 4 | 4 |
| resources | 1 | 32 |
| resource templates | 1 | 3 |
| CLI 注册节点 | 99 | 122 |
| CLI 叶子命令 | 83 | 104 |
| OpenAPI paths | 47 | 49 |

7 项 smoke checks 为：有效隔离配置、分页 MCP 发现、工具名基线、未知工具错误、创建/读取/文件物化、单次 append、正常退出后的文件观察。两个 profile 在两个操作系统上均正常退出（SDK close 完成、exit 0、非强杀），且退出后 fixture 中仍存在预期内容。

运行时 Schema 对比还发现 6 个共有工具的 inputSchema 不同，例如 main 的 read_note 增加 start_line/end_line，search_notes 增加 compact 和时间过滤参数。完整差异摘要见 [profile-comparison.json](../execution/evidence/profile-comparison.json)。描述或默认值差异不一定是 breaking change，不能将所有 changed 字段一概解释为不兼容。

## 本地验证范围

作者环境为 Linux / Python 3.13.5，没有 Rust/cargo/just，依赖下载受 DNS 限制。实际本地执行 `python -m scripts.tasks unit`：42 passed、0 failed、0 skipped、exit 0。Rust 编译和真实引擎结果来自上述 GitHub Actions，不来自本地 mock。

脚本宿主 Python 3.13.5；引擎 Python 3.12.12；Rust 1.90.0；rmcp 3.2.0；uv 0.12.11；just 1.40.0。依赖锁为实际解析产物，不是手写校验和。

## 本批发现并修复的问题

1. 首轮 `34307780159` / `21a7645` 的 Windows runner 缺少 setup-python 3.12.12 包。将宿主 Python 与 uv 托管的引擎 Python 分开；没有改变引擎 Python 基线。
2. 同轮浅克隆无 release tag，使官方动态版本得到 `0.0.1.dev1+c0bd87c`。补取 tag、验证其对应的固定 commit，再构建安装。
3. `34308370383` / `8a1dd54` 的双平台 smoke suites 虽通过，但审查原始报告发现 CLI 清单只有根节点。Typer 内置 Click 与外部 click.Group 类身份不同，导致 isinstance 判断漏掉子命令。改为使用各命令的 context_class 和 group 公共方法，增加关键根命令断言、循环/缺失子命令保护及 4 项回归测试。最终 `34309026376` 已确认 99/122 个 CLI 节点，不再遗漏整棵子树。

首轮远端生成的 Cargo.lock 经复核后已纳入版本控制，SHA256 为 `3c7ff364bc7691c3b225bb20521c099258854d79a352dde47498a699437fa950`。标准 CI 只验证已提交的锁和格式，不生成新锁、不改写源码。

## 未完成的产品门禁

## T02：rmcp 与官方引擎互操作

T02 在 2026-09-12 使用真实官方引擎、分两次调用 `python -m scripts.tasks contract` 完成双 profile 验证。合约断言协商 `protocolVersion` 为 `2025-11-25`，分页采集 tools/resources/templates/prompts，读取首个 resource 与 prompt，并按 profile 记录 inputSchema 指纹。错误包保持可区分：`logging/setLevel` 为本地 `policy`，畸形 `prompts/get` 为 `schema`，缺失 resource 的 `resources/read` 为上游 `rpc_or_transport`（MCP `-32602`），未知工具为 MCP `isError`。写入先标记 `accepted_unverified`，仅在 fixture `wait_note` 观察后记录物化。`search`（required `query`）与 `fetch`（required `id`）保持独立；互换参数的调用以 `isError` 失败。release 仍为 21 个工具，main-preview 仍为 27 个，未合并能力。

详细验收映射、报告 SHA256 和限制见 [t02-rmcp-interoperability.json](../execution/evidence/t02-rmcp-interoperability.json)，逐次原始 transcript 见 gitignored 的 `artifacts/release.contract.json` 与 `artifacts/main-preview.contract.json`。

本任务不是 native GUI、Job Object 或真实用户 vault 证据。丢响应、取消后接受、`timeout_unknown` 现场注入、强杀恢复和磁盘故障仍属 T03 `UNVERIFIED`；没有把这些缺失证据推断为通过。

## T03：Markdown 往返与并发写入

T03 在 2026-09-12 沿用既有双 profile 隔离合约报告，未重跑 `just contract`。release 与 main-preview 各有一份未合并的 gitignored 报告：`artifacts/release.contract.json` SHA256 `520832e513d3b9142a05cbdee58f2b15ad91a0a6086f256e7f26089a3bc97a6d`，`artifacts/main-preview.contract.json` SHA256 `2e2ab0489fa1123edca545f842cf67b5659a91a0cd76d39e68d7beedb1fca645`。每个 profile 在各自新的 `bmdock-fixture` sandbox 中先完成单篇 Markdown 往返（未知 frontmatter、中文正文、wiki-link），再启动两个独立 `bmdock-probe` 进程并发写入不同标题与 sentinel 的笔记。每个结果先保持 `accepted_unverified`，再由 `wait_note` 观察真实 Markdown；正常关闭后文件仍可读取。汇总证据见 [t03-markdown-concurrency-recovery.json](../execution/evidence/t03-markdown-concurrency-recovery.json)。

丢响应、取消后接受、`timeout_unknown` 现场注入、强杀恢复和磁盘故障没有安全的跨平台注入器，全部明确记录为 `UNVERIFIED`。不同目标并发成功不等于相同目标冲突原子性；干净关闭也不等于丢响应、取消后接受或强杀恢复。本任务不是 native GUI、Job Object 或真实用户 vault 证据。后续 T12、T16、T17 负责产品级冲突协调和恢复链。

## T04：命名、所有权、架构与许可方向

T04 是文档与证据任务，没有实现 T05 桌面代码，也没有运行 `just contract`。
产品和组件名称、双 profile 隔离、官方 Basic Memory 数据所有权、typed IPC
设计边界及 T05 进入条件固化在 [ADR-0001](adr/0001-bmdock-boundaries-naming-licensing.md)。
`bmdock-probe` 继续是 fixture-only 的 P0 开发者工具；Tauri Rust crate 名称冻结为
`bmdock-app`，前端目录/npm 包名为 `bmdock-desktop`，renderer 对外名称为 BMDock UI，
不直接访问 rmcp、文件系统或 raw `callTool`。这些名称冻结不等于 T05–T07 验收。
release（v0.23.2 / `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`，21 tools）
与 main-preview（`3452c821d76c083823d020984d71e06904a1ff1e`，27 tools）仍分别维护，
不得合并清单或结果。

原创代码的许可方向记录为 AGPL-3.0-or-later。该方向不构成完整发行法律结论。
`Cargo.toml` 工作区 `license` 字段只是 crate 元数据，不是 `LICENSE` 文件。仓库
当前没有 `LICENSE`、NOTICE 或 SBOM 文件；依赖许可归属、源码交付判断和漏洞处置
仍属于 T36，当前均为 `UNVERIFIED`。README 许可文字不能替代这些产物。
T04 的逐项 AC07/AC54/AC60 映射见 [t04-architecture-adr-licensing.json](../execution/evidence/t04-architecture-adr-licensing.json)。
该记录只证明决策已文档化，不把 G0、真实 vault、native GUI、故障恢复或发行许可
审查提前标为通过。工作树可将 T04 标为 `completed`；这不等于 HEAD 已发布该状态，
也不回退已 `completed` 的 T06/T07。

**完整 G0 仍未通过。** MCP resources/prompts/API/CLI 注册清单不是全部功能验收。完整原文往返、真正并发编辑、取消/丢响应、磁盘故障、强杀与恢复、Windows junction/睡眠恢复仍待验证；没有 Tauri GUI 或原生桌面安装器。具体交接要求见 [G0_HANDOFF.md](G0_HANDOFF.md)。

本批全部操作使用自动生成的 fixture，没有连接用户真实 Obsidian vault、全局 Basic Memory 配置或 Agent 配置。环境过滤不等于 OS 网络沙箱；正常退出后的文件观察也不构成跨 Agent 原子写入保证。`just gate` 返回非零是当前完整产品门禁的真实状态，不能通过跳过测试来消除。

## T05：Tauri + React/TypeScript 桌面骨架

T05 建立了 `apps/bmdock-desktop/` 的静态 React/Vite renderer、Tauri 2 Rust 宿主 crate `bmdock-app`、真实 `package-lock.json` 与工作区 `Cargo.lock`，以及独立的 `just tauri-dev` / `just tauri-build`。P0 的 `just dev`、`just build`、`just contract` 和 `just contract-main` 仍转发到 `scripts.tasks` 探针路径，没有切到桌面入口。renderer `App.tsx` 仍是静态 BMDock UI 壳，不调用 MCP、文件系统或路径写入。`bmdock-app` 是 T05 宿主；同一 crate 里已经存在的 typed `ipc_invoke` 与 Supervisor 属于 T06/T07，本任务不回退它们，也不把空 `main` 当作关闭条件。本任务没有运行或合并 release / main-preview 引擎契约。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t05-tauri-react-skeleton` 通过；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 14 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过；`npm run tauri:build`（`CARGO_NET_OFFLINE=true`）生成 `target/release/bmdock-app.exe`，`bundle.active` 仍为 false。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T06/T07 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 60 项：59 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。`git diff --check` 通过。未把 UI 文案、`just contract` 或编译出的 exe 当作 native GUI / WebView2 / 安装器 / Job Object / hosted CI / 真实 vault 证据。依赖锁是既有 Cargo/npm 解析产物，未提交 `node_modules/`、`dist/`、Tauri `gen/` 或其他构建目录。

T05 证据与验收映射见 [t05-tauri-react-skeleton.json](../execution/evidence/t05-tauri-react-skeleton.json)。真实窗口交互、WebView2 运行时行为、安装器、签名、Job Object、hosted CI 和真实 vault 仍属 UNVERIFIED（T13、T36–T40）；本任务不把静态壳或本地二进制当作这些证据。

## T06：typed IPC 与权限策略

T06 建立了单一 `ipc_invoke` typed 命令入口和 renderer 侧 `invokeTyped`/`listenTyped` DTO 边界。当前仅允许 `get_capabilities`、`get_runtime_state` 和 fixture-only 的 `select_project`；未知命令、任意路径字段、非 `bmdock-fixture` 项目和 raw `callTool` 形状均 fail closed。`get_runtime_state` 是 T07 Supervisor 快照的只读投影：IPC 不 start/stop 引擎，不暴露 `child_pid`；`timeout_unknown` / `transport` / `process` 只出现在 `RuntimeStateDto`，不进入 IPC 错误联合（`policy` / `schema` / `unsupported`）。`App.tsx` 保持静态壳，不调用 `invoke`。P0 的 `just contract` / `just contract-main` 仍是探针入口；`just dev` / `just build` 未切到 Tauri。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。该段不是 T05 的 AC49/AC55/AC60 证据，也不把 T07 生命周期证明提前关闭。

本机 Windows 本轮命令（2026-09-12）：`cargo fmt --all -- --check` 通过；`cargo test -p bmdock-app --locked --offline` 16 passed；`cargo test --workspace --locked --offline`（bmdock-app 16 + bmdock-probe 5）通过；`cargo check --workspace --locked --offline` 通过（T07 Supervisor API 的 dead_code 警告仍在，因 T06 不允许 start/stop IPC）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`python ./.trellis/scripts/task.py validate 09-12-t06-typed-ipc-policy` 通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T06/T07 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 60 项：59 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。`git diff --check` 通过。未把 UI 文案、工具清单、`just contract` 或编译出的 exe 当作 native GUI / WebView2 / Supervisor 实机启动 / 真实引擎 / Job Object / 真实 vault / hosted CI 证据；这些仍为 `UNVERIFIED`。

T06 证据与验收映射见 [t06-typed-ipc-policy.json](../execution/evidence/t06-typed-ipc-policy.json)。`execution/status.json` 中 T06 保持 `completed`；未改 T05/T07/G0。

## T08：桌面布局、导航与 i18n

T08 把 renderer 从英文静态卡换成默认 zh-CN 的桌面壳：`header` / `nav` / `main` landmark，中文分区「工作台 / 运行状态 / 说明」，本地文案表（无新 i18n npm 依赖）。工作台为无笔记空态；运行状态只读调用 T06 `get_capabilities` / `get_runtime_state`，调用失败为错误态，默认 `not_started` 为空态。不启动或停止 Supervisor，不 `callTool`，不选择真实项目，不写 vault。无障碍基线包括 `html lang="zh-CN"`、带 `aria-current` 的导航、可键盘聚焦控件、可见 `:focus-visible`，以及用「空态 / 错误 / 状态」标记区分，而不只靠颜色。完整编辑器仍属 T14/T18；帮助与无障碍完备性仍属 T39。

`just dev` 改为调用现有 `just tauri-dev`。**`just build` 不切换**：`.github/workflows/ci.yml` 仍用 `just build` 编译 G0 探针，因此该入口保持 `scripts.tasks build`。这是记录在案的限制，不是静默偏离。`just tauri-build`、`just contract`、`just contract-main` 保留；`just dev-main` 仍是 main-preview 探针。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合，也未把 `just contract` 当作 GUI 证明。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t08-desktop-layout-i18n` 通过；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 16 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`git diff --check` 通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退 T06/T07）。`python -m unittest tests.test_desktop_shell -v` 6 项通过。`python -m unittest discover -s tests -v` 跑 66 项：65 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）；其中 T08 `DesktopShellTests` 6 项通过。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / WebView2 / Job Object / 真实 vault / hosted CI 证据；这些仍为 `UNVERIFIED`。

T08 证据与验收映射见 [t08-desktop-layout-i18n.json](../execution/evidence/t08-desktop-layout-i18n.json)。`execution/status.json` 仅将 T08 标为 `completed`；未改 T05/T06/T07/G0。

## T09：无副作用预检与配置发现

T09 在现有 `ipc_invoke` 上增加只读命令 `run_preflight` 与 `discover_config`（空 `args`，`deny_unknown_fields`）。预检报告两条隔离的 profile 记录：release（`c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`，21 tools）与 main-preview（`3452c821d76c083823d020984d71e06904a1ff1e`，27 tools），外加不启动引擎的宿主检查。`engine_spawned` 从现有 Supervisor 生命周期投影（`not_started` 为 false；starting/connected/stopping/stopped/failed 为 true）；`files_written` 恒为 false，因为 T09 不写文件。T09 仍不 start/stop。配置发现默认根为 `"none"`，空候选列表表示未找到 BMDock 自有配置，而不是已成功读取用户 Basic Memory 主目录或生产 `config.json`。额外 path 字段（含顶层 path）schema 拒绝；任意用户路径/真实 vault 为 policy，且不打开。IPC 错误联合仍为 `policy` / `schema` / `unsupported`。T07 仍拥有 start/stop。renderer 增加 zh-CN「预检」分区，区分空态 / 错误 / 就绪，并展示快照中的 `engine_spawned` / `files_written`。不写笔记。capabilities 精确允许列表现为 5 个命令，且 `arbitrary_paths_allowed=false`、`raw_call_tool_allowed=false`。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T09 证明。

本机 Windows 检查轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t09-preflight-config-discovery` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 28 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 6 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 66 项：65 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / WebView2 / Job Object / 真实 vault / hosted CI 证据；这些仍为 `UNVERIFIED`。

T09 证据与验收映射见 [t09-preflight-config-discovery.json](../execution/evidence/t09-preflight-config-discovery.json)。`execution/status.json` 仅将 T09 标为 `completed`；未改 T05–T08/G0。

## T10：项目、工作区与显式路由

T10 在现有 `ipc_invoke` 上增加只读 `list_projects`（空 `args`，`deny_unknown_fields`），并保持 `select_project` 仅接受 `bmdock-fixture`。默认目录只返回 BMDock 自有工作区 `bmdock-workspace` 与项目 `bmdock-fixture`，不扫描、不打开用户 Obsidian vault 或全局 Basic Memory 主目录。`get_runtime_state.project` 在显式选择 fixture 之前为 `null`，选择后投影为 `bmdock-fixture`；该投影不是隐式写入目标。`ExplicitRouteArgs` 要求后续读写携带 `workspace` + `project`；额外 path 为 schema，非 fixture 为 policy。未实现 T11 笔记读取或 T15 CRUD。未添加 `search_notes` / raw `callTool`。`cross_project_search_allowed=false`，`implicit_current_project_writes=false`，`local_offline=true`。capabilities 精确允许列为 6 个命令。renderer 增加 zh-CN「项目」分区，区分未选择 / 未找到 / 错误 / 已选择 fixture。不写文件，不 start/stop Supervisor。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T10 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t10-project-workspace-routing` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 35 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 6 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 66 项：65 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / WebView2 / Job Object / 真实 vault / hosted CI 证据；这些仍为 `UNVERIFIED`。

T10 证据与验收映射见 [t10-project-workspace-routing.json](../execution/evidence/t10-project-workspace-routing.json)。`execution/status.json` 仅将 T10 标为 `completed`；未改 T05–T09/G0。

## T11：分页文件树与笔记读取

T11 在现有 `ipc_invoke` 上增加 typed `list_tree` 与 `read_note`。两条命令每次都必须携带 `ExplicitRouteArgs`（`workspace` + `project`）；缺少路由字段或额外 `path`/文件系统路径字段为 schema（`deny_unknown_fields`）。非 fixture 项目或非自有工作区为 policy，且不打开路径。`read_note` 的标识是笔记 identifier/permalink/title，不是用户 vault 文件系统 path 字段。capabilities 精确允许列为 8 个命令；未知 `call_tool` 仍失败；未恢复 T10 那个「`read_note` 命令 DTO 不存在」的断言。

分页（AC19）对齐 `scripts/core.py paginate()`：可选 cursor、有界 page_size（0 或过大为 schema）；响应含 `entries[]`、`next_cursor`（末页为 null）、`page`、`truncated=false`。非法 cursor、循环 next_cursor、截断/部分清单失败关闭（schema 或 unsupported），不会把整棵树一次性倾倒当作分页成功。

笔记读取（AC22）返回 title、identifier、markdown body 和观察分类；成功信封不是磁盘证据。测试在临时自有目录写入含中文与 wiki-link 的 BMDock fixture markdown，并断言返回正文等于物理文件。空库 `list_tree` 是空态，不是用户 vault 成功；未安装库时 `read_note` 为 unsupported。

官方引擎仍是未来数据所有者（AC33）：本任务未加入 rmcp/live MCP。`NoteLibrary` trait 由测试注入 `FixtureLibrary`；生产默认 `EmptyLibrary` 不扫描 `%APPDATA%`、用户 Obsidian 或全局 Basic Memory 主目录。官方 `list_directory` / `read_note` MCP 记录为 `UNVERIFIED`，未把 `just contract` 当作 T11 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

内容安全（AC46）：工作台以 `<pre>` 纯文本显示 Markdown，无 `dangerouslySetInnerHTML`。未实现写入、编辑、移动或删除（T14/T15）。zh-CN 工作台展示分页树与笔记预览（空态 / 错误 / 就绪）；加载更多跟随 `next_cursor`；选中笔记每次复制 ExplicitRouteArgs，不依赖隐式 `runtime.project`。预检 / 项目 / 运行状态 / 说明分区保留。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T11 证明。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t11-paginated-tree-note-read` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 48 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 6 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 66 项：65 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / WebView2 / Job Object / 真实 vault / hosted CI / 官方 MCP 读取证据；这些仍为 `UNVERIFIED`。

T11 证据与验收映射见 [t11-paginated-tree-note-read.json](../execution/evidence/t11-paginated-tree-note-read.json)。`execution/status.json` 仅将 T11 标为 `completed`；未改 T05–T10/G0。

## T12：备份清单与 fixture 恢复基线

T12 在现有 `ipc_invoke` 上增加 typed `list_backups` 与 `restore_fixture`。两条命令每次都必须携带 `ExplicitRouteArgs`（`workspace` + `project`）；缺少路由字段或额外 `path`/`root` 为 schema。非 fixture 项目或非自有工作区、以及看起来像 `%APPDATA%` / 用户 vault / `.basic-memory` 的 `backup_id` 为 policy，且不打开路径。capabilities 精确允许列为 10 个命令；未知 `call_tool` / `write_note` 仍失败。

备份清单（AC40）只列出 BMDock 生成的夹具备份标识。`scanned_user_obsidian_vault=false`，`scanned_user_basic_memory_home=false`。`files_written` 仅在一次实际写入自有文件的恢复之后为 true。空 `backups[]` 是空态，不是用户 vault 成功。生产默认 `EmptyBackupStore` 不扫描 `%APPDATA%`、用户 Obsidian 或全局 Basic Memory 主目录。

夹具恢复（AC52）把生成 Markdown 从命名快照复制到生成的自有目标目录。测试在临时自有目录写入含中文与 wiki-link 的快照，恢复后断言 `target/welcome.md` 存在且正文等于快照。`envelope_is_not_disk_proof=true`。信封成功文案 `"restored"` 分类为 `accepted_unverified`，不是磁盘证据。禁止恢复到 `%APPDATA%`、用户 Obsidian 或全局 Basic Memory 配置。

AC18：强杀、Job Object、睡眠恢复和磁盘故障保持 `UNVERIFIED`。夹具恢复成功不是 T17/T37/T38 证据。恢复清单（AC53）记录于本任务 `implement.md` 与 [t12-backup-fixture-recovery.json](../execution/evidence/t12-backup-fixture-recovery.json)。

renderer 增加 zh-CN「维护」分区，区分空态 / 错误 / 就绪；恢复按钮只对 fixture 备份标识显示。Markdown 不以 HTML 执行。不 start/stop Supervisor。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T12 证明。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t12-backup-fixture-recovery` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 59 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 6 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 66 项：65 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / WebView2 / Job Object / 真实 vault / hosted CI / 安装器恢复证据；这些仍为 `UNVERIFIED`。

T12 证据与验收映射见 [t12-backup-fixture-recovery.json](../execution/evidence/t12-backup-fixture-recovery.json)。`execution/status.json` 仅将 T12 标为 `completed`；未改 T05–T11/G0。

## T13：提前验证干净 Windows 运行时原型

T13 在现有 `ipc_invoke` 上增加 typed `inspect_windows_runtime`（`EmptyArgs`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。capabilities 精确允许列为 11 个命令；未知 `call_tool` / `write_note` 仍失败。`list_backups` / `restore_fixture` 仍每次携带 `ExplicitRouteArgs`。不 start/stop Supervisor，不拉起官方引擎，不混合 release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）。

DTO 把已观察事实与未验证主张分开：`host_os`、`webview2_files_present`（仅 Evergreen/loader 或已知安装目录）、`job_object_api_documented`、`installer_bundle_active`（必须匹配 `tauri.conf.json` `bundle.active`）、`files_written=false`、`scanned_user_obsidian_vault=false`、`scanned_user_basic_memory_home=false`。`webview2_session_verified` 与 `job_object_assigned` 仅在真正打开并交互 WebView2/Tauri 窗口、以及真正创建 Job Object 并分配子进程时可为 true；本轮均为 false。

证据分类（写进测试与证据 JSON）：编译 exe / npm build / cargo test ≠ native GUI；WebView2 文件存在 ≠ WebView2 会话；Job Object API/文档 ≠ Job Object 已分配；`just contract` ≠ Windows 运行时；T12 夹具恢复 ≠ Windows 恢复。

AC49：本机 Windows 观察到 `host_os=windows`，并返回原型 DTO。已知 EdgeWebView `Application` 目录不存在，故 `webview2_files_present=false`。native GUI 会话仍为 `UNVERIFIED`。

AC51：未启用安装器打包或签名。`installer_bundle_active=false` 与 `bundle.active=false` 一致。这是未签名原型；签名仍属 T37。未制作安装器。

AC55：`just build` 仍为 G0 探针；`just tauri-dev` / `just tauri-build` 仍为桌面入口；`just contract*` 仍为探针。未运行 `just contract` 作为 T13 证明。

renderer 在 zh-CN「运行状态」下增加「运行时」卡片（空态 / 错误 / 就绪），已观察与未验证分区分开。无 `dangerouslySetInnerHTML`。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t13-windows-runtime-prototype` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 70 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 7 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 67 项：66 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / WebView2 会话 / Job Object 分配 / 安装器 / Windows 恢复证据；这些仍为 `UNVERIFIED`。

T13 证据与验收映射见 [t13-windows-runtime-prototype.json](../execution/evidence/t13-windows-runtime-prototype.json)。`execution/status.json` 仅将 T13 标为 `completed`；未改 T05–T12/G0。

## T14：草稿持久化与编辑器会话

T14 在现有 `ipc_invoke` 上增加 typed `save_draft` 与 `load_draft`。两条命令每次都必须携带 `ExplicitRouteArgs`（`workspace` + `project`）和 `identifier`；`save_draft` 另加 `body`。缺少字段或额外 `path`/`root` 为 schema。非 fixture 路由或看起来像用户 vault / `%APPDATA%` / `.basic-memory` 的 identifier 为 policy，且不打开草稿存储。capabilities 精确允许列为 13 个命令；未知 `call_tool` / `write_note` 仍失败。

草稿是 BMDock 自有会话工件，不是第二套笔记索引，也不是官方引擎 `write_note`。生产默认 `EmptyDraftStore`：`load_draft` 为空会话（空态，不是用户 vault 成功）；`save_draft` 为 `unsupported`，文案为 `engine/draft store unavailable`（不是 `engine/library unavailable`）。测试注入 `FixtureDraftStore`，根目录为生成的自有 `{temp}/bmdock-t14-*`。

AC10：测试在保存后观察物理草稿文件。夹具正文含中文与 wiki-link `[[欢迎]]`，往返后正文等于磁盘文件。`envelope_is_not_disk_proof=true`。信封 `"saved"` 分类为 `accepted_unverified`，不是磁盘证据。`files_written` 仅在自有草稿字节存在后为 true。未写入 `%APPDATA%`、用户 Obsidian 或全局 Basic Memory 配置。未调用 MCP `write_note`。

AC09：编辑器会话可对同一标识保存并重新加载草稿。内存未保存、`disk_verified` 与 `engine_persisted` 分开；`engine_persisted` 保持 false。T15 CRUD 仍属后续任务。

AC21 / AC57：zh-CN 工作台编辑器使用带 label 的 textarea，可键盘聚焦，区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不 start/stop Supervisor。完整 Windows 编辑器安全仍属 T18；帮助与无障碍完备性仍属 T39。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T14 证明。未启用 `bundle.active`。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t14-draft-persistence-editor-session` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 82 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 8 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 68 项：67 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方引擎持久化 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T14 证据与验收映射见 [t14-draft-persistence-editor-session.json](../execution/evidence/t14-draft-persistence-editor-session.json)。`execution/status.json` 仅将 T14 标为 `completed`；未改 T05–T13/G0。

## T15：完整笔记写入编辑移动删除

T15 在现有 `ipc_invoke` 上增加 typed `write_note` / `edit_note` / `move_note` / `delete_note`。这四条是宿主命令，不是 raw `callTool`。每次都必须携带 `ExplicitRouteArgs`（`workspace` + `project`）。`write_note` 另加 `identifier` + `title` + `body`；`edit_note` 加 `identifier` + `body`；`move_note` 加 `identifier` + `destination`（permalink/标识，不是文件系统路径）；`delete_note` 加 `identifier`。缺少字段或额外 `path`/`root` 为 schema。非 fixture 路由或看起来像用户 vault / `%APPDATA%` / `.basic-memory` 的 identifier/destination 为 policy，且不打开笔记库。`RouteState.project` 不是隐式写入目标。capabilities 精确允许列为 17 个命令；未知 `call_tool` / `search_notes` 仍失败。不完整的 `write_note`（例如只有 `project`）仍为 schema。

生产默认 `EmptyLibrary`：四条 CRUD 均为 `unsupported`，文案为 `engine/library unavailable`。测试注入 `FixtureLibrary`，根目录为生成的自有 `{temp}/bmdock-t15-*`。未写入 `%APPDATA%`、用户 Obsidian 或全局 Basic Memory 配置。未添加 rmcp/live MCP。`save_draft` / `load_draft` 与 `write_note` 保持区分。`engine_persisted` 保持 false（无官方引擎）。

AC08 / AC10：写入/编辑后测试观察物理 Markdown。夹具正文含中文与 wiki-link `[[欢迎]]`，与磁盘文件逐字比对。信封 `"saved"` 不是磁盘证据。分类 `disk_verified` 与 `accepted_unverified`。

AC11：`edit_note` 覆盖已有标识正文，并观察磁盘。

AC12：`move_note` 在自有夹具库内重命名标识；旧路径消失，新路径存在且正文相同。文件系统 destination 为 policy。顺序移动到已存在目标为 `unsupported`，不主张 T16。

AC13：删除后文件从磁盘消失。删除后缺失是带观察的成功，不是用户 vault 成功。仅信封的删除为 `accepted_unverified`。

AC14：每条 CRUD 都携带 `ExplicitRouteArgs`。非 fixture 为 policy。

T16 同目标冲突 / 未知结果仍为 `UNVERIFIED`。不主张原子并发覆盖。

zh-CN 工作台提供 fixture 标识的写入/编辑/移动/删除控件；删除需确认。区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T15 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t15-note-crud-operations` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 97 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 9 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 69 项：68 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。顺序移动到已存在目标为 `unsupported`，不主张 T16。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方引擎持久化 / T16 并发覆盖 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T15 证据与验收映射见 [t15-note-crud-operations.json](../execution/evidence/t15-note-crud-operations.json)。`execution/status.json` 仅将 T15 标为 `completed`；未改 T05–T14/G0。

## T16：并发冲突与未知结果协调

T16 在现有 typed `write_note` / `edit_note` / `move_note` / `delete_note` 上增加进程内同标识 inflight 守卫（`ConflictCoordinator`）。未新增 IPC 命令，未添加 rmcp，未 raw `callTool`。第二次重叠同目标调用返回 CRUD DTO `classified_as: conflict`，`files_written=false`，`disk_verified=false`。该分类不是 `disk_verified`、不是 `timeout_unknown`、不是 policy-for-path，也不进入 IPC 错误联合体。`move_note` 同时占用 source 与 destination。顺序移动到已存在目标仍为 `unsupported`，不是 T16 原子覆盖。不同目标的顺序写入可以同为 `disk_verified`，这不是同目标原子性。

AC15：重叠同目标写入分类为 conflict。两个 OS 线程对 inflight 守卫观察到一次占用与一次 conflict。对共享 coordinator 的 scoped-thread IPC `write_note` 观察到第二次为 conflict、第一次落盘。这是宿主协调，不是 OS file lock。真实并发 OS 文件系统竞态仍为 `UNVERIFIED`。不主张 T03 强杀或原子文件系统覆盖。

AC16：`timeout_unknown` 留在 `RuntimeStateDto` / `ShutdownReceipt`。IPC 错误联合体仍为 `policy` / `schema` / `unsupported`。`auto_retry_non_idempotent_write` 在 `timeout_unknown` 之后不调用 retry helper（测试 AtomicBool 保持 false）。该快照之后的用户发起 typed write 只执行一次，不是自动重试。丢失响应、接受后取消、对官方引擎的 timeout_unknown 现场注入仍为 `UNVERIFIED`。

AC20：zh-CN 工作台用独立文案展示 conflict / timeout_unknown / disk_verified / accepted_unverified。运行状态超时未知来自 `get_runtime_state`，不启动 Supervisor。无 `dangerouslySetInnerHTML`。

AC52：conflict 与 timeout_unknown 已记录。这不是 T17/T37 恢复。强杀、Job Object 分配、磁盘故障仍为 `UNVERIFIED`。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T16 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t16-conflict-unknown-result` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 109 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 10 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 70 项：69 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / OS file lock / 强杀恢复 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T16 证据与验收映射见 [t16-conflict-unknown-result.json](../execution/evidence/t16-conflict-unknown-result.json)。`execution/status.json` 仅将 T16 标为 `completed`；未改 T05–T15/G0。

## T17：正常退出排空与故障恢复

T17 在现有 `ipc_invoke` 上增加 typed `begin_shutdown`（`EmptyArgs`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。该命令不启动 Supervisor、不拉起引擎、也不把强杀活动子进程当作 T17 证明。Supervisor 未启动时的收据是空闲/`not_started` 排空（`forced=false`），不是活动引擎寿命。capabilities 精确允许列为 18 个命令。

排空开始后，新的 typed `write_note` / `edit_note` / `move_note` / `delete_note` 返回 `unsupported`（`host is draining`），不是 `disk_verified`。Inflight `ConflictCoordinator` 键要么由持有者结束，要么记录为 `inflight_unknown`，不静默重试。

AC05：`ShutdownReceipt` 复用 T07 字段（`transport_cancelled`、`child_exited`、`forced`、`timeout_unknown`）。未启动路径不 spawn、不 kill。带 FakeChild 的优雅关闭仍记录 `forced=false`。这些确定性假对象不是 native process-tree。

AC17：优雅路径 `forced=false`。仅在记录未完成 inflight 时把收据标为 `timeout_unknown`，且仍不 `forced`。

AC18：强杀、Job Object、睡眠恢复和磁盘故障保持 `UNVERIFIED`。成功的排空测试不是那些证明，也不是 T37/T38。

AC52：宿主排空、T16 `conflict` 与 T12 `restore_fixture` 分开记录。夹具恢复不是本排空。zh-CN 运行状态用独立文案展示 idle / draining / timeout_unknown / conflict。无 `dangerouslySetInnerHTML`。不启动 Supervisor。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T17 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t17-graceful-exit-recovery` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 120 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 11 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 71 项：70 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / Job Object / 强杀恢复 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T17 证据与验收映射见 [t17-graceful-exit-recovery.json](../execution/evidence/t17-graceful-exit-recovery.json)。`execution/status.json` 仅将 T17 标为 `completed`；未改 T05–T16/G0。

## T18：编辑器内容安全与 Windows 体验

T18 未新增 IPC 命令。笔记与草稿正文保持不透明 UTF-8 文本。zh-CN 工作台继续使用带 label 的 textarea，并增加 `<pre>` 纯文本预览。无 `dangerouslySetInnerHTML`。辅助分类 `unsafe_html_present` 与 `executed=false`，以及 CRLF / LF。夹具 `save_draft` / `write_note` / `edit_note` 按精确字节落盘并回读，包含 CRLF。含 `<script>`、`<img onerror>` 与 wiki-link `[[欢迎]]` 的正文在 T14 `{temp}/bmdock-t14-*` 与 T15 `{temp}/bmdock-t15-*` 路径上按原文往返，并以文本渲染。把 CRLF 规范成 LF 后不得标为 `disk_verified`。Windows 反斜杠文件系统标识仍为 policy。不启动 Supervisor，不 raw `callTool`，不混合双 profile。capabilities 仍为 18 个命令。

AC46：Markdown/HTML 永不执行。textarea + `<pre>` 文本预览。正文中的 script / onerror / `[[欢迎]]` 作为精确文本落盘。`executed` 恒为 false。

AC21：zh-CN 编辑器仍在工作台布局中，区分空态 / 错误 / 就绪。本机 Windows 夹具路径做了 CRLF 精确字节往返。native GUI / IME 会话仍为 `UNVERIFIED`。

AC57：控件有 label、可键盘聚焦、`:focus-visible`。不主张 T39 帮助完备性。未在原生窗口中输入，故 IME / native GUI 为 `UNVERIFIED`。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T18 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t18-editor-windows-safety` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 125 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 12 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 72 项：71 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / IME / 用户 vault / hosted CI 证据；这些仍为 `UNVERIFIED`。

T18 证据与验收映射见 [t18-editor-windows-safety.json](../execution/evidence/t18-editor-windows-safety.json)。`execution/status.json` 仅将 T18 标为 `completed`；未改 T05–T17/G0。

## T19：观察与关系语义面板

T19 在现有 `ipc_invoke` 上增加 typed `list_relations`（`ExplicitRouteArgs` + `identifier`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。非 fixture 路由或文件系统标识为 policy，且不打开库。capabilities 精确允许列为 19 个命令。

关系（AC28）来自 BMDock 自有夹具 Markdown 正文中的 wiki-link `[[...]]`，不是第二套笔记索引，也不是官方引擎图谱 MCP。测试注入 `FixtureLibrary`，并观察列出的目标等于物理文件中的 wiki-link。关系标识是 permalink/identifier，不是文件系统路径。缺失目标是 empty/unsupported，不是用户 vault 成功。生产 `EmptyLibrary` 返回空 `relations[]`（空态），不是用户 vault 成功。官方 `recent_activity` / `build_context` 仍为 `UNVERIFIED`。未加入 rmcp。未运行 `just contract` 作为 T19 证明。

观察（AC27）面板显示当前笔记的 `classified_as`（`disk_verified` / `accepted_unverified` / `conflict` / `empty`），与关系列表分开。夹具 `list_relations` 在核对物理文件后为 `disk_verified`。空库为 empty。同目标冲突仍是 CRUD 观察分类，不是关系目标分类。zh-CN 工作台「观察 / 关系」区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。正文中的 HTML 仍以文本显示（T18）。不启动 Supervisor。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t19-observation-relation-panel` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 130 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 13 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。`python -m unittest discover -s tests -v` 跑 73 项：72 ok，1 ERROR `test_repository_phase_order`（同一 `check_source`）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方图谱 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T19 证据与验收映射见 [t19-observation-relation-panel.json](../execution/evidence/t19-observation-relation-panel.json)。`execution/status.json` 仅将 T19 标为 `completed`；未改 T05–T18/G0。

## T20：局部图谱与渐进展开

T20 在现有 `ipc_invoke` 上增加 typed `expand_graph`（`ExplicitRouteArgs` + `identifier` + optional `cursor` / `page_size`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。非 fixture 路由或文件系统标识为 policy，且不打开库。`page_size` 0 或过大、非法/重复 cursor 为 schema。截断库存为 unsupported，不是成功。capabilities 精确允许列为 20 个命令。

图谱是 BMDock 自有的一跳邻域，来自 T19 夹具 Markdown wiki-link `[[...]]`，不是第二套数据库，也不是官方引擎图谱 MCP，也未引入 vis.js。测试注入 `FixtureLibrary`，并观察 1 跳邻居等于物理源文件中的 wiki-link；展开邻接节点则观察该文件的 wiki-link（第 2 跳）。缺失目标是空节点，不是用户 vault。标识是 permalink，不是文件系统路径。生产 `EmptyLibrary` 返回空节点/边与 empty 观察，不是用户 vault 成功。每次调用深度为 1；`next_cursor` 或展开返回节点加载下一页有界邻居，不会一次倾倒整个夹具库。

AC27：图谱 DTO 上保留 `classified_as`（`disk_verified` / `accepted_unverified` / `conflict` / `empty`），与节点/边分开。夹具展开核对物理文件后为 `disk_verified`。空库为 empty。冲突仍是 CRUD 分类，不是图谱节点分类。zh-CN 工作台「图谱」区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。正文中的 HTML 仍以文本显示（T18）。不启动 Supervisor。

AC29：夹具物理 wiki-link 中的中文标识（如 `欢迎`）作为 permalink 出现，展开不丢 CJK。这是夹具局部图谱身份，不是 T24 召回基准，也不是官方 search MCP。

AC56：有界宿主展开 + 截断 fail-closed。native GUI、WebView2 会话、安装器、hosted CI 仍为 `UNVERIFIED`。cargo test / npm build / UI 文案不是 native 证明。记录为 bounded-host-expansion。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T20 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t20-local-graph-expansion` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 134 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 14 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方图谱 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T20 证据与验收映射见 [t20-local-graph-expansion.json](../execution/evidence/t20-local-graph-expansion.json)。`execution/status.json` 仅将 T20 标为 `completed`；未改 T05–T19/G0。

## T21：全文语义混合检索

T21 在现有 `ipc_invoke` 上增加 typed `search_notes`（`ExplicitRouteArgs` + 必填 `query` + optional `cursor` / `page_size`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。额外 `id`（fetch 身份对调）为 schema。缺/空 query 为 schema。非 fixture 路由或把 query 当成文件系统路径为 policy，且不打开库。`page_size` 0 或过大、非法/重复 cursor 为 schema。截断库存为 unsupported，不是成功。capabilities 精确允许列为 21 个命令。typed `search_notes` 允许；MCP identity `search` 与 `call_tool` 仍拒绝。

检索是 BMDock 自有夹具 Markdown 标题/正文的词法命中，不是第二套数据库，也不是官方引擎语义检索，也不是 raw MCP `search`/`fetch`/`callTool`。测试注入 `FixtureLibrary`，并观察命中标识对应的物理文件精确 UTF-8 文本包含查询（含中文）。命中标识是 permalink，不是文件系统路径。生产 `EmptyLibrary` 返回空 hits 与 empty 观察，不是用户 vault 成功。信封文案不是磁盘证明。

AC23：夹具磁盘词法命中。信封成功不是磁盘证明。核对物理文件后 `classified_as=disk_verified`。空库为空态。

AC24：混合 DTO 分开 `lexical_score` 与 `semantic_score`。`semantic_enabled=false`，无嵌入后端。官方语义/模型仍为 `UNVERIFIED`。未实现 T23 Inspector。未把 release/main-preview 工具数写入检索 DTO。

AC25：每次调用都携带显式 workspace+project。非 fixture 为 policy。检索不泄漏其他项目或用户 vault。缺少路由的跨项目查询是 schema/policy，不是成功。

zh-CN 工作台「检索」区分空态 / 错误 / 就绪，查询输入有 label，结果列出 permalink 与分数。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖，无 vis.js。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T21 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t21-hybrid-search` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 138 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 15 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方语义检索 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T21 证据与验收映射见 [t21-hybrid-search.json](../execution/evidence/t21-hybrid-search.json)。`execution/status.json` 仅将 T21 标为 `completed`；未改 T05–T20/G0。

## T22：上下文预览与近期活动

T22 在现有 `ipc_invoke` 上增加 typed `preview_context`（`ExplicitRouteArgs` + 必填 `identifier` + optional `query`）与 `list_activity`（`ExplicitRouteArgs` + optional `cursor` / `page_size`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。缺 preview identifier 为 schema。非 fixture 路由或文件系统 identifier / query-as-path 为 policy，且不打开库。`page_size` 0 或过大、非法/重复 cursor 为 schema。截断库存为 unsupported，不是成功。capabilities 精确允许列为 23 个命令。typed `preview_context` / `list_activity` 允许；MCP identity `search` / `recent_activity` / `build_context` 与 `call_tool` 仍拒绝。

预览是 BMDock 自有夹具 Markdown 的物理 UTF-8 片段（可选 query 窗口），不是官方 `build_context`，也不是第二套索引。测试注入 `FixtureLibrary`，并观察片段是物理文件的子串（含中文）。HTML 仍以纯文本显示，`executed=false`。信封文案不是磁盘证明。核对物理文件后 `classified_as=disk_verified`。生产 `EmptyLibrary` 与缺文件返回空片段与 empty 观察，不是用户 vault 成功。检索命中预览复用 `preview_context`。

近期活动是夹具 Markdown permalink 的文件 mtime 顺序，不是官方 `recent_activity`。测试观察列出的标识对应夹具目录中的物理文件，顺序跟随 mtime。标识是 permalink，不是文件系统路径。`engine_activity=false`。空库为空态，不是用户 vault。有界分页；空与错误保持区分。

AC30：预览片段匹配物理文件子串（含中文）。信封不是磁盘证明。

AC31：近期活动是夹具本地、有界、空与错误可分。官方 `recent_activity` / `build_context` 仍为 `UNVERIFIED`。未加入 rmcp。未把 `just contract` 当作 T22 证明。

zh-CN 工作台「预览」/「近期活动」区分空态 / 错误 / 就绪。预览在 `<pre data-preview="text" data-executed="false">`。活动列出 permalink 与观察 mtime。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T22 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-12）：`python ./.trellis/scripts/task.py validate 09-12-t22-context-activity-preview` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 145 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 16 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 recent_activity / build_context / hosted CI 证据；这些仍为 `UNVERIFIED`。

T22 证据与验收映射见 [t22-context-activity-preview.json](../execution/evidence/t22-context-activity-preview.json)。`execution/status.json` 仅将 T22 标为 `completed`；未改 T05–T21/G0。

## T23：检索Inspector与模型状态

T23 在现有 `ipc_invoke` 上增加 typed `inspect_search`（`ExplicitRouteArgs` + 必填 `query` + optional `identifier`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。额外 `id`（fetch 身份对调）为 schema。缺/空 query 为 schema。空 identifier 为 schema。非 fixture 路由、把 query 当成文件系统路径、或文件系统 identifier 为 policy，且不打开库。capabilities 精确允许列为 24 个命令。typed `inspect_search` 允许；MCP identity `search` 与 `call_tool` 仍拒绝。

Inspector 解释 BMDock 自有 T21 夹具词法检索：查询、命中 permalink、分开的 `lexical_score` 与 `semantic_score`，以及语义关闭原因。不是官方引擎语义，不是嵌入后端，也不是 T24 召回。测试注入 `FixtureLibrary`，并观察命中标识对应的物理 UTF-8 文件包含查询（含中文），且分数保持区分（lexical > 0，semantic=0）。生产 `EmptyLibrary` 返回空 Inspector（无 hits，`model_loaded=false`），不是用户 vault 成功。信封文案不是磁盘证明。

AC24：Inspector DTO 分开 `lexical_score` 与 `semantic_score`。`semantic_enabled=false`。`model_id` 为空/none。`model_loaded=false`。`embedding_backend=none`。官方语义/模型仍为 `UNVERIFIED`。未把 release/main-preview 工具数写入 Inspector DTO。

AC38：Inspector 只读。`files_written=false`。不启动 Supervisor、不拉起引擎、不写文件。双 profile 隔离（21 vs 27），不混入 Inspector DTO。`timeout_unknown` 留在 `RuntimeStateDto`，不进入 IPC 错误联合体。

AC41：缺失/不可用的语义能力显式标为关闭（`semantic_enabled=false`，`model_loaded=false`），从不静默当成已启用成功。未知模型是 unclassified/unverified，不是已加载模型。未实现 T30 extras 或 T34 providers。`get_runtime_state` 投影 `semantic_model_loaded=false`，不宣称模型已加载。未加入 rmcp。

zh-CN 工作台「检索 Inspector」区分空态 / 错误 / 就绪，显示查询、命中 permalink、分数、`semantic_enabled=false`、`model_loaded=false`。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T23 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-13）：`python ./.trellis/scripts/task.py validate 09-12-t23-search-inspector-model-state` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 148 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 17 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方语义模型 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T23 证据与验收映射见 [t23-search-inspector-model-state.json](../execution/evidence/t23-search-inspector-model-state.json)。`execution/status.json` 仅将 T23 标为 `completed`；未改 T05–T22/G0。

## T24：中文召回与性能基准

T24 在现有 `ipc_invoke` 上增加 typed `run_recall_benchmark`（`ExplicitRouteArgs` + optional `k`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。非 fixture 为 policy，且不打开库。`k` 0 或大于 64 为 schema。默认 `k` 为 20。capabilities 精确允许列为 25 个命令。typed `run_recall_benchmark` 允许；MCP identity `search` 与 `call_tool` 仍拒绝。

基准是 BMDock 自有夹具能力，覆盖 `FixtureLibrary`：小中文 gold 集（查询如 `欢迎`），相关 permalink 以物理 UTF-8 文件正文包含查询为准。recall@k 来自 T21 `search_notes` 命中。命中仅在物理文件包含查询时计数。缺失相关文件为空态，不是用户 vault。DTO 记录进程内 `search_elapsed_ms` 与 `expand_elapsed_ms`。`semantic_enabled=false`。`engine_search=false`。`native_gui=false`。生产 `EmptyLibrary` 返回零查询空基准（`classified_as: empty`），不是用户 vault 成功。信封文案不是磁盘证明。

AC26：中文 recall@k 由夹具磁盘 gold 计算，不是 UI 文案。命中仅在物理文件包含查询时计数。缺失相关文件为空态，不是用户 vault。官方引擎中文召回仍为 `UNVERIFIED`。

AC29：中文 permalink（`欢迎`）仍出现在图谱展开与检索命中中；展开/检索不丢弃 CJK。这是夹具身份，不是官方 search MCP。

AC56：只记录有界进程内 timings。native 窗口卡顿、WebView2、安装包、hosted CI 仍为 `UNVERIFIED`。`cargo test` / `npm build` / UI 文案不是 AC56 native 证明。未宣称全库性能。

zh-CN 工作台「基准」区分空态 / 错误 / 就绪，显示 recall@k、查询数、elapsed_ms、`semantic_enabled=false`、`native_gui=false`。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T24 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-13）：`python ./.trellis/scripts/task.py validate 09-12-t24-chinese-recall-benchmark` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 152 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 18 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方中文召回 / hosted CI 证据；这些仍为 `UNVERIFIED`。

T24 证据与验收映射见 [t24-chinese-recall-benchmark.json](../execution/evidence/t24-chinese-recall-benchmark.json)。`execution/status.json` 仅将 T24 标为 `completed`；未改 T05–T23/G0。

## T25：Schema 工作台

T25 在现有 `ipc_invoke` 上增加 typed `schema_validate`（`ExplicitRouteArgs` + 必填 `identifier` + optional `schema_id`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。缺 identifier 为 schema。提供空 `schema_id` 为 schema。非 fixture 路由、文件系统 identifier 或文件系统 schema_id 为 policy，且不打开库。capabilities 精确允许列为 26 个命令。typed `schema_validate` 允许；MCP identity `schema_infer` / `schema_diff` 与 `call_tool` 仍拒绝。

校验是 BMDock 自有夹具能力，覆盖 `FixtureLibrary` Markdown/JSON-like frontmatter 与宿主 schema 目录（默认 `note` 要求物理 UTF-8 上的 `title`/`body`）。不是官方 MCP `schema_validate` / `schema_infer` / `schema_diff`。生产 `EmptyLibrary` 返回 empty/unsupported，不是用户 vault 成功。测试注入 `FixtureLibrary`，在物理文件匹配 schema 时观察 `valid`，在磁盘缺少必填字段时观察 `invalid`。信封文案不是磁盘证明。`engine_schema=false`。官方 schema MCP 仍为 `UNVERIFIED`。

AC32：夹具笔记相对 BMDock 自有 schema 校验，必填 title/body 在物理 UTF-8 上观察。`valid` / `invalid` / `empty` / `unsupported` 保持区分。生产空库不是用户 vault 成功。未把 UI 文案或工具清单当作磁盘证明。

zh-CN 工作台「Schema 工作台」区分空态 / 错误 / 就绪，显示 verdict、schema_id、已观察 title/body。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T25 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-13）：`python ./.trellis/scripts/task.py validate 09-12-t25-schema-workbench` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 156 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 19 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 schema MCP / hosted CI 证据；这些仍为 `UNVERIFIED`。

T25 证据与验收映射见 [t25-schema-workbench.json](../execution/evidence/t25-schema-workbench.json)。`execution/status.json` 仅将 T25 标为 `completed`；未改 T05–T24/G0。

## T26：MCP 资源与提示词工作台

T26 在现有 `ipc_invoke` 上增加 typed `list_resources` 与 `list_prompts`（`ExplicitRouteArgs` + 可选 `cursor`/`page_size`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。`page_size` 0 或大于 64 为 schema。默认 `page_size` 20，上限 64。非 fixture 路由为 policy，且不打开库。capabilities 精确允许列为 28 个命令。typed `list_resources` / `list_prompts` 允许；MCP identity `resources/list` / `resources/read` / `prompts/list` / `prompts/get` 与 `call_tool` 仍拒绝。

目录是 BMDock 自有夹具能力：顶层 `*.md`（排除 `*.prompt.md`）给出资源标识；`*.prompt.md` sidecar 给出提示词模板。不是官方 MCP `resources/list` / `resources/read` / `prompts/list` / `prompts/get`。生产 `EmptyLibrary` 返回空目录 / `classified_as: empty`，不是用户 vault 成功。测试注入 `FixtureLibrary`（`{temp}/bmdock-t26-*`），命中必须对应物理文件。信封文案不是磁盘证明。`engine_resources=false`，`engine_prompts=false`。官方 resources/prompts MCP 仍为 `UNVERIFIED`。未把 `just contract` 当作 T26 证明。

AC33：资源工作台列出从夹具磁盘观察到的 BMDock 自有资源标识（或空目录）。命中匹配物理 Markdown 文件。生产空库不是用户 vault 成功。未把 UI 文案或工具清单当作磁盘证明。

AC34：提示词工作台列出夹具 sidecar 上的 BMDock 自有提示词模板（或空目录）。`engine_prompts=false`。双 profile 仍隔离（21 vs 27），不写入 DTO。

zh-CN 工作台「资源」/「提示词」区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T26 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-13）：`python ./.trellis/scripts/task.py validate 09-12-t26-mcp-resources-prompts` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 160 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 20 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 MCP resources/prompts / hosted CI 证据；这些仍为 `UNVERIFIED`。

T26 证据与验收映射见 [t26-mcp-resources-prompts.json](../execution/evidence/t26-mcp-resources-prompts.json)。`execution/status.json` 仅将 T26 标为 `completed`；未改 T05–T25/G0。

## T27：受控高级工具与 CLI 任务中心

T27 在现有 `ipc_invoke` 上增加 typed `inspect_tools` 与 `list_cli_inventory`（`ExplicitRouteArgs` + 必填 `profile_id`，后者另有可选 `cursor`/`page_size`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。缺 `profile_id` 为 schema。非 fixture 路由为 policy，且不打开库。capabilities 精确允许列为 30 个命令。typed `inspect_tools` / `list_cli_inventory` 允许；`call_tool` 与 MCP identity `search` / `fetch` / `tools/call` 仍拒绝。

`inspect_tools` 是 BMDock 自有对照：typed 官方工具允许列 vs **单个** profile 的静态基线。不是官方 MCP `tools/list` 或 `call_tool`。每次只对照一个 profile。把 21 与 27 混合/平均为 unsupported。生产 `EmptyLibrary` 返回空 `tools[]` / `classified_as: empty`，不是用户 vault 成功。测试注入 `FixtureLibrary`（`{temp}/bmdock-t27-*`）。`engine_tools=false`。官方工具执行仍为 `UNVERIFIED`。未把 `just contract` 当作 T27 证明。

`list_cli_inventory` 列出已提交的叶子路径段（`compatibility/cli-leaves.json`），不 spawn `engine_worker.py`，不现场执行官方 CLI。会隐藏子命令的粗分组为 unsupported。任务只展示目录（`executed=false`）。生产空库为空态，不是 CLI 成功。`engine_cli=false`。现场官方 CLI 仍为 `UNVERIFIED`。完整 T01 83/104 命名树仍为 `UNVERIFIED`。

AC35：21 vs 27 保持隔离。`search` 与 `fetch` 保持不同身份且为 denied。桌面不暴露 `call_tool`。未知官方工具列为 denied/unverified，不自动放行。

AC36：CLI 目录是叶子名，不是会隐藏遗漏的粗分组。生产空目录是 empty，不是用户 vault / CLI 成功。现场官方 CLI 仍为 UNVERIFIED。

AC37：工具 / CLI 中心只使用 typed 允许列命令。raw `callTool` / 任意 CLI spawn 仍拒绝。

AC38：只读。`files_written=false`。不启动 Supervisor。本地离线。

AC58：未知工具名与额外 MCP 方法以 schema/policy/unsupported 失败关闭。漂移可见为 denied vs allowlisted。不自动接纳新工具。

zh-CN 工作台「工具」/「CLI 任务中心」区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T27 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-13）：`python ./.trellis/scripts/task.py validate 09-12-t27-controlled-tools-cli-center` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 163 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 21 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 MCP 工具执行 / 现场官方 CLI / hosted CI 证据；这些仍为 `UNVERIFIED`。

T27 证据与验收映射见 [t27-controlled-tools-cli-center.json](../execution/evidence/t27-controlled-tools-cli-center.json)。`execution/status.json` 仅将 T27 标为 `completed`；未改 T05–T26/G0。

## T28：导入与日常维护界面

T28 在现有 `ipc_invoke` 上增加 typed `import_notes`（`ExplicitRouteArgs` + 必填 `source_id`，`deny_unknown_fields`）。`source_id` 是 BMDock 自有夹具源标识，不是文件系统路径。额外 `path`/`root` 为 schema。缺 `source_id` 为 schema。非 fixture 路由与文件系统 `source_id` 为 policy，且不打开库或备份 store。capabilities 精确允许列为 31 个命令。typed `import_notes` 允许。

导入把 BMDock 自有夹具 Markdown 复制进自有夹具库。测试注入 `{temp}/bmdock-t28-*`（含 `欢迎.md`），导入后观察物理 UTF-8。信封 `"imported"` 文案不是磁盘证明。生产 `EmptyLibrary` 返回空 `files[]` / `classified_as: empty`，不是用户 vault 成功。`engine_import=false`。官方 extras / 文档摄取与现场 CLI import 仍为 `UNVERIFIED`（T30）。未把 `just contract` 当作 T28 证明。

AC39：物理文件正文（含中文）才是导入证据。空库导入是 empty，不是用户 vault 成功。不扫描用户 Obsidian 或全局 Basic Memory 主目录。

AC40：维护 UI 继续只列出 BMDock 生成的夹具备份（复用 T12 `list_backups` / `restore_fixture`）。`scanned_user_obsidian_vault=false`。不恢复用户 vault。导入与 `restore_fixture` 分开。

zh-CN 工作台「导入」+ 既有「维护」区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T28 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令（2026-09-13）：`python ./.trellis/scripts/task.py validate 09-12-t28-import-maintenance-ui` 通过；`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`（bmdock-app 168 + bmdock-probe 5）和 `cargo check --workspace --locked --offline` 通过（既有 T07 dead_code 警告仍在）；`npm run build`（`apps/bmdock-desktop`，未跑 `npm ci`）通过；`git diff --check` 通过。`python -m unittest tests.test_desktop_shell -v` 22 项通过。`python -m scripts.tasks unit` 以 `A later task was completed before G0` 失败（G0 未 passed，且 T05+ 已 completed，未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 extras / 现场 CLI import / hosted CI 证据；这些仍为 `UNVERIFIED`。

T28 证据与验收映射见 [t28-import-maintenance-ui.json](../execution/evidence/t28-import-maintenance-ui.json)。`execution/status.json` 仅将 T28 标为 `completed`；未改 T05–T27/G0。

## T29：公开 API 与 CLI 遗漏审计

T29 在现有 `ipc_invoke` 上增加 typed `inspect_api_audit`（`ExplicitRouteArgs` + 必填 `profile_id`，`release` | `main-preview`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。缺 `profile_id` 或 mixed 为 schema。非 fixture 路由为 policy，且不打开库。每次只审计一个 profile。把 21 与 27 混合/平均为 unsupported。capabilities 精确允许列为 32 个命令。typed `inspect_api_audit` 允许。

审计列出已提交目录中的命名 CLI/API 叶子（`compatibility/cli-leaves.json` + typed IPC 允许列对照所选 profile MCP 基线）。会隐藏遗漏的粗分组为 unsupported。缺口列为 missing/unverified，不会静默当成已覆盖。生产 `EmptyLibrary` 返回空审计 / `classified_as: empty`，不是用户 vault 成功，也不是完整 API 覆盖。测试注入 `FixtureLibrary`（`{temp}/bmdock-t29-*`）。不 spawn `engine_worker`，不把 `just contract` 当作 T29 证明，不现场执行官方 CLI。

AC36：审计列出命名叶子，不是会隐藏遗漏的粗分组。生产空审计是 empty，不是用户 vault / 完整 API 成功。现场官方 CLI 与完整 T01 83/104 树仍为 UNVERIFIED。

AC41：缺失/不可用能力（semantic、extras ingest、cloud、live MCP、official schema MCP、live CLI）显式标为 `unavailable`/`unverified`，从不静默当成已启用。`semantic_enabled=false`，`model_loaded=false`。

AC58：未知工具/命令失败关闭；漂移可见为 denied vs allowlisted。不自动接纳新工具。

AC59：审计记录哪些允许列 IPC 命令已存在，以及哪些官方 CLI/API 叶子仍未覆盖。渲染器把 missing 与 present 分开显示。不宣称完整 API 覆盖（`full_api_coverage=false`）。

zh-CN 工作台「API/CLI 审计」区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T29 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令见 [t29-api-cli-audit.json](../execution/evidence/t29-api-cli-audit.json)。`python -m scripts.tasks unit` 以 G0 未 passed 且 T05+ 已 completed 失败（未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 现场官方 CLI / live MCP / hosted CI 证据；这些仍为 `UNVERIFIED`。

T29 证据与验收映射见 [t29-api-cli-audit.json](../execution/evidence/t29-api-cli-audit.json)。`execution/status.json` 仅将 T29 标为 `completed`；未改 T05–T28/G0。

## T30：本地可选 extra 与文档摄取

T30 在现有 `ipc_invoke` 上增加 typed `inspect_extras`（`ExplicitRouteArgs` + 可选 `extra_id`，`deny_unknown_fields`）与 typed `ingest_document`（`ExplicitRouteArgs` + 必填 `source_id`，夹具标识，不是文件系统路径）。额外 `path`/`root` 为 schema。缺路由为 schema。空 `extra_id` 为 schema。缺 `source_id` 为 schema。非 fixture 路由与文件系统 extra/source 为 policy，且不打开库。capabilities 精确允许列为 34 个命令。typed `inspect_extras` / `ingest_document` 允许。

Extra 目录是 BMDock 自有夹具 sidecar（`.txt` / `.md`）清单。测试注入 `{temp}/bmdock-t30-*`，命中必须与物理 UTF-8 一致（含中文）。信封成功不是磁盘证明。生产 `EmptyLibrary` 返回 `extras_enabled=false`、空目录、`classified_as: empty`，不是用户 vault 成功。`ingest_document` 把命名 extra 复制进夹具笔记，与 T28 `import_notes`（Markdown 复制）分开。官方 Basic Memory extras / PDF/Office 摄取仍为 `UNVERIFIED`。未把 `just contract` 当作 T30 证明。不启动 Supervisor。无 rmcp。

AC39：物理文件正文（含中文）才是 extras/ingest 证据。空库 extras/ingest 是 empty，不是用户 vault 成功。不扫描用户 Obsidian 或全局 Basic Memory 主目录。

AC41：extras / semantic / cloud 保持显式不可用，除非夹具 extra 实际存在于磁盘。默认 `extras_enabled=false`。没有磁盘文件却宣称 `extras_enabled=true` 为 unsupported。`semantic_enabled=false`。`model_loaded=false`。从不静默当成已启用成功。

zh-CN 工作台「Extra / 文档摄取」区分空态 / 错误 / 就绪。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T30 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令见 [t30-extras-document-ingestion.json](../execution/evidence/t30-extras-document-ingestion.json)。`python -m scripts.tasks unit` 以 G0 未 passed 且 T05+ 已 completed 失败（未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 extras / PDF/Office / hosted CI 证据；这些仍为 `UNVERIFIED`。

T30 证据与验收映射见 [t30-extras-document-ingestion.json](../execution/evidence/t30-extras-document-ingestion.json)。`execution/status.json` 仅将 T30 标为 `completed`；未改 T05–T29/G0。

## T31：Cloud 与远程授权连接

T31 在现有 `ipc_invoke` 上增加 typed `inspect_cloud`（`ExplicitRouteArgs`，`deny_unknown_fields`）。额外 `path`/`root` 为 schema。缺路由为 schema。非 fixture 路由为 policy，且不打开库。capabilities 精确允许列为 35 个命令，并报告 `cloud_allowed=false`。typed `inspect_cloud` 允许。

Cloud 保持 FAIL-CLOSED 本地离线。生产默认 `cloud_enabled=false`、`remote_auth=false`、`credentials_present=false`。宣称 connected/authenticated 而没有 live official cloud session 为 unsupported。测试注入 `{temp}/bmdock-t31-*`，默认仍报告 `cloud_enabled=false`。BMDock 自有夹具 `cloud-claimed` 标记仍不是 live cloud，分类为 unsupported，不是 connected。未存储密钥，未读取用户环境 token，未联系远程主机。未把 `just contract` 当作 T31 证明。不启动 Supervisor。无 rmcp。

AC42：无隐式 cloud/remote 路由。本地离线。release（21）与 main-preview（27）保持隔离，不把 profile 混进 cloud DTO。

AC45：不打开 cloud/remote/credential/real-vault 路由。capabilities 报告 `cloud_allowed=false`。不存储密钥。不从环境读取用户 token。不联系远程主机。

AC47：未授权远程是 policy/unsupported，不是成功。夹具 cloud-claimed 不是 connected。

zh-CN 工作台「Cloud / 远程授权」区分空态 / 错误 / 就绪，显示未连接 / 未授权。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T31 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令见 [t31-cloud-remote-auth.json](../execution/evidence/t31-cloud-remote-auth.json)。`python -m scripts.tasks unit` 以 G0 未 passed 且 T05+ 已 completed 失败（未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 cloud / hosted CI 证据；这些仍为 `UNVERIFIED`。

T31 证据与验收映射见 [t31-cloud-remote-auth.json](../execution/evidence/t31-cloud-remote-auth.json)。`execution/status.json` 仅将 T31 标为 `completed`；未改 T05–T30/G0。

## T32：Cloud 同步共享与恢复管理

T32 在现有 `ipc_invoke` 上增加 typed `inspect_sync`（`ExplicitRouteArgs`，`deny_unknown_fields`）与 typed `list_shares`（同样的 `ExplicitRouteArgs` fail-closed 规则）。额外 `path`/`root`/`token`/`host` 为 schema。缺路由为 schema。非 fixture 路由为 policy，且不打开库。capabilities 精确允许列为 37 个命令。typed `inspect_sync` / `list_shares` 允许。`restore_sync` 不在允许列。

Cloud 同步 / 共享保持 FAIL-CLOSED。生产默认 `sync_enabled=false`、`sharing_enabled=false`、`remote_restore=false`、`last_sync=none`。宣称 synced/shared 而没有 live official cloud session 为 unsupported。生产共享目录为空。宣称 live shared remote 为 unsupported。测试注入 `{temp}/bmdock-t32-*`，默认仍报告 `sync_enabled=false`。BMDock 自有夹具 `sync-claimed` 标记仍不是已同步，分类为 unsupported。`share-claimed` 不是已共享。未存储密钥，未读取用户环境 token，未联系远程主机。未把 `just contract` 当作 T32 证明。不启动 Supervisor。无 rmcp。

AC14：`inspect_sync` / `list_shares` 每次携带 `ExplicitRouteArgs`。非 fixture 为 policy。额外 path/root/token/host 为 schema。缺路由为 schema。

AC42：无隐式 cloud/sync 路由。本地离线。release（21）与 main-preview（27）保持隔离，不把 profile 混进 sync DTO。

AC43：Cloud 同步 / 共享 FAIL-CLOSED。生产 `sync_enabled=false`、`sharing_enabled=false`、`remote_restore=false`、`last_sync=none`。宣称 synced/shared 而没有 live official cloud session 为 unsupported。不联系远程主机、不存储密钥、不读取环境 token。

AC52：恢复仍走 T12 `restore_fixture`，不是 cloud restore。信封 `"synced"` / `"restored"` 不是磁盘证明。夹具本地恢复测试仍可观察 T12 `restore_fixture` 把生成 Markdown 写到自有目标。未发明 cloud restore 磁盘成功。

zh-CN 工作台「同步 / 共享」区分空态 / 错误 / 就绪，显示未同步 / 未共享，与维护分区的 restore_fixture 分开。无 `dangerouslySetInnerHTML`。不启动 Supervisor。无新 npm 依赖。

`just build` 仍为 G0 探针；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。未运行 `just contract` 作为 T32 证明。release（`c0bd87c6`，21 tools）与 main-preview（`3452c821`，27 tools）未混合。

本机 Windows 本轮命令见 [t32-cloud-sync-recovery.json](../execution/evidence/t32-cloud-sync-recovery.json)。`python -m scripts.tasks unit` 以 G0 未 passed 且 T05+ 已 completed 失败（未回退）。未把 UI 文案、工具清单、编译 exe 或 `just contract` 当作 native GUI / 用户 vault / 官方 cloud sync / hosted CI 证据；这些仍为 `UNVERIFIED`。

T32 证据与验收映射见 [t32-cloud-sync-recovery.json](../execution/evidence/t32-cloud-sync-recovery.json)。`execution/status.json` 仅将 T32 标为 `completed`；未改 T05–T31/G0。
