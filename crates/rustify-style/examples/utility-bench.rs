//! Native release timings of the Rust cn and compiler, on the same workloads,
//! inputs and sample protocol as the fixed JS reference benchmarks
//! (`tests/utility/cn/bench.mjs`, `tests/utility/compiler/bench.mjs`) and the
//! Worker batch benchmark. Run through `mbx xtask utility-bench`.
//!
//! Usage: utility-bench <cn-runtime-fixtures.json> <compiler-fixture-dir> <out.json>
use rustify_style::cn::{ClassValue, Cn, Config, EngineOptions};
use rustify_style::compiler::{public_api, Compiler};
use rustify_style::protocol::{CompilerInput, Limits};
use serde_json::{json, Value};
use std::hint::black_box;
use std::time::Instant;

fn percentile(values: &[f64], fraction: f64) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[((sorted.len() as f64 * fraction).ceil() as usize).max(1) - 1]
}

/// Mean ns/call per sample, as the cn reference reports it.
fn per_call(samples: usize, calls: usize, mut run: impl FnMut(usize) -> usize) -> Value {
    let mut sink = 0usize;
    for index in 0..calls.min(20_000) {
        sink ^= run(index);
    }
    let mut times = Vec::with_capacity(samples);
    for _ in 0..samples {
        let start = Instant::now();
        for index in 0..calls {
            sink ^= run(index);
        }
        times.push(start.elapsed().as_nanos() as f64 / calls as f64);
    }
    black_box(sink);
    let mut sorted = times.clone();
    sorted.sort_by(f64::total_cmp);
    json!({"unit": "ns/call", "callsPerSample": calls, "samples": samples,
        "p50": sorted[sorted.len() / 2], "p95": percentile(&times, 0.95), "sortedSamples": sorted})
}

/// Milliseconds per call, as the compiler reference reports it.
fn per_run(times: &[f64]) -> Value {
    json!({"samples": times.len(), "p50Ms": percentile(times, 0.5), "p95Ms": percentile(times, 0.95),
        "minMs": times.iter().copied().fold(f64::INFINITY, f64::min),
        "maxMs": times.iter().copied().fold(0., f64::max)})
}

fn timed<T>(run: impl FnOnce() -> T) -> f64 {
    let start = Instant::now();
    black_box(run());
    start.elapsed().as_secs_f64() * 1e3
}

fn input() -> CompilerInput {
    CompilerInput {
        css: "@tailwind utilities;".into(),
        entry: "memory:input.css".into(),
    }
}

fn limits() -> Limits {
    Limits {
        input_bytes: 1 << 20,
        asset_bytes: 1 << 20,
        candidate_count: 1 << 16,
        ast_depth: 64,
        css_bytes: 64 << 20,
        queued_bytes: 1 << 22,
        cache_bytes: 1 << 30,
    }
}

