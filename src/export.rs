//! Rasterizes a page to PNG, and measures text with the same font used for export.

use std::path::Path;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont};
use anyhow::{anyhow, Context, Result};
use tiny_skia::{
    BlendMode, ColorU8, FillRule, FilterQuality, IntRect, LineCap, LineJoin, Paint, PathBuilder, Pixmap, PixmapPaint,
    PremultipliedColorU8, Stroke as SkStroke, Transform,
};

use crate::i18n::{t, tf};
use crate::bitmap::Bitmap;
use crate::model::{Color, MaskKind, Object, ObjectMask, Page, Rect, Shape, TextAlign, TextStyle, Vec2, WebKind};

pub const LINE_HEIGHT: f32 = 1.2;

fn font() -> Option<&'static FontVec> {
    crate::fonts::outlines()
}

/// The face a text is set in.
fn styled_font(style: &TextStyle) -> Option<&'static FontVec> {
    crate::fonts::face(&style.font, style.bold, style.italic).or_else(font)
}

/// How wide one line of text is.
pub fn line_width(line: &str, size: f32, style: &TextStyle) -> f32 {
    match styled_font(style) {
        Some(f) => {
            let scaled = f.as_scaled(em_scale(f, size));
            line.chars().map(|c| scaled.h_advance(scaled.glyph_id(c))).sum::<f32>()
        }
        None => line.chars().count() as f32 * size * 0.55,
    }
}

/// How far from the left of its box a line of text starts, for an alignment.
pub fn line_start(box_width: f32, line_width: f32, align: TextAlign) -> f32 {
    match align {
        TextAlign::Left => 0.0,
        TextAlign::Center => ((box_width - line_width) / 2.0).max(0.0),
        TextAlign::Right => (box_width - line_width).max(0.0),
    }
}

/// Returns the width and height of multi-line text at `size` pixels, in a style.
pub fn measure_styled(content: &str, size: f32, style: &TextStyle) -> (f32, f32) {
    let line_count = content.split('\n').count() as f32;
    let width = content.split('\n').map(|line| line_width(line, size, style)).fold(0.0, f32::max);
    (width.max(size * 0.5), line_count * size * LINE_HEIGHT)
}

/// The box of the line under underlined text: left, top, width and height, for a line of text starting at `left` on a baseline.
fn underline_box(left: f32, baseline: f32, width: f32, size: f32) -> (f32, f32, f32, f32) {
    let thick = (size / 14.0).max(1.0);
    (left, baseline + thick, width, thick)
}

/// Converts an em size, as used by the canvas, to the font's pixel-height scale.
fn em_scale(font: &FontVec, size: f32) -> PxScale {
    match font.units_per_em() {
        Some(units) => PxScale::from(size * font.height_unscaled() / units),
        None => PxScale::from(size),
    }
}

/// Returns the width and height of multi-line text at `size` pixels.
pub fn measure_text(content: &str, size: f32) -> (f32, f32) {
    let line_count = content.split('\n').count() as f32;
    let width = content
        .split('\n')
        .map(|line| match font() {
            Some(f) => {
                let scaled = f.as_scaled(em_scale(f, size));
                line.chars().map(|c| scaled.h_advance(scaled.glyph_id(c))).sum::<f32>()
            }
            None => line.chars().count() as f32 * size * 0.55,
        })
        .fold(0.0, f32::max);
    (width.max(size * 0.5), line_count * size * LINE_HEIGHT)
}

fn sk_paint(color: Color, opacity: f32) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.anti_alias = true;
    paint.set_color_rgba8(color.r, color.g, color.b, (color.a as f32 * opacity) as u8);
    paint
}

/// The outline of every character that draws something, as closed rings in document
/// space. Text is laid out exactly as [`draw_text`] paints it.
pub fn glyph_outlines(content: &str, size: f32, origin: Vec2, style: &TextStyle) -> Vec<Vec<Vec<Vec2>>> {
    use ab_glyph::OutlineCurve;
    let Some(font) = styled_font(style) else { return Vec::new() };
    let box_width = measure_styled(content, size, style).0;
    let scale = em_scale(font, size);
    let scaled = font.as_scaled(scale);
    let line_height = size * LINE_HEIGHT;
    let lead = (line_height - (scaled.ascent() - scaled.descent())) / 2.0;
    let far = |a: Vec2, b: Vec2| (a.x - b.x).abs() + (a.y - b.y).abs() > 0.01;
    let mut glyphs = Vec::new();
    for (row, line) in content.split('\n').enumerate() {
        let width = line_width(line, size, style);
        let mut caret = origin.x + line_start(box_width, width, style.align);
        let baseline = origin.y + row as f32 * line_height + lead + scaled.ascent();
        if style.underline && !line.is_empty() {
            let (x, y, w, h) = underline_box(caret, baseline, width, size);
            glyphs.push(vec![vec![Vec2 { x, y }, Vec2 { x: x + w, y }, Vec2 { x: x + w, y: y + h }, Vec2 { x, y: y + h }]]);
        }
        for ch in line.chars() {
            let id = scaled.glyph_id(ch);
            if let Some(outline) = font.outline(id) {
                // Font outlines have y pointing up.
                let place = |p: &ab_glyph::Point| Vec2 { x: caret + p.x * scaled.h_scale_factor(), y: baseline - p.y * scaled.v_scale_factor() };
                let mut rings: Vec<Vec<Vec2>> = Vec::new();
                for curve in &outline.curves {
                    let controls: Vec<Vec2> = match curve {
                        OutlineCurve::Line(a, b) => vec![place(a), place(b)],
                        OutlineCurve::Quad(a, b, c) => vec![place(a), place(b), place(c)],
                        OutlineCurve::Cubic(a, b, c, d) => vec![place(a), place(b), place(c), place(d)],
                    };
                    let (start, end) = (controls[0], controls[controls.len() - 1]);
                    // A curve that does not start where the last one ended opens a new ring.
                    if rings.last().and_then(|ring| ring.last()).is_none_or(|last| far(*last, start)) {
                        rings.push(vec![start]);
                    }
                    let Some(ring) = rings.last_mut() else { continue };
                    let length: f32 = controls.windows(2).map(|w| ((w[0].x - w[1].x).powi(2) + (w[0].y - w[1].y).powi(2)).sqrt()).sum();
                    let steps = if controls.len() == 2 { 1 } else { (length.ceil() as usize).clamp(2, 16) };
                    for step in 1..steps {
                        let t = step as f32 / steps as f32;
                        // De Casteljau: works for a quadratic and a cubic alike.
                        let mut level = controls.clone();
                        while level.len() > 1 {
                            level = level.windows(2).map(|w| Vec2 { x: w[0].x + (w[1].x - w[0].x) * t, y: w[0].y + (w[1].y - w[0].y) * t }).collect();
                        }
                        ring.push(level[0]);
                    }
                    ring.push(end);
                }
                for ring in &mut rings {
                    if ring.len() > 1 && !far(ring[0], ring[ring.len() - 1]) {
                        ring.pop();
                    }
                }
                rings.retain(|ring| ring.len() >= 3);
                if !rings.is_empty() {
                    glyphs.push(rings);
                }
            }
            caret += scaled.h_advance(id);
        }
    }
    glyphs
}

fn sk_path(points: &[Vec2], closed: bool) -> Option<tiny_skia::Path> {
    let (first, rest) = points.split_first()?;
    let mut builder = PathBuilder::new();
    builder.move_to(first.x, first.y);
    for p in rest {
        builder.line_to(p.x, p.y);
    }
    if closed {
        builder.close();
    }
    builder.finish()
}

