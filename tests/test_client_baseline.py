"""Offline behavioral tests for the C01 evidence harness."""
from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from scripts import client_baseline as baseline
from scripts.core import create_sandbox, fingerprint


class CorpusTests(unittest.TestCase):
    def test_all_declared_sizes_are_exact_and_reproducible(self):
        for scale, (count, size) in baseline.SCALES.items():
            for index in {0, min(10, count - 1), count - 1}:
                with self.subTest(scale=scale, index=index):
                    first = baseline.note_bytes(index, size, count)
                    self.assertEqual(len(first), size)
                    self.assertEqual(first, baseline.note_bytes(index, size, count))
                    self.assertIn("custom_metadata: 保留原始中文", first.decode("utf-8"))

    def test_corpus_hashes_match_and_mutation_is_visible(self):
        with tempfile.TemporaryDirectory() as root:
            first = create_sandbox(Path(root), "release")
            second = create_sandbox(Path(root), "release")
            before = baseline.populate_fixture(first, "notes-100")
            self.assertEqual(before, baseline.populate_fixture(second, "notes-100"))
            self.assertEqual(before["note_count"], 100)
            self.assertEqual(before["total_bytes"], 409600)
            self.assertEqual(len({Path(row["path"]).name for row in before["files"]}), 1)
            (first / "vault" / baseline.note_path(99)).write_text("changed", encoding="utf-8")
            self.assertNotEqual(before["files_sha256"], baseline.corpus_snapshot(first / "vault")["files_sha256"])
            with self.assertRaisesRegex(ValueError, "must be empty"):
                baseline.populate_fixture(first, "notes-100")

    def test_thirty_independent_labels_include_nonliteral_paraphrases(self):
        labels = baseline.query_labels()
        self.assertEqual(len(labels), 30)
        self.assertEqual(len({row["id"] for row in labels}), 30)
        for category in ("literal", "chinese_mixed", "paraphrase"):
            self.assertEqual(sum(row["category"] == category for row in labels), 10)
        for index, row in enumerate(labels[20:]):
            self.assertEqual(row["expected_identifiers"], [baseline.note_identifier(index)])
            self.assertNotIn(row["query"], baseline.note_bytes(index, 4096, 100).decode("utf-8"))

    def test_ordering_probe_strong_candidate_sorts_last(self):
        weak = baseline.note_bytes(20, 4096, 100).decode("utf-8").lower()
        strong = baseline.note_bytes(99, 4096, 100).decode("utf-8").lower()
        self.assertGreater(strong.count("orderingprobe"), weak.count("orderingprobe"))
        self.assertGreater(baseline.note_identifier(99), baseline.note_identifier(20))


class ContractTests(unittest.TestCase):
    def test_guidance_never_becomes_empty_search_success(self):
        response = {"result": {"content": [{"type": "text", "text": "No matching note. Try a search."}]}}
        self.assertEqual(baseline.decode_tool(response)[0], "guidance_text")
        with self.assertRaisesRegex(ValueError, "Expected observed search"):
            baseline.search_payload(response)

    def test_structured_empty_error_and_json_shapes_stay_distinct(self):
        empty = {"result": {"structuredContent": {"result": {"results": [], "total": 0}}}}
        self.assertEqual(baseline.search_payload(empty)["results"], [])
        self.assertEqual(baseline.decode_tool({"error": {"kind": "policy"}})[0], "control_error")
        self.assertEqual(baseline.decode_tool({"result": {"isError": True}})[0], "tool_error")
        encoded = {"result": {"content": [{"type": "text", "text": json.dumps({"results": []})}]}}
        self.assertEqual(baseline.decode_tool(encoded)[0], "json_text")

    def test_crlf_normalization_is_distinct_from_disk_fidelity(self):
        source = baseline.note_bytes(0, 4096, 100)
        text = source.decode("utf-8").replace("\r\n", "\n")
        response = {"result": {"structuredContent": {"content": text}}}
        observed = baseline.source_comparison(response, source)
        self.assertFalse(observed["physical_bytes_equal"])
        self.assertTrue(observed["equal_after_crlf_to_lf"])
        self.assertTrue(observed["delivered_has_frontmatter"])
        self.assertTrue(observed["delivered_has_chinese"])
        self.assertEqual(response["result"]["structuredContent"]["content"], text)

    def test_full_read_is_unsliced_and_identity_contains_all_arguments(self):
        record = {"scale": "notes-100", "profile": "release", "corpus": {"files_sha256": "abc"}}
        scenario = baseline.scenarios(record)[0]
        self.assertTrue(scenario["args"]["include_frontmatter"])
        self.assertNotIn("start_line", scenario["args"])
        self.assertNotIn("end_line", scenario["args"])
        first = baseline.request_identity(record, 1, scenario)
        second = baseline.request_identity(record, 2, scenario)
        self.assertNotEqual(fingerprint(first), fingerprint(second))
        self.assertEqual(first["arguments"], scenario["args"])

    def test_structured_missing_read_never_passes_timing(self):
        scenario = baseline.scenarios({"scale": "notes-100"})[0]
        for payload in ({"error": "NOTE_NOT_FOUND", "content": None},
                        {"title": None, "permalink": None, "file_path": None, "content": None, "frontmatter": None}):
            response = {"result": {"structuredContent": {"result": payload}}}
            with self.subTest(payload=payload), self.assertRaises(ValueError):
                baseline.validate_observation(scenario, response, b"source")

    def test_manifest_drift_is_rejected_before_execution(self):
        manifest = baseline.measurement_manifest()
        baseline.validate_manifest(manifest)
        manifest["samples"]["warm"] = 1
        with self.assertRaisesRegex(ValueError, "drifted at samples"):
            baseline.validate_manifest(manifest)


