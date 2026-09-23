//! Writing the objects out and reading them back.
//!
//! Two formats, because an export has to survive both ways a file gets used: a
//! text one a person can open and diff, and a binary one that is only ever
//! read by this program. Both are written and parsed here so that "what an
//! export contains" has one answer, and so a round trip can be compared byte
//! for byte rather than field by field.
//!
//! Nothing here touches the application's state. Parsing produces records; the
//! application decides what to do with them, which is what makes a failed
//! import cost nothing.

/// One object, as a file carries it.
#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub id: u32,
    pub name: Option<String>,
    pub group: Option<u32>,
    pub locked: bool,
    pub size: f64,
}

/// The size of one binary record: id, group, locked, size.
const BINARY_RECORD: usize = 4 + 4 + 1 + 8;

/// Tab-separated, one object per line.
///
/// A name can contain anything a person typed, so the two characters that
/// would break the format are escaped. Without this an object called
/// "a\tb" would read back as two fields and every field after it would be
/// wrong - and it would look fine until somebody typed a tab.
pub fn to_text(records: &[Record]) -> Vec<u8> {
    let mut out = String::new();
    for record in records {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\n",
            record.id,
            escape(record.name.as_deref().unwrap_or("")),
            record.group.unwrap_or(0),
            u8::from(record.locked),
            record.size,
        ));
    }
    out.into_bytes()
}

/// Fixed-width little-endian records. No names: the binary form is the compact
/// one, and a name is the field that has no fixed width.
pub fn to_binary(records: &[Record]) -> Vec<u8> {
    let mut out = Vec::with_capacity(records.len() * BINARY_RECORD);
    for record in records {
        out.extend_from_slice(&record.id.to_le_bytes());
        out.extend_from_slice(&record.group.unwrap_or(0).to_le_bytes());
        out.push(u8::from(record.locked));
        out.extend_from_slice(&record.size.to_le_bytes());
    }
    out
}

/// Why a file that passed the size and kind checks still could not be read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Malformed {
    /// A line without the five fields an object needs.
    Line(usize),
    /// A field that is not the number it has to be.
    Field { line: usize, field: &'static str },
    /// A binary file whose length is not a whole number of records.
    Length(usize),
}

impl std::fmt::Display for Malformed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Line(line) => write!(f, "line {line} does not have five fields"),
            Self::Field { line, field } => write!(f, "line {line}: {field} is not a number"),
            Self::Length(len) => write!(f, "{len} bytes is not a whole number of records"),
        }
    }
}

/// Reads a text export.
///
/// The whole file is parsed before anything is returned, so a file that goes
/// wrong on its last line changes nothing at all. An import that applied the
/// first half of a broken file would leave the objects in a state no file
/// describes.
pub fn from_text(bytes: &[u8]) -> Result<Vec<Record>, Malformed> {
    let text = String::from_utf8_lossy(bytes);
    let mut records = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let line_number = index + 1;
        let fields: Vec<&str> = line.split('\t').collect();
        let [id, name, group, locked, size] = fields.as_slice() else {
            return Err(Malformed::Line(line_number));
        };
        let number = |value: &str, field| {
            value.parse::<u32>().map_err(|_| Malformed::Field {
                line: line_number,
                field,
            })
        };
        records.push(Record {
            id: number(id, "id")?,
            name: Some(unescape(name)),
            group: match number(group, "group")? {
                0 => None,
                group => Some(group),
            },
            locked: number(locked, "locked")? != 0,
            size: size.parse::<f64>().map_err(|_| Malformed::Field {
                line: line_number,
                field: "size",
            })?,
        });
    }
    Ok(records)
}

