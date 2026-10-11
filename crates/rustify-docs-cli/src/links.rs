//! `check --external-links`: whether the site's links to other sites still
//! answer.
//!
//! This is the one place the command talks to the network, and only when
//! asked: without the flag nothing here runs. Each address is requested
//! once however many pages link to it (HEAD, then GET where a server refuses
//! HEAD), a few at a time, each within a time limit; redirects are followed.
//! An answer of 400 or more, or no answer, is an error at every place the
//! address is written: a link that could not be checked is not reported as
//! working.

use std::collections::BTreeMap;
use std::ops::Range;
use std::time::Duration;

use rustify_docs::content::{Analysis, Diagnostic, Project, Snapshot, Target};
use rustify_docs::diagnostic::Issue;
use rustify_docs::markdown::RefKind;

/// Requests in flight at once.
const PARALLEL: usize = 8;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const TIMEOUT: Duration = Duration::from_secs(10);
const MAX_REDIRECTS: u32 = 5;

/// Where an address is written: a file, the text it is in, and the span.
struct Place<'a> {
    file: String,
    text: &'a str,
    span: Range<usize>,
}

fn is_web(url: &str) -> bool {
    let lower = url.trim_start().to_ascii_lowercase();
    lower.starts_with("http://") || lower.starts_with("https://")
}

/// The address a request goes to: without its `#fragment`.
fn address(url: &str) -> String {
    url.trim()
        .split_once('#')
        .map_or(url.trim(), |(address, _)| address)
        .to_owned()
}

/// Where `needle` first appears in `text`, for links whose position the
/// parser does not record (a home page's actions).
fn span_of(text: &str, needle: &str) -> Range<usize> {
    match text.find(needle) {
        Some(at) => at..at + needle.len(),
        None => 0..0,
    }
}

/// Every link to another site, by address, with each place it is written.
fn collect<'a>(
    project: &'a Project,
    snapshot: &'a Snapshot,
    analysis: &'a Analysis,
) -> BTreeMap<String, Vec<Place<'a>>> {
    let mut links: BTreeMap<String, Vec<Place<'a>>> = BTreeMap::new();
    for page in analysis.published() {
        let Some(text) = snapshot.markdown.get(&page.path) else {
            continue;
        };
        for (reference, target) in page.fragment.references.iter().zip(&page.targets) {
            if reference.kind == RefKind::Link
                && *target == Target::Keep
                && is_web(&reference.destination)
            {
                links
                    .entry(address(&reference.destination))
                    .or_default()
                    .push(Place {
                        file: page.file.clone(),
                        text,
                        span: reference.span.clone(),
                    });
            }
        }
        for (action, target) in page.front.actions.iter().zip(&page.action_targets) {
            if *target == Target::Keep && is_web(&action.link) {
                links.entry(address(&action.link)).or_default().push(Place {
                    file: page.file.clone(),
                    text,
                    span: span_of(text, action.link.trim()),
                });
            }
        }
    }
    let nav = &project.config.nav;
    for link in nav.top.iter().chain(&nav.social) {
        if is_web(&link.link) {
            links.entry(address(&link.link)).or_default().push(Place {
                file: project.config_file.clone(),
                text: &project.config_text,
                span: link.span.clone(),
            });
        }
    }
    links
}

/// What an address answered.
enum Answer {
    Status(u16),
    Unreachable(String),
}

fn request(agent: &ureq::Agent, url: &str) -> Answer {
    let status = |result: Result<ureq::http::Response<ureq::Body>, ureq::Error>| match result {
        Ok(response) => Answer::Status(response.status().as_u16()),
        Err(error) => Answer::Unreachable(error.to_string()),
    };
    match status(agent.head(url).call()) {
        // Some servers answer only GET.
        Answer::Status(405 | 501) => status(agent.get(url).call()),
        answer => answer,
    }
}

/// Checks every link to another site and returns a diagnostic for each
/// place a failing one is written, with the number of addresses checked.
pub fn check(
    project: &Project,
    snapshot: &Snapshot,
    analysis: &Analysis,
    version: &str,
) -> (Vec<Diagnostic>, usize) {
    let links = collect(project, snapshot, analysis);
    let config = ureq::Agent::config_builder()
        .timeout_connect(Some(CONNECT_TIMEOUT))
        .timeout_global(Some(TIMEOUT))
        .max_redirects(MAX_REDIRECTS)
        .http_status_as_error(false)
        .user_agent(format!("rustify-docs/{version} (link check)"))
        .build();
    let agent = ureq::Agent::new_with_config(config);
    let addresses: Vec<&String> = links.keys().collect();
    let answers =
        rustify_docs::generation::parallel_map(&addresses, PARALLEL, |url| request(&agent, url));
    let mut diagnostics = Vec::new();
    for (url, answer) in addresses.iter().zip(answers) {
        let issue = |span: Range<usize>| match &answer {
            Answer::Status(code) if *code >= 400 => Some(
                Issue::error(span, "external-link", format!("`{url}` answered {code}"))
                    .suggest("fix the address or remove the link"),
            ),
            Answer::Status(_) => None,
            Answer::Unreachable(reason) => Some(
                Issue::error(
                    span,
                    "external-link-unreachable",
                    format!("`{url}` could not be checked: {reason}"),
                )
                .suggest(
                    "check the address and the network, or run check without --external-links",
                ),
            ),
        };
        for place in &links[*url] {
            if let Some(issue) = issue(place.span.clone()) {
                diagnostics.push(Diagnostic::located(&place.file, place.text, issue));
            }
        }
    }
    (diagnostics, addresses.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_drop_fragments_and_only_the_web_is_checked() {
        assert_eq!(address("https://a.example/x#y"), "https://a.example/x");
        assert_eq!(address(" http://a.example "), "http://a.example");
        assert!(is_web("HTTPS://A.EXAMPLE"));
        assert!(!is_web("mailto:a@example.com"));
        assert!(!is_web("ftp://a.example"));
        assert!(!is_web("/guide/"));
        assert_eq!(
            span_of("link: https://a.example\n", "https://a.example"),
            6..23
        );
        assert_eq!(span_of("nothing", "https://a.example"), 0..0);
    }
}
