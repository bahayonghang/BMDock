# T05 实施计划：初始化代码骨架与真实依赖锁

## 前置门禁

1. 确认 T04 已完成且 `docs/adr/0001-bmdock-boundaries-naming-licensing.md` 可读。
2. 运行 `python ./.trellis/scripts/task.py validate 09-12-t05-tauri-react-skeleton`，确认任务规划和依赖一致。
3. 记录当前工作树；保留与 T05 无关的既有修改。

## 有序步骤

1. 创建 `apps/bmdock-desktop` 的 React/TypeScript/Vite 文件和最小可访问启动页，明确 renderer 不含 MCP、文件系统或路径写入代码。
2. 创建 `apps/bmdock-desktop/src-tauri` 的 Tauri 2 Rust 入口、配置和 capabilities；只注册窗口启动，不实现业务命令。
3. 将应用 crate 纳入 Cargo workspace，使用 Cargo 解析并更新真实 `Cargo.lock`；用 npm 解析并提交 `package-lock.json`。依赖版本、许可和来源写入 T05 证据，后续由 T36 做完整 SBOM/许可复核。
4. 增加 `tauri-dev` / `tauri-build` 的显式开发命令；在 T05 范围内不改变 `dev`、`build`、`contract*` 的既有 P0 语义，除非检查结果和回滚点已记录。
5. 更新 `execution/status.json`、`docs/VERIFICATION.md` 与 `execution/evidence/t05-tauri-react-skeleton.json`，逐项链接 AC49、AC55、AC60；缺失 native/hosted 证据写为 `UNVERIFIED`。

## 最小验证

- `python ./.trellis/scripts/task.py validate 09-12-t05-tauri-react-skeleton`
- `python -m scripts.tasks unit`
- `cargo fmt --all -- --check`
- `cargo test --workspace --locked`
- 在具备 Node/Tauri 环境时运行 `npm ci --ignore-scripts`、`npm run build` 和 Tauri dev/build；在当前环境缺失工具时记录命令和缺失原因。
- `git diff --check`，并确认 P0 `just contract*` 文件和命令未被删除或改写。

## 风险文件与回滚点

- 高风险：`Cargo.toml`、`Cargo.lock`、`apps/bmdock-desktop/package-lock.json`、`justfile`。
- 若依赖解析产生非预期升级，删除 T05 新增目录并恢复锁文件，再保留证据记录；不运行隐式网络安装，不修改 `.work/engines`。
- 若 Tauri 配置需要平台专属设置，先保留最小跨平台配置；Windows 原生差异移交 T13。

## 完成门槛

只有当骨架文件、真实锁文件、最小静态构建检查、P0 入口保留证据和三项 AC 映射齐全后，才可把 T05 标为完成并进入 T06/T07；不能以目录存在、UI 文案或工具清单替代构建和边界证据。
