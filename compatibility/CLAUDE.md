# compatibility

> [仓库根](../CLAUDE.md) › `compatibility`
> 上次扫描：2026-09-09 13:16 +08:00

不可变 G0 候选引擎清单。用途：固定来源、commit、静态 MCP 工具名。`purpose` 字段写明：未验证为生产支持矩阵。

唯一数据文件：`profiles.json`（`schema_version: 1`）。

## 接口

`scripts.core.profiles()` 读取 `engines` 对象。合法 key 仅 `release` 与 `main-preview`（`tasks.py` argparse 同步限制）。

每条 profile 字段：

| 字段 | 含义 |
|---|---|
| `repository` | `https://github.com/basicmachines-co/basic-memory.git` |
| `commit` | 40 位不可变 SHA |
| `version` | release 为 `"0.23.2"`；main-preview 为 `null` |
| `expected_tools` | 静态 MCP 工具名列表（须 internally unique） |
| `verified_runtime` | 当前均为 `false` |
| `registry_source` | 上游 `src/basic_memory/mcp/tools/__init__.py` blob URL |

## 当前钉扎

| profile | commit | 工具数 | 备注 |
|---|---|---:|---|
| `release` | `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048` | 21 | tag `v0.23.2` 必须指向同一 commit，否则 `setup` 拒绝 |
| `main-preview` | `3452c821d76c083823d020984d71e06904a1ff1e` | 27 | 相对 release 增加 `cat` `find` `grep` `ls` `man` `tail` |

`setup` 对 release 会 `git fetch` tag 并校验 `tag^{commit}`。main-preview 无 version，不做 tag 校验。

运行时 `scripts.probe.run_contract` 把 MCP `tools/list` 与 `expected_tools` 做 `inventory_delta`；missing 或 added 使 suite 失败。schema 形状差异另记在 [execution/evidence/profile-comparison.json](../execution/evidence/profile-comparison.json)，不自动等于 breaking change。

## 约束

- 禁止引入 `latest` / 浮动分支名作为 profile id。
- 禁止把两套 `expected_tools` 合成一份“当前工具表”。
- 改工具基线必须同时改本文件、单测中的 21/27 断言、以及文档中的发现表。
- `verified_runtime: false` 在完整 G0 通过前保持。

## 关联模块

- 读取方：[scripts/core.py](../scripts/core.py)
- 证据：[execution](../execution/CLAUDE.md)
- 源码链接：[docs/SOURCES.md](../docs/SOURCES.md)
