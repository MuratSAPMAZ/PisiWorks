//! Live filters: effects kept on an object and applied again every time it is drawn,
//! so the object underneath stays editable.
//!
//! A filter works on the object drawn alone on a transparent layer. [`apply`] takes
//! that layer as straight RGBA pixels and returns it changed. The list of filters,
//! their order and their settings follow the Filters menu of Fireworks.

use serde::{Deserialize, Serialize};

use crate::blend::BlendMode;
use crate::i18n::t;
use crate::model::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterKind {
    AutoLevels,
    BrightnessContrast,
    ColorFill,
    HueSaturation,
    Invert,
    Levels,
    InnerBevel,
    InsetEmboss,
    OuterBevel,
    RaisedEmboss,
    Blur,
    BlurMore,
    GaussianBlur,
    MotionBlur,
    RadialBlur,
    ZoomBlur,
    AddNoise,
    ConvertToAlpha,
    FindEdges,
    DropShadow,
    Glow,
    InnerGlow,
    InnerShadow,
    SolidShadow,
    Sharpen,
    SharpenMore,
    UnsharpMask,
}

use FilterKind::*;

/// The groups of the Filters menu, in its order.
pub const MENU: [(&str, &[FilterKind]); 7] = [
    ("Adjust Color", &[AutoLevels, BrightnessContrast, ColorFill, HueSaturation, Invert, Levels]),
    ("Bevel and Emboss", &[InnerBevel, InsetEmboss, OuterBevel, RaisedEmboss]),
    ("Blur", &[Blur, BlurMore, GaussianBlur, MotionBlur, RadialBlur, ZoomBlur]),
    ("Noise", &[AddNoise]),
    ("Other", &[ConvertToAlpha, FindEdges]),
    ("Shadow and Glow", &[DropShadow, Glow, InnerGlow, InnerShadow, SolidShadow]),
    ("Sharpen", &[Sharpen, SharpenMore, UnsharpMask]),
];

/// One setting of a filter: its name, its range and what it starts at.
#[derive(Clone, Copy, Debug)]
pub struct Setting {
    pub name: &'static str,
    pub min: f32,
    pub max: f32,
    pub start: f32,
    /// A setting that is only on or off, shown as a box to tick.
    pub toggle: bool,
}

const fn number(name: &'static str, min: f32, max: f32, start: f32) -> Setting {
    Setting { name, min, max, start, toggle: false }
}

const fn toggle(name: &'static str, start: bool) -> Setting {
    Setting { name, min: 0.0, max: 1.0, start: if start { 1.0 } else { 0.0 }, toggle: true }
}

/// The edge shapes of a bevel, by the number stored for "Edge shape".
pub const BEVEL_SHAPES: [&str; 7] = ["Flat", "Smooth", "Sloped", "Frame 1", "Frame 2", "Ring", "Ruffle"];

impl FilterKind {
    pub fn label(self) -> &'static str {
        match self {
            AutoLevels => t("Auto Levels"),
            BrightnessContrast => t("Brightness/Contrast"),
            ColorFill => t("Color Fill"),
            HueSaturation => t("Hue/Saturation"),
            Invert => t("Invert"),
            Levels => t("Levels"),
            InnerBevel => t("Inner Bevel"),
            InsetEmboss => t("Inset Emboss"),
            OuterBevel => t("Outer Bevel"),
            RaisedEmboss => t("Raised Emboss"),
            Blur => t("Blur"),
            BlurMore => t("Blur More"),
            GaussianBlur => t("Gaussian Blur"),
            MotionBlur => t("Motion Blur"),
            RadialBlur => t("Radial Blur"),
            ZoomBlur => t("Zoom Blur"),
            AddNoise => t("Add Noise"),
            ConvertToAlpha => t("Convert to Alpha"),
            FindEdges => t("Find Edges"),
            DropShadow => t("Drop Shadow"),
            Glow => t("Glow"),
            InnerGlow => t("Inner Glow"),
            InnerShadow => t("Inner Shadow"),
            SolidShadow => t("Solid Shadow"),
            Sharpen => t("Sharpen"),
            SharpenMore => t("Sharpen More"),
            UnsharpMask => t("Unsharp Mask"),
        }
    }

    /// The numbers the filter is set with, in the order they are stored.
    pub fn settings(self) -> &'static [Setting] {
        const BEVEL: [Setting; 5] = [
            number("Edge shape", 0.0, 6.0, 1.0),
            number("Width", 0.0, 35.0, 10.0),
            number("Contrast", 0.0, 100.0, 75.0),
            number("Softness", 0.0, 10.0, 3.0),
            number("Angle", 0.0, 360.0, 135.0),
        ];
        const EMBOSS: [Setting; 5] = [
            number("Width", 0.0, 35.0, 2.0),
            number("Contrast", 0.0, 100.0, 75.0),
            number("Softness", 0.0, 10.0, 2.0),
            number("Angle", 0.0, 360.0, 135.0),
            toggle("Show object", true),
        ];
        const SHADOW: [Setting; 5] = [
            number("Distance", 0.0, 100.0, 7.0),
            number("Opacity", 0.0, 100.0, 65.0),
            number("Softness", 0.0, 30.0, 4.0),
            number("Angle", 0.0, 360.0, 315.0),
            toggle("Knock out", false),
        ];
        const GLOW: [Setting; 4] =
            [number("Width", 0.0, 35.0, 6.0), number("Opacity", 0.0, 100.0, 65.0), number("Softness", 0.0, 30.0, 12.0), number("Offset", 0.0, 30.0, 0.0)];
        const BRIGHTNESS: [Setting; 2] = [number("Brightness", -100.0, 100.0, 0.0), number("Contrast", -100.0, 100.0, 0.0)];
        const FILL: [Setting; 2] = [number("Opacity", 0.0, 100.0, 100.0), number("Blend Mode", 0.0, 44.0, 0.0)];
        const HUE: [Setting; 4] =
            [number("Hue", -180.0, 180.0, 0.0), number("Saturation", -100.0, 100.0, 0.0), number("Lightness", -100.0, 100.0, 0.0), toggle("Colorize", false)];
        const LEVELS: [Setting; 5] = [
            number("Input black", 0.0, 255.0, 0.0),
            number("Gamma", 0.1, 10.0, 1.0),
            number("Input white", 0.0, 255.0, 255.0),
            number("Output black", 0.0, 255.0, 0.0),
            number("Output white", 0.0, 255.0, 255.0),
        ];
        const GAUSSIAN: [Setting; 1] = [number("Radius", 0.1, 250.0, 4.0)];
        const MOTION: [Setting; 2] = [number("Angle", 0.0, 360.0, 0.0), number("Distance", 1.0, 100.0, 10.0)];
        const SPIN: [Setting; 2] = [number("Amount", 1.0, 100.0, 10.0), number("Quality", 1.0, 100.0, 20.0)];
        const NOISE: [Setting; 2] = [number("Amount", 0.0, 400.0, 20.0), toggle("Color", false)];
        const SOLID: [Setting; 3] = [number("Angle", 0.0, 360.0, 315.0), number("Distance", 0.0, 100.0, 10.0), toggle("Solid color", true)];
        const UNSHARP: [Setting; 3] =
            [number("Sharpen amount", 1.0, 500.0, 50.0), number("Pixel radius", 0.1, 250.0, 1.0), number("Threshold", 0.0, 255.0, 0.0)];
        match self {
            BrightnessContrast => &BRIGHTNESS,
            ColorFill => &FILL,
            HueSaturation => &HUE,
            Levels => &LEVELS,
            InnerBevel | OuterBevel => &BEVEL,
            InsetEmboss | RaisedEmboss => &EMBOSS,
            GaussianBlur => &GAUSSIAN,
            MotionBlur => &MOTION,
            RadialBlur | ZoomBlur => &SPIN,
            AddNoise => &NOISE,
            DropShadow | InnerShadow => &SHADOW,
            Glow | InnerGlow => &GLOW,
            SolidShadow => &SOLID,
            UnsharpMask => &UNSHARP,
            AutoLevels | Invert | Blur | BlurMore | ConvertToAlpha | FindEdges | Sharpen | SharpenMore => &[],
        }
    }

    /// The color the filter starts with, for those that have one.
    pub fn start_color(self) -> Option<Color> {
        match self {
            ColorFill | Glow | InnerGlow => Some(Color::rgb(255, 0, 0)),
            OuterBevel => Some(Color::rgb(223, 223, 223)),
            DropShadow | InnerShadow | SolidShadow => Some(Color::BLACK),
            _ => None,
        }
    }
}

