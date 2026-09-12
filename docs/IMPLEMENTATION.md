# 首批实施记录与技术边界

## 与实施包的对应关系

本批开始 T01 的能力基线与采集基础设施，同时编写供 T02/T03 使用的 probe/test 工具；这些工具不是已通过的产品模块，不将 T02/T03 提前标记完成。

40 个任务的阶段、依赖和 AC 映射保存在 execution/status.json。原规划 ZIP 的 SHA256 一并保存；所有后续门禁仍未完成。G0 需要真实引擎、原文往返、并发/物化边界与 ADR 的完整证据。缺少原生环境不能以单元测试顶替。

## 当前结构

```text
justfile → Python task dispatcher
              ├─ isolated upstream checkout + uv.lock
              ├─ sandbox/config/environment policy
              └─ Rust bmdock-probe
                    └─ official rmcp over stdin/stdout
                          └─ official Basic Memory Python process
```

Rust probe 保留 Python 子进程所有权，通过 rmcp 原生异步字节流处理协议，主动关闭后等待子进程退出并记录是否强杀；不经前端转发 stdio。它只支持测试所需的有限 MCP 方法；不能作为未来产品的 raw callTool/exec 接口。

## 退出设计

核读的 rmcp TokioChildProcess 默认 graceful shutdown 等待最多 3 秒后 kill，关闭成功也不能证明写入落盘。当前探针采用独立 Child + SDK transport：SDK 关闭和子进程等待分别设限；forced/nonzero 退出使 suite 失败；即使正常退出，也只在 fixture 文件确实包含预期内容时记录该项通过。

这不是对生产故障恢复的证明：强杀、磁盘满、超时丢响应和 Windows Job Object 后续仍需 T03/T17 验证。

## 命令演进约定

当前 `just ci` / `just build` 仍针对 P0 探针。T08 已将 `just dev` 切到 Tauri 桌面壳，但 **不** 把 `just build` 切到 Tauri，因为 CI 仍用 `just build` 编译 G0 探针。`just contract*` 保持独立。G0 未通过前，不提供可写笔记界面。

`just ci-unit` 是明确的局部测试。`just ci` 包含目前已实现的真实引擎 smoke suites，缺环境失败。`just gate` 反映全部产品门禁，不因为 smoke 通过自动改为 passed。

## 首批仍待实现的 G0 项

1. 全量原文/未知 YAML 的可逆读取、编辑和差异批准路径。
2. 检测并复现 Obsidian 与 Agent 竞争窗口；必要时关闭并发写模式。
3. 发送已接受但丢失响应、取消、强杀、磁盘故障等故障注入。
4. ChatGPT 专用工具限制、资源/提示词调用和完整 CLI/API 叶子清单审查。
5. 真实 Windows 验证、SDK/依赖锁复核与许可风险记录。

本批不宣称上述条件已满足；不会启动任何用户现有引擎实例、停用用户服务或改默认项目。
