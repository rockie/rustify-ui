//! Node filesystem/module ports; Rust owns discovery, fitting and lifecycle.
use crate::cn::{core_error, define, error, get, js_text, merge_config_values, Limits, Parser};
use js_sys::{Array, Function, JsString, Object, Promise, Reflect, Set};
use rustify_style::cn::{self, build, CnError, EmitLanguage, EmitOptions};
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "any")]
    type ThrownValue;
    #[wasm_bindgen(method,getter,structural,catch,js_name=message)]
    fn thrown_message(this: &ThrownValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(typescript_type = "string")]
    type NativeText;
    #[wasm_bindgen(method,structural,catch,js_name=concat)]
    fn concat_value(this: &NativeText, value: &JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_name=String,catch)]
    fn native_string(value: &JsValue) -> Result<JsValue, JsValue>;
}
fn import_failure(file: &str, cause: &JsValue) -> Result<JsValue, JsValue> {
    let value = cause.unchecked_ref::<ThrownValue>().thrown_message()?;
    let prefix = JsValue::from_str(&format!("cannot load config {file}: "));
    // Native concat supplies ECMAScript ToString and its original exceptions.
    let message = prefix.unchecked_ref::<NativeText>().concat_value(&value)?;
    let wrapped = error("Error", "");
    Reflect::set(&wrapped, &"message".into(), &message)?;
    Reflect::set(&wrapped, &"cause".into(), cause)?;
    Ok(wrapped)
}

fn cli_message(value: &JsValue) -> Result<String, JsValue> {
    let first = if value.is_truthy() {
        value.unchecked_ref::<ThrownValue>().thrown_message()?
    } else {
        JsValue::UNDEFINED
    };
    let selected = if first.is_truthy() {
        value.unchecked_ref::<ThrownValue>().thrown_message()?
    } else {
        value.clone()
    };
    text(&native_string(&selected)?)
}

