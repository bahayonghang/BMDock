# T10 实施记录：项目、工作区与显式路由

## 前置门禁

1. 确认 T09 只读预检、T08 中文壳、T06 typed IPC 已存在；不回退 T05–T09，不把 G0 标为 passed。
2. 运行 `python ./.trellis/scripts/task.py validate 09-12-t10-project-workspace-routing`。
3. 只改 T10 范围内的 typed IPC、BMDock 自有项目/工作区目录、显式路由 DTO、中文项目表面、证据与文档；不修改 T11–T40、父任务目录、`.work/engines` 或真实 vault。

## 有序步骤

1. 在现有 `ipc_invoke` union 增加只读 `list_projects`（空 `args`，`deny_unknown_fields`）。默认目录只返回 BMDock 自有工作区 id 与 `bmdock-fixture`，不扫描、不打开用户 Obsidian vault 或全局 Basic Memory 主目录。
2. 保持 `select_project` 仅接受 `bmdock-fixture`。非 fixture 为 policy。额外 path 字段 schema 拒绝。不暴露 raw `callTool`，不 start/stop Supervisor，不实现 T11 读取或 T15 CRUD。
3. 路由 DTO 要求后续读写命令携带显式 `workspace` + `project`；选择状态只用于投影，禁止隐式 current-project 写入。不发明跨项目检索能力。
4. IPC 错误联合保持 `policy` / `schema` / `unsupported`。双 profile 保持隔离（release `c0bd87c6` / 21 vs main-preview `3452c821` / 27）。本地离线（AC38）。
5. renderer 增加 zh-CN「项目」分区：空态（未选择 / 未找到）、错误（policy/schema）、就绪（已选择 fixture）。不写文件。
6. 更新 `typed-ipc-policy.md`、capabilities 精确允许列表，以及原先断言恰好 5 个命令的测试。
7. 不切换 `just build`；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。不把 `just contract` 当作 T10 证明。
8. 记录 `python -m scripts.tasks unit` 因 G0 未通过而失败；用 `python -m unittest discover -s tests -v`（预期 `test_repository_phase_order` ERROR）。不回退 T05–T09。

## 最小验证

- `python ./.trellis/scripts/task.py validate 09-12-t10-project-workspace-routing`
- `cargo fmt --all -- --check`
- `cargo test --workspace --locked --offline`
- `cargo check --workspace --locked --offline`
- `npm run build`（`apps/bmdock-desktop`；本轮不宣称 `npm ci` 除非实际执行）
- `git diff --check`
- `python -m unittest tests.test_desktop_shell -v`
- 记录 `python -m scripts.tasks unit` 失败；不宣称通过

## 风险与回滚

- 高风险：把目录做成扫描用户主目录、把 `select_project` 变成隐式 current-project 写入，或提前实现笔记读写。回滚时删除 `list_projects` 与「项目」导航，恢复 5 命令允许列表。
- 不把 T05–T09 改回 planned 来让 `python -m scripts.tasks unit` 变绿。
- 不混合 release / main-preview 工具基线。

## 完成门槛

显式 fixture 路由、BMDock 自有目录、禁止隐式写入与跨项目检索、精确 IPC 允许列表、AC06/AC14/AC25/AC38 证据齐全后，才把 `execution/status.json` 的 T10 标为 `completed`。不改 T05–T09/G0。

## 本轮关闭记录（2026-09-12）

实现了只读 `list_projects`、fixture-only `select_project` 的 RouteState 投影、`ExplicitRouteArgs`、zh-CN「项目」分区（未选择 / 未找到 / 错误 / 已选择 fixture），以及 capabilities 6 命令允许列表。未 start/stop Supervisor，未拉起官方引擎，未写 vault，未实现笔记读取或 CRUD，未改 `just build`，未跑 `just contract`，未跑 native GUI，未执行 `npm ci`。未回退 T05–T09，未改 G0。

本轮命令与退出码：

- `python ./.trellis/scripts/task.py validate 09-12-t10-project-workspace-routing` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 35 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 dead_code 警告）
- `npm run build`（`apps/bmdock-desktop`，已有 `node_modules`） → 0
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（6 passed）
- `python -m unittest discover -s tests -v` → 1（66 tests：65 ok，1 ERROR `test_repository_phase_order`）
- `python -m scripts.tasks unit` → 1（`A later task was completed before G0`；不宣称通过）
- `npm ci` / `just contract*` / native GUI → 未运行

`execution/status.json` 仅将 T10 标为 `completed`。
