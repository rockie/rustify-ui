//! Reference site B through the real pipeline, timed: a cold analysis and
//! generation with styles, then warm ones after a configuration change and
//! after one page's change, each committed as a whole generation in memory.
//!
//! It writes the cold generation to `--out` for the size and browser
//! measurements and prints one JSON report on stdout.
//!
//! mbx run --profile docs-cli -p rustify-docs --example b_poc -- <site> --out <dir>

use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use rustify_docs::content::{self, FragmentCache, Mode, Project};
use rustify_docs::generation::{self, Generation};
use rustify_docs::highlight::Highlighter;

fn median(values: &mut [f64]) -> f64 {
    values.sort_by(f64::total_cmp);
    values[values.len() / 2]
}

fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let site = PathBuf::from(
        args.first()
            .expect("usage: b_poc <site> --out <dir> [--workers n] [--rounds n]"),
    );
    let option = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let out = PathBuf::from(option("--out").expect("--out <dir>"));
    let workers: usize = option("--workers").map_or_else(generation::default_workers, |n| {
        n.parse().expect("--workers")
    });
    let rounds: usize = option("--rounds").map_or(5, |n| n.parse().expect("--rounds"));

    let read_start = Instant::now();
    let (mut project, _) = Project::load(&site).expect("B's configuration loads");
    let (mut snapshot, problems) = content::snapshot(&project);
    assert!(problems.is_empty(), "B's files: {problems:?}");
    let read_ms = ms(read_start);

    // Cold: grammars, every fragment, the graph, every page, the stylesheet.
    let cold_start = Instant::now();
    let highlighter = Highlighter::new();
    let mut cache = FragmentCache::default();
    let analysis = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut cache,
        &highlighter,
        workers,
    );
    let analyze_ms = ms(cold_start);
    let generate_start = Instant::now();
    let cold = generation::generate(&project, &snapshot, &analysis, &Default::default(), workers)
        .expect("a generation");
    let generate_ms = ms(generate_start);
    let cold_ms = ms(cold_start);
    let errors = analysis.errors();
    let warnings = analysis.warnings();
    for diagnostic in analysis.diagnostics.iter().take(10) {
        eprintln!("{diagnostic:?}");
    }

    let write_start = Instant::now();
    if out.exists() {
        std::fs::remove_dir_all(&out).expect("clearing --out");
    }
    for (path, bytes) in &cold.files {
        let target = out.join(path);
        std::fs::create_dir_all(target.parent().unwrap()).expect("an output directory");
        std::fs::write(&target, bytes).expect("an output file");
    }
    let write_ms = ms(write_start);
    let css_bytes = cold.files[&cold.stylesheet_path].len();
    let candidates = cold.candidates;
    let html_bytes: usize = cold
        .files
        .iter()
        .filter(|(path, _)| path.ends_with(".html"))
        .map(|(_, bytes)| bytes.len())
        .sum();
    let pages = cold.pages;

    // Warm: a configuration change analyses and renders every page again
    // around the cached fragments and commits the whole generation.
    let current: RwLock<Arc<Generation>> = RwLock::new(Arc::new(cold));
    let mut hot_config = Vec::new();
    let mut hot_misses = Vec::new();
    for round in 0..rounds {
        project.config.site.name = format!("Reference site B, revision {}", round + 1);
        let start = Instant::now();
        let misses = cache.misses;
        let analysis = content::analyze(
            &project,
            &snapshot,
            Mode::Production,
            &mut cache,
            &highlighter,
            workers,
        );
        let next =
            generation::generate(&project, &snapshot, &analysis, &Default::default(), workers)
                .expect("a generation");
        *current.write().unwrap() = Arc::new(next);
        hot_config.push(ms(start));
        hot_misses.push(cache.misses - misses);
    }
    // Warm: one page's text changes.
    let mut hot_body = Vec::new();
    let page = snapshot.markdown.keys().nth(7).cloned().expect("a page");
    for round in 0..rounds {
        let text = format!("{}\nEdit {round}.\n", snapshot.markdown[&page]);
        snapshot.markdown.insert(page.clone(), Arc::from(text));
        let start = Instant::now();
        let analysis = content::analyze(
            &project,
            &snapshot,
            Mode::Production,
            &mut cache,
            &highlighter,
            workers,
        );
        let next =
            generation::generate(&project, &snapshot, &analysis, &Default::default(), workers)
                .expect("a generation");
        *current.write().unwrap() = Arc::new(next);
        hot_body.push(ms(start));
    }
    let committed = current.read().unwrap().pages;

    let hot_config_median = median(&mut hot_config.clone());
    let hot_body_median = median(&mut hot_body.clone());
    println!(
        "{{\"pages\":{pages},\"committed_pages\":{committed},\"workers\":{workers},\"read_ms\":{read_ms:.1},\
\"errors\":{errors},\"warnings\":{warnings},\
\"cold\":{{\"analyze_ms\":{analyze_ms:.1},\"generate_ms\":{generate_ms:.1},\"render_and_style_ms\":{cold_ms:.1},\"write_ms\":{write_ms:.1}}},\
\"hot_config_ms\":{hot_config:?},\"hot_config_median_ms\":{hot_config_median:.1},\"hot_config_misses\":{hot_misses:?},\
\"hot_body_ms\":{hot_body:?},\"hot_body_median_ms\":{hot_body_median:.1},\
\"html_bytes\":{html_bytes},\"css_bytes\":{css_bytes},\"candidates\":{candidates},\"stylesheet\":\"{}\",\"out\":\"{}\"}}",
        current.read().unwrap().stylesheet_path,
        out.display()
    );
}
