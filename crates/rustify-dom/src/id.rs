//! Deterministic identities within a rendered scope.

use leptos::prelude::{provide_context, use_context};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Clone)]
struct Ids {
    seed: String,
    next: Arc<AtomicU64>,
}

/// Starts an independent sequence for this Owner and its descendants.
/// Use the same seed and view when generating HTML and taking it over.
pub fn provide(seed: &str) {
    provide_context(Ids {
        seed: token(seed),
        next: Arc::new(AtomicU64::new(1)),
    });
}

/// Allocates an identity, or returns `None` outside a seeded scope.
pub fn next(prefix: &str) -> Option<String> {
    use_context::<Ids>().map(|ids| {
        let number = ids.next.fetch_add(1, Ordering::Relaxed);
        format!("rui-{}-{}-{number}", ids.seed, token(prefix))
    })
}

fn token(value: &str) -> String {
    value.bytes().fold(String::new(), |mut output, byte| {
        if byte.is_ascii_alphanumeric() || byte == b'-' {
            output.push(char::from(byte));
        } else {
            use std::fmt::Write;
            let _ = write!(output, "_{byte:02x}");
        }
        output
    })
}

#[cfg(test)]
mod tests {
    use super::token;

    #[test]
    fn tokens_are_class_safe_without_merging_distinct_seeds() {
        assert_eq!(token("counter-a"), "counter-a");
        assert_eq!(token("岛 1"), "_e5_b2_9b_201");
        assert_ne!(token("a b"), token("a_20b"));
    }
}
