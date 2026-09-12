# T09 实施记录：无副作用预检与配置发现

## 前置门禁

1. 确认 T06 typed IPC 与 T07 Supervisor 已存在；不回退 T05–T08，不把 G0 标为 passed。
2. 运行 `python ./.trellis/scripts/task.py validate 09-12-t09-preflight-config-discovery`。
3. 只改 T09 范围内的 typed IPC、只读预检/发现、中文预检表面、证据与文档；不修改 T10–T40、父任务目录、`.work/engines` 或真实 vault。

## 有序步骤

1. 在现有 `ipc_invoke` union 增加只读命令 `run_preflight` 与 `discover_config`，`args` 为空且 `deny_unknown_fields`。不 start/stop Supervisor，不拉起官方引擎，不写文件，不改全局 Basic Memory 配置，不打开真实 vault，不暴露 raw `callTool`。
2. 预检报告两条隔离的 profile 记录（release `c0bd87c6…` / 21 工具 vs main-preview `3452c821…` / 27 工具）以及不启动引擎的宿主检查。额外 path 字段 schema 拒绝；任意用户路径/真实 vault 为 policy，且不打开。
3. 配置发现默认根不是用户 Basic Memory 主目录；默认空结果表示未找到 BMDock 自有候选，而不是已成功读取生产 `config.json`。不复制或改写生产配置。
4. IPC 错误联合保持 `policy` / `schema` / `unsupported`。`timeout_unknown` / `transport` / `process` 只留在 `RuntimeStateDto`。
5. 在 zh-CN 导航增加「预检」分区，区分空态 / 错误 / 就绪。不写笔记，不把 UI 文案当作 native GUI 证明。
6. 更新 `typed-ipc-policy.md`、capabilities 精确允许列表，以及原先断言恰好 3 个命令的测试。
7. 不切换 `just build`；`just contract*` 仍为探针；`just dev` 保持 T08 的 Tauri 入口。
8. 记录 `python -m scripts.tasks unit` 因 G0 未通过而失败；用 `python -m unittest discover -s tests -v`（预期 `test_repository_phase_order` ERROR）。不把 `just contract` 当作 T09 证明。

## 最小验证

- `python ./.trellis/scripts/task.py validate 09-12-t09-preflight-config-discovery`
- `cargo fmt --all -- --check`
- `cargo test --workspace --locked --offline`
- `cargo check --workspace --locked --offline`
- `npm run build`（`apps/bmdock-desktop`；本轮不宣称 `npm ci` 除非实际执行）
- `git diff --check`
- `python -m unittest tests.test_desktop_shell -v`
- 记录 `python -m scripts.tasks unit` 失败；不宣称通过

## 风险与回滚

- 高风险：把预检做成 Supervisor start 或扫描用户主目录。回滚时删除两个新 IPC 命令与预检导航，恢复 3 命令允许列表。
- 不把 T05–T08 改回 planned 来让 `python -m scripts.tasks unit` 变绿。
- 不混合 release / main-preview 工具基线。

## 完成门槛

只读预检与发现、双 profile 隔离、空发现 ≠ 用户 vault、精确 IPC 允许列表、AC03/AC04/AC19/AC38 证据齐全后，才把 `execution/status.json` 的 T09 标为 `completed`。不改 T05–T08/G0。

## 本轮关闭记录（2026-09-12）

实现了只读 `run_preflight` / `discover_config`、双 profile 隔离预检、默认空配置发现、zh-CN「预检」分区，以及 capabilities 5 命令允许列表。未 start/stop Supervisor，未拉起官方引擎，未写 vault，未改 `just build`，未跑 `just contract`，未跑 native GUI，未执行 `npm ci`。未回退 T05–T08，未改 G0。

本轮命令与退出码：

- `python ./.trellis/scripts/task.py validate 09-12-t09-preflight-config-discovery` → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 26 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 dead_code 警告）
- `npm run build`（`apps/bmdock-desktop`，已有 `node_modules`） → 0
- `git diff --check` → 0
- `python -m unittest tests.test_desktop_shell -v` → 0（6 passed）
- `python -m unittest discover -s tests -v` → 1（66 tests：65 ok，1 ERROR `test_repository_phase_order`）
- `python -m scripts.tasks unit` → 1（`A later task was completed before G0`；不宣称通过）
- `npm ci` / `just contract*` / native GUI → 未运行

`execution/status.json` 仅将 T09 标为 `completed`。

## 检查轮（2026-09-12）

修正了已连接/已停止快照上 `host.engine_spawned` 被硬编码为 false 的问题：该字段改为从生命周期投影，`files_written` 仍为 false。预检 UI 展示这两个快照字段。顶层额外 `path` 也按 schema 拒绝。未 start/stop，未改 T05–T08/G0，未跑 `just contract`。

检查轮退出码：validate 0；`cargo fmt --all -- --check` 0；`cargo test --workspace --locked --offline` 0（bmdock-app 28 + bmdock-probe 5）；`cargo check --workspace --locked --offline` 0（既有 T07 dead_code 警告）；`npm run build` 0；`git diff --check` 0；`python -m unittest tests.test_desktop_shell -v` 0（6 passed）；`python -m unittest discover -s tests -v` 1（66 tests：65 ok，1 ERROR `test_repository_phase_order`）；`python -m scripts.tasks unit` 1（G0 阶段顺序，不宣称通过）。

