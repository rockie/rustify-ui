use std::{collections::HashMap, sync::Arc};

use serde_json::{json, Value};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    affine::{intersects, Rect},
    assets::ImageCache,
    document::Node,
    gesture::Camera,
    painter::{canvas_context, paint_node},
    raster_policy::{self, LruCache},
    scene::Frame,
};

#[derive(Debug)]
pub struct Raster {
    pub key: String,
    pub node_id: String,
    pub node_version: u64,
    pub cache_generation: u64,
    /// Straight-alpha RGBA bytes from Canvas 2D; the GPU uploader premultiplies them.
    pub rgba: Vec<u8>,
    pub w: u32,
    pub h: u32,
    pub padding: f64,
    pub width: f64,
    pub height: f64,
}

pub struct RasterCache {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    entries: LruCache<Arc<Raster>>,
    hits: u64,
    misses: u64,
    evictions: u64,
    total_generated: u64,
    last_generated: u64,
    last_resolution: f64,
    last_project_ms: f64,
    generation: u64,
    pending: usize,
    deferred: u64,
}

const RASTER_BUDGET_MS: f64 = 8.0;

impl RasterCache {
    pub fn new() -> Result<Self, JsValue> {
        let (canvas, context) = canvas_context()?;
        Ok(Self {
            canvas,
            context,
            entries: LruCache::default(),
            hits: 0,
            misses: 0,
            evictions: 0,
            total_generated: 0,
            last_generated: 0,
            last_resolution: 0.0,
            last_project_ms: 0.0,
            generation: 0,
            pending: 0,
            deferred: 0,
        })
    }

    pub fn get(
        &mut self,
        node: &Node,
        resolution: f64,
        images: &ImageCache,
    ) -> Result<Option<Arc<Raster>>, JsValue> {
        let key = raster_policy::cache_key(node, resolution);
        if let Some(raster) = self.cached(&key) {
            return Ok(Some(raster));
        }
        self.rasterize(node, resolution, key, images)
    }

    fn cached(&mut self, key: &str) -> Option<Arc<Raster>> {
        if let Some(raster) = self.entries.get(key) {
            self.hits += 1;
            Some(Arc::clone(raster))
        } else {
            self.misses += 1;
            None
        }
    }

    fn rasterize(
        &mut self,
        node: &Node,
        resolution: f64,
        key: String,
        images: &ImageCache,
    ) -> Result<Option<Arc<Raster>>, JsValue> {
        if node.kind == "image" && images.get(node.string("assetId").unwrap_or("")).is_none() {
            return Ok(None);
        }
        let dimensions = raster_policy::dimensions(node, resolution);
        self.canvas.set_width(dimensions.pixel_width);
        self.canvas.set_height(dimensions.pixel_height);
        self.context.scale(dimensions.scale, dimensions.scale)?;
        self.context
            .translate(dimensions.padding, dimensions.padding)?;
        paint_node(&self.context, node, images)?;
        let width = u16::try_from(dimensions.pixel_width)
            .map_err(|_| JsValue::from_str("Raster width exceeds the pixel limit."))?;
        let height = u16::try_from(dimensions.pixel_height)
            .map_err(|_| JsValue::from_str("Raster height exceeds the pixel limit."))?;
        let rgba = self
            .context
            .get_image_data(0_i16.into(), 0_i16.into(), width.into(), height.into())?
            .data()
            .0;
        let bytes = rgba.len();
        let raster = Arc::new(Raster {
            key: key.clone(),
            node_id: node.id.clone(),
            node_version: node.version,
            cache_generation: self.generation,
            rgba,
            w: dimensions.pixel_width,
            h: dimensions.pixel_height,
            padding: dimensions.padding,
            width: dimensions.width,
            height: dimensions.height,
        });
        let previous_entries = self.entries.len();
        if self.entries.insert(key, Arc::clone(&raster), bytes) {
            self.evictions += (previous_entries + 1 - self.entries.len()) as u64;
        }
        self.total_generated += 1;
        Ok(Some(raster))
    }

    pub fn project(
        &mut self,
        frame: &Frame,
        camera: Camera,
        viewport: (f64, f64),
        dpr: f64,
        images: &ImageCache,
        skip: Option<&str>,
        previous: &[Arc<Raster>],
    ) -> Result<Arc<Vec<Arc<Raster>>>, JsValue> {
        let started = now();
        let previously_generated = self.total_generated;
        self.pending = 0;
        let previous: HashMap<_, _> = previous
            .iter()
            .filter(|raster| raster.cache_generation == self.generation)
            .map(|raster| (raster.node_id.as_str(), raster))
            .collect();
        let viewport = Rect::new(
            -camera.x / camera.zoom,
            -camera.y / camera.zoom,
            viewport.0 / camera.zoom,
            viewport.1 / camera.zoom,
        );
        let resolution = raster_policy::resolution(camera.zoom, dpr.min(3.0));
        self.last_resolution = resolution;
        let result = (|| {
            let mut rasters = Vec::new();
            for item in &frame.items {
                let node = &item.node;
                if skip == Some(node.id.as_str())
                    || !matches!(node.kind.as_str(), "text" | "path" | "line" | "image")
                {
                    continue;
                }
                let bounds = Rect::new(
                    item.bounds.x - 100.0,
                    item.bounds.y - 100.0,
                    item.bounds.w + 200.0,
                    item.bounds.h + 200.0,
                );
                if intersects(bounds, viewport) {
                    let key = raster_policy::cache_key(node, resolution);
                    if let Some(raster) = self.cached(&key) {
                        rasters.push(raster);
                        continue;
                    }
                    if let Some(old) = previous
                        .get(node.id.as_str())
                        .filter(|raster| raster.node_version == node.version)
                    {
                        // The displayed frame can retain desired pixels evicted from the LRU.
                        if old.key == key {
                            rasters.push(Arc::clone(old));
                            continue;
                        }
                        if self.total_generated > previously_generated
                            && now() - started >= RASTER_BUDGET_MS
                        {
                            rasters.push(Arc::clone(old));
                            self.pending += 1;
                            self.deferred += 1;
                            continue;
                        }
                    }
                    if let Some(raster) = self.rasterize(node, resolution, key, images)? {
                        rasters.push(raster);
                    }
                }
            }
            Ok(Arc::new(rasters))
        })();
        self.last_generated = self.total_generated - previously_generated;
        self.last_project_ms = now() - started;
        result
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.generation = self.generation.wrapping_add(1);
        self.pending = 0;
    }
    pub fn bytes(&self) -> usize {
        self.entries.bytes()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn pending(&self) -> usize {
        self.pending
    }

    pub fn stats(&self) -> Value {
        json!({
            "bytes": self.entries.bytes(),
            "budget": self.entries.budget(),
            "entries": self.entries.len(),
            "hits": self.hits,
            "misses": self.misses,
            "evictions": self.evictions,
            "lastResolution": self.last_resolution,
            "lastProjectMs": self.last_project_ms,
            "lastGenerated": self.last_generated,
            "totalGenerated": self.total_generated,
            "pending": self.pending,
            "deferred": self.deferred,
            "lastBudgetMs": RASTER_BUDGET_MS,
        })
    }
}

fn now() -> f64 {
    web_sys::window()
        .and_then(|window| window.performance())
        .map_or(0.0, |performance| performance.now())
}
