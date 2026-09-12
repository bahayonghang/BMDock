# T03 实施记录：Markdown 往返与并发写入验证

## 已完成

1. 保留 T02 落地的真实引擎合约（commit `7adc264`）：双 profile 隔离、`accepted_unverified` 后再 `wait_note`、两独立 probe 并发写不同 fixture 笔记、关闭后读盘。
2. 补 T03 断言缺口：`require_markdown_fidelity` 在创建后与正常关闭后都检查未知 frontmatter、中文正文和 wiki-link；`require_concurrent_outcomes` 要求两个 worker 落到不同 path/sentinel，且 path/sentinel 必须是非空字符串。
3. 增加 Observation 单测覆盖 `wait_note` 唯一性、超时不重试、Markdown 保真失败关闭，以及并发结果不得用 envelope、缺失 path 或丢失 frontmatter 冒充落盘。
4. 丢响应、取消后接受、`timeout_unknown` 现场注入、强杀和磁盘故障保持 `UNVERIFIED`；不同目标并发成功不推断相同目标冲突原子性；干净关闭不推断丢响应/强杀/磁盘故障。
5. 汇总证据见 [t03-markdown-concurrency-recovery.json](../../../execution/evidence/t03-markdown-concurrency-recovery.json)。未重跑 `python -m scripts.tasks contract`：现有 gitignored 报告 SHA256 与 T02 捕获一致，且已含 `checks.concurrent_fixture_writes`、`concurrency`、`fixture_after_shutdown` 及中文/frontmatter/wiki-link。未改 T04 证据、ADR 或 T04+ 任务目录，未回退 T06/T07。
6. Check 收口：并发 worker 超时或第二进程握手失败时 abort 已启动的 probe；返回值不再写入绝对 sandbox 路径；`docs/VERIFICATION.md` 的 T03 叙述改为每个 profile 各自双 probe、报告不合并。

## 验证

- `python ./.trellis/scripts/task.py validate 09-12-t03-markdown-concurrency-recovery`：exit 0
- `python -m unittest tests.test_core.ObservationTests tests.test_core.ResultTests tests.test_core.InteropTests tests.test_core.ProfileTests -v`：31 passed，exit 0
- `python -m unittest discover -s tests -v`：60 ran，59 passed，1 error。失败项为 `test_repository_phase_order` → `RuntimeError: A later task was completed before G0`。未回退 T06/T07。
- `python -m scripts.tasks unit`：exit 1，`ERROR: A later task was completed before G0`
- `cargo fmt --all -- --check`：exit 0
- `cargo test --workspace --locked --offline`：exit 0（bmdock-probe 5，bmdock-app 14）
- `cargo check --workspace --locked --offline`：exit 0
- `git diff --check`：exit 0
- `python -m scripts.tasks contract` / `just contract*`：未重跑；沿用 `artifacts/release.contract.json` SHA256 `520832e513d3b9142a05cbdee58f2b15ad91a0a6086f256e7f26089a3bc97a6d` 与 `artifacts/main-preview.contract.json` SHA256 `2e2ab0489fa1123edca545f842cf67b5659a91a0cd76d39e68d7beedb1fca645`
- `npm run build`：跳过（未改 `apps/`）
- `just setup`：未运行

## 范围限制

当前控制协议没有故障注入或取消控制方法。上述五项恢复场景保持 `UNVERIFIED`，不以成功 envelope、进程 exit 0 或不同目标并发成功推断恢复。后续 T12、T16、T17 负责产品级冲突协调和恢复链。G0 仍为 `in_progress`。
