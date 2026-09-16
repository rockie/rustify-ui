use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

pub type SharedAssets = BTreeMap<String, Rc<str>>;

pub fn uid() -> String {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    let sequence = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    #[cfg(target_arch = "wasm32")]
    let seed = format!(
        "{:x}{:x}",
        js_sys::Date::now() as u64,
        js_sys::Math::random().to_bits()
    );
    #[cfg(not(target_arch = "wasm32"))]
    let seed = format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |time| time.as_nanos())
    );
    format!("v{seed}-{sequence:x}")
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub rotation: f64,
    pub fill: String,
    pub fill2: String,
    pub fill_type: String,
    pub gradient_angle: f64,
    pub fill_opacity: f64,
    pub stroke: String,
    pub stroke_width: f64,
    pub radius: f64,
    pub opacity: f64,
    pub visible: bool,
    pub locked: bool,
    pub clip: bool,
    pub shadow: bool,
    pub shadow_color: String,
    pub shadow_opacity: f64,
    pub shadow_blur: f64,
    pub shadow_x: f64,
    pub shadow_y: f64,
    pub version: u64,
    pub text: String,
    pub font_family: String,
    pub font_size: f64,
    pub font_weight: f64,
    pub font_style: String,
    pub line_height: f64,
    pub letter_spacing: f64,
    pub text_align: String,
    pub text_decoration: String,
    pub text_case: String,
    pub direction: String,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: uid(),
            kind: "rect".into(),
            name: "Rect".into(),
            parent_id: None,
            x: 0.0,
            y: 0.0,
            w: 160.0,
            h: 100.0,
            rotation: 0.0,
            fill: "#a38bff".into(),
            fill2: "#e1d8ff".into(),
            fill_type: "solid".into(),
            gradient_angle: 90.0,
            fill_opacity: 1.0,
            stroke: "#000000".into(),
            stroke_width: 0.0,
            radius: 0.0,
            opacity: 1.0,
            visible: true,
            locked: false,
            clip: false,
            shadow: false,
            shadow_color: "#000000".into(),
            shadow_opacity: 0.16,
            shadow_blur: 20.0,
            shadow_x: 0.0,
            shadow_y: 6.0,
            version: 0,
            text: String::new(),
            font_family: "Inter".into(),
            font_size: 24.0,
            font_weight: 400.0,
            font_style: "normal".into(),
            line_height: 1.35,
            letter_spacing: 0.0,
            text_align: "left".into(),
            text_decoration: "none".into(),
            text_case: "none".into(),
            direction: "auto".into(),
            extra: Map::new(),
        }
    }
}

impl Node {
    pub fn new(kind: &str) -> Self {
        let mut node = Self {
            kind: kind.into(),
            name: capitalize(kind),
            ..Self::default()
        };
        match kind {
            "text" => {
                node.text = "Type something".into();
                node.fill = "#20202a".into();
                node.w = 240.0;
                node.h = 44.0;
            }
            "frame" => {
                node.fill = "#ffffff".into();
                node.w = 400.0;
                node.h = 300.0;
                node.clip = true;
            }
            "group" => node.fill = "none".into(),
            _ => {}
        }
        node
    }

    pub fn attr(&self, key: &str) -> Option<&Value> {
        self.extra.get(key)
    }
    pub fn number(&self, key: &str, default: f64) -> f64 {
        self.attr(key).and_then(Value::as_f64).unwrap_or(default)
    }
    pub fn flag(&self, key: &str) -> bool {
        self.attr(key).and_then(Value::as_bool).unwrap_or(false)
    }
    pub fn string(&self, key: &str) -> Option<&str> {
        self.attr(key).and_then(Value::as_str)
    }
}

