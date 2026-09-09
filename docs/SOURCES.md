# 固定源码证据

以下链接用于实现依据；应用是否运行通过以实际日志为准。

- Basic Memory release: `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`，工具注册 `src/basic_memory/mcp/tools/__init__.py`：https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/mcp/tools/__init__.py
- Basic Memory main: `3452c821d76c083823d020984d71e06904a1ff1e`，工具注册 `src/basic_memory/mcp/tools/__init__.py`：https://github.com/basicmachines-co/basic-memory/blob/3452c821d76c083823d020984d71e06904a1ff1e/src/basic_memory/mcp/tools/__init__.py
- 引擎退出需排空 pending materializations：`src/basic_memory/api/app.py:75–90`：https://github.com/basicmachines-co/basic-memory/blob/3452c821d76c083823d020984d71e06904a1ff1e/src/basic_memory/api/app.py#L75-L90
- main 配置读取也可能迁移文件：`src/basic_memory/config.py` 中 `ConfigManager.load_config`：https://github.com/basicmachines-co/basic-memory/blob/3452c821d76c083823d020984d71e06904a1ff1e/src/basic_memory/config.py
- 官方 rmcp 当前核读发布为 `rmcp-v3.2.0`：https://github.com/modelcontextprotocol/rust-sdk/releases/tag/rmcp-v3.2.0
- 核读 SDK 源码 `744b9f904c7f17d589326e61ddbe126cb9d58888`，`crates/rmcp/src/transport/child_process.rs` 中 `MAX_WAIT_ON_DROP_SECS` 与 `graceful_shutdown`：https://github.com/modelcontextprotocol/rust-sdk/blob/744b9f904c7f17d589326e61ddbe126cb9d58888/crates/rmcp/src/transport/child_process.rs
- Tauri 原生前提：https://v2.tauri.app/start/prerequisites/
- Just 官方手册：https://just.systems/man/en/

源码行号未逐项确认处给出实际符号，不虚构定位。main 与 release 的类型、资源和工具可见性不假定一致。SDK 文档与实际 crate 互操作仍由 bootstrap 测试复核。
