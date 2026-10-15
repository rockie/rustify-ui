import json, glob, sys, collections
files = sorted(glob.glob(sys.argv[1] + "/*gaps-a11y*.json"))
total, violations, serious, binaries, builds, incomplete = 0, 0, 0, set(), set(), collections.Counter()
for f in files:
    r = json.load(open(f))
    binaries.add(json.dumps(r["binary"], sort_keys=True)); builds.add(r["buildId"])
    for scan in r["scans"]:
        total += 1
        for v in scan.get("violations", []):
            violations += 1
            serious += v.get("impact") in ("serious", "critical")
        for i in scan.get("incomplete", []):
            incomplete[i.get("id")] += 1
print(json.dumps({"files": len(files), "scans": total, "violations": violations, "serious_or_critical": serious,
                  "binaries": [json.loads(b) for b in binaries], "build_ids": sorted(builds), "incomplete_rules": dict(incomplete)}, indent=1))
