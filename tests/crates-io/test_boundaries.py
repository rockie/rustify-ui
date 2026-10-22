"""The real package, corpus, documentation and shared-core boundary checks."""

import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("crates_io", ROOT / "scripts/crates-io.py")
release = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(release)
TOOLCHAIN = tomllib.loads((ROOT / "mise.toml").read_text())["tools"]["rust"]["version"]
SYSROOT = Path(subprocess.check_output(["rustc", "--print", "sysroot"], cwd=ROOT, text=True).strip())
if not subprocess.check_output([str(SYSROOT / "bin/rustc"), "--version"], text=True).startswith(f"rustc {TOOLCHAIN} "):
    raise RuntimeError("boundary checks require the mise-pinned Rust compiler")


def run(*args, cwd=ROOT, env=None, success=True):
    if args[0] == "mbx":
        args = (args[0], f"+{TOOLCHAIN}", *args[1:])
        env = {**(env or os.environ), "RUSTC": str(SYSROOT / "bin/rustc"), "RUSTDOC": str(SYSROOT / "bin/rustdoc"), "RUSTUP_TOOLCHAIN": TOOLCHAIN}
    result = subprocess.run(args, cwd=cwd, env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if success and result.returncode:
        raise AssertionError(f"{' '.join(map(str, args))}\n{(result.stdout + result.stderr)[-18000:]}")
    return result


CORPUS = [
    ("rustify-style", ["--all-features"]),
    ("rustify-dom", ["--no-default-features", "--features", "ssr"]),
    ("rustify-components", []),
    ("rustify-style-wasm", ["--no-default-features", "--features", "compiler,scanner"]),
    ("rustify-docs", []),
    ("rustify-docs-cli", []),
]


class BoundaryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.entries = release.inspect()["packages"]

    def test_inventory_and_caret_floors_preserve_locked_third_party_versions(self):
        self.assertEqual(len(self.entries), 9)
        expected = {"serde": "1.0.229", "serde_json": "1.0.151", "syn": "2.0.119", "quote": "1.0.47", "proc-macro2": "1.0.107"}
        for entry in self.entries:
            manifest = tomllib.loads((ROOT / entry["path"] / "Cargo.toml").read_text())
            for table in ("dependencies", "dev-dependencies"):
                for name, dependency in manifest.get(table, {}).items():
                    if name in expected:
                        version = dependency if isinstance(dependency, str) else dependency["version"]
                        # The unmodified upstream host macro retains its original broad major floor.
                        self.assertIn(version, (expected[name], expected[name].split(".")[0]))
                        self.assertFalse(version.startswith("="))
        before = tomllib.loads(run("git", "show", "HEAD:Cargo.lock").stdout)["package"]
        after = tomllib.loads((ROOT / "Cargo.lock").read_text())["package"]
        for name in expected:
            self.assertEqual(
                {(p["version"], p.get("source")) for p in before if p["name"] == name},
                {(p["version"], p.get("source")) for p in after if p["name"] == name}, name,
            )

    def test_corpus_flag_runs_each_real_sentinel(self):
        env = dict(os.environ)
        if "rustify_repo_tests" not in env.get("RUSTFLAGS", ""):
            env["RUSTFLAGS"] = env.get("RUSTFLAGS", "") + " --cfg rustify_repo_tests"
        for package, features in CORPUS:
            with self.subTest(package=package):
                target = ["--bin", "rustify-docs"] if package.endswith("-cli") else ["--lib"]
                result = run("mbx", "test", "--locked", "-p", package, *features, *target, "repo_corpus_sentinel", env=env)
                self.assertRegex(result.stdout, r"test repo_tests::repo_corpus_sentinel \.\.\. ok")
                self.assertRegex(result.stdout, r"test result: ok\. 1 passed;")

    def test_documentation_uses_each_packages_single_docs_rs_mode(self):
        for entry in self.entries:
            if entry["name"] == "rustify-docs-cli":
                continue
            with self.subTest(package=entry["name"]):
                manifest = tomllib.loads((ROOT / entry["path"] / "Cargo.toml").read_text())
                docs = manifest["package"]["metadata"]["docs"]["rs"]
                flags = ["--no-default-features"] if docs.get("no-default-features") else []
                if docs.get("features"):
                    flags.extend(["--features", ",".join(docs["features"])])
                run("mbx", "doc", "--locked", "--no-deps", "-p", entry["name"], "--target", docs["default-target"], *flags)

    def test_real_archives_portable_tests_and_mixed_core_types(self):
        libraries = [entry for entry in self.entries if entry["name"] != "rustify-docs-cli"]
        flags = [flag for entry in libraries for flag in ("-p", entry["name"])]
        run("mbx", "package", "--locked", "--allow-dirty", "--no-verify", *flags)
        archives = [ROOT / "target/package" / f"{entry['name']}-{entry['version']}.crate" for entry in libraries]
        with tempfile.TemporaryDirectory(prefix="rustify-boundaries-") as temporary:
            work = Path(temporary)
            registry = release.stage_registry(ROOT / "Cargo.lock", archives, ROOT / "target/package/tmp-registry/index", work / "registry")
            config = work / "cargo-config.toml"
            config.write_text('[source.crates-io]\nreplace-with = "staging"\n[source.staging]\nlocal-registry = ' + json.dumps(str(registry)) + "\n")
            build_env = {**os.environ, "CARGO_TARGET_DIR": str(ROOT / "target/crates-io-boundaries")}
            for entry, archive in zip(libraries, archives):
                with self.subTest(package=entry["name"]):
                    with tarfile.open(archive) as bundle:
                        names = bundle.getnames()
                        self.assertFalse(any(re.search(r"(^|/)(?:target|\.git|\.env|credentials)(?:/|$)", name) for name in names))
                        self.assertFalse(any("/makepad/" in name for name in names))
                        self.assertTrue(any(name.endswith("/LICENSE") for name in names))
                        bundle.extractall(work / "unpacked", filter="data")
                    extracted = work / "unpacked" / f"{entry['name']}-{entry['version']}"
                    features = next((features for name, features in CORPUS if name == entry["name"]), [])
                    # These builds cannot see repository fixtures, even if a test forgot its cfg.
                    run("mbx", "test", "--offline", "--locked", "--config", str(config), "--manifest-path", str(extracted / "Cargo.toml"), "--lib", *features, env=build_env)
                    if entry["name"] == "rustify-dom":
                        run("mbx", "test", "--offline", "--locked", "--config", str(config), "--manifest-path", str(extracted / "Cargo.toml"), "--no-default-features", "--features", "ssr", "--test", "ssr_scope", env=build_env)
            probe = work / "probe"
            (probe / "src").mkdir(parents=True)
            snapshot = work / "source"
            snapshot.mkdir()
            for package in ("rustify-dom", "rustify-ui", "rustify-components", "rustify-style", "rustify-makepad"):
                shutil.copytree(ROOT / "crates" / package, snapshot / "crates" / package)
            # Preserve the actual optional GPU dependency manifests and source too.
            # Copy tracked source, never nested target directories or test corpora.
            for filename in run("git", "ls-files", "makepad").stdout.splitlines():
                source = ROOT / filename
                if source.is_file():
                    target = snapshot / filename
                    target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(source, target)
            root_manifest = (ROOT / "Cargo.toml").read_text()
            start = root_manifest.index("members = [")
            end = root_manifest.index("]", start) + 1
            members = ', '.join(json.dumps("crates/" + package) for package in (
                "rustify-dom", "rustify-ui", "rustify-components", "rustify-style", "rustify-makepad",
            ))
            (snapshot / "Cargo.toml").write_text(root_manifest[:start] + f"members = [{members}]" + root_manifest[end:])
            shutil.copyfile(ROOT / "Cargo.lock", snapshot / "Cargo.lock")
            run("git", "init", "--quiet", cwd=snapshot)
            run("git", "add", ".", cwd=snapshot)
            run("git", "-c", "user.name=Rustify fixture", "-c", "user.email=fixture@example.invalid", "commit", "--quiet", "-m", "Snapshot actual SDK sources for source identity check", cwd=snapshot)
            revision = run("git", "rev-parse", "HEAD", cwd=snapshot).stdout.strip()
            git = f'git = {json.dumps(snapshot.as_uri())}, rev = "{revision}"'
            manifest = f'''[workspace]
[package]
name = "mixed-core-probe"
version = "0.0.0"
edition = "2021"
[dependencies]
rustify-ui = {{ {git}, default-features = false, features = ["ssr"] }}
rustify-dom = {{ version = "=0.2.0", default-features = false, features = ["ssr"] }}
rustify-components = {{ version = "=0.2.0", default-features = false, features = ["ssr"] }}
'''
            (probe / "Cargo.toml").write_text(manifest)
            (probe / "src/lib.rs").write_text('''pub fn theme(value: rustify_ui::Theme) -> rustify_dom::Theme { value }
pub fn toast(value: rustify_ui::Toast) {
    let _ = rustify_components::toast::ToastProps::builder().toast(value).build();
}
''')
            args = ("mbx", "check", "--offline", "--config", str(config), "--manifest-path", str(probe / "Cargo.toml"))
            run("mbx", "fetch", "--config", str(config), "--manifest-path", str(probe / "Cargo.toml"), env=build_env)
            mixed = run(*args, success=False, env=build_env)
            self.assertNotEqual(mixed.returncode, 0)
            self.assertIn("mismatched types", mixed.stdout + mixed.stderr)
            self.assertIn("rustify_dom", mixed.stdout + mixed.stderr)
            # The documented application-root patch unifies the actual SDK and registry components.
            (probe / "Cargo.toml").write_text(manifest + f'\n[patch.crates-io]\nrustify-dom = {{ {git} }}\n')
            run(*args, env=build_env)
            metadata = json.loads(run("mbx", "metadata", "--offline", "--format-version", "1", "--config", str(config), "--manifest-path", str(probe / "Cargo.toml")).stdout)
            self.assertEqual(len([p for p in metadata["packages"] if p["name"] == "rustify-dom"]), 1)
            same_source = manifest.replace('version = "=0.2.0"', git)
            (probe / "Cargo.toml").write_text(same_source)
            run(*args, env=build_env)


if __name__ == "__main__":
    unittest.main()
