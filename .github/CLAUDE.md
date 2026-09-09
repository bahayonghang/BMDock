# .github

> [仓库根](../CLAUDE.md) › `.github`
> 上次扫描：2026-09-09 13:16 +08:00

当前仅 `workflows/ci.yml`。工作流名 `CI`；权限 `contents: read`；concurrency 按 ref 取消进行中的运行。

## Job `g0`

矩阵：`ubuntu-latest`、`windows-latest`，`fail-fast: false`，超时 45 分钟。

步骤顺序：

1. `actions/checkout`（pin SHA，v6；`persist-credentials: false`）
2. `actions/setup-python` Python **3.13.5**（宿主；与引擎 3.12.12 分离）
3. `pip install uv==0.12.11`
4. `rustup toolchain install 1.90.0 --profile minimal --component rustfmt --component clippy`
5. `cargo install just --version 1.40.0 --locked`
6. `cargo metadata --locked --no-deps` + `cargo fmt --all -- --check`（验证已提交锁与格式，不生成新锁）
7. `just setup`（唯一下载引擎步骤）
8. `just ci`
9. `just build`
10. `git diff --exit-code`（禁止 CI 改写 tracked 文件）
11. 始终上传 `artifacts/` 为 `g0-evidence-<os>`，保留 14 天

触发：`main` push、PR、`workflow_dispatch`。

## 约束

- Action 使用完整 commit SHA pin。
- 标准 CI 不跑 `just lock`，不 `cargo fmt` 写回。
- Windows 与 Ubuntu 都必须跑真实引擎 smoke；缺依赖即失败。
- 已复核成功 run：`34309026376`（代码 `03c2839`）。后续改动以对应新 CI 为准。

## 已知修复（写入文档，避免回归）

1. 宿主 Python 与 uv 引擎 Python 必须分开；Windows runner 曾缺 3.12.12 的 setup-python 包。
2. 浅克隆须补 fetch release tag，否则官方动态版本变成 `0.0.1.dev1+…`。
3. CLI 清单必须走命令公共 group API；`isinstance(click.Group)` 会丢掉 Typer 内置 Click 子树。

## 关联模块

- 命令实现：[scripts](../scripts/CLAUDE.md) · [justfile](../justfile)
- 证据落盘：[execution](../execution/CLAUDE.md)
