from __future__ import annotations

import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from scripts.core import (PROJECT, classify_tool_result, create_sandbox, fingerprint,
                          inventory_delta, isolated_env, paginate, profile, read_json,
                          run, verify_sandbox, write_json)
from scripts.probe import redact, tool_arguments
from scripts import tasks


class ProfileTests(unittest.TestCase):
    def test_immutable_distinct_profiles(self):
        self.assertNotEqual(profile("release")["commit"], profile("main-preview")["commit"])
        self.assertEqual(profile("release")["version"], "0.23.2")
        self.assertEqual(len(profile("release")["expected_tools"]), 21)
        self.assertEqual(len(profile("main-preview")["expected_tools"]), 27)

    def test_no_arbitrary_profile_path(self):
        for name in ("../production", "main", "latest", "release; echo owned"):
            with self.subTest(name=name), self.assertRaises(ValueError):
                profile(name)

    def test_duplicate_discovery_rejected(self):
        with self.assertRaises(ValueError):
            inventory_delta(["read"], ["read", "read"])

    def test_drift_retains_both_directions(self):
        self.assertEqual(inventory_delta(["read", "old"], ["read", "new"]),
                         {"missing": ["old"], "added": ["new"]})

    def test_schema_hash_is_key_order_independent(self):
        self.assertEqual(fingerprint({"x": 1, "y": 2}), fingerprint({"y": 2, "x": 1}))

    def test_schema_hash_changes_with_semantics(self):
        self.assertNotEqual(fingerprint({"required": ["x"]}), fingerprint({"required": []}))


class SandboxTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)
        self.sandbox = create_sandbox(self.base, "release")

    def test_unique_sandbox_each_time(self):
        other = create_sandbox(self.base, "release")
        self.assertNotEqual(self.sandbox, other)

    def test_only_fixture_registered(self):
        config = read_json(self.sandbox / "config/config.json")
        self.assertEqual(set(config["projects"]), {PROJECT})
        self.assertEqual(Path(config["projects"][PROJECT]["path"]), self.sandbox / "vault")

    def test_valid_owned_sandbox(self):
        self.assertEqual(verify_sandbox(self.sandbox), self.sandbox)

    def test_missing_marker_is_rejected(self):
        with self.assertRaises(FileNotFoundError):
            verify_sandbox(self.base)

    def test_forged_marker_path_rejected(self):
        write_json(self.sandbox / ".bmdock-g0-sandbox.json", {"kind": "bmdock-g0", "root": str(self.base), "profile": "release"})
        with self.assertRaises(ValueError):
            verify_sandbox(self.sandbox)

    def test_external_vault_rejected(self):
        file = self.sandbox / "config/config.json"
        config = read_json(file)
        config["projects"][PROJECT]["path"] = str(self.base)
        write_json(file, config)
        with self.assertRaises(ValueError):
            verify_sandbox(self.sandbox)

    def test_extra_project_rejected(self):
        file = self.sandbox / "config/config.json"
        config = read_json(file)
        config["projects"]["production"] = {"path": str(self.base)}
        write_json(file, config)
        with self.assertRaises(ValueError):
            verify_sandbox(self.sandbox)

    def test_cloud_mode_rejected(self):
        file = self.sandbox / "config/config.json"
        config = read_json(file)
        config["projects"][PROJECT]["mode"] = "cloud"
        write_json(file, config)
        with self.assertRaises(ValueError):
            verify_sandbox(self.sandbox)

    def test_database_reference_rejected(self):
        file = self.sandbox / "config/config.json"
        config = read_json(file)
        config["database_url"] = "postgresql://production"
        write_json(file, config)
        with self.assertRaises(ValueError):
            verify_sandbox(self.sandbox)

    def test_autoupdate_rejected(self):
        file = self.sandbox / "config/config.json"
        config = read_json(file)
        config["auto_update"] = True
        write_json(file, config)
        with self.assertRaises(ValueError):
            verify_sandbox(self.sandbox)

    def test_symlink_guard_rejects_a_link(self):
        # Deterministic unit test of the guard; NOT a native junction acceptance test.
        with patch.object(Path, "is_symlink", return_value=True):
            with self.assertRaises(ValueError):
                verify_sandbox(self.sandbox)

    def test_no_provider_keys_or_ambient_routing(self):
        env = isolated_env(self.sandbox, {"PATH": "/tools", "OPENAI_API_KEY": "secret", "LOGFIRE_TOKEN": "secret",
                                          "BASIC_MEMORY_FORCE_CLOUD": "true", "PYTHONPATH": "/untrusted"})
        for key in ("OPENAI_API_KEY", "LOGFIRE_TOKEN", "BASIC_MEMORY_FORCE_CLOUD", "PYTHONPATH"):
            self.assertNotIn(key, env)
        self.assertEqual(env["PATH"], "/tools")
        self.assertEqual(env["BASIC_MEMORY_AUTO_UPDATE"], "false")
        self.assertEqual(env["BASIC_MEMORY_FORCE_LOCAL"], "true")
        self.assertTrue(Path(env["HOME"]).is_relative_to(self.sandbox))

    def test_atomic_json_roundtrip(self):
        target = self.sandbox / "new/report.json"
        write_json(target, {"中文": [1, 2]})
        self.assertEqual(read_json(target), {"中文": [1, 2]})
        write_json(target, {"replacement": True})
        self.assertEqual(read_json(target), {"replacement": True})

    def test_redacts_nested_sandbox_paths(self):
        value = {"items": [str(self.sandbox / "vault/a.md")]}
        self.assertEqual(redact(value, self.sandbox), {"items": ["<G0_SANDBOX>" + os.sep + "vault" + os.sep + "a.md"]})


