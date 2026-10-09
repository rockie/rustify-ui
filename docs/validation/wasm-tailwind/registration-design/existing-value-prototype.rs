//! Isolated storage experiment using the production PublicValue types.
//! No directive handlers, host effects, admission model or production integration.
use rustify_style::compiler::public_api::{PublicObject, PublicValue as Value};
use rustify_style::text::Text;
use std::{collections::BTreeMap, sync::Arc};

type Array = Arc<Vec<Value>>;
type ArrayViews = BTreeMap<usize, (Array, Array)>;

#[derive(Default)]
struct Overlay {
    objects: BTreeMap<usize, (Value, Value)>,
    arrays: ArrayViews,
    // Pin retained array storage versions so pointer keys cannot be recycled.
    versions: BTreeMap<usize, (Array, usize)>,
}
impl Overlay {
    fn array_anchor(&self, array: &Array) -> usize {
        let storage = Arc::as_ptr(array) as usize;
        self.versions
            .get(&storage)
            .map_or(storage, |(_, anchor)| *anchor)
    }
    fn current(&self, original: &Value) -> Value {
        match original {
            Value::Object(object) => self
                .objects
                .get(&(Arc::as_ptr(object.identity()) as usize))
                .map_or_else(|| original.clone(), |(_, current)| current.clone()),
            Value::Array(array) => self.arrays.get(&self.array_anchor(array)).map_or_else(
                || original.clone(),
                |(_, current)| Value::Array(current.clone()),
            ),
            _ => original.clone(),
        }
    }
    fn assign(&mut self, original: &Value, field: &str, value: Value) {
        let mut version = self.current(original);
        version
            .as_object_mut()
            .expect("fixture object")
            .insert(field.into(), value);
        let identity = Arc::as_ptr(original.as_object().unwrap().identity()) as usize;
        self.objects
            .entry(identity)
            .and_modify(|(_, current)| *current = version.clone())
            .or_insert_with(|| (original.clone(), version));
    }
    fn splice(&mut self, original: &Array, index: usize, count: usize, inserted: Vec<Value>) {
        let anchor = self.array_anchor(original);
        let mut version = self.current(&Value::Array(original.clone()));
        version
            .as_array_mut()
            .unwrap()
            .splice(index..index + count, inserted);
        let Value::Array(current) = version else {
            unreachable!()
        };
        self.versions
            .insert(Arc::as_ptr(&current) as usize, (current.clone(), anchor));
        self.arrays
            .entry(anchor)
            .and_modify(|(_, version)| *version = current.clone())
            .or_insert_with(|| (original.clone(), current));
    }
}
fn text(value: &str) -> Value {
    Value::String(value.into())
}
fn node(tag: &str, children: Array) -> Value {
    Value::Object(Arc::new(PublicObject::from([
        ("tag".into(), text(tag)),
        ("nodes".into(), Value::Array(children)),
    ])))
}
fn children(value: &Value) -> Array {
    let Value::Array(children) = value
        .as_object()
        .unwrap()
        .get(&Text::from("nodes"))
        .unwrap()
    else {
        unreachable!()
    };
    children.clone()
}
fn current_children(overlay: &Overlay, value: &Value) -> Value {
    overlay.current(&Value::Array(children(&overlay.current(value))))
}
fn main() {
    let mut overlay = Overlay::default();
    let shared = Arc::new(vec![text("removed"), text("kept")]);
    let left = node("left", shared.clone());
    let right = node("right", shared.clone());
    let before = overlay.current(&Value::Array(shared.clone()));
    overlay.splice(&children(&left), 0, 1, vec![]);
    assert_eq!(
        current_children(&overlay, &right).as_array().unwrap(),
        &vec![text("kept")],
        "shared-array aliases read current storage"
    );
    assert_eq!(
        before.as_array().unwrap().len(),
        2,
        "old storage is immutable"
    );

    let Value::Array(first_version) = overlay.current(&Value::Array(shared.clone())) else {
        unreachable!()
    };
    overlay.splice(&shared, 0, 1, vec![]);
    assert!(
        overlay
            .current(&Value::Array(first_version))
            .as_array()
            .unwrap()
            .is_empty(),
        "an earlier storage version still resolves its current anchor"
    );

    let replacement = Arc::new(vec![text("new edge")]);
    overlay.assign(&left, "nodes", Value::Array(replacement));
    assert_eq!(
        current_children(&overlay, &left).as_array().unwrap().len(),
        1
    );
    assert!(
        current_children(&overlay, &right)
            .as_array()
            .unwrap()
            .is_empty(),
        "edge reassignment is distinct from editing the shared array"
    );

    let assignment_version = overlay.current(&left);
    let fresh_body = children(&assignment_version);
    overlay.splice(&fresh_body, 0, 1, vec![]);
    assert_eq!(
        children(&assignment_version).len(),
        1,
        "captured storage does not drift"
    );
    assert!(
        current_children(&overlay, &left)
            .as_array()
            .unwrap()
            .is_empty(),
        "live current readers resolve the new array version"
    );

    let object_version = overlay.current(&left);
    overlay.assign(&left, "tag", text("latest"));
    assert_eq!(
        overlay
            .current(&left)
            .as_object()
            .unwrap()
            .get(&Text::from("tag")),
        Some(&text("latest"))
    );
    assert_eq!(
        object_version.as_object().unwrap().get(&Text::from("tag")),
        Some(&text("left"))
    );
    assert!(
        Arc::ptr_eq(
            left.as_object().unwrap().identity(),
            overlay.current(&left).as_object().unwrap().identity()
        ),
        "COW retains semantic object identity"
    );
    println!(
        "{{\"prototype\":\"existing production PublicValue storage, isolated structural overlay\",\"checks\":5,\"passed\":5,\"productionIntegrated\":false,\"fullRegistrationConformance\":\"pending\"}}"
    );
}
