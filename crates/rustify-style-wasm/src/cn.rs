//! Direct ECMAScript values at the cn boundary. No JSON input conversion.

use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::{Rc, Weak};

use js_sys::{Array, Function, JsString, Object, Reflect};
use rustify_style::cn::{
    self, Cn, CnError, EngineOptions, Utf16ClassGroupDef as Def, Utf16Config as Config, Utf16Map,
    Utf16Validator,
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Object)]
    fn box_value(value: &JsValue) -> JsValue;
    #[wasm_bindgen(js_name = Number, catch)]
    fn number_value(value: &JsValue) -> Result<f64, JsValue>;
}
type Text = Vec<u16>;

#[derive(Clone, Copy)]
pub(crate) struct Limits {
    pub(crate) depth: usize,
    pub(crate) nodes: usize,
    pub(crate) units: usize,
    pub(crate) tokens: usize,
    pub(crate) cache_bytes: usize,
}
impl Limits {
    pub(crate) fn parse(value: &JsValue) -> Result<Self, JsValue> {
        fn positive(value: &JsValue, key: &str) -> Result<usize, JsValue> {
            let n = get(value, key)?
                .as_f64()
                .ok_or_else(|| error("InvalidLimits", key))?;
            if !n.is_finite() || n <= 0.0 || n.fract() != 0.0 || n > u32::MAX as f64 {
                return Err(error("InvalidLimits", key));
            }
            Ok(n as usize)
        }
        Ok(Self {
            depth: positive(value, "maxDepth")?,
            nodes: positive(value, "maxNodes")?,
            units: positive(value, "maxUtf16Units")?,
            tokens: positive(value, "maxTokens")?,
            cache_bytes: positive(value, "cacheBytes")?,
        })
    }
}
#[derive(Default)]
struct Counters {
    nodes: Cell<usize>,
    units: Cell<usize>,
}
struct Budget {
    limits: Limits,
    counters: Rc<Counters>,
    active: Vec<JsValue>,
}
impl Budget {
    fn new(limits: Limits) -> Self {
        Self {
            limits,
            counters: Rc::new(Counters::default()),
            active: Vec::new(),
        }
    }
    fn with_counters(limits: Limits, counters: Rc<Counters>) -> Self {
        Self {
            limits,
            counters,
            active: Vec::new(),
        }
    }
    fn visit(&mut self, depth: usize) -> Result<(), JsValue> {
        self.counters
            .nodes
            .set(self.counters.nodes.get().saturating_add(1));
        if depth > self.limits.depth || self.counters.nodes.get() > self.limits.nodes {
            Err(error("InputLimit", "input depth or node limit exceeded"))
        } else {
            Ok(())
        }
    }
    fn text(&mut self, value: &JsValue) -> Result<Text, JsValue> {
        if !value.is_string() {
            return Err(error("TypeError", "expected string"));
        }
        let value: &JsString = value.unchecked_ref();
        self.counters.units.set(
            self.counters
                .units
                .get()
                .saturating_add(value.length() as usize),
        );
        if self.counters.units.get() > self.limits.units {
            return Err(error("InputLimit", "UTF-16 unit limit exceeded"));
        }
        Ok(value.iter().collect())
    }
    fn enter(&mut self, value: &JsValue) -> Result<(), JsValue> {
        if self.active.iter().any(|seen| seen == value) {
            return Err(error("CyclicInput", "cyclic input"));
        }
        self.active.push(value.clone());
        Ok(())
    }
    fn leave(&mut self) {
        self.active.pop();
    }
    fn check_tokens(&self, text: &[u16]) -> Result<(), JsValue> {
        let count = text
            .split(|c| char::from_u32(*c as u32).is_some_and(cn::validators::is_whitespace))
            .filter(|s| !s.is_empty())
            .count();
        if count > self.limits.tokens {
            Err(error("InputLimit", "token limit exceeded"))
        } else {
            Ok(())
        }
    }
}
pub(crate) fn get(value: &JsValue, key: &str) -> Result<JsValue, JsValue> {
    Reflect::get(value, &JsValue::from_str(key))
}
pub(crate) fn error(name: &str, message: &str) -> JsValue {
    let value = js_sys::Error::new(message);
    value.set_name(name);
    value.into()
}
pub(crate) fn core_error(value: CnError) -> JsValue {
    let name = if value.message.starts_with("cn: input limit:") {
        "InputLimit"
    } else if value.message.contains("cyclic") {
        "CyclicInput"
    } else {
        "Error"
    };
    error(name, &value.message)
}
pub(crate) fn js_text(units: &[u16]) -> JsValue {
    // Small chunks avoid argument-stack limits in String.fromCharCode.
    let mut output = JsString::from("");
    for chunk in units.chunks(8192) {
        output = output.concat(&JsString::from_char_code(chunk));
    }
    output.into()
}
fn truthy(value: &JsValue) -> bool {
    value.is_truthy()
}
fn append(output: &mut Text, text: &[u16]) {
    if !text.is_empty() {
        if !output.is_empty() {
            output.push(32);
        }
        output.extend_from_slice(text);
    }
}
// Reflect catches proxy/getter exceptions, and its native OwnPropertyKeys order
// preserves integer-index ordering. Non-enumerable shadowing is significant.
fn enumerate(
    value: &JsValue,
    inherited: bool,
    budget: &mut Budget,
    mut emit: impl FnMut(&JsValue, &mut Budget) -> Result<(), JsValue>,
) -> Result<(), JsValue> {
    let mut seen = HashSet::new();
    let mut prototypes = Vec::new();
    let mut current = value.clone();
    // The fixed JS runtime snapshots prototype key lists before invoking
    // property getters; later deletions are still checked via descriptors.
    while !current.is_null() {
        if prototypes.iter().any(|(v, _)| v == &current) {
            return Err(error("CyclicInput", "cyclic prototype"));
        }
        if prototypes.len() > budget.limits.depth {
            return Err(error("InputLimit", "prototype depth limit exceeded"));
        }
        prototypes.push((current.clone(), Reflect::own_keys(&current)?));
        if !inherited {
            break;
        }
        current = Reflect::get_prototype_of(&current)?.into();
    }
    for (depth, (current, own)) in prototypes.into_iter().enumerate() {
        for key in own.iter() {
            budget.visit(depth)?;
            if !key.is_string() {
                continue;
            }
            let units = budget.text(&key)?;
            let descriptor =
                Reflect::get_own_property_descriptor(current.unchecked_ref::<Object>(), &key)?;
            if descriptor.is_undefined() || !seen.insert(units) {
                continue;
            }
            if truthy(&get(&descriptor, "enumerable")?) {
                emit(&key, budget)?;
            }
        }
    }
    Ok(())
}
fn keys(value: &JsValue, inherited: bool, budget: &mut Budget) -> Result<Vec<JsValue>, JsValue> {
    let mut keys = Vec::new();
    enumerate(value, inherited, budget, |key, _| {
        keys.push(key.clone());
        Ok(())
    })?;
    Ok(keys)
}
fn entries(
    value: &JsValue,
    inherited: bool,
    budget: &mut Budget,
) -> Result<Vec<(JsValue, JsValue)>, JsValue> {
    let mut entries = Vec::new();
    enumerate(value, inherited, budget, |key, _| {
        entries.push((key.clone(), Reflect::get(value, key)?));
        Ok(())
    })?;
    Ok(entries)
}
fn resolve(value: &JsValue, mode: u8, budget: &mut Budget, depth: usize) -> Result<Text, JsValue> {
    budget.visit(depth)?;
    if !truthy(value) {
        return Ok(Vec::new());
    }
    if value.is_string() {
        return budget.text(value);
    }
    if mode == 2 {
        return Ok(Vec::new());
    }
    // Upstream reads .length before testing whether clsx input is an array.
    let boxed = box_value(value);
    let length = get(&boxed, "length")?;
    if let Some(length) = length
        .as_f64()
        .filter(|_| mode == 0 || Array::is_array(value))
    {
        if length.is_nan() || length <= 0.0 {
            return Ok(Vec::new());
        }
        if !length.is_finite() || length.ceil() > budget.limits.nodes as f64 {
            return Err(error("InputLimit", "array-like length exceeds node limit"));
        }
        budget.enter(value)?;
        let mut out = Vec::new();
        let result = (|| {
            let mut i = 0usize;
            loop {
                let length = number_value(&get(&boxed, "length")?)?;
                if length.is_nan() || i as f64 >= length {
                    break;
                }
                if !length.is_finite() || length.ceil() > budget.limits.nodes as f64 {
                    return Err(error("InputLimit", "array-like length exceeds node limit"));
                }
                budget.visit(depth)?;
                let item = Reflect::get(&boxed, &JsValue::from_str(&i.to_string()))?;
                append(&mut out, &resolve(&item, mode, budget, depth + 1)?);
                i += 1;
            }
            Ok(out)
        })();
        budget.leave();
        return result;
    }
    if mode == 1 {
        if let Some(number) = value.as_f64() {
            // Core ryu-js provides ECMAScript NumberToString, including infinities.
            let text = cn::clsx(&[cn::ClassValue::Number(number)]);
            return budget.text(&JsValue::from_str(&text));
        }
        if value.is_object() {
            let mut out = Vec::new();
            enumerate(value, true, budget, |key, budget| {
                if truthy(&Reflect::get(value, key)?) {
                    if !out.is_empty() {
                        out.push(32);
                    }
                    out.extend_from_slice(&budget.text(key)?);
                }
                Ok(())
            })?;
            return Ok(out);
        }
    }
    Ok(Vec::new())
}
fn joined(args: &JsValue, mode: u8, limits: Limits) -> Result<Text, JsValue> {
    if !Array::is_array(args) {
        return Err(error("TypeError", "args must be an array"));
    }
    let mut budget = Budget::new(limits);
    budget.enter(args)?;
    let array: &Array = args.unchecked_ref();
    if array.length() as usize > limits.nodes {
        return Err(error("InputLimit", "argument count exceeds node limit"));
    }
    let mut out = Vec::new();
    for index in 0..array.length() {
        append(
            &mut out,
            &resolve(&Reflect::get(args, &index.into())?, mode, &mut budget, 0)?,
        );
    }
    budget.check_tokens(&out)?;
    if out.len() > limits.units {
        return Err(error("InputLimit", "joined string exceeds UTF-16 limit"));
    }
    Ok(out)
}