fn text(value: &JsValue) -> Result<String, JsValue> {
    if !value.is_string() {
        return Err(error(
            "TypeError",
            "filesystem paths and source must be strings",
        ));
    }
    // Node's fs/path ports encode strings as UTF-8; config never crosses this adapter.
    Ok(String::from_utf16_lossy(
        &value.unchecked_ref::<JsString>().iter().collect::<Vec<_>>(),
    ))
}
fn units(value: &JsValue, limits: Limits) -> Result<Vec<u16>, JsValue> {
    if !value.is_string() {
        return Err(error("TypeError", "expected string"));
    }
    let string: &JsString = value.unchecked_ref();
    if string.length() as usize > limits.units {
        return Err(error(
            "RangeError",
            "cn: input limit: UTF-16 units exceeded",
        ));
    }
    Ok(string.iter().collect())
}
fn object(fields: &[(&str, JsValue)]) -> Result<JsValue, JsValue> {
    let out = Object::new();
    for (key, value) in fields {
        define(&out, &(*key).into(), value)?;
    }
    Ok(out.into())
}
fn strings(values: &[String]) -> JsValue {
    values
        .iter()
        .map(|v| JsValue::from_str(v))
        .collect::<Array>()
        .into()
}
fn message(thrown: &JsValue) -> String {
    get(thrown, "message")
        .ok()
        .and_then(|v| text(&v).ok())
        .unwrap_or_else(|| {
            js_sys::JsString::from(thrown.clone())
                .as_string()
                .unwrap_or_default()
        })
}
fn caused(message: &str, cause: &JsValue) -> JsValue {
    let e = error("Error", message);
    let _ = Reflect::set(&e, &"cause".into(), cause);
    e
}
fn function(host: &JsValue, name: &str) -> Result<Function, JsValue> {
    get(host, name)?
        .dyn_into()
        .map_err(|_| error("TypeError", &format!("host.{name} must be a function")))
}
struct Ports {
    host: JsValue,
    last_error: RefCell<Option<JsValue>>,
    exceeded: Cell<bool>,
    nodes: Cell<usize>,
    units: Cell<usize>,
    limits: Limits,
}
impl Ports {
    fn call(&self, name: &str, args: &[JsValue]) -> Result<JsValue, String> {
        if matches!(
            name,
            "readText"
                | "writeText"
                | "realpath"
                | "kind"
                | "children"
                | "dirname"
                | "resolve"
                | "relative"
        ) {
            for value in args.iter().take(if matches!(name, "resolve" | "relative") {
                2
            } else {
                1
            }) {
                if let Some(path) = value.as_string() {
                    if path.split(['/', '\\']).filter(|s| !s.is_empty()).count() > self.limits.depth
                    {
                        self.exceeded.set(true);
                        return Err("cn: input limit: source path depth exceeded".into());
                    }
                }
            }
        }
        let count = self.nodes.get().saturating_add(1);
        self.nodes.set(count);
        let result = if count > self.limits.nodes {
            self.exceeded.set(true);
            Err(error(
                "RangeError",
                "cn: input limit: source operations exceeded",
            ))
        } else {
            function(&self.host, name)
                .and_then(|f| f.apply(&self.host, &args.iter().cloned().collect::<Array>()))
        };
        result.map_err(|e| {
            let m = message(&e);
            *self.last_error.borrow_mut() = Some(e);
            m
        })
    }
    fn read(&self, name: &str, args: &[JsValue]) -> Result<String, String> {
        let value = self.call(name, args)?;
        let s = text(&value).map_err(|e| message(&e))?;
        let count = self.units.get().saturating_add(s.encode_utf16().count());
        self.units.set(count);
        if count > self.limits.units {
            self.exceeded.set(true);
            return Err("cn: input limit: source units exceeded".into());
        }
        Ok(s)
    }
    fn check(&self) -> Result<(), JsValue> {
        if self.exceeded.get() {
            Err(error(
                "RangeError",
                "cn: input limit: source operations or units exceeded",
            ))
        } else {
            Ok(())
        }
    }
    fn failure(&self, failure: CnError) -> JsValue {
        if self.exceeded.get() {
            return error(
                "RangeError",
                "cn: input limit: source operations or units exceeded",
            );
        }
        if let Some(cause) = self.last_error.borrow().as_ref() {
            if failure.message.contains(&message(cause)) {
                return caused(&failure.message, cause);
            }
        }
        core_error(failure)
    }
}
#[derive(Clone)]
struct Driver {
    ports: Rc<Ports>,
    tree: Rc<build::BuildSourceTree>,
    limits: Limits,
}
impl Driver {
    fn new(host: JsValue, limits: Limits) -> Result<Self, JsValue> {
        for key in [
            "cwd",
            "resolve",
            "relative",
            "dirname",
            "readText",
            "writeText",
            "realpath",
            "kind",
            "children",
            "importModule",
            "gzipSize",
        ] {
            function(&host, key)?;
        }
        let ports = Rc::new(Ports {
            host,
            last_error: RefCell::new(None),
            exceeded: Cell::new(false),
            nodes: Cell::new(0),
            units: Cell::new(0),
            limits,
        });
        let p = ports.clone();
        let cwd = Rc::new(move || p.read("cwd", &[]));
        let p = ports.clone();
        let resolve =
            Rc::new(move |base: &str, path: &str| p.read("resolve", &[base.into(), path.into()]));
        let p = ports.clone();
        let relative =
            Rc::new(move |base: &str, path: &str| p.read("relative", &[base.into(), path.into()]));
        let p = ports.clone();
        let dirname = Rc::new(move |path: &str| p.read("dirname", &[path.into()]));
        let p = ports.clone();
        let read_text = Rc::new(move |path: &str| p.read("readText", &[path.into()]));
        let p = ports.clone();
        let write_text = Rc::new(move |path: &str, source: &str| {
            p.call("writeText", &[path.into(), source.into()])
                .map(|_| ())
        });
        let p = ports.clone();
        let realpath = Rc::new(move |path: &str| p.read("realpath", &[path.into()]));
        let p = ports.clone();
        let file_kind = Rc::new(move |path: &str| kind(p.call("kind", &[path.into()])?));
        let p = ports.clone();
        let children = Rc::new(move |path: &str| {
            let value = p.call("children", &[path.into()])?;
            if !Array::is_array(&value) {
                return Err("host.children must return an array".into());
            }
            let input: Array = value.unchecked_into();
            if input.length() as usize > p.limits.nodes {
                p.exceeded.set(true);
                return Err("cn: input limit: source entries exceeded".into());
            }
            input
                .iter()
                .map(|entry| {
                    let entry: Array = entry
                        .dyn_into()
                        .map_err(|_| "host child must be [name, kind]".to_owned())?;
                    Ok((
                        text(&entry.get(0)).map_err(|e| message(&e))?,
                        kind(entry.get(1))?,
                    ))
                })
                .collect()
        });
        Ok(Self {
            tree: Rc::new(build::BuildSourceTree::Host(build::HostSources {
                cwd,
                resolve,
                relative,
                dirname,
                read_text,
                write_text,
                realpath,
                kind: file_kind,
                children,
            })),
            ports,
            limits,
        })
    }
    fn begin(&self, raw_options: JsValue) -> Result<BuildTask, JsValue> {
        // Each flight owns its IO budget and exception realm state, including reentry.
        let driver = Driver::new(self.ports.host.clone(), self.limits)?;
        let raw_options = default_options(raw_options);
        let cwd = option_cwd(&raw_options, &driver)?;
        let raw_out = get(&raw_options, "out")?;
        let out = if raw_out.is_null() || raw_out.is_undefined() {
            "cn-tables.mjs".into()
        } else {
            path_text(&raw_out, self.limits)?
        };
        let out_path = driver.tree.resolve_path(&cwd, &out).map_err(core_error)?;
        let css = if get(&raw_options, "css")? == JsValue::FALSE {
            build::CssInput::Disabled
        } else if get(&raw_options, "css")?.is_undefined() {
            build::CssInput::Discover
        } else {
            build::CssInput::File(path_text(&get(&raw_options, "css")?, self.limits)?)
        };
        let prepared = build::prepare_project_at(&driver.tree, cwd, out_path, &css)
            .map_err(|e| driver.ports.failure(e))?;
        driver.ports.check()?;
        let full = Parser::new(self.limits).config_value(&prepared.config.clone().into())?;
        let config_file = if get(&raw_options, "config")?.is_truthy() {
            Some(path_text(&get(&raw_options, "config")?, self.limits)?)
        } else {
            None
        };
        let imported = if let Some(file) = &config_file {
            let path = driver
                .tree
                .resolve_path(&prepared.cwd, file)
                .map_err(core_error)?;
            Some(
                match function(&driver.ports.host, "importModule")?
                    .call1(&driver.ports.host, &path.into())
                {
                    Ok(value) => Promise::resolve(&value),
                    Err(e) => Promise::reject(&e),
                },
            )
        } else {
            None
        };
        Ok(BuildTask {
            driver,
            raw_options,
            out,
            config_file,
            prepared,
            full,
            imported,
        })
    }
    fn build_promise(&self, raw_options: JsValue) -> Promise {
        let task = match self.begin(raw_options) {
            Ok(task) => task,
            Err(e) => return Promise::reject(&e),
        };
        if let Some(imported) = task.imported.clone() {
            future_to_promise(async move {
                let module = JsFuture::from(imported).await.map_err(|e| {
                    import_failure(task.config_file.as_deref().unwrap_or(""), &e)
                        .unwrap_or_else(|thrown| thrown)
                })?;
                task.finish(Some(module))
            })
        } else {
            match task.finish(None) {
                Ok(value) => Promise::resolve(&value),
                Err(e) => Promise::reject(&e),
            }
        }
    }
}
struct BuildTask {
    driver: Driver,
    raw_options: JsValue,
    out: String,
    config_file: Option<String>,
    prepared: build::PreparedProject,
    full: JsValue,
    imported: Option<Promise>,
}
impl BuildTask {
    fn finish(self, module: Option<JsValue>) -> Result<JsValue, JsValue> {
        let BuildTask {
            driver,
            raw_options,
            out,
            config_file,
            prepared,
            mut full,
            ..
        } = self;
        if let Some(module) = module {
            let default = get(&module, "default")?;
            let extension = if default.is_null() || default.is_undefined() {
                get(&module, "config")?
            } else {
                default
            };
            if !extension.is_truthy() {
                return Err(error(
                    "Error",
                    &format!(
                        "config file {} has no default export",
                        config_file.as_deref().unwrap_or("")
                    ),
                ));
            }
            full = if let Some(transform) = extension.dyn_ref::<Function>() {
                transform.call1(&JsValue::UNDEFINED, &full)?
            } else {
                merge_config_values(&full, &extension, driver.limits)?
            };
        }
        let mut scan = build::ProjectScan {
            tokens: Vec::new(),
            scanned_files: 0,
            warnings: Vec::new(),
        };
        if !get(&raw_options, "full")?.is_truthy() {
            if get(&raw_options, "tokens")?.is_truthy() {
                read_option_words(
                    &driver,
                    &raw_options,
                    "tokens",
                    &prepared.cwd,
                    "tokens file",
                    &mut scan.tokens,
                )?;
            } else {
                let patterns = option_content(&raw_options, driver.limits)?;
                scan = build::scan_project_sources(&driver.tree, &patterns, &prepared)
                    .map_err(|e| driver.ports.failure(e))?;
            }
            if get(&raw_options, "safelist")?.is_truthy() {
                read_option_words(
                    &driver,
                    &raw_options,
                    "safelist",
                    &prepared.cwd,
                    "safelist",
                    &mut scan.tokens,
                )?;
            }
        }
        driver.ports.check()?;
        if scan.tokens.len() > driver.limits.tokens {
            return Err(error(
                "RangeError",
                "cn: input limit: source tokens exceeded",
            ));
        }
        let (config, used, total) = if get(&raw_options, "full")?.is_truthy() {
            (full.clone(), JsValue::NULL, JsValue::NULL)
        } else {
            let tokens = scan
                .tokens
                .iter()
                .map(|t| t.encode_utf16().collect())
                .collect::<Vec<_>>();
            let subset = subset(&full, &tokens, driver.limits)?;
            (
                subset.config,
                (subset.used as f64).into(),
                (subset.total as f64).into(),
            )
        };
        let model = Parser::new(driver.limits).compile_model(&config)?;
        if !model.compiled.tables.custom_validator_names.is_empty() {
            let mut msg="cn compiler: configs with custom validator functions cannot be emitted as a module (functions are not serializable): ".encode_utf16().collect::<Vec<_>>();
            for (i, name) in model
                .compiled
                .tables
                .custom_validator_names
                .iter()
                .enumerate()
            {
                if i > 0 {
                    msg.extend(", ".encode_utf16());
                }
                msg.extend(name);
            }
            msg.extend(". Use createCn(config) at runtime instead.".encode_utf16());
            let e = error("Error", "");
            Reflect::set(&e, &"message".into(), &js_text(&msg))?;
            return Err(e);
        }
        let filename = prepared.out_path.rsplit(['/', '\\']).next().unwrap_or("");
        let banner=format!("// GENERATED by `cn build` — do not edit.\n// Pair with createCn from \"cn/engine\":\n//   import tables from \"./{filename}\"\n//   import {{ createCn }} from \"cn/engine\"\n//   export const cn = createCn(tables)");
        let source = cn::compiler::compile_utf16_model_to_source(
            &model,
            EmitOptions {
                language: if prepared.out_path.ends_with(".ts") {
                    EmitLanguage::Typescript
                } else {
                    EmitLanguage::Javascript
                },
                banner: Some(&banner),
            },
        )
        .map_err(core_error)?;
        if source.encode_utf16().count() > driver.limits.units {
            return Err(error(
                "RangeError",
                "cn: input limit: emitted source units exceeded",
            ));
        }
        let changed = driver
            .tree
            .read_text(&prepared.out_path)
            .map_or(true, |old| old != source);
        driver.ports.check()?;
        if changed {
            driver
                .tree
                .write_text(&prepared.out_path, &source)
                .map_err(|e| {
                    driver
                        .ports
                        .failure(CnError::new(format!("cannot write {out}: {e}")))
                })?;
        }
        driver.ports.check()?;
        let token_set = Set::new(&JsValue::UNDEFINED);
        for t in &scan.tokens {
            token_set.add(&t.into());
        }
        object(&[
            ("outPath", prepared.out_path.into()),
            ("source", source.into()),
            ("changed", changed.into()),
            ("tokens", token_set.into()),
            ("config", config),
            ("fullConfig", full),
            ("scannedFiles", (scan.scanned_files as f64).into()),
            ("cssFiles", strings(&prepared.css_files)),
            ("usedGroups", used),
            ("totalGroups", total),
            ("warnings", strings(&scan.warnings)),
        ])
    }
}
fn kind(value: JsValue) -> Result<build::Kind, String> {
    match value.as_f64() {
        Some(0.) => Ok(build::Kind::File),
        Some(1.) => Ok(build::Kind::Directory),
        Some(2.) => Ok(build::Kind::Symlink),
        Some(3.) => Ok(build::Kind::Other),
        _ => Err("host kind must be 0=file, 1=directory, 2=symlink, 3=other".into()),
    }
}
fn default_options(raw: JsValue) -> JsValue {
    if raw.is_undefined() {
        Object::new().into()
    } else {
        raw
    }
}
fn path_text(raw: &JsValue, limits: Limits) -> Result<String, JsValue> {
    Ok(String::from_utf16_lossy(&units(raw, limits)?))
}
fn option_cwd(raw: &JsValue, driver: &Driver) -> Result<String, JsValue> {
    let value = get(raw, "cwd")?;
    if value.is_null() || value.is_undefined() {
        driver
            .ports
            .read("cwd", &[])
            .map_err(|e| driver.ports.failure(CnError::new(e)))
    } else {
        path_text(&value, driver.limits)
    }
}
fn option_content(raw: &JsValue, limits: Limits) -> Result<Vec<String>, JsValue> {
    let value = get(raw, "content")?;
    if !value.is_null() && !value.is_undefined() && get(&value, "length")?.is_truthy() {
        source_paths(get(raw, "content")?, limits)
    } else {
        Ok(build::DEFAULT_CONTENT.iter().map(|s| (*s).into()).collect())
    }
}
fn read_option_words(
    driver: &Driver,
    raw: &JsValue,
    key: &str,
    cwd: &str,
    what: &str,
    tokens: &mut Vec<String>,
) -> Result<(), JsValue> {
    let file = path_text(&get(raw, key)?, driver.limits)?;
    let path = driver.tree.resolve_path(cwd, &file).map_err(core_error)?;
    let label = get(raw, key)?;
    match driver.tree.read_text(&path) {
        Ok(content) => {
            build::add_project_words(&content, tokens);
            Ok(())
        }
        Err(e) => {
            driver.ports.check()?;
            let cause = driver.ports.last_error.borrow().clone();
            let prefix = JsValue::from_str(&format!("cannot read {what} "));
            let message = prefix.unchecked_ref::<NativeText>().concat_value(&label)?;
            let message = message
                .unchecked_ref::<NativeText>()
                .concat_value(&JsValue::from_str(": "))?;
            let detail = if let Some(cause) = &cause {
                cause.unchecked_ref::<ThrownValue>().thrown_message()?
            } else {
                e.into()
            };
            let message = message
                .unchecked_ref::<NativeText>()
                .concat_value(&detail)?;
            let wrapped = error("Error", "");
            define(
                wrapped.unchecked_ref::<Object>(),
                &"message".into(),
                &message,
            )?;
            if let Some(cause) = cause {
                define(wrapped.unchecked_ref::<Object>(), &"cause".into(), &cause)?;
            }
            Err(wrapped)
        }
    }
}
struct Subset {
    config: JsValue,
    used: usize,
    total: usize,
}
fn clone_config(raw: &JsValue, limits: Limits) -> Result<JsValue, JsValue> {
    // This performs the actual upstream spread/iterable copies; callback handles survive.
    let out = Object::new();
    for key in bounded_keys(raw, limits)?.iter() {
        let descriptor = Reflect::get_own_property_descriptor(raw.unchecked_ref::<Object>(), &key)?;
        if get(&descriptor, "enumerable")?.is_truthy() {
            define(&out, &key, &Reflect::get(raw, &key)?)?;
        }
    }
    for name in [
        "theme",
        "classGroups",
        "conflictingClassGroups",
        "conflictingClassGroupModifiers",
    ] {
        let value = get(raw, name)?;
        let cloned = Object::new();
        if !value.is_null() && !value.is_undefined() {
            for key in bounded_keys(&value, limits)?.iter() {
                let descriptor =
                    Reflect::get_own_property_descriptor(value.unchecked_ref::<Object>(), &key)?;
                if get(&descriptor, "enumerable")?.is_truthy() {
                    define(&cloned, &key, &Reflect::get(&value, &key)?)?;
                }
            }
        }
        define(&out, &name.into(), &cloned.into())?;
    }
    for name in ["orderSensitiveModifiers", "postfixLookupClassGroups"] {
        let value = get(raw, name)?;
        let cloned = Array::new();
        if name == "postfixLookupClassGroups" && !value.is_truthy() {
        } else {
            let iterator = js_sys::try_iter(&value)?
                .ok_or_else(|| error("TypeError", "configuration list is not iterable"))?;
            for value in iterator {
                if cloned.length() as usize >= limits.nodes {
                    return Err(error(
                        "RangeError",
                        "cn: input limit: configuration entries exceeded",
                    ));
                }
                cloned.push(&value?);
            }
        }
        define(&out, &name.into(), &cloned.into())?;
    }
    Ok(out.into())
}
fn subset(raw: &JsValue, tokens: &[Vec<u16>], limits: Limits) -> Result<Subset, JsValue> {
    let cloned = clone_config(raw, limits)?;
    let parser = Parser::new(limits);
    let config = parser.complete(&cloned)?;
    // cloneConfig already materializes prefix; root expansion still precedes its use.
    let prefix = get(&cloned, "prefix")?;
    let prefix = if prefix.is_truthy() {
        Some(units(&prefix, limits)?)
    } else {
        None
    };
    let fitted = cn::compiler::subset_utf16_config_bounded(
        &config,
        tokens,
        limits.depth,
        limits.nodes,
        || Ok(prefix),
    )
    .map_err(|e| parser.propagate(e))?;
    let used: HashSet<Vec<u16>> = fitted
        .config
        .class_groups
        .iter()
        .map(|(k, _)| k.clone())
        .collect();
    for name in [
        "classGroups",
        "conflictingClassGroups",
        "conflictingClassGroupModifiers",
    ] {
        let groups = get(&cloned, name)?;
        for key in Object::keys(groups.unchecked_ref::<Object>()).iter() {
            if !used.contains(&units(&key, limits)?) {
                Reflect::delete_property(groups.unchecked_ref::<Object>(), &key)?;
            }
        }
    }
    Ok(Subset {
        config: cloned,
        used: fitted.used_groups,
        total: fitted.total_groups,
    })
}
#[wasm_bindgen]
pub struct CnBuildSession {
    driver: Driver,
}
#[wasm_bindgen]
impl CnBuildSession {
    #[wasm_bindgen(constructor)]
    pub fn new(host: JsValue, limits: JsValue) -> Result<Self, JsValue> {
        Ok(Self {
            driver: Driver::new(host, Limits::parse(&limits)?)?,
        })
    }
    pub fn build(&self, raw: JsValue) -> Promise {
        self.driver.build_promise(raw)
    }
}

