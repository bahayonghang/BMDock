# T20 局部图谱与渐进展开 — 执行记录

本文件是 T20 的执行记录。未实现 T21–T40，未改 T05–T19/G0，未 git commit。

## 交付

- typed `expand_graph`（`ExplicitRouteArgs` + `identifier` + optional `cursor` / `page_size`，`deny_unknown_fields`）：`ipc.rs` / `ipc.ts`
- BMDock 自有一跳邻域：复用 `FixtureLibrary` wiki-link `[[...]]` 抽取，不是第二套索引，也不是官方引擎图谱 MCP，也未引入 vis.js
- 渐进展开：每次调用 `depth=1`；`next_cursor` 分页当前标识的邻居；展开返回节点则读取该邻接文件的 wiki-link
- `page_size` 默认 20、最大 64；0/过大、非法/重复 cursor 为 schema；截断库存为 unsupported
- 生产 `EmptyLibrary` 返回空节点/边与 `classified_as: empty`，不是用户 vault 成功
- zh-CN 工作台「图谱」面板空态 / 错误 / 就绪，加载更多 / 展开使用 `next_cursor`
- capabilities 精确允许列为 20 个命令
- 证据：`execution/evidence/t20-local-graph-expansion.json`

## 验收映射

- AC27：图谱 DTO 上保留 `observation.classified_as`（`disk_verified` / `accepted_unverified` / `conflict` / `empty`），与节点/边分开。夹具展开核对物理文件后为 `disk_verified`。空库为 empty。冲突仍是 CRUD 分类，不是图谱节点分类。
- AC29：夹具物理 wiki-link 中的中文标识（如 `欢迎`）作为 permalink 出现，展开不丢 CJK。这是夹具局部图谱身份，不是 T24 召回基准，也不是官方 search MCP。
- AC56：有界宿主展开 + 截断 fail-closed。native GUI、WebView2 会话、安装器、hosted CI 仍为 `UNVERIFIED`。cargo test / npm build / UI 文案不是 AC56 native 证明。记录为 bounded-host-expansion。

## 未做

- 未运行 `just contract` / `just contract-main` 作为 T20 证明
- 未启动 Supervisor，未加入 rmcp，未实现 T21 检索或 T24 基准
- 未改 `.work/engines`、用户 vault、父任务目录、T05–T19/G0 状态
- 未 git commit / push / merge / amend

## 验证

- `python ./.trellis/scripts/task.py validate 09-12-t20-local-graph-expansion`：exit 0
- `cargo fmt --all -- --check`：exit 0
- `cargo test --workspace --locked --offline`：exit 0（bmdock-app 134，bmdock-probe 5）
- `cargo check --workspace --locked --offline`：exit 0（既有 T07 dead_code 警告）
- `npm run build`（`apps/bmdock-desktop`）：exit 0
- `git diff --check`：exit 0
- `python -m unittest tests.test_desktop_shell -v`：exit 0（14 passed）
- `python -m scripts.tasks unit`：exit 1（G0 vs T05+，未回退）

## UNVERIFIED

官方引擎图谱 MCP、native GUI / 窗口冻结、WebView2 会话、安装器、hosted CI、真实用户 vault。
