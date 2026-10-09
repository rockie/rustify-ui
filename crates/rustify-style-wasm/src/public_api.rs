//! Public compiler values cross as ordinary JavaScript objects, including
//! explicit undefined fields and shared decoded Source references.

use core::PublicValue;
use js_sys::{Array, JsString, Object, Reflect};
use rustify_style::{
    compiler::{public_api as core, CompileError, Limits},
    text::Text,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use wasm_bindgen::{prelude::*, JsCast};

mod design_system;
mod graph;
mod setup;
#[cfg(feature = "upstream-units")]
mod units;
pub use design_system::DesignSystem;
pub use setup::compile_ast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Array, js_name = isArray, catch)]
    fn is_array(value: &JsValue) -> Result<bool, JsValue>;
    #[wasm_bindgen(js_namespace = Object, js_name = keys, catch)]
    fn own_keys(value: &JsValue) -> Result<Array, JsValue>;
}

#[derive(Deserialize)]
struct Configuration {
    #[serde(default)]
    options: core::CompileOptions,
    #[serde(default)]
    assets: Vec<core::Stylesheet>,
    /// The caller resolves `MissingAsset` errors through its own loader.
    #[serde(default)]
    loader: bool,
    /// Upstream's own test environment omits the license comment.
    #[serde(default = "emit_banner")]
    license_banner: bool,
    limits: Limits,
}
fn emit_banner() -> bool {
    true
}

fn error(error: CompileError) -> JsValue {
    #[derive(Serialize)]
    struct Envelope {
        kind: &'static str,
        error: CompileError,
    }
    let exception: JsValue = match &error {
        CompileError::RangeError { .. } => js_sys::RangeError::new("").into(),
        _ => js_sys::Error::new("").into(),
    };
    let message = match &error {
        CompileError::Syntax { message, .. } | CompileError::RangeError { message } => {
            message.clone()
        }
        _ => Text::from(error.to_string()),
    };
    if let Err(failure) = Reflect::set(&exception, &"message".into(), &text(&message)) {
        return failure;
    }
    // Serialize the typed cause directly: Value cannot hold lone surrogates.
    match serialized(&Envelope {
        kind: "compile",
        error,
    }) {
        Ok(cause) => caused_error(exception, cause),
        Err(failure) => failure,
    }
}
fn protocol(message: impl ToString) -> JsValue {
    let message = message.to_string();
    let exception = js_sys::TypeError::new(&message).into();
    match serialized(&json!({"kind":"protocol", "message":message})) {
        Ok(cause) => caused_error(exception, cause),
        Err(failure) => failure,
    }
}
fn caused_error(exception: JsValue, cause: JsValue) -> JsValue {
    let descriptor = Object::create(JsValue::NULL.unchecked_ref());
    let result = (|| {
        Reflect::set(&descriptor, &"value".into(), &cause)?;
        Reflect::set(&descriptor, &"writable".into(), &JsValue::TRUE)?;
        Reflect::set(&descriptor, &"configurable".into(), &JsValue::TRUE)?;
        Reflect::define_property(
            exception.unchecked_ref::<Object>(),
            &"cause".into(),
            &descriptor,
        )
    })();
    match result {
        Ok(true) => exception,
        Ok(false) => js_sys::Error::new("cannot define compiler error cause").into(),
        Err(failure) => failure,
    }
}
fn limit(resource: &str, value: usize) -> JsValue {
    error(CompileError::Limit {
        resource: resource.into(),
        limit: value,
    })
}
fn configuration(input: &str) -> Result<Configuration, JsValue> {
    let config: Configuration = serde_json::from_str(input).map_err(protocol)?;
    for (resource, value) in [
        ("input_bytes", config.limits.input_bytes),
        ("asset_bytes", config.limits.asset_bytes),
        ("candidate_count", config.limits.candidate_count),
        ("ast_depth", config.limits.ast_depth),
        ("css_bytes", config.limits.css_bytes),
        ("queued_bytes", config.limits.queued_bytes),
        ("cache_bytes", config.limits.cache_bytes),
    ] {
        if value == 0 {
            return Err(error(CompileError::InvalidLimit {
                resource: resource.into(),
            }));
        }
    }
    if input.len() > config.limits.queued_bytes {
        return Err(limit("queued_bytes", config.limits.queued_bytes));
    }
    Ok(config)
}

