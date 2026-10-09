use super::*;
use std::{
    cell::{RefCell, RefMut},
    collections::BTreeMap,
    rc::Rc,
};

struct State {
    compiler: core::DesignSystem,
    limits: Limits,
    candidates: BTreeMap<Text, (usize, JsValue)>,
    variants: BTreeMap<Text, (usize, JsValue)>,
    sources: BTreeMap<usize, (Arc<core::Source>, JsValue)>,
    js_bytes: usize,
    cache_sequence: usize,
}
impl State {
    fn text(&self, input: &JsValue) -> Result<Text, JsValue> {
        if input.is_string() {
            // Account for the native Vec's conservative growth before copying units.
            self.check_bytes(
                (input.unchecked_ref::<JsString>().length() as usize).saturating_mul(8),
            )?;
        }
        Budget {
            bytes: 0,
            limit: self.limits.input_bytes,
            resource: "input_bytes",
        }
        .text(input)
    }
    fn value(&self, input: &JsValue) -> Result<core::PublicValue, JsValue> {
        read_public_value(
            input,
            0,
            &self.limits,
            &mut Budget {
                bytes: self.compiler.retained_bytes().saturating_add(self.js_bytes),
                limit: self.limits.cache_bytes,
                resource: "cache_bytes",
            },
        )
    }
    fn output(&self, value: &core::PublicValue) -> Result<JsValue, JsValue> {
        self.output_with_reserved(value, 0)
    }
    fn output_with_reserved(
        &self,
        value: &core::PublicValue,
        reserved: usize,
    ) -> Result<JsValue, JsValue> {
        self.check_bytes(
            reserved
                .saturating_add(value.owned_bytes())
                .saturating_add(value_bytes(value))
                .saturating_add(crossing_scratch(value)),
        )?;
        write_public_value(value)
    }
    fn transaction<T>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, JsValue>,
    ) -> Result<T, JsValue> {
        let checkpoint = self.compiler.cache_checkpoint();
        let sequence = self.cache_sequence;
        let js_bytes = self.js_bytes;
        let result = operation(self);
        if result.is_err() {
            self.compiler.rollback_cache(checkpoint);
            self.candidates.retain(|_, (entry, _)| *entry < sequence);
            self.variants.retain(|_, (entry, _)| *entry < sequence);
            self.cache_sequence = sequence;
            self.js_bytes = js_bytes;
        }
        result
    }
    fn check_bytes(&self, extra: usize) -> Result<(), JsValue> {
        if self
            .compiler
            .retained_bytes()
            .saturating_add(self.js_bytes)
            .saturating_add(extra)
            > self.limits.cache_bytes
        {
            return Err(limit("cache_bytes", self.limits.cache_bytes));
        }
        Ok(())
    }
    fn variant(&mut self, raw: &Text, reserved: usize) -> Result<JsValue, JsValue> {
        if let Some((_, value)) = self.variants.get(raw) {
            return Ok(value.clone());
        }
        let value = self
            .compiler
            .parse_variant_text_with_reserved_cache(
                raw,
                self.js_bytes
                    .saturating_add(raw.owned_bytes())
                    .saturating_add(reserved),
            )
            .map_err(error)?;
        let child = self
            .compiler
            .with_reserved_cache(
                self.js_bytes
                    .saturating_add(reserved)
                    .saturating_add(value.owned_bytes()),
                |compiler| compiler.variant_child_input_text(raw),
            )
            .map_err(error)?;
        let child = child
            .as_ref()
            .map(|child| self.variant(child, reserved.saturating_add(value.owned_bytes())))
            .transpose()?;
        let bytes = value_bytes(&value)
            .saturating_add(raw.owned_bytes())
            .saturating_add(128);
        self.check_bytes(
            bytes
                .saturating_add(reserved)
                .saturating_add(value.owned_bytes())
                .saturating_add(value_bytes(&value))
                .saturating_add(256),
        )?;
        let output = write_public_value(&value)?;
        if let Some(child) = child.filter(|_| output.is_object()) {
            set(&output, "variant", &child)?;
        }
        self.js_bytes = self.js_bytes.saturating_add(bytes);
        let sequence = self.cache_sequence;
        self.cache_sequence = self.cache_sequence.saturating_add(1);
        self.variants
            .insert(raw.clone(), (sequence, output.clone()));
        Ok(output)
    }
}
// Conservative logical accounting for retained JS values, independent of a JS
// engine's allocator. Caller-owned returned values are not retained by the facade.
fn value_bytes(value: &core::PublicValue) -> usize {
    64usize.saturating_add(match value {
        core::PublicValue::String(value) => value.units().len().saturating_mul(2),
        core::PublicValue::Array(values) => values.iter().fold(0usize, |total, value| {
            total.saturating_add(value_bytes(value))
        }),
        core::PublicValue::Object(values) => values.iter().fold(0usize, |total, (key, value)| {
            total
                .saturating_add(key.units().len().saturating_mul(2))
                .saturating_add(value_bytes(value))
        }),
        _ => 0,
    })
}

