import copy
import importlib.util
from pathlib import Path
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("crates_io", ROOT / "scripts/crates-io.py")
release = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(release)


class InspectTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        (self.root / "release").mkdir()
        (self.root / "release/crates-io.toml").write_text('''
[[packages]]
name = "core"
path = "core"
group = "docs"
dependencies = []
[[packages]]
name = "app"
path = "app"
group = "docs"
dependencies = ["core"]
''')
        self.packages = []
        for name in ("core", "app", "private"):
            directory = self.root / name
            directory.mkdir()
            (directory / "LICENSE").write_text("MIT fixture")
            (directory / "README.md").write_text("fixture")
            (directory / "Cargo.toml").write_text('[package]\ninclude = ["src/**", "README.md", "LICENSE"]\n')
            package = {
                "id": name, "name": name, "version": "0.2.0", "source": None,
                "manifest_path": str(directory / "Cargo.toml"),
                "publish": ["crates-io"] if name != "private" else [],
                "description": "fixture", "license": "MIT", "readme": "README.md",
                "repository": "https://example.test/repo", "documentation": "https://example.test/docs",
                "include": ["src/**", "README.md", "LICENSE"],
                "targets": [{"kind": ["lib"]}],
                "metadata": {"docs": {"rs": {"default-target": "x86_64-unknown-linux-gnu", "targets": []}}},
                "dependencies": [],
            }
            self.packages.append(package)
        self.packages[1]["dependencies"] = [{
            "name": "core", "kind": None, "path": str(self.root / "core"),
            "req": "=0.2.0", "rename": "local_core",
        }]
        self.metadata = {
            "packages": self.packages,
            "workspace_members": ["core", "app", "private"],
            "resolve": {"nodes": [
                {"id": "core", "deps": []},
                {"id": "app", "deps": [{"pkg": "core", "dep_kinds": [{"kind": None}]}]},
                {"id": "private", "deps": []},
            ]},
        }

    def inspect(self):
        return release.inspect(self.root, self.metadata)

    def test_valid_inventory_and_alias_keep_exact_registry_identity(self):
        result = self.inspect()
        self.assertEqual([p["name"] for p in result["packages"]], ["core", "app"])
        self.assertEqual(result["packages"][1]["version"], "0.2.0")

    def test_path_without_exact_matching_version_is_rejected(self):
        for requirement in ("*", "^0.2.0", "=0.1.0"):
            with self.subTest(requirement=requirement):
                self.packages[1]["dependencies"][0]["req"] = requirement
                with self.assertRaisesRegex(release.ReleaseError, "exact version"):
                    self.inspect()

    def test_private_workspace_cannot_accidentally_be_published(self):
        self.packages[2]["publish"] = None
        with self.assertRaisesRegex(release.ReleaseError, "private"):
            self.inspect()

    def test_optional_gpu_or_other_path_dependency_is_rejected(self):
        self.packages[0]["dependencies"].append({
            "name": "private", "kind": None, "optional": True,
            "path": str(self.root / "private"), "req": "=0.2.0",
        })
        with self.assertRaisesRegex(release.ReleaseError, "outside.*inventory"):
            self.inspect()

    def test_transitive_registry_override_is_rejected(self):
        third = copy.deepcopy(self.packages[2])
        third.update(id="patched", name="smallvec", source=None)
        self.metadata["packages"].append(third)
        self.metadata["resolve"]["nodes"][0]["deps"].append({
            "pkg": "patched", "dep_kinds": [{"kind": "build"}],
        })
        self.metadata["resolve"]["nodes"].append({"id": "patched", "deps": []})
        with self.assertRaisesRegex(release.ReleaseError, "path.*smallvec"):
            self.inspect()

    def test_missing_metadata_and_unsafe_content_are_rejected(self):
        for key, value in (("repository", None), ("readme", None)):
            with self.subTest(key=key):
                original = self.packages[0][key]
                self.packages[0][key] = value
                with self.assertRaises(release.ReleaseError):
                    self.inspect()
                self.packages[0][key] = original
        (self.root / "core/Cargo.toml").write_text('[package]\ninclude = ["**"]\n')
        with self.assertRaisesRegex(release.ReleaseError, "unsafe"):
            self.inspect()

    def test_inventory_topology_is_checked(self):
        path = self.root / "release/crates-io.toml"
        path.write_text(path.read_text().replace('dependencies = []', 'dependencies = ["app"]', 1))
        with self.assertRaisesRegex(release.ReleaseError, "topolog"):
            self.inspect()


if __name__ == "__main__":
    unittest.main()