/// Reads a binary export.
pub fn from_binary(bytes: &[u8]) -> Result<Vec<Record>, Malformed> {
    if !bytes.len().is_multiple_of(BINARY_RECORD) {
        return Err(Malformed::Length(bytes.len()));
    }
    Ok(bytes
        .as_chunks::<BINARY_RECORD>()
        .0
        .iter()
        .map(|record| Record {
            id: u32::from_le_bytes(record[0..4].try_into().unwrap()),
            name: None,
            group: match u32::from_le_bytes(record[4..8].try_into().unwrap()) {
                0 => None,
                group => Some(group),
            },
            locked: record[8] != 0,
            size: f64::from_le_bytes(record[9..17].try_into().unwrap()),
        })
        .collect())
}

/// Reads whatever the name says this is.
pub fn parse(name: &str, bytes: &[u8]) -> Result<Vec<Record>, Malformed> {
    if name.to_ascii_lowercase().ends_with(".bin") {
        from_binary(bytes)
    } else {
        from_text(bytes)
    }
}

fn escape(name: &str) -> String {
    name.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    let mut chars = name.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{from_binary, from_text, parse, to_binary, to_text, Malformed, Record};

    fn sample() -> Vec<Record> {
        vec![
            Record {
                id: 1,
                name: Some("object-0001".into()),
                group: Some(3),
                locked: true,
                size: 45.0,
            },
            Record {
                id: 2,
                name: Some("object-0002".into()),
                group: None,
                locked: false,
                size: 0.0,
            },
        ]
    }

    #[test]
    fn text_written_and_read_back_is_the_same_objects() {
        let written = to_text(&sample());
        assert_eq!(from_text(&written), Ok(sample()));
        // And writing what was read produces the same bytes, which is what
        // makes a round trip something a test can compare rather than inspect.
        assert_eq!(to_text(&from_text(&written).unwrap()), written);
    }

    #[test]
    fn binary_written_and_read_back_keeps_everything_it_carries() {
        let written = to_binary(&sample());
        assert_eq!(written.len(), 2 * 17);
        let read = from_binary(&written).expect("readable");
        // The binary form has no names, and says so rather than inventing one.
        assert_eq!(read.iter().map(|r| r.id).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(read[0].group, Some(3));
        assert!(read[0].locked);
        assert_eq!(read[0].name, None);
        assert_eq!(read[1].size, 0.0);
        assert_eq!(to_binary(&read), written);
    }

    #[test]
    fn a_name_with_a_tab_in_it_survives_the_text_format() {
        // The character that would break the format, in the field a person
        // types into. Without escaping, this reads back as six fields.
        let records = vec![Record {
            id: 7,
            name: Some("a\tb\nc\\d".into()),
            group: None,
            locked: false,
            size: 5.0,
        }];
        let written = to_text(&records);
        assert_eq!(written.iter().filter(|b| **b == b'\n').count(), 1);
        assert_eq!(from_text(&written), Ok(records));
    }

    #[test]
    fn a_line_that_goes_wrong_stops_the_whole_import() {
        let mut written = to_text(&sample());
        written.extend_from_slice(b"9\tname\tnot-a-number\t0\t5\n");
        assert_eq!(
            from_text(&written),
            Err(Malformed::Field {
                line: 3,
                field: "group"
            })
        );

        // Too few fields is its own answer, because it is its own mistake.
        assert_eq!(from_text(b"1\t2\t3\n"), Err(Malformed::Line(1)));
    }

    #[test]
    fn a_binary_file_of_the_wrong_length_is_not_read_at_all() {
        let mut written = to_binary(&sample());
        written.push(0);
        assert_eq!(from_binary(&written), Err(Malformed::Length(35)));
    }

    #[test]
    fn the_name_decides_which_reader_runs() {
        let text = to_text(&sample());
        let binary = to_binary(&sample());
        assert_eq!(parse("objects.txt", &text), Ok(sample()));
        assert!(parse("OBJECTS.BIN", &binary).is_ok());
        // A binary file read as text is not readable, rather than read wrong.
        assert!(parse("objects.txt", &binary).is_err());
    }

    #[test]
    fn an_empty_file_is_no_objects_rather_than_an_error() {
        assert_eq!(from_text(b""), Ok(Vec::new()));
        assert_eq!(from_binary(b""), Ok(Vec::new()));
    }
}
