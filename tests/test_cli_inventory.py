from types import SimpleNamespace
import unittest
from scripts.engine_worker import collect_cli_commands


class Context:
    def __init__(self, command, **kwargs):
        self.command = command
        self.parent = kwargs["parent"]


class Leaf:
    context_class = Context
    def __init__(self):
        self.params = [SimpleNamespace(name="project", required=False, opts=["--project"], type="TEXT")]


class Group(Leaf):
    # Deliberately unrelated to external click.Group, as with bundled Click.
    def __init__(self, children):
        super().__init__()
        self.children = children
    def list_commands(self, context):
        assert isinstance(context, Context)
        return sorted(self.children)
    def get_command(self, context, name):
        return self.children[name]


class InventoryTests(unittest.TestCase):
    def test_nested_groups_are_not_lost(self):
        root = Group({"cloud": Group({"login": Leaf()}), "mcp": Leaf()})
        rows = collect_cli_commands(root)
        self.assertEqual([row["path"] for row in rows], [["bm"], ["bm", "cloud"], ["bm", "cloud", "login"], ["bm", "mcp"]])
        self.assertTrue(rows[0]["group"])
        self.assertFalse(rows[-1]["group"])
        self.assertEqual(rows[-1]["parameters"][0]["opts"], ["--project"])

    def test_cycle_is_rejected(self):
        root = Group({})
        root.children["self"] = root
        with self.assertRaises(ValueError):
            collect_cli_commands(root)

    def test_missing_advertised_child_fails_closed(self):
        with self.assertRaises(ValueError):
            collect_cli_commands(Group({"missing": None}))

    def test_aliases_in_different_branches_are_preserved(self):
        leaf = Leaf()
        rows = collect_cli_commands(Group({"first": leaf, "second": leaf}))
        self.assertEqual(len(rows), 3)


if __name__ == "__main__":
    unittest.main()