fn resolved_arguments(args: &JsValue, limits: Limits) -> Result<(Vec<Text>, bool, bool), JsValue> {
    if !Array::is_array(args) {
        return Err(error("TypeError", "args must be an array"));
    }
    let mut budget = Budget::new(limits);
    budget.enter(args)?;
    let mut values = args.clone();
    let mut original_depth = 0;
    let count = args.unchecked_ref::<Array>().length();
    let raw_two = count == 2;
    let mut merge_empty = count == 1;
    if count == 1 {
        let value = Reflect::get(args, &0.into())?;
        if Array::is_array(&value) {
            merge_empty = false;
            original_depth = 1;
            budget.visit(0)?;
            let slice = get(&value, "slice")?;
            let function: &Function = slice
                .dyn_ref()
                .ok_or_else(|| error("TypeError", "value.slice is not a function"))?;
            values = function.call0(&value)?;
        }
    }
    let count = get(&values, "length")?
        .as_f64()
        .ok_or_else(|| error("TypeError", "argument length must be numeric"))?;
    if !count.is_finite() || count < 0.0 || count > limits.nodes as f64 {
        return Err(error("InputLimit", "argument count exceeds node limit"));
    }
    let mut output = Vec::new();
    let mut all_strings = true;
    for index in 0..count as u32 {
        let value = Reflect::get(&values, &index.into())?;
        all_strings &= value.is_string();
        let text = resolve(&value, 1, &mut budget, original_depth)?;
        if !text.is_empty() {
            output.push(text);
        }
    }
    let mut joined = Vec::new();
    for value in &output {
        append(&mut joined, value);
    }
    budget.check_tokens(&joined)?;
    if joined.len() > limits.units {
        return Err(error("InputLimit", "joined string exceeds UTF-16 limit"));
    }
    Ok((output, raw_two && count == 2.0 && all_strings, merge_empty))
}

