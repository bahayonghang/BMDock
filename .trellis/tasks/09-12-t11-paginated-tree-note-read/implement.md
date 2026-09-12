# T11 实施记录：分页文件树与笔记读取

## 目标

把 P2 T11 做成可独立验收的产品能力：typed `list_tree` / `read_note`（每次携带 `ExplicitRouteArgs`）、失败即关闭的分页、磁盘观察分类，以及 zh-CN 工作台纯文本预览。不引入 rmcp/live MCP，不写笔记，不扫描用户 vault。

## 已完成

1. 新增 `apps/bmdock-desktop/src-tauri/src/library.rs`：`NoteLibrary` trait、生产默认 `EmptyLibrary`、测试专用 `FixtureLibrary`。分页对齐 `scripts/core.py paginate()`（有界 `page_size`、`next_cursor`、非法/循环 cursor、截断失败关闭）。
2. `ipc_invoke` 允许列变为 8：`list_tree` 与 `read_note` 消费 `ExplicitRouteArgs`。额外 path 为 schema；非 fixture / 文件系统 identifier 为 policy 且不打开路径。未知 `call_tool` 仍失败。
3. `read_note` 返回 title / identifier / markdown body / observation。`envelope_is_not_disk_proof=true`。夹具测试写入中文 + wiki-link Markdown，并断言正文等于物理文件。
4. 工作台展示分页树与 `<pre>` 预览；加载更多跟随 `next_cursor`；每次 `copyFixtureRoute()`。无 `dangerouslySetInnerHTML`。预检 / 项目 / 运行状态 / 说明保留。
5. 更新 `typed-ipc-policy.md`、`ipc.ts` 联合、`DesktopShellTests`、`execution/status.json`（仅 T11 planned→completed）和证据。

## 本轮命令退出

| 命令 | 退出码 |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t11-paginated-tree-note-read` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 48 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code 警告） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（6 passed） |
| `python -m unittest discover -s tests -v` | 1（65 ok，1 ERROR `test_repository_phase_order`） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+ completed；未回退） |
| `just contract` / `just contract-main` | 未运行 |

## UNVERIFIED

- 官方引擎 `list_directory` / `read_note` MCP（未加入 rmcp，未跑 `just contract`）
- native GUI / WebView2 / Job Object / 真实 vault / hosted CI

## 未做

- 未 git commit / push / merge / amend
- 未切换 `just build`
- 未改 T05–T10 / G0、父任务目录、T12–T40、`.work/engines`、用户 vault 或密钥
- 未实现写入 / 编辑 / 移动 / 删除（T14/T15）

## 回滚点

删除 `library.rs`、从 `IpcCommand` 去掉 `list_tree`/`read_note`、恢复 6 命令允许列和工作台空态。不要为了让 `python -m scripts.tasks unit` 变绿而回退 T05–T10 或 G0。
