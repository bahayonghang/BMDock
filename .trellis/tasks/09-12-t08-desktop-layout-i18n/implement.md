# T08 实施记录：桌面布局、导航与 i18n

## 前置门禁

1. 确认 T05 骨架与 T06 typed IPC 已存在；不回退 T06/T07，不把 G0 标为 passed。
2. 运行 `python ./.trellis/scripts/task.py validate 09-12-t08-desktop-layout-i18n`。
3. 只改 T08 范围内的 renderer 壳、justfile `dev` 入口、证据与文档；不修改 T09–T40、父任务目录、`.work/engines` 或真实 vault。

## 有序步骤

1. 增加本地 zh-CN 文案表，不引入 i18n npm 依赖或重型 UI 库。
2. 用 landmark（banner/nav/main）实现中文分区导航：工作台 / 运行状态 / 说明。
3. 区分空态与错误态。只读调用 `get_capabilities` / `get_runtime_state`；调用失败为错误态，默认运行状态为 `not_started` 空态。不 start/stop Supervisor，不 `callTool`，不选择真实项目，不写 vault。
4. 无障碍基线：`html lang="zh-CN"`、带 `aria-current` 的导航、可键盘聚焦、可见 `:focus-visible`、状态与错误不只靠颜色。
5. 将 `just dev` 切到现有 `just tauri-dev`。**不切换 `just build`**：`.github/workflows/ci.yml` 仍用 `just build` 编译 G0 探针。保留 `just tauri-build`、`just contract`、`just contract-main`。
6. 记录 AC21 / AC57 / AC60 证据。不以 UI 文案、工具清单、编译 exe 或 `just contract` 作为 native GUI / WebView2 / Job Object / 真实 vault / hosted CI 证明。

## justfile 语义（本任务关闭后）

| 命令 | 语义 |
|---|---|
| `just dev` | 桌面壳：调用 `just tauri-dev` |
| `just tauri-dev` | 既有 Tauri 开发入口（保留） |
| `just tauri-build` | 既有 Tauri 构建入口（保留；不是安装器） |
| `just build` | **仍为** `scripts.tasks build`（G0 探针）。CI 使用该入口，故 T08 不把它切到 Tauri。这是记录在案的限制，不是静默偏离。 |
| `just contract` / `just contract-main` | 仍为真实引擎探针 smoke |
| `just dev-main` | 仍为 main-preview 探针交互入口 |

父设计曾写 T05–T08 后 `just build` 也可切到桌面入口。本任务明确不切：CI 的 `just build` 仍必须编译 G0 探针。

## 最小验证

- `python ./.trellis/scripts/task.py validate 09-12-t08-desktop-layout-i18n`
- `npm run build`（`apps/bmdock-desktop`；仅在 `node_modules` 缺失时才 `npm ci`）
- `cargo fmt --all -- --check`
- `cargo test --workspace --locked --offline`
- `cargo check --workspace --locked --offline`
- `git diff --check`
- `python -m unittest discover -s tests -v`（预期 `test_repository_phase_order` ERROR）
- 记录 `python -m scripts.tasks unit` 因 `A later task was completed before G0` 失败；不宣称它通过
- 不运行 `just contract`，不跑 native GUI

## 风险与回滚

- 高风险：`justfile` 的 `dev` 入口。若桌面壳无法启动，将 `just dev` 恢复为 `scripts.tasks main(["dev", "release"])`，并保持 `just tauri-dev` / `just contract*` / `just build`。
- 不把 T06/T07 改回 planned 来让 `python -m scripts.tasks unit` 变绿。
- 不混合 release / main-preview 工具基线。

## 完成门槛

中文布局导航、空态/错误态、无障碍基线、`just dev`→Tauri、`just build` 探针例外有文档、三项 AC 有证据后，才把 `execution/status.json` 的 T08 标为 `completed`。不把 G0、T05、T06、T07 一并改掉。

## 本轮关闭记录（2026-09-12）

实现了 zh-CN 本地文案表、中文分区导航、空态/错误态只读快照，以及无障碍基线。`just dev` 指向 `just tauri-dev`。`just build` 仍为 G0 探针，因为 CI 继续执行 `just build`；该例外写在本文件与 `execution/evidence/t08-desktop-layout-i18n.json`。未回退 T06/T07，未改 G0，未跑 `just contract`，未跑 native GUI，未执行 `npm ci`。

本轮命令与退出码：

- `python ./.trellis/scripts/task.py validate 09-12-t08-desktop-layout-i18n` → 0
- `npm run build`（`apps/bmdock-desktop`，已有 `node_modules`） → 0
- `cargo fmt --all -- --check` → 0
- `cargo test --workspace --locked --offline` → 0（bmdock-app 16 + bmdock-probe 5）
- `cargo check --workspace --locked --offline` → 0（既有 T07 dead_code 警告）
- `git diff --check` → 0
- `python -m unittest discover -s tests -v` → 1（65 tests：64 ok，1 ERROR `test_repository_phase_order`）
- `python -m scripts.tasks unit` → 1（`A later task was completed before G0`；不宣称通过）
- `npm ci` / `just contract*` / native GUI → 未运行

`execution/status.json` 仅将 T08 标为 `completed`。