pub(crate) struct Parser {
    limits: Limits,
    compile_counters: Rc<Counters>,
    thrown: Rc<RefCell<Option<JsValue>>>,
    theme_source: RefCell<Option<JsValue>>,
    validators: RefCell<Vec<(JsValue, Utf16Validator)>>,
    theme_getters: RefCell<Vec<(JsValue, cn::Utf16ThemeGetter)>>,
    sequences: RefCell<Vec<(JsValue, cn::config::Utf16Sequence)>>,
}
impl Parser {
    pub(crate) fn new(limits: Limits) -> Rc<Self> {
        Rc::new(Self {
            limits,
            compile_counters: Rc::new(Counters::default()),
            thrown: Rc::new(RefCell::new(None)),
            theme_source: RefCell::new(None),
            validators: RefCell::new(Vec::new()),
            theme_getters: RefCell::new(Vec::new()),
            sequences: RefCell::new(Vec::new()),
        })
    }
    pub(crate) fn propagate(&self, failure: CnError) -> JsValue {
        self.thrown
            .borrow_mut()
            .take()
            .unwrap_or_else(|| core_error(failure))
    }
    fn callback_error(&self, thrown: JsValue) -> CnError {
        *self.thrown.borrow_mut() = Some(thrown);
        CnError::new("cn: JavaScript callback threw")
    }
    fn sequence(self: &Rc<Self>, value: &JsValue) -> Def {
        if let Some((_, sequence)) = self.sequences.borrow().iter().find(|(v, _)| v == value) {
            return Def::Sequence(sequence.clone());
        }
        let value = value.clone();
        let source = value.clone();
        let weak = Rc::downgrade(self);
        let sequence: cn::config::Utf16Sequence = Rc::new(move || {
            let parser = weak
                .upgrade()
                .ok_or_else(|| CnError::new("cn: disposed parser"))?;
            let iterator = js_sys::try_iter(&source)
                .map_err(|e| parser.callback_error(e))?
                .ok_or_else(|| {
                    parser.callback_error(error("TypeError", "class group is not iterable"))
                })?;
            let weak = weak.clone();
            Ok(Box::new(iterator.map(move |item| {
                let parser = weak
                    .upgrade()
                    .ok_or_else(|| CnError::new("cn: disposed parser"))?;
                let value = item.map_err(|e| parser.callback_error(e))?;
                Ok(parser.deferred(value))
            })) as cn::config::Utf16DefinitionIterator)
        });
        self.sequences
            .borrow_mut()
            .push((value.clone(), sequence.clone()));
        Def::Sequence(sequence)
    }
    fn deferred(self: &Rc<Self>, value: JsValue) -> Def {
        let parser = Rc::downgrade(self);
        Def::Deferred(Rc::new(move || {
            let parser = parser
                .upgrade()
                .ok_or_else(|| CnError::new("cn: disposed parser"))?;
            parser
                .definition(
                    &value,
                    &mut Budget::with_counters(parser.limits, parser.compile_counters.clone()),
                    0,
                )
                .map_err(|e| parser.callback_error(e))
        }))
    }
    fn definitions(
        self: &Rc<Self>,
        value: &JsValue,
        single: bool,
        budget: &mut Budget,
        depth: usize,
    ) -> Result<Vec<Def>, JsValue> {
        budget.visit(depth)?;
        Ok(if single {
            vec![self.deferred(value.clone())]
        } else {
            vec![self.sequence(value)]
        })
    }
    fn definition(
        self: &Rc<Self>,
        value: &JsValue,
        budget: &mut Budget,
        depth: usize,
    ) -> Result<Def, JsValue> {
        budget.visit(depth)?;
        if value.is_string() {
            return Ok(Def::Literal(budget.text(value)?));
        }
        if value.is_function() {
            if get(value, "isThemeGetter")?.as_bool() == Some(true) {
                if let Some((_, callback)) =
                    self.theme_getters.borrow().iter().find(|(v, _)| v == value)
                {
                    return Ok(Def::ThemeGetter(callback.clone()));
                }
                let function: Function = value.clone().unchecked_into();
                let parser: Weak<Self> = Rc::downgrade(self);
                let callback: cn::Utf16ThemeGetter = Rc::new(move |theme| {
                    let parser = parser
                        .upgrade()
                        .ok_or_else(|| CnError::new("cn: disposed parser"))?;
                    let original = parser.theme_source.borrow().clone();
                    let input = if let Some(original) = original {
                        get(&original, "theme").map_err(|e| parser.callback_error(e))?
                    } else {
                        parser
                            .group_object(theme)
                            .map_err(|e| parser.callback_error(e))?
                    };
                    let output = function
                        .call1(&JsValue::UNDEFINED, &input)
                        .map_err(|e| parser.callback_error(e))?;
                    parser
                        .definitions(
                            &output,
                            false,
                            &mut Budget::with_counters(
                                parser.limits,
                                parser.compile_counters.clone(),
                            ),
                            0,
                        )
                        .map_err(|e| parser.callback_error(e))
                });
                self.theme_getters
                    .borrow_mut()
                    .push((value.clone(), callback.clone()));
                return Ok(Def::ThemeGetter(callback));
            }
            if let Some((_, callback)) = self.validators.borrow().iter().find(|(v, _)| v == value) {
                return Ok(Def::Validator(callback.clone()));
            }
            let function: Function = value.clone().unchecked_into();
            let thrown = self.thrown.clone();
            let callback: Utf16Validator = Rc::new(move |units| {
                function
                    .call1(&JsValue::UNDEFINED, &js_text(units))
                    .map(|v| truthy(&v))
                    .map_err(|e| {
                        *thrown.borrow_mut() = Some(e);
                        CnError::new("cn: JavaScript callback threw")
                    })
            });
            self.validators
                .borrow_mut()
                .push((value.clone(), callback.clone()));
            return Ok(Def::Validator(callback));
        }
        if value.is_null() || value.is_undefined() {
            return Err(error(
                "TypeError",
                "Cannot convert undefined or null to object",
            ));
        }
        let boxed;
        let value = if value.is_object() {
            value
        } else {
            boxed = box_value(value);
            &boxed
        };
        let own = keys(value, false, budget)?;
        if own.len() == 1 {
            let key = budget.text(&own[0])?;
            if key == "$t".encode_utf16().collect::<Vec<_>>() && get(value, "$t")?.is_string() {
                let key = budget.text(&get(value, "$t")?)?;
                let parser = Rc::downgrade(self);
                return Ok(Def::Deferred(Rc::new(move || {
                    let parser = parser
                        .upgrade()
                        .ok_or_else(|| CnError::new("cn: disposed parser"))?;
                    let source = parser
                        .theme_source
                        .borrow()
                        .clone()
                        .ok_or_else(|| CnError::new("cn: missing theme source"))?;
                    let result = (|| {
                        let theme = get(&source, "theme")?;
                        let value = Reflect::get(&theme, &js_text(&key))?;
                        Ok(if value.is_null() || value.is_undefined() {
                            Def::Object(Vec::new())
                        } else {
                            parser.sequence(&value)
                        })
                    })();
                    result.map_err(|e| parser.callback_error(e))
                })));
            }
        }
        let own = keys(value, false, budget)?;
        if own.len() == 1 {
            let key = budget.text(&own[0])?;
            if key == "$v".encode_utf16().collect::<Vec<_>>() && get(value, "$v")?.is_string() {
                let name = get(value, "$v")?;
                let text = budget.text(&name)?;
                let op = cn::validators::NAMES
                    .iter()
                    .position(|name| name.encode_utf16().eq(text.iter().copied()))
                    .ok_or_else(|| {
                        let failure = error("Error", "");
                        let mut message: Text = "cn: unknown validator \"".encode_utf16().collect();
                        message.extend_from_slice(&text);
                        message.push(34);
                        match Reflect::set(&failure, &"message".into(), &js_text(&message)) {
                            Ok(_) => failure,
                            Err(thrown) => thrown,
                        }
                    })?;
                return Ok(Def::Builtin(op));
            }
        }
        budget.enter(value)?;
        let result: Result<Utf16Map<Vec<Def>>, JsValue> = (|| {
            entries(value, false, budget)?
                .into_iter()
                .map(|(key, value)| {
                    Ok((
                        budget.text(&key)?,
                        self.definitions(&value, false, budget, depth + 1)?,
                    ))
                })
                .collect()
        })();
        budget.leave();
        Ok(Def::Object(result?))
    }
    fn groups(
        self: &Rc<Self>,
        value: &JsValue,
        single: bool,
        budget: &mut Budget,
    ) -> Result<Utf16Map<Vec<Def>>, JsValue> {
        if !value.is_object() || value.is_null() {
            return Err(error("TypeError", "expected class group map"));
        }
        budget.enter(value)?;
        let result = (|| {
            entries(value, false, budget)?
                .into_iter()
                .map(|(key, value)| {
                    Ok((
                        budget.text(&key)?,
                        self.definitions(&value, single, budget, 0)?,
                    ))
                })
                .collect()
        })();
        budget.leave();
        result
    }
    fn strings(&self, value: &JsValue, budget: &mut Budget) -> Result<Vec<Text>, JsValue> {
        if !Array::is_array(value) {
            return Err(error("TypeError", "expected string array"));
        }
        let array: &Array = value.unchecked_ref();
        (0..array.length())
            .map(|i| {
                budget.visit(0)?;
                budget.text(&Reflect::get(value, &i.into())?)
            })
            .collect()
    }
    pub(crate) fn complete(self: &Rc<Self>, value: &JsValue) -> Result<Config, JsValue> {
        let mut budget = Budget::with_counters(self.limits, self.compile_counters.clone());
        *self.theme_source.borrow_mut() = Some(value.clone());
        Ok(Config {
            theme: Vec::new(),
            class_groups: self.groups(&get(value, "classGroups")?, false, &mut budget)?,
            conflicting_class_groups: Vec::new(),
            conflicting_class_group_modifiers: Vec::new(),
            order_sensitive_modifiers: Vec::new(),
            postfix_lookup_class_groups: Vec::new(),
            prefix: None,
        })
    }
    fn iterable_strings(&self, value: &JsValue, budget: &mut Budget) -> Result<Vec<Text>, JsValue> {
        let iterator = js_sys::try_iter(value)?
            .ok_or_else(|| error("TypeError", "targets are not iterable"))?;
        iterator
            .map(|value| {
                budget.visit(0)?;
                budget.text(&value?)
            })
            .collect()
    }
    fn mapped_value(
        &self,
        value: JsValue,
        ids: cn::compiler::Utf16GroupIds,
    ) -> Result<JsValue, JsValue> {
        let value = if value.is_null() || value.is_undefined() {
            Array::new().into()
        } else {
            value
        };
        let method = get(&value, "map")?;
        let method: &Function = method
            .dyn_ref()
            .ok_or_else(|| error("TypeError", "targets.map is not a function"))?;
        let limit = self.limits.units;
        let callback =
            Closure::<dyn Fn(JsValue) -> Result<JsValue, JsValue>>::new(move |value: JsValue| {
                if !value.is_string() {
                    return Ok(JsValue::UNDEFINED);
                }
                let string: &JsString = value.unchecked_ref();
                if string.length() as usize > limit {
                    return Err(error("InputLimit", "mapped key exceeds UTF-16 limit"));
                }
                let key: Vec<_> = string.iter().collect();
                Ok(ids.get(&key).map_or(JsValue::UNDEFINED, |id| (*id).into()))
            })
            .into_js_value();
        let mapped = method.call1(&value, &callback)?;
        if !Array::is_array(&mapped) {
            return Err(error("TypeError", "mapped targets must be an array"));
        }
        Ok(mapped)
    }
    fn mapped_ids(
        &self,
        mapped: &JsValue,
        budget: &mut Budget,
        base: bool,
    ) -> Result<Vec<i32>, JsValue> {
        if base {
            let length = get(mapped, "length")?;
            if !truthy(&length) {
                return Ok(Vec::new());
            }
            let length = length
                .as_f64()
                .ok_or_else(|| error("InputLimit", "mapped length must be numeric"))?;
            if !length.is_finite()
                || length < 0.0
                || length.fract() != 0.0
                || length > self.limits.nodes as f64
            {
                return Err(error("InputLimit", "mapped length exceeds node limit"));
            }
        }
        let iterator = js_sys::try_iter(mapped)?
            .ok_or_else(|| error("TypeError", "mapped targets are not iterable"))?;
        iterator
            .map(|item| {
                budget.visit(0)?;
                let number = number_value(&item?)?;
                Ok(if !number.is_finite() || number == 0.0 {
                    0
                } else {
                    number.trunc().rem_euclid(4294967296.0) as u32 as i32
                })
            })
            .collect()
    }
    fn compile(
        self: &Rc<Self>,
        source: &JsValue,
    ) -> Result<cn::compiler::Utf16CompiledTables, JsValue> {
        Ok(self.compile_model(source)?.compiled)
    }
    pub(crate) fn compile_model(
        self: &Rc<Self>,
        source: &JsValue,
    ) -> Result<cn::compiler::Utf16CompiledModel, JsValue> {
        let config = self.complete(source)?;
        let weak = Rc::downgrade(self);
        let raw = source.clone();
        let targets = Rc::new(move || {
            let parser = weak
                .upgrade()
                .ok_or_else(|| CnError::new("cn: disposed parser"))?;
            let result = (|| {
                let mut budget =
                    Budget::with_counters(parser.limits, parser.compile_counters.clone());
                let mut out = Vec::new();
                for field in ["conflictingClassGroups", "conflictingClassGroupModifiers"] {
                    for (_, value) in entries(&get(&raw, field)?, false, &mut budget)? {
                        out.extend(parser.iterable_strings(&value, &mut budget)?);
                    }
                }
                let value = get(&raw, "postfixLookupClassGroups")?;
                if !value.is_null() && !value.is_undefined() {
                    out.extend(parser.iterable_strings(&value, &mut budget)?);
                }
                Ok(out)
            })();
            result.map_err(|e| parser.callback_error(e))
        });
        let prepared = Rc::new(RefCell::new((JsValue::UNDEFINED, JsValue::UNDEFINED)));
        let pair = prepared.clone();
        let weak = Rc::downgrade(self);
        let raw = source.clone();
        let prepare_conflicts = Rc::new(move |name: &[u16], ids: cn::compiler::Utf16GroupIds| {
            let parser = weak
                .upgrade()
                .ok_or_else(|| CnError::new("cn: disposed parser"))?;
            let result = (|| {
                let base = Reflect::get(&get(&raw, "conflictingClassGroups")?, &js_text(name))?;
                let base = parser.mapped_value(base, ids.clone())?;
                let modifiers = Reflect::get(
                    &get(&raw, "conflictingClassGroupModifiers")?,
                    &js_text(name),
                )?;
                let modifiers = parser.mapped_value(modifiers, ids)?;
                *pair.borrow_mut() = (base, modifiers);
                Ok(())
            })();
            result.map_err(|e| parser.callback_error(e))
        });
        let weak = Rc::downgrade(self);
        let conflicts = Rc::new(
            move |_name: &[u16], modifiers: bool, _ids: cn::compiler::Utf16GroupIds| {
                let parser = weak
                    .upgrade()
                    .ok_or_else(|| CnError::new("cn: disposed parser"))?;
                let value = {
                    let pair = prepared.borrow();
                    if modifiers {
                        pair.1.clone()
                    } else {
                        pair.0.clone()
                    }
                };
                parser
                    .mapped_ids(
                        &value,
                        &mut Budget::with_counters(parser.limits, parser.compile_counters.clone()),
                        !modifiers,
                    )
                    .map_err(|e| parser.callback_error(e))
            },
        );
        let weak = Rc::downgrade(self);
        let raw = source.clone();
        let postfix = Rc::new(move |ids: cn::compiler::Utf16GroupIds| {
            let parser = weak
                .upgrade()
                .ok_or_else(|| CnError::new("cn: disposed parser"))?;
            let result = (|| {
                let mut budget =
                    Budget::with_counters(parser.limits, parser.compile_counters.clone());
                let value = get(&raw, "postfixLookupClassGroups")?;
                let mapped = parser.mapped_value(value, ids)?;
                parser.mapped_ids(&mapped, &mut budget, false)
            })();
            result.map_err(|e| parser.callback_error(e))
        });
        let weak = Rc::downgrade(self);
        let raw = source.clone();
        let finish = Rc::new(move || {
            let parser = weak
                .upgrade()
                .ok_or_else(|| CnError::new("cn: disposed parser"))?;
            let result = (|| {
                let mut budget =
                    Budget::with_counters(parser.limits, parser.compile_counters.clone());
                let order = get(&raw, "orderSensitiveModifiers")?;
                let join = get(&order, "join")?;
                let join: &Function = join.dyn_ref().ok_or_else(|| {
                    error(
                        "TypeError",
                        "orderSensitiveModifiers.join is not a function",
                    )
                })?;
                let order = vec![budget.text(&join.call1(&order, &" ".into())?)?];
                let prefix = get(&raw, "prefix")?;
                Ok((
                    order,
                    if prefix.is_undefined() {
                        None
                    } else {
                        Some(budget.text(&prefix)?)
                    },
                ))
            })();
            result.map_err(|e| parser.callback_error(e))
        });
        cn::compiler::compile_utf16_model(
            &config,
            Some((self.limits.depth, self.limits.nodes)),
            Some(&cn::compiler::Utf16CompileHooks {
                targets,
                prepare_conflicts,
                conflicts,
                postfix,
                finish,
            }),
        )
        .map_err(|e| self.propagate(e))
    }
    fn group_object(&self, groups: &Utf16Map<Vec<Def>>) -> Result<JsValue, JsValue> {
        let object = Object::new();
        for (key, defs) in groups {
            let array = Array::new();
            for def in defs {
                array.push(&self.definition_value(def)?);
            }
            define(&object, &js_text(key), &array.into())?;
        }
        Ok(object.into())
    }
    fn definition_value(&self, def: &Def) -> Result<JsValue, JsValue> {
        Ok(match def {
            Def::Literal(units) => js_text(units),
            Def::Object(groups) => self.group_object(groups)?,
            Def::Deferred(_) | Def::Sequence(_) => {
                return Err(error("Error", "lazy JS definitions are not serializable"))
            }
            Def::Builtin(op) => marker("$v", JsValue::from_str(cn::validators::NAMES[*op]))?,
            Def::Theme(key) => marker("$t", js_text(key))?,
            Def::Validator(callback) => self
                .validators
                .borrow()
                .iter()
                .find(|(_, v)| Rc::ptr_eq(v, callback))
                .map(|(f, _)| f.clone())
                .ok_or_else(|| error("Error", "unknown callback handle"))?,
            Def::ThemeGetter(callback) => self
                .theme_getters
                .borrow()
                .iter()
                .find(|(_, v)| Rc::ptr_eq(v, callback))
                .map(|(f, _)| f.clone())
                .ok_or_else(|| error("Error", "unknown theme getter handle"))?,
        })
    }
    pub(crate) fn config_value(&self, config: &Config) -> Result<JsValue, JsValue> {
        let object = Object::new();
        for (key, value) in [
            ("theme", self.group_object(&config.theme)?),
            ("classGroups", self.group_object(&config.class_groups)?),
            (
                "conflictingClassGroups",
                string_map_value(&config.conflicting_class_groups)?,
            ),
            (
                "conflictingClassGroupModifiers",
                string_map_value(&config.conflicting_class_group_modifiers)?,
            ),
            (
                "orderSensitiveModifiers",
                strings_value(&config.order_sensitive_modifiers),
            ),
            (
                "postfixLookupClassGroups",
                strings_value(&config.postfix_lookup_class_groups),
            ),
        ] {
            Reflect::set(&object, &key.into(), &value)?;
        }
        if let Some(prefix) = &config.prefix {
            Reflect::set(&object, &"prefix".into(), &js_text(prefix))?;
        }
        Ok(object.into())
    }
}
fn strings_value(values: &[Text]) -> JsValue {
    let array = Array::new();
    for text in values {
        array.push(&js_text(text));
    }
    array.into()
}
fn string_map_value(values: &Utf16Map<Vec<Text>>) -> Result<JsValue, JsValue> {
    let object = Object::new();
    for (key, values) in values {
        define(&object, &js_text(key), &strings_value(values))?;
    }
    Ok(object.into())
}
fn marker(key: &str, value: JsValue) -> Result<JsValue, JsValue> {
    let object = Object::new();
    Reflect::set(&object, &key.into(), &value)?;
    Ok(object.into())
}

