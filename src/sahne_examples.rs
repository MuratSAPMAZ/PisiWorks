//! The examples that come with PisiSahne: short animations to open, play and take apart.
//!
//! Like PisiWorks' own, they are built here from the commands a user has, so their
//! texts follow the language of the interface and there are no files to install.

use crate::autoshape::AutoKind;
use crate::examples::{add, bold, boxed, centered, finish, gradient, layer, rect, Example, BLUE, INK, ORANGE, PALE, TEAL, WHITE, YELLOW};
use crate::i18n::t;
use crate::model::{Action, Color, Document, Editor, FillKind, LayerKind, Shape, Stroke, SymbolKind, TextStyle, Tween, Vec2};

pub const EXAMPLES: &[Example] = &[
    Example { name: "Bouncing ball", about: "Motion tweens with easing: the ball speeds up as it falls and slows as it rises", build: bouncing_ball },
    Example { name: "Changing shapes", about: "Shape tweens: a square becomes a circle, and the circle a star", build: changing_shapes },
    Example { name: "Along a path", about: "A guide layer: the arrow follows the path and turns to face the way it goes", build: along_a_path },
    Example { name: "Spotlight", about: "A mask layer: a moving circle lets the bright words show", build: spotlight },
    Example { name: "Turning gears", about: "A movie clip with a timeline of its own, placed twice on the stage", build: turning_gears },
    Example { name: "One after another", about: "Layers with times of their own: the car drives for four seconds and stops, then the sun rises, then the words appear", build: one_after_another },
    Example { name: "Two scenes with buttons", about: "Scenes, frame actions that stop the film, and buttons that lead from scene to scene", build: two_scenes },
];

fn start(width: u32, height: u32, first_layer: &str, fps: f32) -> Editor {
    let mut e = Editor::new(Document::new(width, height));
    e.doc.page_mut().layers[0].name = first_layer.to_string();
    e.doc.page_mut().start_timeline(fps);
    e
}

/// Makes every layer of the scene last as long as the longest one, so that what stands still shows throughout.
fn fill_out(e: &mut Editor) {
    e.doc.page_mut().edit_timeline(|timeline| {
        let frames = timeline.frame_count();
        for layer in 0..timeline.layers.len() {
            let own = timeline.layers[layer].last().map_or(0, |span| span.start + span.length);
            if own < frames {
                timeline.insert_frames(layer, own.saturating_sub(1), frames - own);
            }
        }
    });
    e.goto_frame(0);
}

/// A keyframe on a frame of the active layer, with the objects it starts with changed by `change`.
fn keyframe(e: &mut Editor, frame: usize, ids: &[u64], change: impl Fn(&mut crate::model::Object)) {
    e.goto_frame(frame);
    e.on_timeline(|timeline, layer, frame| timeline.insert_keyframe(layer, frame, false));
    e.selection = ids.to_vec();
    e.for_selected(change);
    e.selection.clear();
}

fn tween(e: &mut Editor, frame: usize, tween: Tween) {
    e.goto_frame(frame);
    e.on_timeline(|timeline, layer, frame| timeline.set_tween(layer, frame, Some(tween)));
}

fn bouncing_ball() -> Document {
    let mut e = start(480, 320, t("Background"), 24.0);
    let sky = boxed(&mut e, t("Sky"), rect(0.0, 0.0, 480.0, 320.0), 0.0, PALE);
    gradient(&mut e, sky, FillKind::Linear, WHITE, Color::rgb(0xc8, 0xdc, 0xf4), (0.5, 0.0), (0.5, 1.0));
    boxed(&mut e, t("Ground"), rect(0.0, 270.0, 480.0, 50.0), 0.0, TEAL);
    // The shadow grows as the ball comes down and shrinks as it goes up.
    layer(&mut e, t("Shadow"));
    let shadow = add(&mut e, t("Shadow"), Shape::Ellipse, rect(215.0, 276.0, 50.0, 10.0), Some(INK), None);
    e.for_selected(|o| o.opacity = 0.15);
    keyframe(&mut e, 12, &[shadow], |o| {
        o.set_bounds(rect(195.0, 272.0, 90.0, 18.0));
        o.opacity = 0.45;
    });
    keyframe(&mut e, 24, &[shadow], |o| {
        o.set_bounds(rect(215.0, 276.0, 50.0, 10.0));
        o.opacity = 0.15;
    });
    tween(&mut e, 0, Tween { ease: -100.0, ..Tween::default() });
    tween(&mut e, 12, Tween { ease: 100.0, ..Tween::default() });
    e.goto_frame(0);
    layer(&mut e, t("Ball"));
    let ball = add(&mut e, t("Ball"), Shape::Ellipse, rect(210.0, 30.0, 60.0, 60.0), Some(ORANGE), None);
    gradient(&mut e, ball, FillKind::Radial, YELLOW, ORANGE, (0.35, 0.35), (1.0, 0.5));
    // Down to the ground, squashed a little where it lands, and up again.
    keyframe(&mut e, 12, &[ball], |o| o.set_bounds(rect(204.0, 222.0, 72.0, 50.0)));
    keyframe(&mut e, 24, &[ball], |o| o.set_bounds(rect(210.0, 30.0, 60.0, 60.0)));
    tween(&mut e, 0, Tween { ease: -100.0, ..Tween::default() });
    tween(&mut e, 12, Tween { ease: 100.0, ..Tween::default() });
    fill_out(&mut e);
    finish(e)
}

