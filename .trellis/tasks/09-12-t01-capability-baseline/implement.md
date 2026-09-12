# T01 实施记录：固定版本并采集完整能力清单

## 已完成

1. `compatibility/profiles.json` 固定 `release`（v0.23.2 / `c0bd87c6…`）和 `main-preview`（`3452c821…`）的 immutable commit、仓库和 registry source；两套 `expected_tools` 保持独立（21 vs 27）。
2. `scripts/tasks.py setup` 校验 checkout HEAD、release tag 指向和干净工作树；未知 profile id 被拒绝。本任务未重跑 `just setup`。
3. `scripts/engine_worker.py inventory` 采集官方 CLI 注册树和 OpenAPI；`scripts/probe.py` 分页采集 MCP tools/resources/templates/prompts，并用 `inventory_delta` 阻断工具名漂移。
4. 将双平台运行数量、独立工具名、profile 差异、AC 映射和 `UNVERIFIED` 项固化到 [t01-capability-baseline.json](../../../execution/evidence/t01-capability-baseline.json)。

本轮未改 `scripts/core.py`、`scripts/probe.py`、`tests/test_core.py` 或 `apps/`。能力清单产品代码已在提交树中；本任务关闭规划与证据。

## 验证

不得用 `just contract*` 替代 T01 清单证据。本轮结果：

- `python ./.trellis/scripts/task.py validate 09-12-t01-capability-baseline`：exit 0
- `python -m unittest discover -s tests -v`：43 ran，42 passed，1 error。失败项为 `test_repository_phase_order` → `RuntimeError: A later task was completed before G0`（`check_source` 因 T06/T07 `completed` 且 G0 未 `passed`）。未回退 T06/T07。
- `python -m unittest tests.test_core.ProfileTests tests.test_cli_inventory.InventoryTests -v`：10 passed，exit 0
- `python -m scripts.tasks unit`：exit 1，`ERROR: A later task was completed before G0`
- `cargo fmt --all -- --check`：exit 0
- `cargo test --workspace --locked --offline`：exit 0（bmdock-probe 4，bmdock-app 14）
- `cargo check --workspace --locked --offline`：exit 0
- `git diff --check`：exit 0
- `npm run build`：跳过（未改 `apps/`）

## 边界

本任务只冻结并采集能力基线。工具/CLI 清单不等于逐项功能成功；完整 rmcp 互操作、Markdown 往返、并发和故障恢复由 T02/T03 验证。桌面负向测试、UI 漂移标记、named CLI 叶子目录、已提交握手/分页页数字段，以及独立于工具名的 schema/命令叶子 CI 阻断，仍为 `UNVERIFIED`。G0 仍保持未通过。
