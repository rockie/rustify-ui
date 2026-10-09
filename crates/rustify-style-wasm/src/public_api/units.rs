//! Upstream internal helper names for executing the fixed upstream unit tests.
use super::*;
use rustify_style::compiler::units;

// The unit tests carry no quotas; these bound a runaway case.
const UNIT_LIMITS: Limits = Limits {
    input_bytes: 64 << 20,
    asset_bytes: 64 << 20,
    candidate_count: 1 << 20,
    ast_depth: 4096,
    css_bytes: 256 << 20,
    queued_bytes: 64 << 20,
    cache_bytes: 1 << 30,
};

fn input(value: &JsValue) -> Result<Text, JsValue> {
    Budget {
        bytes: 0,
        limit: UNIT_LIMITS.input_bytes,
        resource: "input_bytes",
    }
    .text(value)
}
fn list(values: Vec<Text>) -> JsValue {
    let output = Array::new();
    for value in &values {
        output.push(&text(value));
    }
    output.into()
}

#[wasm_bindgen(js_name = unitSegment)]
pub fn unit_segment(value: JsValue, separator: JsValue) -> Result<JsValue, JsValue> {
    units::segment(&input(&value)?, &input(&separator)?, UNIT_LIMITS.ast_depth)
        .map(list)
        .map_err(error)
}

#[wasm_bindgen(js_name = unitDecodeArbitraryValue)]
pub fn unit_decode_arbitrary_value(value: JsValue) -> Result<JsValue, JsValue> {
    units::decode_arbitrary_value(&input(&value)?, UNIT_LIMITS.ast_depth)
        .map(|value| text(&value))
        .map_err(error)
}

#[wasm_bindgen(js_name = unitExpand)]
pub fn unit_expand(pattern: JsValue) -> Result<JsValue, JsValue> {
    units::expand(&input(&pattern)?, &UNIT_LIMITS)
        .map(list)
        .map_err(error)
}

#[wasm_bindgen(js_name = unitCompare)]
pub fn unit_compare(left: JsValue, right: JsValue) -> Result<i32, JsValue> {
    Ok(units::compare(&input(&left)?, &input(&right)?) as i32)
}

#[wasm_bindgen(js_name = unitReplaceAlpha)]
pub fn unit_replace_alpha(color: JsValue, alpha: JsValue) -> Result<JsValue, JsValue> {
    Ok(text(&units::replace_alpha(
        &input(&color)?,
        &input(&alpha)?,
    )))
}

#[wasm_bindgen(js_name = unitToKeyPath)]
pub fn unit_to_key_path(path: JsValue) -> Result<JsValue, JsValue> {
    units::to_key_path(&input(&path)?).map(list).map_err(error)
}

#[wasm_bindgen(js_name = unitEscape)]
pub fn unit_escape(value: JsValue) -> Result<JsValue, JsValue> {
    Ok(text(&units::escape(&input(&value)?)))
}

#[wasm_bindgen(js_name = unitUnescape)]
pub fn unit_unescape(value: JsValue) -> Result<JsValue, JsValue> {
    units::unescape(&input(&value)?)
        .map(|value| text(&value))
        .map_err(error)
}

/// The replacement callback runs in the caller's realm for each shadow color.
#[wasm_bindgen(js_name = unitReplaceShadowColors)]
pub fn unit_replace_shadow_colors(
    value: JsValue,
    replacement: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let mut failure = None;
    let output = units::replace_shadow_colors(&input(&value)?, UNIT_LIMITS.css_bytes, |color| {
        if failure.is_some() {
            return Text::default();
        }
        match replacement
            .call1(&JsValue::UNDEFINED, &text(color))
            .and_then(|value| input(&value))
        {
            Ok(value) => value,
            Err(error) => {
                failure = Some(error);
                Text::default()
            }
        }
    });
    if let Some(failure) = failure {
        return Err(failure);
    }
    output.map(|value| text(&value)).map_err(error)
}

