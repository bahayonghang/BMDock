# T05 技术设计：Tauri 2 + React/TypeScript 桌面骨架

## 目标与边界

T05 只建立可编译、可锁定、可回滚的桌面工程入口，为 T06/T07/T08 提供宿主和 renderer 边界。它不实现笔记读写、引擎调用、typed IPC 业务命令或生产写入权限。

- Tauri 应用包名固定为 `bmdock-app`；React renderer 固定为 BMDock UI。
- P0 的 `crates/bmdock-probe`、`just contract`、`just contract-main` 和现有 `just build` 语义保持可独立调用。
- renderer 仅提供静态骨架和启动状态占位，不读取文件系统、Basic Memory 配置、MCP 或任意路径。
- 新增 JavaScript/Rust 依赖必须进入真实锁文件；不提交生成目录、凭据或本地构建产物。

## 目录与依赖

```text
apps/bmdock-desktop/
  package.json
  package-lock.json
  tsconfig.json
  vite.config.ts
  index.html
  src/main.tsx
  src/App.tsx
  src/styles.css
  src-tauri/
    Cargo.toml
    tauri.conf.json
    capabilities/default.json
    src/main.rs
```

前端使用 React、TypeScript、Vite 和 `@tauri-apps/api` 的固定版本；Rust 应用使用 Tauri 2 的最小运行时依赖。版本必须由包管理器解析并写入 `package-lock.json`，Rust 依赖必须由 Cargo 写入工作区锁文件。若当前环境无法联网或缺少 Node/Tauri 工具，保留结构化 `UNVERIFIED` 记录，不手写伪造锁文件。

## 数据流与所有权

```text
Tauri window → React App (静态壳)
                  │
                  └─ 后续 T06 的 typed invoke/listen 边界
                             │
                       bmdock-app Rust core
                             │
                       后续 T07 的 rmcp Supervisor
```

T05 的 Rust `main` 只负责启动 Tauri 应用并注册空状态；不得注册 raw `callTool`、任意路径参数或直接文件操作。T06 将在同一入口增加 DTO 和策略，T07 再接管官方引擎进程。官方 Basic Memory 仍是笔记和私有数据库的权威所有者。

## 命令与回滚

- 在 T05 骨架能够完成最小 dev/build 检查后，增加独立的 `just tauri-dev` / `just tauri-build` 入口。
- `just dev` 的切换留到 T05–T08 全部通过并且 G0 入口策略明确之后；切换必须是可回滚的单独 justfile 变更。
- 任一前端或 Tauri 构建失败时，回滚新增入口和锁文件变更即可；不得删除或改写 P0 probe/contract 路径。

## 验收映射

- **AC49**：记录 Tauri 工程、Rust 入口、真实依赖锁和可执行构建结果；Windows 原生运行若不可用则标为 `UNVERIFIED`。
- **AC55**：保留探针构建和合约命令，记录桌面骨架与 P0 工具的独立入口；不把静态目录当作安装器证据。
- **AC60**：在 T05 证据 JSON 和 `docs/VERIFICATION.md` 中链接文件、命令、版本、限制和回滚点。

## 风险与未验证项

真实 Tauri 窗口、Windows WebView2、安装器、签名、GPU/睡眠恢复和 hosted CI 不由 T05 的静态或单元检查证明；这些证据留给 T13、T36–T40。T05 不连接真实 vault，也不启动用户已有引擎。
