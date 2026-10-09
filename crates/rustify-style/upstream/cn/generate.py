#!/usr/bin/env python3
"""Reproduce typed default data and the fixed table-emission template."""
import argparse
import json
from pathlib import Path
import re
import subprocess

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
FIXTURES = ROOT / "tests/utility/cn"
NAMES = [
    "isAny", "isAnyNonArbitrary", "isArbitraryValue", "isArbitraryVariable",
    "isFraction", "isNumber", "isInteger", "isPercent", "isTshirtSize",
    "isNamedContainerQuery", "isArbitraryLength", "isArbitraryNumber",
    "isArbitraryWeight", "isArbitraryFamilyName", "isArbitraryPosition",
    "isArbitrarySize", "isArbitraryImage", "isArbitraryShadow",
    "isArbitraryVariableLength", "isArbitraryVariableFamilyName",
    "isArbitraryVariablePosition", "isArbitraryVariableSize",
    "isArbitraryVariableImage", "isArbitraryVariableShadow", "isArbitraryVariableWeight",
]


def string(value):
    return json.dumps(value, ensure_ascii=False)


def mapping(value, convert):
    return "vec![" + ",".join(f"({string(key)}.to_owned(),{convert(child)})" for key, child in value.items()) + "]"


def definitions(values):
    return "vec![" + ",".join(definition(value) for value in values) + "]"


def strings(values):
    return "vec![" + ",".join(string(value) + ".to_owned()" for value in values) + "]"


def definition(value):
    if isinstance(value, str):
        return "ClassGroupDef::Literal(" + string(value) + ".to_owned())"
    if set(value) == {"$v"}:
        return "ClassGroupDef::Builtin(" + str(NAMES.index(value["$v"])) + ")"
    if set(value) == {"$t"}:
        return "ClassGroupDef::Theme(" + string(value["$t"]) + ".to_owned())"
    return "ClassGroupDef::Object(" + mapping(value, definitions) + ")"


def rustfmt(source):
    return subprocess.run(["rustfmt", "--edition", "2024", "--emit", "stdout"], input=source, text=True, capture_output=True, check=True).stdout


def artifacts():
    config_bytes = (FIXTURES / "default-config.json").read_bytes()
    config = json.loads(config_bytes)
    tables = json.loads((FIXTURES / "default-tables.json").read_bytes())
    assert not tables["customValidatorNames"]
    assert all(0 <= op < len(NAMES) for op in tables["vlistOps"])
    lines = [
        "// Generated from fixed cn commit b7ec0fce43d824a60517fab508584eb860fdf717.",
        "// MIT, Copyright (c) 2026 shadcn. See LICENSE and NOTICE.md.",
        "use crate::cn::engine::Tables;", "pub(crate) fn tables() -> Tables {", "    Tables {",
    ]
    for key, value in tables.items():
        field = "group_count" if key == "GROUP_COUNT" else re.sub(r"(?<!^)(?=[A-Z])", "_", key).lower()
        expression = "vec![" + ",".join(str(item) for item in value) + "]" if isinstance(value, list) else string(value) + ".to_owned()" if isinstance(value, str) else str(value)
        lines.append(f"        {field}: {expression},")
    lines.extend(["        prefix: None,", "    }", "}"])
    table_source = rustfmt("\n".join(lines) + "\n")
    lines = [
        "// Generated from fixed cn commit b7ec0fce43d824a60517fab508584eb860fdf717.",
        "// MIT, Copyright (c) 2026 shadcn. Default configuration: Dany Castillo (MIT).",
        "// See LICENSE and NOTICE.md.", "use crate::cn::{ClassGroupDef, Config};",
        "pub(crate) fn config() -> Config {", "    Config {",
    ]
    fields = [
        ("theme", "theme", lambda value: mapping(value, definitions)),
        ("classGroups", "class_groups", lambda value: mapping(value, definitions)),
        ("conflictingClassGroups", "conflicting_class_groups", lambda value: mapping(value, strings)),
        ("conflictingClassGroupModifiers", "conflicting_class_group_modifiers", lambda value: mapping(value, strings)),
        ("orderSensitiveModifiers", "order_sensitive_modifiers", strings),
        ("postfixLookupClassGroups", "postfix_lookup_class_groups", strings),
    ]
    for key, field, convert in fields:
        lines.append(f"        {field}: {convert(config.get(key, []))},")
    lines.extend(["        prefix: None,", "    }", "}"])
    config_source = rustfmt("\n".join(lines) + "\n")
    compiler = (ROOT / "ref/cn/packages/cn/src/compiler.ts").read_text()
    start = compiler.index("return `${banner}\nconst P") + len("return `")
    end = compiler.index("\n`\n}", start) + 1
    template = compiler[start:end]
    expressions = list(dict.fromkeys(re.findall(r"\$\{(.*?)\}", template, re.S)))
    for index, expression in enumerate(expressions):
        template = template.replace("${" + expression + "}", f"@@{index}@@")
    return {
        "tables.rs": table_source.encode(), "config.rs": config_source.encode(),
        "default-config.json": config_bytes, "emit-template.txt": template.encode(),
        "emit-placeholders.json": (json.dumps(expressions, indent=2) + "\n").encode(),
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write", action="store_true")
    args = parser.parse_args()
    subprocess.run(["node", str(FIXTURES / "reference.mjs"), "--check"], check=True)
    for name, content in artifacts().items():
        target = HERE / name
        if args.check:
            if target.read_bytes() != content:
                raise SystemExit(f"generated cn source drift: {target}")
        else:
            target.write_bytes(content)
    print("cn typed default data and emitter template verified" if args.check else "cn typed default data and emitter template regenerated")
