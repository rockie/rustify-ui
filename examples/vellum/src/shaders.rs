use crate::affine::Point;
use crate::scene::{Clip, SceneItem};

#[derive(Clone, Debug)]
pub struct PackedInstance {
    pub data: [f32; 32],
}

impl PackedInstance {
    pub fn raster(&mut self, padding: f64, width: f64, height: f64) {
        let d = &mut self.data;
        d[4] -= (d[0] + d[2]) * padding as f32;
        d[5] -= (d[1] + d[3]) * padding as f32;
        d[6] = width as f32;
        d[7] = height as f32;
        d[18] = 2.0;
        d[20..24].copy_from_slice(&[0.0, 0.0, 1.0, 1.0]);
    }
}

pub fn pack_instance(s: &SceneItem, origin: Point, clip_id: f32, shadow: bool) -> PackedInstance {
    let n = &s.node;
    let m = s.matrix;
    let mut d = [0.0; 32];
    d[0..8].copy_from_slice(&[
        m[0] as f32,
        m[1] as f32,
        m[2] as f32,
        m[3] as f32,
        (m[4] - origin.x) as f32,
        (m[5] - origin.y) as f32,
        n.w as f32,
        n.h as f32,
    ]);
    d[8..12].copy_from_slice(&color(&n.fill, n.fill_opacity));
    d[12..16].copy_from_slice(&color(&n.stroke, 1.0));
    d[16..20].copy_from_slice(&[
        n.radius as f32,
        n.stroke_width as f32,
        if n.kind == "ellipse" { 1.0 } else { 0.0 },
        s.opacity as f32,
    ]);
    d[24..28].copy_from_slice(&color(&n.fill2, n.fill_opacity));
    d[28] = if n.fill_type == "linear" && n.fill != "none" {
        n.gradient_angle.to_radians() as f32
    } else {
        -1000.0
    };
    d[30] = clip_id;
    if shadow {
        d[4] += n.shadow_x as f32;
        d[5] += n.shadow_y as f32;
        d[8..12].copy_from_slice(&color(
            if n.shadow_color.is_empty() {
                "#000000"
            } else {
                &n.shadow_color
            },
            n.shadow_opacity,
        ));
        d[17] = 0.0;
        d[28] = -1000.0;
        d[29] = if n.shadow_blur == 0.0 {
            20.0
        } else {
            n.shadow_blur
        } as f32
            * 0.5;
    }
    PackedInstance { data: d }
}

pub fn pack_clip(clip: &Clip, origin: Point, previous: f32) -> [f32; 12] {
    let m = clip.inverse;
    [
        m[0] as f32,
        m[1] as f32,
        m[2] as f32,
        m[3] as f32,
        (m[4] + m[0] * origin.x + m[2] * origin.y) as f32,
        (m[5] + m[1] * origin.x + m[3] * origin.y) as f32,
        clip.w as f32,
        clip.h as f32,
        clip.radius as f32,
        previous,
        0.0,
        0.0,
    ]
}

pub fn color(value: &str, opacity: f64) -> [f32; 4] {
    if value.is_empty() || value == "none" {
        return [0.0; 4];
    }
    let mut hex = value.replacen('#', "", 1);
    if hex.len() == 3 {
        hex = hex.chars().flat_map(|c| [c, c]).collect();
    }
    let channel = |start: usize| {
        let digits: String = hex
            .chars()
            .skip(start)
            .take(2)
            .take_while(char::is_ascii_hexdigit)
            .collect();
        u8::from_str_radix(&digits, 16).unwrap_or(0) as f64 / 255.0
    };
    [
        channel(0) as f32,
        channel(2) as f32,
        channel(4) as f32,
        (if hex.len() == 8 { channel(6) } else { 1.0 } * opacity) as f32,
    ]
}

pub fn premultiply_bgra(pixel: u32) -> u32 {
    let a = pixel >> 24;
    let r = ((pixel >> 16 & 255) * a + 127) / 255;
    let g = ((pixel >> 8 & 255) * a + 127) / 255;
    let b = ((pixel & 255) * a + 127) / 255;
    a << 24 | r << 16 | g << 8 | b
}

#[cfg(target_arch = "wasm32")]
pub use gpu::{script_mod, DrawVellumRaster, DrawVellumShape};

#[cfg(target_arch = "wasm32")]
mod gpu {
    use rustify_ui::makepad_widgets::*;

