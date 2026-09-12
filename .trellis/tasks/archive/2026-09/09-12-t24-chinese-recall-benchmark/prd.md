# 中文召回与性能基准

## Goal

完成设计中的 T24（P4）交付，使其成为可独立实施和验收的产品/验证能力。

## Scope

- 按父任务设计实现：中文召回与性能基准。
- 前置任务：T20, T21, T23。
- 关联验收要求：AC26, AC29, AC56。
- 遵守双 profile 隔离、typed IPC、官方引擎所有权和证据边界；不得绕过前置任务。

## Acceptance Criteria

- [x] T24 的实现或验证结果可通过自动化检查、真实运行记录或明确的 `UNVERIFIED` 证据记录复核。
- [x] 关联 acceptance IDs（AC26, AC29, AC56）逐项有文件、日志、测试或文档证据。
- [x] 失败、拒绝、超时未知、磁盘状态与成功结果保持区分，不以 UI 文案或工具清单替代行为证据。
- [x] 不修改真实 vault、全局 Basic Memory 配置或未授权远程资源；生成物和凭据不进入提交。
- [x] 完成后同步 `execution/status.json`、父任务依赖和相关文档，运行本任务范围内的最小检查。

## Dependencies

- Depends on: T20, T21, T23
- Phase: P4
- Parent: `09-12-bmdock-product-completion`

## Out of Scope

- 不提前完成后续任务或改变未授权的产品范围。
- 不删除失败测试、绕过门禁或把缺失的 native/现场/生产证据推断为已验证。