/// A filter on an object, with its settings.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveFilter {
    pub kind: FilterKind,
    pub values: Vec<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(default = "on")]
    pub enabled: bool,
}

fn on() -> bool {
    true
}

impl LiveFilter {
    pub fn new(kind: FilterKind) -> Self {
        Self { kind, values: kind.settings().iter().map(|s| s.start).collect(), color: kind.start_color(), enabled: true }
    }

    /// A setting by its place, kept inside its range. Files with fewer values than
    /// the filter has today read the missing ones at their starting value.
    pub fn value(&self, index: usize) -> f32 {
        let Some(setting) = self.kind.settings().get(index) else { return 0.0 };
        self.values.get(index).copied().filter(|v| v.is_finite()).unwrap_or(setting.start).clamp(setting.min, setting.max)
    }

    pub fn set_value(&mut self, index: usize, value: f32) {
        let Some(setting) = self.kind.settings().get(index) else { return };
        self.values.resize(self.kind.settings().len(), 0.0);
        self.values[index] = if value.is_finite() { value.clamp(setting.min, setting.max) } else { setting.start };
    }

    fn rgb(&self) -> [f32; 3] {
        let c = self.color.or(self.kind.start_color()).unwrap_or(Color::BLACK);
        [c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0]
    }
}

/// An object drawn alone, as straight RGBA with every channel from 0 to 1.
#[derive(Clone)]
pub struct Layer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[f32; 4]>,
}

impl Layer {
    fn at(&self, x: i64, y: i64) -> [f32; 4] {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return [0.0; 4];
        }
        self.pixels[y as usize * self.width + x as usize]
    }

    /// The pixel at a point between pixels, mixed from the four around it.
    fn sample(&self, x: f32, y: f32) -> [f32; 4] {
        let (left, top) = (x.floor(), y.floor());
        let (fx, fy) = (x - left, y - top);
        let (left, top) = (left as i64, top as i64);
        let mut out = [0.0; 4];
        for (dx, dy, weight) in [(0, 0, (1.0 - fx) * (1.0 - fy)), (1, 0, fx * (1.0 - fy)), (0, 1, (1.0 - fx) * fy), (1, 1, fx * fy)] {
            let px = self.at(left + dx, top + dy);
            // Weighed by alpha, so clear pixels do not darken what is mixed with them.
            for c in 0..3 {
                out[c] += px[c] * px[3] * weight;
            }
            out[3] += px[3] * weight;
        }
        if out[3] > 0.0 {
            for c in 0..3 {
                out[c] /= out[3];
            }
        }
        out
    }

    fn alpha(&self) -> Vec<f32> {
        self.pixels.iter().map(|p| p[3]).collect()
    }
}

