# T02 实施记录：rmcp 与官方引擎互操作验证

## 已完成

1. 扩展 `scripts/probe.py` 的真实进程合约：双 profile 分别完成 2025-11-25 握手断言、分页 tools/resources/templates/prompts 发现、首个 resource 读取和首个 prompt 获取。
2. 对每个运行时工具校验 `inputSchema` 对象形状、required/property 闭包并保存 schema 指纹；工具基线仍按 release 21、main-preview 27 独立比较。
3. 验证控制通道错误分类：未允许方法返回 `policy`，畸形 rmcp 请求返回 `schema`，缺失 resource 的 `resources/read` 返回上游 `rpc_or_transport`；未知工具与互换身份的 search/fetch 调用返回 MCP `isError`，不与本地拒绝混淆。
4. 将 MCP 写入包分类为 `accepted_unverified`（含 FastMCP `structuredContent.result.action`），仅在 fixture Markdown 物化观察后记录成功。
5. 生成双 profile 原始报告及 [t02-rmcp-interoperability.json](../../../execution/evidence/t02-rmcp-interoperability.json)，同步 `execution/status.json` 的 T02 字段和验证文档。未改 T03/T04 工作树状态，未回退 T06/T07。

## 验证

- `python ./.trellis/scripts/task.py validate 09-12-t02-rmcp-interoperability`：exit 0
- `python -m unittest tests.test_core.ResultTests tests.test_core.InteropTests tests.test_core.ProfileTests tests.test_cli_inventory.InventoryTests -v`：26 passed，exit 0
- `python -m unittest discover -s tests -v`：51 ran，50 passed，1 error。失败项为 `test_repository_phase_order` → `RuntimeError: A later task was completed before G0`（`check_source` 因 T06/T07 `completed` 且 G0 未 `passed`）。未回退 T06/T07。
- `python -m scripts.tasks unit`：exit 1，`ERROR: A later task was completed before G0`
- `cargo fmt --all -- --check`：exit 0
- `cargo test --workspace --locked --offline`：exit 0（bmdock-probe 5，bmdock-app 14）
- `cargo check --workspace --locked --offline`：exit 0
- `git diff --check`：exit 0
- `python -m scripts.tasks contract release`：exit 0，真实引擎 suite passed
- `python -m scripts.tasks contract main-preview`：exit 0，真实引擎 suite passed（单独调用，未合并报告）
- `npm run build`：跳过（未改 `apps/`）
- `just setup`：未运行

## 范围限制

所有运行均在自动生成的 `bmdock-fixture` sandbox 中；没有用户 vault、Cloud、native GUI 或 Job Object 证据。丢响应、取消后接受、`timeout_unknown` 现场注入、强杀恢复和磁盘故障注入保留为 `UNVERIFIED`，由 T03 验证。G0 仍未通过。
