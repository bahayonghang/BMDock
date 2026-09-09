# bmdock-probe

> [仓库根](../../CLAUDE.md) › `crates/bmdock-probe`
> 上次扫描：2026-09-09 13:16 +08:00

Cargo workspace 唯一成员。开发者 G0 探针：官方 rmcp 处理 MCP 握手与 framing；本 crate 持有子进程、白名单控制面、关闭证据。`publish = false`。

**禁止**把本控制通道暴露为桌面 IPC。

## 入口

| 项 | 值 |
|---|---|
| 二进制 | `bmdock-probe` → `target/debug|release/bmdock-probe[.exe]` |
| 源码 | `src/main.rs`（单文件，约 209 行，含 4 个单元测试） |
| 用法 | `bmdock-probe <managed-python> <engine-worker.py> <owned-sandbox>` |
| 版本 | `--version` 打印 `bmdock-probe {CARGO_PKG_VERSION}` |

argv 必须正好 3 个路径参数。Python 与 worker 必须为已存在的绝对路径。sandbox 必须能 `canonicalize`，且含 `.bmdock-g0-sandbox.json`，`kind == "bmdock-g0"`。`BASIC_MEMORY_CONFIG_DIR` 必须解析到 `sandbox/config`。

启动子进程：

```text
<managed-python> <engine-worker.py> serve
cwd = sandbox
stdin/stdout piped；stderr inherit；kill_on_drop = true
```

握手超时 120s。控制面每行一条 JSON；行长上限 `MAX_CONTROL_LINE = 1_048_576`。RPC 超时 90s；SDK cancel 与 `child.wait` 各 30s。超时未知结果：发出 `kind: timeout_unknown`，**停止本会话**，不重试。

关闭事件：`{event: shutdown, sdkClosed, process: {forced, exitCode}, materialization: "not_proven_by_process_exit"}`。`sdkClosed` 为假、`forced` 为真或 `exitCode != 0` 时进程以错误退出。

## 控制面协议

stdin JSON 请求：`{id, method, params}`。stdout JSON 行：`{event: connected, server, childPid}`、`{id, result}`、`{id, error: {kind, message}}`、shutdown 事件。

`error.kind`：`policy` | `rpc_or_transport` | `timeout_unknown` | `schema`。

允许的 `method`：

- `tools/list`
- `resources/list`
- `resources/templates/list`
- `prompts/list`
- `prompts/get`
- `resources/read`
- `tools/call`

`tools/call` 允许的 `params.name`：

- `list_memory_projects`
- `read_note` / `read_content` / `search_notes` / `search` / `fetch`
- `write_note` / `edit_note`（`arguments.project` 必须为 `"bmdock-fixture"`）
- `__bmdock_missing_tool__`（负向测试）

其余 method/tool 返回 policy 错误。发现接口覆盖完整 registry；**调用**白名单小于 `compatibility/profiles.json` 的静态 21/27 工具基线。

## 依赖

见 `Cargo.toml`：

- `rmcp = "=3.2.0"`，`default-features = false`，`client` + `transport-async-rw`
- `serde` / `serde_json`
- `tokio`：`macros`、`rt-multi-thread`、`process`、`io-std`、`io-util`、`time`

工作区：`../../Cargo.toml`（resolver 2，edition 2021，license AGPL-3.0-or-later）。`.cargo/config.toml` 设置 `incompatible-rust-versions = "fallback"`。构建一律 `--locked`。

## 环境契约

探针启动前校验：

| 变量 | 要求 |
|---|---|
| `BASIC_MEMORY_CONFIG_DIR` | 等于 `sandbox/config` |
| `BASIC_MEMORY_AUTO_UPDATE` | `false` |
| `BASIC_MEMORY_SEMANTIC_SEARCH_ENABLED` | `false` |
| `BASIC_MEMORY_FORCE_LOCAL` | `true` |

ClientInfo：`protocolVersion = "2025-11-25"`，`name = "BMDock-G0"`。

## 测试

`#[cfg(test)]` 在 `src/main.rs`：未知 method、未知 tool、fixture 写入必须带 `bmdock-fixture`、发现方法放行。由 `just ci` 的 `cargo test --workspace --locked` 运行。4 项。

## 关键文件

| 文件 | 说明 |
|---|---|
| `src/main.rs` | 全部实现与单测 |
| `Cargo.toml` | 包元数据与钉扎依赖 |
| `../../rust-toolchain.toml` | channel 1.90.0 |
| `../../scripts/engine_worker.py` | 子进程实际入口 |
| `../../scripts/probe.py` | 控制面客户端 |

## 约束

- 不经前端转发 stdio。
- 进程退出不等于写入落盘；物化由 Python smoke 在 fixture 上观察。
- rmcp `TokioChildProcess` 默认约 3s 后 kill；本探针不用该默认，独立 Child + SDK transport。
- T05 桌面 IPC 需要新 crate / 新协议，不复用本白名单控制面。

## 关联模块

- 上游调用方：[scripts](../../scripts/CLAUDE.md)
- sandbox 策略：[scripts/core.py](../../scripts/core.py)
- 引擎基线：[compatibility](../../compatibility/CLAUDE.md)