/// Blurs one channel with three passes of a box, which is close to a Gaussian of that radius.
fn blur_channel(values: &[f32], width: usize, height: usize, radius: f32) -> Vec<f32> {
    if radius <= 0.05 || width == 0 || height == 0 {
        return values.to_vec();
    }
    // Three boxes of this half-width have the spread of a Gaussian with sigma = radius / 2.
    let sigma = radius / 2.0;
    let half = ((sigma * sigma * 4.0 + 1.0).sqrt() - 1.0) / 2.0;
    let (whole, part) = (half.floor() as i64, half.fract());
    let pass = |source: &[f32], length: usize, stride: usize, lines: usize, line_stride: usize| {
        let mut out = vec![0.0; source.len()];
        let weight = 1.0 / (2.0 * half + 1.0);
        for line in 0..lines {
            let base = line * line_stride;
            let get = |i: i64| if i < 0 || i >= length as i64 { 0.0 } else { source[base + i as usize * stride] };
            for i in 0..length as i64 {
                let mut sum = 0.0;
                for k in -whole..=whole {
                    sum += get(i + k);
                }
                sum += part * (get(i - whole - 1) + get(i + whole + 1));
                out[base + i as usize * stride] = sum * weight;
            }
        }
        out
    };
    let mut current = values.to_vec();
    for _ in 0..3 {
        current = pass(&current, width, 1, height, width);
        current = pass(&current, height, width, width, 1);
    }
    current
}

/// Blurs a layer, colors and transparency together.
fn blur_layer(layer: &Layer, radius: f32) -> Layer {
    let (w, h) = (layer.width, layer.height);
    let alpha = blur_channel(&layer.alpha(), w, h, radius);
    let mut out = layer.clone();
    for c in 0..3 {
        let premultiplied: Vec<f32> = layer.pixels.iter().map(|p| p[c] * p[3]).collect();
        let blurred = blur_channel(&premultiplied, w, h, radius);
        for (px, (value, a)) in out.pixels.iter_mut().zip(blurred.iter().zip(&alpha)) {
            px[c] = if *a > 1e-4 { (value / a).clamp(0.0, 1.0) } else { 0.0 };
        }
    }
    for (px, a) in out.pixels.iter_mut().zip(&alpha) {
        px[3] = a.clamp(0.0, 1.0);
    }
    out
}

/// Puts `top` over `bottom`.
fn over(top: [f32; 4], bottom: [f32; 4]) -> [f32; 4] {
    let alpha = top[3] + bottom[3] * (1.0 - top[3]);
    if alpha <= 0.0 {
        return [0.0; 4];
    }
    let mut out = [0.0, 0.0, 0.0, alpha];
    for c in 0..3 {
        out[c] = (top[c] * top[3] + bottom[c] * bottom[3] * (1.0 - top[3])) / alpha;
    }
    out
}

fn luminance(p: [f32; 4]) -> f32 {
    0.3 * p[0] + 0.59 * p[1] + 0.11 * p[2]
}

/// A number from -1 to 1 that is always the same for the same pixel and channel.
fn noise(x: usize, y: usize, channel: usize) -> f32 {
    let mut n = (x as u32).wrapping_mul(374_761_393).wrapping_add((y as u32).wrapping_mul(668_265_263)).wrapping_add((channel as u32).wrapping_mul(2_246_822_519));
    n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    ((n ^ (n >> 16)) & 0xffff) as f32 / 32767.5 - 1.0
}

/// The unit vector a light or a shadow goes along, for an angle in degrees, counterclockwise from the right.
fn direction(degrees: f32) -> (f32, f32) {
    let radians = degrees.to_radians();
    (radians.cos(), -radians.sin())
}

/// Sharpens by adding back the difference from a blurred copy.
fn unsharp(layer: &Layer, amount: f32, radius: f32, threshold: f32) -> Layer {
    let blurred = blur_layer(layer, radius);
    let mut out = layer.clone();
    for (px, soft) in out.pixels.iter_mut().zip(&blurred.pixels) {
        for c in 0..3 {
            let difference = px[c] - soft[c];
            if difference.abs() >= threshold {
                px[c] = (px[c] + difference * amount).clamp(0.0, 1.0);
            }
        }
    }
    out
}

/// How high the surface stands at each pixel of a bevel: 0 at the edge of the object, 1 on the flat top.
fn bevel_height(alpha: &[f32], width: usize, height: usize, bevel: f32, softness: f32, shape: usize, inside: bool) -> Vec<f32> {
    // A blurred edge rises across about the width of the blur; outside the object it is the mirror image.
    let ramp = blur_channel(alpha, width, height, bevel.max(0.5));
    let profile = |t: f32| -> f32 {
        let t = t.clamp(0.0, 1.0);
        match shape {
            0 => t,
            2 => t * t,
            3 => (t * 2.0).min(1.0) * 0.5 + if t > 0.5 { 0.5 } else { 0.0 },
            4 => ((t * 3.0).floor() / 2.0).min(1.0),
            5 => (t * std::f32::consts::PI).sin(),
            6 => 0.5 - 0.5 * (t * std::f32::consts::PI * 3.0).cos(),
            _ => t * t * (3.0 - 2.0 * t),
        }
    };
    let raw: Vec<f32> = ramp
        .iter()
        .map(|r| if inside { profile((r - 0.5) * 2.0) } else { profile(r * 2.0) })
        .collect();
    blur_channel(&raw, width, height, softness)
}

/// Lights a surface from an angle: more than 0 where it faces the light, less where it faces away.
fn lighting(heights: &[f32], width: usize, height: usize, angle: f32) -> Vec<f32> {
    let (lx, ly) = direction(angle);
    let at = |x: i64, y: i64| heights[y.clamp(0, height as i64 - 1) as usize * width + x.clamp(0, width as i64 - 1) as usize];
    let mut out = vec![0.0; heights.len()];
    for y in 0..height as i64 {
        for x in 0..width as i64 {
            let (dx, dy) = (at(x + 1, y) - at(x - 1, y), at(x, y + 1) - at(x, y - 1));
            // The slope rises away from the edge, so the side whose slope points at the light is lit.
            out[y as usize * width + x as usize] = -(dx * lx + dy * ly);
        }
    }
    out
}

