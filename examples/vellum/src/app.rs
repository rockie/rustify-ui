use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, HashSet};
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use crate::browser_frame::{request_animation_frame_with_handle, AnimationFrameRequestHandle};
use leptos::prelude::*;
use rustify_ui::{AppHandle, GpuRegion, MountConfig, RegionState, Theme, ThemedScope};
use serde_json::{json, Value};
use wasm_bindgen::{JsCast, JsValue};

use crate::assets::ImageCache;
use crate::document::{Document, Node, Page};
use crate::fonts::FontCache;
use crate::gesture::Camera;
use crate::history::History;
use crate::icons::icon;
use crate::painter::CanvasMeasure;
use crate::pointer::InputState;
use crate::raster::RasterCache;
use crate::region::{SceneAction, SceneProps, VellumRegion};
use crate::scene::{compose, Frame};
use crate::shell::ShellState;

thread_local! {
    static EDITOR: Cell<Option<Editor>> = const { Cell::new(None) };
    static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
    static NEXT_HANDLE: Cell<u32> = const { Cell::new(1) };
}

#[derive(Clone, Copy, Debug, Default)]
pub struct RenderStats {
    pub visible: usize,
    pub instances: usize,
    pub draw_items: usize,
    pub cpu_ms: f64,
    pub gpu_bytes: usize,
}

#[derive(Clone)]
pub struct PageView {
    pub camera: Camera,
    pub selection: Vec<String>,
}

#[derive(Clone, Copy)]
pub struct Editor {
    pub doc: RwSignal<Document, LocalStorage>,
    pub camera: RwSignal<Camera>,
    pub selection: RwSignal<Vec<String>>,
    pub input: RwSignal<InputState, LocalStorage>,
    pub text_session: RwSignal<Option<crate::shell::text_session::Session>>,
    pub text_element: StoredValue<Option<(u64, web_sys::HtmlTextAreaElement)>, LocalStorage>,
    pub shell: RwSignal<ShellState, LocalStorage>,
    pub tool: RwSignal<String>,
    pub dark: RwSignal<bool>,
    pub grid: RwSignal<bool>,
    pub snap: RwSignal<bool>,
    pub rulers: RwSignal<bool>,
    pub viewport: RwSignal<(f64, f64, f64)>,
    pub frame: RwSignal<Arc<Frame>>,
    pub stats: RwSignal<RenderStats>,
    pub region_state: RwSignal<RegionState>,
    pub error: RwSignal<Option<String>>,
    pub blank_badges: RwSignal<bool>,
    pub history: StoredValue<History, LocalStorage>,
    pub measure: StoredValue<CanvasMeasure, LocalStorage>,
    pub rasters: StoredValue<RasterCache, LocalStorage>,
    pub images: StoredValue<ImageCache, LocalStorage>,
    pub fonts: StoredValue<FontCache, LocalStorage>,
    pub image_version: RwSignal<u64>,
    pub page_views: StoredValue<BTreeMap<String, PageView>, LocalStorage>,
    pub storage: StoredValue<crate::storage::Storage, LocalStorage>,
    pub storage_status: RwSignal<crate::storage::SaveStatus>,
    pub storage_mode: RwSignal<&'static str>,
    pub hydrated: RwSignal<bool>,
    pub nogpu: bool,
}

