from __future__ import annotations

import json
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[1]


def recipe_body(justfile: str, name: str) -> str:
    lines = justfile.splitlines()
    body: list[str] = []
    capturing = False
    for line in lines:
        if capturing:
            if line.startswith("#") or line.startswith(" ") or line.startswith("\t") or line.strip() == "":
                body.append(line)
                continue
            break
        if line == f"{name}:" or line.startswith(f"{name}:"):
            capturing = True
    return "\n".join(body)


class DesktopShellTests(unittest.TestCase):
    def test_html_lang_is_zh_cn(self):
        html = (ROOT / "apps/bmdock-desktop/index.html").read_text(encoding="utf-8")
        self.assertIn('lang="zh-CN"', html)

    def test_just_dev_switches_to_tauri_and_build_stays_probe(self):
        justfile = (ROOT / "justfile").read_text(encoding="utf-8")
        dev = recipe_body(justfile, "dev")
        build = recipe_body(justfile, "build")
        contract = recipe_body(justfile, "contract")
        contract_main = recipe_body(justfile, "contract-main")
        tauri_dev = recipe_body(justfile, "tauri-dev")
        tauri_build = recipe_body(justfile, "tauri-build")
        self.assertIn("tauri-dev", dev)
        self.assertNotIn('main(["dev"', dev)
        self.assertIn('main(["build"])', build)
        self.assertNotIn("tauri", build.lower())
        self.assertIn('main(["contract", "release"])', contract)
        self.assertIn('main(["contract", "main-preview"])', contract_main)
        self.assertIn("tauri:dev", tauri_dev)
        self.assertIn("tauri:build", tauri_build)

    def test_agent_docs_match_just_dev_tauri_and_build_probe(self):
        for name in ("AGENTS.md", "CLAUDE.md"):
            text = (ROOT / name).read_text(encoding="utf-8")
            self.assertIn("just tauri-dev", text)
            self.assertIn("just tauri-build", text)
            self.assertIn("just contract", text)
            self.assertIn("just dev-main", text)
            self.assertRegex(
                text,
                r"(?is)just dev.{0,300}(tauri-dev|tauri 桌面|desktop shell|desktop layout)",
                msg=f"{name} must describe just dev as the Tauri desktop shell",
            )
            self.assertRegex(
                text,
                r"(?is)just build.{0,240}(bmdock-probe|g0 probe|g0 探针)",
                msg=f"{name} must describe just build as the G0 probe",
            )

    def test_ci_still_builds_g0_probe_via_just_build(self):
        workflow = (ROOT / ".github/workflows/ci.yml").read_text(encoding="utf-8")
        self.assertIn("just build", workflow)

    def test_chinese_nav_catalog_and_no_i18n_dependency(self):
        catalog = (ROOT / "apps/bmdock-desktop/src/i18n.ts").read_text(encoding="utf-8")
        self.assertIn('export const LOCALE = "zh-CN"', catalog)
        self.assertIn("工作台", catalog)
        self.assertIn("运行状态", catalog)
        self.assertIn("预检", catalog)
        self.assertIn("说明", catalog)
        package = json.loads((ROOT / "apps/bmdock-desktop/package.json").read_text(encoding="utf-8"))
        deps = {**package.get("dependencies", {}), **package.get("devDependencies", {})}
        for name in deps:
            self.assertNotIn("i18n", name.lower())

    def test_landmarks_empty_error_and_no_writes(self):
        app = (ROOT / "apps/bmdock-desktop/src/App.tsx").read_text(encoding="utf-8")
        shell = (ROOT / "apps/bmdock-desktop/src/shell.ts").read_text(encoding="utf-8")
        styles = (ROOT / "apps/bmdock-desktop/src/styles.css").read_text(encoding="utf-8")
        catalog = (ROOT / "apps/bmdock-desktop/src/i18n.ts").read_text(encoding="utf-8")
        main = (ROOT / "apps/bmdock-desktop/src/main.tsx").read_text(encoding="utf-8")
        self.assertIn("<header", app)
        self.assertIn("<nav", app)
        self.assertIn("<main", app)
        self.assertIn('href="#main"', app)
        self.assertIn("skip-link", app)
        self.assertIn("aria-current", app)
        self.assertIn("aria-label", app)
        self.assertIn('data-state="empty"', app)
        self.assertIn('data-state="error"', app)
        self.assertIn("workbenchErrorTitle", app)
        self.assertIn("capabilities.policy.project", app)
        self.assertIn(":focus-visible", styles)
        self.assertIn("document.documentElement.lang", main)
        self.assertIn("emptyBadge", catalog)
        self.assertIn("errorBadge", catalog)
        self.assertIn("unexpectedCapabilities", catalog)
        self.assertIn("unexpectedRuntime", catalog)
        self.assertIn("unexpectedPreflight", catalog)
        self.assertIn("unexpectedDiscovery", catalog)
        self.assertIn("discoveryEmptyTitle", catalog)
        self.assertIn('t("unexpectedCapabilities")', shell)
        self.assertIn('t("unexpectedRuntime")', shell)
        self.assertIn("get_capabilities", shell)
        self.assertIn("get_runtime_state", shell)
        self.assertIn("run_preflight", shell)
        self.assertIn("discover_config", shell)
        self.assertIn('t("unexpectedPreflight")', shell)
        self.assertIn('t("unexpectedDiscovery")', shell)
        self.assertNotIn("select_project", shell)
        self.assertNotIn("callTool", shell)
        self.assertNotIn("call_tool", shell)
        self.assertNotIn("start_supervisor", shell)
        self.assertNotIn("stop_supervisor", shell)
        self.assertNotIn("select_project", app)
        self.assertNotIn("selectFixtureProject", app)
        self.assertNotIn("callTool", app)
        self.assertNotIn("start_supervisor", app)
        self.assertNotIn("stop_supervisor", app)
        self.assertIn("preflight", app)
        self.assertIn("discoveryEmptyTitle", app)
        ipc = (ROOT / "apps/bmdock-desktop/src/ipc.ts").read_text(encoding="utf-8")
        self.assertIn("run_preflight", ipc)
        self.assertIn("discover_config", ipc)
        self.assertNotIn('command: "call_tool"', ipc)
        self.assertNotIn('"call_tool"', ipc)
        self.assertIn("host.engine_spawned", app)
        self.assertIn("host.files_written", app)
        self.assertIn("arbitrary_paths_allowed", ipc)
        self.assertIn("raw_call_tool_allowed", ipc)


if __name__ == "__main__":
    unittest.main()