/// Applies one filter to a layer.
pub fn apply(layer: &Layer, filter: &LiveFilter) -> Layer {
    let (w, h) = (layer.width, layer.height);
    let v = |index: usize| filter.value(index);
    let each = |change: &dyn Fn([f32; 4], usize, usize) -> [f32; 4]| {
        let mut out = layer.clone();
        for (index, px) in out.pixels.iter_mut().enumerate() {
            if px[3] > 0.0 {
                let changed = change(*px, index % w, index / w);
                *px = [changed[0].clamp(0.0, 1.0), changed[1].clamp(0.0, 1.0), changed[2].clamp(0.0, 1.0), changed[3].clamp(0.0, 1.0)];
            }
        }
        out
    };
    match filter.kind {
        Invert => each(&|p, _, _| [1.0 - p[0], 1.0 - p[1], 1.0 - p[2], p[3]]),
        BrightnessContrast => {
            let (brightness, contrast) = (v(0) / 100.0, v(1) / 100.0);
            // Contrast turns around mid gray: 100 is a step, -100 is flat gray.
            let slope = if contrast >= 0.0 { 1.0 / (1.0 - contrast).max(0.01) } else { 1.0 + contrast };
            each(&|p, _, _| {
                let f = |c: f32| (c - 0.5) * slope + 0.5 + brightness;
                [f(p[0]), f(p[1]), f(p[2]), p[3]]
            })
        }
        ColorFill => {
            let (opacity, color) = (v(0) / 100.0, filter.rgb());
            let mode = BlendMode::ALL[(v(1).round() as usize).min(BlendMode::ALL.len() - 1)];
            each(&|p, _, _| {
                let mixed = mode.mix([p[0], p[1], p[2]], color);
                [p[0] + (mixed[0] - p[0]) * opacity, p[1] + (mixed[1] - p[1]) * opacity, p[2] + (mixed[2] - p[2]) * opacity, p[3]]
            })
        }
        HueSaturation => {
            let (turn, saturation, lightness, colorize) = (v(0), v(1) / 100.0, v(2) / 100.0, v(3) >= 0.5);
            each(&|p, _, _| {
                let (mut hue, mut sat, mut light) = to_hsl([p[0], p[1], p[2]]);
                if colorize {
                    // One hue for everything, as strong as the saturation says.
                    hue = turn.rem_euclid(360.0);
                    sat = ((saturation + 1.0) / 2.0).clamp(0.0, 1.0);
                } else {
                    hue = (hue + turn).rem_euclid(360.0);
                    sat = if saturation >= 0.0 { sat + (1.0 - sat) * saturation } else { sat * (1.0 + saturation) };
                }
                light = if lightness >= 0.0 { light + (1.0 - light) * lightness } else { light * (1.0 + lightness) };
                let rgb = from_hsl(hue, sat, light);
                [rgb[0], rgb[1], rgb[2], p[3]]
            })
        }
        Levels => {
            let (in_black, gamma, in_white, out_black, out_white) = (v(0) / 255.0, v(1), v(2) / 255.0, v(3) / 255.0, v(4) / 255.0);
            let span = (in_white - in_black).max(1.0 / 255.0);
            each(&|p, _, _| {
                let f = |c: f32| out_black + ((c - in_black) / span).clamp(0.0, 1.0).powf(1.0 / gamma) * (out_white - out_black);
                [f(p[0]), f(p[1]), f(p[2]), p[3]]
            })
        }
        AutoLevels => {
            // Each channel is stretched so its darkest value is black and its lightest white.
            let mut range = [(1.0f32, 0.0f32); 3];
            for p in layer.pixels.iter().filter(|p| p[3] > 0.0) {
                for c in 0..3 {
                    range[c] = (range[c].0.min(p[c]), range[c].1.max(p[c]));
                }
            }
            each(&|p, _, _| {
                let f = |c: usize| if range[c].1 > range[c].0 { (p[c] - range[c].0) / (range[c].1 - range[c].0) } else { p[c] };
                [f(0), f(1), f(2), p[3]]
            })
        }
        ConvertToAlpha => each(&|p, _, _| [p[0], p[1], p[2], p[3] * (1.0 - luminance(p))]),
        AddNoise => {
            let (amount, colored) = (v(0) / 255.0, v(1) >= 0.5);
            each(&|p, x, y| {
                let gray = noise(x, y, 0);
                let n = |c: usize| if colored { noise(x, y, c + 1) } else { gray } * amount;
                [p[0] + n(0), p[1] + n(1), p[2] + n(2), p[3]]
            })
        }
        Blur => blur_layer(layer, 1.0),
        BlurMore => blur_layer(layer, 2.0),
        GaussianBlur => blur_layer(layer, v(0)),
        MotionBlur => {
            let ((dx, dy), distance) = (direction(v(0)), v(1));
            let steps = (distance.ceil() as usize).max(1);
            streak(layer, steps, &|x, y, k| (x + dx * (k - 0.5) * distance, y + dy * (k - 0.5) * distance))
        }
        RadialBlur | ZoomBlur => {
            let (amount, steps) = (v(0) / 100.0, (v(1).round() as usize).clamp(2, 100));
            let (cx, cy) = (bounds_center(layer).0, bounds_center(layer).1);
            if filter.kind == ZoomBlur {
                // Each pixel is smeared along the line to the middle of the object.
                streak(layer, steps, &|x, y, k| (cx + (x - cx) * (1.0 - amount * 0.5 * k), cy + (y - cy) * (1.0 - amount * 0.5 * k)))
            } else {
                // Each pixel is smeared along its circle around the middle.
                streak(layer, steps, &|x, y, k| {
                    let (sin, cos) = ((k - 0.5) * amount * 0.5).sin_cos();
                    (cx + (x - cx) * cos - (y - cy) * sin, cy + (x - cx) * sin + (y - cy) * cos)
                })
            }
        }
        Sharpen => unsharp(layer, 0.6, 1.0, 0.0),
        SharpenMore => unsharp(layer, 1.5, 1.0, 0.0),
        UnsharpMask => unsharp(layer, v(0) / 100.0, v(1), v(2) / 255.0),
        FindEdges => {
            let mut out = layer.clone();
            for y in 0..h as i64 {
                for x in 0..w as i64 {
                    let here = layer.at(x, y);
                    if here[3] <= 0.0 {
                        continue;
                    }
                    let mut px = [0.0, 0.0, 0.0, here[3]];
                    for c in 0..3 {
                        let g = |dx: i64, dy: i64| {
                            let p = layer.at(x + dx, y + dy);
                            if p[3] > 0.0 { p[c] } else { here[c] }
                        };
                        let gx = g(1, -1) + 2.0 * g(1, 0) + g(1, 1) - g(-1, -1) - 2.0 * g(-1, 0) - g(-1, 1);
                        let gy = g(-1, 1) + 2.0 * g(0, 1) + g(1, 1) - g(-1, -1) - 2.0 * g(0, -1) - g(1, -1);
                        px[c] = (gx * gx + gy * gy).sqrt().min(1.0);
                    }
                    out.pixels[y as usize * w + x as usize] = px;
                }
            }
            out
        }
        DropShadow | InnerShadow | Glow | InnerGlow | SolidShadow => shadow_or_glow(layer, filter),
        InnerBevel | OuterBevel | InsetEmboss | RaisedEmboss => bevel_or_emboss(layer, filter),
    }
}