// write_public_value constructs one graph. Only the deepest active path and
// text()'s current concat prefix/chunk coexist with that output, not a second graph.
fn crossing_scratch(value: &core::PublicValue) -> usize {
    fn text_scratch(value: &Text) -> usize {
        let units = value.units().len();
        units
            .saturating_mul(2)
            .saturating_add(units.min(8192).saturating_mul(4))
    }
    256usize.saturating_add(match value {
        core::PublicValue::String(value) => text_scratch(value),
        core::PublicValue::Array(values) => values.iter().map(crossing_scratch).max().unwrap_or(0),
        core::PublicValue::Object(values) => values
            .iter()
            .map(|(key, value)| {
                text_scratch(key).max(
                    crossing_scratch(value).saturating_add(key.units().len().saturating_mul(2)),
                )
            })
            .max()
            .unwrap_or(0),
        _ => 0,
    })
}

// Consuming rows releases their native payloads, while IntoIter keeps the outer
// Vec allocation until the final row. Check every transition before creating JS.
fn array_crossing_peak(values: &Vec<core::PublicValue>) -> usize {
    let mut remaining = values
        .capacity()
        .saturating_mul(std::mem::size_of::<core::PublicValue>())
        .saturating_add(
            values
                .iter()
                .map(core::PublicValue::owned_bytes)
                .sum::<usize>(),
        );
    let mut written = 64usize;
    let mut peak = remaining.saturating_add(written);
    for value in values {
        let output = value_bytes(value);
        peak = peak.max(
            remaining
                .saturating_add(written)
                .saturating_add(output)
                .saturating_add(crossing_scratch(value)),
        );
        remaining = remaining.saturating_sub(value.owned_bytes());
        written = written.saturating_add(output);
    }
    peak
}

fn borrow(state: &Rc<RefCell<State>>) -> Result<RefMut<'_, State>, JsValue> {
    state
        .try_borrow_mut()
        .map_err(|_| protocol("reentrant design-system call"))
}

#[wasm_bindgen]
pub struct DesignSystem {
    state: Rc<RefCell<State>>,
}

