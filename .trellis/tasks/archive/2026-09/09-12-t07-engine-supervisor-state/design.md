# T07 技术设计：Engine Supervisor 与连接状态机

## 1. 边界

T07 只负责 Tauri Rust 核心中的引擎生命周期抽象和连接状态转换：

- release 与 main-preview 作为不可混用的 `EngineProfile`；
- Supervisor 独占引擎子进程句柄，外部只能通过显式 start/connected/shutdown 操作改变状态；
- 连接状态可序列化为 typed runtime snapshot，供后续 IPC/UI 任务消费；
- 关闭分为 transport cancel 和 child wait 两阶段，超时后的强杀与退出码单独记录；
- P0 `bmdock-probe`、`just contract` 和 `just contract-main` 不复用产品 Supervisor。

T07 不实现笔记 CRUD、真实 vault 路由、搜索、安装器或云连接。当前环境无法证明真实官方引擎和 native GUI 生命周期，因此这些运行证据保持 `UNVERIFIED`。

## 2. 数据模型

- `EngineProfile::{Release, MainPreview}`：携带 profile id、固定 upstream commit 和期望工具数；不提供合并 profile。
- `ConnectionState::{NotStarted, Starting, Connected, Stopping, Stopped, Failed}`：只允许定义的有向转换。
- `FailureKind::{Policy, Transport, TimeoutUnknown, Process, Unverified}`：禁止把超时未知、传输错误和拒绝合并成成功。
- `RuntimeSnapshot`：状态、profile、child pid、failure kind 和 shutdown receipt；不包含 vault 路径或 raw MCP payload。
- `EngineLaunchSpec`：程序、参数、工作目录、环境和 profile 的显式启动规格；调用方负责提供已隔离的 engine worker。

## 3. 生命周期与所有权

1. `start(spec)` 仅允许从 `NotStarted` 或 `Stopped` 进入 `Starting`，校验 profile 和启动规格后取得子进程所有权。
2. transport 握手完成后调用 `mark_connected()`，从 `Starting` 进入 `Connected`。
3. transport 或进程错误调用 `mark_failed(kind)`，保留失败分类和最后 profile。
4. `shutdown(cancel_transport)` 先执行 transport cancel，再在限定预算内等待 child；等待超时才强杀，并返回 `ShutdownReceipt`。
5. 关闭后只进入 `Stopped`；若 cancel 或 wait 结果未知，receipt 保留 `timeout_unknown`/`forced`，不得伪装为正常退出。

## 4. 测试替身

子进程句柄和 transport cancel 通过小型 trait 注入。单元测试使用 fake child/transport 验证状态转换、拒绝非法转换、双阶段顺序、强杀标记和错误分类，不启动真实用户引擎。

## 5. IPC 接线与后续边界

T07 将 `Supervisor` 放入 Tauri managed state，并让现有 `get_runtime_state`
通过 typed DTO 投影 snapshot；事件广播、start/stop 用户控件和真实 transport
接线留给后续任务。T07 不扩展 T06 的 raw `callTool` 边界，也不改变 P0 控制协议。
