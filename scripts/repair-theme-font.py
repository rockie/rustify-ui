#!/usr/bin/env python3
"""Deterministically repair the pinned WenKai font; maintenance only, never a build step.

Uses Python's standard library. No glyphs, metrics or layout tables are rewritten.
Only the overflowing format-4 cmap records, primary names and sfnt checksums change.
OpenType references:
https://learn.microsoft.com/en-us/typography/opentype/spec/cmap#format-12-segmented-coverage
https://learn.microsoft.com/en-us/typography/opentype/spec/name
https://learn.microsoft.com/en-us/typography/opentype/spec/otff#calculating-checksums

Create a candidate from the pinned Git blob (or pass --input original.ttf):
  python3 scripts/repair-theme-font.py --output /tmp/theme-font-repair/RustifyWenKai-Regular.ttf
Check a candidate without Git, a network connection, or an original font:
  python3 scripts/repair-theme-font.py --check /tmp/theme-font-repair/RustifyWenKai-Regular.ttf
For an additional byte-for-byte source comparison, add --input original.ttf to --check.
--report writes machine-readable evidence. This does not run Chrome's OTS sanitizer.
The original copyright and SIL OFL metadata remain intact; the modified family is
Rustify WenKai to avoid the original font's Reserved Font Names.
"""

import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import sys