    script_mod! {
        use mod.prelude.widgets_internal.*

        mod.draw.VellumShape = set_type_default() do #(DrawVellumShape::script_shader(vm)){
            ..mod.draw.DrawQuad
            clip_texture: texture_2d(float)
            local: varying(vec2f)
            scene_world: varying(vec2f)
            screen: varying(vec2f)

            vertex: fn() {
                let pad = max(self.props.y * 0.5 + 1.5 / self.viewport.z, self.params.y * 2.0)
                self.local = self.geom.pos * (self.item_box.zw + vec2(pad * 2.0)) - vec2(pad)
                self.scene_world = vec2(
                    self.matrix.x * self.local.x + self.matrix.z * self.local.y,
                    self.matrix.y * self.local.x + self.matrix.w * self.local.y
                ) + self.item_box.xy
                self.screen = self.scene_world * self.viewport.z + self.viewport.xy
                let shifted = self.screen + self.draw_list.view_shift
                self.world = self.draw_list.view_transform * vec4(shifted.x, shifted.y, self.draw_depth + self.draw_call.zbias, 1.0)
                self.vertex_pos = self.draw_pass.camera_projection * (self.draw_pass.camera_view * self.world)
            }

            sd_round: fn(p: vec2, size: vec2, radius: float) -> float {
                let r = min(radius, min(size.x, size.y) * 0.5)
                let q = abs(p - size * 0.5) - size * 0.5 + vec2(r)
                return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - r
            }

            clip_texel: fn(index: float) -> vec4 {
                let size = self.clip_texture.size()
                let row = floor(index / size.x)
                let col = index - row * size.x
                return self.clip_texture.sample_nearest(vec2((col + 0.5) / size.x, (row + 0.5) / size.y))
            }

            clip_alpha: fn() -> float {
                let bounds = vec4(
                    max(self.draw_clip.x, self.draw_list.view_clip.x - self.draw_list.view_shift.x),
                    max(self.draw_clip.y, self.draw_list.view_clip.y - self.draw_list.view_shift.y),
                    min(self.draw_clip.z, self.draw_list.view_clip.z - self.draw_list.view_shift.x),
                    min(self.draw_clip.w, self.draw_list.view_clip.w - self.draw_list.view_shift.y)
                )
                if self.screen.x < bounds.x || self.screen.y < bounds.y || self.screen.x > bounds.z || self.screen.y > bounds.w {
                    discard()
                }
                let aa = 0.75 / self.viewport.z
                var alpha = 1.0
                var clip_id = self.params.z
                var count = 0.0
                loop {
                    if clip_id < 0.0 || count >= 100.0 { break }
                    let matrix = self.clip_texel(clip_id * 3.0)
                    let box = self.clip_texel(clip_id * 3.0 + 1.0)
                    let info = self.clip_texel(clip_id * 3.0 + 2.0)
                    let cp = vec2(
                        matrix.x * self.scene_world.x + matrix.z * self.scene_world.y,
                        matrix.y * self.scene_world.x + matrix.w * self.scene_world.y
                    ) + box.xy
                    alpha = alpha * (1.0 - smoothstep(-aa, aa, self.sd_round(cp, box.zw, info.x)))
                    clip_id = info.y
                    count = count + 1.0
                }
                if alpha < 0.001 { discard() }
                return alpha
            }

            pixel: fn() {
                let clip_alpha = self.clip_alpha()
                let aa = 0.75 / self.viewport.z
                var d = self.sd_round(self.local, self.item_box.zw, self.props.x)
                if self.props.z == 1.0 {
                    let radii = max(self.item_box.zw * 0.5, vec2(0.001))
                    let p = (self.local - radii) / radii
                    let k0 = length(p)
                    let k1 = length(p / radii)
                    d = -min(radii.x, radii.y)
                    if k1 > 0.00001 { d = k0 * (k0 - 1.0) / max(k1, 0.00001) }
                }
                let edge = max(aa, self.params.y)
                let coverage = 1.0 - smoothstep(-edge, edge, d)
                var fill = self.fill
                if self.params.x > -999.0 {
                    let axis = vec2(cos(self.params.x), sin(self.params.x))
                    let len = max(dot(abs(axis), self.item_box.zw), 0.001)
                    let t = clamp(dot(self.local - self.item_box.zw * 0.5, axis) / len + 0.5, 0.0, 1.0)
                    fill = mix(fill, self.fill2, t)
                }
                var alpha = fill.a * coverage
                var rgb = fill.rgb * alpha
                if self.props.y > 0.0 {
                    let sc = 1.0 - smoothstep(-aa, aa, abs(d) - self.props.y * 0.5)
                    let sa = self.stroke.a * sc
                    rgb = self.stroke.rgb * sa + rgb * (1.0 - sa)
                    alpha = sa + alpha * (1.0 - sa)
                }
                return vec4(rgb, alpha) * self.props.w * clip_alpha
            }
        }

        mod.draw.VellumRaster = set_type_default() do #(DrawVellumRaster::script_shader(vm)){
            ..mod.draw.VellumShape
            image_texture: texture_2d(float)

            vertex: fn() {
                self.local = self.geom.pos * self.item_box.zw
                self.scene_world = vec2(
                    self.matrix.x * self.local.x + self.matrix.z * self.local.y,
                    self.matrix.y * self.local.x + self.matrix.w * self.local.y
                ) + self.item_box.xy
                self.screen = self.scene_world * self.viewport.z + self.viewport.xy
                let shifted = self.screen + self.draw_list.view_shift
                self.world = self.draw_list.view_transform * vec4(shifted.x, shifted.y, self.draw_depth + self.draw_call.zbias, 1.0)
                self.vertex_pos = self.draw_pass.camera_projection * (self.draw_pass.camera_view * self.world)
            }