impl Editor {
    fn new() -> Result<Self, JsValue> {
        let doc = crate::starter::make_starter().map_err(js_error)?;
        let mut shell = ShellState::default();
        let (options, welcome) = crate::storage::preferences();
        shell.welcome = welcome;
        shell.canvas_color = options.canvas_color;
        let mut selection = Vec::new();
        if let Some(hero) = doc
            .nodes()
            .iter()
            .find(|node| node.kind == "frame" && node.name == "Your ideas. In motion.")
        {
            selection.push(hero.id.clone());
            if let Some(parent) = &hero.parent_id {
                shell.expanded.insert(parent.clone());
            }
        }
        let image_version = RwSignal::new(0u64);
        let images = ImageCache::new(Rc::new(move || {
            let _ = image_version.try_update(|version| *version += 1);
        }));
        let nogpu = window()
            .location()
            .search()
            .unwrap_or_default()
            .split(['?', '&'])
            .any(|part| part == "nogpu" || part == "nogpu=1");
        Ok(Self {
            doc: RwSignal::new_local(doc),
            camera: RwSignal::new(Camera {
                x: 60.0,
                y: 70.0,
                zoom: 0.6,
            }),
            selection: RwSignal::new(selection),
            input: RwSignal::new_local(InputState::default()),
            text_session: RwSignal::new(None),
            text_element: StoredValue::new_local(None),
            shell: RwSignal::new_local(shell),
            tool: RwSignal::new("select".into()),
            dark: RwSignal::new(options.theme == "dark"),
            grid: RwSignal::new(options.grid),
            snap: RwSignal::new(options.snap),
            rulers: RwSignal::new(options.rulers),
            viewport: RwSignal::new((0.0, 0.0, 1.0)),
            frame: RwSignal::new(Arc::new(Frame::default())),
            stats: RwSignal::new(RenderStats::default()),
            region_state: RwSignal::new(RegionState::Starting),
            error: RwSignal::new(None),
            blank_badges: RwSignal::new(false),
            history: StoredValue::new_local(History::default()),
            measure: StoredValue::new_local(CanvasMeasure::new()?),
            rasters: StoredValue::new_local(RasterCache::new()?),
            images: StoredValue::new_local(images),
            fonts: StoredValue::new_local(FontCache::default()),
            image_version,
            page_views: StoredValue::new_local(BTreeMap::new()),
            storage: StoredValue::new_local(crate::storage::Storage::default()),
            storage_status: RwSignal::new(crate::storage::SaveStatus::Hydrating),
            storage_mode: RwSignal::new("IndexedDB"),
            hydrated: RwSignal::new(false),
            nogpu,
        })
    }

    pub fn select(self, ids: Vec<String>) {
        self.selection.set(self.doc.with_untracked(|doc| {
            let mut seen = HashSet::new();
            ids.into_iter()
                .filter(|id| doc.get(id).is_some() && seen.insert(id.clone()))
                .collect()
        }));
        self.input.update(|input| input.path_edit = None);
    }

    pub fn restore_history(self, redo: bool) {
        crate::shell::text_session::finish(self);
        let mut label = None;
        self.doc.update(|doc| {
            self.history.update_value(|history| {
                label = if redo {
                    history.redo(doc)
                } else {
                    history.undo(doc)
                };
            });
        });
        self.reset_raster_caches();
        self.select(self.selection.get_untracked());
        if let Some(label) = label {
            self.refresh_stored_fonts();
            crate::shell::toast(
                self,
                format!("{}: {label}", if redo { "Redo" } else { "Undo" }),
            );
        }
    }

    pub fn delete_selection(self) -> Result<(), JsValue> {
        let selected = self.selection.get_untracked();
        if selected.is_empty() {
            return Ok(());
        }
        self.edit("Delete layers", |doc| {
            doc.remove(&selected);
            Ok(())
        })?;
        self.select(Vec::new());
        Ok(())
    }

    pub fn nudge(self, dx: f64, dy: f64) -> Result<(), JsValue> {
        let selected = self.selection.get_untracked();
        if selected.is_empty() {
            return Ok(());
        }
        self.edit("Nudge layers", |doc| {
            let roots: Vec<_> = doc
                .roots(&selected)
                .into_iter()
                .filter(|node| !node.locked)
                .map(|node| node.id.clone())
                .collect();
            for id in roots {
                if let Some(node) = doc.get_mut(&id) {
                    node.x += dx;
                    node.y += dy;
                    doc.touch(Some(&id));
                }
            }
            Ok(())
        })
    }

    pub fn reorder(self, order: crate::commands::Order) -> Result<(), JsValue> {
        let selected = self.selection.get_untracked();
        if selected.is_empty() {
            return Ok(());
        }
        self.edit("Reorder layers", |doc| {
            crate::commands::reorder(doc, &selected, order);
            Ok(())
        })
    }

    pub fn fit(self, ids: Option<&[String]>) {
        let bounds = self.doc.with_untracked(|doc| compose(doc).bounds(ids));
        let (w, h, _) = self.viewport.get_untracked();
        self.camera
            .set(crate::gesture::fit(bounds, crate::affine::Point::new(w, h)));
    }

    pub fn zoom_at(self, factor: f64, x: Option<f64>, y: Option<f64>) {
        let (w, h, _) = self.viewport.get_untracked();
        let screen = crate::affine::Point::new(x.unwrap_or(w / 2.0), y.unwrap_or(h / 2.0));
        self.camera
            .update(|camera| *camera = crate::gesture::zoom_at(*camera, factor, screen));
    }

