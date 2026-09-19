# Journal - lyh (Part 1)

> AI development session journal
> Started: 2026-09-09

---



## Session 1: T07 引擎 Supervisor 与连接状态机

**Date**: 2026-09-12
**Task**: T07 引擎 Supervisor 与连接状态机
**Package**: bmdock-probe
**Branch**: `main`

### Summary

完成 T07 Supervisor 状态机、质量检查、code-spec 更新与提交；修复 spawn 先拉进程导致的 policy/process 误分类。

### Main Changes

- 实现 Supervisor 生命周期、双 profile 隔离与 typed get_runtime_state 投影
- 质量检查后补上 spawn 先 policy 校验、Drop kill 与 DTO/失败分类测试
- 更新 supervisor-state 与 typed-ipc-policy code-spec

### Git Commits

| Hash | Message |
|------|---------|
| `736e07a` | (see git log) |

### Testing

- [OK] task validate; cargo fmt/test/check offline; npm build; git diff check

### Status

[OK] **Completed**

### Next Steps

- T08 桌面布局与 i18n；真实引擎/native GUI/Job Object 仍为 UNVERIFIED


## Session 2: 客户端优化交付与部分验收归档

**Date**: 2026-09-19
**Task**: 客户端优化交付与部分验收归档
**Package**: bmdock-probe
**Branch**: `main`

### Summary

完成本轮逻辑拆分提交，归档 C02/C04，保留原生与完整矩阵待验收任务；按用户选择忽略十份大型原始证据并提交摘要及紧凑回放输入。

### Main Changes

- 官方隔离只读会话、保真查询和八请求上限；最新意图与草稿保护、按需诊断、三页结果窗口。
- C02/C04 已验收归档；父任务及 C01/C03/C05/C06/C07 保持开放；六个原有未跟踪文件保留。

### Git Commits

| Hash | Message |
|------|---------|
| `3986f64` | (see git log) |
| `1e64708` | (see git log) |
| `bd4c6da` | (see git log) |
| `b75bd15` | (see git log) |

### Testing

- [OK] 233 Rust、68 Python、43 前端行为/SSR及构建通过；双引擎140测量样本和四个大正文检查通过；原生与完整矩阵未通过验收，历史G0/Clippy失败保留。
- [OK] 紧凑回放1项通过；独立核对六个payload及十份原始哈希；生产代码和已测二进制未变；归档上下文及父子关系通过。

### Status

[OK] **Completed**

### Next Steps

- 按 parent research/continuation-prompt.md 完成原生交互、统一时钟的完整性能矩阵与进程树RSS；不放宽阈值或伪造产品门禁。
