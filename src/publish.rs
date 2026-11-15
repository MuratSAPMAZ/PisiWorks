//! Publishing an animation as a page that plays in a browser.
//!
//! The page carries the whole animation in itself: one file to hand in or put on a
//! site. What stands still in a frame is drawn as the program draws it and stored as
//! a picture, so gradients, filters and masks look exactly as they do on the stage;
//! pictures that are the same are stored once. Movie clips and buttons are kept
//! apart as pictures of their own, so that a clip can play by its own count of frames
//! and a button can answer the mouse. A short script plays the scenes one after
//! another, runs the actions of frames and buttons, and starts the sounds.

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
    let mut set_of: HashMap<u64, Option<usize>> = HashMap::new();
    let mut moving = |symbol: u64, pictures: &mut Pictures, sets: &mut Vec<String>| -> Result<Option<Moving>> {
        let Some(entry) = doc.symbols.iter().find(|s| s.id == symbol) else { return Ok(None) };
        let known = set_of.get(&symbol).copied();
        let made = match (entry.kind, known) {
            (SymbolKind::Graphic, _) => return Ok(None),
            (_, Some(set)) => set,
            (SymbolKind::Animation, None) => {
                // A clip's frames, each drawn against the top left of its first frame.
                let frames = doc.clip_page(symbol).map(|index| &doc.pages[index]).and_then(|page| {
                    let count = page.timeline.as_ref()?.frame_count();
                    (count > 1).then_some((page, count))
                });
                let set = match frames {
                    None => None,
                    Some((page, count)) => {
                        let shown = |frame: usize| -> Vec<Object> { page.at_frame(frame).played().objects().filter(|o| o.visible && !o.is_web()).cloned().collect() };
                        let origin = shown(0).iter().map(|o| o.bounds).reduce(Rect::union).unwrap_or_default();
                        let mut texts = Vec::with_capacity(count);
                        for frame in 0..count.min(MAX_PUBLISHED) {
                            let mut objects = shown(frame);
                            for object in &mut objects {
                                object.translate(-origin.x, -origin.y);
                            }
                            texts.push(sprite_text(pictures.sprite(&objects)?));
                        }
                        sets.push(format!("[{}]", texts.join(",")));
                        Some(sets.len() - 1)
                    }
                };
                set_of.insert(symbol, set);
                set
            }
            (SymbolKind::Button, None) => {
                // Up, Over and Down, each placed against the Up look.
                let mut texts = vec![sprite_text(pictures.sprite(&entry.objects)?)];
                for look in entry.looks.iter().take(2) {
                    texts.push(if look.is_empty() { "null".to_string() } else { sprite_text(pictures.sprite(look)?) });
                }
                sets.push(format!("[{}]", texts.join(",")));
                set_of.insert(symbol, Some(sets.len() - 1));
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
                    Shape::Instance { symbol, .. } => moving(symbol, &mut pictures, &mut sets)?.map(|kind| (kind, symbol)),
                    _ => None,
                };
                let Some((kind, symbol)) = kind else {
                    run.push(object.clone());
                    continue;
                };
                if !run.is_empty() {
                    ops.push(still(&run, &mut pictures)?);
                    run.clear();
                }
                // The copy's own box is filled by the symbol's first frame, then turned or slanted as the copy is.
                let natural = doc.symbols.iter().find(|s| s.id == symbol).and_then(|s| s.objects.iter().map(|o| o.bounds).reduce(Rect::union)).unwrap_or_default();
                let frame_box = object.placement.map_or(object.bounds, |p| p.frame);
                let fit = Warp::fit(Rect { x: 0.0, y: 0.0, w: natural.w, h: natural.h }, frame_box);
                let m = object.placement.map_or(fit, |p| fit.then(&p.warp)).affine();
                let matrix = format!("[{},{},{},{},{},{}]", m[0], m[1], m[2], m[3], m[4], m[5]);
                let b = object.bounds;
                ops.push(match kind {
                    Moving::Clip(set) => format!("[\"c\",{set},{matrix},{},{}]", object.opacity, object.id),
                    Moving::Button(set) => format!("[\"b\",{set},{matrix},{},{},{},[{},{},{},{}]]", object.opacity, object.id, serde_json::to_string(&object.link)?, b.x, b.y, b.w, b.h),
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
function draw() {
  stage.setTransform(1, 0, 0, 1, 0, 0);
  stage.globalAlpha = 1;
  stage.clearRect(0, 0, movie.width, movie.height);
  if (movie.background) { stage.fillStyle = movie.background; stage.fillRect(0, 0, movie.width, movie.height); }
  buttons = [];
  movie.scenes[scene][frame].o.forEach(function (op) {
    if (op[0] === "i") {
      stage.setTransform(1, 0, 0, 1, 0, 0);
      stage.globalAlpha = 1;
      stage.drawImage(images[op[1]], 0, 0);
      return;
    }
    var set = movie.sets[op[1]], picture;
    if (op[0] === "c") {
      picture = set[(clipFrames[op[4]] || 0) % set.length];
    } else {
      buttons.push({ key: op[4], action: op[5], box: op[6] });
      picture = set[hover === op[4] ? (pressed ? 2 : 1) : 0] || set[0];
    }
    if (!picture) return;
    var m = op[2];
    stage.setTransform(m[0], m[1], m[2], m[3], m[4], m[5]);
    stage.globalAlpha = op[3];
    stage.drawImage(images[picture[0]], picture[1], picture[2]);
  });
}
function tick() {
  // Every movie clip on the stage goes on by a frame of its own, whether or not the scene is playing.
  movie.scenes[scene][frame].o.forEach(function (op) { if (op[0] === "c") clipFrames[op[4]] = (clipFrames[op[4]] || 0) + 1; });
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
// The button under the mouse: the topmost one whose box holds the point.
function buttonAt(event) {
  var box = canvas.getBoundingClientRect();
  var x = (event.clientX - box.left) * movie.width / box.width, y = (event.clientY - box.top) * movie.height / box.height;
  for (var i = buttons.length - 1; i >= 0; i -= 1) {
    var b = buttons[i].box;
    if (x >= b[0] && y >= b[1] && x <= b[0] + b[2] && y <= b[1] + b[3]) return buttons[i];
  }
  return null;
}
canvas.addEventListener("mousemove", function (event) {
  var over = buttonAt(event);
  hover = over ? over.key : null;
  canvas.style.cursor = over ? "pointer" : "default";
  draw();
});
canvas.addEventListener("mouseleave", function () { hover = null; pressed = false; draw(); });
canvas.addEventListener("mousedown", function () { pressed = true; draw(); });
canvas.addEventListener("mouseup", function (event) {
  pressed = false;
  var over = buttonAt(event);
  if (over) { depth = 0; act(over.action); }
  draw();
});
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
    }
}
