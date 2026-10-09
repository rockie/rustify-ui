use super::*;
use js_sys::{Map, Set};
use std::collections::BTreeMap;

type Key = (u8, usize);
#[derive(Clone)]
struct Entry {
    value: PublicValue,
    javascript: JsValue,
}
#[derive(Default)]
pub(super) struct Graph {
    entries: BTreeMap<Key, Entry>,
}
fn key(value: &PublicValue) -> Option<Key> {
    match value {
        PublicValue::Array(values) => Some((0, Arc::as_ptr(values) as usize)),
        PublicValue::Object(values) => Some((1, Arc::as_ptr(values.identity()) as usize)),
        _ => None,
    }
}
fn same_storage(a: &PublicValue, b: &PublicValue) -> bool {
    match (a, b) {
        (PublicValue::Array(a), PublicValue::Array(b)) => Arc::ptr_eq(a, b),
        (PublicValue::Object(a), PublicValue::Object(b)) => Arc::ptr_eq(a, b),
        _ => false,
    }
}
pub(super) fn read(
    input: &JsValue,
    limits: &Limits,
    budget: &mut Budget,
) -> Result<(PublicValue, Graph), JsValue> {
    struct Reader<'a> {
        aliases: Map,
        values: Vec<PublicValue>,
        graph: Graph,
        limits: &'a Limits,
        budget: &'a mut Budget,
    }
    impl Reader<'_> {
        fn value(&mut self, input: &JsValue, depth: usize) -> Result<PublicValue, JsValue> {
            if depth > self.limits.ast_depth.saturating_mul(2).saturating_add(4) {
                return Err(limit("ast_depth", self.limits.ast_depth));
            }
            if !input.is_object() || input.is_function() {
                return read_public_value(input, depth, self.limits, self.budget);
            }
            self.budget.charge(std::mem::size_of::<PublicValue>())?;
            if let Some(index) = self.aliases.get(input).as_f64() {
                return Ok(self.values[index as usize].clone());
            }
            // The temporary alias table and the retained graph entry are reserved
            // before publishing either handle. Cycles hit the normal depth limit.
            self.budget.charge(192)?;
            let value = if is_array(input)? {
                let count = array_length(input)?;
                let mut values = Vec::new();
                for index in 0..count {
                    values.push(self.value(
                        &Reflect::get(input, &JsValue::from_f64(index as f64))?,
                        depth + 1,
                    )?);
                }
                PublicValue::Array(Arc::new(values))
            } else {
                let mut fields = core::PublicObject::new();
                for name in own_keys(input)?.iter() {
                    let field = self.budget.text(&name)?;
                    let value = self.value(&Reflect::get(input, &name)?, depth + 1)?;
                    fields.insert(field, value);
                }
                PublicValue::Object(Arc::new(fields))
            };
            self.aliases
                .set(input, &JsValue::from_f64(self.values.len() as f64));
            self.values.push(value.clone());
            self.graph.entries.insert(
                key(&value).ok_or_else(|| protocol("invalid graph node"))?,
                Entry {
                    value: value.clone(),
                    javascript: input.clone(),
                },
            );
            Ok(value)
        }
    }
    let mut reader = Reader {
        aliases: Map::new(),
        values: vec![],
        graph: Graph::default(),
        limits,
        budget,
    };
    let value = reader.value(input, 0)?;
    Ok((value, reader.graph))
}
fn shallow_javascript(value: &PublicValue) -> usize {
    fn child(value: &PublicValue) -> usize {
        match value {
            PublicValue::String(value) => {
                64usize.saturating_add(value.units().len().saturating_mul(2))
            }
            PublicValue::Array(_) | PublicValue::Object(_) => 8,
            _ => 64,
        }
    }
    64usize.saturating_add(match value {
        PublicValue::Array(values) => values.iter().map(child).sum(),
        PublicValue::Object(values) => values
            .iter()
            .map(|(key, value)| {
                key.units()
                    .len()
                    .saturating_mul(2)
                    .saturating_add(child(value))
            })
            .sum(),
        _ => 0,
    })
}
impl Graph {
    pub(super) fn retain_created(&mut self, created: Self) {
        for (key, entry) in created.entries {
            self.entries.entry(key).or_insert(entry);
        }
    }
    pub(super) fn retained_bytes(&self) -> usize {
        self.entries
            .values()
            .map(|entry| 96usize.saturating_add(shallow_javascript(&entry.value)))
            .sum()
    }
    pub(super) fn output_bytes(nodes: &[PublicValue], mutations: &[core::InputMutation]) -> usize {
        fn visit(value: &PublicValue, seen: &mut BTreeMap<Key, ()>) -> usize {
            let Some(key) = key(value) else {
                return 0;
            };
            if seen.insert(key, ()).is_some() {
                return 0;
            }
            let children: usize = match value {
                PublicValue::Array(values) => values.iter().map(|value| visit(value, seen)).sum(),
                PublicValue::Object(values) => {
                    values.values().map(|value| visit(value, seen)).sum()
                }
                _ => 0,
            };
            96usize
                .saturating_add(shallow_javascript(value))
                .saturating_add(children)
        }
        let mut seen = BTreeMap::new();
        64usize
            .saturating_add(nodes.len().saturating_mul(8))
            .saturating_add(
                nodes
                    .iter()
                    .map(|value| visit(value, &mut seen))
                    .sum::<usize>(),
            )
            .saturating_add(
                mutations
                    .iter()
                    .map(|mutation| {
                        visit(mutation.target(), &mut seen)
                            .saturating_add(visit(mutation.value(), &mut seen))
                    })
                    .sum::<usize>(),
            )
    }
    pub(super) fn apply(&mut self, effect: &core::InputMutation) -> Result<(), JsValue> {
        let mut created = Graph::default();
        let result = self.write_created(
            &[],
            std::slice::from_ref(effect),
            &Graph::default(),
            &mut created,
            false,
        );
        // A setter can capture a newly created context before throwing. Keep its
        // handle so an already launched task can still complete that object.
        self.retain_created(created);
        result.map(|_| ())
    }
    pub(super) fn write(
        &self,
        nodes: &[PublicValue],
        mutations: &[core::InputMutation],
        previous: &Graph,
    ) -> Result<(JsValue, Graph), JsValue> {
        let mut created = Graph::default();
        let output = self.write_created(nodes, mutations, previous, &mut created, true)?;
        Ok((output, created))
    }
    fn write_created(
        &self,
        nodes: &[PublicValue],
        mutations: &[core::InputMutation],
        previous: &Graph,
        next: &mut Graph,
        publish: bool,
    ) -> Result<JsValue, JsValue> {
        fn value(
            value: &PublicValue,
            input: &Graph,
            previous: &Graph,
            next: &mut Graph,
        ) -> Result<JsValue, JsValue> {
            let Some(id) = key(value) else {
                return write_public_value(value);
            };
            if let Some(entry) = next
                .entries
                .get(&id)
                .filter(|entry| same_storage(value, &entry.value))
            {
                return Ok(entry.javascript.clone());
            }
            // Unchanged caller storage names the live host value. Earlier
            // ordered mutations may already have changed its array contents.
            if let Some(entry) = input
                .entries
                .get(&id)
                .filter(|entry| same_storage(value, &entry.value))
            {
                next.entries.entry(id).or_insert_with(|| entry.clone());
                return Ok(entry.javascript.clone());
            }
            let cached = next
                .entries
                .get(&id)
                .or_else(|| input.entries.get(&id))
                .or_else(|| previous.entries.get(&id))
                .map(|entry| entry.javascript.clone());
            let output = match value {
                PublicValue::Array(values) => {
                    let output: JsValue = cached.unwrap_or_else(|| Array::new().into());
                    let length = array_length(&output)?;
                    for (index, child) in values.iter().enumerate() {
                        let field = JsValue::from_f64(index as f64);
                        let child = value_inner(child, input, previous, next)?;
                        if Reflect::get(&output, &field)? != child {
                            set_key(&output, &field, &child)?;
                        }
                    }
                    if length != values.len()
                        && !Reflect::set(
                            &output,
                            &"length".into(),
                            &JsValue::from_f64(values.len() as f64),
                        )?
                    {
                        return Err(protocol("AST array length is not writable"));
                    }
                    output
                }
                PublicValue::Object(values)
                    if values.len() == 1
                        && (values.contains_key(&Text::from("undefined"))
                            || values.contains_key(&Text::from("bigint"))) =>
                {
                    return write_public_value(value);
                }
                PublicValue::Object(values) => {
                    let output: JsValue = cached.unwrap_or_else(|| Object::new().into());
                    let fields = Set::new(&own_keys(&output)?.into());
                    for (field, child) in values.iter() {
                        let field = text(field);
                        let child = value_inner(child, input, previous, next)?;
                        if Reflect::get(&output, &field)? != child || !fields.has(&field) {
                            set_key(&output, &field, &child)?;
                        }
                    }
                    output
                }
                _ => return write_public_value(value),
            };
            next.entries.insert(
                id,
                Entry {
                    value: value.clone(),
                    javascript: output.clone(),
                },
            );
            Ok(output)
        }
        // A separate name keeps the recursive helper's argument from shadowing it.
        fn value_inner(
            node: &PublicValue,
            input: &Graph,
            previous: &Graph,
            next: &mut Graph,
        ) -> Result<JsValue, JsValue> {
            value(node, input, previous, next)
        }
        for mutation in mutations {
            let target = key(mutation.target())
                .and_then(|id| {
                    self.entries
                        .get(&id)
                        .or_else(|| next.entries.get(&id))
                        .or_else(|| previous.entries.get(&id))
                })
                .map(|entry| entry.javascript.clone());
            // Nested imports are parsed before their enclosing context is
            // published. Their arrays still need stable handles for the journal.
            let target = match target {
                Some(target) => target,
                None => value(mutation.target(), self, previous, next)?,
            };
            let child = value(mutation.value(), self, previous, next)?;
            match mutation {
                core::InputMutation::Set { field, .. } => {
                    if !Reflect::set(&target, &text(field), &child)? {
                        return Err(
                            js_sys::TypeError::new("AST input field is not writable").into()
                        );
                    }
                }
                core::InputMutation::Splice {
                    start,
                    delete_count,
                    ..
                } => {
                    let splice = Reflect::get(&target, &JsValue::from_str("splice"))?
                        .dyn_into::<js_sys::Function>()?;
                    let args = Array::of2(
                        &JsValue::from_f64(*start as f64),
                        &JsValue::from_f64(*delete_count as f64),
                    );
                    for item in Array::from(&child).iter() {
                        args.push(&item);
                    }
                    splice.apply(&target, &args)?;
                }
            }
        }
        if !publish {
            return Ok(JsValue::UNDEFINED);
        }
        let output = Array::new();
        for node in nodes {
            output.push(&value(node, self, previous, next)?);
        }
        Ok(output.into())
    }
}
