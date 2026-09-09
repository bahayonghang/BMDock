# 验证记录：首个实施批次

## 本地环境与单元测试

作者环境：Linux，Python 3.13.5；Rust/cargo/just 不存在，依赖下载受 DNS 限制。实际执行 `python -m scripts.tasks unit`：**42 passed、0 failed、0 skipped、exit 0**，其中 4 项是 CLI 清单遍历回归测试。本地没有运行真实引擎或 Rust 编译；这些由下列 GitHub Actions 验证。

## 已完成的远端运行

### 首轮 bootstrap：失败得到真实证据

Run [34307780159](https://github.com/bahayonghang/BMDock/actions/runs/34307780159)，commit `21a7645`。Ubuntu：38 项 Python 单测、Rust fmt/clippy、4 项 Rust 单测、debug 编译成功；真实引擎在执行 MCP 前被版本预检拦截：浅克隆未取 release tag，构建版本成为 `0.0.1.dev1+c0bd87c`。Windows 被 setup-python 缺少 3.12.12 安装包阻塞。

修复：显式获取并核验 release tag 指向固定 SHA；CI 宿主使用 Python 3.13.5，引擎 Python 3.12.12 由固定 uv 0.12.11 管理。远端实际生成的 Cargo.lock 和 rustfmt 输出已经复核提交；后续 CI 不再修改它们。

候选 ZIP SHA256：`77139a3aeba07ea1ac91dfd85656b403d36c03742134b05bf72412e5f591ea0c`。
Cargo.lock SHA256：`3c7ff364bc7691c3b225bb20521c099258854d79a352dde47498a699437fa950`。

### 严格 CI：双平台执行成功，但清单审查发现遗漏

Run [34308370383](https://github.com/bahayonghang/BMDock/actions/runs/34308370383)，commit `8a1dd54b2ea294e84ba3998c2931afb93e94d994`。

| 项目 | Ubuntu | Windows |
|---|---|---|
| 已提交 lock 与 fmt 检查 | passed | passed |
| Python 38 项测试、Rust clippy 与 4 项测试 | passed | passed |
| release 与 main-preview 的真实 MCP smoke suites | passed | passed |
| `just build` release 探针 | passed | passed |
| 构建后 tracked git diff | clean | clean |

已下载并核对 Ubuntu 原始报告，7 项 smoke checks 均通过：有效隔离配置、分页 MCP 发现、工具名基线、未知工具错误、创建/读取/文件物化、单次 append、正常退出后的文件观察。两个引擎均正常退出（exit 0、非强杀）。

| 运行时发现结果 | release | main-preview |
|---|---:|---:|
| MCP tools | 21 | 27 |
| prompts | 4 | 4 |
| resources | 1 | 32 |
| resource templates | 1 | 3 |
| OpenAPI paths | 47 | 49 |

**审查发现该版本 CLI 清单只有根节点，不能视为完整。** 原因是 Typer 内置 Click 与外部 `click.Group` 类身份不一致，使 `isinstance` 漏掉所有子命令。当前修复改用各命令的 context_class 与 group 公共方法，并加入关键根命令完整性断言、循环/缺失子命令保护和 4 项回归测试。修复后的远端结果必须单独核验，不能引用上一版绿色 CI 作为修复通过的证据。

## 验收范围

MCP resources/prompts/API/CLI 的注册清单并不等于已逐个成功执行。当前没有完整原文往返、并发、取消、丢响应、磁盘故障或原生桌面安装器的验收结果。所有完整产品门禁仍未通过；`just gate` 返回非零是当前真实状态。

探针及测试不会接入用户真实 Obsidian vault。环境过滤不是 OS 网络沙箱，跨 Agent 原子写入也未被本批 smoke test 证明。后续门禁严格区分已实现、已测试与已具备生产安全保证。