fn changing_shapes() -> Document {
    let mut e = start(480, 300, t("Background"), 12.0);
    boxed(&mut e, t("Background"), rect(0.0, 0.0, 480.0, 300.0), 0.0, INK);
    centered(&mut e, t("Shape tween"), 240.0, 250.0, 18.0, PALE, TextStyle::default());
    layer(&mut e, t("Shape"));
    add(&mut e, t("Square"), Shape::Rect { radius: 0.0 }, rect(60.0, 90.0, 100.0, 100.0), Some(BLUE), None);
    // Each new shape is drawn on a blank keyframe: a shape tween changes one object into another.
    let blank = |e: &mut Editor, frame: usize| {
        e.goto_frame(frame);
        e.on_timeline(|timeline, layer, frame| timeline.insert_keyframe(layer, frame, true));
    };
    blank(&mut e, 15);
    add(&mut e, t("Circle"), Shape::Ellipse, rect(180.0, 80.0, 120.0, 120.0), Some(ORANGE), None);
    blank(&mut e, 30);
    add(&mut e, t("Star"), Shape::Auto { kind: AutoKind::Star, a: 0.45, b: 0.0, count: 5 }, rect(310.0, 70.0, 140.0, 140.0), Some(YELLOW), None);
    // The star stays for a while before the film starts over.
    e.on_timeline(|timeline, layer, frame| timeline.insert_frames(layer, frame, 8));
    tween(&mut e, 0, Tween { shape: true, ..Tween::default() });
    tween(&mut e, 15, Tween { shape: true, ease: 60.0, ..Tween::default() });
    fill_out(&mut e);
    finish(e)
}

fn along_a_path() -> Document {
    let mut e = start(520, 320, t("Background"), 24.0);
    boxed(&mut e, t("Background"), rect(0.0, 0.0, 520.0, 320.0), 0.0, PALE);
    layer(&mut e, t("Arrow"));
    let arrow = add(&mut e, t("Arrow"), Shape::Auto { kind: AutoKind::Arrow, a: 0.5, b: 0.5, count: 0 }, rect(20.0, 235.0, 60.0, 30.0), Some(BLUE), None);
    keyframe(&mut e, 47, &[arrow], |o| o.translate(420.0, 0.0));
    tween(&mut e, 0, Tween { orient: true, ..Tween::default() });
    e.goto_frame(0);
    // The path the arrow keeps to: up over a hill and down again. It is a guide, and is not part of the film.
    layer(&mut e, t("Guide"));
    let hill: Vec<Vec2> = (0..=24).map(|step| {
        let part = step as f32 / 24.0;
        Vec2 { x: 50.0 + part * 420.0, y: 250.0 - (part * std::f32::consts::PI).sin() * 170.0 }
    }).collect();
    let path = add(&mut e, t("Path"), Shape::Path { closed: false }, crate::model::Rect::of_points(&hill), None, Some(Stroke { color: INK, width: 1.0 }));
    if let Some(object) = e.object_mut(path) {
        object.points = hill;
    }
    e.doc.page_mut().edit_timeline(|timeline| {
        let top = timeline.layers.len() - 1;
        timeline.set_kind(top, LayerKind::Guide);
    });
    fill_out(&mut e);
    finish(e)
}