fn capitalize(value: &str) -> String {
    let mut chars = value.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().collect::<String>() + chars.as_str()
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub id: String,
    pub name: String,
    pub nodes: Vec<Node>,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl Page {
    pub fn new(name: &str) -> Self {
        Self {
            id: uid(),
            name: name.into(),
            nodes: Vec::new(),
            extra: Map::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentData {
    pub format: String,
    pub version: u32,
    #[serde(default)]
    pub name: String,
    pub page_id: String,
    pub pages: Vec<Page>,
    #[serde(default = "default_tokens")]
    pub tokens: Value,
    #[serde(default)]
    pub assets: SharedAssets,
    #[serde(default)]
    pub fonts: SharedAssets,
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

#[derive(Clone, Debug)]
pub struct Document {
    pub data: DocumentData,
    pub revision: u64,
    index: HashMap<String, usize>,
}

#[derive(Debug)]
pub enum DocumentError {
    Json(serde_json::Error),
    Invalid(String),
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => error.fmt(f),
            Self::Invalid(message) => message.fmt(f),
        }
    }
}

impl std::error::Error for DocumentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Invalid(_) => None,
        }
    }
}

impl From<serde_json::Error> for DocumentError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

fn invalid(message: &str) -> DocumentError {
    DocumentError::Invalid(message.into())
}

impl Document {
    pub fn new(mut data: DocumentData) -> Self {
        if data.pages.is_empty() {
            data.pages.push(Page::new("Page 1"));
        }
        let mut doc = Self {
            data,
            revision: 0,
            index: HashMap::new(),
        };
        doc.refresh();
        doc
    }

    pub fn empty(name: &str) -> Self {
        let page = Page::new("Page 1");
        Self::new(DocumentData {
            format: "vellum".into(),
            version: 1,
            name: name.into(),
            page_id: page.id.clone(),
            pages: vec![page],
            tokens: default_tokens(),
            assets: BTreeMap::new(),
            fonts: BTreeMap::new(),
            extra: Map::new(),
        })
    }

    fn page_index(&self) -> usize {
        self.data
            .pages
            .iter()
            .position(|page| page.id == self.data.page_id)
            .unwrap_or(0)
    }
    pub fn page(&self) -> &Page {
        &self.data.pages[self.page_index()]
    }
    pub fn page_mut(&mut self) -> &mut Page {
        let index = self.page_index();
        &mut self.data.pages[index]
    }
    pub fn nodes(&self) -> &[Node] {
        &self.page().nodes
    }
    pub fn get(&self, id: &str) -> Option<&Node> {
        self.index
            .get(id)
            .and_then(|index| self.nodes().get(*index))
            .filter(|node| node.id == id)
    }
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Node> {
        let index = *self.index.get(id)?;
        self.page_mut()
            .nodes
            .get_mut(index)
            .filter(|node| node.id == id)
    }

