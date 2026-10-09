use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use tailwindcss_oxide::{
    extractor::{Extracted, Extractor},
    scanner::pre_process_input,
};

#[derive(Deserialize, Serialize)]
struct Fixture {
    name: String,
    extension: String,
    content: String,
    #[serde(default)]
    candidates: Vec<String>,
    #[serde(default)]
    variables: Vec<String>,
}

mod sources;

fn main() {
    if std::env::args().nth(2).as_deref() == Some("--sources") {
        sources::generate(&std::env::args().nth(1).unwrap());
        return;
    }
    let path = std::env::args().nth(1).expect("fixture path required");
    let data = std::fs::read(path).unwrap();
    let mut fixtures: Vec<Fixture> = serde_json::from_slice(&data).unwrap();
    for fixture in &mut fixtures {
        let content = pre_process_input(fixture.content.as_bytes(), &fixture.extension);
        let mut candidates = BTreeSet::new();
        let mut variables = BTreeSet::new();
        for line in content.split(|byte| *byte == b'\n') {
            let mut extractor = Extractor::new(line);
            let extracted = extractor.extract();
            for item in extracted {
                match item {
                    Extracted::Candidate(bytes) => {
                        candidates.insert(std::str::from_utf8(bytes).unwrap().to_owned());
                    }
                    Extracted::CssVariable(bytes) => {
                        variables.insert(std::str::from_utf8(bytes).unwrap().to_owned());
                    }
                }
            }
        }
        fixture.candidates = candidates.into_iter().collect();
        fixture.variables = variables.into_iter().collect();
    }
    println!("{}", serde_json::to_string_pretty(&fixtures).unwrap());
}