struct BuilderState {
    driver: Driver,
    raw_options: JsValue,
    watch: build::ProjectWatch,
    last: RefCell<Option<JsValue>>,
    running: RefCell<Option<Promise>>,
    queued: Cell<bool>,
}
fn run(state: Rc<BuilderState>) -> Promise {
    let active = state.running.borrow().clone();
    if let Some(promise) = active {
        state.queued.set(true);
        return promise;
    }
    let pending = state.driver.build_promise(state.raw_options.clone());
    let inner = state.clone();
    let promise = future_to_promise(async move {
        match JsFuture::from(pending).await {
            Ok(report) => {
                *inner.last.borrow_mut() = Some(report.clone());
                inner.running.borrow_mut().take();
                if inner.queued.replace(false) {
                    JsFuture::from(run(inner)).await
                } else {
                    Ok(report)
                }
            }
            Err(e) => {
                inner.running.borrow_mut().take();
                inner.queued.set(false);
                Err(e)
            }
        }
    });
    *state.running.borrow_mut() = Some(promise.clone());
    promise
}
fn changed(state: Rc<BuilderState>, file: String) -> Result<Promise, JsValue> {
    let last = state.last.borrow().clone();
    let promise = if let Some(last) = last {
        let abs = state
            .watch
            .resolve_change(&state.driver.tree, &file)
            .map_err(core_error)?;
        let css: Array = get(&last, "cssFiles")?.unchecked_into();
        let known_theme = css.iter().any(|value| value == JsValue::from_str(&abs));
        let selection = if known_theme
            || get(&state.raw_options, "css")?.is_undefined()
                && state
                    .watch
                    .discovered_theme_source(&state.driver.tree, &abs)
                    .map_err(core_error)?
        {
            build::WatchChange::Rebuild
        } else if get(&state.raw_options, "full")?.is_truthy()
            || get(&state.raw_options, "tokens")?.is_truthy()
        {
            build::WatchChange::Keep
        } else {
            let out = text(&get(&last, "outPath")?)?;
            let values = get(&last, "tokens")?;
            let mut tokens = Vec::new();
            for value in js_sys::try_iter(&values)?
                .ok_or_else(|| error("TypeError", "tokens are not iterable"))?
            {
                tokens.push(text(&value?)?);
            }
            state
                .watch
                .source_candidates_at(&state.driver.tree, &abs, &out, &tokens)
                .map_err(core_error)?
        };
        match selection {
            build::WatchChange::Rebuild => run(state.clone()),
            build::WatchChange::Keep => Promise::resolve(&last),
            build::WatchChange::Candidates(fresh) => {
                let input = fresh
                    .iter()
                    .map(|v| v.encode_utf16().collect())
                    .collect::<Vec<_>>();
                let reached =
                    subset(&get(&last, "fullConfig")?, &input, state.driver.limits)?.config;
                let reached = get(&reached, "classGroups")?;
                let kept = get(&get(&last, "config")?, "classGroups")?;
                let mut rebuild = false;
                for key in Object::keys(reached.unchecked_ref::<Object>()).iter() {
                    if !Reflect::has(&kept, &key)? {
                        rebuild = true;
                        break;
                    }
                }
                if rebuild {
                    run(state.clone())
                } else {
                    Promise::resolve(&last)
                }
            }
        }
    } else {
        run(state)
    };
    // Upstream changed is async; its wrapper has its own Promise identity.
    Ok(future_to_promise(
        async move { JsFuture::from(promise).await },
    ))
}
#[wasm_bindgen]
pub struct CnBuilder {
    state: Rc<BuilderState>,
}
impl CnBuilder {
    fn create(driver: Driver, raw_options: JsValue) -> Result<Self, JsValue> {
        let raw_options = default_options(raw_options);
        let cwd = option_cwd(&raw_options, &driver)?;
        let patterns = option_content(&raw_options, driver.limits)?;
        let watch = build::ProjectWatch::new_at(cwd, &patterns, build::ProjectOptions::default())
            .map_err(core_error)?;
        Ok(Self {
            state: Rc::new(BuilderState {
                driver,
                raw_options,
                watch,
                last: RefCell::new(None),
                running: RefCell::new(None),
                queued: Cell::new(false),
            }),
        })
    }
}
#[wasm_bindgen]
impl CnBuilder {
    pub fn run(&self) -> Promise {
        run(self.state.clone())
    }
    pub fn changed(&self, file: JsValue) -> Promise {
        match text(&file).and_then(|file| changed(self.state.clone(), file)) {
            Ok(p) => p,
            Err(e) => Promise::reject(&e),
        }
    }
    #[wasm_bindgen(getter)]
    pub fn last(&self) -> JsValue {
        self.state.last.borrow().clone().unwrap_or(JsValue::NULL)
    }
}
#[wasm_bindgen]
impl CnBuildSession {
    pub fn create_builder(&self, raw: JsValue) -> Result<CnBuilder, JsValue> {
        CnBuilder::create(self.driver.clone(), raw)
    }
    pub fn expand_globs(&self, patterns: JsValue, cwd: JsValue) -> Result<JsValue, JsValue> {
        let patterns = source_paths(patterns, self.driver.limits)?;
        let driver = Driver::new(self.driver.ports.host.clone(), self.driver.limits)?;
        let mut state = build::ScanState::default();
        let files = build::expand_globs(&driver.tree, &patterns, &text(&cwd)?, &mut state)
            .map_err(|e| driver.ports.failure(e))?;
        driver.ports.check()?;
        object(&[
            ("files", strings(&files)),
            (
                "skippedUnreadable",
                (state.skipped_unreadable as f64).into(),
            ),
        ])
    }
    pub fn create_content_matcher(
        &self,
        patterns: JsValue,
        cwd: JsValue,
    ) -> Result<CnContentMatcher, JsValue> {
        let input = source_paths(patterns, self.driver.limits)?;
        Ok(CnContentMatcher {
            tree: self.driver.tree.clone(),
            cwd: text(&cwd)?,
            matcher: build::create_content_matcher(&input, &text(&cwd)?).map_err(core_error)?,
        })
    }
    pub fn theme_from_css(&self, entries: JsValue) -> Result<JsValue, JsValue> {
        let input = source_paths(entries, self.driver.limits)?;
        let driver = Driver::new(self.driver.ports.host.clone(), self.driver.limits)?;
        let theme =
            build::theme_from_css(&driver.tree, &input).map_err(|e| driver.ports.failure(e))?;
        driver.ports.check()?;
        // CSS was decoded by Node UTF-8. Convert only its typed extension, not caller config.
        let extension = theme_extension(&theme.extension, self.driver.limits)?;
        object(&[("extension", extension), ("files", strings(&theme.files))])
    }
}
#[wasm_bindgen]
pub struct CnContentMatcher {
    tree: Rc<build::BuildSourceTree>,
    cwd: String,
    matcher: build::ContentMatcher,
}
#[wasm_bindgen]
impl CnContentMatcher {
    pub fn matches(&self, file: JsValue) -> Result<bool, JsValue> {
        Ok(self.matcher.matches_relative(
            &self
                .tree
                .relative_path(&self.cwd, &text(&file)?)
                .map_err(core_error)?,
        ))
    }
}
#[wasm_bindgen]
pub fn cn_is_tailwind_css(css: JsValue) -> Result<bool, JsValue> {
    Ok(build::is_tailwind_css(&text(&css)?))
}
fn theme_extension(extension: &cn::ConfigExtension, limits: Limits) -> Result<JsValue, JsValue> {
    let parser = Parser::new(limits);
    let mut sections = Vec::new();
    for (name, groups) in [
        ("extend", &extension.extend),
        ("override", &extension.override_groups),
    ] {
        if groups.theme.is_empty() {
            continue;
        }
        let config = cn::Config {
            theme: groups.theme.clone(),
            class_groups: Vec::new(),
            ..cn::Config::default()
        };
        let value = parser.config_value(&config.into())?;
        sections.push((name, object(&[("theme", get(&value, "theme")?)])?));
    }
    object(&sections)
}
#[wasm_bindgen]
pub fn cn_subset_config(
    config: JsValue,
    tokens: JsValue,
    limits: JsValue,
) -> Result<JsValue, JsValue> {
    let limits = Limits::parse(&limits)?;
    let iterator =
        js_sys::try_iter(&tokens)?.ok_or_else(|| error("TypeError", "tokens are not iterable"))?;
    let mut values = Vec::new();
    let mut count = 0usize;
    for token in iterator {
        if values.len() >= limits.tokens {
            return Err(error("RangeError", "cn: input limit: tokens exceeded"));
        }
        let value = units(&token?, limits)?;
        count = count.saturating_add(value.len());
        if count > limits.units {
            return Err(error("RangeError", "cn: input limit: token units exceeded"));
        }
        values.push(value);
    }
    let result = subset(&config, &values, limits)?;
    object(&[
        ("config", result.config),
        ("usedGroups", (result.used as f64).into()),
        ("totalGroups", (result.total as f64).into()),
    ])
}
#[wasm_bindgen]
pub fn cn_compile_to_source(
    config: JsValue,
    raw_options: JsValue,
    limits: JsValue,
) -> Result<JsValue, JsValue> {
    let limits = Limits::parse(&limits)?;
    let model = Parser::new(limits).compile_model(&config)?;
    let raw_options = if raw_options.is_undefined() {
        Object::new().into()
    } else {
        raw_options
    };
    let language = if get(&raw_options, "lang")? == JsValue::from_str("ts") {
        EmitLanguage::Typescript
    } else {
        EmitLanguage::Javascript
    };
    let banner = get(&raw_options, "banner")?;
    let banner = if banner.is_undefined() {
        None
    } else {
        Some(units(&banner, limits)?)
    };
    let source = cn::compiler::compile_utf16_model_to_source(
        &model,
        EmitOptions {
            language,
            banner: banner.as_ref().map(|_| ""),
        },
    )
    .map_err(core_error)?;
    let source = if let Some(mut banner) = banner {
        banner.extend(source.encode_utf16());
        banner
    } else {
        source.encode_utf16().collect()
    };
    if source.len() > limits.units {
        return Err(error(
            "RangeError",
            "cn: input limit: emitted source units exceeded",
        ));
    }
    Ok(js_text(&source))
}
#[wasm_bindgen]
pub fn cn_compile_stats(config: JsValue, limits: JsValue) -> Result<JsValue, JsValue> {
    let model = Parser::new(Limits::parse(&limits)?).compile_model(&config)?;
    let table = &model.compiled.tables;
    object(&[
        ("groups", (table.group_count as f64).into()),
        ("nodes", (table.node_group.len() as f64).into()),
        ("edges", (table.edge_target.len() as f64).into()),
        ("liftedLiterals", (table.lit_anchor.len() as f64).into()),
        (
            "uniqueTails",
            ((table.pool_offsets.len() / 2) as f64).into(),
        ),
        ("vlists", (table.vlist_ref.len() as f64).into()),
    ])
}
fn options_value(options: &build::ProjectOptions) -> Result<JsValue, JsValue> {
    let mut fields = vec![
        ("content", strings(&options.content)),
        ("out", options.out.clone().into()),
        ("full", options.full.into()),
    ];
    for (name, value) in [
        ("cwd", &options.cwd),
        ("safelist", &options.safelist),
        ("config", &options.config),
        ("tokens", &options.tokens),
    ] {
        if let Some(value) = value {
            fields.push((name, value.clone().into()));
        }
    }
    match &options.css {
        build::CssInput::Discover => {}
        build::CssInput::Disabled => fields.push(("css", false.into())),
        build::CssInput::File(file) => fields.push(("css", file.clone().into())),
    }
    object(&fields)
}
fn parse_args(args: JsValue, limits: Limits) -> Result<build::CliCommand, JsValue> {
    let input = source_paths(args, limits)?;
    build::parse_cli(&input).map_err(core_error)
}
fn summary(report: &JsValue, options: &JsValue, driver: &Driver) -> Result<String, JsValue> {
    let raw_source = get(report, "source")?;
    // Node zlib is the original host primitive. Native miniz is a distinct
    // compression implementation and must not determine Node CLI observables.
    let gzip = function(&driver.ports.host, "gzipSize")?
        .call1(&driver.ports.host, &raw_source)?
        .as_f64()
        .ok_or_else(|| error("TypeError", "host.gzipSize must return a byte count"))?;
    if !gzip.is_finite() || gzip < 0. || gzip.fract() != 0. || gzip > u32::MAX as f64 {
        return Err(error(
            "TypeError",
            "host.gzipSize must return a bounded byte count",
        ));
    }
    let parts = if get(options, "full")?.is_truthy() {
        "all class groups kept (--full)".into()
    } else {
        let files = get(report, "scannedFiles")?.as_f64().unwrap_or_default() as usize;
        let count = get(&get(report, "tokens")?, "size")?
            .as_f64()
            .unwrap_or_default() as usize;
        let used = get(report, "usedGroups")?.as_f64().unwrap_or_default() as usize;
        let total = get(report, "totalGroups")?.as_f64().unwrap_or_default() as usize;
        let scan = if files > 0 {
            format!("scanned {files} files, ")
        } else {
            String::new()
        };
        format!("{scan}{count} candidate tokens, {used}/{total} class groups kept")
    };
    let out = path_text(&get(options, "out")?, driver.limits)?;
    let source = units(&get(report, "source")?, driver.limits)?;
    Ok(format!(
        "cn build: {parts}\n  → {} ({:.1} KB source, {:.1} KB gzip)\n",
        out,
        (source.len() as f64 * 10. / 1024.).round() / 10.,
        (gzip * 10. / 1024.).round() / 10.
    ))
}
#[wasm_bindgen]
impl CnBuildSession {
    pub fn parse_cli(&self, args: JsValue) -> Result<JsValue, JsValue> {
        match parse_args(args, self.driver.limits)? {
            build::CliCommand::Help => object(&[("kind", "help".into())]),
            build::CliCommand::Version => object(&[("kind", "version".into())]),
            build::CliCommand::Build { options, quiet } => object(&[
                ("kind", "build".into()),
                ("options", options_value(&options)?),
                ("quiet", quiet.into()),
            ]),
        }
    }
    pub fn run_cli(&self, args: JsValue) -> Promise {
        let parsed = parse_args(args, self.driver.limits);
        let driver = self.driver.clone();
        future_to_promise(async move {
            let result = match parsed {
                Ok(build::CliCommand::Help) => {
                    Ok((build::CLI_HELP.to_owned() + "\n", String::new()))
                }
                Ok(build::CliCommand::Version) => {
                    Ok((build::UPSTREAM_VERSION.to_owned() + "\n", String::new()))
                }
                Ok(build::CliCommand::Build { options, quiet }) => {
                    match JsFuture::from(driver.build_promise(options_value(&options)?)).await {
                        Ok(report) => {
                            if quiet {
                                Ok((String::new(), String::new()))
                            } else {
                                summary(&report, &options_value(&options)?, &driver).and_then(
                                    |stdout| {
                                        let warnings: Array =
                                            get(&report, "warnings")?.unchecked_into();
                                        let mut stderr = String::new();
                                        for warning in warnings.iter() {
                                            stderr.push_str(&format!("cn: {}\n", text(&warning)?));
                                        }
                                        Ok((stdout, stderr))
                                    },
                                )
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            };
            let (stdout, stderr, code) = match result {
                Ok((out, err)) => (out, err, 0),
                Err(e) => (String::new(), format!("cn: {}\n", cli_message(&e)?), 1),
            };
            object(&[
                ("stdout", stdout.into()),
                ("stderr", stderr.into()),
                ("exitCode", code.into()),
            ])
        })
    }
    pub fn cli_summary(&self, report: JsValue, raw_options: JsValue) -> Result<String, JsValue> {
        summary(&report, &raw_options, &self.driver)
    }
    pub fn vite(&self, raw_options: JsValue) -> Result<JsValue, JsValue> {
        let state: Rc<RefCell<Option<Rc<BuilderState>>>> = Rc::new(RefCell::new(None));
        let target = state.clone();
        let driver = self.driver.clone();
        let opts = raw_options.clone();
        let config =
            Closure::<dyn Fn(JsValue) -> Result<(), JsValue>>::new(move |config: JsValue| {
                let raw = Object::new();
                define(&raw, &"cwd".into(), &get(&config, "root")?)?;
                copy_properties(&raw, &opts, driver.limits)?;
                let builder = CnBuilder::create(driver.clone(), raw.into())?;
                *target.borrow_mut() = Some(builder.state);
                Ok(())
            });
        let target = state.clone();
        let driver = self.driver.clone();
        let opts = raw_options.clone();
        let start = Closure::<dyn Fn() -> Promise>::new(move || {
            let result = (|| -> Result<Promise, JsValue> {
                let existing = target.borrow().clone();
                let builder = if let Some(builder) = existing {
                    builder
                } else {
                    let builder = CnBuilder::create(driver.clone(), opts.clone())?.state;
                    *target.borrow_mut() = Some(builder.clone());
                    builder
                };
                let promise = run(builder);
                Ok(future_to_promise(async move {
                    JsFuture::from(promise).await?;
                    Ok(JsValue::UNDEFINED)
                }))
            })();
            result.unwrap_or_else(|e| Promise::reject(&e))
        });
        let target = state;
        let watch = Closure::<dyn Fn(JsValue, JsValue) -> Promise>::new(
            move |id: JsValue, change: JsValue| {
                let result = (|| -> Result<Promise, JsValue> {
                    if get(&change, "event")? == JsValue::from_str("delete") {
                        return Ok(Promise::resolve(&JsValue::UNDEFINED));
                    }
                    let builder = target.borrow().clone();
                    let Some(builder) = builder else {
                        return Ok(Promise::resolve(&JsValue::UNDEFINED));
                    };
                    let promise = changed(builder, text(&id)?)?;
                    Ok(future_to_promise(async move {
                        JsFuture::from(promise).await?;
                        Ok(JsValue::UNDEFINED)
                    }))
                })();
                result.unwrap_or_else(|e| Promise::reject(&e))
            },
        );
        object(&[
            ("name", "cn:build".into()),
            ("configResolved", config.into_js_value()),
            ("buildStart", start.into_js_value()),
            ("watchChange", watch.into_js_value()),
        ])
    }
    pub fn with_cn(&self, next_config: JsValue, raw_options: JsValue) -> Result<JsValue, JsValue> {
        let next_config = if next_config.is_undefined() {
            Object::new().into()
        } else {
            next_config
        };
        let driver = self.driver.clone();
        let factory = Closure::<dyn Fn(JsValue, JsValue) -> Promise>::new(
            move |phase: JsValue, context: JsValue| {
                let driver = driver.clone();
                let config = next_config.clone();
                let raw = raw_options.clone();
                // Like the original async function, creation and run happen before
                // returning its Promise; all failures still reject that Promise.
                let started = CnBuilder::create(driver.clone(), raw.clone()).map(|builder| {
                    let pending = builder.run();
                    (builder, pending)
                });
                future_to_promise(async move {
                    let (builder, pending) = started?;
                    JsFuture::from(pending).await?;
                    if phase == JsValue::from_str("phase-development-server") {
                        let cwd = option_cwd(&default_options(raw), &driver)?;
                        let state = builder.state.clone();
                        let host = driver.ports.host.clone();
                        let directory = cwd.clone();
                        let callback = Closure::<dyn Fn(JsValue, JsValue)>::new(
                            move |_event: JsValue, filename: JsValue| {
                                if !filename.is_truthy() {
                                    return;
                                }
                                let state = state.clone();
                                let host = host.clone();
                                let result = text(&filename)
                                    .and_then(|file| {
                                        state
                                            .driver
                                            .tree
                                            .resolve_path(&directory, &file)
                                            .map_err(core_error)
                                    })
                                    .and_then(|file| changed(state, file));
                                let promise = match result {
                                    Ok(p) => p,
                                    Err(e) => Promise::reject(&e),
                                };
                                wasm_bindgen_futures::spawn_local(async move {
                                    if let Err(e) = JsFuture::from(promise).await {
                                        if let Ok(report) = function(&host, "reportError") {
                                            let selected = if e.dyn_ref::<js_sys::Error>().is_some()
                                            {
                                                e.unchecked_ref::<ThrownValue>().thrown_message()
                                            } else {
                                                Ok(e.clone())
                                            };
                                            if let Ok(value) = selected.and_then(|value| {
                                                JsValue::from_str("cn: ")
                                                    .unchecked_ref::<NativeText>()
                                                    .concat_value(&value)
                                            }) {
                                                let _ = report.call1(&host, &value);
                                            }
                                        }
                                    }
                                });
                            },
                        );
                        function(&driver.ports.host, "watch")?.call2(
                            &driver.ports.host,
                            &cwd.into(),
                            &callback.into_js_value(),
                        )?;
                    }
                    let value = if let Some(factory) = config.dyn_ref::<Function>() {
                        factory.call2(&JsValue::UNDEFINED, &phase, &context)?
                    } else {
                        config
                    };
                    JsFuture::from(Promise::resolve(&value)).await
                })
            },
        );
        Ok(factory.into_js_value())
    }
}
fn copy_properties(out: &Object, value: &JsValue, limits: Limits) -> Result<(), JsValue> {
    if value.is_undefined() || value.is_null() {
        return Ok(());
    }
    for key in bounded_keys(value, limits)?.iter() {
        let descriptor =
            Reflect::get_own_property_descriptor(value.unchecked_ref::<Object>(), &key)?;
        if get(&descriptor, "enumerable")?.is_truthy() {
            define(out, &key, &Reflect::get(value, &key)?)?;
        }
    }
    Ok(())
}

#[wasm_bindgen]
pub fn cn_extract_tokens(
    content: JsValue,
    into: JsValue,
    limits: JsValue,
) -> Result<usize, JsValue> {
    let limits = Limits::parse(&limits)?;
    let content = units(&content, limits)?;
    let mut thrown = None;
    let mut count = 0usize;
    let result = build::extract_tokens_utf16(&content, |candidate| {
        count += 1;
        if count > limits.tokens {
            return Err(CnError::new("cn: input limit: candidate tokens exceeded"));
        }
        let result = function(&into, "add").and_then(|add| add.call1(&into, &js_text(candidate)));
        result.map(|_| ()).map_err(|e| {
            thrown = Some(e);
            CnError::new("cn: candidate add threw")
        })
    });
    result.map_err(|e| thrown.unwrap_or_else(|| core_error(e)))
}

fn bounded_keys(value: &JsValue, limits: Limits) -> Result<Array, JsValue> {
    let keys = Reflect::own_keys(value)?;
    if keys.length() as usize > limits.nodes {
        return Err(error("RangeError", "cn: input limit: object keys exceeded"));
    }
    Ok(keys)
}
fn source_paths(value: JsValue, limits: Limits) -> Result<Vec<String>, JsValue> {
    let values: Array = value
        .dyn_into()
        .map_err(|_| error("TypeError", "paths must be an array"))?;
    if values.length() as usize > limits.nodes {
        return Err(error("RangeError", "cn: input limit: paths exceeded"));
    }
    let mut count = 0usize;
    let mut out = Vec::new();
    for value in values.iter() {
        let value = units(&value, limits)?;
        count = count.saturating_add(value.len());
        if count > limits.units {
            return Err(error("RangeError", "cn: input limit: path units exceeded"));
        }
        out.push(String::from_utf16_lossy(&value));
    }
    Ok(out)
}
