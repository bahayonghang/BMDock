"""C02 evidence acceptance rejects false connection/persistence/close claims."""
import copy
import unittest

from scripts.client_session import run, validate_report
from scripts.core import ROOT


class ClientSessionEvidenceTests(unittest.TestCase):
    def report(self):
        session = {"profile": "release", "generation": 3}
        note = {"kind": "note_read", "identifier": "corpus/note-00000", "body": "---\r\n中文\r\n---\r\n正文\r\n",
                "session": session, "observation": {"disk_verified": False}}
        return {"session": session, "first": note, "second": copy.deepcopy(note), "child_reused": True,
                "root": {"kind": "tree_page", "session": session, "entries": [{"identifier": "group-000", "kind": "directory"}]},
                "nested": {"kind": "tree_page", "session": session, "entries": [{"identifier": "corpus/note-00000", "kind": "note"}]},
                "stale": {"kind": "error", "category": "policy"}, "wrong_route": {"kind": "error", "category": "policy"},
                "runtime": {"status": "connected", "profile": "release", "session_generation": 3, "shutdown": None},
                "drain": {"engine_spawned": False, "child_killed": False},
                "shutdown": {"transport_cancelled": True, "child_exited": True, "forced": False, "timeout_unknown": False, "exit_code": 0},
                "stopped": {"status": "stopped"}}

    def test_delivered_string_and_single_fixture_byte_equality_are_distinct(self):
        report = self.report()
        source = report["first"]["body"].encode("utf-8")
        exact = validate_report(report, source, "release")
        self.assertTrue(exact["physical_byte_equality_for_this_fixture"])
        self.assertFalse(exact["general_byte_fidelity_claim"])
        report["first"]["body"] = report["second"]["body"] = source.decode("utf-8").replace("\r\n", "\n")
        normalized = validate_report(report, source, "release")
        self.assertTrue(normalized["delivered_string_preserved"])
        self.assertTrue(normalized["upstream_crlf_normalized"])
        self.assertFalse(normalized["physical_byte_equality_for_this_fixture"])

    def test_bad_observations_cannot_pass_evidence_acceptance(self):
        for path, replacement in ((('first', 'body'), 'missing frontmatter'),
                                  (('second', 'session'), {"profile": "main-preview", "generation": 3}),
                                  (('shutdown', 'forced'), True),
                                  (('runtime', 'shutdown'), {"fake_receipt": True}),
                                  (('wrong_route', 'kind'), 'note_read')):
            with self.subTest(path=path):
                report = self.report()
                source = report["first"]["body"].encode("utf-8")
                report[path[0]][path[1]] = replacement
                with self.assertRaises(AssertionError):
                    validate_report(report, source, "release")

    def test_output_outside_owned_evidence_is_rejected_before_launch(self):
        with self.assertRaises(ValueError):
            run("release", ROOT / "not-a-task-evidence-file.json")


if __name__ == "__main__":
    unittest.main()
