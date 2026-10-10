//! When each content file last changed, from the repository's history.
//!
//! One `git log` reads the history of the content directory; the newest
//! commit that touched a file dates it. Dates are the commit's, in UTC, so a
//! rebuild on another machine or in another time zone writes the same
//! bytes, and neither the clock nor a file's modification time is read. A
//! project outside a repository, or a machine without git, has no dates.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

/// Project-relative path to the `YYYY-MM-DD` of the last commit touching it.
pub type Dates = BTreeMap<String, String>;

/// Reads the dates of every file under `dir` (project-relative) in `root`.
/// `Err` says why there are none.
pub fn read(root: &Path, dir: &str) -> Result<Dates, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args([
            "-c",
            "core.quotePath=false",
            "log",
            "--no-renames",
            "--relative",
            "--format=%x1e%ct",
            "--name-only",
            "--",
            dir,
        ])
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => "git is not installed".to_owned(),
            _ => format!("git could not run: {error}"),
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let first = stderr.lines().next().unwrap_or("git log failed").trim();
        return Err(first.trim_start_matches("fatal: ").to_owned());
    }
    Ok(parse(&String::from_utf8_lossy(&output.stdout)))
}

/// Reads `git log --format=%x1e%ct --name-only` output, newest commit first.
fn parse(log: &str) -> Dates {
    let mut dates = Dates::new();
    for commit in log.split('\u{1e}').skip(1) {
        let mut lines = commit.lines();
        let Some(Ok(seconds)) = lines.next().map(|line| line.trim().parse::<i64>()) else {
            continue;
        };
        let date = utc_date(seconds);
        for file in lines.map(str::trim).filter(|line| !line.is_empty()) {
            dates.entry(file.to_owned()).or_insert_with(|| date.clone());
        }
    }
    dates
}

/// The UTC calendar date of a Unix time, as `YYYY-MM-DD`.
pub fn utc_date(seconds: i64) -> String {
    // Howard Hinnant's days-to-civil conversion, in the proleptic Gregorian
    // calendar.
    let z = seconds.div_euclid(86_400) + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_dates() {
        assert_eq!(utc_date(0), "1970-01-01");
        assert_eq!(utc_date(951_782_400), "2000-02-29");
        assert_eq!(utc_date(1_791_590_399), "2026-10-09");
        assert_eq!(utc_date(1_791_590_400), "2026-10-10");
        assert_eq!(utc_date(-1), "1969-12-31");
    }

    #[test]
    fn the_newest_commit_dates_a_file() {
        let log =
            "\u{1e}1791590400\n\ncontent/a.md\n\u{1e}951782400\n\ncontent/a.md\ncontent/b.md\n";
        let dates = parse(log);
        assert_eq!(dates["content/a.md"], "2026-10-10");
        assert_eq!(dates["content/b.md"], "2000-02-29");
    }

    #[test]
    fn outside_a_repository_there_are_no_dates() {
        let dir = std::env::temp_dir().join(format!("rustify-docs-history-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        // A temporary directory is outside any repository (unless git is
        // missing, which is the same answer).
        assert!(read(&dir, "content").is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