struct Budget {
    bytes: usize,
    limit: usize,
    resource: &'static str,
}
impl Budget {
    fn charge(&mut self, bytes: usize) -> Result<(), JsValue> {
        self.bytes = self.bytes.saturating_add(bytes);
        if self.bytes > self.limit {
            return Err(limit(self.resource, self.limit));
        }
        Ok(())
    }
    fn text(&mut self, value: &JsValue) -> Result<Text, JsValue> {
        if !value.is_string() {
            return Err(protocol("expected a string"));
        }
        let string: &JsString = value.unchecked_ref();
        let mut units = string.iter().peekable();
        let mut output = Vec::new();
        while let Some(unit) = units.next() {
            if matches!(unit, 0xd800..=0xdbff)
                && units
                    .peek()
                    .is_some_and(|unit| matches!(*unit, 0xdc00..=0xdfff))
            {
                self.charge(4)?;
                output.push(unit);
                if let Some(low) = units.next() {
                    output.push(low);
                }
            } else {
                self.charge(match unit {
                    0..=0x7f => 1,
                    0x80..=0x7ff => 2,
                    _ => 3,
                })?;
                output.push(unit);
            }
        }
        Ok(Text::from_units(output))
    }
}
fn candidates_text(input: &JsValue, limits: &Limits) -> Result<Vec<Text>, JsValue> {
    if !is_array(input)? {
        return Err(protocol("candidates must be an array"));
    }
    let count = array_length(input)?;
    if count > limits.candidate_count {
        return Err(limit("candidate_count", limits.candidate_count));
    }
    let mut budget = Budget {
        bytes: 0,
        limit: limits.queued_bytes,
        resource: "queued_bytes",
    };
    (0..count)
        .map(|index| budget.text(&Reflect::get(input, &JsValue::from_f64(index as f64))?))
        .collect()
}
fn array_length(input: &JsValue) -> Result<usize, JsValue> {
    let length = Reflect::get(input, &"length".into())?
        .as_f64()
        .filter(|length| {
            length.is_finite()
                && *length >= 0.0
                && *length <= u32::MAX as f64
                && length.fract() == 0.0
        })
        .ok_or_else(|| protocol("invalid array length"))?;
    Ok(length as usize)
}