fn spotlight() -> Document {
    let mut e = start(520, 200, t("Background"), 24.0);
    boxed(&mut e, t("Background"), rect(0.0, 0.0, 520.0, 200.0), 0.0, INK);
    // The same words twice: dim underneath, bright on the layer that the mask lets through.
    centered(&mut e, "PisiSahne", 260.0, 60.0, 64.0, Color::rgb(0x34, 0x40, 0x5c), bold());
    layer(&mut e, t("Bright"));
    boxed(&mut e, t("Light"), rect(0.0, 0.0, 520.0, 200.0), 0.0, Color::rgb(0x2c, 0x3a, 0x58));
    centered(&mut e, "PisiSahne", 260.0, 60.0, 64.0, YELLOW, bold());
    layer(&mut e, t("Mask"));
    let light = add(&mut e, t("Spot"), Shape::Ellipse, rect(-20.0, 30.0, 140.0, 140.0), Some(WHITE), None);
    keyframe(&mut e, 30, &[light], |o| o.translate(420.0, 0.0));
    keyframe(&mut e, 60, &[light], |o| o.translate(-420.0, 0.0));
    tween(&mut e, 0, Tween { ease: 40.0, ..Tween::default() });
    tween(&mut e, 30, Tween { ease: 40.0, ..Tween::default() });
    e.doc.page_mut().edit_timeline(|timeline| {
        let top = timeline.layers.len() - 1;
        timeline.set_kind(top, LayerKind::Mask);
    });
    fill_out(&mut e);
    finish(e)
}

fn turning_gears() -> Document {
    let mut e = start(480, 300, t("Background"), 24.0);
    boxed(&mut e, t("Background"), rect(0.0, 0.0, 480.0, 300.0), 0.0, PALE);
    layer(&mut e, t("Gears"));
    // One gear, made a movie clip: on its own timeline it turns once round in two seconds.
    let gear = add(&mut e, t("Gear"), Shape::Auto { kind: AutoKind::Gear, a: 0.7, b: 0.0, count: 12 }, rect(90.0, 60.0, 180.0, 180.0), Some(TEAL), None);
    e.selection = vec![gear];
    let Some(symbol) = e.make_symbol() else { return finish(e) };
    let large = e.selection.first().copied();
    if let Some(entry) = e.doc.symbols.iter_mut().find(|s| s.id == symbol) {
        (entry.kind, entry.name) = (SymbolKind::Animation, t("Gear").to_string());
    }
    e.open_clip(symbol);
    let part = e.doc.page().objects().next().map(|o| o.id);
    keyframe(&mut e, 47, &part.into_iter().collect::<Vec<_>>(), |_| {});
    tween(&mut e, 0, Tween { turns: 1, ..Tween::default() });
    e.goto_frame(0);
    e.leave_clip();
    // A second, smaller copy of the same clip beside the first.
    e.place_symbol_at(symbol, Vec2 { x: 345.0, y: 150.0 });
    e.for_selected(|o| o.set_bounds(rect(285.0, 90.0, 120.0, 120.0)));
    e.selection = large.into_iter().collect();
    finish(e)
}

