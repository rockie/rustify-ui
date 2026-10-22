#!/usr/bin/env python3
"""Compose a language overlay on an isolated copy of the real Rust consumer."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import shutil
import subprocess
import sys


def prepare(destination: Path, cli: Path, base: str | None = None) -> Path:
    repository = Path(__file__).resolve().parents[2]
    source = repository / "tests/docs-ssg/fixtures/components"
    overlay = repository / "tests/docs-ssg/fixtures/i18n/components"
    destination.mkdir(parents=True, exist_ok=True)
    if any(destination.iterdir()):
        raise ValueError("the fixture destination must be empty")
    for directory in ("content", "src", "static", "fonts"):
        shutil.copytree(source / directory, destination / directory)
    for name in ("Cargo.toml", "Cargo.lock", "rustify-docs.toml", "theme.json"):
        shutil.copyfile(source / name, destination / name)
    for directory in ("src", "locales"):
        shutil.copytree(overlay / directory, destination / directory, dirs_exist_ok=True)
    registry = (destination / "src/lib.rs").read_text()
    registry = registry.replace("mod catalog;", "mod catalog;\nmod locale;", 1)
    registry = registry.replace("        pages {", '''        pages {
            LocaleEn { route: "/locale/", title: "Locale page", description: "One logical page in several languages." } => crate::locale::english,
            LocaleZh { route: "/locale/", title: "语言页面", description: "同一逻辑页面的中文版本。", lang: "zh" } => crate::locale::chinese,''', 1)
    registry = registry.replace("before_content => crate::custom::before_content", "before_content => crate::locale::before_content")
    registry = registry.replace("not_found => crate::custom::not_found", "not_found => crate::locale::not_found")
    (destination / "src/lib.rs").write_text(registry)
    config = (destination / "rustify-docs.toml").read_text()
    config = config.replace('routes = ["/custom/", "/plain/"]', 'routes = ["/locale/", "/custom/", "/plain/"]', 1)
    config += "\n" + (overlay / "locales.toml").read_text()
    if base is not None:
        config = config.replace('url = "https://example.com/docs/"', f'url = "https://example.com{base}"')
        config = config.replace('base = "/docs/"', f'base = "{base}"')
    (destination / "rustify-docs.toml").write_text(config)
    subprocess.run([
        sys.executable, str(repository / "scripts/docs-pages-prepare.py"),
        "--project", str(destination / "Cargo.toml"), "--cli", str(cli),
    ], check=True, stdout=subprocess.DEVNULL)
    return destination


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--cli", type=Path, required=True)
    parser.add_argument("--base")
    args = parser.parse_args()
    print(prepare(args.out.resolve(), args.cli.resolve(), args.base))
