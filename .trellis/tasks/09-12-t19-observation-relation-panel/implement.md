# T19 实施记录：观察与关系语义面板

本文件是 T19 的执行记录。未实现 T20–T40，未改 T05–T18/G0，未 git commit。

## 交付

- typed `list_relations`（`ExplicitRouteArgs` + `identifier`，`deny_unknown_fields`）：`ipc.rs` / `ipc.ts`
- 关系来自 T15 `FixtureLibrary` 笔记正文 wiki-link `[[...]]`，不是第二套数据库，也不是官方引擎图谱 MCP
- 生产 `EmptyLibrary`：空关系列表（空态），不是用户 vault 成功
- 观察面板显示当前笔记 `classified_as`（`disk_verified` / `accepted_unverified` / `conflict` / `empty`），与关系列表分开
- zh-CN 工作台「观察 / 关系」空态 / 错误 / 就绪；无 `dangerouslySetInnerHTML`
- capabilities 精确允许列 18 → 19
- 证据：`execution/evidence/t19-observation-relation-panel.json`

## AC 映射

- AC27：观察面板展示当前笔记的 `classified_as`，与关系列表分开。同目标冲突来自既有 CRUD DTO，不是关系条目。
- AC28：关系列表是 permalink/identifier，不是文件系统路径。缺失目标是 empty/unsupported。官方 `recent_activity` / `build_context` 为 `UNVERIFIED`。未加入 rmcp，未运行 `just contract`。

## 验证命令与退出码

| 命令 | exit |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t19-observation-relation-panel` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 130 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（13 passed） |
| `python -m unittest discover -s tests -v` | 1（72 ok，1 ERROR phase order） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+，未回退） |
| `just contract*` | 未运行 |

## UNVERIFIED

官方 `recent_activity` / `build_context` MCP、官方引擎图谱、native GUI、真实用户 vault、hosted CI、T20 局部图谱。夹具 wiki-link 列表不是引擎图谱证明。