fn one_after_another() -> Document {
    // Twelve frames a second, eight seconds: every layer has its own keyframes, so each part has its own time.
    let mut e = start(520, 300, t("Background"), 12.0);
    let sky = boxed(&mut e, t("Sky"), rect(0.0, 0.0, 520.0, 300.0), 0.0, PALE);
    gradient(&mut e, sky, FillKind::Linear, Color::rgb(0xc8, 0xdc, 0xf4), WHITE, (0.5, 0.0), (0.5, 1.0));
    // A blank keyframe on a later frame: until then the layer shows nothing.
    let blank = |e: &mut Editor, frame: usize| {
        e.goto_frame(frame);
        e.on_timeline(|timeline, layer, frame| timeline.insert_keyframe(layer, frame, true));
    };
    // The sun is not there for the first four seconds. Then it rises for two, and stays.
    layer(&mut e, t("Sun"));
    blank(&mut e, 48);
    let sun = add(&mut e, t("Sun"), Shape::Ellipse, rect(380.0, 240.0, 80.0, 80.0), Some(YELLOW), None);
    gradient(&mut e, sun, FillKind::Radial, YELLOW, ORANGE, (0.5, 0.5), (1.0, 0.5));
    keyframe(&mut e, 72, &[sun], |o| o.translate(0.0, -200.0));
    tween(&mut e, 48, Tween { ease: 100.0, ..Tween::default() });
    e.goto_frame(0);
    layer(&mut e, t("Ground"));
    boxed(&mut e, t("Ground"), rect(0.0, 230.0, 520.0, 70.0), 0.0, TEAL);
    // The car drives for the first four seconds. Its last keyframe has no tween: it stands still to the end.
    layer(&mut e, t("Car"));
    let body = boxed(&mut e, t("Car"), rect(20.0, 186.0, 110.0, 34.0), 8.0, BLUE);
    let top = boxed(&mut e, t("Car"), rect(42.0, 162.0, 60.0, 30.0), 8.0, BLUE);
    let back = add(&mut e, t("Wheel"), Shape::Ellipse, rect(34.0, 208.0, 28.0, 28.0), Some(INK), None);
    let front = add(&mut e, t("Wheel"), Shape::Ellipse, rect(90.0, 208.0, 28.0, 28.0), Some(INK), None);
    keyframe(&mut e, 48, &[body, top, back, front], |o| o.translate(250.0, 0.0));
    tween(&mut e, 0, Tween { ease: 100.0, ..Tween::default() });
    e.goto_frame(0);
    // The words come last: from the sixth second they fade in for a second.
    layer(&mut e, t("Words"));
    blank(&mut e, 72);
    let words = centered(&mut e, t("Good morning!"), 200.0, 60.0, 40.0, INK, bold());
    e.selection = vec![words];
    e.for_selected(|o| o.opacity = 0.0);
    keyframe(&mut e, 84, &[words], |o| o.opacity = 1.0);
    tween(&mut e, 72, Tween::default());
    // The film is eight seconds long: the last keyframes last until then.
    e.on_timeline(|timeline, layer, _| timeline.insert_frames(layer, 95, 1));
    fill_out(&mut e);
    finish(e)
}