fn read_public_value(
    input: &JsValue,
    depth: usize,
    limits: &Limits,
    budget: &mut Budget,
) -> Result<PublicValue, JsValue> {
    // Arrays and objects each add a JSON level; the core checks node depth.
    if depth > limits.ast_depth.saturating_mul(2).saturating_add(4) {
        return Err(limit("ast_depth", limits.ast_depth));
    }
    budget.charge(std::mem::size_of::<PublicValue>())?;
    if input.is_undefined() {
        return Ok(PublicValue::Object(Arc::new(core::PublicObject::from([(
            Text::from("undefined"),
            PublicValue::Bool(true),
        )]))));
    }
    if input.is_null() {
        return Ok(PublicValue::Null);
    }
    if input.is_string() {
        return budget.text(input).map(PublicValue::String);
    }
    if let Some(value) = input.as_bool() {
        return Ok(PublicValue::Bool(value));
    }
    if let Some(value) = input.as_f64() {
        return serde_json::Number::from_f64(value)
            .map(PublicValue::Number)
            .ok_or_else(|| protocol("non-finite AST number"));
    }
    if is_array(input)? {
        let count = array_length(input)?;
        let mut output = Vec::new();
        for index in 0..count {
            let value = Reflect::get(input, &JsValue::from_f64(index as f64))?;
            output.push(read_public_value(&value, depth + 1, limits, budget)?);
        }
        return Ok(PublicValue::Array(Arc::new(output)));
    }
    if !input.is_object() || input.is_function() {
        return Err(protocol("unsupported AST value"));
    }
    let mut output = core::PublicObject::new();
    for key in own_keys(input)?.iter() {
        let name = budget.text(&key)?;
        let value = Reflect::get(input, &key)?;
        output.insert(name, read_public_value(&value, depth + 1, limits, budget)?);
    }
    Ok(PublicValue::Object(Arc::new(output)))
}
fn set(output: &JsValue, key: &str, value: &JsValue) -> Result<(), JsValue> {
    set_key(output, &JsValue::from_str(key), value)
}
fn set_key(output: &JsValue, key: &JsValue, value: &JsValue) -> Result<(), JsValue> {
    let descriptor = Object::create(JsValue::NULL.unchecked_ref());
    Reflect::set(&descriptor, &"value".into(), value)?;
    Reflect::set(&descriptor, &"enumerable".into(), &JsValue::TRUE)?;
    Reflect::set(&descriptor, &"writable".into(), &JsValue::TRUE)?;
    Reflect::set(&descriptor, &"configurable".into(), &JsValue::TRUE)?;
    if !Reflect::define_property(output.unchecked_ref::<Object>(), key, &descriptor)? {
        return Err(protocol("cannot define output field"));
    }
    Ok(())
}
fn write_public_value(value: &PublicValue) -> Result<JsValue, JsValue> {
    Ok(match value {
        PublicValue::Null => JsValue::NULL,
        PublicValue::Bool(value) => JsValue::from_bool(*value),
        PublicValue::Number(value) => JsValue::from_f64(
            value
                .as_f64()
                .ok_or_else(|| protocol("invalid output number"))?,
        ),
        PublicValue::String(value) => text(value),
        PublicValue::Array(values) => {
            let output = Array::new();
            for value in values.iter() {
                output.push(&write_public_value(value)?);
            }
            output.into()
        }
        PublicValue::Object(values)
            if values.len() == 1
                && values.get(&Text::from("undefined")) == Some(&PublicValue::Bool(true)) =>
        {
            JsValue::UNDEFINED
        }
        PublicValue::Object(values)
            if values.len() == 1 && values.contains_key(&Text::from("bigint")) =>
        {
            let value = values[&Text::from("bigint")]
                .as_text()
                .ok_or_else(|| protocol("invalid output bigint"))?;
            js_sys::BigInt::new(&text(value))
                .map_err(JsValue::from)?
                .into()
        }
        PublicValue::Object(values) => {
            let output: JsValue = Object::new().into();
            for (key, value) in values.iter() {
                set_key(&output, &text(key), &write_public_value(value)?)?;
            }
            output
        }
    })
}
fn serialized(value: &impl Serialize) -> Result<JsValue, JsValue> {
    // Only metadata uses JSON. AST presence and Source identity use explicit adapters.
    js_sys::JSON::parse(&serde_json::to_string(value).map_err(protocol)?)
}
fn text(value: &Text) -> JsValue {
    let mut output = JsString::from("");
    for chunk in value.units().chunks(8192) {
        output = output.concat(&JsString::from_char_code(chunk));
    }
    output.into()
}
fn position(value: core::Position) -> Result<JsValue, JsValue> {
    let output: JsValue = Object::new().into();
    set(&output, "line", &JsValue::from_f64(value.line as f64))?;
    set(&output, "column", &JsValue::from_f64(value.column as f64))?;
    Ok(output)
}
fn source_map(map: &core::DecodedSourceMap) -> Result<JsValue, JsValue> {
    let sources = Array::new();
    for source in &map.sources {
        let output: JsValue = Object::new().into();
        set(
            &output,
            "url",
            &source
                .url
                .as_ref()
                .map_or(JsValue::NULL, |value| JsValue::from_str(value)),
        )?;
        set(
            &output,
            "content",
            &source.content.as_ref().map_or(JsValue::NULL, text),
        )?;
        set(&output, "ignore", &JsValue::from_bool(source.ignore))?;
        sources.push(&output);
    }
    let source_indexes: std::collections::BTreeMap<_, _> = map
        .sources
        .iter()
        .enumerate()
        .map(|(index, source)| (Arc::as_ptr(source) as usize, index as u32))
        .collect();
    let mappings = Array::new();
    for mapping in &map.mappings {
        let output: JsValue = Object::new().into();
        let original = if let Some(original) = &mapping.original_position {
            let output = position(original.position)?;
            let index = source_indexes
                .get(&(Arc::as_ptr(&original.source) as usize))
                .ok_or_else(|| protocol("map Source absent from table"))?;
            set(&output, "source", &sources.get(*index))?;
            output
        } else {
            JsValue::NULL
        };
        set(
            &output,
            "name",
            &mapping
                .name
                .as_ref()
                .map_or(JsValue::NULL, |value| JsValue::from_str(value)),
        )?;
        set(&output, "originalPosition", &original)?;
        set(
            &output,
            "generatedPosition",
            &position(mapping.generated_position)?,
        )?;
        mappings.push(&output);
    }
    let output: JsValue = Object::new().into();
    set(
        &output,
        "file",
        &map.file
            .as_ref()
            .map_or(JsValue::NULL, |value| JsValue::from_str(value)),
    )?;
    set(&output, "sources", &sources)?;
    set(&output, "mappings", &mappings)?;
    Ok(output)
}