    pub fn refresh(&mut self) {
        self.index = self
            .nodes()
            .iter()
            .enumerate()
            .map(|(index, node)| (node.id.clone(), index))
            .collect();
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn touch(&mut self, id: Option<&str>) {
        if let Some(node) = id.and_then(|id| self.get_mut(id)) {
            node.version = node.version.wrapping_add(1);
        }
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn add(&mut self, node: Node) -> String {
        let id = node.id.clone();
        let index = self.nodes().len();
        self.page_mut().nodes.push(node);
        self.index.insert(id.clone(), index);
        self.touch(Some(&id));
        id
    }

    pub fn children(&self, id: Option<&str>) -> Vec<&Node> {
        self.nodes()
            .iter()
            .filter(|node| node.parent_id.as_deref() == id)
            .collect()
    }

    pub fn descendants(&self, id: &str) -> Vec<&Node> {
        let mut out = Vec::new();
        let mut seen = HashSet::from([id]);
        let mut pending: Vec<_> = self.children(Some(id)).into_iter().rev().collect();
        while let Some(node) = pending.pop() {
            if seen.insert(&node.id) {
                out.push(node);
                pending.extend(self.children(Some(&node.id)).into_iter().rev());
            }
        }
        out
    }

    pub fn ancestors(&self, node: &Node) -> Vec<&Node> {
        let mut out = Vec::new();
        let mut seen = HashSet::from([node.id.as_str()]);
        let mut parent = node.parent_id.as_deref().and_then(|id| self.get(id));
        while let Some(node) = parent {
            if !seen.insert(&node.id) {
                break;
            }
            out.push(node);
            parent = node.parent_id.as_deref().and_then(|id| self.get(id));
        }
        out
    }

    pub fn roots(&self, ids: &[String]) -> Vec<&Node> {
        let selected: HashSet<_> = ids.iter().map(String::as_str).collect();
        ids.iter()
            .filter_map(|id| self.get(id))
            .filter(|node| {
                !self
                    .ancestors(node)
                    .iter()
                    .any(|parent| selected.contains(parent.id.as_str()))
            })
            .collect()
    }

    pub fn remove(&mut self, ids: &[String]) {
        let mut remove: HashSet<String> = ids.iter().cloned().collect();
        for id in ids {
            remove.extend(self.descendants(id).iter().map(|node| node.id.clone()));
        }
        self.page_mut()
            .nodes
            .retain(|node| !remove.contains(&node.id));
        self.refresh();
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        let mut value = serde_json::to_value(&self.data)?;
        value["format"] = json!("vellum");
        value["version"] = json!(1);
        serde_json::to_string_pretty(&value)
    }

    pub fn snapshot(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(
            &json!({"name":self.data.name,"pages":self.data.pages,"pageId":self.data.page_id,"tokens":self.data.tokens}),
        )
    }

    pub fn restore(&mut self, snapshot: &str) -> Result<(), DocumentError> {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Snapshot {
            name: String,
            pages: Vec<Page>,
            page_id: String,
            tokens: Value,
        }
        let snapshot: Snapshot = serde_json::from_str(snapshot)?;
        if snapshot.pages.is_empty() {
            return Err(invalid("This is not a supported Vellum document."));
        }
        self.data.name = snapshot.name;
        self.data.pages = snapshot.pages;
        self.data.page_id = snapshot.page_id;
        self.data.tokens = snapshot.tokens;
        self.refresh();
        Ok(())
    }

    pub fn parse(text: &str) -> Result<Self, DocumentError> {
        let mut value: Value = serde_json::from_str(text)?;
        if value["format"] != "vellum"
            || value["version"].as_f64() != Some(1.0)
            || !value["pages"]
                .as_array()
                .is_some_and(|pages| !pages.is_empty())
        {
            return Err(invalid("This is not a supported Vellum document."));
        }
        let pages = value["pages"]
            .as_array_mut()
            .ok_or_else(|| invalid("This is not a supported Vellum document."))?;
        if pages.len() > 100 {
            return Err(invalid("A document can contain at most 100 pages."));
        }
        let mut all = HashSet::new();
        let mut total = 0;
        for page in pages.iter_mut() {
            if !valid_id(&page["id"]) || !page["name"].is_string() || !page["nodes"].is_array() {
                return Err(invalid("Invalid page."));
            }
            let nodes = page["nodes"]
                .as_array_mut()
                .ok_or_else(|| invalid("Invalid page."))?;
            total += nodes.len();
            if total > 50000 {
                return Err(invalid("Document exceeds 50,000 layers."));
            }
            let ids: HashSet<String> = nodes
                .iter()
                .filter_map(|node| node["id"].as_str().map(str::to_owned))
                .collect();
            for node in nodes.iter_mut() {
                let kind = node["type"].as_str().unwrap_or("").to_owned();
                if !valid_id(&node["id"])
                    || !matches!(
                        kind.as_str(),
                        "rect" | "ellipse" | "text" | "frame" | "group" | "path" | "line" | "image"
                    )
                    || !all.insert(node["id"].as_str().unwrap_or("").to_owned())
                {
                    return Err(invalid("Invalid or duplicate layer."));
                }
                for field in ["x", "y", "w", "h", "rotation", "opacity"] {
                    if !node[field]
                        .as_f64()
                        .is_some_and(|number| number.is_finite() && number.abs() <= 1e7)
                    {
                        return Err(invalid(&format!("Invalid {field} in layer.")));
                    }
                }
                if node["w"].as_f64().unwrap_or(0.0) < 0.0
                    || node["h"].as_f64().unwrap_or(0.0) < 0.0
                {
                    return Err(invalid("Negative layer dimensions."));
                }
                if truthy(&node["parentId"])
                    && !node["parentId"].as_str().is_some_and(|id| ids.contains(id))
                {
                    return Err(invalid("Invalid layer parent."));
                }
                if !node["name"].is_string() {
                    node["name"] = json!(kind);
                }
                if node.get("text").is_some_and(|text| !text.is_string()) {
                    return Err(invalid("Invalid text content."));
                }
                if truthy(&node["points"]) {
                    let points = node["points"]
                        .as_array()
                        .filter(|points| points.len() <= 50000)
                        .ok_or_else(|| invalid("Invalid vector path."))?;
                    for point in points {
                        for coordinate in [point, &point["in"], &point["out"]]
                            .into_iter()
                            .filter(|point| truthy(point))
                        {
                            if !["x", "y"]
                                .into_iter()
                                .all(|axis| coordinate[axis].as_f64().is_some_and(f64::is_finite))
                            {
                                return Err(invalid("Invalid vector point."));
                            }
                        }
                    }
                }
                if node["text"]
                    .as_str()
                    .is_some_and(|text| text.encode_utf16().count() > 100000)
                {
                    return Err(invalid("Text layer is too large."));
                }
                let mut defaults = serde_json::to_value(Node::new(&kind))?;
                if let (Some(defaults), Some(properties)) =
                    (defaults.as_object_mut(), node.as_object())
                {
                    defaults.extend(properties.clone());
                }
                *node = defaults;
            }
            let parents: HashMap<_, _> = nodes
                .iter()
                .map(|node| (node["id"].as_str().unwrap_or(""), node["parentId"].as_str()))
                .collect();
            for node in nodes.iter() {
                let mut seen = HashSet::from([node["id"].as_str().unwrap_or("")]);
                let mut parent = node["parentId"].as_str();
                let mut depth = 0;
                while let Some(id) = parent {
                    depth += 1;
                    if !seen.insert(id) || depth > 100 {
                        return Err(invalid("Cyclic or overly deep hierarchy."));
                    }
                    parent = parents.get(id).copied().flatten();
                }
            }
        }
        let first_page = pages[0]["id"].clone();
        let page_ids: HashSet<String> = pages
            .iter()
            .filter_map(|page| page["id"].as_str().map(str::to_owned))
            .collect();
        if !value["pageId"]
            .as_str()
            .is_some_and(|id| page_ids.contains(id))
        {
            value["pageId"] = first_page;
        }
        if !truthy(&value["assets"]) {
            value["assets"] = json!({});
        }
        let assets = value["assets"]
            .as_object()
            .ok_or_else(|| invalid("Unsupported image asset."))?;
        if !assets
            .values()
            .all(|asset| asset.as_str().is_some_and(supported_image))
        {
            return Err(invalid("Unsupported image asset."));
        }
        if !truthy(&value["tokens"]) {
            value["tokens"] = default_tokens();
        }
        if !truthy(&value["fonts"]) {
            value["fonts"] = json!({});
        }
        value["version"] = json!(1);
        Ok(Self::new(serde_json::from_value(value)?))
    }
}

fn valid_id(value: &Value) -> bool {
    value.as_str().is_some_and(|id| {
        !id.is_empty()
            && id.len() <= 128
            && id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    })
}

fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        Value::Number(value) => value.as_f64().is_some_and(|number| number != 0.0),
        Value::String(value) => !value.is_empty(),
        _ => true,
    }
}

