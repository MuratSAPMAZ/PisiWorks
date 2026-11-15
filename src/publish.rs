//! Publishing an animation as a page that plays in a browser.
//!
//! The page carries the whole animation in itself: one file to hand in or put on a
//! site. What stands still in a frame is drawn as the program draws it and stored as
//! a picture, so gradients, filters and masks look exactly as they do on the stage;
//! pictures that are the same are stored once. Movie clips and buttons are kept
//! apart as pictures of their own, so that a clip can play by its own count of frames
//! and a button can answer the mouse. A short script plays the scenes one after
//! another, runs the actions of frames and buttons, and starts the sounds. An object
//! with behaviors is a picture of its own as well, which the script moves by the rules
//! of [`crate::behavior`].

use std::collections::HashMap;

use anyhow::{ensure, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::i18n::t;
use crate::model::{Document, Object, Page, Rect, Shape, SymbolKind, Warp};

/// The most frames a published page may hold, over all its scenes.
const MAX_PUBLISHED: usize = 3000;

fn escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// The pictures of the page, each stored once.
#[derive(Default)]
struct Pictures {
    sources: Vec<String>,
    seen: HashMap<Vec<u8>, usize>,
}

impl Pictures {
    fn add(&mut self, png: Vec<u8>) -> usize {
        if let Some(index) = self.seen.get(&png) {
            return *index;
        }
        self.sources.push(format!("\"data:image/png;base64,{}\"", STANDARD.encode(&png)));
        self.seen.insert(png, self.sources.len() - 1);
        self.sources.len() - 1
    }

    /// Objects drawn as one picture of their own: the picture, and where its top left lies. `None` if they draw nothing.
    fn sprite(&mut self, objects: &[Object]) -> Result<Option<(usize, f32, f32)>> {
        let Some((bitmap, area)) = crate::export::render_objects(objects) else { return Ok(None) };
        Ok(Some((self.add(bitmap.encode_png()?), area.x, area.y)))
    }
}

/// One picture of a sprite as the script reads it: the picture, and where it is drawn from.
fn sprite_text(sprite: Option<(usize, f32, f32)>) -> String {
    match sprite {
        Some((picture, x, y)) => format!("[{picture},{x},{y}]"),
        None => "null".to_string(),
    }
}

/// The longest run of frames a movie clip's pictures are made for.
const MAX_CLIP_FRAMES: usize = 600;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// The frames of a movie clip's own timeline, if it has more than one.
fn clip_frames(doc: &Document, symbol: u64) -> Option<(&Page, usize)> {
    let page = doc.clip_page(symbol).map(|index| &doc.pages[index])?;
    let count = page.timeline.as_ref()?.frame_count();
    (count > 1).then_some((page, count))
}

/// What a clip's page shows on a frame, as the finished film shows it.
fn clip_shown(page: &Page, frame: usize) -> Vec<Object> {
    page.at_frame(frame).played().objects().filter(|o| o.visible && !o.is_web()).cloned().collect()
}

/// After how many frames a movie clip looks as it did at the start: its own length,
/// stretched until the clips inside it have come round as well.
fn clip_period(doc: &Document, symbol: u64, depth: usize) -> usize {
    let Some((page, count)) = clip_frames(doc, symbol) else { return 1 };
    if depth >= 4 {
        return count;
    }
    let mut period = count;
    for frame in 0..count {
        for object in clip_shown(page, frame) {
            if let Shape::Instance { symbol: inner, .. } = object.shape {
                if inner != symbol && doc.symbols.iter().any(|s| s.id == inner && s.kind == SymbolKind::Animation) {
                    let other = clip_period(doc, inner, depth + 1);
                    period = (period / gcd(period, other)).saturating_mul(other);
                }
            }
        }
    }
    period.min(MAX_CLIP_FRAMES)
}

/// What a movie clip shows after `tick` frames of playing, placed against the top left of its
/// first frame. The clips inside it are shown at the frame they have reached by then, each
/// by its own timeline, so that a clip within a clip keeps playing.
fn clip_frame(doc: &Document, symbol: u64, tick: usize, depth: usize) -> Vec<Object> {
    let Some(entry) = doc.symbols.iter().find(|s| s.id == symbol) else { return Vec::new() };
    let Some((page, count)) = clip_frames(doc, symbol) else { return entry.objects.clone() };
    let origin = clip_shown(page, 0).iter().map(|o| o.bounds).reduce(Rect::union).unwrap_or_default();
    let mut shown = Vec::new();
    for mut object in clip_shown(page, tick % count) {
        let inner = match object.shape {
            Shape::Instance { symbol: inner, .. } if depth < 4 && inner != symbol && clip_frames(doc, inner).is_some() => Some(inner),
            _ => None,
        };
        match inner {
            None => {
                object.translate(-origin.x, -origin.y);
                shown.push(object);
            }
            Some(inner) => {
                // The inner clip's frame, carried into the box its copy has here as the copy's own first frame would be.
                let natural = doc.symbols.iter().find(|s| s.id == inner).and_then(|s| s.objects.iter().map(|o| o.bounds).reduce(Rect::union)).unwrap_or_default();
                let frame_box = object.placement.map_or(object.bounds, |p| p.frame);
                let fit = Warp::fit(Rect { x: 0.0, y: 0.0, w: natural.w, h: natural.h }, frame_box);
                let carry = object.placement.map_or(fit, |p| fit.then(&p.warp));
                for mut part in clip_frame(doc, inner, tick, depth + 1) {
                    part.warp(&carry);
                    part.opacity *= object.opacity;
                    part.translate(-origin.x, -origin.y);
                    shown.push(part);
                }
            }
        }
    }
    shown
}

/// Objects with the live filters of the copy they are shown through, as one object the filters work on.
fn filtered(objects: Vec<Object>, filters: &[crate::effects::LiveFilter]) -> Vec<Object> {
    let Some(bounds) = objects.iter().map(|o| o.bounds).reduce(Rect::union).filter(|_| !filters.is_empty()) else { return objects };
    let mut group = Object::new_web(u64::MAX, crate::model::WebKind::Slice, bounds);
    group.shape = Shape::Group { objects: std::sync::Arc::new(objects) };
    group.filters = filters.to_vec();
    vec![group]
}

/// What a copy of a symbol is in the finished animation.
enum Moving {
    /// A movie clip with frames of its own: the number of its set of pictures.
    Clip(usize),
    /// A button: the number of its set of pictures (up, over, down).
    Button(usize),
}

/// The animation as a single HTML page. `title` names the page.
pub fn html(doc: &Document, title: &str) -> Result<String> {
    let scenes = doc.scenes();
    let first = scenes.first().map(|index| &doc.pages[*index]).context(t("This document has no timeline"))?;
    let fps = first.timeline.as_ref().context(t("This document has no timeline"))?.fps.clamp(1.0, 120.0);
    let (width, height) = (first.width, first.height);
    let mut pictures = Pictures::default();

    // The sets of pictures of the movie clips and the buttons, made when a copy of one is first met.
    let mut sets: Vec<String> = Vec::new();
    // A copy with live filters has pictures of its own, with the filters at work in them.
    let mut set_of: HashMap<(u64, String), Option<usize>> = HashMap::new();
    let mut moving = |symbol: u64, filters: &[crate::effects::LiveFilter], pictures: &mut Pictures, sets: &mut Vec<String>| -> Result<Option<Moving>> {
        let Some(entry) = doc.symbols.iter().find(|s| s.id == symbol) else { return Ok(None) };
        let key = (symbol, if filters.is_empty() { String::new() } else { format!("{filters:?}") });
        let known = set_of.get(&key).copied();
        let made = match (entry.kind, known) {
            (SymbolKind::Graphic, _) => return Ok(None),
            (_, Some(set)) => set,
            (SymbolKind::Animation, None) => {
                // A clip's pictures, one for each frame until it and the clips inside it have come round.
                let period = clip_period(doc, symbol, 0);
                let set = if period <= 1 {
                    None
                } else {
                    let mut texts = Vec::with_capacity(period);
                    for tick in 0..period {
                        texts.push(sprite_text(pictures.sprite(&filtered(clip_frame(doc, symbol, tick, 0), filters))?));
                    }
                    sets.push(format!("[{}]", texts.join(",")));
                    Some(sets.len() - 1)
                };
                set_of.insert(key, set);
                set
            }
            (SymbolKind::Button, None) => {
                // Up, Over and Down, each placed against the Up look.
                let mut texts = vec![sprite_text(pictures.sprite(&filtered(entry.objects.clone(), filters))?)];
                for look in entry.looks.iter().take(2) {
                    texts.push(if look.is_empty() { "null".to_string() } else { sprite_text(pictures.sprite(&filtered(look.clone(), filters))?) });
                }
                sets.push(format!("[{}]", texts.join(",")));
                set_of.insert(key, Some(sets.len() - 1));
                Some(sets.len() - 1)
            }
        };
        Ok(made.map(|set| if entry.kind == SymbolKind::Button { Moving::Button(set) } else { Moving::Clip(set) }))
    };

    // What stands still, drawn as one picture the size of the stage.
    let still = |objects: &[Object], pictures: &mut Pictures| -> Result<String> {
        let mut page = Page::new("", width, height);
        page.canvas = None;
        page.layers[0].objects = objects.to_vec();
        Ok(format!("[\"i\",{}]", pictures.add(crate::export::render_page(&page)?.encode_png()?)))
    };

    let mut total = 0;
    let mut scene_texts = Vec::new();
    for index in &scenes {
        let page = &doc.pages[*index];
        let timeline = page.timeline.as_ref().context(t("This document has no timeline"))?;
        let count = timeline.frame_count().max(1);
        total += count;
        ensure!(total <= MAX_PUBLISHED, t("The animation has too many frames to publish as one page"));
        let mut frame_texts = Vec::with_capacity(count);
        for frame in 0..count {
            let shown = page.at_frame(frame).played();
            let mut ops = Vec::new();
            let mut run: Vec<Object> = Vec::new();
            for object in shown.layers.iter().filter(|l| l.visible).flat_map(|l| l.objects.iter()).filter(|o| o.visible && !o.is_web()) {
                let kind = match object.shape {
                    Shape::Instance { symbol, .. } => moving(symbol, &object.filters, &mut pictures, &mut sets)?.map(|kind| (kind, symbol)),
                    _ => None,
                };
                // What the script needs to move an object by its behaviors: the point it turns around, its box, and the list.
                let behaving = (!object.behaviors.is_empty()).then(|| {
                    let (p, b) = (object.pivot_point().unwrap_or(object.bounds.center()), object.bounds);
                    let list: Vec<String> = object.behaviors.iter().map(|behavior| behavior.text()).collect();
                    format!("{{\"p\":[{},{}],\"box\":[{},{},{},{}],\"l\":[{}]}}", p.x, p.y, b.x, b.y, b.w, b.h, list.join(","))
                });
                if kind.is_none() && behaving.is_none() {
                    run.push(object.clone());
                    continue;
                }
                if !run.is_empty() {
                    ops.push(still(&run, &mut pictures)?);
                    run.clear();
                }
                let Some((kind, symbol)) = kind else {
                    // Anything else that has behaviors is a picture of its own, drawn as it stands at rest.
                    let mut at_rest = object.clone();
                    at_rest.behaviors.clear();
                    if let Some(sprite) = pictures.sprite(&[at_rest])? {
                        ops.push(format!("[\"m\",{},{},{}]", sprite_text(Some(sprite)), object.id, behaving.unwrap_or_default()));
                    }
                    continue;
                };
                let behaving = behaving.map_or_else(String::new, |text| format!(",{text}"));
                // The copy's own box is filled by the symbol's first frame, then turned or slanted as the copy is.
                let natural = doc.symbols.iter().find(|s| s.id == symbol).and_then(|s| s.objects.iter().map(|o| o.bounds).reduce(Rect::union)).unwrap_or_default();
                let frame_box = object.placement.map_or(object.bounds, |p| p.frame);
                let fit = Warp::fit(Rect { x: 0.0, y: 0.0, w: natural.w, h: natural.h }, frame_box);
                let m = object.placement.map_or(fit, |p| fit.then(&p.warp)).affine();
                let matrix = format!("[{},{},{},{},{},{}]", m[0], m[1], m[2], m[3], m[4], m[5]);
                let b = object.bounds;
                ops.push(match kind {
                    Moving::Clip(set) => format!("[\"c\",{set},{matrix},{},{}{behaving}]", object.opacity, object.id),
                    Moving::Button(set) => format!("[\"b\",{set},{matrix},{},{},{},[{},{},{},{}]{behaving}]", object.opacity, object.id, serde_json::to_string(&object.link)?, b.x, b.y, b.w, b.h),
                });
            }
            if !run.is_empty() {
                ops.push(still(&run, &mut pictures)?);
            }
            // The action and the sound of the keyframes that start on this frame; the top layer's come first.
            let starting = || (0..timeline.layers.len()).rev().filter_map(|layer| timeline.layers[layer].iter().find(|span| span.start == frame));
            let action = starting().map(|span| span.action.as_str()).find(|action| !action.is_empty()).unwrap_or("");
            let sound = starting().find_map(|span| span.sound).and_then(|id| doc.sounds.iter().position(|s| s.id == id)).map_or(-1, |at| at as i64);
            frame_texts.push(format!("{{\"o\":[{}],\"a\":{},\"s\":{sound}}}", ops.join(","), serde_json::to_string(action)?));
        }
        scene_texts.push(format!("[{}]", frame_texts.join(",")));
    }
    let background = first.canvas.map_or_else(|| "null".to_string(), |c| format!("\"rgba({},{},{},{})\"", c.r, c.g, c.b, c.a as f32 / 255.0));
    let sounds: Vec<String> = doc.sounds.iter().map(|sound| format!("\"data:{};base64,{}\"", sound.mime, sound.data)).collect();
    Ok(PAGE
        .replace("{title}", &escape(title))
        .replace("{width}", &width.to_string())
        .replace("{height}", &height.to_string())
        .replace("{fps}", &fps.to_string())
        .replace("{background}", &background)
        .replace("{pictures}", &pictures.sources.join(","))
        .replace("{sets}", &sets.join(","))
        .replace("{sounds}", &sounds.join(","))
        .replace("{scenes}", &scene_texts.join(",")))
}

/// The page: the stage, the animation's data, and the script that plays it.
const PAGE: &str = r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="generator" content="PisiSahne">
<title>{title}</title>
<style>
  html, body { margin: 0; height: 100%; background: #222; }
  body { display: flex; align-items: center; justify-content: center; }
  canvas { max-width: 100%; max-height: 100%; background: #fff; }
</style>
</head>
<body>
<canvas id="stage" width="{width}" height="{height}"></canvas>
<script>
// Made with PisiSahne.
// pictures: every picture of the animation, each stored once.
// sets: the pictures of each movie clip (one for each of its frames) and of each button (up, over, down).
// scenes: for each scene its frames; a frame has what to draw (o), an action (a) and a sound (s).
// What is drawn is a picture of the stage ("i"), a movie clip ("c"), a button ("b") or an object
// with behaviors ("m"); the last three may carry behaviors: the point to turn around (p), the
// object's box (box) and the list (l) of [kind, first number, second number].
var movie = {
  width: {width}, height: {height}, fps: {fps}, background: {background},
  pictures: [{pictures}],
  sets: [{sets}],
  sounds: [{sounds}],
  scenes: [{scenes}]
};
var canvas = document.getElementById("stage"), stage = canvas.getContext("2d");
var scene = 0, frame = 0, playing = true, depth = 0;
var clipFrames = {}, buttons = [], hover = null, pressed = false, loaded = 0;
// For each object with behaviors: how long it has been there (n, in frames), how far the viewer
// has moved it (x, y), how fast it is falling (vy) and whether it stands on the ground.
var actors = {}, keys = {}, mouse = null, grips = [], dragging = null;
var sounds = movie.sounds.map(function (source) { return new Audio(source); });
var images = movie.pictures.map(function (source) {
  var image = new Image();
  image.onload = function () { loaded += 1; if (loaded === movie.pictures.length) start(); };
  image.src = source;
  return image;
});
if (movie.pictures.length === 0) start();

// Goes to a frame of a scene, both counted from 0, playing on from there or stopping.
function go(toScene, toFrame, play) {
  if (toScene < 0 || toScene >= movie.scenes.length) return;
  scene = toScene;
  frame = Math.max(0, Math.min(toFrame, movie.scenes[scene].length - 1));
  playing = play;
  enter();
}
// Does what an action says. Frames and scenes are written counted from 1.
function act(action) {
  if (!action) return;
  if (action === "action:stop") playing = false;
  else if (action === "action:play") playing = true;
  else if (action.indexOf("action:goto-play:") === 0) go(scene, Number(action.slice(17)) - 1, true);
  else if (action.indexOf("action:goto-stop:") === 0) go(scene, Number(action.slice(17)) - 1, false);
  else if (action.indexOf("action:scene:") === 0) go(Number(action.slice(13)) - 1, 0, true);
  else if (action.indexOf("action:") !== 0) window.open(action, "_blank");
}
// On reaching a frame its sound starts and its action is done. An action that leads
// back to itself is followed only a few times, so that it cannot hold the page up.
function enter() {
  var now = movie.scenes[scene][frame];
  if (now.s >= 0 && sounds[now.s]) { try { sounds[now.s].currentTime = 0; sounds[now.s].play(); } catch (error) {} }
  depth += 1;
  if (depth < 8) act(now.a);
}
// The behaviors an op carries, and the number its object is known by.
function behaviorsOf(op) { return op[0] === "m" ? op[3] : op[0] === "c" ? op[5] : op[0] === "b" ? op[7] : null; }
function keyOf(op) { return op[0] === "m" ? op[2] : op[4]; }
function actor(key) { return actors[key] || (actors[key] = { n: 0, x: 0, y: 0, vy: 0, ground: false }); }
function span(seconds) { return Math.abs(seconds) < 0.01 ? 0.01 : seconds; }
function wrapped(at, size, room) { var m = room + size; return (((at + size) % m) + m) % m - size; }
function has(b, kind) { return b.l.some(function (l) { return l[0] === kind; }); }
// What its behaviors have done to an object by now: moved it (dx, dy), turned it (angle) and
// resized it (scale) around its point, and let it through by a part (alpha). The first nine
// kinds follow the same rules as playing inside PisiSahne does.
function moved(key, b) {
  var a = actor(key), t = a.n / movie.fps, box = b.box, TAU = Math.PI * 2;
  var m = { dx: a.x, dy: a.y, angle: 0, scale: 1, alpha: 1 };
  b.l.forEach(function (l) {
    var k = l[0], p = l[1], q = l[2];
    if (k === "spin") m.angle += TAU * t / span(p);
    else if (k === "bounce") m.dy -= p * Math.abs(Math.sin(Math.PI * t / span(q)));
    else if (k === "swing") m.dx += p * (1 - Math.cos(TAU * t / span(q))) / 2;
    else if (k === "rise") m.dy -= p * (1 - Math.cos(TAU * t / span(q))) / 2;
    else if (k === "drift") {
      var turn = q * Math.PI / 180;
      m.dx += wrapped(box[0] + p * Math.cos(turn) * t, box[2], movie.width) - box[0];
      m.dy += wrapped(box[1] - p * Math.sin(turn) * t, box[3], movie.height) - box[1];
    }
    else if (k === "shake") { m.dx += p * Math.sin(a.n * 12.9898); m.dy += p * Math.cos(a.n * 78.233); }
    else if (k === "pulse") m.scale *= 1 + p / 100 * (1 - Math.cos(TAU * t / span(q))) / 2;
    else if (k === "blink") { var one = Math.abs(span(p)); if (((t % one) + one) % one >= one / 2) m.alpha = 0; }
    else if (k === "fade") m.alpha *= (1 + Math.cos(TAU * t / span(p))) / 2;
  });
  return m;
}
// One frame's worth of what the viewer does to an object: the mouse draws it on, the arrow
// keys walk it, the ground pulls it down and Up or Space makes it jump.
function step(key, b) {
  var a = actor(key), dt = 1 / movie.fps, box = b.box;
  a.n += 1;
  if (dragging && dragging.key === key) return;
  var falls = has(b, "gravity");
  b.l.forEach(function (l) {
    var k = l[0], p = l[1], q = l[2];
    if (k === "mouse" && mouse) {
      var dx = mouse.x - (box[0] + box[2] / 2 + a.x), dy = mouse.y - (box[1] + box[3] / 2 + a.y);
      var far = Math.sqrt(dx * dx + dy * dy), go = p <= 0 ? far : Math.min(far, p * dt);
      if (far > 0.5) { a.x += dx / far * go; a.y += dy / far * go; }
    } else if (k === "keys") {
      if (keys.ArrowLeft) a.x -= p * dt;
      if (keys.ArrowRight) a.x += p * dt;
      if (!falls && keys.ArrowUp) a.y -= p * dt;
      if (!falls && keys.ArrowDown) a.y += p * dt;
      a.x = Math.max(-box[0], Math.min(a.x, movie.width - box[0] - box[2]));
      a.y = Math.max(-box[1], Math.min(a.y, movie.height - box[1] - box[3]));
    } else if (k === "gravity") {
      if (a.ground && q > 0 && (keys.ArrowUp || keys[" "])) { a.vy = -q; a.ground = false; }
      a.vy += p * dt;
      a.y += a.vy * dt;
      var floor = movie.height - box[1] - box[3];
      if (a.y >= floor) { a.y = floor; a.vy = 0; a.ground = true; } else a.ground = false;
    }
  });
}
// Sets the stage to draw an object as its behaviors have it.
function place(m, b) {
  var cos = Math.cos(m.angle) * m.scale, sin = Math.sin(m.angle) * m.scale, px = b.p[0], py = b.p[1];
  stage.setTransform(cos, sin, -sin, cos, px + m.dx - (cos * px - sin * py), py + m.dy - (sin * px + cos * py));
}
function draw() {
  stage.setTransform(1, 0, 0, 1, 0, 0);
  stage.globalAlpha = 1;
  stage.clearRect(0, 0, movie.width, movie.height);
  if (movie.background) { stage.fillStyle = movie.background; stage.fillRect(0, 0, movie.width, movie.height); }
  buttons = [];
  grips = [];
  movie.scenes[scene][frame].o.forEach(function (op) {
    if (op[0] === "i") {
      stage.setTransform(1, 0, 0, 1, 0, 0);
      stage.globalAlpha = 1;
      stage.drawImage(images[op[1]], 0, 0);
      return;
    }
    var b = behaviorsOf(op), m = b ? moved(keyOf(op), b) : null;
    if (b && has(b, "drag")) grips.push({ key: keyOf(op), box: [b.box[0] + m.dx, b.box[1] + m.dy, b.box[2], b.box[3]] });
    if (op[0] === "m") {
      place(m, b);
      stage.globalAlpha = m.alpha;
      stage.drawImage(images[op[1][0]], op[1][1], op[1][2]);
      return;
    }
    var set = movie.sets[op[1]], picture;
    if (op[0] === "c") {
      picture = set[(clipFrames[op[4]] || 0) % set.length];
    } else {
      var box = op[6];
      buttons.push({ key: op[4], action: op[5], box: m ? [box[0] + m.dx, box[1] + m.dy, box[2], box[3]] : box });
      picture = set[hover === op[4] ? (pressed ? 2 : 1) : 0] || set[0];
    }
    if (!picture) return;
    var matrix = op[2];
    if (m) { place(m, b); stage.transform(matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5]); }
    else stage.setTransform(matrix[0], matrix[1], matrix[2], matrix[3], matrix[4], matrix[5]);
    stage.globalAlpha = op[3] * (m ? m.alpha : 1);
    stage.drawImage(images[picture[0]], picture[1], picture[2]);
  });
}
function tick() {
  // Every movie clip on the stage goes on by a frame of its own, whether or not the scene is playing.
  movie.scenes[scene][frame].o.forEach(function (op) { if (op[0] === "c") clipFrames[op[4]] = (clipFrames[op[4]] || 0) + 1; });
  // So does every object with behaviors.
  movie.scenes[scene][frame].o.forEach(function (op) { var b = behaviorsOf(op); if (b) step(keyOf(op), b); });
  if (playing) {
    frame += 1;
    if (frame >= movie.scenes[scene].length) { scene = (scene + 1) % movie.scenes.length; frame = 0; }
    depth = 0;
    enter();
  }
  draw();
}
function start() {
  depth = 0;
  enter();
  draw();
  setInterval(tick, 1000 / movie.fps);
}
// Where the mouse is on the stage.
function pointOf(event) {
  var box = canvas.getBoundingClientRect();
  return { x: (event.clientX - box.left) * movie.width / box.width, y: (event.clientY - box.top) * movie.height / box.height };
}
// The topmost of some boxes that holds a point.
function topmost(list, at) {
  for (var i = list.length - 1; i >= 0; i -= 1) {
    var b = list[i].box;
    if (at.x >= b[0] && at.y >= b[1] && at.x <= b[0] + b[2] && at.y <= b[1] + b[3]) return list[i];
  }
  return null;
}
canvas.addEventListener("mousemove", function (event) {
  var at = pointOf(event);
  if (dragging) {
    var a = actor(dragging.key);
    a.x += at.x - mouse.x; a.y += at.y - mouse.y; a.vy = 0;
  }
  mouse = at;
  var over = topmost(buttons, at);
  hover = over ? over.key : null;
  canvas.style.cursor = over ? "pointer" : dragging ? "grabbing" : topmost(grips, at) ? "grab" : "default";
  draw();
});
canvas.addEventListener("mouseleave", function () { hover = null; pressed = false; dragging = null; draw(); });
canvas.addEventListener("mousedown", function (event) {
  pressed = true;
  mouse = pointOf(event);
  dragging = topmost(buttons, mouse) ? null : topmost(grips, mouse);
  draw();
});
canvas.addEventListener("mouseup", function (event) {
  pressed = false;
  dragging = null;
  var over = topmost(buttons, pointOf(event));
  if (over) { depth = 0; act(over.action); }
  draw();
});
// The keys held down. In a film that listens to them the arrows and Space do not scroll the page.
var listens = JSON.stringify(movie.scenes).search(/\["(keys|gravity)",/) >= 0;
function keyed(event, down) {
  keys[event.key] = down;
  if (listens && (event.key.indexOf("Arrow") === 0 || event.key === " ")) event.preventDefault();
}
window.addEventListener("keydown", function (event) { keyed(event, true); });
window.addEventListener("keyup", function (event) { keyed(event, false); });
</script>
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Action, Color, Document, Editor, Stroke, Tween, WebKind};

    fn boxed(id: u64, x: f32, y: f32, w: f32, h: f32, color: Color) -> Object {
        let mut object = Object::new_web(id, WebKind::Slice, Rect { x, y, w, h });
        (object.shape, object.fill, object.stroke) = (Shape::Rect { radius: 0.0 }, Some(color), None::<Stroke>);
        object
    }

    #[test]
    fn an_animation_is_published_as_one_page_with_its_frames() {
        let mut e = Editor::new((crate::examples::EXAMPLES[0].build)());
        e.doc.page_mut().start_timeline(12.0);
        // The star's layer gets a second keyframe on frame 4, the star moved, and a tween between the two.
        e.doc.page_mut().active_layer = 0;
        e.goto_frame(3);
        e.on_timeline(|timeline, layer, frame| timeline.insert_keyframe(layer, frame, false));
        let star = e.doc.page().layers[0].objects[1].id;
        e.selection = vec![star];
        e.for_selected(|o| o.translate(300.0, 0.0));
        e.goto_frame(0);
        e.on_timeline(|timeline, layer, frame| timeline.set_tween(layer, frame, Some(Tween::default())));
        // The other layers last one frame; made to last four, they show throughout.
        for layer in 1..e.doc.page().layers.len() {
            e.doc.page_mut().edit_timeline(|timeline| timeline.insert_frames(layer, 0, 3));
        }
        let html = html(&e.doc, "Bahar <Şenliği>").unwrap();
        assert!(html.contains("<title>Bahar &lt;Şenliği&gt;</title>") && html.contains("width=\"728\" height=\"90\""));
        assert!(html.contains("fps: 12,"));
        // Four frames, each one picture of the whole stage, all different.
        assert_eq!(html.matches("{\"o\":[[\"i\",").count(), 4);
        assert_eq!(html.matches("data:image/png;base64,").count(), 4);
        // A scene that stands still is one picture, however long it lasts.
        let mut still = Editor::new(Document::new(20, 10));
        still.doc.page_mut().start_timeline(24.0);
        still.add_object(boxed(1, 2.0, 2.0, 5.0, 5.0, Color::BLACK));
        still.doc.page_mut().edit_timeline(|timeline| timeline.insert_frames(0, 0, 9));
        let html = super::html(&still.doc, "x").unwrap();
        assert_eq!((html.matches("{\"o\":[[\"i\",0]]").count(), html.matches("data:image/png").count()), (10, 1));
        assert!(super::html(&Document::new(20, 10), "x").is_err(), "a document with no timeline is no animation");
    }

    #[test]
    fn clips_buttons_actions_sounds_and_scenes_reach_the_page() {
        let mut e = Editor::new(Document::new(200, 100));
        e.doc.page_mut().start_timeline(10.0);
        // A movie clip of three frames: a box that moves right.
        e.add_object(boxed(1, 10.0, 10.0, 20.0, 20.0, Color::rgb(255, 0, 0)));
        e.doc.next_id = 50;
        e.selection = vec![1];
        let clip = e.make_symbol().unwrap();
        let clip_copy = e.selection[0];
        e.doc.symbols[0].kind = SymbolKind::Animation;
        e.open_clip(clip);
        e.goto_frame(2);
        e.on_timeline(|timeline, layer, frame| timeline.insert_keyframe(layer, frame, false));
        let part = e.doc.page().objects().next().unwrap().id;
        e.selection = vec![part];
        e.for_selected(|o| o.translate(30.0, 0.0));
        e.goto_frame(0);
        e.on_timeline(|timeline, layer, frame| timeline.set_tween(layer, frame, Some(Tween::default())));
        e.leave_clip();
        // A button with an Over look, which stops the animation when clicked.
        let button = e.new_symbol("Dur", SymbolKind::Button);
        e.edit_symbol(button, None, true);
        e.add_object(boxed(0, 100.0, 60.0, 40.0, 20.0, Color::rgb(0, 0, 255)));
        e.set_symbol_look(1);
        e.copy_previous_look();
        e.for_selected(|o| o.fill = Some(Color::rgb(0, 255, 0)));
        e.leave_symbol();
        let button_copy = e.selection[0];
        e.for_selected(|o| o.link = Action::Stop.text());
        // The scene lasts five frames; the last sends it back to frame 2 and rings a bell.
        e.goto_frame(4);
        e.on_timeline(|timeline, layer, frame| timeline.insert_keyframe(layer, frame, false));
        let bell = e.add_sound("zil.wav", "audio/wav", b"RIFF");
        e.on_timeline(|timeline, layer, frame| {
            let at = timeline.span_at(layer, frame).unwrap();
            (timeline.layers[layer][at].action, timeline.layers[layer][at].sound) = (Action::GotoPlay(2).text(), Some(bell));
        });
        // A second scene with one still frame.
        e.add_page();
        e.add_object(boxed(0, 0.0, 0.0, 10.0, 10.0, Color::BLACK));
        e.set_active_page(0);
        let html = html(&e.doc, "film").unwrap();
        // The clip and the button are kept apart from the still picture, each with its matrix and number.
        assert!(html.contains(&format!("[\"c\",0,[1,0,0,1,10,10],1,{clip_copy}]")), "{}", &html[html.find("scenes:").unwrap()..][..400]);
        assert!(html.contains(&format!("[\"b\",1,[1,0,0,1,100,60],1,{button_copy},\"action:stop\",[100,60,40,20]]")));
        // The clip's set has its three frames, the second drawn 15 further right; the button's has up and over.
        let sets = &html[html.find("sets: [").unwrap()..html.find("sounds: [").unwrap()];
        assert_eq!(sets.matches(",0,0]").count() + sets.matches(",15,0]").count() + sets.matches(",30,0]").count(), 5, "{sets}");
        assert!(sets.contains(",15,0]") && sets.contains(",30,0]"));
        // The last frame of the first scene has its action and its sound; there are two scenes and one sound.
        assert!(html.contains("\"a\":\"action:goto-play:2\",\"s\":0}"));
        assert!(html.contains("sounds: [\"data:audio/wav;base64,UklGRg==\"]"));
        let scenes = &html[html.find("scenes: [").unwrap()..];
        assert_eq!(scenes.matches("\"a\":").count(), 6, "five frames and one");
        // The page is left among the build's own files, to be looked at in a browser.
        let _ = std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/target/pisisahne-test.html"), &html);

        // A clip within a clip keeps playing: the outer clip, of three frames, holds a copy of an inner one of two,
        // so the outer comes round after six frames, and its pictures show the inner one at both of its frames.
        let inner = {
            e.add_object(boxed(0, 0.0, 0.0, 10.0, 10.0, Color::BLACK));
            let id = e.make_symbol().unwrap();
            e.doc.symbols.iter_mut().find(|s| s.id == id).unwrap().kind = SymbolKind::Animation;
            e.open_clip(id);
            e.goto_frame(1);
            e.on_timeline(|timeline, layer, frame| timeline.insert_keyframe(layer, frame, false));
            let part = e.doc.page().objects().next().unwrap().id;
            e.selection = vec![part];
            e.for_selected(|o| o.translate(0.0, 7.0));
            e.leave_clip();
            let copy = e.selection.first().copied();
            e.selection = copy.into_iter().collect();
            e.delete_selected();
            id
        };
        e.open_clip(clip);
        e.goto_frame(0);
        e.place_symbol_at(inner, crate::model::Vec2 { x: 40.0, y: 80.0 });
        e.leave_clip();
        assert_eq!((clip_period(&e.doc, inner, 0), clip_period(&e.doc, clip, 0)), (2, 6));
        let lowest = |tick: usize| clip_frame(&e.doc, clip, tick, 0).iter().map(|o| o.bounds.bottom()).fold(0.0, f32::max);
        assert_eq!(lowest(1) - lowest(0), 7.0, "the inner clip has moved on by a frame of its own");
        // Back on its own first frame after three, the outer clip shows the inner one on its second frame;
        // after six, both are where they started.
        assert_eq!(lowest(3) - lowest(0), 7.0);
        assert_eq!(lowest(6), lowest(0));
        // A copy of the clip with a live filter gets pictures of its own, larger by the filter's reach.
        let plain = sprite_sizes(&super::html(&e.doc, "x").unwrap());
        e.selection = vec![clip_copy];
        e.for_selected(|o| o.filters.push(crate::effects::LiveFilter::new(crate::effects::FilterKind::Glow)));
        let glowing = sprite_sizes(&super::html(&e.doc, "x").unwrap());
        assert!(glowing > plain, "{glowing} pictures' worth against {plain}");
    }

    /// How much picture data a page carries: a measure that grows when pictures do.
    fn sprite_sizes(html: &str) -> usize {
        html.len()
    }
}