fn blend_pixel(pixmap: &mut Pixmap, x: i32, y: i32, color: Color, coverage: f32) {
    let (w, h) = (pixmap.width() as i32, pixmap.height() as i32);
    if x < 0 || y < 0 || x >= w || y >= h {
        return;
    }
    let alpha = (color.a as f32 / 255.0 * coverage).clamp(0.0, 1.0);
    let index = (y * w + x) as usize;
    let dst = pixmap.pixels()[index];
    let over = |src: u8, dst: u8| (src as f32 * alpha + dst as f32 * (1.0 - alpha)).round() as u8;
    let a = (255.0 * alpha + dst.alpha() as f32 * (1.0 - alpha)).round() as u8;
    let min = |c: u8| c.min(a);
    if let Some(px) = PremultipliedColorU8::from_rgba(
        min(over(color.r, dst.red())),
        min(over(color.g, dst.green())),
        min(over(color.b, dst.blue())),
        a,
    ) {
        pixmap.pixels_mut()[index] = px;
    }
}

fn draw_text(pixmap: &mut Pixmap, object: &Object, content: &str, size: f32, style: &TextStyle) {
    let Some(font) = styled_font(style) else { return };
    let Some(fill) = object.fill else { return };
    let color = Color { a: (fill.a as f32 * object.opacity) as u8, ..fill };
    let scale = em_scale(font, size);
    let scaled = font.as_scaled(scale);
    let line_height = size * LINE_HEIGHT;
    let lead = (line_height - (scaled.ascent() - scaled.descent())) / 2.0;
    for (row, line) in content.split('\n').enumerate() {
        let width = line_width(line, size, style);
        let mut caret = object.bounds.x + line_start(object.bounds.w, width, style.align);
        let baseline = object.bounds.y + row as f32 * line_height + lead + scaled.ascent();
        if style.underline && !line.is_empty() {
            let (x, y, w, h) = underline_box(caret, baseline, width, size);
            for py in y.round() as i32..(y + h).round().max(y.round() + 1.0) as i32 {
                for px in x.round() as i32..(x + w).round() as i32 {
                    blend_pixel(pixmap, px, py, color, 1.0);
                }
            }
        }
        for ch in line.chars() {
            let id = scaled.glyph_id(ch);
            let glyph = id.with_scale_and_position(scale, ab_glyph::point(caret, baseline));
            if let Some(outlined) = font.outline_glyph(glyph) {
                let b = outlined.px_bounds();
                outlined.draw(|gx, gy, coverage| {
                    blend_pixel(pixmap, b.min.x as i32 + gx as i32, b.min.y as i32 + gy as i32, color, coverage);
                });
            }
            caret += scaled.h_advance(id);
        }
    }
}

fn draw_bitmap(target: &mut Pixmap, object: &Object, bitmap: &Bitmap) {
    let Some(mut source) = Pixmap::new(bitmap.width(), bitmap.height()) else { return };
    for (dst, px) in source.pixels_mut().iter_mut().zip(bitmap.rgba().chunks_exact(4)) {
        *dst = ColorU8::from_rgba(px[0], px[1], px[2], px[3]).premultiply();
    }
    let b = object.bounds;
    let (sx, sy) = (b.w / bitmap.width() as f32, b.h / bitmap.height() as f32);
    let quality = if sx == 1.0 && sy == 1.0 && b.x.fract() == 0.0 && b.y.fract() == 0.0 {
        FilterQuality::Nearest
    } else {
        FilterQuality::Bilinear
    };
    let paint = PixmapPaint { opacity: object.opacity, blend_mode: BlendMode::SourceOver, quality };
    target.draw_pixmap(0, 0, source.as_ref(), &paint, Transform::from_row(sx, 0.0, 0.0, sy, b.x, b.y), None);
}

/// How much of each pixel a mask lets through, from 0 to 1, for a page of the given size.
fn mask_coverage(mask: &ObjectMask, width: u32, height: u32) -> Option<Vec<f32>> {
    let mut drawn = Pixmap::new(width, height)?;
    let mut shape = mask.object.clone();
    shape.visible = true;
    shape.mask = None;
    shape.blend = crate::blend::BlendMode::Normal;
    if mask.kind == MaskKind::PathOutline {
        // Only the outline counts: whatever the mask is filled or stroked with, its inside shows.
        // A mask made of several objects shows what is inside any of them.
        for part in shape.leaves() {
            if let Some(path) = sk_path(&part.outline(), true) {
                drawn.fill_path(&path, &sk_paint(Color::WHITE, 1.0), FillRule::EvenOdd, Transform::identity(), None);
            }
        }
        return Some(drawn.pixels().iter().map(|px| px.alpha() as f32 / 255.0).collect());
    }
    shape.opacity = 1.0;
    draw_object(&mut drawn, &shape);
    Some(
        drawn
            .pixels()
            .iter()
            .map(|px| match mask.kind {
                MaskKind::Alpha => px.alpha() as f32 / 255.0,
                // Premultiplied channels already carry the alpha, so a clear pixel is black.
                _ => (0.3 * px.red() as f32 + 0.59 * px.green() as f32 + 0.11 * px.blue() as f32) / 255.0,
            })
            .collect(),
    )
}

/// One object drawn on nothing, as a bitmap, with where it sits in the document.
/// Draws several objects together, the first lowest, onto nothing: the image, and where on the page it lies.
pub fn render_objects(objects: &[Object]) -> Option<(Bitmap, Rect)> {
    let area = objects
        .iter()
        .map(|object| {
            let reach = (object.stroke.map_or(0.0, |s| s.width / 2.0) + object.filters.iter().filter(|f| f.enabled).map(crate::effects::reach).sum::<f32>()).ceil();
            let b = object.bounds;
            Rect::from_corners(Vec2 { x: (b.x - reach).floor(), y: (b.y - reach).floor() }, Vec2 { x: (b.right() + reach).ceil(), y: (b.bottom() + reach).ceil() })
        })
        .reduce(Rect::union)?;
    let (width, height) = (area.w.max(1.0) as u32, area.h.max(1.0) as u32);
    let mut pixmap = Pixmap::new(width, height)?;
    for object in objects {
        let mut moved = object.clone();
        moved.translate(-area.x, -area.y);
        draw_object(&mut pixmap, &moved);
    }
    let rgba: Vec<u8> = pixmap.pixels().iter().flat_map(|px| {
        let c = px.demultiply();
        [c.red(), c.green(), c.blue(), c.alpha()]
    }).collect();
    Some((Bitmap::from_rgba(width, height, rgba)?, Rect { x: area.x, y: area.y, w: width as f32, h: height as f32 }))
}

pub fn render_object(object: &Object) -> Option<(Bitmap, Rect)> {
    let b = object.bounds;
    let reach = (object.stroke.map_or(0.0, |s| s.width / 2.0) + object.filters.iter().filter(|f| f.enabled).map(crate::effects::reach).sum::<f32>()).ceil();
    let (left, top) = ((b.x - reach).floor(), (b.y - reach).floor());
    let (width, height) = (((b.right() + reach).ceil() - left).max(1.0) as u32, ((b.bottom() + reach).ceil() - top).max(1.0) as u32);
    let mut pixmap = Pixmap::new(width, height)?;
    let mut moved = object.clone();
    moved.translate(-left, -top);
    moved.visible = true;
    draw_object(&mut pixmap, &moved);
    let rgba: Vec<u8> = pixmap.pixels().iter().flat_map(|px| {
        let c = px.demultiply();
        [c.red(), c.green(), c.blue(), c.alpha()]
    }).collect();
    Some((Bitmap::from_rgba(width, height, rgba)?, Rect { x: left, y: top, w: width as f32, h: height as f32 }))
}