fn supported_image(value: &str) -> bool {
    let prefix = value.split_once(';').map_or(value, |(prefix, _)| prefix);
    value.contains(';')
        && [
            "data:image/png",
            "data:image/jpeg",
            "data:image/webp",
            "data:image/gif",
            "data:image/svg+xml",
        ]
        .iter()
        .any(|candidate| prefix.eq_ignore_ascii_case(candidate))
}

pub fn default_tokens() -> Value {
    json!({"colors":[
        {"name":"Brand / Iris","value":"#8462e8"},{"name":"Brand / Lavender","value":"#eee8fb"},
        {"name":"Neutral / Ink","value":"#242130"},{"name":"Neutral / Paper","value":"#faf9fc"},
        {"name":"Accent / Mint","value":"#c7e6d7"},{"name":"Accent / Peach","value":"#f2d4bb"}
    ],"typography":[
        {"name":"Display / Large","size":40,"weight":600,"lineHeight":1.1},
        {"name":"Heading / Medium","size":24,"weight":600,"lineHeight":1.25},
        {"name":"Body / Regular","size":14,"weight":400,"lineHeight":1.5},
        {"name":"Label / Small","size":11,"weight":500,"lineHeight":1.3}
    ]})
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    fn valid() -> Value {
        json!({"format":"vellum","version":1,"name":"Test","pageId":"page",
        "pages":[{"id":"page","name":"Page","nodes":[{
            "id":"layer","type":"rect","x":0,"y":0,"w":100,"h":80,
            "rotation":0,"opacity":1
        }]}]})
    }

    fn rejects(value: Value, message: &str) {
        assert_eq!(
            Document::parse(&value.to_string()).unwrap_err().to_string(),
            message
        );
    }

    #[test]
    fn parses_forma_with_three_pages_and_202_layers() {
        let doc =
            Document::parse(include_str!("../../../tests/vellum/fixtures/forma.vellum")).unwrap();
        assert_eq!(
            doc.data
                .pages
                .iter()
                .map(|page| page.nodes.len())
                .collect::<Vec<_>>(),
            [171, 31, 0]
        );
        assert_eq!(doc.nodes().len(), 171);
    }

    #[test]
    fn preserves_unknown_fields_at_every_document_level() {
        let mut value = valid();
        value["custom"] = json!({"answer":42});
        value["pages"][0]["custom"] = json!([1, 2]);
        value["pages"][0]["nodes"][0]["custom"] = json!({"nested":[true,"kept"]});
        let doc = Document::parse(&value.to_string()).unwrap();
        let serialized: Value = serde_json::from_str(&doc.serialize().unwrap()).unwrap();
        assert_eq!(serialized["custom"], value["custom"]);
        assert_eq!(
            serialized["pages"][0]["custom"],
            value["pages"][0]["custom"]
        );
        assert_eq!(
            serialized["pages"][0]["nodes"][0]["custom"],
            value["pages"][0]["nodes"][0]["custom"]
        );
        assert_eq!(
            Document::parse(&doc.serialize().unwrap()).unwrap().data,
            doc.data
        );
    }

    #[test]
    fn portable_roundtrip_preserves_geometry_and_unknown_floats_exactly() {
        // These values lost one ULP after rotation, Bézier drawing, and image placement
        // in the continuous browser smoke when parsed without float_roundtrip.
        let coordinates = [109.99999999999997, 100.95558048322647, 426.29809471616716];
        let mut doc = Document::empty("Floating point recovery");
        let mut path = Node::new("path");
        path.x = coordinates[0];
        path.y = coordinates[2];
        path.extra.insert("points".into(), json!([
            {"x":coordinates[1],"y":coordinates[2],"out":{"x":coordinates[0],"y":coordinates[1]}}
        ]));
        path.extra
            .insert("futureGeometry".into(), json!({"samples":coordinates}));
        doc.add(path);
        doc.page_mut()
            .extra
            .insert("futureOrigin".into(), json!(coordinates));
        doc.data
            .extra
            .insert("futurePrecision".into(), json!({"nested":coordinates}));
        let expected = doc.data.clone();
        for _ in 0..3 {
            doc = Document::parse(&doc.serialize().unwrap()).unwrap();
            assert_eq!(doc.data, expected);
        }
    }

    #[test]
    fn applies_type_defaults_and_falls_back_to_first_page() {
        let mut value = valid();
        value["pageId"] = json!("missing");
        value["pages"][0]["nodes"][0]["type"] = json!("text");
        let doc = Document::parse(&value.to_string()).unwrap();
        assert_eq!(doc.data.page_id, "page");
        assert_eq!(doc.nodes()[0].text, "Type something");
        assert_eq!(doc.nodes()[0].fill, "#20202a");
        assert_eq!(doc.nodes()[0].name, "text");
    }

    #[test]
    fn rejects_unsupported_document_format() {
        let mut value = valid();
        value["format"] = json!("figma");
        rejects(value, "This is not a supported Vellum document.");
    }

    #[test]
    fn rejects_unsupported_document_version() {
        let mut value = valid();
        value["version"] = json!(2);
        rejects(value, "This is not a supported Vellum document.");
    }

    #[test]
    fn accepts_json_float_notation_for_version_one() {
        let mut value = valid();
        value["version"] = json!(1.0);
        assert!(Document::parse(&value.to_string()).is_ok());
    }

    #[test]
    fn preserves_empty_string_parent_id_as_the_reference_does() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["parentId"] = json!("");
        let doc = Document::parse(&value.to_string()).unwrap();
        assert_eq!(doc.nodes()[0].parent_id.as_deref(), Some(""));
    }

    #[test]
    fn rejects_empty_pages() {
        let mut value = valid();
        value["pages"] = json!([]);
        rejects(value, "This is not a supported Vellum document.");
    }

    #[test]
    fn rejects_more_than_100_pages() {
        let mut value = valid();
        value["pages"] = json!(vec![value["pages"][0].clone(); 101]);
        rejects(value, "A document can contain at most 100 pages.");
    }

    #[test]
    fn rejects_injected_page_ids() {
        let mut value = valid();
        value["pages"][0]["id"] = json!("page\" onclick=\"alert(1)");
        rejects(value, "Invalid page.");
    }

    #[test]
    fn rejects_invalid_page_name() {
        let mut value = valid();
        value["pages"][0]["name"] = json!(false);
        rejects(value, "Invalid page.");
    }

    #[test]
    fn rejects_invalid_page_nodes() {
        let mut value = valid();
        value["pages"][0]["nodes"] = json!({});
        rejects(value, "Invalid page.");
    }

    #[test]
    fn rejects_more_than_50000_layers() {
        let mut value = valid();
        value["pages"][0]["nodes"] = json!(vec![value["pages"][0]["nodes"][0].clone(); 50001]);
        rejects(value, "Document exceeds 50,000 layers.");
    }

    #[test]
    fn rejects_injected_node_ids() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["id"] = json!("<script>");
        rejects(value, "Invalid or duplicate layer.");
    }

    #[test]
    fn enforces_page_and_layer_id_length_limits() {
        for id in [String::new(), "x".repeat(129)] {
            let mut page = valid();
            page["pages"][0]["id"] = json!(id);
            rejects(page, "Invalid page.");
            let mut layer = valid();
            layer["pages"][0]["nodes"][0]["id"] = json!(id);
            rejects(layer, "Invalid or duplicate layer.");
        }
        let mut value = valid();
        value["pages"][0]["id"] = json!("p".repeat(128));
        value["pages"][0]["nodes"][0]["id"] = json!("n".repeat(128));
        assert!(Document::parse(&value.to_string()).is_ok());
    }

    #[test]
    fn rejects_duplicate_layer_ids_across_pages() {
        let mut value = valid();
        let mut other = value["pages"][0].clone();
        other["id"] = json!("other");
        value["pages"].as_array_mut().unwrap().push(other);
        rejects(value, "Invalid or duplicate layer.");
    }

    #[test]
    fn rejects_unknown_layer_type() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["type"] = json!("video");
        rejects(value, "Invalid or duplicate layer.");
    }

    #[test]
    fn rejects_missing_invalid_and_excessive_geometry() {
        for field in ["x", "y", "w", "h", "rotation", "opacity"] {
            for invalid in [Value::Null, json!("1"), json!(10000001)] {
                let mut value = valid();
                value["pages"][0]["nodes"][0][field] = invalid;
                rejects(value, &format!("Invalid {field} in layer."));
            }
        }
    }

    #[test]
    fn rejects_negative_dimensions() {
        for field in ["w", "h"] {
            let mut value = valid();
            value["pages"][0]["nodes"][0][field] = json!(-1);
            rejects(value, "Negative layer dimensions.");
        }
    }

    #[test]
    fn rejects_parents_on_another_page() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["parentId"] = json!("elsewhere");
        rejects(value, "Invalid layer parent.");
    }

    #[test]
    fn rejects_non_string_text() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["text"] = json!(5);
        rejects(value, "Invalid text content.");
    }

    #[test]
    fn rejects_text_beyond_100000_utf16_units() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["text"] = json!("😀".repeat(50001));
        rejects(value, "Text layer is too large.");
    }

    #[test]
    fn rejects_invalid_vector_paths() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["points"] = json!({});
        rejects(value, "Invalid vector path.");
    }

    #[test]
    fn rejects_more_than_50000_vector_points() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["points"] = json!(vec![json!({"x":0,"y":0}); 50001]);
        rejects(value, "Invalid vector path.");
    }

    #[test]
    fn rejects_invalid_vector_coordinates_and_handles() {
        for point in [
            json!({"x":0}),
            json!({"x":0,"y":0,"in":{"x":"bad","y":0}}),
            json!({"x":0,"y":0,"out":{"x":0,"y":null}}),
        ] {
            let mut value = valid();
            value["pages"][0]["nodes"][0]["points"] = json!([point]);
            rejects(value, "Invalid vector point.");
        }
    }

    #[test]
    fn rejects_cycles() {
        let mut value = valid();
        value["pages"][0]["nodes"][0]["parentId"] = json!("layer");
        rejects(value, "Cyclic or overly deep hierarchy.");
    }

    #[test]
    fn accepts_100_ancestors_and_rejects_101() {
        let mut value = valid();
        let template = value["pages"][0]["nodes"][0].clone();
        value["pages"][0]["nodes"] = json!((0..101)
            .map(|i| {
                let mut node = template.clone();
                node["id"] = json!(format!("n{i}"));
                node["parentId"] = if i == 0 {
                    Value::Null
                } else {
                    json!(format!("n{}", i - 1))
                };
                node
            })
            .collect::<Vec<_>>());
        assert!(Document::parse(&value.to_string()).is_ok());
        let mut too_deep = template;
        too_deep["id"] = json!("n101");
        too_deep["parentId"] = json!("n100");
        value["pages"][0]["nodes"]
            .as_array_mut()
            .unwrap()
            .push(too_deep);
        rejects(value, "Cyclic or overly deep hierarchy.");
    }

    #[test]
    fn rejects_unsupported_image_assets() {
        for asset in [
            json!("https://example.com/image.png"),
            json!("data:text/html;base64,bad"),
            json!(7),
        ] {
            let mut value = valid();
            value["assets"] = json!({"image":asset});
            rejects(value, "Unsupported image asset.");
        }
    }

    #[test]
    fn accepts_supported_image_types_case_insensitively() {
        let mut value = valid();
        value["assets"] = json!({
            "png":"data:image/png;base64,a",
            "jpeg":"data:IMAGE/JPEG;base64,b",
            "webp":"data:image/webp;base64,c",
            "gif":"data:image/gif;base64,d",
            "svg":"data:image/svg+xml;charset=utf-8,%3Csvg/%3E"
        });
        assert_eq!(
            Document::parse(&value.to_string())
                .unwrap()
                .data
                .assets
                .len(),
            5
        );
    }

    #[test]
    fn walks_hierarchy_and_removes_descendants() {
        let mut doc = Document::empty("Test");
        let mut root = Node::new("frame");
        root.id = "root".into();
        let mut child = Node::new("group");
        child.id = "child".into();
        child.parent_id = Some(root.id.clone());
        let mut leaf = Node::new("rect");
        leaf.id = "leaf".into();
        leaf.parent_id = Some(child.id.clone());
        doc.add(root);
        doc.add(child);
        doc.add(leaf);
        assert_eq!(
            doc.descendants("root")
                .iter()
                .map(|n| n.id.as_str())
                .collect::<Vec<_>>(),
            ["child", "leaf"]
        );
        assert_eq!(
            doc.ancestors(doc.get("leaf").unwrap())
                .iter()
                .map(|n| n.id.as_str())
                .collect::<Vec<_>>(),
            ["child", "root"]
        );
        assert_eq!(
            doc.roots(&["leaf".into(), "root".into()])
                .iter()
                .map(|n| n.id.as_str())
                .collect::<Vec<_>>(),
            ["root"]
        );
        doc.remove(&["child".into()]);
        assert_eq!(doc.nodes().len(), 1);
    }

    #[test]
    fn touch_and_refresh_invalidate_revision_and_update_versions() {
        let mut doc = Document::empty("Test");
        let initial = doc.revision;
        let id = doc.add(Node::new("rect"));
        assert_eq!(doc.revision, initial + 1);
        assert_eq!(doc.get(&id).unwrap().version, 1);
        doc.touch(Some(&id));
        assert_eq!(doc.get(&id).unwrap().version, 2);
        doc.refresh();
        assert_eq!(doc.revision, initial + 3);
    }
}
