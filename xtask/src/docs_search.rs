//! The installed CLI's search product, measured through input and painted DOM.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

use serde_json::{json, Value};

fn validate(data: &Value, lang: &str, entrance: &str) -> Result<(), String> {
    if data["schema_version"] != 1
        || data["lang"] != lang
        || data["entrance"] != entrance
        || data["index"]["pages"] != 1000
        || data["index"]["fallback"] != if lang == "zh" { 500 } else { 0 }
        || data["warm_requests"]
            .as_array()
            .is_none_or(|requests| !requests.is_empty())
    {
        return Err(format!(
            "{lang}/{entrance}: missing capacity, identity or zero-network evidence"
        ));
    }
    let metrics = data["metrics"].as_array().ok_or("no query measurements")?;
    let expected: BTreeSet<_> = [
        "chinese", "english", "prefix", "empty", "fallback", "custom",
    ]
    .into_iter()
    .collect();
    let names: BTreeSet<_> = metrics
        .iter()
        .filter_map(|metric| metric["name"].as_str())
        .collect();
    if names != expected || metrics.len() != expected.len() {
        return Err("all six query kinds must be measured".into());
    }
    for metric in metrics {
        for (phase, limit) in [("cold", 1000.0), ("warm", 100.0)] {
            let samples = metric[phase]
                .as_array()
                .ok_or_else(|| format!("missing {phase} samples"))?;
            if samples.len() != 5 {
                return Err(format!("{} {phase}: need five samples", metric["name"]));
            }
            let mut times = Vec::new();
            for sample in samples {
                let elapsed = sample["elapsed"]
                    .as_f64()
                    .filter(|value| value.is_finite() && *value >= 0.0)
                    .ok_or("invalid elapsed time")?;
                let id = sample["id"]
                    .as_u64()
                    .ok_or("missing query request id")?
                    .to_string();
                if sample["focusImmediate"] != true
                    || sample["resultId"].as_str() != Some(id.as_str())
                {
                    return Err("sample does not identify the focused input and corresponding painted request".into());
                }
                times.push(elapsed);
            }
            times.sort_by(f64::total_cmp);
            if times[2] > limit {
                return Err(format!(
                    "{} {phase}: {:.1}ms median exceeds {limit}ms",
                    metric["name"], times[2]
                ));
            }
        }
    }
    Ok(())
}

pub fn verify(args: &[String]) -> Result<(), String> {
    let root = crate::build::repo_root();
    let cli = PathBuf::from(
        std::env::var_os("RUSTIFY_DOCS_BIN")
            .ok_or("set RUSTIFY_DOCS_BIN to the fixed C installation before measuring search")?,
    );
    let identity = Command::new(&cli)
        .arg("--build-info")
        .output()
        .map_err(|error| error.to_string())?;
    if !identity.status.success() {
        return Err("cannot read installed CLI identity".into());
    }
    let identity = String::from_utf8(identity.stdout).map_err(|error| error.to_string())?;
    if !identity.contains("\npanic: unwind\n") {
        return Err("measure the installed docs-cli profile".into());
    }
    let work = root.join("target/docs-ssg/search-performance");
    if work.exists() {
        std::fs::remove_dir_all(&work).map_err(|error| error.to_string())?;
    }
    let cases_dir = work.join("cases");
    std::fs::create_dir_all(&cases_dir).map_err(|error| error.to_string())?;
    let status = Command::new("npx")
        .current_dir(&root)
        .args([
            "playwright",
            "test",
            "-c",
            "playwright.docs.config.ts",
            "--project=docs-search-perf",
            "search-perf.spec.ts",
        ])
        .env("RUSTIFY_TIER", "evidence")
        .env("RUSTIFY_SEARCH_REPORT_DIR", &cases_dir)
        .status()
        .map_err(|error| error.to_string())?;
    let mut problems = Vec::new();
    if !status.success() {
        problems.push(format!("Playwright exited with {status}"));
    }
    let mut cases = Vec::new();
    for lang in ["en", "zh"] {
        for entrance in ["largest", "404"] {
            let file = cases_dir.join(format!("{lang}-{entrance}.json"));
            let case = std::fs::read(&file)
                .map_err(|error| error.to_string())
                .and_then(|bytes| {
                    serde_json::from_slice::<Value>(&bytes).map_err(|error| error.to_string())
                });
            match case {
                Ok(data) => {
                    if let Err(error) = validate(&data, lang, entrance) {
                        problems.push(format!("{}: {error}", file.display()));
                    }
                    cases.push(data);
                }
                Err(error) => problems.push(format!(
                    "missing valid {lang}/{entrance} measurements: {error}"
                )),
            }
        }
    }
    let report = json!({ "schema_version": 1, "passed": problems.is_empty(), "environment": crate::docs_perf::environment(),
        "cli": cli, "cli_build_info": identity, "cases": cases, "failures": problems });
    let destination = crate::option(args, "--report")
        .map(PathBuf::from)
        .unwrap_or_else(|| work.join("report.json"));
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    std::fs::write(
        &destination,
        serde_json::to_vec_pretty(&report).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    println!(
        "search performance: {} / report {}",
        if problems.is_empty() {
            "passed"
        } else {
            "failed"
        },
        destination.display()
    );
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("; "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete() -> Value {
        let sample = json!({ "elapsed": 10.0, "id": 1, "resultId": "1", "focusImmediate": true });
        let metrics: Vec<_> = ["chinese", "english", "prefix", "empty", "fallback", "custom"].iter().map(|name| json!({ "name": name, "cold": vec![sample.clone(); 5], "warm": vec![sample.clone(); 5] })).collect();
        json!({ "schema_version": 1, "lang": "zh", "entrance": "404", "index": { "pages": 1000, "fallback": 500 }, "warm_requests": [],
            "metrics": metrics })
    }

    #[test]
    fn reports_require_every_query_and_five_samples_per_phase() {
        assert!(validate(&complete(), "zh", "404").is_ok());
        for pointer in ["/metrics", "/metrics/0/cold", "/metrics/0/warm"] {
            let mut data = complete();
            data.pointer_mut(pointer)
                .unwrap()
                .as_array_mut()
                .unwrap()
                .pop();
            assert!(validate(&data, "zh", "404").is_err());
        }
        let mut over_budget = complete();
        over_budget["metrics"][0]["warm"] = json!(vec![
            json!({ "elapsed": 101, "id": 1, "resultId": "1", "focusImmediate": true });
            5
        ]);
        assert!(validate(&over_budget, "zh", "404").is_err());
        let mut wrong_request = complete();
        wrong_request["metrics"][0]["cold"][0]["resultId"] = json!("2");
        assert!(validate(&wrong_request, "zh", "404").is_err());
        let mut missing_request = complete();
        let sample = missing_request["metrics"][0]["cold"][0]
            .as_object_mut()
            .unwrap();
        sample.remove("id");
        sample.remove("resultId");
        assert!(validate(&missing_request, "zh", "404").is_err());
    }
}