    pub fn edit(
        self,
        label: &str,
        change: impl FnOnce(&mut Document) -> Result<(), JsValue>,
    ) -> Result<(), JsValue> {
        crate::shell::text_session::finish(self);
        let mut result = Ok(());
        self.doc.update(|doc| {
            self.history
                .update_value(|history| history.begin(doc, label));
            result = change(doc);
            if result.is_ok() {
                crate::layout::apply_all_layouts(doc);
                result = crate::commands::sync_components(doc).map_err(js_error);
            }
            self.history.update_value(|history| {
                if result.is_ok() {
                    history.commit(doc);
                } else {
                    history.cancel(doc);
                }
            });
        });
        result
    }

    pub fn set_property(self, ids: &[String], prop: &str, value: Value) -> Result<(), JsValue> {
        self.edit(&format!("Change {prop}"), |doc| {
            self.measure.with_value(|measure| {
                crate::commands::set_property(doc, ids, prop, value, measure).map_err(js_error)
            })
        })
    }

    /// History can restore an old node version, so version-keyed rasters must be discarded.
    pub fn reset_raster_caches(self) {
        self.rasters.update_value(|cache| cache.clear());
        self.images.with_value(|images| images.clear());
        self.image_version.update(|version| *version += 1);
    }

    fn ensure_mounted(self) -> Result<(), JsValue> {
        let failed = rustify_makepad::listener_options()
            .and_then(|options| options.get_signal())
            .is_some_and(|signal| signal.aborted());
        if self.doc.is_disposed() || failed {
            Err(JsValue::from_str("Vellum is no longer mounted"))
        } else {
            Ok(())
        }
    }

    pub async fn import_font(self, name: &str, data_url: &str) -> Result<Value, JsValue> {
        self.ensure_mounted()?;
        let family = crate::fonts::family_name(name);
        if family.is_empty() {
            return Err(JsValue::from_str("Font family is empty"));
        }
        let fonts = self.fonts.get_value();
        let source: Rc<str> = data_url.into();
        let loading = fonts.load(&family, Rc::clone(&source))?;
        wasm_bindgen_futures::JsFuture::from(loading).await?;
        self.ensure_mounted()?;
        // The reference saves the face before opening the property history transaction.
        self.doc.update(|doc| {
            doc.data.fonts.insert(family.clone(), source);
            doc.touch(None);
        });
        let selected = self.selection.get_untracked();
        let has_text = self.doc.with_untracked(|doc| {
            selected
                .iter()
                .any(|id| doc.get(id).is_some_and(|node| node.kind == "text"))
        });
        if has_text {
            self.set_property(&selected, "fontFamily", json!(family))?;
        }
        self.reset_raster_caches();
        Ok(json!({"family":family}))
    }

    fn start_stored_fonts(
        self,
    ) -> Result<(FontCache, Vec<wasm_bindgen_futures::JsFuture>), JsValue> {
        self.ensure_mounted()?;
        let sources = self.doc.with_untracked(|doc| doc.data.fonts.clone());
        let fonts = self.fonts.get_value();
        fonts.retain_sources(&sources);
        // Start every face before yielding. A superseded document must not resume
        // its loop later and reintroduce an old source for another family.
        let pending = sources
            .into_iter()
            .filter(|(_, source)| source.starts_with("data:"))
            .filter_map(|(family, source)| {
                fonts
                    .load(&family, source)
                    .ok()
                    .map(wasm_bindgen_futures::JsFuture::from)
            })
            .collect();
        Ok((fonts, pending))
    }

    async fn finish_stored_fonts(
        self,
        fonts: FontCache,
        pending: Vec<wasm_bindgen_futures::JsFuture>,
    ) -> Result<Value, JsValue> {
        for loading in pending {
            // An unavailable stored face must not prevent opening the rest of a file.
            let _ = loading.await;
        }
        self.ensure_mounted()?;
        // Preserve saved layer geometry; the next projection measures the loaded face.
        self.reset_raster_caches();
        fonts.ready().await
    }

