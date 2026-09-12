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