#[wasm_bindgen]
impl DesignSystem {
    #[wasm_bindgen(constructor)]
    pub fn new(input: JsValue, config: &str) -> Result<DesignSystem, JsValue> {
        let config = configuration(config)?;
        let input = Budget {
            bytes: 0,
            limit: config.limits.input_bytes,
            resource: "input_bytes",
        }
        .text(&input)?;
        let compiler = core::load_design_system_with_loader(
            &input,
            config.options,
            config.assets,
            config.limits.clone(),
            config.loader,
        )
        .map_err(error)?;
        Ok(Self {
            state: Rc::new(RefCell::new(State {
                compiler,
                limits: config.limits,
                candidates: BTreeMap::new(),
                variants: BTreeMap::new(),
                sources: BTreeMap::new(),
                js_bytes: 0,
                cache_sequence: 0,
            })),
        })
    }
    #[wasm_bindgen(js_name = parseCandidate)]
    pub fn parse_candidate(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let raw = state.text(&input)?;
        if let Some((_, value)) = state.candidates.get(&raw) {
            return Ok(value.clone());
        }
        state.transaction(|state| {
            let parsed = core::PublicValue::Array(Arc::new(
                state
                    .compiler
                    .parse_candidate_text_with_reserved_cache(
                        &raw,
                        state.js_bytes.saturating_add(raw.owned_bytes()),
                    )
                    .map_err(error)?,
            ));
            let variant_inputs = if parsed.as_array().is_some_and(|values| !values.is_empty()) {
                state
                    .compiler
                    .with_reserved_cache(
                        state
                            .js_bytes
                            .saturating_add(parsed.owned_bytes())
                            .saturating_add(raw.owned_bytes()),
                        |compiler| compiler.candidate_variant_inputs_text(&raw),
                    )
                    .map_err(error)?
            } else {
                Vec::new()
            };
            let reserved = parsed
                .owned_bytes()
                .saturating_add(raw.owned_bytes())
                .saturating_add(
                    variant_inputs
                        .capacity()
                        .saturating_mul(std::mem::size_of::<Text>()),
                )
                .saturating_add(variant_inputs.iter().map(Text::owned_bytes).sum::<usize>());
            state.check_bytes(
                reserved.saturating_add(
                    variant_inputs
                        .len()
                        .saturating_mul(std::mem::size_of::<JsValue>()),
                ),
            )?;
            let mut variants = Vec::with_capacity(variant_inputs.len());
            for raw in variant_inputs {
                variants.push(
                    state.variant(
                        &raw,
                        reserved.saturating_add(
                            variants
                                .capacity()
                                .saturating_mul(std::mem::size_of::<JsValue>()),
                        ),
                    )?,
                );
            }
            let bytes = value_bytes(&parsed)
                .saturating_add(raw.owned_bytes())
                .saturating_add(128);
            state.check_bytes(
                bytes
                    .saturating_add(raw.owned_bytes())
                    .saturating_add(parsed.owned_bytes())
                    .saturating_add(value_bytes(&parsed))
                    .saturating_add(
                        variants
                            .capacity()
                            .saturating_mul(std::mem::size_of::<JsValue>()),
                    )
                    .saturating_add(256),
            )?;
            let output = write_public_value(&parsed)?;
            let array = Array::new();
            for variant in &variants {
                array.push(variant);
            }
            let array: JsValue = array.into();
            for node in output.unchecked_ref::<Array>().iter() {
                set(&node, "variants", &array)?;
            }
            state.js_bytes = state.js_bytes.saturating_add(bytes);
            let sequence = state.cache_sequence;
            state.cache_sequence = state.cache_sequence.saturating_add(1);
            state
                .candidates
                .insert(raw.clone(), (sequence, output.clone()));
            Ok(output)
        })
    }
    #[wasm_bindgen(js_name = parseVariant)]
    pub fn parse_variant(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let raw = state.text(&input)?;
        state.transaction(|state| state.variant(&raw, raw.owned_bytes()))
    }
    #[wasm_bindgen(js_name = printCandidate)]
    pub fn print_candidate(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let value = state.value(&input)?;
        let reserved = state.js_bytes.saturating_add(value.owned_bytes());
        let output = state
            .compiler
            .with_reserved_cache(reserved, |compiler| compiler.print_candidate_text(&value))
            .map_err(error)?;
        state.check_bytes(
            value
                .owned_bytes()
                .saturating_add(output.owned_bytes())
                .saturating_add(output.len().saturating_mul(4)),
        )?;
        Ok(text(&output))
    }
    #[wasm_bindgen(js_name = printVariant)]
    pub fn print_variant(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let value = state.value(&input)?;
        let reserved = state.js_bytes.saturating_add(value.owned_bytes());
        let output = state
            .compiler
            .with_reserved_cache(reserved, |compiler| compiler.print_variant_text(&value))
            .map_err(error)?;
        state.check_bytes(
            value
                .owned_bytes()
                .saturating_add(output.owned_bytes())
                .saturating_add(output.len().saturating_mul(4)),
        )?;
        Ok(text(&output))
    }
    #[wasm_bindgen(js_name = compileAstNodes)]
    pub fn compile_ast_nodes(
        &self,
        input: JsValue,
        flags: Option<u32>,
    ) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let value = state.value(&input)?;
        state.transaction(|state| {
            let result = state
                .compiler
                .with_reserved_cache(
                    state.js_bytes.saturating_add(value.owned_bytes()),
                    |compiler| compiler.compile_ast_nodes_text(&value, flags.unwrap_or(1)),
                )
                .map_err(error)?;
            state.output_with_reserved(
                &core::PublicValue::Array(Arc::new(result)),
                value.owned_bytes(),
            )
        })
    }
    #[wasm_bindgen(js_name = getClassOrder)]
    pub fn get_class_order(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let classes = candidates_text(&input, &state.limits)?;
        state.transaction(|state| {
            let reserved = state
                .js_bytes
                .saturating_add(
                    classes
                        .capacity()
                        .saturating_mul(std::mem::size_of::<Text>()),
                )
                .saturating_add(classes.iter().map(Text::owned_bytes).sum::<usize>());
            let value = state
                .compiler
                .with_reserved_cache(reserved, |compiler| compiler.get_class_order_text(&classes))
                .map_err(error)?;
            state.output_with_reserved(
                &value,
                classes
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Text>())
                    .saturating_add(classes.iter().map(Text::owned_bytes).sum::<usize>()),
            )
        })
    }
    #[wasm_bindgen(js_name = candidatesToCss)]
    pub fn candidates_to_css(&self, input: JsValue) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let classes = candidates_text(&input, &state.limits)?;
        state.transaction(|state| {
            let reserved = state
                .js_bytes
                .saturating_add(
                    classes
                        .capacity()
                        .saturating_mul(std::mem::size_of::<Text>()),
                )
                .saturating_add(classes.iter().map(Text::owned_bytes).sum::<usize>());
            let value = state
                .compiler
                .with_reserved_cache(reserved, |compiler| {
                    compiler.candidates_to_css_text(&classes)
                })
                .map_err(error)?;
            state.output_with_reserved(
                &value,
                classes
                    .capacity()
                    .saturating_mul(std::mem::size_of::<Text>())
                    .saturating_add(classes.iter().map(Text::owned_bytes).sum::<usize>()),
            )
        })
    }
    #[wasm_bindgen(js_name = getClassList)]
    pub fn get_class_list(&self) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let reserved = state.js_bytes;
        let value = state
            .compiler
            .with_reserved_cache(reserved, |compiler| compiler.get_class_list_text())
            .map_err(error)?;
        let core::PublicValue::Array(values) = value else {
            return Err(protocol("class list is not an array"));
        };
        let values = Arc::unwrap_or_clone(values);
        state.check_bytes(array_crossing_peak(&values))?;
        let output = Array::new();
        for row in values {
            output.push(&write_public_value(&row)?);
            drop(row);
        }
        Ok(output.into())
    }
    #[wasm_bindgen(js_name = getVariants)]
    pub fn get_variants(&self) -> Result<JsValue, JsValue> {
        let values = {
            let mut state = borrow(&self.state)?;
            let reserved = state.js_bytes;
            state
                .compiler
                .with_reserved_cache(reserved, |compiler| compiler.get_variants_text())
                .map_err(error)?
        };
        let storage = values
            .capacity()
            .saturating_mul(std::mem::size_of::<core::PublicValue>())
            .saturating_add(
                values
                    .iter()
                    .map(core::PublicValue::owned_bytes)
                    .sum::<usize>(),
            );
        let crossing = values.iter().fold(64usize, |bytes, value| {
            bytes.saturating_add(value_bytes(value)).saturating_add(256)
        });
        borrow(&self.state)?.check_bytes(storage.saturating_add(crossing.saturating_mul(2)))?;
        let output = Array::new();
        for metadata in values {
            let root = metadata
                .get("name")
                .and_then(core::PublicValue::as_text)
                .ok_or_else(|| protocol("variant name absent"))?
                .clone();
            let value = borrow(&self.state)?.output(&metadata)?;
            let state = self.state.clone();
            // JS owns this callback. Its Rc keeps the design system alive until
            // the callback is collected, even after the class wrapper is freed.
            let callback = Closure::<dyn FnMut(JsValue) -> Result<JsValue, JsValue>>::new(
                move |options: JsValue| {
                    let mut state = borrow(&state)?;
                    let options = if options.is_undefined() {
                        core::PublicValue::Object(Arc::default())
                    } else {
                        state.value(&options)?
                    };
                    state.transaction(|state| {
                        let result = state
                            .compiler
                            .with_reserved_cache(
                                state
                                    .js_bytes
                                    .saturating_add(options.owned_bytes())
                                    .saturating_add(root.owned_bytes()),
                                |compiler| compiler.variant_selectors_text(&root, &options),
                            )
                            .map_err(error)?;
                        state.output_with_reserved(
                            &result,
                            options.owned_bytes().saturating_add(root.owned_bytes()),
                        )
                    })
                },
            );
            set(&value, "selectors", &callback.into_js_value())?;
            output.push(&value);
        }
        Ok(output.into())
    }
    #[wasm_bindgen(js_name = getVariantOrder)]
    pub fn get_variant_order(&self) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        state.transaction(|state| {
            let entries = state
                .compiler
                .with_reserved_cache(state.js_bytes, |compiler| {
                    compiler.variant_order_entries_text()
                })
                .map_err(error)?;
            let storage = entries
                .capacity()
                .saturating_mul(std::mem::size_of::<(Text, core::PublicValue, usize)>())
                .saturating_add(entries.iter().fold(0usize, |bytes, (raw, value, _)| {
                    bytes
                        .saturating_add(raw.owned_bytes())
                        .saturating_add(value.owned_bytes())
                }));
            state.check_bytes(
                storage
                    .saturating_add(entries.len().saturating_mul(128))
                    .saturating_add(64),
            )?;
            let reserved = storage
                .saturating_add(entries.len().saturating_mul(128))
                .saturating_add(64);
            let output = js_sys::Map::new();
            for (raw, _value, order) in entries {
                output.set(
                    &state.variant(&raw, reserved)?,
                    &JsValue::from_f64(order as f64),
                );
            }
            Ok(output.into())
        })
    }
    #[wasm_bindgen(js_name = resolveThemeValue)]
    pub fn resolve_theme_value(
        &self,
        input: JsValue,
        force_inline: Option<bool>,
    ) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let raw = state.text(&input)?;
        let reserved = state.js_bytes.saturating_add(raw.owned_bytes());
        let value = state
            .compiler
            .with_reserved_cache(reserved, |compiler| {
                compiler.resolve_theme_value_text(&raw, force_inline.unwrap_or(true))
            })
            .map_err(error)?;
        state.output_with_reserved(&value, raw.owned_bytes())
    }
    #[wasm_bindgen(js_name = trackUsedVariables)]
    pub fn track_used_variables(&self, input: JsValue) -> Result<(), JsValue> {
        let mut state = borrow(&self.state)?;
        let raw = state.text(&input)?;
        let reserved = state.js_bytes.saturating_add(raw.owned_bytes());
        state
            .compiler
            .track_used_variables_text_with_reserved_cache(&raw, reserved)
            .map_err(error)
    }
    #[wasm_bindgen(js_name = themeEntries)]
    pub fn theme_entries(&self) -> Result<JsValue, JsValue> {
        let mut state = borrow(&self.state)?;
        let (count, storage) = state.compiler.theme_entries_storage_bytes();
        // Snapshot S, JS text <= S, concat scratch <= 2S; 606 per entry
        // covers pair/field containers (414) and an optional src tuple (192).
        let peak = storage
            .saturating_mul(4)
            .saturating_add(count.saturating_mul(606))
            .saturating_add(64);
        state.check_bytes(peak)?;
        let mut needed = BTreeMap::new();
        let mut retained = 0usize;
        let mut scratch = 0usize;
        for location in state.compiler.theme_entry_sources() {
            let identity = Arc::as_ptr(&location.source) as usize;
            if state.sources.contains_key(&identity) || needed.contains_key(&identity) {
                continue;
            }
            let units = location.source.code.units().len().saturating_add(
                location
                    .source
                    .file
                    .as_ref()
                    .map_or(0, |file| file.encode_utf16().count()),
            );
            // Source object/fields (256), final native cache slot (128), text.
            retained = retained.saturating_add(384usize.saturating_add(units.saturating_mul(2)));
            scratch = scratch.max(256usize.saturating_add(units.saturating_mul(4)));
            // Preflight borrowed plan and staged cache slots before either insert.
            state.check_bytes(
                peak.saturating_add(retained)
                    .saturating_add(scratch)
                    .saturating_add(needed.len().saturating_add(1).saturating_mul(256)),
            )?;
            needed.insert(identity, &location.source);
        }
        let entries = state.compiler.theme_entries().map_err(error)?;
        let mut sources = BTreeMap::new();
        for (&identity, &source) in &needed {
            let output: JsValue = Object::new().into();
            set(
                &output,
                "file",
                &source
                    .file
                    .as_ref()
                    .map_or(JsValue::NULL, |file| JsValue::from_str(file)),
            )?;
            set(&output, "code", &text(&source.code))?;
            sources.insert(identity, (source.clone(), output));
        }
        let output = Array::new();
        for (key, token) in entries {
            let value: JsValue = Object::new().into();
            set(&value, "value", &text(&token.value))?;
            set(
                &value,
                "options",
                &JsValue::from_f64(f64::from(token.options)),
            )?;
            let src = if let Some(location) = token.src {
                let identity = Arc::as_ptr(&location.source) as usize;
                let source = state
                    .sources
                    .get(&identity)
                    .or_else(|| sources.get(&identity))
                    .ok_or_else(|| protocol("theme Source absent from cache"))?;
                let tuple = Array::new();
                tuple.push(&source.1);
                tuple.push(&JsValue::from_f64(location.start as f64));
                tuple.push(&JsValue::from_f64(location.end as f64));
                tuple.into()
            } else {
                JsValue::UNDEFINED
            };
            set(&value, "src", &src)?;
            let entry = Array::new();
            entry.push(&text(&key));
            entry.push(&value);
            output.push(&entry);
        }
        drop(needed);
        // Publish only after all output fields succeed; Arc guards prevent reuse
        // of an identity while its JavaScript Source object is cached.
        state.sources.extend(sources);
        state.js_bytes = state.js_bytes.saturating_add(retained);
        Ok(output.into())
    }
    #[wasm_bindgen(getter, js_name = invalidCandidates)]
    pub fn invalid_candidates(&self) -> Result<JsValue, JsValue> {
        let state = borrow(&self.state)?;
        let values = state.output(&state.compiler.invalid_candidates_text())?;
        Ok(js_sys::Set::new(&values).into())
    }
    /// Accepts any iterable, as upstream code assigns a `Set`.
    #[wasm_bindgen(setter, js_name = invalidCandidates)]
    pub fn set_invalid_candidates(&self, value: JsValue) -> Result<(), JsValue> {
        let values = js_sys::Array::from(&value);
        let mut state = borrow(&self.state)?;
        let mut texts = Vec::with_capacity(values.length() as usize);
        for value in values.iter() {
            texts.push(state.text(&value)?);
        }
        state
            .compiler
            .set_invalid_candidates_text(texts)
            .map_err(error)
    }
    #[wasm_bindgen(getter)]
    pub fn important(&self) -> Result<bool, JsValue> {
        Ok(borrow(&self.state)?.compiler.important)
    }
    #[wasm_bindgen(setter)]
    pub fn set_important(&self, value: bool) -> Result<(), JsValue> {
        borrow(&self.state)?.compiler.important = value;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn javascript_storage_counts_original_utf16_units() {
        for units in [vec![0xd800], vec![0xfffd], vec![0xd83d, 0xde00]] {
            let string = core::PublicValue::String(Text::from_units(units.clone()));
            assert_eq!(value_bytes(&string), 64 + units.len() * 2);
            let object = core::PublicValue::Object(Arc::new(core::PublicObject::from([(
                Text::from_units(units.clone()),
                core::PublicValue::Bool(true),
            )])));
            assert_eq!(value_bytes(&object), 128 + units.len() * 2);
        }
    }

    #[test]
    fn crossing_scratch_tracks_active_path_and_current_text_chunk() {
        let string = core::PublicValue::String(Text::from_units(vec![0xd800]));
        let single = core::PublicValue::Array(Arc::new(vec![string.clone()]));
        let siblings = core::PublicValue::Array(Arc::new(vec![string; 7257]));
        assert_eq!(crossing_scratch(&single), crossing_scratch(&siblings));
        assert_eq!(crossing_scratch(&single), 512 + 6);
        let nested = core::PublicValue::Array(Arc::new(vec![single.clone()]));
        assert_eq!(crossing_scratch(&nested), crossing_scratch(&single) + 256);
        let chunked = core::PublicValue::String(Text::from_units(vec![0xd800; 8193]));
        assert_eq!(crossing_scratch(&chunked), 256 + 8193 * 2 + 8192 * 4);
    }

    #[test]
    fn class_list_progressive_peak_fits_original_public_quota() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../tests/utility/compiler/public-api-fixtures.json"
        ))
        .unwrap();
        for case in fixture["designSystemCases"].as_array().unwrap() {
            let limits = Limits {
                input_bytes: 65536,
                asset_bytes: 65536,
                candidate_count: 1024,
                ast_depth: 32,
                css_bytes: 262144,
                queued_bytes: 65536,
                cache_bytes: 8_000_000,
            };
            let api = core::load_design_system(
                case["input"].as_str().unwrap(),
                core::CompileOptions::default(),
                vec![],
                limits,
            )
            .unwrap();
            let value = api.get_class_list_text().unwrap();
            let native = value.owned_bytes();
            let javascript = value_bytes(&value);
            let core::PublicValue::Array(rows) = value else {
                panic!("class list is not an array");
            };
            let peak = array_crossing_peak(&rows);
            println!(
                "{}: rows={} native={} javascript={} progressive_peak={} retained={} public_value_size={}",
                case["id"], rows.len(), native, javascript, peak, api.retained_bytes(),
                std::mem::size_of::<core::PublicValue>()
            );
            assert_eq!(rows.len(), 7257);
            assert!(api.retained_bytes().saturating_add(peak) <= 4_194_304);
            assert!(api.retained_bytes().saturating_add(peak) > 32768);
            assert!(peak < native.saturating_add(javascript));
        }
    }
}
