# 验证记录：首个实施批次

作者环境：Linux；Python 3.13.5，Node 22 可用；Rust/cargo/just 不存在；访问 GitHub raw、Rust 下载站、PyPI 和 npm registry 时 DNS 解析失败。

| 验证 | 命令 | 实际结果 |
|---|---|---|
| Python 源码、基线与阶段顺序检查 | `python -m scripts.tasks unit` | exit 0 |
| Python 单元测试 | 同上 | 38 passed，0 failed，0 skipped |
| 真实 rmcp + Basic Memory | `just ci` 的真实引擎部分 | 作者环境未执行；不能从 unit 通过推断 |
| Rust fmt/clippy/test/build | `just ci` / `just build` | 作者环境未执行；缺 Rust 和依赖下载 |
| Tauri/Windows 安装器 | 尚未进入 T05/T37 | 未实现、未验收 |

GitHub bootstrap 是显式生成候选锁和格式化源码的工作流；若产生 artifact，必须按提交、OS 和日志复核。它不自动覆盖源代码、不自动把 execution/status.json 改为 passed。没有看到成功运行结果之前，远端验证状态保持未验证。

当前 probe smoke tests 覆盖握手、工具/资源/模板/提示词发现、工具名差异、未知工具错误、fixture create/read/append 和退出后文件观察。CLI/OpenAPI 仅清单采集，不等于已逐个执行。

G0 仍需完整往返、并发、取消、丢响应及原生异常场景；所有产品 AC 保持未完成。`just gate` 返回非零是预期的真实状态，不是要用忽略错误规避的检查。