    fn refresh_stored_fonts(self) {
        if let Ok((fonts, pending)) = self.start_stored_fonts() {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = self.finish_stored_fonts(fonts, pending).await;
            });
        }
    }

    pub async fn load_stored_fonts(self) -> Result<Value, JsValue> {
        let (fonts, pending) = self.start_stored_fonts()?;
        self.finish_stored_fonts(fonts, pending).await
    }

    pub async fn font_ready(self) -> Result<Value, JsValue> {
        self.ensure_mounted()?;
        let fonts = self.fonts.get_value();
        let status = fonts.ready().await?;
        self.ensure_mounted()?;
        Ok(status)
    }

    pub async fn set_asset(self, id: &str, source: &str) -> Result<Value, JsValue> {
        self.ensure_mounted()?;
        if id.is_empty() {
            return Err(JsValue::from_str("Image asset id is empty"));
        }
        // Reuse the file parser's supported embedded image policy before decoding.
        let mut candidate = Document::empty("Asset validation");
        candidate.data.assets.insert(id.into(), source.into());
        Document::parse(&candidate.serialize().map_err(js_error)?).map_err(js_error)?;
        let image = web_sys::HtmlImageElement::new()?;
        image.set_src(source);
        wasm_bindgen_futures::JsFuture::from(image.decode()).await?;
        self.ensure_mounted()?;
        self.edit("Replace image asset", |doc| {
            doc.data.assets.insert(id.into(), source.into());
            doc.touch(None);
            Ok(())
        })?;
        self.reset_raster_caches();
        self.doc.with_untracked(|doc| {
            self.images.with_value(|images| {
                images.sync(doc);
                images.insert_decoded(id.into(), image.clone());
            });
        });
        Ok(json!({"id":id,"width":image.natural_width(),"height":image.natural_height()}))
    }

    pub fn create_at_center(self, kind: &str, properties: Value) -> Result<Value, JsValue> {
        let mut value = serde_json::to_value(Node::new(kind)).map_err(js_error)?;
        if let (Some(target), Some(properties)) = (value.as_object_mut(), properties.as_object()) {
            target.extend(properties.clone());
        }
        let mut node: Node = serde_json::from_value(value).map_err(js_error)?;
        let (w, h, _) = self.viewport.get_untracked();
        let center = self
            .camera
            .get_untracked()
            .screen_to_world(crate::affine::Point::new(w / 2.0, h / 2.0));
        node.x = center.x - node.w / 2.0;
        node.y = center.y - node.h / 2.0;
        let id = node.id.clone();
        self.edit(&format!("Insert {kind}"), |doc| {
            doc.add(node);
            Ok(())
        })?;
        self.select(vec![id.clone()]);
        self.doc
            .with_untracked(|doc| serde_json::to_value(doc.get(&id)))
            .map_err(js_error)
    }

    pub fn switch_page(self, id: &str) {
        crate::shell::text_session::finish(self);
        if !self
            .doc
            .with_untracked(|doc| doc.data.pages.iter().any(|page| page.id == id))
        {
            return;
        }
        let current = self.doc.with_untracked(|doc| doc.data.page_id.clone());
        self.page_views.update_value(|views| {
            views.insert(
                current,
                PageView {
                    camera: self.camera.get_untracked(),
                    selection: self.selection.get_untracked(),
                },
            );
        });
        self.doc.update(|doc| {
            doc.data.page_id = id.into();
            doc.refresh();
        });
        self.selection.set(Vec::new());
        self.input.update(|input| {
            input.path_edit = None;
            input.pen.clear();
        });
        if let Some(view) = self.page_views.with_value(|views| views.get(id).cloned()) {
            self.camera.set(view.camera);
            self.select(view.selection);
        } else {
            self.fit(None);
        }
    }

    pub fn reset_starter(self) -> Result<(), JsValue> {
        let starter = crate::starter::make_starter().map_err(js_error)?;
        self.edit("Restore example file", |doc| {
            doc.data = starter.data;
            doc.refresh();
            Ok(())
        })?;
        self.reset_raster_caches();
        self.refresh_stored_fonts();
        self.selection.set(Vec::new());
        self.fit(None);
        Ok(())
    }

    pub fn stress_test(self) -> Result<(), JsValue> {
        self.edit("Generate rendering stress test", |doc| {
            let mut page = Page::new("GPU stress test · 5,000 shapes");
            for index in 0..5000 {
                let mut node = Node::new(if index % 3 == 0 { "ellipse" } else { "rect" });
                node.x = (index % 100) as f64 * 30.0;
                node.y = (index / 100) as f64 * 30.0;
                node.w = 24.0;
                node.h = 24.0;
                node.radius = 5.0;
                node.fill = [
                    "#a58ad6", "#7d6bb8", "#c6b3e2", "#d4bfde", "#9dbbb0", "#debea3",
                ][index % 6]
                    .into();
                node.name = format!("Shape {}", index + 1);
                page.nodes.push(node);
            }
            doc.data.page_id = page.id.clone();
            doc.data.pages.push(page);
            doc.refresh();
            Ok(())
        })?;
        self.selection.set(Vec::new());
        self.fit(None);
        Ok(())
    }

    pub fn snapshot(self) -> Value {
        let (width, height, dpr) = self.viewport.get_untracked();
        let camera = self.camera.get_untracked();
        let stats = self.stats.get_untracked();
        let state = self.region_state.get_untracked();
        let error = self.error.get_untracked().or_else(|| {
            if self.nogpu {
                Some("GPU unavailable".into())
            } else if let RegionState::Failed(error) = state {
                Some(error.to_string())
            } else {
                None
            }
        });
        self.doc.with_untracked(|doc| {
            let mut data = serde_json::to_value(&doc.data).unwrap_or(Value::Null);
            let mut input = self.input.with_untracked(InputState::snapshot);
            input["selection"] = json!(self.selection.get_untracked());
            input["camera"] = json!(camera);
            input["tool"] = json!(self.tool.get_untracked());
            let mut storage = self.storage.with_value(|storage| storage.snapshot());
            input["dirty"] = storage["dirty"].clone();
            storage["mode"] = json!(self.storage_mode.get_untracked());
            storage["status"] = json!(self.storage_status.get_untracked().label());
            self.shell.with_untracked(|shell| {
                input["expanded"] = json!(shell.expanded);
                input["leftTab"] = json!(shell.left_tab);
                input["inspectorTab"] = json!(shell.inspector_tab);
                input["previewIndex"] = json!(shell.preview.as_ref().map_or(0, |preview|preview.index));
                input["previewFrames"] = json!(shell.preview.as_ref().map(|preview|preview.frames.clone()).unwrap_or_default());
            });
            data["assets"] = json!(doc.data.assets.keys().map(|key| (key.clone(), Value::Null)).collect::<BTreeMap<_,_>>());
            data["fonts"] = json!(doc.data.fonts.keys().map(|key| (key.clone(), Value::Null)).collect::<BTreeMap<_,_>>());
            json!({
                "ready": self.hydrated.get_untracked() && width > 0.0 && (self.nogpu || matches!(state, RegionState::Ready | RegionState::Failed(_))),
                "doc":{"data":data,"nodes":doc.nodes(),"page":doc.page(),"revision":doc.revision},
                "state":input,"storage":storage,
                "options":{"theme":if self.dark.get_untracked(){"dark"}else{"light"},"grid":self.grid.get_untracked(),"snap":self.snap.get_untracked(),"rulers":self.rulers.get_untracked(),"canvasColor":self.shell.with_untracked(|shell|shell.canvas_color.clone())},
                "renderer":{"backend":"Makepad WebGL2","instanceCount":stats.instances,"visibleCount":stats.visible,"drawCalls":stats.draw_items,"cpuMs":stats.cpu_ms,"gpuBytes":stats.gpu_bytes,"rasterBytes":self.rasters.with_value(|cache|cache.bytes()),"rasterCount":self.rasters.with_value(|cache|cache.len()),"gpuError":error,"width":width,"height":height,"dpr":dpr}
            })
        })
    }
}

