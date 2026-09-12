# Repository Guidelines

<!-- TRELLIS:START -->
# Trellis Instructions

These instructions are for AI assistants working in this project.

This project is managed by Trellis. The working knowledge you need lives under `.trellis/`:

- `.trellis/workflow.md` — development phases, when to create tasks, skill routing
- `.trellis/spec/` — package- and layer-scoped coding guidelines (read before writing code in a given layer)
- `.trellis/workspace/` — per-developer journals and session traces
- `.trellis/tasks/` — active and archived tasks (PRDs, research, jsonl context)

If a Trellis command is available on your platform (e.g. `/trellis:finish-work`, `/trellis:continue`), prefer it over manual steps. Not every platform exposes every command.

If you're using Codex or another agent-capable tool, additional project-scoped helpers may live in:
- `.agents/skills/` — reusable Trellis skills
- `.codex/agents/` — optional custom subagents

Managed by Trellis. Edits outside this block are preserved; edits inside may be overwritten by a future `trellis update`.

<!-- TRELLIS:END -->

---

## Project Overview

BMDock is a desktop workbench for [Basic Memory](https://github.com/basicmachines-co/basic-memory).
The target production stack is:
- **Tauri 2** (native windowing and desktop shell)
- **React + TypeScript** (frontend UI)
- **Rust + rmcp (`=3.2.0`)** (IPC, MCP client, engine process supervisor)
- **Official Basic Memory** (upstream Python MCP memory server)

### Current Delivery Stage: P0 / G0 Technical Verification
The repository is strictly in the **P0 / G0 verification stage** (`execution/status.json`). It delivers verified developer tooling, capability probes, and smoke tests. It does **not** yet contain:
- A Tauri desktop installer or bundle
- A React web or desktop UI (`apps/` and `packages/` do not exist yet)
- Direct user IPC or raw `callTool` interfaces

Desktop product development starts in Task **T05** (Phase P1 / Gate G1) after Gate G0 passes.

### Core Data Boundaries
- The probe only connects to an ephemeral fixture project (`bmdock-fixture`) inside `.work/g0/`.
- The probe must **never** connect to or modify a user's real Obsidian vault, global Basic Memory configuration, or AI agent memory vaults.
- The project manages two immutable upstream engine profiles in `compatibility/profiles.json`:
  1. `release`: Tag `v0.23.2` (commit `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`), expecting 21 MCP tools.
  2. `main-preview`: Development snapshot (commit `3452c821d76c083823d020984d71e06904a1ff1e`), expecting 27 MCP tools.
- Never mix, merge, or average these two engine profiles into a single contract.

---

## Architecture & Data Flow

BMDock uses a three-tier isolated architecture:

```text
[ justfile ]
     │
     ▼
[ scripts/tasks.py ] ──(CLI dispatcher, env checks, AST validation)
     │
     ├─ [ .work/engines/<profile> ] (upstream engine checkout, uv-managed Python 3.12.12)
     ├─ [ .work/g0/<sandbox> ]      (isolated temporary sandbox with scrubbed env)
     └─ [ crates/bmdock-probe ]     (Rust binary: rmcp client, process supervisor, policy gatekeeper)
           │
           ├─ Spawns child process: scripts/engine_worker.py serve
           ├─ rmcp stdio JSON-RPC framing (protocol version 2025-11-25, client BMDock-G0)
           └─ Validates request whitelist & bmdock-fixture project policy
```

### Component Roles
1. **Host Task Harness (`scripts/tasks.py`, `scripts/core.py`)**:
   Standard library Python orchestrator. Manages sandbox provisioning, environment scrubbing, engine checkouts, unit testing, and CI pipelines without third-party dependencies.
2. **Probe Client Controller (`scripts/probe.py`)**:
   Spawns `bmdock-probe`, sends JSON-line control requests via stdin, reads JSON-line events via stdout, and executes the 7-check contract smoke test suite.
3. **Rust Probe Binary (`crates/bmdock-probe/src/main.rs`)**:
   Supervises the engine process. Enforces a strict whitelist of allowed MCP methods (`tools/list`, `resources/list`, etc.) and tools (`read_note`, `write_note`, etc.). Rejects any write outside `bmdock-fixture`.
4. **Engine Worker Bridge (`scripts/engine_worker.py`)**:
   Runs inside the isolated engine Python environment. Operates in `inventory` mode (traverses the Click/Typer CLI tree and extracts OpenAPI schemas) or `serve` mode (invokes `basic_memory.cli.main.app()` over stdio).

### Data Flow for Contract Verification
1. `scripts/tasks.py` calls `probe.py:run_contract(profile_id)`.
2. `core.py:create_sandbox()` creates an ephemeral `.work/g0/<profile>-*` directory containing empty `vault/`, `config/`, `home/`, `tmp/`, and `cache/` folders.
3. `probe.py` invokes `engine_worker.py inventory` to capture static CLI commands and OpenAPI routes.
4. `probe.py` launches `bmdock-probe`, which starts `engine_worker.py serve` and performs the `rmcp` handshake.
5. `probe.py` sends discovery and write requests over stdin as JSON lines.
6. `bmdock-probe` validates policy, proxies requests through `rmcp` to the engine, and streams JSON results to stdout.
7. `probe.py:wait_note()` polls the physical filesystem in `sandbox/vault/*.md` for a unique sentinel string.
8. `probe.py` closes stdin; `bmdock-probe` issues `service.cancel()`, waits for child process exit (30s timeout each), and outputs a shutdown receipt.

---

## Key Directories

| Directory | Purpose | Key Content |
|---|---|---|
| `crates/bmdock-probe/` | Rust G0 probe binary source | `src/main.rs`, `Cargo.toml`, `CLAUDE.md` |
| `scripts/` | Host automation and test harness (zero dependencies) | `tasks.py`, `core.py`, `probe.py`, `engine_worker.py`, `CLAUDE.md` |
| `tests/` | Pure Python unit tests | `test_core.py`, `test_cli_inventory.py`, `CLAUDE.md` |
| `compatibility/` | Immutable engine profiles and tool baselines | `profiles.json`, `CLAUDE.md` |
| `execution/` | Machine-readable gate tracking and CI evidence | `status.json`, `evidence/*.json`, `CLAUDE.md` |
| `docs/` | Architectural decisions, evidence logs, and handoff criteria | `SOURCES.md`, `VERIFICATION.md`, `IMPLEMENTATION.md`, `G0_HANDOFF.md` |
| `.trellis/` | Trellis workflow configuration and guidelines | `workflow.md`, `spec/`, `workspace/`, `tasks/` |
| `.github/workflows/` | Continuous integration workflows | `ci.yml`, `CLAUDE.md` |

---

## Development Commands

All developer tasks run through `just` (configured with `python -c` as recipe shell) or directly via `python -m scripts.tasks`:

### Environment & Setup
```bash
# Check required host tools (Python >= 3.12, git, uv, cargo, rustc, just)
just doctor
python -m scripts.tasks doctor

# Download and sync immutable engine checkouts into .work/engines/ (ONLY network step)
just setup
python -m scripts.tasks setup

# Regenerate Cargo.lock for manual review and commit
just lock
python -m scripts.tasks lock
```

### Build & Run
```bash
# T08 desktop layout shell (same as just tauri-dev). Not a production write UI.
just dev
just tauri-dev
just tauri-build  # desktop build entry; not an installer

# CI still uses just build; do not switch this recipe to Tauri.
just build        # G0 probe: target/release/bmdock-probe(.exe); not Tauri
python -m scripts.tasks build

# Interactive main-preview probe session in an ephemeral sandbox
just dev-main     # against main-preview profile (commit 3452c821)
```

### Testing & Validation
```bash
# Run pure Python unit tests and AST syntax validation
just ci-unit
python -m scripts.tasks unit

# Run full local CI pipeline (unit tests, clippy, cargo test/build, live contract smoke tests)
just ci
python -m scripts.tasks ci

# Run live engine contract smoke test against specific profile
just contract       # release profile
just contract-main  # main-preview profile

# Check gate status in execution/status.json (returns exit code 2 if gates are incomplete)
just gate
python -m scripts.tasks gate
```

---

## Code Conventions & Common Patterns

### Naming Conventions
- **Rust**: `snake_case` for functions/variables, `PascalCase` for structs/enums, `SCREAMING_SNAKE_CASE` for constants (`RPC_TIMEOUT`, `MAX_CONTROL_LINE`).
- **Python**: PEP 8 style. `snake_case` for functions/variables, `PascalCase` for classes (`Probe`), `SCREAMING_SNAKE_CASE` for module constants (`ROOT`, `PROJECT`, `MARKER`, `SYSTEM_ENV`).
- **Tests**:
  - Python: Files `tests/test_*.py`, test classes `<Subject>Tests` (`SandboxTests`, `ProfileTests`), methods `test_<scenario_or_behavior>`.
  - Rust: `#[test]` functions named `<scenario>_<expected_outcome>` (e.g. `unknown_method_is_denied`, `fixture_write_requires_explicit_project`).

### Error Handling & Fail-Closed Policy
- **Rust Error Type**:
  Uses `Result<T, Box<dyn Error + Send + Sync>>`. Errors emitted on control lines use distinct categories:
  - `"policy"`: Request method or tool is not in the whitelist.
  - `"rpc_or_transport"`: Upstream rmcp communication error.
  - `"timeout_unknown"`: RPC operation exceeded 90s. The probe terminates the session immediately; never retry non-idempotent writes blindly.
  - `"schema"`: Invalid JSON payload on control channel.
- **Python Error Guards**:
  - `tasks.py:require(tool)`: Fails immediately if an external binary (`cargo`, `git`, `uv`, etc.) is missing from `PATH`.
  - `core.py:verify_sandbox(dir)`: Fails if the sandbox marker is missing, if symlinks escape the sandbox root, or if unauthorized database/vault configurations are detected.
  - `core.py:classify_tool_result(res)`: Cautious categorization (`tool_error` > `rejected` > `accepted_unverified` > `unclassified`).
- **Semantic Rule: Text Success != Disk Persistence**:
  Text containing phrases like "Saved successfully" or a standard MCP 200 return envelope does **not** prove data persistence. Always verify actual physical files in `sandbox/vault/*.md` via `wait_note()`.

### Async & Process Management Patterns
- **No `shell=True`**: All subprocess invocations pass explicit argument lists `argv: list[str]` (`core.run()`, `subprocess.Popen`).
- **Process Ownership in Rust**:
  `bmdock-probe` retains direct ownership of the child process. It sets `child.kill_on_drop(true)` and enforces bounded line reading (`(&mut control).take((MAX_CONTROL_LINE + 1) as u64).read_until(...)`) to prevent memory exhaustion.
- **Two-Stage Shutdown**:
  The probe shuts down the SDK transport and child process independently with dedicated 30-second budgets (`service.cancel()`, `child.wait()`).

### State Isolation & Immutability Patterns
- **Fresh Sandboxes**: Every test or dev run creates a unique, throwaway sandbox under `.work/g0/` using `tempfile.TemporaryDirectory` or timestamped directory names.
- **Environment Sanitization (`core.py:isolated_env`)**:
  - Whitelists minimal system variables (`PATH`, `SYSTEMROOT`, `TEMP`).
  - Strips API credentials (`OPENAI_API_KEY`, `LOGFIRE_TOKEN`, etc.) and cloud routing flags (`BASIC_MEMORY_FORCE_CLOUD`).
  - Injects offline flags: `BASIC_MEMORY_AUTO_UPDATE=false`, `BASIC_MEMORY_SEMANTIC_SEARCH_ENABLED=false`, `BASIC_MEMORY_FORCE_LOCAL=true`, `HF_HUB_OFFLINE=1`.
- **Atomic File Persistence (`core.py:write_json`)**:
  Writes JSON data to a sibling `.tmp` file, flushes with `os.fsync()`, and atomically renames to the destination.
- **Phase Order Guard (`tasks.py:check_source`)**:
  Inspects `execution/status.json` and programmatically halts execution if tasks T05+ are marked `completed` before gate G0 passes.

---

## Important Files

| File | Significance |
|---|---|
| `crates/bmdock-probe/src/main.rs` | Probe binary entry point, rmcp loop, method/tool whitelists, process supervisor, and unit tests |
| `crates/bmdock-probe/Cargo.toml` | Manifest pinning `rmcp = "=3.2.0"`, `tokio`, and `serde` |
| `scripts/tasks.py` | Command dispatcher (`doctor`, `setup`, `lock`, `unit`, `ci`, `dev`, `contract`, `build`, `gate`) |
| `scripts/core.py` | Shared safety utilities: sandbox lifecycle, env scrubbing, pagination, atomic writes |
| `scripts/probe.py` | Python `Probe` controller class and end-to-end `run_contract` smoke test runner |
| `scripts/engine_worker.py` | Subprocess bridge executed inside the isolated engine venv for CLI inventory and MCP serving |
| `compatibility/profiles.json` | Immutable baseline defining engine repository, commits, tags, and expected MCP tools |
| `execution/status.json` | Single source of truth for development phases (P0–P7), gates (G0–G7), and tasks (T01–T40) |
| `execution/evidence/g0-smoke-03c2839.json` | Verified CI evidence log for dual-OS validation |
| `justfile` | Recipe interface forwarding commands to `python -m scripts.tasks` |
| `Cargo.toml` & `Cargo.lock` | Workspace definition and committed version 4 lockfile (`--locked` enforced in CI) |
| `rust-toolchain.toml` | Pinned Rust toolchain (channel `1.90.0`, minimal profile, `rustfmt`, `clippy`) |
| `.github/workflows/ci.yml` | GitHub Actions workflow for Ubuntu and Windows runners |

---

## Runtime/Tooling Preferences

BMDock enforces strict runtime separation:

### 1. Host Runtime
- **Python**: Version **>= 3.12** required on `PATH` as `python` (CI uses **3.13.5**).
  - **Zero Third-Party Dependencies**: The host harness and test runner use the Python Standard Library exclusively (`unittest`, `ast`, `json`, `subprocess`, `argparse`, `shutil`, `tempfile`).
  - Do **not** create a virtual environment or install `requirements.txt` for host scripts.
- **Rust Toolchain**: Pinned to **1.90.0** (`rust-toolchain.toml`) with minimal profile, `rustfmt`, and `clippy`. Cargo workspace `rust-version = "1.88"`, edition `2021`.
- **Task Runner**: `just` **1.40.0**.
- **Package Manager for Upstream Engines**: `uv` **0.12.11**.
- **JavaScript / Web Tooling**: **None**. Do not use Node.js, Bun, npm, or yarn at this stage.

### 2. Managed Engine Runtime (`.work/engines/`)
- Pinned strictly to **Python 3.12.12**, managed by `uv` using the upstream repository's committed `uv.lock`.
- Synchronized via `uv sync --frozen --no-dev --no-editable --python 3.12.12`.

### 3. Tooling Constraints
- `Cargo.lock` is committed to git. CI builds always enforce `--locked`.
- `just setup` is the **only** command permitted to access the network.
- Line endings are fixed to LF via `.gitattributes`.

---

## Testing & QA

### Test Frameworks
1. **Python Unit Tests (`tests/`)**:
   - Framework: Standard library `unittest`.
   - 51 tests in `test_core.py` and `test_cli_inventory.py`.
   - Zero third-party dependencies. Uses lightweight structural fakes rather than importing Click/Typer or mocking engines.
2. **Rust Unit Tests (`crates/bmdock-probe/src/main.rs`)**:
   - Framework: Standard Rust test runner (`cargo test`).
   - 5 unit tests validating the probe's request whitelist, fixture write boundary, discovery methods, and distinct `search`/`fetch` tools (raw `callTool` remains denied).
3. **AST & Phase Order Linter (`scripts/tasks.py:check_source`)**:
   - Uses `ast.parse()` to validate all `scripts/*.py` syntax.
   - Asserts that task phase progression in `execution/status.json` respects gate boundaries.
4. **Real-Engine Contract Smoke Suite (`scripts/probe.py:run_contract`)**:
   - Executes per-profile real-engine checks against official processes (reports are never merged):
     1. MCP Handshake & Protocol framing (`event: connected`, `protocolVersion` `2025-11-25`)
     2. Paginated tool/resource/template/prompt discovery and independent 21/27 tool baselines
     3. Distinguishable `policy` / `schema` / `rpc_or_transport` / MCP `isError` envelopes
     4. Note write classified `accepted_unverified`, then fixture disk observation (`wait_note`)
     5. Note readback and append idempotency
     6. Search/fetch identity (required `query` vs `id`; swapped calls are MCP `isError`)
     7. Clean two-stage shutdown verification
     Lost-response, cancel-after-accept, `timeout_unknown` injection, forced-kill, and disk-failure remain `UNVERIFIED`.

### Running Tests
```bash
# Run all Python unit tests with AST validation
just ci-unit
python -m scripts.tasks unit

# Run single Python test file or test case
python -m unittest tests/test_core.py
python -m unittest tests.test_core.SandboxTests.test_unique_sandbox_each_time

# Run all Rust unit tests
cargo test --workspace --locked

# Run single Rust unit test
cargo test -p bmdock-probe tests::fixture_write_requires_explicit_project -- --exact

# Run live contract smoke test against release profile (requires just setup + just build)
just contract

# Run full CI suite locally
just ci
```

### Coverage & Acceptance Expectations
- **100% Pass Rate**: Required across all unit tests and contract smoke checks on both Ubuntu and Windows.
- **Fail-Closed Capability Drift**: Any added or missing tools compared to `compatibility/profiles.json` immediately halts verification.
- **Clean Workspace Verification**: `git diff --exit-code` runs in CI to ensure no command modifies tracked files or invents lockfiles.