/// One realm-local lazy cn instance. Inputs are native JS argument arrays.
#[wasm_bindgen]
pub struct CnSession {
    input: JsValue,
    parser: Rc<Parser>,
    engine: RefCell<Option<Rc<Cn>>>,
    initializing: Cell<bool>,
    call_depth: Cell<usize>,
}
#[wasm_bindgen]
impl CnSession {
    #[wasm_bindgen(constructor)]
    pub fn new(config: JsValue, limits: JsValue) -> Result<CnSession, JsValue> {
        let limits = Limits::parse(&limits)?;
        Ok(Self {
            input: config,
            parser: Parser::new(limits),
            engine: RefCell::new(None),
            initializing: Cell::new(false),
            call_depth: Cell::new(0),
        })
    }
    pub fn merge(&self, args: JsValue) -> Result<JsValue, JsValue> {
        self.merge_args(&args, 1)
    }
    pub fn tw_merge(&self, args: JsValue) -> Result<JsValue, JsValue> {
        self.merge_args(&args, 0)
    }
    pub fn join(&self, args: JsValue) -> Result<JsValue, JsValue> {
        let _call = self.enter_call()?;
        joined(&args, 1, self.parser.limits).map(|units| js_text(&units))
    }
    pub fn tw_join(&self, args: JsValue) -> Result<JsValue, JsValue> {
        let _call = self.enter_call()?;
        joined(&args, 0, self.parser.limits).map(|units| js_text(&units))
    }
    pub fn lite(&self, args: JsValue) -> Result<JsValue, JsValue> {
        let _call = self.enter_call()?;
        joined(&args, 2, self.parser.limits).map(|units| js_text(&units))
    }
    pub fn seen_before(&self, input: JsValue) -> Result<bool, JsValue> {
        let _call = self.enter_call()?;
        let mut budget = Budget::new(self.parser.limits);
        let units = budget.text(&input)?;
        budget.check_tokens(&units)?;
        Ok(self.get_engine()?.seen_before_utf16(&units))
    }
    pub fn cache_usage(&self) -> Result<JsValue, JsValue> {
        let engine = self.get_engine()?;
        let (count, bytes) = engine.cache_usage();
        let memo = engine.memo_string_bytes();
        let arguments = engine.argument_string_bytes();
        let output = Object::new();
        Reflect::set(&output, &"entries".into(), &(count as f64).into())?;
        Reflect::set(&output, &"stringBytes".into(), &(bytes as f64).into())?;
        Reflect::set(&output, &"memoStringBytes".into(), &(memo as f64).into())?;
        Reflect::set(
            &output,
            &"totalStringBytes".into(),
            &((bytes + memo + arguments) as f64).into(),
        )?;
        Reflect::set(
            &output,
            &"argumentStringBytes".into(),
            &(arguments as f64).into(),
        )?;
        Ok(output.into())
    }
    pub fn merge_string(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let _call = self.enter_call()?;
        let mut budget = Budget::new(self.parser.limits);
        let units = budget.text(&input)?;
        budget.check_tokens(&units)?;
        let engine = self.get_engine()?;
        engine
            .merge_utf16(&units)
            .map(|units| js_text(&units))
            .map_err(|e| self.parser.propagate(e))
    }
    pub fn merge_uncached(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let _call = self.enter_call()?;
        let mut budget = Budget::new(self.parser.limits);
        let units = budget.text(&input)?;
        budget.check_tokens(&units)?;
        let engine = self.get_engine()?;
        engine
            .merge_utf16_uncached(&units)
            .map(|units| js_text(&units))
            .map_err(|e| self.parser.propagate(e))
    }
}
struct CallGuard<'a>(&'a Cell<usize>);
impl Drop for CallGuard<'_> {
    fn drop(&mut self) {
        self.0.set(self.0.get() - 1);
    }
}
impl CnSession {
    fn enter_call(&self) -> Result<CallGuard<'_>, JsValue> {
        let depth = self.call_depth.get();
        if depth >= self.parser.limits.depth {
            return Err(error("InputLimit", "reentrant call depth exceeded"));
        }
        self.call_depth.set(depth + 1);
        Ok(CallGuard(&self.call_depth))
    }
    fn merge_args(&self, args: &JsValue, mode: u8) -> Result<JsValue, JsValue> {
        let _call = self.enter_call()?;
        if mode == 1 {
            let (values, two_strings, merge_empty) = resolved_arguments(args, self.parser.limits)?;
            if values.is_empty() && !merge_empty {
                return Ok(js_text(&[]));
            }
            let engine = self.get_engine()?;
            if values.is_empty() {
                return engine
                    .merge_utf16(&[])
                    .map(|units| js_text(&units))
                    .map_err(|e| self.parser.propagate(e));
            }
            return engine
                .merge_arguments_utf16(&values, two_strings)
                .map(|units| js_text(&units))
                .map_err(|e| self.parser.propagate(e));
        }
        let units = joined(args, mode, self.parser.limits)?;
        self.get_engine()?
            .merge_utf16(&units)
            .map(|units| js_text(&units))
            .map_err(|e| self.parser.propagate(e))
    }
    fn get_engine(&self) -> Result<Rc<Cn>, JsValue> {
        if let Some(engine) = self.engine.borrow().as_ref() {
            return Ok(engine.clone());
        }
        if self.input.is_undefined() {
            let options = EngineOptions {
                cache_size: 8192usize.min(self.parser.limits.nodes),
                cache_bytes: Some(self.parser.limits.cache_bytes),
                prefix: None,
            };
            let engine = Rc::new(Cn::default_with_options(options));
            engine.set_argument_limit(self.parser.limits.nodes);
            *self.engine.borrow_mut() = Some(engine.clone());
            return Ok(engine);
        }
        if self.initializing.replace(true) {
            return Err(error(
                "ReentrantConfiguration",
                "configuration recursively requested its own instance",
            ));
        }
        let result = (|| {
            let mut cache_size = 8192usize.min(self.parser.limits.nodes);
            let mut raw = self.parser.config_value(&cn::Config::default().into())?;
            if self.input.is_function() {
                let function: &Function = self.input.unchecked_ref();
                raw = function.call1(&JsValue::UNDEFINED, &raw)?;
            } else if !self.input.is_undefined() {
                let full = Reflect::has(&self.input, &"classGroups".into())?
                    && Reflect::has(&self.input, &"theme".into())?
                    && Reflect::has(&self.input, &"conflictingClassGroups".into())?;
                if full {
                    raw = self.input.clone()
                } else {
                    raw = merge_config_values(&raw, &self.input, self.parser.limits)?;
                    let cache = get(&self.input, "cacheSize")?;
                    if !cache.is_undefined() {
                        let n = cache.as_f64().ok_or_else(|| {
                            error("TypeError", "cacheSize must be nonnegative integer")
                        })?;
                        if !n.is_finite()
                            || n < 0.0
                            || n.fract() != 0.0
                            || n > self.parser.limits.nodes as f64
                        {
                            return Err(error("InputLimit", "cacheSize exceeds node limit"));
                        }
                        cache_size = n as usize
                    }
                }
            }
            self.parser.compile_counters.nodes.set(0);
            self.parser.compile_counters.units.set(0);

            let options = EngineOptions {
                cache_size,
                cache_bytes: Some(self.parser.limits.cache_bytes),
                prefix: None,
            };
            let compiled = self.parser.compile(&raw)?;
            let mut tables = compiled.tables;
            tables.prefix = compiled.prefix;
            Cn::from_utf16_tables(tables, compiled.validator_impls, options)
                .map(Rc::new)
                .map_err(|e| self.parser.propagate(e))
        })();
        self.initializing.set(false);
        if let Ok(engine) = &result {
            engine.set_argument_limit(self.parser.limits.nodes);
            *self.engine.borrow_mut() = Some(engine.clone())
        }
        result
    }
}
#[wasm_bindgen]
pub fn cn_default_config() -> Result<JsValue, JsValue> {
    // Output does not traverse caller data; helper's fixed configuration is bounded.
    let parser = Parser::new(Limits {
        depth: 1,
        nodes: 1,
        units: 1,
        tokens: 1,
        cache_bytes: 1,
    });
    parser.config_value(&cn::Config::default().into())
}
#[wasm_bindgen]
pub fn cn_from_theme(key: JsValue) -> Result<JsValue, JsValue> {
    if !key.is_string() {
        return Err(error("TypeError", "theme key must be string"));
    }
    marker("$t", key)
}
#[wasm_bindgen]
pub fn cn_validator_marker(name: &str) -> Result<JsValue, JsValue> {
    if !cn::validators::NAMES.contains(&name) {
        return Err(error("Error", "unknown validator"));
    }
    marker("$v", name.into())
}
#[wasm_bindgen]
pub fn cn_validator(name: &str, value: JsValue) -> Result<bool, JsValue> {
    let op = cn::validators::NAMES
        .iter()
        .position(|v| *v == name)
        .ok_or_else(|| error("Error", "unknown validator"))?;
    if !value.is_string() {
        return Err(error("TypeError", "validator input must be string"));
    }
    let value: &JsString = value.unchecked_ref();
    Ok(cn::validators::test(
        op,
        &String::from_utf16_lossy(&value.iter().collect::<Vec<_>>()),
    ))
}