class PaginationTests(unittest.TestCase):
    def test_follows_all_pages(self):
        calls = []
        def request(method, params):
            calls.append((method, params))
            return {"tools": [2]} if params else {"tools": [1], "nextCursor": "next"}
        self.assertEqual(paginate(request, "tools/list", "tools"), ([1, 2], 2))
        self.assertEqual(calls[1][1], {"cursor": "next"})

    def test_empty_final_page(self):
        self.assertEqual(paginate(lambda *_: {"tools": []}, "tools/list", "tools"), ([], 1))

    def test_loop_is_an_error(self):
        with self.assertRaisesRegex(ValueError, "repeated"):
            paginate(lambda *_: {"tools": [], "nextCursor": "x"}, "tools/list", "tools")

    def test_invalid_cursor_is_an_error(self):
        with self.assertRaises(ValueError):
            paginate(lambda *_: {"tools": [], "nextCursor": 5}, "tools/list", "tools")

    def test_truncation_is_an_error(self):
        i = iter(range(5))
        with self.assertRaisesRegex(ValueError, "partial"):
            paginate(lambda *_: {"tools": [], "nextCursor": str(next(i))}, "tools/list", "tools", max_pages=2)

    def test_wrong_result_shape_is_an_error(self):
        with self.assertRaises(ValueError):
            paginate(lambda *_: {"tools": {}}, "tools/list", "tools")

    def test_rpc_error_is_not_an_empty_page(self):
        def fail(*_):
            raise RuntimeError("transport failed")
        with self.assertRaises(RuntimeError):
            paginate(fail, "tools/list", "tools")


class ResultTests(unittest.TestCase):
    def test_text_success_does_not_imply_saved(self):
        self.assertEqual(classify_tool_result({"content": [{"text": "Saved successfully"}]}), "unclassified")

    def test_tool_error_has_precedence(self):
        self.assertEqual(classify_tool_result({"isError": True, "structuredContent": {"kind": "created"}}), "tool_error")

    def test_created_is_not_disk_verified(self):
        for kind in ("created", "updated"):
            self.assertEqual(classify_tool_result({"structuredContent": {"kind": kind}}), "accepted_unverified")

    def test_rejections_are_not_success(self):
        for kind in ("already_exists", "locked", "target_moved"):
            self.assertEqual(classify_tool_result({"structuredContent": {"kind": kind}}), "rejected")

    def test_unknown_discriminator_is_not_success(self):
        self.assertEqual(classify_tool_result({"structuredContent": {"kind": "new_future_state"}}), "unclassified")

    def test_only_advertised_arguments_are_forwarded(self):
        tool = {"name": "read", "inputSchema": {"properties": {"identifier": {}}, "required": ["identifier"]}}
        self.assertEqual(tool_arguments(tool, {"identifier": "x", "output_format": "json"}), {"identifier": "x"})

    def test_new_required_parameter_fails_closed(self):
        tool = {"name": "read", "inputSchema": {"properties": {"new_field": {}}, "required": ["new_field"]}}
        with self.assertRaisesRegex(ValueError, "required"):
            tool_arguments(tool, {"identifier": "x"})


class CommandTests(unittest.TestCase):
    def test_missing_tool_is_not_skipped(self):
        with patch("shutil.which", return_value=None), self.assertRaises(RuntimeError):
            tasks.require("cargo")

    def test_subprocess_has_no_shell(self):
        with patch("subprocess.run") as mocked:
            run(["python", "a file.py", "literal;not-command"])
        self.assertNotIn("shell", mocked.call_args.kwargs)
        self.assertTrue(mocked.call_args.kwargs["check"])

    def test_product_gate_not_pretended_passed(self):
        self.assertEqual(tasks.main(["gate"]), 2)

    def test_repository_phase_order(self):
        tasks.check_source()


if __name__ == "__main__":
    unittest.main()
