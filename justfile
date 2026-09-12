# BMDock: portable developer commands. Python 3.12+ must be on PATH as `python`.
set shell := ["python", "-c"]

# List available commands.
default:
    import subprocess; raise SystemExit(subprocess.call(["just", "--list"]))

# Check tooling without starting Basic Memory or touching user configuration.
doctor:
    from scripts.tasks import main; raise SystemExit(main(["doctor"]))

# Explicit network operation: install both pinned engines into .work/engines.
setup:
    from scripts.tasks import main; raise SystemExit(main(["setup"]))

# Generate a real Rust lockfile, for review and commit (never implicit in CI).
lock:
    from scripts.tasks import main; raise SystemExit(main(["lock"]))

# Full CURRENT-stage CI: unit, fmt, clippy, Rust tests and both real-engine smoke suites.
ci:
    from scripts.tasks import main; raise SystemExit(main(["ci"]))

# Dependency-free unit checks; NOT a substitute for ci or a phase gate.
ci-unit:
    from scripts.tasks import main; raise SystemExit(main(["unit"]))

# T08 desktop shell. Same entry as just tauri-dev. Probe remains on just contract*.
dev:
    import subprocess; raise SystemExit(subprocess.call(["just", "tauri-dev"]))

# Same verification with the immutable main-preview profile.
dev-main:
    from scripts.tasks import main; raise SystemExit(main(["dev", "main-preview"]))

# Compile the G0 probe. CI still uses this recipe; T08 does not switch it to Tauri.
build:
    from scripts.tasks import main; raise SystemExit(main(["build"]))

# Real-engine contract smoke tests without the other CI layers.
contract:
    from scripts.tasks import main; raise SystemExit(main(["contract", "release"]))

contract-main:
    from scripts.tasks import main; raise SystemExit(main(["contract", "main-preview"]))

# Strict product gate report: returns nonzero while any G0-G7 gate is unpassed.
gate:
    from scripts.tasks import main; raise SystemExit(main(["gate"]))

# T05 desktop shell development entrypoint; independent from the P0 probe.
tauri-dev:
    import subprocess; raise SystemExit(subprocess.call(["npm", "--prefix", "apps/bmdock-desktop", "run", "tauri:dev"]))

tauri-build:
    import subprocess; raise SystemExit(subprocess.call(["npm", "--prefix", "apps/bmdock-desktop", "run", "tauri:build"]))