pub(crate) fn define(object: &Object, key: &JsValue, value: &JsValue) -> Result<(), JsValue> {
    let descriptor = Object::new();
    for (key, value) in [
        ("value", value.clone()),
        ("enumerable", true.into()),
        ("configurable", true.into()),
        ("writable", true.into()),
    ] {
        Reflect::set(&descriptor, &key.into(), &value)?;
    }
    if !Reflect::define_property(object, key, &descriptor)? {
        return Err(error("TypeError", "cannot define configuration key"));
    }
    Ok(())
}
fn clone_object(value: &JsValue, budget: &mut Budget) -> Result<JsValue, JsValue> {
    let object = Object::new();
    for (key, value) in entries(value, false, budget)? {
        define(&object, &key, &value)?;
    }
    Ok(object.into())
}
fn copy_array(value: &JsValue, budget: &mut Budget) -> Result<JsValue, JsValue> {
    if !Array::is_array(value) {
        return Err(error("TypeError", "configuration list must be an array"));
    }
    let output = Array::new();
    let input: &Array = value.unchecked_ref();
    for i in 0..input.length() {
        budget.visit(0)?;
        output.push(&Reflect::get(value, &i.into())?);
    }
    Ok(output.into())
}
pub(crate) fn merge_config_values(
    base: &JsValue,
    extension: &JsValue,
    limits: Limits,
) -> Result<JsValue, JsValue> {
    let mut budget = Budget::new(limits);
    let config = clone_object(base, &mut budget)?;
    for key in [
        "theme",
        "classGroups",
        "conflictingClassGroups",
        "conflictingClassGroupModifiers",
    ] {
        Reflect::set(
            &config,
            &key.into(),
            &clone_object(&get(base, key)?, &mut budget)?,
        )?;
    }
    for key in ["orderSensitiveModifiers", "postfixLookupClassGroups"] {
        let mut value = get(base, key)?;
        if value.is_undefined() {
            value = Array::new().into()
        }
        Reflect::set(&config, &key.into(), &copy_array(&value, &mut budget)?)?;
    }
    let prefix = get(extension, "prefix")?;
    if !prefix.is_undefined() {
        Reflect::set(&config, &"prefix".into(), &prefix)?;
    }
    for (section, append) in [("override", false), ("extend", true)] {
        let groups = get(extension, section)?;
        if !truthy(&groups) {
            continue;
        }
        let modifiers = get(&groups, "orderSensitiveModifiers")?;
        if truthy(&modifiers) {
            let values = copy_array(&modifiers, &mut budget)?;
            let values = if append {
                concat_lists(
                    &get(&config, "orderSensitiveModifiers")?,
                    &values,
                    &mut budget,
                )?
            } else {
                values
            };
            Reflect::set(&config, &"orderSensitiveModifiers".into(), &values)?;
        }
        for key in [
            "theme",
            "classGroups",
            "conflictingClassGroups",
            "conflictingClassGroupModifiers",
        ] {
            let source = get(&groups, key)?;
            if !truthy(&source) {
                continue;
            }
            let target = get(&config, key)?;
            for (key, value) in entries(&source, true, &mut budget)? {
                if append {
                    if truthy(&value) {
                        let mut existing = Reflect::get(&target, &key)?;
                        if existing.is_null() || existing.is_undefined() {
                            existing = Array::new().into()
                        }
                        Reflect::set(
                            &target,
                            &key,
                            &concat_lists(&existing, &value, &mut budget)?,
                        )?;
                    }
                } else if !value.is_undefined() {
                    Reflect::set(&target, &key, &value)?;
                }
            }
        }
    }
    Ok(config)
}
fn concat_lists(base: &JsValue, add: &JsValue, budget: &mut Budget) -> Result<JsValue, JsValue> {
    let output = copy_array(base, budget)?;
    let array: &Array = output.unchecked_ref();
    if Array::is_array(add) {
        let input: &Array = add.unchecked_ref();
        for i in 0..input.length() {
            budget.visit(0)?;
            array.push(&Reflect::get(add, &i.into())?);
        }
    } else {
        budget.visit(0)?;
        array.push(add);
    }
    Ok(output)
}
#[wasm_bindgen]
pub fn cn_merge_configs(
    base: JsValue,
    extension: JsValue,
    limits: JsValue,
) -> Result<JsValue, JsValue> {
    merge_config_values(&base, &extension, Limits::parse(&limits)?)
}

