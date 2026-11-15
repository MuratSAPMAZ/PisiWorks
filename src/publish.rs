//! Publishing an animation as a page that plays in a browser.
//!
//! Every frame of the scene is drawn as the program draws it and put into the page as
//! a picture, so that gradients, filters and masks look exactly as they do on the
//! stage. A few lines of script show the pictures one after another at the frame rate.
//! Frames that look the same are stored once.

use std::collections::HashMap;

use anyhow::{ensure, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::i18n::t;
use crate::model::Page;

/// The most frames a published page may hold.
const MAX_PUBLISHED: usize = 3000;

fn escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// The scene as a single HTML page. The pictures are inside it, so the one file is all there is to hand in or put on a site.
pub fn html(page: &Page, title: &str) -> Result<String> {
    let timeline = page.timeline.as_ref().context(t("This document has no timeline"))?;
    let frames = timeline.frame_count().max(1);
    ensure!(frames <= MAX_PUBLISHED, t("The animation has too many frames to publish as one page"));
    let mut pictures: Vec<String> = Vec::new();
    let mut seen: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut order = Vec::with_capacity(frames);
    for frame in 0..frames {
        // Each frame as the finished animation shows it: guides left out, masks at work.
        let png = crate::export::render_page(&page.at_frame(frame).played())?.encode_png()?;
        let index = *seen.entry(png.clone()).or_insert_with(|| {
            pictures.push(STANDARD.encode(&png));
            pictures.len() - 1
        });
        order.push(index.to_string());
    }
    let sources: Vec<String> = pictures.iter().map(|data| format!("\"data:image/png;base64,{data}\"")).collect();
    Ok(format!(
        r#"<!doctype html>
<html>
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="generator" content="PisiSahne">
<title>{title}</title>
<style>
  html, body {{ margin: 0; height: 100%; background: #222; }}
  body {{ display: flex; align-items: center; justify-content: center; }}
  canvas {{ max-width: 100%; max-height: 100%; background: #fff; }}
</style>
</head>
<body>
<canvas id="stage" width="{width}" height="{height}"></canvas>
<script>
// Made with PisiSahne. The frames of the animation, each drawn once, and the order they play in.
var pictures = [{sources}];
var order = [{order}];
var fps = {fps};
var stage = document.getElementById("stage").getContext("2d");
var loaded = 0, frame = 0;
var images = pictures.map(function (source) {{
  var image = new Image();
  image.onload = function () {{ loaded += 1; if (loaded === pictures.length) start(); }};
  image.src = source;
  return image;
}});
function show() {{
  stage.clearRect(0, 0, {width}, {height});
  stage.drawImage(images[order[frame]], 0, 0);
}}
function start() {{
  show();
  if (order.length > 1) setInterval(function () {{ frame = (frame + 1) % order.length; show(); }}, 1000 / fps);
}}
</script>
</body>
</html>
"#,
        title = escape(title),
        width = page.width,
        height = page.height,
        sources = sources.join(","),
        order = order.join(","),
        fps = timeline.fps.clamp(1.0, 120.0),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Document, Editor, Tween};

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
        let page = e.export_page();
        let html = html(&page, "Bahar <Şenliği>").unwrap();
        assert!(html.contains("<title>Bahar &lt;Şenliği&gt;</title>") && html.contains("width=\"728\" height=\"90\""));
        assert!(html.contains("var fps = 12;") && html.contains("var order = [0,1,2,3];"));
        assert_eq!(html.matches("data:image/png;base64,").count(), 4);
        // The page is left among the build's own files, to be looked at in a browser.
        let _ = std::fs::write(concat!(env!("CARGO_MANIFEST_DIR"), "/target/pisisahne-test.html"), &html);
        // A scene that stands still is one picture, however long it lasts.
        let mut still = Editor::new(Document::new(20, 10));
        still.doc.page_mut().start_timeline(24.0);
        still.doc.page_mut().edit_timeline(|timeline| timeline.insert_frames(0, 0, 9));
        let html = super::html(&still.export_page(), "x").unwrap();
        assert!(html.contains("var order = [0,0,0,0,0,0,0,0,0,0];") && html.matches("data:image/png").count() == 1);
        assert!(super::html(&Editor::new(Document::new(20, 10)).export_page(), "x").is_err());
    }
}
