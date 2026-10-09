/// Native equivalents of the public cn/clsx argument shapes.
#[derive(Clone, Debug)]
pub enum ClassValue<'a> {
    Text(&'a str),
    Number(f64),
    BigInt(&'a str),
    Bool(bool),
    Null,
    Undefined,
    List(Vec<ClassValue<'a>>),
    Dict(Vec<(&'a str, ClassValue<'a>)>),
    /// Truthy function-like input, ignored as a direct class argument.
    Function,
}

impl ClassValue<'_> {
    fn truthy(&self) -> bool {
        match self {
            Self::Text(value) => !value.is_empty(),
            Self::Number(value) => *value != 0.0 && !value.is_nan(),
            Self::BigInt(value) => !value.trim_start_matches(['+', '-', '0']).is_empty(),
            Self::Bool(value) => *value,
            Self::Null | Self::Undefined => false,
            Self::List(_) | Self::Dict(_) | Self::Function => true,
        }
    }
}

fn resolve(value: &ClassValue<'_>, clsx: bool, output: &mut Vec<String>) {
    if !value.truthy() {
        return;
    }
    match value {
        ClassValue::Text(text) => output.push((*text).to_owned()),
        ClassValue::Number(number) if clsx => {
            output.push(ryu_js::Buffer::new().format(*number).to_owned())
        }
        ClassValue::List(values) => {
            for value in values {
                resolve(value, clsx, output);
            }
        }
        ClassValue::Dict(entries) if clsx => {
            let mut joined = String::new();
            for (key, value) in entries {
                if value.truthy() {
                    if !joined.is_empty() {
                        joined.push(' ');
                    }
                    joined.push_str(key);
                }
            }
            if !joined.is_empty() {
                output.push(joined);
            }
        }
        ClassValue::Dict(entries) => {
            if let Some((_, ClassValue::Number(length))) =
                entries.iter().find(|(key, _)| *key == "length")
            {
                if length.is_finite() && *length > 0.0 {
                    for index in 0..length.ceil() as usize {
                        let key = index.to_string();
                        if let Some((_, value)) =
                            entries.iter().find(|(candidate, _)| *candidate == key)
                        {
                            resolve(value, false, output);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Join clsx-style inputs without conflict resolution.
pub fn clsx(inputs: &[ClassValue<'_>]) -> String {
    let mut output = Vec::new();
    for input in inputs {
        resolve(input, true, &mut output);
    }
    output.join(" ")
}

/// Join twJoin-style string/array inputs without conflict resolution.
pub fn tw_join(inputs: &[ClassValue<'_>]) -> String {
    let mut output = Vec::new();
    for input in inputs {
        resolve(input, false, &mut output);
    }
    output.join(" ")
}

/// Join only direct nonempty strings, as cn/lite does.
pub fn lite(inputs: &[ClassValue<'_>]) -> String {
    inputs
        .iter()
        .filter_map(|input| match input {
            ClassValue::Text(text) if !text.is_empty() => Some(*text),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ")
}