fn draw_object(pixmap: &mut Pixmap, object: &Object) {
    // The stand-in for a mask being edited is not artwork.
    if !object.visible || object.is_web() || object.mask_of.is_some() {
        return;
    }
    // An object with a blend mode is drawn on a layer of its own, which is then mixed into the page.
    if !object.blend.is_normal() {
        let Some(mut layer) = Pixmap::new(pixmap.width(), pixmap.height()) else { return };
        let mut plain = object.clone();
        plain.blend = crate::blend::BlendMode::Normal;
        draw_object(&mut layer, &plain);
        crate::blend::composite(pixmap, &layer, object.blend);
        return;
    }
    // Live filters work on the object drawn alone, in the part of the page it can reach.
    if object.filters.iter().any(|filter| filter.enabled) {
        let reach: f32 = object.filters.iter().filter(|f| f.enabled).map(crate::effects::reach).sum::<f32>() + object.stroke.map_or(0.0, |s| s.width) + 2.0;
        let (page_w, page_h) = (pixmap.width() as f32, pixmap.height() as f32);
        let b = object.bounds;
        let (x0, y0) = ((b.x - reach).floor().clamp(0.0, page_w), (b.y - reach).floor().clamp(0.0, page_h));
        let (x1, y1) = ((b.right() + reach).ceil().clamp(0.0, page_w), (b.bottom() + reach).ceil().clamp(0.0, page_h));
        let (width, height) = ((x1 - x0) as u32, (y1 - y0) as u32);
        let Some(mut drawn) = Pixmap::new(width.max(1), height.max(1)).filter(|_| width > 0 && height > 0) else { return };
        let mut bare = object.clone();
        bare.filters.clear();
        bare.translate(-x0, -y0);
        draw_object(&mut drawn, &bare);
        let mut layer = crate::effects::Layer {
            width: width as usize,
            height: height as usize,
            pixels: drawn.pixels().iter().map(|px| {
                let c = px.demultiply();
                [c.red() as f32 / 255.0, c.green() as f32 / 255.0, c.blue() as f32 / 255.0, c.alpha() as f32 / 255.0]
            }).collect(),
        };
        // The shadow and the glow of Photoshop Live Effects are mixed into the page by their own
        // blend modes, under the object, so they are kept on layers of their own. A filter that
        // comes after works on the object and on each of those layers alike.
        use crate::effects::FilterKind;
        let mut under: Vec<(crate::effects::Layer, crate::blend::BlendMode)> = Vec::new();
        for filter in object.filters.iter().filter(|f| f.enabled) {
            if filter.kind == FilterKind::PhotoshopEffects && under.is_empty() {
                (layer, under) = crate::pseffects::apply_apart(&layer, filter);
                continue;
            }
            layer = crate::effects::apply(&layer, filter);
            // A filter that draws around the object draws around the object only, not around its shadow as well.
            let surrounds = matches!(
                filter.kind,
                FilterKind::DropShadow | FilterKind::SolidShadow | FilterKind::Glow | FilterKind::OuterBevel | FilterKind::RaisedEmboss | FilterKind::InsetEmboss | FilterKind::PhotoshopEffects
            );
            if !surrounds {
                for (part, _) in &mut under {
                    *part = crate::effects::apply(part, filter);
                }
            }
        }
        let fill = |target: &mut Pixmap, layer: &crate::effects::Layer| {
            for (px, value) in target.pixels_mut().iter_mut().zip(&layer.pixels) {
                let alpha = value[3].clamp(0.0, 1.0);
                let a = (alpha * 255.0).round() as u8;
                let channel = |v: f32| ((v.clamp(0.0, 1.0) * alpha * 255.0).round() as u8).min(a);
                *px = PremultipliedColorU8::from_rgba(channel(value[0]), channel(value[1]), channel(value[2]), a).unwrap_or(PremultipliedColorU8::TRANSPARENT);
            }
        };
        for (part, mode) in &under {
            fill(&mut drawn, part);
            if mode.is_normal() {
                pixmap.draw_pixmap(x0 as i32, y0 as i32, drawn.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
            } else if let Some(mut whole) = Pixmap::new(pixmap.width(), pixmap.height()) {
                whole.draw_pixmap(x0 as i32, y0 as i32, drawn.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
                crate::blend::composite(pixmap, &whole, *mode);
            }
        }
        fill(&mut drawn, &layer);
        pixmap.draw_pixmap(x0 as i32, y0 as i32, drawn.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        return;
    }
    // A masked object is drawn on a layer of its own, and the mask takes away what it hides.
    if let Some(mask) = object.mask.as_ref().filter(|mask| mask.enabled) {
        let (Some(mut layer), Some(coverage)) = (Pixmap::new(pixmap.width(), pixmap.height()), mask_coverage(mask, pixmap.width(), pixmap.height())) else {
            return;
        };
        let mut bare = object.clone();
        bare.mask = None;
        draw_object(&mut layer, &bare);
        for (px, shown) in layer.pixels_mut().iter_mut().zip(&coverage) {
            let keep = |c: u8| (c as f32 * shown).round() as u8;
            *px = PremultipliedColorU8::from_rgba(keep(px.red()), keep(px.green()), keep(px.blue()), keep(px.alpha())).unwrap_or(PremultipliedColorU8::TRANSPARENT);
        }
        // A path mask can show itself as well: its fill behind what it masks, its stroke in front.
        let shows_itself = mask.show_fill_stroke && mask.kind == MaskKind::PathOutline;
        let part_of_mask = |fill: bool| {
            let mut part = mask.object.clone();
            if fill { part.stroke = None } else { part.fill = None }
            part.mask = None;
            part
        };
        if shows_itself {
            draw_object(pixmap, &part_of_mask(true));
        }
        pixmap.draw_pixmap(0, 0, layer.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
        if shows_itself {
            draw_object(pixmap, &part_of_mask(false));
        }
        return;
    }
    if let Some(paths) = object.placed_text_paths() {
        return draw_object(pixmap, &paths);
    }
    if object.is_instance() {
        for part in object.instance_parts() {
            draw_object(pixmap, &part);
        }
        return;
    }
    if let Shape::Text { content, size, style } = &object.shape {
        draw_text(pixmap, object, content, *size, style);
        return;
    }
    if let Shape::Image { bitmap } = &object.shape {
        draw_bitmap(pixmap, object, bitmap);
        return;
    }
    if let (Some(style), Some(_)) = (&object.fill_style, object.fill) {
        draw_styled_fill(pixmap, object, style);
    } else if let Some(fill) = object.fill {
        if let Some(path) = sk_path(&object.outline(), object.is_closed()) {
            pixmap.fill_path(&path, &sk_paint(fill, object.opacity), FillRule::EvenOdd, Transform::identity(), None);
        }
    }
    if let Some(stroke) = object.stroke {
        let style = SkStroke {
            width: stroke.width.max(0.1),
            line_cap: LineCap::Round,
            line_join: LineJoin::Round,
            ..SkStroke::default()
        };
        // A stroke of uneven weight is drawn as a ribbon: a piece between each two points, and a round joint at each.
        if let Some(weights) = object.stroke_weights() {
            let count = object.points.len();
            let pieces = if object.is_closed() { count } else { count.saturating_sub(1) };
            let half = |index: usize| (stroke.width * weights[index] / 2.0).max(0.0);
            // Drawn on its own first, so where pieces overlap a see-through stroke is not painted twice.
            let Some(mut layer) = Pixmap::new(pixmap.width(), pixmap.height()) else { return };
            let ink = sk_paint(stroke.color, 1.0);
            for (index, point) in object.points.iter().enumerate() {
                if let Some(joint) = PathBuilder::from_circle(point.x, point.y, half(index)).filter(|_| half(index) > 0.05) {
                    layer.fill_path(&joint, &ink, FillRule::Winding, Transform::identity(), None);
                }
            }
            for index in 0..pieces {
                let (a, b) = (object.points[index], object.points[(index + 1) % count]);
                let (dx, dy) = (b.x - a.x, b.y - a.y);
                let length = dx.hypot(dy);
                if length < 0.001 {
                    continue;
                }
                // Across the piece, as far as half the stroke at each end.
                let (nx, ny, wa, wb) = (-dy / length, dx / length, half(index), half((index + 1) % count));
                let mut piece = PathBuilder::new();
                piece.move_to(a.x + nx * wa, a.y + ny * wa);
                piece.line_to(b.x + nx * wb, b.y + ny * wb);
                piece.line_to(b.x - nx * wb, b.y - ny * wb);
                piece.line_to(a.x - nx * wa, a.y - ny * wa);
                piece.close();
                if let Some(piece) = piece.finish() {
                    layer.fill_path(&piece, &ink, FillRule::Winding, Transform::identity(), None);
                }
            }
            let paint = PixmapPaint { opacity: object.opacity, ..PixmapPaint::default() };
            pixmap.draw_pixmap(0, 0, layer.as_ref(), &paint, Transform::identity(), None);
            return;
        }
        for (run, closed) in object.stroke_runs() {
            if let Some(path) = sk_path(&run, closed) {
                pixmap.stroke_path(&path, &sk_paint(stroke.color, object.opacity), &style, Transform::identity(), None);
            }
        }
    }
}

/// Fills an object with its gradient or pattern: its shape is drawn on its own to
/// find how much of each pixel it covers, and each covered pixel gets the color the fill has there.
fn draw_styled_fill(pixmap: &mut Pixmap, object: &Object, style: &crate::model::FillStyle) {
    use crate::model::FillKind;
    let Some(path) = sk_path(&object.outline(), object.is_closed()) else { return };
    let b = object.bounds;
    // The part of the page the shape can touch.
    let (left, top) = ((b.x.floor() as i32 - 1).max(0), (b.y.floor() as i32 - 1).max(0));
    let (right, bottom) = ((b.right().ceil() as i32 + 1).min(pixmap.width() as i32), (b.bottom().ceil() as i32 + 1).min(pixmap.height() as i32));
    if right <= left || bottom <= top {
        return;
    }
    let (w, h) = ((right - left) as u32, (bottom - top) as u32);
    let Some(mut shape) = Pixmap::new(w, h) else { return };
    shape.fill_path(&path, &sk_paint(Color::BLACK, 1.0), FillRule::EvenOdd, Transform::from_translate(-left as f32, -top as f32), None);
    let covered: Vec<f32> = shape.pixels().iter().map(|px| px.alpha() as f32 / 255.0).collect();
    // A contour follows the outline, so it needs to know how deep inside each pixel lies.
    let depth: Vec<f32> = if style.kind == FillKind::Contour {
        let outside: Vec<f32> = covered.iter().map(|c| if *c < 0.5 { 1.0 } else { 0.0 }).collect();
        let far = crate::pseffects::distances(&outside, w as usize, h as usize);
        // The frame of the box counts as outside too, for a shape that fills it.
        let edge = |at: usize| ((at % w as usize).min(w as usize - 1 - at % w as usize).min(at / w as usize).min(h as usize - 1 - at / w as usize)) as f32 + 1.0;
        let far: Vec<f32> = far.iter().enumerate().map(|(at, d)| d.min(edge(at))).collect();
        let deepest = far.iter().zip(&covered).filter(|(_, c)| **c >= 0.5).map(|(d, _)| *d).fold(1.0, f32::max);
        far.iter().map(|d| (d / deepest).clamp(0.0, 1.0)).collect()
    } else {
        Vec::new()
    };
    let scale = (style.scale / 100.0).max(0.01);
    for (at, coverage) in covered.iter().enumerate().filter(|(_, c)| **c > 0.0) {
        let (x, y) = (left + (at as u32 % w) as i32, top + (at as u32 / w) as i32);
        let p = Vec2 { x: x as f32 + 0.5, y: y as f32 + 0.5 };
        let (color, opacity) = if style.kind == FillKind::Pattern {
            // A pattern is laid from the corner of the object, so it moves with it.
            (crate::pseffects::pattern(style.pattern as usize, (p.x - b.x) / scale, (p.y - b.y) / scale), 1.0)
        } else {
            crate::pseffects::gradient_color(&style.stops, style.place(p, b, depth.get(at).copied().unwrap_or(0.0)))
        };
        let channel = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        let tone = Color { r: channel(color[0]), g: channel(color[1]), b: channel(color[2]), a: channel(opacity * object.opacity) };
        blend_pixel(pixmap, x, y, tone, *coverage);
    }
}

pub fn render_page(page: &Page) -> Result<Pixmap> {
    let mut pixmap = Pixmap::new(page.width.max(1), page.height.max(1))
        .ok_or_else(|| anyhow!(tf("canvas size {}x{} is too large", &[&page.width, &page.height])))?;
    if let Some(c) = page.canvas {
        pixmap.fill(tiny_skia::Color::from_rgba8(c.r, c.g, c.b, c.a));
    }
    for layer in page.layers.iter().filter(|l| l.visible) {
        for object in &layer.objects {
            draw_object(&mut pixmap, object);
        }
    }
    Ok(pixmap)
}

/// Writes the page to `path` in the format named by its extension.
pub fn export_image(page: &Page, path: &Path) -> Result<()> {
    save_pixmap(&render_page(page)?, path)
}

/// Writes slice images and a hotspot image map beside an exported image.
/// Returns how many extra files were written.
pub fn export_web(page: &Page, path: &Path) -> Result<usize> {
    let shapes: Vec<(&Object, WebKind, &str)> = page
        .layers
        .iter()
        .filter(|l| l.visible)
        .flat_map(|l| l.objects.iter())
        .filter(|o| o.visible)
        .filter_map(|o| match &o.shape {
            Shape::Web { kind, url, .. } => Some((o, *kind, url.as_str())),
            _ => None,
        })
        .collect();
    if shapes.is_empty() {
        return Ok(0);
    }
    let stem = path.file_stem().map_or_else(|| "export".into(), |s| s.to_string_lossy().into_owned());
    let extension = path.extension().map_or_else(|| "png".into(), |e| e.to_string_lossy().into_owned());
    let pixmap = render_page(page)?;
    let mut written = 0;
    for (object, _, _) in shapes.iter().filter(|(_, kind, _)| *kind == WebKind::Slice) {
        let Some(area) = page.rect().intersect(object.bounds) else { continue };
        let crop = IntRect::from_xywh(area.x.round() as i32, area.y.round() as i32, area.w.round().max(1.0) as u32, area.h.round().max(1.0) as u32);
        let Some(mut piece) = crop.and_then(|c| pixmap.clone_rect(c)) else { continue };
        // A many-sided slice keeps only what lies inside its outline; the rest of its box is left clear.
        if let Some(corners) = object.web_outline() {
            let (left, top, columns) = (area.x.round(), area.y.round(), piece.width() as usize);
            for (at, px) in piece.pixels_mut().iter_mut().enumerate() {
                let p = Vec2 { x: left + (at % columns) as f32 + 0.5, y: top + (at / columns) as f32 + 0.5 };
                if !crate::model::point_in_polygon(p, &corners) {
                    *px = PremultipliedColorU8::TRANSPARENT;
                }
            }
        }
        written += 1;
        save_pixmap(&piece, &path.with_file_name(format!("{stem}_slice{written}.{extension}")))?;
    }
    let hotspots: Vec<_> = shapes.iter().filter(|(_, kind, _)| *kind == WebKind::Hotspot).collect();
    if !hotspots.is_empty() {
        let image = path.file_name().map_or_else(String::new, |n| n.to_string_lossy().into_owned());
        let html = image_map(&image, page, hotspots.iter().map(|(object, _, url)| (*object, object.name.as_str(), *url)));
        let target = path.with_file_name(format!("{stem}_map.html"));
        std::fs::write(&target, html).with_context(|| tf("could not write {}", &[&target.display()]))?;
        written += 1;
    }
    Ok(written)
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;").replace('"', "&quot;").replace('<', "&lt;").replace('>', "&gt;")
}

fn image_map<'a>(image: &str, page: &Page, areas: impl Iterator<Item = (&'a Object, &'a str, &'a str)>) -> String {
    let mut html = format!(
        "<!doctype html>\n<img src=\"{}\" width=\"{}\" height=\"{}\" usemap=\"#map\" alt=\"\">\n<map name=\"map\">\n",
        escape_html(image),
        page.width,
        page.height
    );
    for (object, name, url) in areas {
        let href = if url.is_empty() { "#" } else { url };
        let b = object.bounds;
        // A hotspot is a rectangle, a circle or a polygon, each with the coordinates HTML asks for.
        let (shape, coords) = match &object.shape {
            Shape::Web { form: crate::model::WebForm::Circle, .. } => {
                ("circle", format!("{},{},{}", b.center().x.round() as i32, b.center().y.round() as i32, (b.w.min(b.h) / 2.0).round() as i32))
            }
            Shape::Web { form: crate::model::WebForm::Polygon(_), .. } => {
                let corners = object.web_outline().unwrap_or_default();
                ("poly", corners.iter().map(|p| format!("{},{}", p.x.round() as i32, p.y.round() as i32)).collect::<Vec<_>>().join(","))
            }
            _ => ("rect", format!("{},{},{},{}", b.x.round() as i32, b.y.round() as i32, b.right().round() as i32, b.bottom().round() as i32)),
        };
        html.push_str(&format!("  <area shape=\"{shape}\" coords=\"{coords}\" href=\"{}\" alt=\"{}\">\n", escape_html(href), escape_html(name)));
    }
    html.push_str("</map>\n");
    html
}

fn save_pixmap(pixmap: &Pixmap, path: &Path) -> Result<()> {
    let extension = path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    if extension == "png" {
        return pixmap.save_png(path).with_context(|| tf("could not write {}", &[&path.display()]));
    }
    let mut rgba = Vec::with_capacity(pixmap.data().len());
    for px in pixmap.pixels() {
        let c = px.demultiply();
        rgba.extend_from_slice(&[c.red(), c.green(), c.blue(), c.alpha()]);
    }
    let image = image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), rgba)
        .ok_or_else(|| anyhow!(t("rendered image has an invalid size")))?;
    let result = if matches!(extension.as_str(), "jpg" | "jpeg") {
        flatten_on_white(&image).save(path)
    } else {
        image.save(path)
    };
    result.with_context(|| tf("could not write {}", &[&path.display()]))
}

