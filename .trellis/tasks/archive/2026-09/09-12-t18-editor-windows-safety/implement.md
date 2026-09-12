# T18 实施记录：编辑器内容安全与 Windows 体验

本文件是 T18 的执行记录。未实现 T19–T40，未改 T05–T17/G0，未 git commit。未改父任务目录。

## 交付

- 内容安全辅助：`apps/bmdock-desktop/src-tauri/src/content_safety.rs`、`apps/bmdock-desktop/src/contentSafety.ts`
- Fixture `save_draft` / `write_note` / `edit_note` 精确 UTF-8 字节往返（含 CRLF）
- zh-CN 工作台：labeled textarea + `<pre>` 纯文本预览；`unsafe_html_present` / `executed=false`
- 含 `<script>`、`<img onerror>`、`[[欢迎]]` 的正文在 T14/T15 夹具路径原文落盘
- CRLF 规范成 LF 不得标为 `disk_verified`
- 证据：`execution/evidence/t18-editor-windows-safety.json`

## AC 映射

- AC46：永不执行 HTML；textarea + `<pre>`；无 `dangerouslySetInnerHTML`
- AC21：zh-CN 编辑器空态 / 错误 / 就绪；夹具 CRLF 精确字节往返；native GUI / IME `UNVERIFIED`
- AC57：label、键盘聚焦、`:focus-visible`；不主张 T39

## 验证命令与退出码

| 命令 | exit |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t18-editor-windows-safety` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0（bmdock-app 125 + bmdock-probe 5） |
| `cargo check --workspace --locked --offline` | 0（既有 T07 dead_code） |
| `npm run build`（`apps/bmdock-desktop`） | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0（12 passed） |
| `python -m unittest discover -s tests -v` | 1（71 ok，1 ERROR phase order） |
| `python -m scripts.tasks unit` | 1（G0 vs T05+，未回退） |
| `just contract*` | 未运行 |
| native GUI / IME | 未运行 |

## UNVERIFIED

native GUI 会话、IME 输入、真实 vault、hosted CI、T39 帮助完备性。cargo test / npm build 不是原生窗口输入。
