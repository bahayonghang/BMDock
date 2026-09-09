# BMDock

Basic Memory 专用桌面工作台。目标技术栈：**Tauri 2 + React/TypeScript + Rust/rmcp + 官方 Basic Memory**。

## 当前交付边界

本仓库从实施包的 **P0 / G0 技术验证**开始，不跳过数据安全门禁。当前实现是可执行的开发验证工具，而不是一个已完成的桌面应用：固定引擎基线、隔离 sandbox、真实 MCP 探针、能力快照、文件物化 smoke tests、任务状态与跨平台命令。

**当前 `just dev` 启动 G0 探针，不启动 GUI；`just build` 构建探针，不生成桌面安装器。** Tauri/React 产品工程在 G0 完成后进入 T05。没有连接或修改你的真实 Obsidian vault、Basic Memory 配置或 Agent 配置。

## 开发环境

需要 Python 3.12+（命令名 `python`）、Git、Rust/rustup、uv 和 just。Rust 工具链固定为 1.90.0，rmcp 固定为 3.2.0。CI 使用 uv 0.12.11、just 1.40.0；脚本宿主 Python 3.13.5，引擎采用 uv 管理的 Python 3.12.12。后续版本升级必须显式审查，不以浮动 latest 替代锁定。

```powershell
# 在已安装开发工具的 Windows PowerShell 或其他终端执行：
git clone https://github.com/bahayonghang/BMDock.git
cd BMDock
just doctor
just setup     # 显式联网：在 .work/engines 安装两个固定版本和其锁定依赖
just ci
just dev
just build
```

`Cargo.lock` 已提交，正常使用不需要重新生成。`justfile` 使用 Python 作为 recipe shell，不依赖 bash、PowerShell 专属语法或 `rm -rf`。也可直接执行 `python -m scripts.tasks unit` 等同名子命令。

| 命令 | 当前行为 |
|---|---|
| `just doctor` | 检查工具，不启动 Basic Memory；缺工具返回非零 |
| `just setup` | 拉取两个固定 commit，使用各自 `uv.lock` 安装隔离引擎；唯一主动下载引擎的任务 |
| `just lock` | 显式重新解析 Cargo 依赖，变更须审查提交；日常运行不需要 |
| `just ci` | Python 单测、Rust fmt/clippy/单测、两版真实引擎 smoke tests；依赖缺失即失败 |
| `just ci-unit` | 无第三方依赖的 Python 单测；不能替代真实引擎测试或 G0 |
| `just dev` / `just dev-main` | 在全新临时 sandbox 中运行 release/main 探针 |
| `just contract` / `just contract-main` | 单独运行指定基线的真实引擎 smoke tests |
| `just build` | 构建 `target/release/bmdock-probe.exe`（Windows）或无扩展名可执行文件 |
| `just gate` | 输出 G0–G7 状态；任何门禁未通过都返回非零 |

## 已实现

- 两套不可变引擎 profile：v0.23.2 对应 21 个工具，main 快照对应 27 个；不混用两版契约。
- 从零生成测试项目与配置，不复制包含生产路径的用户配置；清除环境中的 provider key、Cloud 路由和 PYTHONPATH。
- 通过官方 Rust SDK 处理 MCP 握手与协议 framing；开发控制通道只运行已列明的方法/fixture 工具。
- 分页发现 tools/resources/templates/prompts；检测重复 cursor、重复工具、新增/缺失工具；保留 schema hash。
- 读取 CLI 注册树和 OpenAPI；使用结构化遍历兼容 Typer 内置的 Click，要求关键根命令存在；不把注册存在当成功执行。
- 在 fixture 中验证创建、中文内容、自定义 metadata、wiki-link、读取、单次 append、正常退出后的文件可见性。
- 对结果 unknown、不认识的 structuredContent、MCP tool error 和业务拒绝保持区分；不自动重试写入。

## 验证状态与剩余门禁

已在 GitHub Actions Windows 和 Ubuntu 上运行 `just ci`、`just build`，核验过两个真实引擎。具体提交、运行记录与已发现的问题见 [验证记录](docs/VERIFICATION.md)；最新修改的状态应以其对应 CI 为准，不沿用前一提交的绿色结果。

当前 smoke suite **不是完整 G0**。丢响应故障注入、全部原文往返、真正并发写入、Windows junction/睡眠恢复、客户端限制、全部资源/prompt 调用仍未关闭。G0 未通过前，不实施真实资料库的写入 UI，不把 probe 通过宣称为桌面安全可用。

标准 `CI` 仅检查已提交的依赖锁与源码，不在验证中重新解析依赖或格式化代码。缺少本地 Rust/just 时可以运行 `python -m scripts.tasks unit`，但这不能代替完整 CI。

## 数据边界与来源

探针是**可信开发者工具**，不是面对不可信 renderer 的生产 IPC。它只接入 `.work/g0` 下生成的 fixture；请勿手工把真实目录接入探针。应用不会重写 Basic Memory 私有数据库，也不创建第二套权威索引。禁用模型/更新并清理环境不等于操作系统级网络隔离，完整离线保证需另行验证。

资料：[实施状态](execution/status.json)、[技术决策](docs/IMPLEMENTATION.md)、[源码证据](docs/SOURCES.md)。不整体 Fork Memmy；它只作为产品与工程参考。

当前原创代码拟采用 AGPL-3.0-or-later，与官方引擎分发方向对齐；正式发行前仍需完整 LICENSE、依赖许可、SBOM 与源码交付审查，不将这一声明作为法律结论。