/// Averages each pixel with others found by `along`, which is given a number from 0 to 1 for each step.
fn streak(layer: &Layer, steps: usize, along: &dyn Fn(f32, f32, f32) -> (f32, f32)) -> Layer {
    let mut out = layer.clone();
    for y in 0..layer.height {
        for x in 0..layer.width {
            let mut sum = [0.0f32; 4];
            for step in 0..=steps {
                let (sx, sy) = along(x as f32, y as f32, step as f32 / steps as f32);
                let p = layer.sample(sx, sy);
                for c in 0..3 {
                    sum[c] += p[c] * p[3];
                }
                sum[3] += p[3];
            }
            let alpha = sum[3] / (steps + 1) as f32;
            out.pixels[y * layer.width + x] =
                if sum[3] > 1e-4 { [sum[0] / sum[3], sum[1] / sum[3], sum[2] / sum[3], alpha.clamp(0.0, 1.0)] } else { [0.0; 4] };
        }
    }
    out
}

/// The middle of the part of the layer that has anything in it.
fn bounds_center(layer: &Layer) -> (f32, f32) {
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for (index, _) in layer.pixels.iter().enumerate().filter(|(_, p)| p[3] > 0.0) {
        let (x, y) = (index % layer.width, index / layer.width);
        (x0, y0, x1, y1) = (x0.min(x), y0.min(y), x1.max(x), y1.max(y));
    }
    if x0 == usize::MAX {
        return (layer.width as f32 / 2.0, layer.height as f32 / 2.0);
    }
    ((x0 + x1) as f32 / 2.0, (y0 + y1) as f32 / 2.0)
}