/// The upstream `compile(css).build(candidates)` surface.
fn css_session() -> public_api::CssCompiler {
    public_api::compile(
        "@tailwind utilities;",
        public_api::CompileOptions::default(),
        vec![],
        limits(),
    )
    .unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [fixtures, compiler_dir, out] = &args[..] else {
        panic!("usage: utility-bench <cn-runtime-fixtures.json> <compiler-fixture-dir> <out.json>");
    };
    let read = |path: &str| -> Value {
        serde_json::from_str(&std::fs::read_to_string(path).expect(path)).expect(path)
    };

    // cn: the 186 upstream battery inputs, as the JS reference selects them.
    let cases = read(fixtures);
    let corpus: Vec<String> = cases["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|case| case["id"].as_str().unwrap().starts_with("upstream-") && case["api"] == "cn")
        .map(|case| case["args"][0].as_str().unwrap().to_string())
        .collect();
    let (samples, calls) = (7, 100_000);
    let cached = Cn::default();
    let uncached = Cn::default_with_options(EngineOptions {
        cache_size: 0,
        ..EngineOptions::default()
    });
    let variadic = |index: usize| {
        [
            ClassValue::Text("inline-flex items-center px-2 py-1"),
            ClassValue::Text(&corpus[index % corpus.len()]),
            ClassValue::Text("px-4"),
        ]
    };
    let cn = json!({
        "cnVariadicWarm": per_call(samples, calls, |i| cached.merge(&variadic(i)).unwrap().len()),
        "cnSingleWarm": per_call(samples, calls, |i| cached.merge(&[ClassValue::Text(&corpus[i % corpus.len()])]).unwrap().len()),
        "mergeStringWarm": per_call(samples, calls, |i| cached.merge_str(&corpus[i % corpus.len()]).unwrap().len()),
        "mergeUncached": per_call(samples, calls, |i| uncached.merge_str(&corpus[i % corpus.len()]).unwrap().len()),
        "clsxJoinOnly": per_call(samples, calls, |i| cached.join(&variadic(i)).len()),
        "createConfigAndFirstMerge": per_call(samples, 40, |i| Cn::new(Config::default()).unwrap().merge_str(&corpus[i % corpus.len()]).unwrap().len()),
    });

    // Compiler: the reference's cold/warm/no-op workload through the same
    // compile().build() surface, after the same ten warm-up sessions.
    let candidates = ["w-[137px]".to_string(), "bg-[#123456]".to_string()];
    for _ in 0..10 {
        let mut session = css_session();
        let output = session.build(&candidates).unwrap();
        assert!(output.contains("width: 137px") && output.contains("#123456"));
    }
    let iterations = 100;
    let cold: Vec<f64> = (0..iterations)
        .map(|_| {
            timed(|| {
                let mut session = css_session();
                session.build(&candidates).unwrap()
            })
        })
        .collect();
    let mut session = css_session();
    session.build(&candidates).unwrap();
    let warm: Vec<f64> = (0..iterations)
        .map(|i| timed(|| session.build(&[format!("w-[{}px]", 200 + i)]).unwrap()))
        .collect();
    let last = session.build(&[]).unwrap();
    let no_op: Vec<f64> = (0..iterations)
        .map(|_| {
            let start = Instant::now();
            let output = session.build(&candidates).unwrap();
            let elapsed = start.elapsed().as_secs_f64() * 1e3;
            assert_eq!(output, last);
            elapsed
        })
        .collect();

    // The Worker batch: a new session preparing every static and dynamic
    // fixture candidate at once, 5 samples of 20 runs after 5 warm-ups.
    let statics = read(&format!("{compiler_dir}/static-fixtures.json"));
    let variants = read(&format!("{compiler_dir}/dynamic-variant-fixtures.json"));
    let mut batch: Vec<String> = Vec::new();
    for candidate in statics["union"]["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .chain(
            variants["cases"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|case| case["candidates"].as_array().unwrap()),
        )
    {
        let candidate = candidate.as_str().unwrap().to_string();
        if !batch.contains(&candidate) {
            batch.push(candidate);
        }
    }
    let prepare = || {
        let mut session = Compiler::new(input(), vec![], limits()).unwrap();
        session.prepare_add(&batch).unwrap()
    };
    let shape = prepare();
    for _ in 0..5 {
        black_box(prepare());
    }
    let batch_samples: Vec<(f64, f64)> = (0..5)
        .map(|_| {
            let times: Vec<f64> = (0..20).map(|_| timed(prepare)).collect();
            (percentile(&times, 0.5), percentile(&times, 0.95))
        })
        .collect();
    let p50s: Vec<f64> = batch_samples.iter().map(|sample| sample.0).collect();
    let p95s: Vec<f64> = batch_samples.iter().map(|sample| sample.1).collect();

    let report = json!({
        "schemaVersion": 1,
        "build": "native release, this host",
        "environment": {"os": std::env::consts::OS, "arch": std::env::consts::ARCH},
        "cn": {"inputs": corpus.len(), "chars": corpus.iter().map(String::len).sum::<usize>(), "measurements": cn},
        "compiler": {
            "workload": "same css, initial candidates and 100 cold/warm/no-op calls as tests/utility/compiler/bench.mjs",
            "metrics": {"cold": per_run(&cold), "warm": per_run(&warm), "noOp": per_run(&no_op)},
        },
        "batch": {
            "candidates": batch.len(),
            "admitted": shape.report().admitted.len(),
            "cssBytes": shape.report().css.len(),
            "p50Ms": percentile(&p50s, 0.5), "p95Ms": percentile(&p95s, 0.5),
        },
    });
    std::fs::write(out, serde_json::to_string_pretty(&report).unwrap() + "\n").unwrap();
    println!(
        "{}",
        serde_json::to_string(&report["compiler"]["metrics"]).unwrap()
    );
}
