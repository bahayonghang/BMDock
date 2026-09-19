"""Offline C03 evidence checks; no engine import or live subprocess required."""
import copy
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

from scripts import client_queries as queries
from scripts.client_baseline import note_identifier, note_path, populate_fixture
from scripts.core import ROOT, create_sandbox


class QueryEvidenceTests(unittest.TestCase):
    def pair(self):
        request = {"name": "search_notes", "arguments": {"page_size": 50}}
        hits = [{"permalink": "corpus/z-last", "type": "entity", "score": 1.75},
                {"permalink": "corpus/a-first", "type": "observation", "score": -3.25}]
        raw = {"results": hits, "current_page": 1, "page_size": 50,
               "total": 0, "total_is_exact": False, "has_more": True}
        direct = {"request": request, "command_count": 1,
                  "response": {"structuredContent": {"result": raw}, "isError": False}}
        adapter = {"request": request, "command_count": 1,
                   "response": {"kind": "search_page", "engine_search": True,
                    "hits": [{"identifier": hit["permalink"], "result_kind": hit["type"], "score": hit["score"]} for hit in hits],
                    "page": 1, "page_size": 50, "next_cursor": "2", "has_more": True,
                    "total": 0, "total_is_exact": False}}
        return direct, adapter

    def test_order_scores_over_one_and_unknown_zero_are_preserved(self):
        result = queries.validate_pair(*self.pair())["semantics"]
        self.assertEqual(result["hits"][0]["identifier"], "corpus/z-last")
        self.assertEqual(result["hits"][0]["score"], 1.75)
        self.assertFalse(result["total_is_exact"])
        self.assertTrue(result["has_more"])

    def test_context_own_result_identity_does_not_become_owning_note(self):
        row = {"type": "relation", "relation_id": 7, "from_entity_external_id": "owner-uuid",
               "content": "中文" * 200, "score": 1.5}
        result = queries.hit_projection(row)
        self.assertEqual(result["identifier"], "relation:7")
        self.assertEqual(result["note_identifier"], "owner-uuid")
        self.assertEqual(result["excerpt"], "中文" * 120)
        self.assertEqual(result["score"], 1.5)

    def test_reordered_clamped_or_fabricated_exact_results_fail(self):
        for mutation in (lambda p: p["hits"].reverse(),
                         lambda p: p["hits"][0].update(score=1),
                         lambda p: p.update(total_is_exact=True),
                         lambda p: p.update(next_cursor=None)):
            direct, adapter = self.pair()
            mutation(adapter["response"])
            with self.assertRaises(AssertionError):
                queries.validate_pair(direct, adapter)

    def test_guidance_iserror_missingread_and_malformed_are_not_empty_success(self):
        for response in ({"structuredContent": {"result": "Guidance text"}},
                         {"isError": True, "structuredContent": {"result": {"results": []}}},
                         {"structuredContent": {"result": {"error": "NOTE_NOT_FOUND", "content": None}}},
                         {"structuredContent": {"result": {"results": None}}}):
            with self.subTest(response=response), self.assertRaises(AssertionError):
                queries.search_semantics({"response": response})

    def test_command_amplification_and_argument_drift_are_rejected(self):
        for mutation in (lambda a: a.update(command_count=2),
                         lambda a: a.update(request={"name": "search_notes", "arguments": {"page_size": 100}})):
            direct, adapter = self.pair()
            mutation(adapter)
            with self.assertRaises(AssertionError):
                queries.validate_pair(direct, adapter)

    def test_mutating_freshness_records_score_drift_but_still_rejects_stale_identity(self):
        direct, adapter = self.pair()
        adapter["response"]["hits"][0]["score"] = 1.8
        with self.assertRaises(AssertionError):
            queries.validate_pair(direct, adapter)
        result = queries.validate_pair(direct, adapter, compare_scores=False)
        self.assertFalse(result["scores_equal_observed"])
        self.assertFalse(result["scores_required_equal"])
        adapter["response"]["hits"][0]["identifier"] = "stale-note"
        with self.assertRaises(AssertionError):
            queries.validate_pair(direct, adapter, compare_scores=False)

    def test_full_read_compares_yamland_unicode_without_claiming_byte_equality(self):
        source = "---\r\ncustom: 中文\r\n---\r\nbody\r\n".encode()
        text = source.decode().replace("\r\n", "\n")
        request = {"name": "read_note", "arguments": {"include_frontmatter": True}}
        direct = {"request": request, "command_count": 1,
            "response": {"structuredContent": {"result": {"content": text, "permalink": note_identifier(0)}}}}
        adapter = {"request": request, "command_count": 1,
            "response": {"kind": "note_read", "body": text, "identifier": note_identifier(0)}}
        result = queries.validate_pair(direct, adapter, source)
        self.assertTrue(result["upstream_crlf_normalized"])
        self.assertFalse(result["source_bytes_equal"])
        adapter["response"]["body"] = "body"
        with self.assertRaises(AssertionError):
            queries.validate_pair(direct, adapter, source)

    def test_schedule_and_frozen_budget_do_not_relax_on_failure(self):
        self.assertEqual([queries.lane_order(i)[0] for i in range(4)], ["direct", "adapter", "direct", "adapter"])
        samples = [{"phase": phase, "lane": lane, "outer_elapsed_ms": value}
            for phase, count in (("cold", 5), ("warm", 30))
            for lane, value in (("direct", 100), ("adapter", 151)) for _ in range(count)]
        result = queries.paired_budget(samples)
        self.assertEqual(result["allowed_overhead_ms"], 50)
        self.assertFalse(result["within_frozen_budget"])
        with self.assertRaisesRegex(AssertionError, "sample count"):
            queries.paired_budget(samples[:-1])

    def test_query_operators_cjk_and_generation_are_not_normalized_away(self):
        request = queries.command({"profile": "release", "generation": 1},
                                  query='中文 AND "Foo BAR"', generation=7)
        self.assertEqual(request["args"]["query"], '中文 AND "Foo BAR"')
        self.assertEqual(request["args"]["request_generation"], 7)
        self.assertNotIn("page_size", request["args"])

    def test_readiness_waits_for_three_matching_semantics_not_first_visibility(self):
        values = iter([{"score": 2}, {"score": 1}, {"score": 1}, {"score": 1}])
        with patch.object(queries.time, "sleep"):
            result = queries.observe_until(lambda: next(values), lambda _: True, stable_key=lambda row: row)
        self.assertEqual(result["attempts"], 4)
        self.assertEqual(result["matching_consecutive_observations"], 3)
        self.assertEqual(result["last_observation"], {"score": 1})

    def test_freshness_restores_after_rename_delete_and_exception(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            sandbox = create_sandbox(root / ".work/g0/client-queries", "release")
            populate_fixture(sandbox, "notes-100")
            original = sandbox / "vault" / note_path(0)
            before, stamp = original.read_bytes(), original.stat().st_mtime_ns
            with patch.object(queries, "ROOT", root), self.assertRaisesRegex(RuntimeError, "injected"):
                with queries.preserved_note(sandbox, note_path(0)) as (path, renamed, _):
                    path.write_text("mutation", encoding="utf-8")
                    path.rename(renamed)
                    renamed.unlink()
                    raise RuntimeError("injected")
            self.assertEqual(original.read_bytes(), before)
            self.assertEqual(original.stat().st_mtime_ns, stamp)

    def test_canonical_baseline_cannot_be_freshness_mutation_target(self):
        with patch.object(queries, "verify_sandbox", return_value=ROOT / ".work/g0/client-baseline/fixture"):
            with self.assertRaisesRegex(ValueError, "disposable"):
                with queries.preserved_note(ROOT / ".work/g0/client-baseline/fixture", "note.md"):
                    self.fail("Mutation boundary admitted a canonical corpus")

    def test_evidence_cannot_overwrite_unrelated_artifacts(self):
        for path in (ROOT / "unrelated.json", ROOT / "execution/evidence/c01-anything.json"):
            with self.assertRaises(ValueError):
                queries.output_path(path)

    def test_forced_or_fake_shutdown_fails(self):
        receipt = {"shutdown": {"transport_cancelled": True, "child_exited": True,
            "forced": False, "timeout_unknown": False, "exit_code": 0}, "stopped": {"status": "stopped"}}
        queries.validate_close(receipt)
        receipt["shutdown"]["forced"] = True
        with self.assertRaises(AssertionError):
            queries.validate_close(receipt)


if __name__ == "__main__":
    unittest.main()
