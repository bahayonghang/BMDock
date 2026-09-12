# tests

> [仓库根](../CLAUDE.md) › `tests`
> 上次扫描：2026-09-09 13:16 +08:00

宿主 Python 单测，无第三方包、无真实引擎、无 Rust 编译。入口：`python -m unittest discover -s tests -v`（`just ci-unit` / `scripts.tasks.unit`）。

当前约 60 项。本目录 **不能** 证明 MCP 互操作或 G0。`test_repository_phase_order` 在 T06/T07 已 `completed` 且 G0 未 `passed` 时会失败；不要为了让 `python -m scripts.tasks unit` 变绿而回退那些任务状态。

## 文件

| 文件 | 覆盖 |
|---|---|
| `test_core.py` | profile、sandbox、分页、结果分类、命令/子进程、`just gate` 非零 |
| `test_cli_inventory.py` | `collect_cli_commands`：嵌套组、循环、缺失子命令、跨分支别名 |

## `test_core.py` 用例组

- **ProfileTests**：两 profile commit 不同；release 版本 `0.23.2`、21 工具；main-preview 27 工具；非法名字（路径穿越、`latest`、shell 拼接）抛 `ValueError`；重复发现名失败；delta 保留 missing/added；fingerprint 与键序无关、与语义有关。
- **SandboxTests**：每次 `create_sandbox` 新目录；仅 fixture；伪造 marker / 外部 vault / 额外 project / cloud mode / `database_url` / `auto_update` / symlink 均拒绝；`isolated_env` 丢弃 `OPENAI_API_KEY`、`LOGFIRE_TOKEN`、`BASIC_MEMORY_FORCE_CLOUD`、`PYTHONPATH`；原子 JSON 含中文；`redact` 替换 sandbox 前缀。symlink 用例用 `Path.is_symlink` patch，**不是** Windows junction 验收。
- **PaginationTests**：跨页、空末页、重复 cursor、非字符串 cursor、超页截断、错误 shape、RPC 异常不得当成空页。
- **ResultTests**：文本成功 ≠ 已保存；`isError` 优先；`created`/`updated` → `accepted_unverified`；FastMCP `structuredContent.result.action` 同样不是磁盘核实；拒绝 kind；未知 discriminator；只转发 schema 已声明参数；新 required 字段失败关闭。
- **InteropTests**：握手必须协商 `2025-11-25`；inputSchema 指纹按工具隔离；`search`/`fetch` 身份与 required 字段不可别名；控制面 `policy`/`schema`/`rpc_or_transport` 不得互相折叠；互换身份必须是 MCP `isError` 而不是本地拒绝；恢复边界保持 `UNVERIFIED`。
- **ObservationTests**：`wait_note` 只接受唯一 Markdown 命中；重复 sentinel 失败；超时文案声明不重试；未知 frontmatter/中文/wiki-link 保真；并发 worker 必须落到不同笔记、分类保持 `accepted_unverified`，且不得用缺失 path 或 envelope 冒充落盘。
- **CommandTests**：缺 cargo 失败；`run()` 不传 `shell` 且 `check=True`；`tasks.main(["gate"])` 返回 2；`check_source()` 在 T05+ 于 G0 前 completed 时失败。

## `test_cli_inventory.py`

用与外部 `click.Group` 无关的假 Group/Leaf，模拟 Typer 内置 Click。断言 `bm/cloud/login` 子树不被丢掉。

## 运行注意

- 工作目录为仓库根。
- `test_product_gate_not_pretended_passed` 依赖当前 `execution/status.json` 中 G0 未 `passed`。G0 真通过后该断言需要同步修改。
- 新增真实引擎路径测试应放 `scripts/probe.py` 的 contract suite，不放本目录 mock。

## 关联模块

- 被测代码：[scripts](../scripts/CLAUDE.md)
- 门禁状态：[execution](../execution/CLAUDE.md)
