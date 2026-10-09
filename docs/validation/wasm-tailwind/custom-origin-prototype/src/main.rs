use rustify_style::{
    compiler::public_api::{PublicObject, PublicValue as V},
    text::Text,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap, sync::Arc};

type Key = (u8, usize);
fn semantic(value: &V) -> Option<Key> {
    match value {
        V::Object(v) => Some((1, Arc::as_ptr(v.identity()) as usize)),
        V::Array(v) => Some((0, Arc::as_ptr(v) as usize)),
        _ => None,
    }
}
fn storage(value: &V) -> Option<Key> {
    match value {
        V::Object(v) => Some((1, Arc::as_ptr(v) as usize)),
        V::Array(v) => Some((0, Arc::as_ptr(v) as usize)),
        _ => None,
    }
}
fn object(fields: impl IntoIterator<Item = (&'static str, V)>) -> V {
    V::Object(Arc::new(
        fields
            .into_iter()
            .map(|(k, v)| (k.into(), v))
            .collect::<PublicObject>(),
    ))
}
fn array(values: Vec<V>) -> V {
    V::Array(Arc::new(values))
}
fn text(value: &str) -> V {
    V::String(value.into())
}
fn declaration(property: &str, value: &str) -> V {
    object([
        ("kind", text("declaration")),
        ("property", text(property)),
        ("value", text(value)),
        ("important", V::Bool(false)),
    ])
}
fn fixture() -> (V, V, V, V) {
    let source = || {
        object([
            ("file", V::String(Text::from_units(vec![115, 0xd800]))),
            ("code", V::String(Text::from_units(vec![99, 0xdc00]))),
            ("extra", text("source")),
        ])
    };
    let first = source();
    let tuple = array(vec![
        first.clone(),
        V::Number(serde_json::Number::from_f64(1.5).unwrap()),
        V::Number(9.into()),
    ]);
    let mut child = declaration("color", "--alpha(--value(--color) / 50%)");
    child["extra"] = object([("first", first.clone()), ("equalDistinct", source())]);
    child["src"] = tuple.clone();
    child["dst"] = tuple;
    let mut second = declaration("background", "red");
    second["extra"] = object([("child", child.clone())]);
    let body = array(vec![child.clone(), second]);
    let rule = object([
        ("kind", text("rule")),
        ("selector", text("& .nested")),
        ("nodes", body.clone()),
        (
            "extra",
            object([("body", body.clone()), ("child", child.clone())]),
        ),
    ]);
    let supports = object([
        ("kind", text("at-rule")),
        ("name", text("@supports")),
        ("params", text("(display: grid)")),
        ("nodes", array(vec![rule.clone()])),
        ("extra", first),
    ]);
    let comment = object([
        ("kind", text("comment")),
        ("value", text("comment")),
        ("extra", text("comment metadata")),
    ]);
    let context = object([
        ("kind", text("context")),
        ("context", object([("probe", V::Bool(true))])),
        ("nodes", array(vec![comment])),
        ("extra", text("context metadata")),
    ]);
    let root = object([
        ("kind", text("at-root")),
        ("nodes", array(vec![declaration("--probe", "1")])),
        ("extra", text("root metadata")),
    ]);
    let nodes = array(vec![supports, context, root]);
    let definition = object([
        ("kind", text("at-rule")),
        ("name", text("@utility")),
        ("params", text("tone-*")),
        ("nodes", nodes.clone()),
        (
            "extra",
            object([("wrapperOnly", text("wrapper")), ("body", nodes)]),
        ),
    ]);
    (definition, child, body, rule)
}
// Views select current COW storage before the deep-clone memo selects identity.
fn clone_view(
    value: &V,
    views: &BTreeMap<Key, V>,
    memo: &mut BTreeMap<Key, V>,
    allocations: &mut usize,
) -> V {
    let current = semantic(value)
        .and_then(|key| views.get(&key))
        .unwrap_or(value);
    let Some(key) = storage(current) else {
        return current.clone();
    };
    if let Some(value) = memo.get(&key) {
        return value.clone();
    }
    *allocations += 1;
    let cloned = match current {
        V::Array(values) => array(
            values
                .iter()
                .map(|v| clone_view(v, views, memo, allocations))
                .collect(),
        ),
        V::Object(values) => V::Object(Arc::new(
            values
                .iter()
                .map(|(k, v)| (k.clone(), clone_view(v, views, memo, allocations)))
                .collect::<PublicObject>(),
        )),
        _ => unreachable!(),
    };
    memo.insert(key, cloned.clone());
    cloned
}
// Conservative experiment reserve; not the production allocator/peak meter.
fn reserve(root: &V, views: &BTreeMap<Key, V>) -> usize {
    root.owned_bytes()
        .saturating_mul(4)
        .saturating_add(
            views
                .values()
                .map(V::owned_bytes)
                .sum::<usize>()
                .saturating_mul(4),
        )
        .saturating_add(views.len().saturating_mul(256))
}
fn checked_clone(
    root: &V,
    views: &BTreeMap<Key, V>,
    limit: usize,
    allocations: &mut usize,
) -> Result<V, ()> {
    if reserve(root, views) > limit {
        return Err(());
    }
    Ok(clone_view(root, views, &mut BTreeMap::new(), allocations))
}
fn snapshot(roots: &[V]) -> Value {
    fn visit(value: &V, ids: &mut BTreeMap<Key, usize>, records: &mut Vec<Value>) -> Value {
        let Some(key) = semantic(value) else {
            return match value {
                V::String(v) => json!({"units":v.units()}),
                _ => serde_json::to_value(value).unwrap(),
            };
        };
        if let Some(id) = ids.get(&key) {
            return json!({"reference":id});
        }
        let id = records.len();
        ids.insert(key, id);
        records.push(Value::Null);
        let (is_array, fields) = match value {
            V::Array(v) => (
                true,
                v.iter()
                    .enumerate()
                    .map(|(i, v)| json!([i.to_string(), visit(v, ids, records)]))
                    .collect::<Vec<_>>(),
            ),
            V::Object(v) => (
                false,
                v.iter()
                    .map(|(k, v)| {
                        json!([
                            String::from_utf16(k.units()).unwrap(),
                            visit(v, ids, records)
                        ])
                    })
                    .collect(),
            ),
            _ => unreachable!(),
        };
        records[id] = json!({"array":is_array,"fields":fields});
        json!({"reference":id})
    }
    let mut records = Vec::new();
    let mut ids = BTreeMap::new();
    let roots = roots
        .iter()
        .map(|v| visit(v, &mut ids, &mut records))
        .collect::<Vec<_>>();
    json!({"roots":roots,"records":records})
}
fn main() {
    let oracle: Value =
        serde_json::from_str(&std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap())
            .unwrap();
    let (definition, child, body, rule) = fixture();
    let mut updated = child.clone();
    updated["value"] = text(oracle["value"].as_str().unwrap());
    assert_eq!(semantic(&child), semantic(&updated));
    assert_ne!(storage(&child), storage(&updated));
    let mut updated_body = body.clone();
    updated_body.as_array_mut().unwrap()[0] = updated.clone();
    let mut updated_rule = rule.clone();
    updated_rule["nodes"] = updated_body.clone();
    let views = BTreeMap::from([
        (semantic(&child).unwrap(), updated.clone()),
        (semantic(&body).unwrap(), updated_body),
        (semantic(&rule).unwrap(), updated_rule),
    ]);
    let roots = array(vec![updated, child.clone()]);
    let mut count = 0;
    let naive = clone_view(&roots, &BTreeMap::new(), &mut BTreeMap::new(), &mut count);
    assert_ne!(semantic(&naive[0]), semantic(&naive[1]));
    let effective = checked_clone(&roots, &views, usize::MAX, &mut count).unwrap();
    assert_eq!(semantic(&effective[0]), semantic(&effective[1]));
    let functional_reserve = reserve(&definition, &views);
    let static_reserve = reserve(&definition["nodes"], &views);
    assert!(functional_reserve > static_reserve);
    let mut rejected_allocations = 0;
    assert!(checked_clone(
        &definition,
        &views,
        functional_reserve - 1,
        &mut rejected_allocations
    )
    .is_err());
    assert_eq!(rejected_allocations, 0);
    let first = checked_clone(&definition, &views, functional_reserve, &mut count).unwrap();
    let second = checked_clone(&definition, &views, functional_reserve, &mut count).unwrap();
    let body_clone =
        checked_clone(&definition["nodes"], &views, static_reserve, &mut count).unwrap();
    assert_ne!(semantic(&first), semantic(&second));
    assert_eq!(snapshot(&[first.clone()]), oracle["functional"]);
    assert_eq!(snapshot(&[body_clone]), oracle["static"]);
    assert_ne!(
        semantic(&first["nodes"][0]["nodes"][0]["nodes"][0]),
        semantic(&child)
    );
    println!(
        "{}",
        json!({"cowOldStorageAlias":true,"naiveCloneSplitsAlias":true,"oracleCloneScopes":2,
        "independentClones":true,"functionalReserve":functional_reserve,"staticReserve":static_reserve,
        "rejectedDeepCloneAllocations":rejected_allocations,"fullNativeOriginIntegration":"pending",
        "productionPeakAccounting":"pending"})
    );
}
