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

**完整 G0 仍未通过。** MCP resources/prompts/API/CLI 注册清单不是全部功能验收。完整原文往返、真正并发编辑、取消/丢响应、磁盘故障、强杀与恢复、Windows junction/睡眠恢复仍待验证；没有 Tauri GUI 或原生桌面安装器。具体交接要求见 [G0_HANDOFF.md](G0_HANDOFF.md)。

本批全部操作使用自动生成的 fixture，没有连接用户真实 Obsidian vault、全局 Basic Memory 配置或 Agent 配置。环境过滤不等于 OS 网络沙箱；正常退出后的文件观察也不构成跨 Agent 原子写入保证。`just gate` 返回非零是当前完整产品门禁的真实状态，不能通过跳过测试来消除。

## T05：Tauri + React/TypeScript 桌面骨架

T05 建立了 `apps/bmdock-desktop/` 的静态 React/Vite renderer、Tauri 2 Rust 启动入口和真实依赖锁文件。`bmdock-app` 仅创建默认窗口，不注册 typed IPC、raw `callTool`、文件系统或笔记命令；P0 的 `just build`、`just contract` 和 `just contract-main` 入口保持独立。显式桌面命令为 `just tauri-dev` 与 `just tauri-build`。

T06 建立了单一 `ipc_invoke` typed 命令入口和 renderer 侧 `invokeTyped`/`listenTyped` DTO 边界。当前仅允许 `get_capabilities`、`get_runtime_state` 和 fixture-only 的 `select_project`；未知命令、任意路径字段、非 `bmdock-fixture` 项目和 raw `callTool` 形状均 fail closed。T06 不启动 Supervisor、不调用 MCP、不访问真实 vault 或文件系统；错误响应保留 `policy`、`schema`、`unsupported` 分类。证据见 [t06-typed-ipc-policy.json](../execution/evidence/t06-typed-ipc-policy.json)。

本机 Windows 验证：`npm ci --ignore-scripts`、`npm run build`、`cargo fmt --all -- --check`、`cargo test --workspace --locked --offline`、`cargo check --workspace --locked --offline` 和 `npm run tauri:build` 均通过；后者生成 `target/release/bmdock-app.exe`。依赖锁是 Cargo/npm 解析产物，未提交 `node_modules/`、`dist/`、Tauri `gen/` 或其他构建目录。

T05 证据与验收映射见 [t05-tauri-react-skeleton.json](../execution/evidence/t05-tauri-react-skeleton.json)。真实窗口交互、WebView2、安装器、签名、hosted CI、真实 vault 和产品 IPC 仍分别属于 T13、T36–T40 或后续任务；本任务不把静态构建当作这些证据。