BASELINE = "7f5e5042cb647ab474f0ef6c833ea568c75b53d4"
RESOURCE = "makepad/widgets/resources/LXGWWenKaiRegular.ttf"
SOURCE_SHA256 = "0819604ad9357afbc6e7cc097328a16ed85143ee0848ffda09fda35a30da6506"
# Filled from the deterministic repair of SOURCE_SHA256, not a local font install.
REPAIRED_SHA256 = "1d4b2f008ac7e02780c19801d4b04dd4538e40215943cc8ad374dba8ec2278c0"
CMAP_SHA256 = {
    12: "63ac8fc7300a5f2450aaef715a4af4dd9e2106891d851e2db2463e534d03ee1b",
    6: "4ce470683e821e3cdec4860ce0be5bac58d4471644adb1cedd6561b0d6367257",
}
# Original table digests. head is normalized by zeroing checksumAdjustment.
PRESERVED_SHA256 = {
    "BASE": "460dcccb059f385cfa05e11d8beed70b0818d6e36e9feb01e4a1fa3712c4eefa",
    "FFTM": "fb6769964c8891dcb3bf0847534dc0f316989e9d7518190c3f34a70f13234bf6",
    "GDEF": "c71f8f81fa7fc6cbcfe60e02b045c1e746d94dc94847c2400de14c9a30765935",
    "GPOS": "70bc95ad3267671a69862b2a2eaf41aedc818fe5f26c8f3e07392f5007ec2fa8",
    "GSUB": "cfd801762968031277fad43d4558c54515c612c6faaddac01c76b705e9db323f",
    "OS/2": "e9ef82b33636b012fa32a4ab769bfd35854df41bf998311319203b5dd379007c",
    "bsln": "eddc9fcd4fa115bc0ec642b071f422e99cd6e5ff209efe5fef4fd1777c2e810a",
    "feat": "a2fdb74b04fa831bdda32a53952748d4b0d54c036f27783350e958a3a67c5111",
    "gasp": "4ca731f86ad506ac0e320283dc9461926346de3c4d91d539ea7f6620d3826940",
    "glyf": "9983a7545e2155059c58e7d6f9ab3e763dcedfa51e3326f96d352e8caa2b31fa",
    "head": "41051a40e16f0aedaf4fde6f9c2975fb6079aae3b85d86f4ac93fad2f4027cae",
    "hhea": "33e9ab0e6861f76af5b3797d13dd65af926114b62f843089b19f2e62e4ab8682",
    "hmtx": "42e9e1feb8dfb893d8fcb1c682e215d9ce7754da91615505c81cfb04141a19f0",
    "loca": "2778a37aa5869f1f0110682e753eeb254323a4289b4748f53a402d9db0190948",
    "maxp": "c787f9ba12585321122150b31a03fe789046c41e1402a5a292a32c6af01017e5",
    "morx": "5984464c470f7a66b2fbea65280104ff08b951df252648aa11bdc34a78177b37",
    "post": "d2995b91524bf6db98dfa5f185ad433450d5788f9335a831c67cc72d65ab05d1",
    "prop": "5770b5884d5008147be14bcf3c516a5c46b3afb2fcd3bc5c062fc421ece963b9",
    "vhea": "a260939184874412cbeffa193ed382dad1ed75784b53a7b1ce85f176d0cdd252",
    "vmtx": "f4516c936268bfbc77d095d9e969d237cbb26975cfe6fcae0ed722245f61ba55",
}
RENAMED = {
    1: "Rustify WenKai",
    3: "RustifyWenKai-Regular;Version 1.330;Rustify-cmap-repair-1",
    4: "Rustify WenKai Regular",
    6: "RustifyWenKai-Regular",
    16: "Rustify WenKai",
    18: "Rustify WenKai Regular",
    20: "RustifyWenKai-Regular",
    21: "Rustify WenKai",
    25: "RustifyWenKai",
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def digest(data):
    return hashlib.sha256(data).hexdigest()


def checksum(data):
    padded = data + b"\0" * (-len(data) % 4)
    return sum(item[0] for item in struct.iter_unpack(">I", padded)) & 0xFFFFFFFF


def normalized(tag, data):
    return data[:8] + b"\0" * 4 + data[12:] if tag == "head" else data


def sfnt(data):
    require(len(data) >= 12 and data[:4] == b"\0\1\0\0", "expected a standalone TrueType sfnt")
    count = struct.unpack_from(">H", data, 4)[0]
    require(len(data) >= 12 + count * 16, "truncated table directory")
    tables, entries, ranges = {}, {}, []
    for i in range(count):
        raw_tag, check, offset, size = struct.unpack_from(">4sIII", data, 12 + i * 16)
        tag = raw_tag.decode("ascii")
        require(tag not in tables, f"duplicate table {tag}")
        require(offset % 4 == 0 and offset >= 12 + count * 16 and offset + size <= len(data),
                f"invalid bounds/alignment for {tag}")
        tables[tag] = data[offset:offset + size]
        entries[tag] = (check, offset, size)
        ranges.append((offset, offset + size, tag))
    for left, right in zip(sorted(ranges), sorted(ranges)[1:]):
        require(left[1] <= right[0], f"overlapping tables {left[2]} and {right[2]}")
    require(set(tables) == set(PRESERVED_SHA256) | {"cmap", "name"}, "unexpected table set")
    return tables, entries


def cmap_records(data, glyphs, drop_overflow=False):
    version, count = struct.unpack_from(">HH", data)
    require(version == 0 and len(data) >= 4 + count * 8, "invalid cmap header")
    records = [struct.unpack_from(">HHI", data, 4 + i * 8) for i in range(count)]
    require(records == sorted(records), "unsorted cmap encoding records")
    retained, removed = [], []
    for platform, encoding, offset in records:
        require(4 + count * 8 <= offset <= len(data) - 6, "invalid cmap subtable offset")
        fmt = struct.unpack_from(">H", data, offset)[0]
        if fmt == 4:
            length, _, seg_x2 = struct.unpack_from(">HHH", data, offset + 2)
            minimum = 16 + 4 * seg_x2
            following = min(x[2] for x in records if x[2] > offset)
            evidence = {"platform": platform, "encoding": encoding, "offset": offset,
                        "format": fmt, "declared_length": length, "segments": seg_x2 // 2,
                        "minimum_arrays_length": minimum, "occupied_length": following - offset}
            require(drop_overflow and seg_x2 % 2 == 0 and length < minimum
                    and following - offset == length + 65536,
                    f"overflowing format 4: {json.dumps(evidence, sort_keys=True)}")
            removed.append(evidence)
            continue
        require(fmt in (6, 12), f"unexpected cmap format {fmt}")
        if fmt == 12:
            require(offset + 16 <= len(data), "truncated format 12 header")
            _, reserved, length, language, groups = struct.unpack_from(">HHIII", data, offset)
            require(reserved == 0 and language == 0 and length == 16 + groups * 12
                    and offset + length <= len(data), "invalid format 12 header")
            previous = -1
            for i in range(groups):
                start, end, glyph = struct.unpack_from(">III", data, offset + 16 + i * 12)
                require(previous < start <= end <= 0x10FFFF and glyph + end - start < glyphs,
                        f"invalid format 12 group {i}")
                previous = end
        else:
            _, length, _, first, count_glyphs = struct.unpack_from(">HHHHH", data, offset)
            require(length == 10 + count_glyphs * 2 and offset + length <= len(data)
                    and first + count_glyphs <= 65536, "invalid format 6 header")
            require(all(g[0] < glyphs for g in struct.iter_unpack(">H", data[offset + 10:offset + length])),
                    "format 6 maps outside the glyph set")
        subtable = data[offset:offset + length]
        require(digest(subtable) == CMAP_SHA256[fmt], f"format {fmt} differs from pinned source")
        retained.append((platform, encoding, offset, fmt, subtable))
    require([(r[0], r[1], r[3]) for r in retained] == [(0, 4, 12), (1, 0, 6), (3, 10, 12)],
            "unexpected retained cmap records")
    return retained, removed


def repaired_cmap(data, glyphs):
    records, removed = cmap_records(data, glyphs, drop_overflow=True)
    require([(r["platform"], r["encoding"]) for r in removed] == [(0, 3), (3, 1)],
            "expected exactly the two overflowing format-4 records")
    body, offsets, encoded = bytearray(), {}, []
    start = 4 + len(records) * 8
    for platform, encoding, old_offset, _, subtable in records:
        if old_offset not in offsets:
            offsets[old_offset] = start + len(body)
            body.extend(subtable)
        encoded.append(struct.pack(">HHI", platform, encoding, offsets[old_offset]))
    return struct.pack(">HH", 0, len(records)) + b"".join(encoded) + body, removed


def name_records(data):
    version, count, storage = struct.unpack_from(">HHH", data)
    require(version == 0 and storage >= 6 + count * 12 and storage <= len(data),
            "expected a complete format-0 name table")
    records = []
    for i in range(count):
        platform, encoding, language, name_id, size, offset = struct.unpack_from(">HHHHHH", data, 6 + i * 12)
        require(storage + offset + size <= len(data), "truncated name string")
        records.append(((platform, encoding, language, name_id), data[storage + offset:storage + offset + size]))
    require([r[0] for r in records] == sorted(r[0] for r in records), "unsorted name records")
    return records


def renamed_bytes(key):
    platform, _, _, name_id = key
    require(platform in (0, 1, 3), "unexpected name platform")
    # All replacements are ASCII: also valid in the original Mac Chinese encoding.
    return RENAMED[name_id].encode("utf-16-be" if platform in (0, 3) else "ascii")


def repaired_name(data):
    records, body = [], bytearray()
    for key, original in name_records(data):
        value = renamed_bytes(key) if key[3] in RENAMED else original
        records.append(struct.pack(">HHHHHH", *key, len(value), len(body)))
        body.extend(value)
    require(len(body) < 65536, "name string storage exceeds 16-bit offsets")
    return struct.pack(">HHH", 0, len(records), 6 + len(records) * 12) + b"".join(records) + body


def pack_sfnt(tables):
    count = len(tables)
    exponent = count.bit_length() - 1
    output = bytearray(struct.pack(">IHHHH", 0x10000, count, (1 << exponent) * 16,
                                   exponent, count * 16 - (1 << exponent) * 16))
    output.extend(b"\0" * (count * 16))
    head_offset = None
    for i, tag in enumerate(sorted(tables)):
        table = normalized(tag, tables[tag])
        offset = len(output)
        struct.pack_into(">4sIII", output, 12 + i * 16, tag.encode("ascii"), checksum(table), offset, len(table))
        output.extend(table)
        output.extend(b"\0" * (-len(output) % 4))
        if tag == "head":
            head_offset = offset
    struct.pack_into(">I", output, head_offset + 8, (0xB1B0AFBA - checksum(output)) & 0xFFFFFFFF)
    return bytes(output)


def verify(data, original=None):
    tables, entries = sfnt(data)
    glyphs = struct.unpack_from(">H", tables["maxp"], 4)[0]
    records, _ = cmap_records(tables["cmap"], glyphs)
    for tag, expected in PRESERVED_SHA256.items():
        require(digest(normalized(tag, tables[tag])) == expected, f"modified source table {tag}")
    for tag, (expected, _, _) in entries.items():
        require(checksum(normalized(tag, tables[tag])) == expected, f"incorrect directory checksum for {tag}")
    require(checksum(data) == 0xB1B0AFBA, "incorrect whole-font checksumAdjustment")
    names = name_records(tables["name"])
    for key, value in names:
        if key[3] in RENAMED:
            require(value == renamed_bytes(key), f"unrenamed primary name {key}")
    if original is not None:
        before, _ = sfnt(original)
        expected_cmap, _ = repaired_cmap(before["cmap"], glyphs)
        require(tables["cmap"] == expected_cmap, "cmap differs from deterministic repair")
        require(tables["name"] == repaired_name(before["name"]), "name differs from deterministic rename")
        unchanged_names = [(key, value) for key, value in names if key[3] not in RENAMED]
        require(unchanged_names == [(key, value) for key, value in name_records(before["name"]) if key[3] not in RENAMED],
                "copyright/license/other name metadata changed")
    require(digest(data) == REPAIRED_SHA256, "font differs from fixed repaired output")
    fmt12 = records[0][4]
    groups = struct.unpack_from(">I", fmt12, 12)[0]
    mapped = sum(end - start + 1 for start, end, _ in struct.iter_unpack(">III", fmt12[16:]))
    return {"sha256": digest(data), "bytes": len(data), "table_count": len(tables),
            "whole_font_checksum": hex(checksum(data)), "glyph_count": glyphs,
            "format12_groups": groups, "format12_codepoints": mapped,
            "cmap_records": [{"platform": r[0], "encoding": r[1], "offset": r[2],
                              "format": r[3], "bytes": len(r[4]), "sha256": digest(r[4])} for r in records],
            "preserved_table_sha256": PRESERVED_SHA256,
            "renamed_name_ids": sorted({key[3] for key, _ in names if key[3] in RENAMED}),
            "original_metadata_compared": original is not None,
            "browser_ots_verified": False}


def source(path):
    if path:
        data = path.read_bytes()
    else:
        root = Path(__file__).resolve().parent.parent
        data = subprocess.check_output(["git", "show", f"{BASELINE}:{RESOURCE}"], cwd=root)
        lock = json.loads((root / "sources.lock.json").read_text())
        require(lock["makepad"]["files"]["widgets/resources/LXGWWenKaiRegular.ttf"] == SOURCE_SHA256,
                "sources.lock does not contain the pinned original digest")
    require(digest(data) == SOURCE_SHA256, "input is not the pinned original font")
    return data


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    action = parser.add_mutually_exclusive_group(required=True)
    action.add_argument("--output", type=Path, help="write a repaired candidate; refuses to overwrite")
    action.add_argument("--check", type=Path, help="verify the fixed candidate without requiring Git")
    parser.add_argument("--input", type=Path, help="original font with the pinned SHA256")
    parser.add_argument("--report", type=Path, help="write JSON evidence")
    args = parser.parse_args()
    original = source(args.input) if args.output or args.input else None
    if args.output:
        tables, _ = sfnt(original)
        glyphs = struct.unpack_from(">H", tables["maxp"], 4)[0]
        tables["cmap"], removed = repaired_cmap(tables["cmap"], glyphs)
        tables["name"] = repaired_name(tables["name"])
        candidate = pack_sfnt(tables)
        report = verify(candidate, original)
        with args.output.open("xb") as handle:
            handle.write(candidate)
        report["removed_cmap_records"] = removed
    else:
        report = verify(args.check.read_bytes(), original)
    report["source_baseline"] = BASELINE
    report["source_sha256"] = SOURCE_SHA256
    if args.report:
        args.report.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ("sha256", "bytes", "glyph_count", "format12_codepoints", "browser_ots_verified")}, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, struct.error, subprocess.CalledProcessError) as error:
        print(f"font repair check failed: {error}", file=sys.stderr)
        sys.exit(1)