pub fn current() -> Result<Editor, JsValue> {
    EDITOR
        .with(|editor| editor.get())
        .ok_or_else(|| JsValue::from_str("Vellum is not mounted"))
}
pub fn js_error(error: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&error.to_string())
}
fn now() -> f64 {
    window()
        .performance()
        .map_or(0.0, |performance| performance.now())
}

fn schedule_raster_frame(
    editor: Editor,
    frame: StoredValue<Option<AnimationFrameRequestHandle>, LocalStorage>,
) {
    if frame.get_value().is_some() || editor.rasters.with_value(|cache| cache.pending() == 0) {
        return;
    }
    match request_animation_frame_with_handle(move || {
        frame.update_value(|request| *request = None);
        editor.image_version.update(|version| *version += 1);
    }) {
        Ok(request) => frame.set_value(Some(request)),
        Err(error) => editor
            .error
            .set(Some(format!("Raster refresh unavailable: {error:?}"))),
    }
}

pub fn mount(container_id: &str) -> Result<u32, JsValue> {
    let container = document()
        .get_element_by_id(container_id)
        .ok_or_else(|| JsValue::from_str("Vellum container not found"))?
        .dyn_into::<web_sys::HtmlElement>()?;
    container.set_class_name("vellum");
    let handle = rustify_ui::mount(
        container,
        MountConfig {
            scope: container_id.into(),
            url_owner: false,
            base: String::new(),
        },
        App,
    )
    .map_err(js_error)?;
    current()?;
    let id = NEXT_HANDLE.with(|next| {
        let id = next.get();
        next.set(id + 1);
        id
    });
    HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
    Ok(id)
}