fn two_scenes() -> Document {
    let mut e = start(480, 300, t("Background"), 24.0);
    // A button that leads to a scene: a symbol with an Over look, and an action on its copy.
    let button = |e: &mut Editor, name: &str, x: f32, to_scene: usize| {
        let symbol = e.new_symbol(name, SymbolKind::Button);
        e.edit_symbol(symbol, None, true);
        boxed(e, t("Button"), rect(x, 220.0, 140.0, 40.0), 8.0, BLUE);
        centered(e, name, x + 70.0, 229.0, 17.0, WHITE, bold());
        e.set_symbol_look(1);
        e.copy_previous_look();
        let ids = e.selection.clone();
        for id in ids {
            if let Some(object) = e.object_mut(id) {
                if matches!(object.shape, Shape::Rect { .. }) {
                    object.fill = Some(Color::rgb(0x4a, 0x9d, 0xee));
                }
            }
        }
        e.set_symbol_look(0);
        e.leave_symbol();
        e.for_selected(|o| o.link = Action::Scene(to_scene).text());
    };
    // Each scene stops on its only frame, so the film waits for a button.
    let stop_here = |e: &mut Editor| {
        e.on_timeline(|timeline, layer, frame| {
            if let Some(at) = timeline.span_at(layer, frame) {
                timeline.layers[layer][at].action = Action::Stop.text();
            }
        });
    };
    e.doc.page_mut().name = t("Scene 1").to_string();
    let back = boxed(&mut e, t("Background"), rect(0.0, 0.0, 480.0, 300.0), 0.0, PALE);
    gradient(&mut e, back, FillKind::Linear, WHITE, Color::rgb(0xc8, 0xdc, 0xf4), (0.5, 0.0), (0.5, 1.0));
    centered(&mut e, t("The first scene"), 240.0, 80.0, 36.0, INK, bold());
    layer(&mut e, t("Buttons"));
    button(&mut e, t("Next"), 170.0, 2);
    stop_here(&mut e);
    e.add_page();
    e.doc.page_mut().layers[0].name = t("Background").to_string();
    let back = boxed(&mut e, t("Background"), rect(0.0, 0.0, 480.0, 300.0), 0.0, INK);
    gradient(&mut e, back, FillKind::Linear, INK, TEAL, (0.5, 0.0), (0.5, 1.0));
    centered(&mut e, t("The second scene"), 240.0, 80.0, 36.0, WHITE, bold());
    layer(&mut e, t("Buttons"));
    button(&mut e, t("Back"), 170.0, 1);
    stop_here(&mut e);
    e.set_active_page(0);
    finish(e)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_animation_example_builds_plays_and_is_published() {
        for example in EXAMPLES {
            let doc = (example.build)();
            let page = &doc.pages[0];
            let timeline = page.timeline.as_ref().unwrap_or_else(|| panic!("{} has no timeline", example.name));
            let frames = timeline.frame_count();
            assert_eq!(timeline.current, 0, "{}", example.name);
            // Every layer lasts the whole scene, so nothing drops out halfway.
            assert!(timeline.layers.iter().all(|spans| spans.last().is_some_and(|span| span.start + span.length == frames)), "{}", example.name);
            for frame in [0, frames / 2, frames - 1] {
                crate::export::render_page(&page.at_frame(frame).played()).unwrap();
            }
            let html = crate::publish::html(&doc, example.name).unwrap();
            assert!(html.contains("scenes: [["), "{}", example.name);
            let again = crate::model::document_from_json(&crate::model::document_to_json(&doc).unwrap()).unwrap();
            assert_eq!(again.pages.len(), doc.pages.len(), "{}", example.name);
        }
    }

    #[test]
    fn the_animation_examples_show_what_they_say() {
        let find = |name: &str| (EXAMPLES.iter().find(|e| e.name == name).unwrap().build)();
        // The ball is lowest in the middle of the film, and back where it began at the end.
        let doc = find("Bouncing ball");
        let ball_top = |frame: usize| doc.pages[0].at_frame(frame).layers[2].objects[0].bounds.y;
        assert!(ball_top(12) > 200.0 && ball_top(0) == ball_top(24) && ball_top(6) < 130.0, "falling, it is still slow a quarter of the way through");
        // Halfway between the square and the circle the shape is a path.
        let doc = find("Changing shapes");
        assert!(matches!(doc.pages[0].at_frame(7).layers[1].objects[0].shape, Shape::Path { closed: true }));
        // On the hill the arrow is high up and turned; the guide is not in the finished film.
        let doc = find("Along a path");
        let middle = doc.pages[0].at_frame(24);
        assert!(middle.layers[1].objects[0].bounds.center().y < 120.0);
        assert_eq!(middle.played().layers[2].objects.len(), 0);
        // The bright words show only through the mask.
        let doc = find("Spotlight");
        assert!(doc.pages[0].played().layers[1].objects.iter().all(|o| o.mask.is_some()));
        // The gear is one movie clip of 48 frames, placed twice.
        let doc = find("Turning gears");
        assert_eq!((doc.symbols.len(), doc.scenes().len(), doc.pages.len()), (1, 1, 2));
        assert_eq!(doc.pages[1].timeline.as_ref().unwrap().frame_count(), 48);
        assert_eq!(doc.pages[0].objects().filter(|o| matches!(o.shape, Shape::Instance { .. })).count(), 2);
        // Each layer keeps its own time: the car is done after four seconds, when the sun first shows; the words come at six.
        let doc = find("One after another");
        let page = &doc.pages[0];
        assert_eq!(page.timeline.as_ref().unwrap().frame_count(), 96);
        let (sun, car, words) = (|frame: usize| page.at_frame(frame).layers[1].objects.clone(), |frame: usize| page.at_frame(frame).layers[3].objects[0].bounds.x, |frame: usize| page.at_frame(frame).layers[4].objects.clone());
        assert!(sun(47).is_empty() && sun(48).len() == 1 && sun(72)[0].bounds.y < sun(60)[0].bounds.y && sun(95)[0].bounds.y == sun(72)[0].bounds.y);
        assert!(car(0) < car(24) && car(24) < car(48) && car(48) == car(95));
        assert!(words(71).is_empty() && words(72)[0].opacity == 0.0 && words(78)[0].opacity > 0.3 && words(95)[0].opacity == 1.0);
        // Two scenes, each stopping on its frame, each with a button that leads to the other.
        let doc = find("Two scenes with buttons");
        assert_eq!(doc.scenes().len(), 2);
        let html = crate::publish::html(&doc, "x").unwrap();
        assert!(html.contains("\"action:scene:2\"") && html.contains("\"action:scene:1\"") && html.matches("\"a\":\"action:stop\"").count() == 2);
    }
}