fn shadow_or_glow(layer: &Layer, filter: &LiveFilter) -> Layer {
    let (w, h) = (layer.width, layer.height);
    let v = |index: usize| filter.value(index);
    let color = filter.rgb();
    let alpha = layer.alpha();
    let shifted = |source: &[f32], dx: f32, dy: f32| -> Vec<f32> {
        let (ix, iy) = (dx.round() as i64, dy.round() as i64);
        (0..w * h)
            .map(|index| {
                let (x, y) = ((index % w) as i64 - ix, (index / w) as i64 - iy);
                if x < 0 || y < 0 || x >= w as i64 || y >= h as i64 { 0.0 } else { source[y as usize * w + x as usize] }
            })
            .collect()
    };
    let mut out = layer.clone();
    match filter.kind {
        DropShadow => {
            let ((dx, dy), distance, opacity, knock_out) = (direction(v(3)), v(0), v(1) / 100.0, v(4) >= 0.5);
            let shade = blur_channel(&shifted(&alpha, dx * distance, dy * distance), w, h, v(2));
            for (px, s) in out.pixels.iter_mut().zip(&shade) {
                let shadow = [color[0], color[1], color[2], (s * opacity).clamp(0.0, 1.0)];
                // Knocked out, the object leaves a hole in its own shadow.
                *px = if knock_out { [shadow[0], shadow[1], shadow[2], shadow[3] * (1.0 - px[3])] } else { over(*px, shadow) };
            }
        }
        SolidShadow => {
            let ((dx, dy), distance, solid) = (direction(v(0)), v(1), v(2) >= 0.5);
            // The object is stepped back along the angle, each copy behind the last.
            let steps = distance.ceil() as usize;
            for step in (1..=steps).rev() {
                let far = shifted(&alpha, dx * step as f32, dy * step as f32);
                let fade = if solid { 1.0 } else { 1.0 - step as f32 / (steps + 1) as f32 };
                for (px, a) in out.pixels.iter_mut().zip(&far) {
                    *px = over(*px, [color[0], color[1], color[2], a * fade]);
                }
            }
            // The copies went on top in turn, so the object itself is laid back over them.
            for (px, original) in out.pixels.iter_mut().zip(&layer.pixels) {
                *px = over(*original, *px);
            }
        }
        Glow => {
            let (width, opacity, softness, offset) = (v(0), v(1) / 100.0, v(2), v(3));
            // The outline is widened by the width, then pushed out by the offset and softened.
            let spread: Vec<f32> = blur_channel(&alpha, w, h, (width + offset).max(0.5) * 2.0).iter().map(|a| (a * 4.0).min(1.0)).collect();
            let halo = blur_channel(&spread, w, h, softness);
            let hole: Vec<f32> = if offset > 0.0 {
                blur_channel(&alpha, w, h, offset * 2.0).iter().map(|a| (a * 4.0).min(1.0)).collect()
            } else {
                vec![0.0; alpha.len()]
            };
            for ((px, s), inner) in out.pixels.iter_mut().zip(&halo).zip(&hole) {
                *px = over(*px, [color[0], color[1], color[2], (s * (1.0 - inner) * opacity).clamp(0.0, 1.0)]);
            }
        }
        InnerGlow => {
            let (width, opacity, softness) = (v(0), v(1) / 100.0, v(2));
            // What is not the object, grown inward and softened, shows only on the object.
            let outside: Vec<f32> = alpha.iter().map(|a| 1.0 - a).collect();
            // Half of the blurred outside reaches the edge, and it is gone about a width further in.
            let grown: Vec<f32> = blur_channel(&outside, w, h, width.max(0.5)).iter().map(|a| (a * 2.0).min(1.0)).collect();
            let edge = blur_channel(&grown, w, h, softness);
            for (px, e) in out.pixels.iter_mut().zip(&edge) {
                let alpha = px[3];
                let glowing = over([color[0], color[1], color[2], (e * opacity).clamp(0.0, 1.0)], [px[0], px[1], px[2], 1.0]);
                *px = [glowing[0], glowing[1], glowing[2], alpha];
            }
        }
        _ => {
            let ((dx, dy), distance, opacity, knock_out) = (direction(v(3)), v(0), v(1) / 100.0, v(4) >= 0.5);
            // The shadow falls on the object from outside: its shifted outside, softened.
            let outside: Vec<f32> = alpha.iter().map(|a| 1.0 - a).collect();
            let mut far = shifted(&outside, dx * distance, dy * distance);
            // What was shifted in from beyond the layer is outside too.
            for (index, value) in far.iter_mut().enumerate() {
                let (x, y) = ((index % w) as f32 - dx * distance, (index / w) as f32 - dy * distance);
                if x < 0.0 || y < 0.0 || x >= w as f32 || y >= h as f32 {
                    *value = 1.0;
                }
            }
            let shade = blur_channel(&far, w, h, v(2));
            for (px, s) in out.pixels.iter_mut().zip(&shade) {
                let alpha = px[3];
                let strength = (s * opacity).clamp(0.0, 1.0);
                *px = if knock_out {
                    [color[0], color[1], color[2], alpha * strength]
                } else {
                    let shaded = over([color[0], color[1], color[2], strength], [px[0], px[1], px[2], 1.0]);
                    [shaded[0], shaded[1], shaded[2], alpha]
                };
            }
        }
    }
    out
}

fn bevel_or_emboss(layer: &Layer, filter: &LiveFilter) -> Layer {
    let (w, h) = (layer.width, layer.height);
    let v = |index: usize| filter.value(index);
    let alpha = layer.alpha();
    let mut out = layer.clone();
    // Light where the surface faces the light, dark where it faces away, as strong as the contrast.
    let shade = |px: [f32; 4], light: f32, contrast: f32| -> [f32; 4] {
        let amount = (light * contrast).clamp(-1.0, 1.0);
        let toward = if amount >= 0.0 { 1.0 } else { 0.0 };
        let mix = amount.abs();
        [px[0] + (toward - px[0]) * mix, px[1] + (toward - px[1]) * mix, px[2] + (toward - px[2]) * mix, px[3]]
    };
    match filter.kind {
        InnerBevel => {
            let heights = bevel_height(&alpha, w, h, v(1), v(3), v(0).round() as usize, true);
            let light = lighting(&heights, w, h, v(4));
            let strength = v(2) / 100.0 * v(1).max(1.0) * 1.5;
            for (px, l) in out.pixels.iter_mut().zip(&light) {
                if px[3] > 0.0 {
                    *px = shade(*px, *l, strength);
                }
            }
        }
        OuterBevel => {
            let color = filter.rgb();
            let heights = bevel_height(&alpha, w, h, v(1), v(3), v(0).round() as usize, false);
            let light = lighting(&heights, w, h, v(4));
            let strength = v(2) / 100.0 * v(1).max(1.0) * 1.5;
            // The bevel is a rim of the chosen color around the object, lit like a slope.
            let rim: Vec<f32> = blur_channel(&alpha, w, h, v(1).max(0.5)).iter().map(|a| (a * 4.0).min(1.0)).collect();
            for ((px, l), r) in out.pixels.iter_mut().zip(&light).zip(&rim) {
                let edge = shade([color[0], color[1], color[2], *r], *l, strength);
                *px = over(*px, edge);
            }
        }
        _ => {
            let raised = filter.kind == RaisedEmboss;
            let (width, contrast, softness, angle, show) = (v(0), v(1) / 100.0, v(2), v(3), v(4) >= 0.5);
            // An emboss is the object's edge lit on one side and shaded on the other, on what is around it.
            let soft = blur_channel(&alpha, w, h, softness.max(0.3));
            let (dx, dy) = direction(angle);
            let reach = width.max(0.5);
            let sample = |x: f32, y: f32| -> f32 {
                let (ix, iy) = (x.round() as i64, y.round() as i64);
                if ix < 0 || iy < 0 || ix >= w as i64 || iy >= h as i64 { 0.0 } else { soft[iy as usize * w + ix as usize] }
            };
            for y in 0..h {
                for x in 0..w {
                    let (fx, fy) = (x as f32, y as f32);
                    let relief = sample(fx - dx * reach, fy - dy * reach) - sample(fx + dx * reach, fy + dy * reach);
                    let relief = if raised { relief } else { -relief };
                    let strength = (relief.abs() * contrast).clamp(0.0, 1.0);
                    let edge = if relief >= 0.0 { [1.0, 1.0, 1.0, strength] } else { [0.0, 0.0, 0.0, strength] };
                    let px = &mut out.pixels[y * w + x];
                    *px = if show { over(*px, edge) } else { edge };
                }
            }
        }
    }
    out
}

