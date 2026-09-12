# T36 实施记录：安全审查SBOM及隐私验收

本文件是 T36 的执行记录，不是规划草案。未 git commit / push / merge / amend。未改 T05–T35/G0。未改父任务目录。未实现 T37–T40。

## 做了什么

1. 仓库根加入 `LICENSE`（GNU AGPL-3.0 正文，BMDock 原创 AGPL-3.0-or-later）。未把 Cargo.toml `license` 字段当作 LICENSE。
2. 仓库根加入 `NOTICE`，区分 BMDock 原创、官方 Basic Memory（不作为 BMDock 再分发）与第三方锁文件依赖。双 profile 保持隔离（release 21 vs main-preview 27）。
3. 加入 BMDock 自有离线 lockfile inventory：`docs/sbom/lockfile-inventory.json`，从已提交 `Cargo.lock` 与 `apps/bmdock-desktop/package-lock.json` 导出名称/版本。未联网。未编造 Cargo 许可证。`vulnerability_scan` 与 `human_legal_review` 为 UNVERIFIED。不是 hosted-CI 扫描，不是 G7。
4. 在既有 `ipc_invoke` 上增加 typed `inspect_privacy`（`ExplicitRouteArgs`，`deny_unknown_fields`）。
5. 额外 `path` / `root` / `token` / `host` / `api_key` 为 schema。缺路由为 schema。非 fixture 为 policy，且不打开库。
6. 生产 `EmptyLibrary`：空 catalog、`classified_as: empty`、`files_written=false`。`telemetry=false`、`cloud_allowed=false`、`provider_enabled=false`、`html_executed=false`、`executed=false`、`secrets_stored=false`、`env_tokens_read=false`、`remote_hosts_contacted=false`。LICENSE/NOTICE/SBOM 路径报告为已存在文件。`vulnerability_scan=UNVERIFIED`。不宣称 G7。
7. 测试注入 `{temp}/bmdock-t36-*`。夹具 `privacy-claimed` / `sbom-cleared` 为 unsupported，不是通过的安全审查。
8. conflict / `timeout_unknown` / `disk_verified` / `accepted_unverified` 保持区分。IPC 错误联合仍为 policy/schema/unsupported。providers 保持 fail-closed。无 `dangerouslySetInnerHTML`。无新 npm 依赖。无 rmcp。不启动 Supervisor。
9. zh-CN「安全 / 隐私 / SBOM」空态 / 错误 / 就绪均显示未启用 telemetry，不宣称 G7，并始终列出 LICENSE / NOTICE / SBOM 路径为已存在文件、`vulnerability_scan` 为 UNVERIFIED。
10. capabilities 精确允许列 40 → 41。DesktopShellTests 第 30 项：`test_security_sbom_privacy_is_fail_closed_not_g7`。
11. 更新 `typed-ipc-policy.md`、`docs/VERIFICATION.md`、ADR/README/G0_HANDOFF 中原先写 LICENSE/NOTICE/SBOM 缺失的措辞；`execution/status.json` 仅将 T36 标为 completed。

## 验证命令

见 `execution/evidence/t36-security-sbom-privacy.json`。`just contract` / `just contract-main` 未作为 T36 证明运行。`python -m scripts.tasks unit` 因 G0 未 passed 且 T05+ 已 completed 失败，未回退。

## 未做 / UNVERIFIED

- G0 / G7
- 漏洞扫描器（未运行，未伪造）
- 人工法律签署
- hosted CI
- native GUI / WebView2 / Job Object / 安装器
- 真实用户 vault
- 官方 live 引擎 / MCP session
- 远程主机联系、密钥存储（明确未做）
- T37–T40