class MeasurementTests(unittest.TestCase):
    def test_protocol_mismatch_aborts_owned_probe(self):
        from unittest.mock import Mock
        probe = Mock(connected={"server": {"protocolVersion": "wrong"}})
        with patch.object(baseline, "Probe", return_value=probe), \
             patch.object(baseline, "engine_python", return_value=Path("python")), \
             patch.object(baseline, "isolated_env", return_value={}):
            with self.assertRaises(AssertionError):
                baseline.start_probe({"profile": "release"}, Path("unused"))
        probe.abort.assert_called_once()

    def test_nearest_rank_p95_uses_rank_29_for_thirty_samples(self):
        summary = baseline.summarize(list(range(1, 31)))
        self.assertEqual(summary, {"n": 30, "median_ms": 15.5, "p95_ms": 29})
        for invalid in ([], [float("nan")], [-1], [float("inf")]):
            with self.assertRaises(ValueError):
                baseline.summarize(invalid)

    def test_frozen_protocol_reports_unavailable_not_zero(self):
        manifest = baseline.measurement_manifest()
        self.assertEqual(manifest["samples"]["cold"], 5)
        self.assertEqual(manifest["samples"]["warm"], 30)
        self.assertEqual(manifest["typing"]["native_p95_ms"], {"large-1mib": 50, "large-5mib": 100})
        self.assertEqual(manifest["availability"]["native_ui"]["status"], "unavailable")
        self.assertNotEqual(manifest["profiles"]["release"], manifest["profiles"]["main-preview"])

    def test_native_offsets_use_normalized_textarea_coordinates(self):
        for scale, plan in baseline.typing_insertions().items():
            source = baseline.note_bytes(0, baseline.SCALES[scale][1], 1).decode("utf-8")
            api_text = source.replace("\r\n", "\n").replace("\r", "\n")
            expected_length = len(api_text.encode("utf-16-le")) // 2
            self.assertEqual(plan["textarea_api_utf16_length"], expected_length)
            self.assertGreater(plan["source_utf16_length"], expected_length)
            self.assertEqual(len(plan["samples"]), 30)
            self.assertTrue(all(0 <= row["utf16_offset"] <= expected_length for row in plan["samples"]))

    def test_coverage_rejects_invalidated_or_incomplete_measurements(self):
        manifest = baseline.measurement_manifest()
        good = {"manifest_sha256": fingerprint(manifest), "profile": "release", "engine_sha": manifest["profiles"]["release"],
                "status": "measured_selected_scenarios", "physical_corpus_unchanged": True, "samples": []}
        for change in ({"physical_corpus_unchanged": False}, {"status": "incomplete"}, {"failure": {"message": "failed"}}, {"engine_sha": "wrong"}):
            with self.subTest(change=change), patch.object(baseline, "read_json", side_effect=[manifest, {**good, **change}]):
                with self.assertRaises(ValueError):
                    baseline.coverage(Path("manifest"), [Path("report")], baseline.ROOT / "artifacts/client-baseline/not-written.json")

    def test_evidence_destination_cannot_target_vault_or_repository_source(self):
        self.assertEqual(baseline.confined_output(baseline.ROOT / "artifacts/client-baseline/run.json"),
                         baseline.ROOT / "artifacts/client-baseline/run.json")
        with self.assertRaises(ValueError):
            baseline.confined_output(baseline.ROOT / "scripts/overwrite.py")

    def test_measurement_restarts_each_cold_sample_and_uses_one_warm_session(self):
        class FakeProbe:
            connected = {"childPid": 123}
            process = type("Process", (), {"pid": 456})()

            def __init__(self):
                self.calls = 0
                self.closed = False
                self.transcript = []

            def raw(self, method, params):
                self.calls += 1
                return {"result": {"structuredContent": {"content": "source", "permalink": baseline.note_identifier(0)}}}

            def close(self):
                self.closed = True
                return {"event": "shutdown"}

            def abort(self):
                self.closed = True

        instances = []

        def start(*args):
            probe = FakeProbe()
            instances.append(probe)
            return probe, 7.0

        record = {"profile": "release", "engine_sha": "pin", "scale": "large-1mib",
                  "corpus": {"files_sha256": "hash"},
                  "measurement_manifest": {"measurement_boundary": "direct MCP"}}
        output = baseline.ROOT / "artifacts/client-baseline/not-written.json"
        with patch.object(baseline, "load_fixture", return_value=(record, Path("unused"))), \
             patch.object(baseline, "verify_engine"), patch.object(baseline, "start_probe", side_effect=start), \
             patch.object(baseline, "wait_index", return_value={"status": "ready"}), \
             patch.object(baseline, "corpus_snapshot", return_value=record["corpus"]), \
             patch.object(baseline, "peak_rss", return_value={"bytes": 123}), \
             patch.object(Path, "read_bytes", return_value=b"source"), \
             patch.object(baseline, "write_json"):
            result = baseline.measure(Path("unused"), output, ["read-full"])
        self.assertEqual(len(instances), 7)  # preparation + 5 cold + 1 warm
        self.assertEqual([item.calls for item in instances], [0, 1, 1, 1, 1, 1, 31])
        self.assertTrue(all(item.closed for item in instances))
        self.assertEqual(len(result["samples"]), 35)
        self.assertEqual(result["summary"]["read-full/cold"]["n"], 5)
        self.assertEqual(result["summary"]["read-full/warm"]["n"], 30)


if __name__ == "__main__":
    unittest.main()