#[wasm_bindgen(js_name = unitCssParse)]
pub fn unit_css_parse(css: JsValue, from: Option<String>) -> Result<JsValue, JsValue> {
    let from = from.filter(|from| !from.is_empty());
    write_public_value(
        &units::parse_css(&input(&css)?, from, UNIT_LIMITS.ast_depth).map_err(error)?,
    )
}

#[wasm_bindgen(js_name = unitValueParse)]
pub fn unit_value_parse(value: JsValue) -> Result<JsValue, JsValue> {
    write_public_value(&units::parse_value(&input(&value)?, UNIT_LIMITS.ast_depth).map_err(error)?)
}

#[wasm_bindgen(js_name = unitValueToCss)]
pub fn unit_value_to_css(nodes: JsValue) -> Result<JsValue, JsValue> {
    let mut budget = Budget {
        bytes: 0,
        limit: UNIT_LIMITS.cache_bytes,
        resource: "cache_bytes",
    };
    let nodes = read_public_value(&nodes, 0, &UNIT_LIMITS, &mut budget)?;
    units::value_to_css(&nodes, UNIT_LIMITS.ast_depth)
        .map(|value| text(&value))
        .map_err(error)
}

/// UTF-16 offset to one-based line and zero-based column, and back.
#[wasm_bindgen]
pub struct UnitLineTable {
    table: core::LineTable,
}
#[wasm_bindgen]
impl UnitLineTable {
    #[wasm_bindgen(constructor)]
    pub fn new(source: JsValue) -> Result<UnitLineTable, JsValue> {
        Ok(Self {
            table: core::LineTable::new(&input(&source)?),
        })
    }
    pub fn find(&self, offset: usize) -> Result<JsValue, JsValue> {
        position(self.table.find(offset))
    }
    #[wasm_bindgen(js_name = findOffset)]
    pub fn find_offset(&self, line: usize, column: usize) -> usize {
        self.table.find_offset(core::Position { line, column })
    }
}

fn read_nodes(ast: &JsValue) -> Result<Vec<rustify_style::compiler::ast::AstNode>, JsValue> {
    let mut budget = Budget {
        bytes: 0,
        limit: UNIT_LIMITS.cache_bytes,
        resource: "cache_bytes",
    };
    let nodes = read_public_value(ast, 0, &UNIT_LIMITS, &mut budget)?;
    units::typed_nodes(&nodes, UNIT_LIMITS.ast_depth).map_err(error)
}

/// With `track`, each printed caller node receives `dst: [source, start, end]`
/// in the same order the compiler prints, sharing one generated Source.
#[wasm_bindgen(js_name = unitToCss)]
pub fn unit_to_css(ast: JsValue, track: Option<bool>) -> Result<JsValue, JsValue> {
    let nodes = read_nodes(&ast)?;
    let (css, spans) = units::to_css(&nodes);
    let css = text(&css);
    if track.unwrap_or(false) {
        let source: JsValue = Object::new().into();
        set(&source, "file", &JsValue::NULL)?;
        set(&source, "code", &css)?;
        let mut spans = spans.into_iter();
        fn visit(
            nodes: &JsValue,
            source: &JsValue,
            spans: &mut impl Iterator<Item = (usize, usize)>,
        ) -> Result<(), JsValue> {
            for index in 0..array_length(nodes)? {
                let node = Reflect::get(nodes, &JsValue::from_f64(index as f64))?;
                let kind = Reflect::get(&node, &"kind".into())?.as_string();
                if matches!(kind.as_deref(), Some("context" | "at-root")) {
                    continue;
                }
                let (start, end) = spans
                    .next()
                    .ok_or_else(|| protocol("printed span count differs from the AST"))?;
                let dst = Array::of3(
                    source,
                    &JsValue::from_f64(start as f64),
                    &JsValue::from_f64(end as f64),
                );
                set(&node, "dst", &dst)?;
                let children = Reflect::get(&node, &"nodes".into())?;
                if is_array(&children)? && array_length(&children)? > 0 {
                    visit(&children, source, spans)?;
                }
            }
            Ok(())
        }
        visit(&ast, &source, &mut spans)?;
    }
    Ok(css)
}

