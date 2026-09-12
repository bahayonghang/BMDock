# 实现引擎Supervisor与连接状态机

## Goal

完成设计中的 T07（P1）交付，使其成为可独立实施和验收的产品/验证能力。

## Scope

- 按父任务设计实现：实现引擎Supervisor与连接状态机。
- 前置任务：T05, T02。
- 关联验收要求：AC04, AC05, AC16, AC17, AC18, AC47。
- 遵守双 profile 隔离、typed IPC、官方引擎所有权和证据边界；不得绕过前置任务。

## Acceptance Criteria

- [ ] T07 的实现或验证结果可通过自动化检查、真实运行记录或明确的 `UNVERIFIED` 证据记录复核。
- [ ] 关联 acceptance IDs（AC04, AC05, AC16, AC17, AC18, AC47）逐项有文件、日志、测试或文档证据。
- [ ] 失败、拒绝、超时未知、磁盘状态与成功结果保持区分，不以 UI 文案或工具清单替代行为证据。
- [ ] 不修改真实 vault、全局 Basic Memory 配置或未授权远程资源；生成物和凭据不进入提交。
- [ ] 完成后同步 `execution/status.json`、父任务依赖和相关文档，运行本任务范围内的最小检查。

## Dependencies

- Depends on: T05, T02
- Phase: P1
- Parent: `09-12-bmdock-product-completion`

## Out of Scope

- 不提前完成后续任务或改变未授权的产品范围。
- 不删除失败测试、绕过门禁或把缺失的 native/现场/生产证据推断为已验证。
