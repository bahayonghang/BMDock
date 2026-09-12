# scripts

> [仓库根](../CLAUDE.md) › `scripts`
> 上次扫描：2026-09-09 13:16 +08:00

无第三方 Python 依赖的开发验证工具。通过 `python -m scripts.tasks` 或 `justfile` 调用。不替代官方 Basic Memory 引擎。

## 文件职责

| 文件 | 职责 |
|---|---|
| `__init__.py` | 包标记 |
| `tasks.py` | 命令分发：doctor / setup / lock / unit / ci / dev / contract / build / gate |
| `core.py` | sandbox、隔离环境、profile 读取、分页、结果分类、原子 JSON |
| `probe.py` | 真实进程 smoke：Probe 控制面客户端、`run_contract` |
| `engine_worker.py` | **仅**由隔离引擎 Python 执行：`inventory` 或 `serve` |

## 入口与接口

### `tasks.main(argv)`

`argparse` 子命令。`dev` / `contract` 接受 profile：`release`（默认）或 `main-preview`。

| 命令 | 行为 |
|---|---|
| `doctor` | 检查 Python≥3.12 与 git/uv/cargo/rustc/just；不打开用户配置 |
| `setup` | `bootstrap_engines`：浅克隆固定 commit，release 再取 `v{version}` tag 并核对 SHA；`uv sync --frozen --python 3.12.12` |
| `lock` | `cargo generate-lockfile` |
| `unit` | `check_source` + `unittest discover -s tests` |
| `ci` | unit + fmt `--check` + clippy `-D warnings --locked` + `cargo test/build --locked` + 两 profile `run_contract` |
| `dev` / `contract` | `cargo build -p bmdock-probe --locked` 后 `run_contract(profile)` |
| `build` | `cargo build --release -p bmdock-probe --locked` |
| `gate` | 读 `execution/status.json`；任一 G0–G7 非 `passed` 返回 2 |

`check_source`：解析全部 `scripts/*.py`；profile 工具名去重；G0 未通过时拒绝 T05+ 标 `completed`。

`bootstrap_engines` 目标目录 `.work/engines/<name>`。已存在但缺少 `.bmdock-engine.json` 或 commit 不符则拒绝覆盖。

### `core.py` 公共函数

- `ROOT`：仓库根。`PROJECT = "bmdock-fixture"`。`MARKER = ".bmdock-g0-sandbox.json"`。
- `profiles()` / `profile(name)`：读 `compatibility/profiles.json`；commit 必须为 40 位小写 hex。
- `create_sandbox(base, profile_id)`：在 `base` 下 `mkdtemp`，创建 `config/vault/home/tmp/cache`，写入最小 `config.json`（仅 fixture、local、sqlite、关闭 auto_update 与 embeddings）。
- `verify_sandbox`：标记、root、无 symlink、路径不逃逸、仅 fixture、无额外 config 键。
- `isolated_env`：只保留 `SYSTEM_ENV` 白名单，写入 BASIC_MEMORY_* 与离线/UTF-8 标志；丢弃 provider key 与 `PYTHONPATH`。
- `run(argv, ...)`：`subprocess.run(..., check=True)`，无 shell。
- `paginate`：跟随 `nextCursor`；重复 cursor、非字符串 cursor、超 `max_pages` 失败。
- `inventory_delta`：重复名失败；返回 `missing` / `added`。
- `classify_tool_result`：`tool_error` > `rejected` > `accepted_unverified` > `unclassified`。文本 “Saved successfully” 不得升格为已落盘。
- `write_json`：同目录临时文件 + `fsync` + `os.replace`；仅用于 harness 自有文件。

### `probe.Probe`

启动 `bmdock-probe` debug 二进制，线程读 stdout JSON 行。`request` 遇 `error` 抛错；`raw` 保留错误供负向测试。`close` 关 stdin，校验 shutdown 事件与 exit 0。`abort` 先给清理预算再 kill。

`run_contract(profile_id)` 流程：校验引擎 HEAD == profile commit → 新 sandbox → worker `inventory` → Probe 握手并断言协商 `protocolVersion` 为 `2025-11-25` → 分页发现 tools/resources/templates/prompts → 与静态 `expected_tools` 比 delta（release 21 / main-preview 27，报告不合并）→ `search`/`fetch` 身份与 inputSchema 指纹 → 读取首个 resource 与 prompt → 区分 `policy` / `schema` / `rpc_or_transport` / 未知工具 `isError` / 互换身份 `isError` → `list_memory_projects` → `write_note` 先标 `accepted_unverified`（中文、自定义 metadata、wiki-link）→ 磁盘 sentinel → `read_note` → 单次 `edit_note` append → 正常关闭后再读文件 → 并发 fixture 写入（T03）与 UNVERIFIED 恢复边界记录。报告写到 `artifacts/<profile>.contract.json`，路径经 `redact` 替换为 `<G0_SANDBOX>`。`gate_status` 保持 `not_passed`。UI 文案不是落盘证据。

核心 checks：`effective_isolation`、`handshake`、`mcp_discovery`、`registry_names`、`search_fetch_identity`、`tool_input_schemas`、`resource_prompt_roundtrip`、`error_categories`、`unknown_tool_error`、`search_fetch_swapped_calls`、`create_read_materialize`、`append_once`、`shutdown_file_observation`。丢响应、取消后接受、`timeout_unknown` 现场注入、强杀、磁盘故障保持 `UNVERIFIED`。

### `engine_worker.py`

运行时必须存在父目录 `.bmdock-g0-sandbox.json`，否则退出。改写 `sys.argv` 为 `basic-memory mcp --transport stdio`，避免环境参数污染官方 CLI。

- `serve`：`from basic_memory.cli.main import app; app()`。
- `inventory`：走 Typer `get_command` + 命令自身 `context_class` / `list_commands` / `get_command`（**不用** `isinstance(..., click.Group)`，Typer 内置 Click 与外部 Click 类身份不同）。要求根命令包含 `mcp`、`project`、`cloud`、`import`。写出 `inventory.json`（version、CLI 树、OpenAPI）。循环、重复名、缺失子命令、深度/数量上限失败。清单存在 ≠ 命令已成功执行。

## 依赖

- 标准库 only（tasks/core/probe）。
- `engine_worker` 依赖隔离 venv 中的 `basic-memory`、`typer`、`click` 等，由 `just setup` 安装。
- 调用 `crates/bmdock-probe` 与 `compatibility/profiles.json`、`execution/status.json`。

## 测试

见 [tests](../tests/CLAUDE.md)。本目录无独立 test 文件。`engine_worker.collect_cli_commands` 的回归在 `tests/test_cli_inventory.py`。

## 约束

- 缺工具不得跳过：`require()` 找不到即失败。
- `just ci-unit` 不能替代 `just ci` 或 G0。
- 不复制含生产路径的用户配置。
- `engine_python` 保留 venv 的 Scripts/bin 路径，不 `resolve()` 以免丢掉 `pyvenv.cfg`。
- 未知 `structuredContent.kind` 记 `unclassified`，不当地成功。

## 关联模块

- 探针：[bmdock-probe](../crates/bmdock-probe/CLAUDE.md)
- 基线：[compatibility](../compatibility/CLAUDE.md)
- 状态：[execution](../execution/CLAUDE.md)