pub fn dispose(id: u32) -> bool {
    HANDLES
        .with(|handles| handles.borrow_mut().remove(&id))
        .is_some()
}

fn theme(dark: bool) -> Theme {
    let mut theme = if dark { Theme::dark() } else { Theme::light() };
    theme.accent = if dark { 0xa38bff } else { 0x7954df };
    theme.border = if dark { 0x37373b } else { 0xe4e2e8 };
    theme.muted = if dark { 0x85838e } else { 0x928d9c };
    theme
}

#[component]
fn App() -> impl IntoView {
    let editor = match Editor::new() {
        Ok(editor) => editor,
        Err(error) => {
            return view! { <p role="alert">{format!("Vellum could not start: {error:?}")}</p> }
                .into_any()
        }
    };
    EDITOR.with(|slot| slot.set(Some(editor)));
    on_cleanup(move || {
        editor.storage.with_value(|storage| storage.dispose());
        editor.fonts.with_value(|fonts| fonts.dispose());
        editor.images.with_value(|images| images.dispose());
        EDITOR.with(|slot| slot.set(None));
    });
    crate::storage::install(editor, crate::storage::preferences().0);
    let area = NodeRef::<leptos::html::Section>::new();
    let overlay = NodeRef::<leptos::html::Canvas>::new();
    let scene = NodeRef::<leptos::html::Canvas>::new();
    scene.on_load(|canvas| {
        let _ = canvas.set_attribute("premultipliedAlpha", "true");
    });
    let world = NodeRef::<leptos::html::Div>::new();
    let props = RwSignal::new(SceneProps {
        frame: Arc::new(Frame::default()),
        rasters: Arc::new(Vec::new()),
        camera: editor.camera.get_untracked(),
        skip: None,
    });
    let projection_cpu = RwSignal::new(0.0);
    let first_resize = StoredValue::new(false);
    let observation = StoredValue::new_local(None::<rustify_makepad::ResizeObservation>);
    let raster_frame = StoredValue::new_local(None::<AnimationFrameRequestHandle>);
    let pointer_bindings = StoredValue::new_local(None::<crate::pointer::Bindings>);
    let key_bindings = StoredValue::new_local(None::<crate::keys::Bindings>);
    let drop_bindings = StoredValue::new_local(None::<crate::fileio::DropBindings>);
    match crate::keys::install(editor) {
        Ok(bindings) => key_bindings.set_value(Some(bindings)),
        Err(error) => editor
            .error
            .set(Some(format!("Keyboard input unavailable: {error:?}"))),
    }
    Effect::new(move || {
        let Some(canvas) = overlay.get() else {
            return;
        };
        match crate::fileio::install_drop(editor, canvas.clone().unchecked_into()) {
            Ok(bindings) => drop_bindings.set_value(Some(bindings)),
            Err(error) => crate::shell::report(editor, Err(error)),
        }
        match crate::pointer::install(editor, canvas.unchecked_into()) {
            Ok(bindings) => pointer_bindings.set_value(Some(bindings)),
            Err(error) => editor
                .error
                .set(Some(format!("Pointer input unavailable: {error:?}"))),
        }
    });
    Effect::new(move || {
        let Some(area_element) = area.get() else {
            return;
        };
        let element: web_sys::Element = area_element.into();
        let observed = element.clone();
        let update = move || {
            let rect = element.get_bounding_client_rect();
            editor.viewport.set((
                rect.width(),
                rect.height(),
                window().device_pixel_ratio().max(1.0),
            ));
            if !first_resize.get_value() && rect.width() > 0.0 {
                first_resize.set_value(true);
                editor.fit(None);
            }
        };
        update();
        observation.set_value(rustify_makepad::observe_resize(&observed, update));
    });
    on_cleanup(move || {
        pointer_bindings.update_value(|bindings| {
            bindings.take();
        });
        key_bindings.update_value(|bindings| {
            bindings.take();
        });
        drop_bindings.update_value(|bindings| {
            bindings.take();
        });
        observation.update_value(|observer| {
            observer.take();
        });
        raster_frame.update_value(|request| {
            if let Some(request) = request.take() {
                request.cancel();
            }
        });
    });
    Effect::new(move || {
        if let Some(canvas) = scene.get() {
            canvas.set_id("scene");
            canvas
                .set_attribute("aria-label", "Editable design canvas")
                .ok();
        }
    });
    Effect::new(move || {
        let camera = editor.camera.get();
        let grid = editor.grid.get();
        let color = editor.shell.with(|shell| shell.canvas_color.clone());
        let (left_hidden, panels_hidden) = editor
            .shell
            .with(|shell| (shell.left_hidden, shell.panels_hidden));
        if let Some(world) = world.get() {
            let spacing = 20.0 * camera.zoom;
            let world: web_sys::HtmlElement = world.unchecked_into();
            let style = world.style();
            let _ = style.set_property("background-color", color.as_deref().unwrap_or(""));
            if let Ok(Some(root)) = world.closest(".vellum") {
                let _ = root
                    .class_list()
                    .toggle_with_force("left-hidden", left_hidden);
                let _ = root
                    .class_list()
                    .toggle_with_force("panels-hidden", panels_hidden);
            }
            let _ = style.set_property(
                "background-image",
                if grid && spacing >= 5.0 {
                    "radial-gradient(var(--dot) .7px,transparent .7px)"
                } else {
                    "none"
                },
            );
            let _ = style.set_property("background-size", &format!("{spacing}px {spacing}px"));
            let _ = style.set_property(
                "background-position",
                &format!("{}px {}px", camera.x, camera.y),
            );
        }
    });
    Effect::new(move || {
        let has_requests = editor.input.with(|input| !input.requests.is_empty());
        if has_requests {
            untrack(move || crate::shell::consume_pointer_requests(editor));
        }
    });
    Effect::new(move || {
        editor.image_version.track();
        // A newer projection supersedes any refinement queued by an earlier draw.
        raster_frame.update_value(|request| {
            if let Some(request) = request.take() {
                request.cancel();
            }
        });
        let started = now();
        let frame = editor.doc.with(|doc| {
            if editor.images.with_value(|images| images.sync(doc)) {
                editor.rasters.update_value(|cache| cache.clear());
            }
            Arc::new(compose(doc))
        });
        let camera = editor.camera.get();
        let skip = editor
            .text_session
            .with(|session| session.as_ref().map(|session| session.id.clone()));
        let (width, height, dpr) = editor.viewport.get();
        let previous = props.get_untracked().rasters;
        let Some(rasters) = editor.rasters.try_update_value(|cache| {
            editor.images.with_value(|images| {
                cache.project(
                    &frame,
                    camera,
                    (width, height),
                    dpr,
                    images,
                    skip.as_deref(),
                    previous.as_slice(),
                )
            })
        }) else {
            return;
        };
        let pending = rasters.is_ok() && editor.rasters.with_value(|cache| cache.pending() > 0);
        match rasters {
            Ok(rasters) => props.set(SceneProps {
                frame: Arc::clone(&frame),
                rasters,
                camera,
                skip,
            }),
            Err(error) => editor
                .error
                .set(Some(format!("Rasterization failed: {error:?}"))),
        }
        // With a live GPU, wait for Stats from the submitted draw before refining.
        if pending
            && (editor.nogpu
                || matches!(editor.region_state.get_untracked(), RegionState::Failed(_)))
        {
            schedule_raster_frame(editor, raster_frame);
        }
        editor.frame.set(frame);
        projection_cpu.set(now() - started);
    });
    Effect::new(move || {
        if matches!(editor.region_state.get(), RegionState::Failed(_)) {
            schedule_raster_frame(editor, raster_frame);
        }
    });
    Effect::new(move || {
        let frame = editor.frame.get();
        let camera = editor.camera.get();
        let viewport = editor.viewport.get();
        let selection = editor.selection.get();
        let input = editor.input.get();
        let tool = editor.tool.get();
        let dark = editor.dark.get();
        let rulers = editor.rulers.get();
        if let Some(canvas) = overlay.get() {
            let element: web_sys::HtmlElement = canvas.clone().unchecked_into();
            let _ = element.style().set_property("cursor", &input.cursor);
            let result = editor.doc.with(|doc| {
                crate::overlay::draw_overlay(
                    &canvas,
                    doc,
                    &frame,
                    camera,
                    viewport,
                    crate::overlay::OverlayOptions {
                        selection: &selection,
                        input: &input,
                        tool: &tool,
                        dark,
                        rulers,
                    },
                )
            });
            if let Err(error) = result {
                editor
                    .error
                    .set(Some(format!("Overlay unavailable: {error:?}")));
            }
        }
    });
    let gpu = if editor.nogpu {
        view! { <div class="gpu-unavailable" role="alert">"GPU unavailable"</div> }.into_any()
    } else {
        view! { <GpuRegion app={PhantomData::<VellumRegion>} props=Signal::from(props) state=editor.region_state node_ref=scene class="vellum-scene" test_id="scene" on_action=move |action| {
            let SceneAction::Stats { visible, instances, draw_items, cpu_ms, gpu_bytes } = action;
            editor.stats.set(RenderStats { visible, instances, draw_items, cpu_ms: cpu_ms + projection_cpu.get_untracked(), gpu_bytes });
            schedule_raster_frame(editor, raster_frame);
        }/> }.into_any()
    };
    view! {
        <ThemedScope theme=Signal::derive(move || theme(editor.dark.get()))/>
        <crate::shell::topbar::Topbar editor/>
        <main class="workspace">
            <crate::shell::left_panel::LeftPanel editor/>
            <section class="canvas-area" id="canvas-area" aria-label="Design canvas" node_ref=area>
                <div class="canvas-world" id="canvas-world" node_ref=world></div>{gpu}
                <canvas id="overlay" node_ref=overlay tabindex="0" aria-label="Canvas interaction area"></canvas>
                <Show when=move ||!editor.nogpu && matches!(editor.region_state.get(),RegionState::Failed(_))><div class="gpu-unavailable" role="alert">"GPU unavailable"</div></Show>
                <div class="canvas-topline"><span class="canvas-crumb" id="canvas-page-name">{move ||editor.doc.with(|doc|doc.page().name.clone())}</span><button class="canvas-pill" id="canvas-status" on:click=move |_|crate::shell::toast(editor,"Saved in this browser only. Export a .vellum copy for backup.")><i></i><span>"All changes stay on this device"</span></button></div>
                <div class="canvas-footer"><span class="canvas-hint" id="canvas-hint">{move || if !editor.input.with(|input|input.tool_changed) {view!{"Space to pan "<span>"·"</span>" ⌘ scroll to zoom "<span>"·"</span>" ? for shortcuts"}.into_any()} else {match editor.tool.get().as_str() {"pen"=>"Click to add points · Drag for Bézier handles · Enter to finish · Click first point to close","text"=>"Click to add text · Double-click existing text to edit",_=>"Space to pan · ⌘/Ctrl scroll to zoom · ? for shortcuts"}.into_any()}}</span><span class="performance" id="performance">{move || if editor.blank_badges.get(){"Performance".into()}else{let stats=editor.stats.get();format!("{} layers · {:.1} ms CPU",stats.visible,stats.cpu_ms)}}</span></div>
                <crate::shell::toolbar::Toolbar editor/>
                <crate::shell::zoom::Zoom editor/>
                <Show when=move ||editor.shell.with(|shell|shell.welcome)><div class="welcome-tip" id="welcome-tip"><div class="welcome-icon">{icon("sparkles",18)}</div><div><strong>"A little more possible."</strong><p>"Your next idea starts here. Everything is editable."</p></div><button class="icon-button small" id="dismiss-tip" title="Dismiss" aria-label="Dismiss" on:click=move |_|editor.shell.update(|shell|shell.welcome=false)>{icon("close",18)}</button></div></Show>
            </section>
            <crate::shell::inspector::Inspector editor/>
        </main>
        <crate::shell::menus::Menus editor/>
        <crate::shell::dialogs::Dialogs editor/>
        <crate::shell::text_session::TextSession editor canvas=overlay/>
        <crate::shell::presentation::Presentation editor/>
        <crate::shell::toast::Toast editor/>
    }.into_any()
}
