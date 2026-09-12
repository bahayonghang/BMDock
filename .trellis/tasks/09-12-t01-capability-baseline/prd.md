# 固定版本并采集完整能力清单

## Goal

完成设计中的 T01（P0）交付，使其成为可独立实施和验收的产品/验证能力。

## Scope

- 按父任务设计实现：固定版本并采集完整能力清单。
- 前置任务：无。
- 关联验收要求：AC01, AC02, AC35, AC36, AC58。
- 遵守双 profile 隔离、typed IPC、官方引擎所有权和证据边界；不得绕过前置任务。

## Acceptance Criteria

- [x] T01 的实现或验证结果可通过自动化检查、真实运行记录或明确的 `UNVERIFIED` 证据记录复核。
- [x] 关联 acceptance IDs（AC01, AC02, AC35, AC36, AC58）逐项有文件、日志、测试或文档证据。
- [x] 失败、拒绝、超时未知、磁盘状态与成功结果保持区分，不以 UI 文案或工具清单替代行为证据。
- [x] 不修改真实 vault、全局 Basic Memory 配置或未授权远程资源；生成物和凭据不进入提交。
- [x] 完成后同步 `execution/status.json`、父任务依赖和相关文档，运行本任务范围内的最小检查。

证据汇总见 [t01-capability-baseline.json](../../../execution/evidence/t01-capability-baseline.json)。AC02 在本任务只覆盖真实引擎发现；完整互操作属于 T02。CLI/API 叶子功能成功、桌面负向测试、UI 漂移标记、已提交握手/分页页数字段，以及独立于工具名的 schema/命令叶子 CI 阻断为 `UNVERIFIED`。

## Dependencies

- Depends on: 无
- Phase: P0
- Parent: `09-12-bmdock-product-completion`

## Out of Scope

- 不提前完成后续任务或改变未授权的产品范围。
- 不删除失败测试、绕过门禁或把缺失的 native/现场/生产证据推断为已验证。