fn to_hsl(c: [f32; 3]) -> (f32, f32, f32) {
    let (max, min) = (c[0].max(c[1]).max(c[2]), c[0].min(c[1]).min(c[2]));
    let light = (max + min) / 2.0;
    let delta = max - min;
    if delta < 1e-6 {
        return (0.0, 0.0, light);
    }
    let sat = delta / (1.0 - (2.0 * light - 1.0).abs()).max(1e-6);
    let hue = if max == c[0] {
        60.0 * ((c[1] - c[2]) / delta).rem_euclid(6.0)
    } else if max == c[1] {
        60.0 * ((c[2] - c[0]) / delta + 2.0)
    } else {
        60.0 * ((c[0] - c[1]) / delta + 4.0)
    };
    (hue, sat.clamp(0.0, 1.0), light)
}

fn from_hsl(hue: f32, sat: f32, light: f32) -> [f32; 3] {
    let chroma = (1.0 - (2.0 * light - 1.0).abs()) * sat;
    let sector = hue.rem_euclid(360.0) / 60.0;
    let x = chroma * (1.0 - (sector.rem_euclid(2.0) - 1.0).abs());
    let (r, g, b) = match sector as u32 {
        0 => (chroma, x, 0.0),
        1 => (x, chroma, 0.0),
        2 => (0.0, chroma, x),
        3 => (0.0, x, chroma),
        4 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };
    let lift = light - chroma / 2.0;
    [r + lift, g + lift, b + lift]
}

