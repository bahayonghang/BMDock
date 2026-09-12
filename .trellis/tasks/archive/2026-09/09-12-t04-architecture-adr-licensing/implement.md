# T04 实施记录：命名、许可方向与架构 ADR

## 已完成

1. 新增并收紧 `docs/adr/0001-bmdock-boundaries-naming-licensing.md`：冻结产品、探针、
   Tauri crate、桌面前端目录和 renderer 名称；双 profile 使用完整 commit 且不得合并；
   官方引擎拥有笔记语义；AGPL-3.0-or-later 仅为方向；明确仓库没有 LICENSE/NOTICE/SBOM。
2. 收紧 `execution/evidence/t04-architecture-adr-licensing.json`，使 AC07/AC54/AC60
   的 claim 与现存文件一致；LICENSE/NOTICE/SBOM 与 G0 保持 `UNVERIFIED` /
   `in_progress`。`Cargo.toml` 工作区 `license` 字段记为方向元数据，不是 LICENSE。
3. 同步 `docs/VERIFICATION.md`、`docs/CLAUDE.md`、`docs/G0_HANDOFF.md`、
   `execution/CLAUDE.md`、`execution/status.json` 与本任务元数据。没有实现 T05
   桌面代码，没有运行 `just contract`，没有回退 T06/T07，也没有接触真实 vault。
4. Check 收口：补上 `bmdock-desktop` 名称冻结；写明 Cargo crate `license` 不能替代
   `LICENSE`；G0_HANDOFF 不再把“记录 ADR”写成随即验收 G0。

## 验收证据

- AC07：ADR 数据所有权与分层；官方 Basic Memory 是笔记语义真源。
- AC54：ADR/README/`Cargo.toml` 记录 AGPL 目标方向；LICENSE/NOTICE/SBOM/依赖扫描
  仍为 T36 `UNVERIFIED`。
- AC60：ADR、证据 JSON、验证记录和机器状态互相链接；不把工作树 T04 completed
  写成 HEAD 已发布。

## 检查

- `python ./.trellis/scripts/task.py validate 09-12-t04-architecture-adr-licensing` → exit 0
- `python -m json.tool execution/status.json` → exit 0；G0 `in_progress`；T04
  `completed`；T05 `in_progress`；T06/T07 `completed`（未回退）
- `python -m json.tool execution/evidence/t04-architecture-adr-licensing.json` → exit 0
- `python -m unittest discover -s tests -v` → exit 1；Ran 60；59 passed；
  `test_repository_phase_order` 因 G0 未通过且 T06/T07 已 completed 报错（预期）
- `python -m scripts.tasks unit` → exit 1；`ERROR: A later task was completed before G0`
- `cargo fmt --all -- --check` → exit 0
- `cargo test --workspace --locked --offline` → exit 0；bmdock-probe 5、bmdock-app 14
- `cargo check --workspace --locked --offline` → exit 0（既有 bmdock-app
  dead_code 警告，T04 未改该代码）
- `git diff --check` → exit 0
- `just contract` 未运行（不能当作许可、native GUI 或真实 vault 证据）

## 未验证项

G0 仍未通过。native GUI、真实 vault、丢响应/取消/超时/强杀恢复、磁盘故障、
`LICENSE`、NOTICE、SBOM、依赖许可扫描、源码交付、漏洞处置和 hosted CI 证据均由
后续任务负责。
