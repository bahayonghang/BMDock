# docs

> [仓库根](../CLAUDE.md) › `docs`
> 上次扫描：2026-09-09 13:16 +08:00

P0/G0 实施与验收文档。状态以 `execution/status.json` 与 CI 证据为准；本文说明边界与交接，不把规划写成已通过。

## 文件

| 文件 | 内容 |
|---|---|
| [IMPLEMENTATION.md](IMPLEMENTATION.md) | 首批结构、退出设计、命令演进、仍待 G0 项 |
| [SOURCES.md](SOURCES.md) | 上游 commit、rmcp、Tauri 前提、just 手册等固定链接 |
| [VERIFICATION.md](VERIFICATION.md) | 提交 `03c2839` / Actions run `34309026376` 的双平台复核 |
| [G0_HANDOFF.md](G0_HANDOFF.md) | P0 相关 AC 摘录与“不可扩大证据范围”条款 |
| [adr/0001-bmdock-boundaries-naming-licensing.md](adr/0001-bmdock-boundaries-naming-licensing.md) | T04 命名、所有权、双 profile 隔离与 AGPL 方向；不是 LICENSE/SBOM 证明 |

根 [README.md](../README.md) 面向开发者命令与当前交付边界。

## 关键结论（供 Agent 引用）

- 本批交付 probe/test 基础设施，对应 T01 能力采集，供 T02/T03 使用。T02/T03 不得仅因 T01 文档标完成。
- T04 将命名、官方引擎所有权、双 profile 隔离和 AGPL-3.0-or-later 方向记入 ADR-0001；`Cargo.toml` 的 `license` 字段只是 crate 元数据。`LICENSE` / NOTICE / SBOM 仍为 T36 `UNVERIFIED`。T04 文档不能把 G0 或发行合规写成已通过。
- G0 通过后，T05 才会把 `dev`/`build` 切到 Tauri 入口；probe/contract 保留独立命令。
- 正常退出后看到文件 ≠ 强杀/丢响应安全；单次 append ≠ 并发写安全；列出 MCP 工具 ≠ 全部工具已执行；Windows 探针成功 ≠ Tauri 安装包验收。
- 已核验能力数量（release / main-preview）：tools 21/27，prompts 4/4，resources 1/32，resource templates 1/3，CLI 节点 99/122，CLI 叶子 83/104，OpenAPI paths 47/49。数量不等于功能验收。
- 6 个共有工具的 inputSchema 有差异（如 `read_note` 增加 `start_line`/`end_line`）。完整表在 `execution/evidence/profile-comparison.json`。

## G0 仍缺（摘自 IMPLEMENTATION / HANDOFF）

1. 全量原文与未知 YAML 的可逆读写、差异批准。
2. Obsidian/Agent 竞争窗口；无法保障时关闭并发写模式。
3. 已接受但丢失响应、取消、强杀、磁盘故障注入。
4. ChatGPT 专用工具限制、resource/prompt 实际调用、CLI/API 叶子审查。
5. 许可/SBOM/依赖风险记录与完整原生桌面验收。

相关 AC 标识：AC01、AC02、AC07、AC08、AC10、AC15–AC18、AC35、AC36、AC54、AC58、AC60。全文见 `G0_HANDOFF.md`。

## 关联模块

- 状态机：[execution](../execution/CLAUDE.md)
- 根索引：[仓库根](../CLAUDE.md)