/// JPEG has no alpha channel, so transparency is composited over white.
fn flatten_on_white(image: &image::RgbaImage) -> image::RgbImage {
    let (w, h) = image.dimensions();
    image::RgbImage::from_fn(w, h, |x, y| {
        let [r, g, b, a] = image.get_pixel(x, y).0;
        let alpha = a as f32 / 255.0;
        let over = |c: u8| (c as f32 * alpha + 255.0 * (1.0 - alpha)).round() as u8;
        image::Rgb([over(r), over(g), over(b)])
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Rect, Stroke};

    fn page() -> Page {
        Page::new("t", 40, 40)
    }

    fn rgba(p: &Pixmap, x: u32, y: u32) -> (u8, u8, u8, u8) {
        let c = p.pixel(x, y).unwrap().demultiply();
        (c.red(), c.green(), c.blue(), c.alpha())
    }

    fn red_box(x: f32, w: f32) -> Object {
        Object {
            id: 1,
            name: "box".into(),
            shape: Shape::Rect { radius: 0.0 },
            bounds: Rect { x, y: 0.0, w, h: 40.0 },
            points: vec![],
            handles: Vec::new(),
            placement: None,
            blend: crate::blend::BlendMode::Normal,
            mask: None,
            mask_of: None,
            fill_style: None,
            stroke_profile: Vec::new(),
            filters: Vec::new(),
            fill: Some(Color::rgb(255, 0, 0)),
            stroke: None,
            opacity: 1.0,
            visible: true,
            locked: false,
        }
    }

    #[test]
    fn fills_canvas_and_object() {
        let mut p = page();
        p.layers[0].objects.push(red_box(0.0, 20.0));
        let img = render_page(&p).unwrap();
        assert_eq!(rgba(&img, 5, 20), (255, 0, 0, 255));
        assert_eq!(rgba(&img, 30, 20), (255, 255, 255, 255));
    }

    #[test]
    fn transparent_canvas_stays_transparent() {
        let mut p = page();
        p.canvas = None;
        assert_eq!(rgba(&render_page(&p).unwrap(), 10, 10).3, 0);
    }

    #[test]
    fn hidden_layers_and_objects_are_not_exported() {
        let mut p = page();
        let mut o = red_box(0.0, 40.0);
        o.visible = false;
        p.layers[0].objects.push(o);
        assert_eq!(rgba(&render_page(&p).unwrap(), 5, 5), (255, 255, 255, 255));
        p.layers[0].objects[0].visible = true;
        p.layers[0].visible = false;
        assert_eq!(rgba(&render_page(&p).unwrap(), 5, 5), (255, 255, 255, 255));
    }

    #[test]
    fn opacity_blends_with_canvas() {
        let mut p = page();
        let mut o = red_box(0.0, 40.0);
        o.opacity = 0.5;
        p.layers[0].objects.push(o);
        let (r, g, _, a) = rgba(&render_page(&p).unwrap(), 20, 20);
        assert_eq!((r, a), (255, 255));
        assert!((120..=135).contains(&g), "g was {g}");
    }

    #[test]
    fn stroke_is_painted_on_open_path() {
        let mut p = page();
        let mut o = red_box(0.0, 40.0);
        o.shape = Shape::Line;
        o.fill = None;
        o.points = vec![crate::model::v2(0.0, 20.0), crate::model::v2(40.0, 20.0)];
        o.stroke = Some(Stroke { color: Color::BLACK, width: 4.0 });
        p.layers[0].objects.push(o);
        let img = render_page(&p).unwrap();
        assert_eq!(rgba(&img, 20, 20), (0, 0, 0, 255));
        assert_eq!(rgba(&img, 20, 5), (255, 255, 255, 255));
    }

    #[test]
    fn a_scrubbed_stroke_is_heavier_where_it_was_scrubbed() {
        let mut p = page();
        p.canvas = None;
        let mut line = red_box(0.0, 40.0);
        line.shape = Shape::Line;
        line.fill = None;
        line.points = vec![Vec2 { x: 2.0, y: 20.0 }, Vec2 { x: 38.0, y: 20.0 }];
        line.stroke = Some(Stroke { color: Color::rgb(0, 0, 0), width: 4.0 });
        assert!(line.stroke_weights().is_none());
        assert!(line.prepare_scrub());
        assert_eq!(line.stroke_weights().map(|w| w.len()), Some(line.points.len()));
        // Thicker on the left, thinner on the right, untouched in the middle.
        line.scrub(Vec2 { x: 8.0, y: 20.0 }, 6.0, 1.0);
        line.scrub(Vec2 { x: 32.0, y: 20.0 }, 6.0, -0.9);
        p.layers[0].objects.push(line.clone());
        let image = render_page(&p).unwrap();
        let tall = |x: u32| (0..40).filter(|y| rgba(&image, x, *y).3 > 128).count();
        assert!(tall(8) >= 7 && tall(20) == 4 && tall(32) <= 1, "{} {} {}", tall(8), tall(20), tall(32));
        // The weights are kept in the file, and are let go once the path no longer has a number for each point.
        let back: Object = serde_json::from_str(&serde_json::to_string(&line).unwrap()).unwrap();
        assert_eq!(back.stroke_profile, line.stroke_profile);
        line.points.pop();
        assert!(line.stroke_weights().is_none());
        assert!(!serde_json::to_string(&red_box(0.0, 4.0)).unwrap().contains("stroke_profile"));
    }

    #[test]
    fn a_gradient_or_pattern_fills_an_object_in_place_of_its_color() {
        use crate::model::{FillKind, FillStyle};
        use std::sync::Arc;
        let (black, white) = (Color::rgb(0, 0, 0), Color::rgb(255, 255, 255));
        let filled = |style: FillStyle| {
            let mut p = page();
            p.canvas = None;
            let mut object = red_box(4.0, 32.0);
            object.bounds = Rect { x: 4.0, y: 4.0, w: 32.0, h: 32.0 };
            object.fill_style = Some(Arc::new(style));
            p.layers[0].objects.push(object);
            render_page(&p).unwrap()
        };
        // Linear: black at the left of the object, white at the right, gray in between, nothing outside it.
        let linear = filled(FillStyle::new(FillKind::Linear, black, white));
        let (left, middle, right) = (rgba(&linear, 5, 20), rgba(&linear, 20, 20), rgba(&linear, 35, 20));
        assert!(left.0 < 20 && right.0 > 235 && (110..150).contains(&middle.0), "{left:?} {middle:?} {right:?}");
        assert_eq!((left.3, rgba(&linear, 1, 20).3, rgba(&linear, 20, 38).3), (255, 0, 0));
        assert_eq!(rgba(&linear, 20, 6), rgba(&linear, 20, 34), "the same all the way down");
        // Radial: black in the middle, lighter toward every side.
        let radial = filled(FillStyle::new(FillKind::Radial, black, white));
        assert!(rgba(&radial, 20, 20).0 < 20 && rgba(&radial, 34, 20).0 > 200 && rgba(&radial, 20, 6).0 > 200 && rgba(&radial, 6, 20).0 > 200);
        // Contour: like radial it starts deep inside, but it follows the outline: white along every edge.
        let contour = filled(FillStyle::new(FillKind::Contour, black, white));
        assert!(rgba(&contour, 20, 20).0 < 60 && rgba(&contour, 5, 20).0 > 200 && rgba(&contour, 20, 34).0 > 200 && rgba(&contour, 5, 5).0 > 200, "{:?}", rgba(&contour, 20, 20));
        // A handle moved to the middle makes the gradient half as long.
        let mut short = FillStyle::new(FillKind::Linear, black, white);
        short.set_handle(true, Vec2 { x: 20.0, y: 20.0 }, Rect { x: 4.0, y: 4.0, w: 32.0, h: 32.0 });
        assert_eq!(short.handles(Rect { x: 4.0, y: 4.0, w: 32.0, h: 32.0 }).1, Vec2 { x: 20.0, y: 20.0 });
        assert!(rgba(&filled(short), 21, 20).0 > 235);
        // A stop that is see-through lets what is under the object show.
        let mut fading = FillStyle::new(FillKind::Linear, black, Color { r: 0, g: 0, b: 0, a: 0 });
        fading.stops[0].1 = Color::rgb(255, 0, 0);
        let faded = filled(fading);
        assert!(rgba(&faded, 5, 20).3 > 235 && rgba(&faded, 35, 20).3 < 20);
        // A pattern is two grays, and twice as large at a scale of 200.
        let mut pattern = FillStyle::new(FillKind::Pattern, black, white);
        let small = filled(pattern.clone());
        assert_ne!(rgba(&small, 6, 6), rgba(&small, 14, 6));
        pattern.scale = 200.0;
        let large = filled(pattern);
        assert_eq!(rgba(&large, 6, 6), rgba(&large, 14, 6));
        assert_ne!(rgba(&large, 6, 6), rgba(&large, 22, 6));
        // Every kind draws something and stays inside the object.
        for kind in FillKind::ALL {
            let drawn = filled(FillStyle::new(kind, black, white));
            assert!(rgba(&drawn, 20, 20).3 == 255 && rgba(&drawn, 1, 1).3 == 0, "{kind:?}");
        }
        // The fill is kept in the file, and an object without one is written as before.
        let mut object = red_box(0.0, 10.0);
        assert!(!serde_json::to_string(&object).unwrap().contains("fill_style"));
        object.fill_style = Some(Arc::new(FillStyle::new(FillKind::Cone, black, white)));
        let back: Object = serde_json::from_str(&serde_json::to_string(&object).unwrap()).unwrap();
        assert_eq!(back.fill_style, object.fill_style);
    }

    #[test]
    fn text_is_aligned_underlined_and_set_in_other_faces() {
        use crate::model::{TextAlign, TextStyle};
        let plain = TextStyle::default();
        assert!(plain.is_plain());
        // The lines of a text stand in a box as wide as its longest line.
        let (width, height) = measure_styled("HHHH\nH", 20.0, &plain);
        let short = line_width("H", 20.0, &plain);
        assert!(width > short * 3.5 && height == 2.0 * 20.0 * LINE_HEIGHT);
        assert_eq!(measure_styled("HHHH\nH", 20.0, &plain), measure_text("HHHH\nH", 20.0));
        assert_eq!(line_start(width, short, TextAlign::Left), 0.0);
        assert_eq!(line_start(width, short, TextAlign::Center), (width - short) / 2.0);
        assert_eq!(line_start(width, short, TextAlign::Right), width - short);
        assert_eq!(line_start(10.0, 30.0, TextAlign::Right), 0.0, "a line never starts left of its box");
        if font().is_none() {
            return;
        }
        // Right-aligned, the short second line is drawn at the right of the box.
        let left_most = |style: &TextStyle| {
            let glyphs = glyph_outlines("HHHH\nH", 20.0, Vec2 { x: 0.0, y: 0.0 }, style);
            glyphs.last().unwrap().iter().flatten().map(|p| p.x).fold(f32::MAX, f32::min)
        };
        let right = TextStyle { align: TextAlign::Right, ..TextStyle::default() };
        assert!(left_most(&plain) < short && left_most(&right) > width - short - 1.0);
        // Underlined text has a bar under each line, as wide as the line.
        let underlined = TextStyle { underline: true, ..TextStyle::default() };
        let (bare, lined) = (glyph_outlines("HH", 20.0, Vec2 { x: 0.0, y: 0.0 }, &plain), glyph_outlines("HH", 20.0, Vec2 { x: 0.0, y: 0.0 }, &underlined));
        assert_eq!(lined.len(), bare.len() + 1);
        let bar = &lined[0][0];
        assert!((bar[1].x - bar[0].x - line_width("HH", 20.0, &plain)).abs() < 0.01);
        // And it is drawn: more of the page is inked than without it.
        let inked = |style: &TextStyle| {
            let mut p = page();
            let mut text = red_box(2.0, 30.0);
            text.shape = Shape::Text { content: "HH".into(), size: 14.0, style: style.clone() };
            p.layers[0].objects.push(text);
            render_page(&p).unwrap().pixels().iter().filter(|px| px.red() > 200 && px.green() < 60).count()
        };
        assert!(inked(&underlined) > inked(&plain));
        // A style that is not plain is kept in the file; plain text is written as before.
        let styled = Shape::Text { content: "a".into(), size: 12.0, style: TextStyle { font: "Some Font".into(), bold: true, ..TextStyle::default() } };
        let back: Shape = serde_json::from_str(&serde_json::to_string(&styled).unwrap()).unwrap();
        assert_eq!(back, styled);
        assert!(!serde_json::to_string(&Shape::Text { content: "a".into(), size: 12.0, style: plain }).unwrap().contains("style"));
        let old: Shape = serde_json::from_str(r#"{"Text":{"content":"a","size":12.0}}"#).unwrap();
        assert!(matches!(old, Shape::Text { style, .. } if style.is_plain()));
        // An unknown family falls back to a font that is there, so text never vanishes.
        assert!(line_width("H", 20.0, &TextStyle { font: "No Such Font 12345".into(), ..TextStyle::default() }) > 0.0);
    }

    #[test]
    fn a_photoshop_shadow_is_mixed_into_the_page_by_its_blend_mode() {
        use crate::blend::BlendMode;
        use crate::effects::{FilterKind, LiveFilter};
        use crate::pseffects::{index, DROP_SHADOW};
        let mode = |mode: BlendMode| BlendMode::ALL.iter().position(|m| *m == mode).unwrap() as f32;
        let mut p = page();
        // A blue box on red, with a hard white shadow thrown down and to the right.
        p.layers[0].objects.push(red_box(0.0, 40.0));
        let mut boxed = red_box(10.0, 12.0);
        boxed.fill = Some(Color::rgb(0, 0, 255));
        let mut effects = LiveFilter::new(FilterKind::PhotoshopEffects);
        effects.set_ps_color(DROP_SHADOW, Color::rgb(255, 255, 255));
        for (slot, value) in [(1, 100.0), (3, 8.0), (4, 0.0)] {
            effects.set_value(index(DROP_SHADOW, slot), value);
        }
        boxed.filters = vec![effects];
        p.layers[0].objects.push(boxed);
        let shadow_at = |p: &Page| rgba(&render_page(p).unwrap(), 24, 27);
        // Multiplied, as a shadow starts out, white leaves the red as it is.
        assert_eq!(shadow_at(&p), (255, 0, 0, 255));
        // Laid on normally it covers the red; screened it lightens it to white as well.
        p.layers[0].objects[1].filters[0].set_value(index(DROP_SHADOW, 5), mode(BlendMode::Normal));
        assert_eq!(shadow_at(&p), (255, 255, 255, 255));
        // A dark green shadow screened onto red gives a dull yellow; multiplied, black.
        p.layers[0].objects[1].filters[0].set_ps_color(DROP_SHADOW, Color::rgb(0, 128, 0));
        p.layers[0].objects[1].filters[0].set_value(index(DROP_SHADOW, 5), mode(BlendMode::Screen));
        assert_eq!(shadow_at(&p), (255, 128, 0, 255));
        p.layers[0].objects[1].filters[0].set_value(index(DROP_SHADOW, 5), mode(BlendMode::Multiply));
        assert_eq!(shadow_at(&p), (0, 0, 0, 255));
        // The box itself is drawn over its shadow, untouched.
        assert_eq!(rgba(&render_page(&p).unwrap(), 15, 15), (0, 0, 255, 255));
        // A filter after it works on the shadow as on the object, and the shadow is still mixed by its mode:
        // inverted, the dark green shadow is a light magenta, which multiplied into red leaves red.
        p.layers[0].objects[1].filters.push(LiveFilter::new(FilterKind::Invert));
        p.layers[0].objects[1].filters[1].enabled = false;
        assert_eq!(shadow_at(&p), (0, 0, 0, 255), "a filter that is off does nothing");
        p.layers[0].objects[1].filters[1].enabled = true;
        assert_eq!(shadow_at(&p), (255, 0, 0, 255));
        assert_eq!(rgba(&render_page(&p).unwrap(), 15, 15), (255, 255, 0, 255), "and the box is inverted too");
        // A Fireworks shadow added after it falls from the object, not from the Photoshop shadow as well.
        p.layers[0].objects[1].filters[1] = LiveFilter::new(FilterKind::DropShadow);
        assert_eq!(rgba(&render_page(&p).unwrap(), 15, 15), (0, 0, 255, 255));
    }

    #[test]
    fn a_blend_mode_mixes_an_object_with_what_is_under_it() {
        use crate::blend::BlendMode;
        let mut p = page();
        // A red box, and over its right half a box of pure green.
        p.layers[0].objects.push(red_box(0.0, 40.0));
        let mut over = red_box(20.0, 20.0);
        over.fill = Some(Color::rgb(0, 255, 0));
        over.blend = BlendMode::Multiply;
        p.layers[0].objects.push(over.clone());
        let img = render_page(&p).unwrap();
        // Red times green is black; outside the green box the red is untouched.
        assert_eq!(rgba(&img, 30, 10), (0, 0, 0, 255));
        assert_eq!(rgba(&img, 10, 10), (255, 0, 0, 255));
        // Screen adds them to yellow, and Normal just covers.
        p.layers[0].objects[1].blend = BlendMode::Screen;
        assert_eq!(rgba(&render_page(&p).unwrap(), 30, 10), (255, 255, 0, 255));
        p.layers[0].objects[1].blend = BlendMode::Normal;
        assert_eq!(rgba(&render_page(&p).unwrap(), 30, 10), (0, 255, 0, 255));
        // Erase cuts through to the canvas.
        p.layers[0].objects[1].blend = BlendMode::Erase;
        p.canvas = None;
        assert_eq!(rgba(&render_page(&p).unwrap(), 30, 10).3, 0);
        // The mode is saved with the object, and a normal object is written as before.
        let text = serde_json::to_string(&over).unwrap();
        assert!(text.contains("Multiply"));
        assert_eq!(serde_json::from_str::<Object>(&text).unwrap().blend, BlendMode::Multiply);
        assert!(!serde_json::to_string(&red_box(0.0, 1.0)).unwrap().contains("blend"));
    }

    /// A red box 40 wide, masked by `mask`.
    fn masked(mask: Object, kind: MaskKind) -> Page {
        let mut p = page();
        let mut o = red_box(0.0, 40.0);
        o.mask = Some(std::sync::Arc::new(ObjectMask { object: mask, kind, show_fill_stroke: false, enabled: true, linked: true }));
        p.layers[0].objects.push(o);
        p
    }

    #[test]
    fn a_path_mask_shows_only_what_is_inside_its_outline() {
        // The mask covers the left half. What it is filled with does not matter.
        let mut left = red_box(0.0, 20.0);
        left.fill = Some(Color::BLACK);
        let mut p = masked(left, MaskKind::PathOutline);
        let img = render_page(&p).unwrap();
        assert_eq!(rgba(&img, 10, 10), (255, 0, 0, 255));
        assert_eq!(rgba(&img, 30, 10), (255, 255, 255, 255), "the canvas shows where the mask hides the box");
        // Switched off, the mask hides nothing.
        let object = &mut p.layers[0].objects[0];
        std::sync::Arc::make_mut(object.mask.as_mut().unwrap()).enabled = false;
        assert_eq!(rgba(&render_page(&p).unwrap(), 30, 10), (255, 0, 0, 255));
    }

    #[test]
    fn a_grayscale_mask_shows_more_where_it_is_lighter() {
        let shade = |level: u8| {
            let mut mask = red_box(0.0, 40.0);
            mask.fill = Some(Color::rgb(level, level, level));
            masked(mask, MaskKind::Grayscale)
        };
        assert_eq!(rgba(&render_page(&shade(255)).unwrap(), 10, 10), (255, 0, 0, 255));
        assert_eq!(rgba(&render_page(&shade(0)).unwrap(), 10, 10), (255, 255, 255, 255));
        // Mid gray lets half of the red through, over the white canvas.
        let (r, g, _, _) = rgba(&render_page(&shade(128)).unwrap(), 10, 10);
        assert!(r == 255 && (120..=135).contains(&g), "g was {g}");
        // By its alpha channel, the same black mask shows everything.
        let mut black = red_box(0.0, 40.0);
        black.fill = Some(Color::BLACK);
        assert_eq!(rgba(&render_page(&masked(black, MaskKind::Alpha)).unwrap(), 10, 10), (255, 0, 0, 255));
    }

    #[test]
    fn a_path_mask_can_show_its_own_fill_and_stroke() {
        let mut frame = red_box(0.0, 20.0);
        frame.fill = Some(Color::rgb(0, 0, 255));
        // The masked box is narrower than the mask, so the mask's fill shows beside it.
        let mut p = page();
        let mut inside = red_box(0.0, 10.0);
        inside.mask = Some(std::sync::Arc::new(ObjectMask { object: frame, kind: MaskKind::PathOutline, show_fill_stroke: true, enabled: true, linked: true }));
        p.layers[0].objects.push(inside);
        let img = render_page(&p).unwrap();
        assert_eq!(rgba(&img, 5, 10), (255, 0, 0, 255));
        assert_eq!(rgba(&img, 15, 10), (0, 0, 255, 255));
        assert_eq!(rgba(&img, 30, 10), (255, 255, 255, 255));
    }

    #[test]
    fn a_mask_moves_and_stretches_with_its_object() {
        let mut p = masked(red_box(0.0, 20.0), MaskKind::PathOutline);
        let object = &mut p.layers[0].objects[0];
        object.translate(10.0, 0.0);
        assert_eq!(object.mask.as_ref().unwrap().object.bounds.x, 10.0);
        object.set_bounds(Rect { x: 0.0, y: 0.0, w: 80.0, h: 40.0 });
        let mask = object.mask.as_ref().unwrap().object.bounds;
        assert_eq!((mask.x, mask.w), (0.0, 40.0), "twice as wide, and still over the left half of the box");
        // Saved and read back, the mask is still there; an object without one is written as before.
        let text = serde_json::to_string(&*object).unwrap();
        assert_eq!(serde_json::from_str::<Object>(&text).unwrap().mask, object.mask);
        assert!(!serde_json::to_string(&red_box(0.0, 1.0)).unwrap().contains("mask"));
    }

    #[test]
    fn curved_path_is_exported_along_its_curve() {
        use crate::model::{v2, Handles};
        let mut p = page();
        let mut o = red_box(0.0, 40.0);
        o.shape = Shape::Path { closed: false };
        o.fill = None;
        // From (0,30) to (40,30), pulled up so the middle of the curve passes y = 7.5.
        o.points = vec![v2(0.0, 30.0), v2(40.0, 30.0)];
        o.handles = vec![
            Handles { before: v2(0.0, 0.0), after: v2(0.0, -30.0) },
            Handles { before: v2(0.0, -30.0), after: v2(0.0, 0.0) },
        ];
        o.stroke = Some(Stroke { color: Color::BLACK, width: 4.0 });
        p.layers[0].objects.push(o);
        let img = render_page(&p).unwrap();
        assert_eq!(rgba(&img, 20, 7), (0, 0, 0, 255));
        // Nothing is drawn on the straight line between the two ends.
        assert_eq!(rgba(&img, 20, 30), (255, 255, 255, 255));
    }

    #[test]
    fn png_file_is_written_with_page_size() {
        let path = std::env::temp_dir().join("emberframe_export_test.png");
        export_image(&page(), &path).unwrap();
        let img = Pixmap::load_png(&path).unwrap();
        assert_eq!((img.width(), img.height()), (40, 40));
        std::fs::remove_file(path).ok();
    }

    fn green_bitmap() -> Object {
        let pixels = [0u8, 255, 0, 255].repeat(20 * 20);
        Object::new_bitmap(9, "g", Bitmap::from_rgba(20, 20, pixels).unwrap(), 10.0, 10.0)
    }

    #[test]
    fn bitmap_is_drawn_at_its_bounds() {
        let mut p = page();
        p.layers[0].objects.push(green_bitmap());
        let img = render_page(&p).unwrap();
        assert_eq!(rgba(&img, 20, 20), (0, 255, 0, 255));
        assert_eq!(rgba(&img, 5, 5), (255, 255, 255, 255));
    }

    #[test]
    fn scaled_bitmap_covers_scaled_bounds() {
        let mut p = page();
        let mut o = green_bitmap();
        o.set_bounds(Rect { x: 0.0, y: 0.0, w: 40.0, h: 40.0 });
        p.layers[0].objects.push(o);
        assert_eq!(rgba(&render_page(&p).unwrap(), 36, 36), (0, 255, 0, 255));
    }

    #[test]
    fn jpeg_export_flattens_transparency_on_white() {
        let path = std::env::temp_dir().join("emberframe_export_test.jpg");
        let mut p = page();
        p.canvas = None;
        export_image(&p, &path).unwrap();
        let decoded = image::open(&path).unwrap().to_rgb8();
        assert!(decoded.get_pixel(20, 20).0.iter().all(|c| *c > 250));
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn web_areas_stay_out_of_the_image_but_produce_files() {
        let dir = std::env::temp_dir().join("emberframe_web_export_test");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("site.png");
        let mut p = page();
        let before = rgba(&render_page(&p).unwrap(), 5, 5);
        p.layers[0].objects.push(Object::new_web(7, WebKind::Slice, Rect { x: 0.0, y: 0.0, w: 10.0, h: 20.0 }));
        let mut hotspot = Object::new_web(8, WebKind::Hotspot, Rect { x: 5.0, y: 6.0, w: 10.0, h: 10.0 });
        hotspot.shape = Shape::Web { kind: WebKind::Hotspot, url: "https://example.com/?a=1&b=2".into(), form: Default::default() };
        p.layers[0].objects.push(hotspot);
        assert_eq!(rgba(&render_page(&p).unwrap(), 5, 5), before);
        assert_eq!(export_web(&p, &path).unwrap(), 2);
        let slice = Pixmap::load_png(dir.join("site_slice1.png")).unwrap();
        assert_eq!((slice.width(), slice.height()), (10, 20));
        let html = std::fs::read_to_string(dir.join("site_map.html")).unwrap();
        assert!(html.contains("coords=\"5,6,15,16\""));
        assert!(html.contains("a=1&amp;b=2"));
        // A round hotspot and a many-sided one are written as HTML's circle and poly.
        p.layers[0].objects.push(Object::new_web_circle(50, Rect { x: 10.0, y: 10.0, w: 20.0, h: 20.0 }));
        p.layers[0].objects.push(Object::new_web_polygon(51, WebKind::Hotspot, &[Vec2 { x: 1.0, y: 2.0 }, Vec2 { x: 30.0, y: 2.0 }, Vec2 { x: 15.0, y: 25.0 }]));
        export_web(&p, &path).unwrap();
        let html = std::fs::read_to_string(path.with_file_name(format!("{}_map.html", path.file_stem().unwrap().to_string_lossy()))).unwrap();
        assert!(html.contains("shape=\"circle\" coords=\"20,20,10\""), "{html}");
        assert!(html.contains("shape=\"poly\" coords=\"1,2,30,2,15,25\""), "{html}");
        assert!(html.contains("shape=\"rect\" coords=\"5,6,15,16\""));
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn filled_open_path_keeps_its_fill() {
        let mut p = page();
        let mut piece = Object::new_web(9, WebKind::Slice, Rect::default());
        piece.shape = Shape::Path { closed: false };
        piece.points = vec![
            Vec2 { x: 0.0, y: 0.0 },
            Vec2 { x: 39.0, y: 0.0 },
            Vec2 { x: 39.0, y: 39.0 },
            Vec2 { x: 0.0, y: 39.0 },
        ];
        piece.fill = Some(Color::rgb(0, 255, 0));
        p.layers[0].objects.push(piece);
        assert_eq!(rgba(&render_page(&p).unwrap(), 20, 20), (0, 255, 0, 255));
    }

    #[test]
    fn no_web_areas_means_no_extra_files() {
        assert_eq!(export_web(&page(), Path::new("unused.png")).unwrap(), 0);
    }

    #[test]
    fn unknown_extension_reports_an_error() {
        let path = std::env::temp_dir().join("emberframe_export_test.nope");
        assert!(export_image(&page(), &path).is_err());
    }

    #[test]
    fn text_is_measured_at_em_size() {
        let Some(f) = font() else { return };
        let id = f.glyph_id('H');
        let upem = f.units_per_em().unwrap();
        let expected = 4.0 * f.h_advance_unscaled(id) / upem * 100.0;
        let (width, _) = measure_text("HHHH", 100.0);
        assert!((width - expected).abs() < 0.5, "width {width}, expected {expected}");
    }

    #[test]
    fn measure_grows_with_lines() {
        let (_, one) = measure_text("a", 10.0);
        let (_, two) = measure_text("a\nb", 10.0);
        assert!(two > one);
    }
}