/// How far, in pixels, a filter can paint beyond the object it is on.
pub fn reach(filter: &LiveFilter) -> f32 {
    let v = |index: usize| filter.value(index);
    match filter.kind {
        DropShadow => v(0) + v(2) * 2.0,
        SolidShadow => v(1),
        Glow => (v(0) + v(3)) * 3.0 + v(2) * 2.0,
        OuterBevel => v(1) * 3.0 + v(3) * 2.0,
        InsetEmboss | RaisedEmboss => v(0) + v(2) * 2.0,
        GaussianBlur => v(0) * 2.0,
        Blur | BlurMore => 4.0,
        MotionBlur => v(1),
        RadialBlur | ZoomBlur => 8.0,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A layer with a square of one color in its middle.
    fn square(size: usize, margin: usize, color: [f32; 3]) -> Layer {
        let mut pixels = vec![[0.0; 4]; size * size];
        for y in margin..size - margin {
            for x in margin..size - margin {
                pixels[y * size + x] = [color[0], color[1], color[2], 1.0];
            }
        }
        Layer { width: size, height: size, pixels }
    }

    fn with(kind: FilterKind, values: &[f32]) -> LiveFilter {
        let mut filter = LiveFilter::new(kind);
        for (index, value) in values.iter().enumerate() {
            filter.set_value(index, *value);
        }
        filter
    }

    fn near(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.02
    }

    #[test]
    fn every_filter_keeps_its_pixels_in_range() {
        let layer = square(40, 10, [0.8, 0.4, 0.1]);
        for (_, kinds) in MENU {
            for kind in kinds.iter() {
                let out = apply(&layer, &LiveFilter::new(*kind));
                assert_eq!(out.pixels.len(), layer.pixels.len(), "{kind:?}");
                for p in &out.pixels {
                    assert!(p.iter().all(|c| c.is_finite() && (0.0..=1.0).contains(c)), "{kind:?} gave {p:?}");
                }
                assert!(!kind.label().is_empty());
                // Every setting starts inside its own range.
                for setting in kind.settings() {
                    assert!(setting.min <= setting.start && setting.start <= setting.max, "{kind:?} {}", setting.name);
                }
            }
        }
        assert_eq!(MENU.iter().map(|(_, kinds)| kinds.len()).sum::<usize>(), 27);
    }

    #[test]
    fn the_color_filters_do_what_their_names_say() {
        let layer = square(8, 2, [0.8, 0.4, 0.1]);
        let middle = |l: &Layer| l.pixels[4 * 8 + 4];
        let inverted = middle(&apply(&layer, &LiveFilter::new(Invert)));
        assert!(near(inverted[0], 0.2) && near(inverted[1], 0.6) && near(inverted[2], 0.9) && inverted[3] == 1.0);
        let brighter = middle(&apply(&layer, &with(BrightnessContrast, &[20.0, 0.0])));
        assert!(near(brighter[0], 1.0) && near(brighter[1], 0.6) && near(brighter[2], 0.3));
        // Full contrast pushes each channel to black or white, around mid gray.
        let stark = middle(&apply(&layer, &with(BrightnessContrast, &[0.0, 100.0])));
        assert_eq!((stark[0], stark[1], stark[2]), (1.0, 0.0, 0.0));
        // A color fill at full opacity is that color, and at half is half way.
        assert!(near(middle(&apply(&layer, &LiveFilter::new(ColorFill)))[1], 0.0));
        assert!(near(middle(&apply(&layer, &with(ColorFill, &[50.0])))[1], 0.2));
        // No saturation is gray, and turning the hue half way round turns orange to blue.
        let gray = middle(&apply(&layer, &with(HueSaturation, &[0.0, -100.0])));
        assert!(near(gray[0], gray[1]) && near(gray[1], gray[2]));
        let turned = middle(&apply(&layer, &with(HueSaturation, &[180.0])));
        assert!(turned[2] > turned[0] && turned[2] > turned[1], "{turned:?}");
        // Levels that take mid gray as white clip the red and double the blue.
        let levelled = middle(&apply(&layer, &with(Levels, &[0.0, 1.0, 128.0])));
        assert!(near(levelled[0], 1.0) && near(levelled[2], 0.2), "{levelled:?}");
        // The clear pixels around the square stay clear.
        assert_eq!(apply(&layer, &LiveFilter::new(Invert)).pixels[0], [0.0; 4]);
        // Lighter is clearer when a picture is turned into its own transparency.
        let white = square(8, 2, [1.0, 1.0, 1.0]);
        assert!(middle(&apply(&white, &LiveFilter::new(ConvertToAlpha)))[3] < 0.01);
    }

    #[test]
    fn a_blur_spreads_the_object_without_losing_any_of_it() {
        let layer = square(60, 20, [1.0, 0.0, 0.0]);
        let blurred = apply(&layer, &with(GaussianBlur, &[6.0]));
        let total = |l: &Layer| l.pixels.iter().map(|p| p[3]).sum::<f32>();
        assert!((total(&blurred) - total(&layer)).abs() < total(&layer) * 0.01);
        assert!(blurred.pixels[20 * 60 + 17][3] > 0.0, "it reaches past the old edge");
        assert!(blurred.pixels[30 * 60 + 30][3] > 0.99, "the middle is still solid");
        assert!(near(blurred.pixels[20 * 60 + 17][0], 1.0), "and is still red where it spread");
    }

    #[test]
    fn a_drop_shadow_falls_where_the_angle_points() {
        let layer = square(60, 20, [1.0, 0.0, 0.0]);
        // The default angle, 315 degrees, throws the shadow down and to the right.
        let shadowed = apply(&layer, &with(DropShadow, &[8.0, 100.0, 0.0]));
        let at = |l: &Layer, x: usize, y: usize| l.pixels[y * 60 + x];
        assert!(at(&shadowed, 43, 43)[3] > 0.9 && at(&shadowed, 43, 43)[0] < 0.01, "black shadow below right");
        assert!(at(&shadowed, 16, 16)[3] < 0.01, "nothing above left");
        assert_eq!(at(&shadowed, 30, 30), [1.0, 0.0, 0.0, 1.0], "the object is on top of its shadow");
        // Knocked out, only the shadow is left, with a hole where the object was.
        let hollow = apply(&layer, &with(DropShadow, &[8.0, 100.0, 0.0, 315.0, 1.0]));
        assert!(at(&hollow, 30, 30)[3] < 0.01 && at(&hollow, 43, 43)[3] > 0.9);
        // An inner shadow darkens the object on the side the shadow comes from and nothing outside it.
        let inner = apply(&layer, &with(InnerShadow, &[6.0, 100.0, 0.0]));
        assert!(at(&inner, 22, 22)[0] < 0.1 && at(&inner, 36, 36)[0] > 0.9, "{:?} {:?}", at(&inner, 22, 22), at(&inner, 36, 36));
        assert!(at(&inner, 43, 43)[3] < 0.01);
    }

    #[test]
    fn a_glow_surrounds_the_object_and_an_inner_glow_stays_inside() {
        let layer = square(60, 20, [0.0, 0.0, 1.0]);
        let at = |l: &Layer, x: usize, y: usize| l.pixels[y * 60 + x];
        let glowing = apply(&layer, &with(Glow, &[4.0, 100.0, 2.0]));
        for (x, y) in [(17, 30), (42, 30), (30, 17), (30, 42)] {
            assert!(at(&glowing, x, y)[3] > 0.2 && at(&glowing, x, y)[0] > 0.9, "red glow at {x},{y}: {:?}", at(&glowing, x, y));
        }
        assert_eq!(at(&glowing, 30, 30), [0.0, 0.0, 1.0, 1.0]);
        assert!(at(&glowing, 2, 2)[3] < 0.01);
        let inner = apply(&layer, &with(InnerGlow, &[4.0, 100.0, 2.0]));
        assert!(at(&inner, 21, 30)[0] > 0.3, "red just inside the edge");
        assert!(at(&inner, 30, 30)[0] < 0.05, "none in the middle");
        assert!(at(&inner, 17, 30)[3] < 0.01, "and none outside");
    }

    #[test]
    fn a_bevel_lights_the_side_that_faces_the_light() {
        let layer = square(60, 15, [0.5, 0.5, 0.5]);
        let at = |l: &Layer, x: usize, y: usize| l.pixels[y * 60 + x][0];
        // Light from the upper left, at 135 degrees: the left and top edges face it.
        let bevelled = apply(&layer, &with(InnerBevel, &[1.0, 6.0, 100.0, 1.0, 135.0]));
        assert!(at(&bevelled, 17, 30) > 0.6, "the left edge is lit: {}", at(&bevelled, 17, 30));
        assert!(at(&bevelled, 30, 17) > 0.6, "the top edge is lit: {}", at(&bevelled, 30, 17));
        assert!(at(&bevelled, 42, 30) < 0.4, "the right edge is in shade: {}", at(&bevelled, 42, 30));
        assert!(at(&bevelled, 30, 42) < 0.4, "the bottom edge is in shade: {}", at(&bevelled, 30, 42));
        assert!(near(at(&bevelled, 30, 30), 0.5), "the flat top is untouched");
        assert_eq!(bevelled.pixels[0], [0.0; 4]);
    }

    #[test]
    fn settings_stay_in_range_and_old_files_read() {
        let mut filter = LiveFilter::new(GaussianBlur);
        filter.set_value(0, 9999.0);
        assert_eq!(filter.value(0), 250.0);
        filter.set_value(0, f32::NAN);
        assert_eq!(filter.value(0), 4.0);
        // A filter saved with fewer values than it has now reads the rest at their start.
        let short: LiveFilter = serde_json::from_str(r#"{"kind":"DropShadow","values":[12.0]}"#).unwrap();
        assert_eq!((short.value(0), short.value(1), short.value(3)), (12.0, 65.0, 315.0));
        assert!(short.enabled);
        assert!(reach(&short) > 12.0);
    }
}