#[wasm_bindgen]
pub struct CssCompiler {
    compiler: core::CssCompiler,
    limits: Limits,
}
#[wasm_bindgen]
impl CssCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new(input: JsValue, config: &str) -> Result<CssCompiler, JsValue> {
        let config = configuration(config)?;
        let mut budget = Budget {
            bytes: 0,
            limit: config.limits.input_bytes,
            resource: "input_bytes",
        };
        let input = budget.text(&input)?;
        let mut compiler = core::compile_text_with_loader(
            &input,
            config.options,
            config.assets,
            config.limits.clone(),
            config.loader,
        )
        .map_err(error)?;
        compiler.set_license_banner(config.license_banner);
        Ok(Self {
            compiler,
            limits: config.limits,
        })
    }
    pub fn build(&mut self, input: JsValue) -> Result<JsValue, JsValue> {
        let candidates = candidates_text(&input, &self.limits)?;
        self.compiler
            .build_text(&candidates)
            .map(|value| text(&value))
            .map_err(error)
    }
    #[wasm_bindgen(js_name = substituteFunctionsInValue)]
    pub fn substitute_functions_in_value(
        &self,
        input: JsValue,
        at_rule: Option<bool>,
    ) -> Result<JsValue, JsValue> {
        let input = Budget {
            bytes: 0,
            limit: self.limits.input_bytes,
            resource: "input_bytes",
        }
        .text(&input)?;
        self.compiler
            .substitute_functions_in_value(&input, at_rule.unwrap_or(false))
            .map(|value| text(&value))
            .map_err(error)
    }
    #[wasm_bindgen(js_name = decodeArbitraryValue)]
    pub fn decode_arbitrary_value(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let input = Budget {
            bytes: 0,
            limit: self.limits.input_bytes,
            resource: "input_bytes",
        }
        .text(&input)?;
        let value =
            rustify_style::compiler::value::decode_arbitrary_text(&input, self.limits.ast_depth)
                .map_err(error)?;
        if value.byte_len() > self.limits.css_bytes {
            return Err(limit("css_bytes", self.limits.css_bytes));
        }
        Ok(text(&value))
    }
    #[wasm_bindgen(js_name = buildSourceMap)]
    pub fn build_source_map(&self) -> Result<JsValue, JsValue> {
        source_map(&self.compiler.build_source_map().map_err(error)?)
    }
    #[wasm_bindgen(getter)]
    pub fn features(&self) -> u32 {
        self.compiler.features().0
    }
    #[wasm_bindgen(getter)]
    pub fn sources(&self) -> Result<JsValue, JsValue> {
        serialized(&self.compiler.sources().sources)
    }
    #[wasm_bindgen(getter)]
    pub fn root(&self) -> Result<JsValue, JsValue> {
        serialized(&self.compiler.sources().root)
    }
    #[wasm_bindgen(getter)]
    pub fn warnings(&self) -> Result<JsValue, JsValue> {
        serialized(&self.compiler.warnings())
    }
    #[wasm_bindgen(getter, js_name = stylesheetLoads)]
    pub fn stylesheet_loads(&self) -> Result<JsValue, JsValue> {
        serialized(&self.compiler.stylesheet_loads())
    }
}

#[wasm_bindgen]
pub struct AstCompiler {
    compiler: core::AstCompiler,
    input: core::PublicAst,
    input_js: JsValue,
    input_graph: graph::Graph,
    output_graph: graph::Graph,
    previous: Option<(core::PublicAst, JsValue)>,
    limits: Limits,
}
#[wasm_bindgen]
impl AstCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new(input: JsValue, config: &str) -> Result<AstCompiler, JsValue> {
        setup::compile(input, config)
    }
    pub fn build(&mut self, input: JsValue) -> Result<JsValue, JsValue> {
        let candidates = candidates_text(&input, &self.limits)?;
        let reserved = self
            .input_graph
            .retained_bytes()
            .saturating_add(self.output_graph.retained_bytes());
        let limit = self.limits.cache_bytes;
        let nodes = self
            .compiler
            .build_text_with_input_check(&candidates, |nodes, mutations, native| {
                let bytes = native
                    .saturating_add(reserved)
                    .saturating_add(graph::Graph::output_bytes(nodes, mutations));
                if bytes > limit {
                    return Err(CompileError::Limit {
                        resource: "cache_bytes".into(),
                        limit,
                    });
                }
                Ok(())
            })
            .map_err(error)?;
        if Arc::ptr_eq(&nodes, &self.input) {
            return Ok(self.input_js.clone());
        }
        if let Some((previous, value)) = &self.previous {
            if Arc::ptr_eq(previous, &nodes) {
                return Ok(value.clone());
            }
        }
        let (output, graph) = match self.input_graph.write(
            &nodes,
            self.compiler.input_mutations(),
            &self.output_graph,
        ) {
            Ok(output) => output,
            Err(failure) => {
                self.compiler.restore_output_after_crossing_error(
                    self.previous.as_ref().map(|(nodes, _)| nodes.clone()),
                );
                return Err(failure);
            }
        };
        self.output_graph = graph;
        self.previous = Some((nodes, output.clone()));
        Ok(output)
    }
    #[wasm_bindgen(getter)]
    pub fn features(&self) -> u32 {
        self.compiler.features().0
    }
    #[wasm_bindgen(getter)]
    pub fn sources(&self) -> Result<JsValue, JsValue> {
        serialized(&self.compiler.sources().sources)
    }
    #[wasm_bindgen(getter)]
    pub fn root(&self) -> Result<JsValue, JsValue> {
        serialized(&self.compiler.sources().root)
    }
    #[wasm_bindgen(getter, js_name = stylesheetLoads)]
    pub fn stylesheet_loads(&self) -> Result<JsValue, JsValue> {
        serialized(&self.compiler.stylesheet_loads())
    }
}