            pixel: fn() {
                let clip_alpha = self.clip_alpha()
                let uv = self.uv.xy + self.local / self.item_box.zw * self.uv.zw
                let sample = self.image_texture.sample_as_bgra(uv)
                return sample * self.props.w * clip_alpha
            }
        }
    }

    #[derive(Script, ScriptHook)]
    #[repr(C)]
    pub struct DrawVellumShape {
        #[deref]
        pub draw_super: DrawQuad,
        #[live]
        pub matrix: Vec4f,
        #[live]
        pub item_box: Vec4f,
        #[live]
        pub fill: Vec4f,
        #[live]
        pub stroke: Vec4f,
        #[live]
        pub props: Vec4f,
        #[live]
        pub uv: Vec4f,
        #[live]
        pub fill2: Vec4f,
        #[live]
        pub params: Vec4f,
        #[live]
        pub viewport: Vec4f,
    }

    impl DrawVellumShape {
        pub fn set_instance(&mut self, instance: &super::PackedInstance) {
            let d = &instance.data;
            let vector = |i| vec4(d[i], d[i + 1], d[i + 2], d[i + 3]);
            self.matrix = vector(0);
            self.item_box = vector(4);
            self.fill = vector(8);
            self.stroke = vector(12);
            self.props = vector(16);
            self.uv = vector(20);
            self.fill2 = vector(24);
            self.params = vector(28);
        }
    }

    #[derive(Script, ScriptHook)]
    #[repr(C)]
    pub struct DrawVellumRaster {
        #[deref]
        pub draw_super: DrawVellumShape,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::affine::{centered, inverse, Point};
    use crate::document::{Document, Node};
    use crate::scene::{compose, Clip};

    #[test]
    fn rebasing_retains_subpixel_offsets_at_large_coordinates() {
        let mut doc = Document::empty("rebase");
        let mut n = Node::new("rect");
        n.id = "n".into();
        n.x = 9_000_000.125;
        n.y = -8_000_000.25;
        doc.add(n);
        let frame = compose(&doc);
        let packed = pack_instance(
            frame.world("n").unwrap(),
            Point::new(9_000_000.0, -8_000_000.0),
            -1.0,
            false,
        );
        assert_eq!(&packed.data[4..6], &[0.125, -0.25]);
    }

    #[test]
    fn clips_transform_rebased_world_back_into_clip_local_space() {
        let matrix = centered(1000.0, 2000.0, 400.0, 300.0, 30.0);
        let clip = Clip {
            matrix,
            inverse: inverse(matrix),
            w: 400.0,
            h: 300.0,
            radius: 16.0,
        };
        let data = pack_clip(&clip, Point::new(1000.0, 2000.0), 7.0);
        let world = crate::affine::point(matrix, 20.0, 30.0);
        let x = (world.x - 1000.0) as f32;
        let y = (world.y - 2000.0) as f32;
        assert!((data[0] * x + data[2] * y + data[4] - 20.0).abs() < 0.0001);
        assert_eq!(&data[8..10], &[16.0, 7.0]);
    }

    #[test]
    fn shadow_preserves_shape_transform_and_uses_half_blur() {
        let mut doc = Document::empty("shadow");
        let mut n = Node::new("ellipse");
        n.id = "n".into();
        n.shadow_x = 4.0;
        n.shadow_y = 6.0;
        n.shadow_blur = 20.0;
        n.stroke_width = 2.0;
        doc.add(n);
        let frame = compose(&doc);
        let p = pack_instance(frame.world("n").unwrap(), Point::default(), -1.0, true);
        assert_eq!(
            (p.data[4], p.data[5], p.data[17], p.data[18], p.data[28], p.data[29]),
            (4.0, 6.0, 0.0, 1.0, -1000.0, 10.0)
        );
    }

    #[test]
    fn raster_padding_is_transformed_with_both_matrix_axes() {
        let mut doc = Document::empty("raster");
        let mut n = Node::new("line");
        n.id = "n".into();
        n.rotation = 90.0;
        doc.add(n);
        let frame = compose(&doc);
        let mut p = pack_instance(frame.world("n").unwrap(), Point::default(), -1.0, false);
        let old = (p.data[4], p.data[5]);
        p.raster(4.0, 168.0, 108.0);
        assert_eq!((p.data[4] - old.0, p.data[5] - old.1), (4.0, -4.0));
        assert_eq!(&p.data[6..8], &[168.0, 108.0]);
    }

    #[test]
    fn color_and_uploaded_pixels_use_matching_premultiplied_alpha() {
        assert_eq!(color("#ff000080", 0.5), [1.0, 0.0, 0.0, 128.0 / 510.0]);
        assert_eq!(premultiply_bgra(0x80ff8040), 0x80804020);
        assert_eq!(premultiply_bgra(0x00ffffff), 0);
    }
}
