use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;
use rustify_ui::{Pace, RegionApp};

use crate::affine::{intersects, Point, Rect as SceneRect};
use crate::gesture::Camera;
use crate::raster::Raster;
use crate::scene::Frame;
use crate::shaders::{
    pack_clip, pack_instance, premultiply_bgra, DrawVellumRaster, DrawVellumShape, PackedInstance,
};

#[derive(Clone, Debug)]
pub struct SceneProps {
    pub frame: Arc<Frame>,
    pub rasters: Arc<Vec<Arc<Raster>>>,
    pub camera: Camera,
    pub skip: Option<String>,
}

impl PartialEq for SceneProps {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.frame, &other.frame)
            && Arc::ptr_eq(&self.rasters, &other.rasters)
            && self.camera == other.camera
            && self.skip == other.skip
    }
}

#[derive(Clone, Debug)]
pub enum SceneAction {
    Stats {
        visible: usize,
        instances: usize,
        draw_items: usize,
        cpu_ms: f64,
        gpu_bytes: usize,
    },
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.VellumViewBase = #(VellumView::register_widget(vm))
    mod.widgets.VellumView = set_type_default() do mod.widgets.VellumViewBase{
        width: Fill
        height: Fill
    }

    startup() do #(VellumRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                pass +: { clear_color: #0000 }
                show_caption_bar: false
                body +: {
                    vellum_scene := mod.widgets.VellumView{
                        width: Fill
                        height: Fill
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct VellumRegion {
    #[live]
    ui: WidgetRef,
}

impl RegionApp for VellumRegion {
    type Props = SceneProps;
    type Action = SceneAction;

    fn pace(_: &SceneAction) -> Pace {
        Pace::Continuous("stats")
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        crate::shaders::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &SceneProps) {
        if let Some(mut view) = self
            .ui
            .widget(cx, ids!(vellum_scene))
            .borrow_mut::<VellumView>()
        {
            view.show(cx, props);
        }
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<SceneAction>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        if let Some(mut view) = self
            .ui
            .widget(cx, ids!(vellum_scene))
            .borrow_mut::<VellumView>()
        {
            if let Some(stats) = view.pending_stats.take() {
                outbox.push(stats);
            }
        }
    }
}

struct RasterTexture {
    texture: Texture,
    bytes: usize,
    raster: Arc<Raster>,
}

struct Submission {
    instance: PackedInstance,
    raster: Option<String>,
}

#[derive(Script, ScriptHook, Widget)]
pub struct VellumView {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    shape: DrawVellumShape,
    #[live]
    raster: DrawVellumRaster,
    #[rust]
    props: Option<SceneProps>,
    #[rust]
    textures: HashMap<String, RasterTexture>,
    #[rust]
    clip_texture: Option<Texture>,
    #[rust]
    previous_calls: HashSet<(DrawListId, usize)>,
    #[rust]
    projection_ms: f64,
    #[rust]
    pending_stats: Option<SceneAction>,
}

fn now() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map_or(0.0, |p| p.now())
}

fn bind_texture(cx: &Cx, vars: &mut DrawVars, name: LiveId, texture: &Texture) {
    let Some(shader) = vars.draw_shader_id else {
        return;
    };
    if let Some(slot) = cx.draw_shaders[shader.index]
        .mapping
        .textures
        .iter()
        .position(|input| input.id == name)
    {
        vars.set_texture(slot, texture);
    }
}

impl VellumView {
    fn show(&mut self, cx: &mut Cx, props: &SceneProps) {
        if self.props.as_ref() == Some(props) {
            return;
        }
        let start = now();
        // Retired draw calls otherwise keep old texture keys alive after cache eviction.
        for (list_id, item_id) in self.previous_calls.drain() {
            if let Some(call) = cx.draw_lists[list_id].draw_items[item_id]
                .kind
                .draw_call_mut()
            {
                call.texture_slots.fill(None);
            }
        }
        self.shape.draw_vars.empty_texture(0);
        self.raster.draw_vars.empty_texture(0);
        self.raster.draw_vars.empty_texture(1);
        let keys: HashSet<_> = props.rasters.iter().map(|r| r.key.as_str()).collect();
        self.textures.retain(|key, _| keys.contains(key.as_str()));
        for raster in props.rasters.iter() {
            if self
                .textures
                .get(&raster.key)
                .is_some_and(|cached| Arc::ptr_eq(&cached.raster, raster))
            {
                continue;
            }
            if let Ok(mut pixels) =
                ImageBuffer::new(&raster.rgba, raster.w as usize, raster.h as usize)
            {
                for pixel in &mut pixels.data {
                    *pixel = premultiply_bgra(*pixel);
                }
                let bytes = raster.w as usize * raster.h as usize * 4;
                if let Some(cached) = self.textures.get_mut(&raster.key) {
                    *cached.texture.get_format(cx) = TextureFormat::VecBGRAu8_32 {
                        width: pixels.width,
                        height: pixels.height,
                        data: Some(pixels.data),
                        updated: TextureUpdated::Full,
                    };
                    cached.bytes = bytes;
                    cached.raster = Arc::clone(raster);
                } else {
                    self.textures.insert(
                        raster.key.clone(),
                        RasterTexture {
                            texture: pixels.into_new_texture(cx),
                            bytes,
                            raster: Arc::clone(raster),
                        },
                    );
                }
            }
        }
        self.props = Some(props.clone());
        self.projection_ms = now() - start;
        self.redraw(cx);
    }

    fn record_call(&mut self, area: Area) {
        if let Area::Instance(instance) = area {
            self.previous_calls
                .insert((instance.draw_list_id, instance.draw_item_id));
        }
    }
}

impl Widget for VellumView {
    fn handle_event(&mut self, _: &mut Cx, _: &Event, _: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _: &mut Scope, walk: Walk) -> DrawStep {
        let start = now();
        let pane = cx.walk_turtle(walk);
        let Some(props) = self.props.as_ref() else {
            return DrawStep::done();
        };
        let origin = Point::new(
            -props.camera.x / props.camera.zoom,
            -props.camera.y / props.camera.zoom,
        );
        let viewport = SceneRect::new(
            origin.x,
            origin.y,
            pane.size.x / props.camera.zoom,
            pane.size.y / props.camera.zoom,
        );
        let rasters: HashMap<_, _> = props
            .rasters
            .iter()
            .map(|r| (r.node_id.as_str(), r.as_ref()))
            .collect();
        let mut clips = Vec::new();
        let mut submissions = Vec::new();
        let mut visible = 0;
        for item in &props.frame.items {
            let n = &item.node;
            let b = item.bounds;
            if props.skip.as_deref() == Some(n.id.as_str())
                || n.kind == "group"
                || !intersects(
                    SceneRect::new(b.x - 100.0, b.y - 100.0, b.w + 200.0, b.h + 200.0),
                    viewport,
                )
            {
                continue;
            }
            visible += 1;
            let mut clip_id = -1.0;
            for clip in &item.clips {
                let id = clips.len() / 12;
                clips.extend_from_slice(&pack_clip(clip, origin, clip_id));
                clip_id = id as f32;
            }
            if n.shadow && !matches!(n.kind.as_str(), "text" | "path" | "line") {
                submissions.push(Submission {
                    instance: pack_instance(item, origin, clip_id, true),
                    raster: None,
                });
            }
            let mut instance = pack_instance(item, origin, clip_id, false);
            let raster = if matches!(n.kind.as_str(), "text" | "path" | "line" | "image") {
                let Some(raster) = rasters
                    .get(n.id.as_str())
                    .filter(|r| self.textures.contains_key(&r.key))
                else {
                    continue;
                };
                instance.raster(raster.padding, raster.width, raster.height);
                Some(raster.key.clone())
            } else {
                None
            };
            submissions.push(Submission { instance, raster });
        }
        let zoom = props.camera.zoom as f32;
        let width = (clips.len() / 4).clamp(1, 1024);
        let height = clips.len().div_ceil(width * 4).max(1);
        clips.resize(width * height * 4, 0.0);
        let clip_texture = self
            .clip_texture
            .get_or_insert_with(|| Texture::new_with_format(cx, TextureFormat::Unknown));
        *clip_texture.get_format(cx) = TextureFormat::VecRGBAf32 {
            width,
            height,
            data: Some(clips),
            updated: TextureUpdated::Full,
        };
        bind_texture(
            cx,
            &mut self.shape.draw_vars,
            live_id!(clip_texture),
            clip_texture,
        );
        bind_texture(
            cx,
            &mut self.raster.draw_vars,
            live_id!(clip_texture),
            clip_texture,
        );
        self.shape.viewport = vec4(pane.pos.x as f32, pane.pos.y as f32, zoom, 0.0);
        self.raster.viewport = self.shape.viewport;
        self.shape.draw_clip = vec4(
            pane.pos.x as f32,
            pane.pos.y as f32,
            (pane.pos.x + pane.size.x) as f32,
            (pane.pos.y + pane.size.y) as f32,
        );
        self.raster.draw_clip = self.shape.draw_clip;
        self.previous_calls.clear();
        let instances = submissions.len();
        for submission in submissions {
            if let Some(key) = submission.raster {
                if let Some(texture) = self.textures.get(&key) {
                    bind_texture(
                        cx,
                        &mut self.raster.draw_vars,
                        live_id!(image_texture),
                        &texture.texture,
                    );
                    self.raster.set_instance(&submission.instance);
                    self.raster.draw_abs(cx, pane);
                    self.record_call(self.raster.draw_vars.area);
                }
            } else {
                self.shape.set_instance(&submission.instance);
                self.shape.draw_abs(cx, pane);
                self.record_call(self.shape.draw_vars.area);
            }
        }
        self.pending_stats = Some(SceneAction::Stats {
            visible,
            instances,
            draw_items: self.previous_calls.len(),
            cpu_ms: self.projection_ms + now() - start,
            gpu_bytes: self
                .textures
                .values()
                .map(|texture| texture.bytes)
                .sum::<usize>()
                + width * height * 16,
        });
        self.projection_ms = 0.0;
        DrawStep::done()
    }
}