#[wasm_bindgen(js_name = unitOptimizeAst)]
pub fn unit_optimize_ast(ast: JsValue, polyfills: Option<u32>) -> Result<JsValue, JsValue> {
    let nodes = read_nodes(&ast)?;
    let polyfills = core::Polyfills(polyfills.unwrap_or(core::Polyfills::ALL.0));
    let optimized = units::optimize_ast(nodes, polyfills, UNIT_LIMITS.ast_depth).map_err(error)?;
    write_public_value(&units::public_nodes(&optimized))
}

type Translate = Box<
    dyn Fn(Option<&core::SourceLocation>, Option<&core::SourceLocation>) -> Vec<core::Translation>,
>;

/// Upstream `createTranslationMap`; the caller passes a node's `src`/`dst` offsets.
#[wasm_bindgen]
pub struct UnitTranslationMap {
    translate: Translate,
}
#[wasm_bindgen]
impl UnitTranslationMap {
    #[wasm_bindgen(constructor)]
    pub fn new(original: JsValue, generated: JsValue) -> Result<UnitTranslationMap, JsValue> {
        Ok(Self {
            translate: Box::new(core::create_translation_map(
                &input(&original)?,
                &input(&generated)?,
            )),
        })
    }
    pub fn translate(
        &self,
        src: Option<Vec<usize>>,
        dst: Option<Vec<usize>>,
    ) -> Result<JsValue, JsValue> {
        let location = |start, end| core::SourceLocation {
            source: Arc::new(core::Source {
                file: None,
                code: Text::default(),
            }),
            start,
            end,
            public: None,
        };
        let span = |offsets: Option<Vec<usize>>| {
            offsets.and_then(|offsets| match offsets[..] {
                [start, end] => Some(location(start, end)),
                _ => None,
            })
        };
        let (src, dst) = (span(src), span(dst));
        let output = Array::new();
        for (original_start, original_end, generated_start, generated_end) in
            (self.translate)(src.as_ref(), dst.as_ref())
        {
            let generated =
                |value: Option<core::Position>| value.map_or(Ok(JsValue::NULL), position);
            output.push(&Array::of4(
                &position(original_start)?,
                &position(original_end)?,
                &generated(generated_start)?,
                &generated(generated_end)?,
            ));
        }
        Ok(output.into())
    }
}

fn read_value(value: &JsValue) -> Result<PublicValue, JsValue> {
    let mut budget = Budget {
        bytes: 0,
        limit: UNIT_LIMITS.cache_bytes,
        resource: "cache_bytes",
    };
    read_public_value(value, 0, &UNIT_LIMITS, &mut budget)
}

#[wasm_bindgen(js_name = unitParseCandidate)]
pub fn unit_parse_candidate(raw: JsValue, registry: JsValue) -> Result<JsValue, JsValue> {
    let registry = units::test_registry(&read_value(&registry)?).map_err(error)?;
    write_public_value(
        &units::parse_candidate(&input(&raw)?, &registry, UNIT_LIMITS.ast_depth).map_err(error)?,
    )
}

#[wasm_bindgen(js_name = unitParseVariant)]
pub fn unit_parse_variant(raw: JsValue, registry: JsValue) -> Result<JsValue, JsValue> {
    let registry = units::test_registry(&read_value(&registry)?).map_err(error)?;
    write_public_value(
        &units::parse_variant(&input(&raw)?, &registry, UNIT_LIMITS.ast_depth).map_err(error)?,
    )
}

#[wasm_bindgen(js_name = unitCompoundsForSelectors)]
pub fn unit_compounds_for_selectors(selectors: JsValue) -> Result<u8, JsValue> {
    let selectors = (0..array_length(&selectors)?)
        .map(|index| input(&Reflect::get(&selectors, &JsValue::from_f64(index as f64))?))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(units::compounds_for_selectors(&selectors))
}