fn read_tables(value: &JsValue, limits: Limits) -> Result<cn::Utf16Tables, JsValue> {
    let mut budget = Budget::new(limits);
    fn ints(value: &JsValue, budget: &mut Budget) -> Result<Vec<i32>, JsValue> {
        let length = get(value, "length")?
            .as_f64()
            .ok_or_else(|| error("TypeError", "expected table array"))?;
        if !length.is_finite()
            || length < 0.0
            || length.fract() != 0.0
            || length > budget.limits.nodes as f64
        {
            return Err(error("InputLimit", "table array length exceeds limit"));
        }
        (0..length as u32)
            .map(|i| {
                budget.visit(0)?;
                let n = Reflect::get(value, &i.into())?
                    .as_f64()
                    .ok_or_else(|| error("TypeError", "expected table integer"))?;
                if !n.is_finite() || n.fract() != 0.0 || n < i32::MIN as f64 || n > i32::MAX as f64
                {
                    return Err(error("TypeError", "invalid table integer"));
                }
                Ok(n as i32)
            })
            .collect()
    }
    let count = get(value, "GROUP_COUNT")?
        .as_f64()
        .ok_or_else(|| error("TypeError", "missing GROUP_COUNT"))?;
    if !count.is_finite() || count < 0.0 || count.fract() != 0.0 || count > limits.nodes as f64 {
        return Err(error("InputLimit", "invalid GROUP_COUNT"));
    }
    let prefix = get(value, "prefix")?;
    let parser = Parser::new(limits);
    let tables = cn::Utf16Tables {
        group_count: count as usize,
        edge_start: ints(&get(value, "edgeStart")?, &mut budget)?,
        label_start: ints(&get(value, "labelStart")?, &mut budget)?,
        edge_target: ints(&get(value, "edgeTarget")?, &mut budget)?,
        node_group: ints(&get(value, "nodeGroup")?, &mut budget)?,
        node_vlist: ints(&get(value, "nodeVlist")?, &mut budget)?,
        vlist_pat: ints(&get(value, "vlistPat")?, &mut budget)?,
        vlist_ops: ints(&get(value, "vlistOps")?, &mut budget)?,
        vlist_ref: ints(&get(value, "vlistRef")?, &mut budget)?,
        vlist_group: ints(&get(value, "vlistGroup")?, &mut budget)?,
        lit_anchor: ints(&get(value, "litAnchor")?, &mut budget)?,
        lit_group: ints(&get(value, "litGroup")?, &mut budget)?,
        lit_pool: ints(&get(value, "litPool")?, &mut budget)?,
        pool_offsets: ints(&get(value, "poolOffsets")?, &mut budget)?,
        adj_gid: ints(&get(value, "adjGid")?, &mut budget)?,
        adj_start: ints(&get(value, "adjStart")?, &mut budget)?,
        adj_tgt: ints(&get(value, "adjTgt")?, &mut budget)?,
        pat_gid: ints(&get(value, "patGid")?, &mut budget)?,
        pat_tgt: ints(&get(value, "patTgt")?, &mut budget)?,
        postfix_lookup_groups: ints(&get(value, "postfixLookupGroups")?, &mut budget)?,
        label_text: budget.text(&get(value, "labelText")?)?,
        pool_text: budget.text(&get(value, "poolText")?)?,
        order_sensitive_modifiers: budget.text(&get(value, "orderSensitiveModifiers")?)?,
        custom_validator_names: parser
            .strings(&get(value, "customValidatorNames")?, &mut budget)?,
        prefix: if prefix.is_undefined() {
            None
        } else {
            Some(budget.text(&prefix)?)
        },
    };
    tables.validate().map_err(core_error)?;
    Ok(tables)
}
fn tables_value(tables: &cn::Utf16Tables) -> Result<JsValue, JsValue> {
    let object = Object::new();
    Reflect::set(
        &object,
        &"GROUP_COUNT".into(),
        &(tables.group_count as f64).into(),
    )?;
    Reflect::set(
        &object,
        &"edgeStart".into(),
        &js_sys::Int32Array::from(tables.edge_start.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"labelStart".into(),
        &js_sys::Int32Array::from(tables.label_start.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"edgeTarget".into(),
        &js_sys::Int32Array::from(tables.edge_target.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"nodeGroup".into(),
        &js_sys::Int32Array::from(tables.node_group.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"nodeVlist".into(),
        &js_sys::Int32Array::from(tables.node_vlist.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"vlistPat".into(),
        &js_sys::Int32Array::from(tables.vlist_pat.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"vlistOps".into(),
        &js_sys::Int32Array::from(tables.vlist_ops.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"vlistRef".into(),
        &js_sys::Int32Array::from(tables.vlist_ref.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"vlistGroup".into(),
        &js_sys::Int32Array::from(tables.vlist_group.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"litAnchor".into(),
        &js_sys::Int32Array::from(tables.lit_anchor.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"litGroup".into(),
        &js_sys::Int32Array::from(tables.lit_group.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"litPool".into(),
        &js_sys::Int32Array::from(tables.lit_pool.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"poolOffsets".into(),
        &js_sys::Int32Array::from(tables.pool_offsets.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"adjGid".into(),
        &js_sys::Int32Array::from(tables.adj_gid.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"adjStart".into(),
        &js_sys::Int32Array::from(tables.adj_start.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"adjTgt".into(),
        &js_sys::Int32Array::from(tables.adj_tgt.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"patGid".into(),
        &js_sys::Int32Array::from(tables.pat_gid.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"patTgt".into(),
        &js_sys::Int32Array::from(tables.pat_tgt.as_slice()).into(),
    )?;
    Reflect::set(
        &object,
        &"postfixLookupGroups".into(),
        &js_sys::Int32Array::from(tables.postfix_lookup_groups.as_slice()).into(),
    )?;
    Reflect::set(&object, &"labelText".into(), &js_text(&tables.label_text))?;
    Reflect::set(&object, &"poolText".into(), &js_text(&tables.pool_text))?;
    Reflect::set(
        &object,
        &"orderSensitiveModifiers".into(),
        &js_text(&tables.order_sensitive_modifiers),
    )?;
    Reflect::set(
        &object,
        &"customValidatorNames".into(),
        &strings_value(&tables.custom_validator_names),
    )?;
    if let Some(prefix) = &tables.prefix {
        Reflect::set(&object, &"prefix".into(), &js_text(prefix))?;
    }
    Ok(object.into())
}
#[wasm_bindgen]
pub fn cn_compile_to_tables(config: JsValue, limits: JsValue) -> Result<JsValue, JsValue> {
    let parser = Parser::new(Limits::parse(&limits)?);
    let compiled = parser.compile(&config)?;
    let object = Object::new();
    let implementations = Object::new();
    for (name, callback) in compiled
        .tables
        .custom_validator_names
        .iter()
        .zip(&compiled.validator_impls)
    {
        let function = parser.definition_value(&Def::Validator(callback.clone()))?;
        define(&implementations, &js_text(name), &function)?;
    }
    Reflect::set(&object, &"tables".into(), &tables_value(&compiled.tables)?)?;
    Reflect::set(&object, &"validatorImpls".into(), &implementations)?;
    Reflect::set(
        &object,
        &"prefix".into(),
        &compiled
            .prefix
            .as_ref()
            .map_or(JsValue::UNDEFINED, |prefix| js_text(prefix)),
    )?;
    Ok(object.into())
}
/// Original public compiler model, including raw arrays and realm-local impls.
#[wasm_bindgen]
pub fn cn_compile_model(config: JsValue, limits: JsValue) -> Result<JsValue, JsValue> {
    let parser = Parser::new(Limits::parse(&limits)?);
    let model = parser.compile_model(&config)?;
    let table = &model.compiled.tables;
    let object = Object::new();
    let put = |name: &str, value: JsValue| define(&object, &name.into(), &value);
    let numbers = |values: &[i32]| -> JsValue {
        let array = Array::new();
        for value in values {
            array.push(&(*value as f64).into());
        }
        array.into()
    };
    let counts = |starts: &[i32]| starts.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>();
    let implementations = Object::new();
    for (name, callback) in table
        .custom_validator_names
        .iter()
        .zip(&model.compiled.validator_impls)
    {
        define(
            &implementations,
            &js_text(name),
            &parser.definition_value(&Def::Validator(callback.clone()))?,
        )?;
    }
    put("G", (table.group_count as f64).into())?;
    put("customNames", strings_value(&table.custom_validator_names))?;
    put("impls", implementations.into())?;
    put(
        "edgeCounts",
        js_sys::Int32Array::from(counts(&table.edge_start).as_slice()).into(),
    )?;
    put(
        "edgeLabelLen",
        js_sys::Int32Array::from(counts(&table.label_start).as_slice()).into(),
    )?;
    put("labelText", js_text(&table.label_text))?;
    put("edgeTargetActual", numbers(&table.edge_target))?;
    put(
        "nodeGroup",
        js_sys::Int32Array::from(table.node_group.as_slice()).into(),
    )?;
    put(
        "nodeVlist",
        js_sys::Int32Array::from(table.node_vlist.as_slice()).into(),
    )?;
    put("nodeCount", (table.node_group.len() as f64).into())?;
    put("totalEdges", (table.edge_target.len() as f64).into())?;
    put("patCounts", numbers(&counts(&table.vlist_pat)))?;
    put("patOps", numbers(&table.vlist_ops))?;
    put("listPat", numbers(&table.vlist_ref))?;
    put(
        "vlistGroup",
        js_sys::Int32Array::from(table.vlist_group.as_slice()).into(),
    )?;
    let literals = Array::new();
    for (anchor, tail, gid) in &model.literals {
        let entry = Object::new();
        define(&entry, &"anchor".into(), &(*anchor as f64).into())?;
        define(&entry, &"tail".into(), &js_text(tail))?;
        define(&entry, &"gid".into(), &(*gid as f64).into())?;
        literals.push(&entry);
    }
    put("litEntries", literals.into())?;
    let sets = Array::new();
    for tails in &model.sets {
        sets.push(&strings_value(tails));
    }
    put("sets", sets.clone().into())?;
    let attachments = Array::new();
    for (index, (anchor, gid, set)) in model.attachments.iter().enumerate() {
        let entry = Object::new();
        define(&entry, &"anchor".into(), &(*anchor as f64).into())?;
        define(&entry, &"gid".into(), &(*gid as f64).into())?;
        let tails = if model.attachment_owns_set[index] {
            sets.get(*set as u32)
        } else {
            strings_value(&model.sets[*set])
        };
        define(&entry, &"tails".into(), &tails)?;
        define(&entry, &"set".into(), &(*set as f64).into())?;
        attachments.push(&entry);
    }
    put("attachments", attachments.into())?;
    put(
        "uniqueTailCount",
        ((table.pool_offsets.len() / 2) as f64).into(),
    )?;
    put("adjGid", numbers(&table.adj_gid))?;
    put("adjCnt", numbers(&counts(&table.adj_start)))?;
    for (name, values) in [
        ("adjTgt", &table.adj_tgt),
        ("patGid", &table.pat_gid),
        ("patTgt", &table.pat_tgt),
        ("postfixLookup", &table.postfix_lookup_groups),
    ] {
        put(name, numbers(values))?;
    }
    put(
        "orderSensitiveModifiers",
        js_text(&table.order_sensitive_modifiers),
    )?;
    put(
        "prefix",
        model
            .compiled
            .prefix
            .as_ref()
            .map_or(JsValue::UNDEFINED, |prefix| js_text(prefix)),
    )?;
    Ok(object.into())
}
#[wasm_bindgen]
impl CnSession {
    #[wasm_bindgen(js_name=fromTables)]
    pub fn from_tables(
        tables: JsValue,
        implementations: JsValue,
        options: JsValue,
        limits: JsValue,
    ) -> Result<CnSession, JsValue> {
        let limits = Limits::parse(&limits)?;
        let parser = Parser::new(limits);
        let mut tables = read_tables(&tables, limits)?;
        let mut callbacks = Vec::new();
        for name in &tables.custom_validator_names {
            let function = Reflect::get(&implementations, &js_text(name))?;
            if !function.is_function() {
                return Err(error(
                    "Error",
                    &format!("cn: missing validator {}", String::from_utf16_lossy(name)),
                ));
            }
            if let Def::Validator(callback) =
                parser.definition(&function, &mut Budget::new(limits), 0)?
            {
                callbacks.push(callback)
            } else {
                return Err(error(
                    "TypeError",
                    "validator implementation is a theme getter",
                ));
            }
        }
        let mut cache_size = 8192usize.min(limits.nodes);
        if !options.is_undefined() {
            let prefix = get(&options, "prefix")?;
            if !prefix.is_undefined() {
                tables.prefix = Some(Budget::new(limits).text(&prefix)?)
            }
            let size = get(&options, "cacheSize")?;
            if !size.is_undefined() {
                let n = size
                    .as_f64()
                    .ok_or_else(|| error("TypeError", "invalid cacheSize"))?;
                if !n.is_finite() || n < 0.0 || n.fract() != 0.0 || n > limits.nodes as f64 {
                    return Err(error("InputLimit", "invalid cacheSize"));
                }
                cache_size = n as usize
            }
        }
        let engine = Cn::from_utf16_tables(
            tables,
            callbacks,
            EngineOptions {
                cache_size,
                cache_bytes: Some(limits.cache_bytes),
                prefix: None,
            },
        )
        .map_err(core_error)?;
        engine.set_argument_limit(limits.nodes);
        Ok(Self {
            input: JsValue::UNDEFINED,
            parser,
            engine: RefCell::new(Some(Rc::new(engine))),
            initializing: Cell::new(false),
            call_depth: Cell::new(0),
        })
    }
}
