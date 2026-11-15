//! Bodies of the dockable panels added on top of the core workspace.

use gpui::{
    anchored, canvas, deferred, div, fill, point, prelude::*, px, rgb, size, AnyElement, Bounds, Context, Div, FontWeight,
    MouseButton, MouseDownEvent, SharedString, Stateful, Window,
};

use crate::i18n::{t, tf};
use crate::autoshape::AutoKind;
use crate::canvas_view::paint_thumbnail;
use crate::library::common_items;
use crate::menus::{Cmd, Panel, SlotAction, COLOR_TOOLS, DOCK_GROUPS, TOOL_SECTIONS, ZOOM_LEVELS};
use crate::model::SymbolKind;
use crate::model::{Align, Color, Object, Rect, Shape, Stroke};
use crate::bitmap::Filter;
use crate::optimize::{format_size, ExportFormat, PRESETS};
use crate::paths::PathOp;
use crate::theme::*;
use crate::ui::{color_icon, color_u32, icon, label, label_width, tip};
use crate::workspace::{ColorTarget, Combo, FieldId, FindMode, NumericDialog, NumericMode, PanelItem, SizeDialogKind, Workspace};

const TILE: f32 = 44.0;
/// Width of a tile with its name underneath.
const CAPTIONED_TILE: f32 = 60.0;
const SPECIAL_CHARACTERS: &str = "©®™€£¥¢§¶•…–—«»‹›°±×÷µ¼½¾¡¿ÀÉÑÜßæøπΩ→←↑↓✓★♥";
/// Color steps drawn in a Color Mixer ramp.
const RAMP_STEPS: usize = 32;
const PALETTE_HUES: usize = 24;
const PALETTE_ROWS: usize = 7;
const PALETTE_CELL: f32 = 10.5;
const ALIGN_ITEMS: [(&str, Align); 6] = [
    ("Left", Align::Left),
    ("Center Horizontally", Align::CenterX),
    ("Right", Align::Right),
    ("Top", Align::Top),
    ("Center Vertically", Align::CenterY),
    ("Bottom", Align::Bottom),
];

pub(crate) fn mask_kind_label(kind: crate::model::MaskKind) -> &'static str {
    match kind {
        crate::model::MaskKind::PathOutline => t("Path Outline"),
        crate::model::MaskKind::Grayscale => t("Grayscale Appearance"),
        crate::model::MaskKind::Alpha => t("Alpha Channel"),
    }
}

/// The tallest a drop-down list grows before it scrolls.
const COMBO_MAX_HEIGHT: f32 = 420.0;

type Action = dyn Fn(&mut Workspace, &MouseDownEvent, &mut Window, &mut Context<Workspace>);

fn style_preview(fill: Option<Color>, stroke: Option<Stroke>, opacity: f32) -> Object {
    Object {
        id: 0,
        name: String::new(),
        shape: Shape::Rect { radius: 5.0 },
        bounds: Rect { x: 0.0, y: 0.0, w: 32.0, h: 32.0 },
        points: Vec::new(),
        handles: Vec::new(),
        placement: None,
        blend: crate::blend::BlendMode::Normal,
        mask: None,
        mask_of: None,
        fill_style: None,
        stroke_profile: Vec::new(),
        fill_look: Default::default(),
        link: String::new(),
        filters: Vec::new(),
        fill,
        stroke,
        opacity,
        visible: true,
        locked: false,
    }
}

fn auto_preview(kind: AutoKind) -> Object {
    let d = kind.defaults();
    Object {
        shape: Shape::Auto { kind, a: d.a, b: d.b, count: d.count },
        ..style_preview(Some(Color::rgb(0x4f, 0x8f, 0xd8)), Some(Stroke { color: Color::rgb(0x24, 0x52, 0x8c), width: 1.0 }), 1.0)
    }
}

fn heading(text: impl Into<SharedString>) -> Div {
    div().h(px(16.)).px_1().flex().flex_none().items_center().text_color(rgb(TEXT_DIM)).border_b_1().border_color(rgb(SEPARATOR)).child(text.into())
}

/// Converts hue (0..360), saturation and value (0..1) to a color.
fn from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
    let sector = (hue.rem_euclid(360.0)) / 60.0;
    let chroma = value * saturation;
    let x = chroma * (1.0 - (sector % 2.0 - 1.0).abs());
    let (r, g, b) = match sector as u32 {
        0 => (chroma, x, 0.0),
        1 => (x, chroma, 0.0),
        2 => (0.0, chroma, x),
        3 => (0.0, x, chroma),
        4 => (x, 0.0, chroma),
        _ => (chroma, 0.0, x),
    };
    let channel = |c: f32| (((c + value - chroma) * 255.0).round()) as u8;
    Color::rgb(channel(r), channel(g), channel(b))
}

/// Returns the hue, saturation and value of a color.
fn to_hsv(color: Color) -> (f32, f32, f32) {
    let (r, g, b) = (color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0);
    let max = r.max(g).max(b);
    let delta = max - r.min(g).min(b);
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    (hue, if max == 0.0 { 0.0 } else { delta / max }, max)
}

fn labelled_row(name: impl Into<SharedString>, width: f32, control: impl IntoElement) -> Div {
    let name = name.into();
    let width = label_width(&[&name], width);
    div().min_h(px(22.)).flex().flex_none().items_center().gap_1().child(div().w(px(width)).flex_none().child(label(name))).child(control)
}

impl Workspace {
    pub(crate) fn text_button(
        &self,
        text: impl Into<SharedString>,
        enabled: bool,
        cx: &mut Context<Self>,
        action: impl Fn(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    ) -> Div {
        div()
            .h(px(20.))
            .px_2()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .bg(rgb(PANEL_RAISED))
            .border_1()
            .border_color(rgb(BORDER))
            .text_color(rgb(if enabled { TEXT } else { TEXT_DISABLED }))
            .when(enabled, |d| {
                d.hover(|s| s.bg(rgb(HOVER))).on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, window, cx| {
                        this.finish_pending();
                        action(this, window, cx);
                        cx.notify();
                    }),
                )
            })
            .child(text.into())
    }

    fn icon_button(
        &self,
        name: &'static str,
        hint: &'static str,
        active: bool,
        cx: &mut Context<Self>,
        action: impl Fn(&mut Self, &mut Context<Self>) + 'static,
    ) -> Stateful<Div> {
        div()
            .id(name)
            .size(px(20.))
            .flex()
            .items_center()
            .justify_center()
            .tooltip(tip(hint))
            .when(active, |d| d.bg(rgb(PANEL_RAISED)))
            .hover(|s| s.bg(rgb(HOVER)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.finish_pending();
                    action(this, cx);
                    cx.notify();
                }),
            )
            .child(color_icon(name, 13., true))
    }

    fn footer() -> Div {
        div().h(px(24.)).flex().flex_none().items_center().justify_end().px_1().gap_1().bg(rgb(PANEL)).border_t_1().border_color(rgb(SEPARATOR))
    }

    /// A combo box for a setting that cannot be changed yet.
    pub(crate) fn disabled_combo(&self, value: impl Into<SharedString>, width: f32) -> Div {
        div()
            .w(px(width))
            .h(px(18.))
            .px_1()
            .flex()
            .flex_none()
            .items_center()
            .justify_between()
            .bg(rgb(PANEL_RAISED))
            .border_1()
            .border_color(rgb(SEPARATOR))
            .text_color(rgb(TEXT_DISABLED))
            .child(value.into())
            .child(icon("chevron-down", 8., TEXT_DISABLED))
    }

    /// A tile with its name below it.
    fn captioned_tile(
        &self,
        id: impl Into<SharedString>,
        caption: SharedString,
        objects: Vec<Object>,
        cx: &mut Context<Self>,
        action: Box<Action>,
    ) -> Div {
        div()
            .w(px(CAPTIONED_TILE + 3.))
            .flex()
            .flex_none()
            .flex_col()
            .items_center()
            .child(self.tile(id, caption.clone(), objects, false, cx, action).size(px(CAPTIONED_TILE)))
            .child(div().w_full().h(px(14.)).px(px(2.)).text_center().truncate().text_size(px(9.)).text_color(rgb(TEXT_DIM)).child(caption))
    }

    /// A list row's name, or its rename field while that is being edited.
    pub(crate) fn name_or_field(&self, id: FieldId, name: String, width: f32, cx: &mut Context<Self>) -> AnyElement {
        if self.field.as_ref().is_some_and(|f| f.id == id) {
            self.field(id, width, cx).into_any_element()
        } else {
            div().flex_1().min_w_0().overflow_hidden().child(name).into_any_element()
        }
    }

    fn tile(
        &self,
        id: impl Into<SharedString>,
        hint: impl Into<SharedString>,
        objects: Vec<Object>,
        selected: bool,
        cx: &mut Context<Self>,
        action: Box<Action>,
    ) -> Stateful<Div> {
        let id: SharedString = id.into();
        div()
            .id(id)
            .size(px(TILE))
            .flex_none()
            .m(px(1.))
            .bg(rgb(0xf4f4f4))
            .border_1()
            .border_color(rgb(if selected { SELECTION } else { BORDER }))
            .tooltip(tip(hint.into()))
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, ev, window, cx| {
                this.finish_pending();
                action(this, ev, window, cx);
                cx.notify();
            }))
            .child(canvas(|_, _, _| (), move |bounds, _, window, cx| paint_thumbnail(&objects, bounds, window, cx)).size_full())
    }

    // ---- Optimize -------------------------------------------------------------------

    pub(crate) fn combo(&self, combo: Combo, value: impl Into<SharedString>, width: f32, cx: &mut Context<Self>) -> Div {
        let value: SharedString = value.into();
        div()
            .w(px(width))
            .h(px(18.))
            .px_1()
            .flex()
            .flex_none()
            .items_center()
            .justify_between()
            .overflow_hidden()
            .bg(rgb(INPUT))
            .border_1()
            .border_color(rgb(INPUT_BORDER))
            .hover(|s| s.bg(rgb(HOVER)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                    this.finish_pending();
                    this.keep_overlays = true;
                    this.open_combo = if this.open_combo.is_some_and(|(open, _)| open == combo) { None } else { Some((combo, ev.position)) };
                    cx.notify();
                }),
            )
            .child(div().min_w_0().overflow_hidden().child(value))
            .child(icon("chevron-down", 8., TEXT_DIM))
    }

    pub(crate) fn combo_popup(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (combo, position) = self.open_combo?;
        type ItemAction = Box<dyn Fn(&mut Workspace, &mut Window, &mut Context<Workspace>)>;
        let item = |text: SharedString, chosen: bool, cx: &mut Context<Self>, action: ItemAction| {
            div()
                .h(px(20.))
                .px_3()
                .flex()
                .flex_none()
                .items_center()
                .when(chosen, |d| d.bg(rgb(ACCENT)))
                .hover(|s| s.bg(rgb(ACCENT)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, window, cx| {
                        this.open_combo = None;
                        action(this, window, cx);
                        cx.notify();
                    }),
                )
                .child(text)
                .into_any_element()
        };
        let page = self.ed.doc.page();
        let settings = page.export;
        let items: Vec<AnyElement> = match combo {
            Combo::Format => ExportFormat::ALL
                .iter()
                .map(|format| {
                    let format = *format;
                    item(format.label().into(), settings.format == format, cx, Box::new(move |this, _, _| this.set_export(|s| s.format = format)))
                })
                .collect(),
            Combo::Transparency => [(t("No Transparency"), false), (t("Index Transparency"), true)]
                .into_iter()
                .map(|(text, value)| item(text.into(), settings.transparent == value, cx, Box::new(move |this, _, _| this.set_export(|s| s.transparent = value))))
                .collect(),
            Combo::Preset => PRESETS
                .iter()
                .map(|(name, preset)| {
                    let preset = *preset;
                    item(t(name).into(), settings == preset, cx, Box::new(move |this, _, _| this.set_export(|s| *s = preset)))
                })
                .collect(),
            Combo::Zoom => ZOOM_LEVELS
                .iter()
                .map(|level| {
                    let level = *level;
                    let current = (self.zoom * 100.0).round() as u32 == level;
                    item(format!("{level}%").into(), current, cx, Box::new(move |this, window, cx| this.run(Cmd::ZoomTo(level), window, cx)))
                })
                .collect(),
            Combo::WorkspaceMode => [(t("Expanded Mode"), false), (t("Collapsed Mode"), true)]
                .into_iter()
                .map(|(text, compact)| item(text.into(), self.compact_workspace == compact, cx, Box::new(move |this, _, _| this.set_compact_workspace(compact))))
                .collect(),
            Combo::Page => self
                .ed
                .doc
                .pages
                .iter()
                .enumerate()
                .map(|(index, page)| item(page.name.clone().into(), index == self.ed.doc.active_page, cx, Box::new(move |this, _, _| this.ed.set_active_page(index))))
                .collect(),
            Combo::State => page
                .states
                .iter()
                .enumerate()
                .map(|(index, state)| item(state.name.clone().into(), index == page.active_state, cx, Box::new(move |this, _, _| this.go_to_state(index))))
                .collect(),
            Combo::Align => ALIGN_ITEMS
                .into_iter()
                .map(|(text, op)| {
                    item(
                        t(text).into(),
                        false,
                        cx,
                        Box::new(move |this, window, cx| {
                            if this.is_enabled(Cmd::Align(op)) {
                                this.run(Cmd::Align(op), window, cx);
                            }
                        }),
                    )
                })
                .collect(),
            Combo::AddFilter => crate::effects::MENU
                .iter()
                .flat_map(|(group, kinds)| {
                    // The name of the group, then its filters.
                    let heading = div().h(px(18.)).px_2().flex().flex_none().items_center().text_color(rgb(TEXT_DISABLED)).child(t(group)).into_any_element();
                    let entries = kinds.iter().map(|kind| {
                        let kind = *kind;
                        item(kind.label().into(), false, cx, Box::new(move |this, _, _| this.add_filter(kind)))
                    });
                    std::iter::once(heading).chain(entries).collect::<Vec<_>>()
                })
                .collect(),
            Combo::TextOrientation => crate::model::TextOrientation::ALL
                .into_iter()
                .map(|orientation| {
                    item(orientation.label().into(), self.shown_text_style().orientation == orientation, cx, Box::new(move |this, window, cx| this.run(Cmd::TextOrientation(orientation), window, cx)))
                })
                .collect(),
            Combo::PathOrientation => crate::model::PathOrientation::ALL
                .into_iter()
                .map(|orientation| {
                    item(orientation.label().into(), self.shown_text_style().on_path == orientation, cx, Box::new(move |this, window, cx| this.run(Cmd::PathOrientation(orientation), window, cx)))
                })
                .collect(),
            Combo::FillEdge => crate::model::FillEdge::ALL
                .into_iter()
                .map(|edge| item(edge.label().into(), self.current_fill_look().edge == edge, cx, Box::new(move |this, _, _| this.change_fill_look(|look| look.edge = edge))))
                .collect(),
            Combo::FillTexture => std::iter::once(t("None"))
                .chain(crate::pseffects::PATTERNS.iter().map(|name| crate::pseffects::label(name)))
                .enumerate()
                .map(|(index, name)| item(name.into(), self.current_fill_look().texture as usize == index, cx, Box::new(move |this, _, _| this.change_fill_look(|look| look.texture = index as u8))))
                .collect(),
            Combo::ToneRange => crate::bitmap::ToneRange::ALL
                .into_iter()
                .map(|range| item(range.label().into(), self.tool_options.range == range, cx, Box::new(move |this, _, _| this.tool_options.range = range)))
                .collect(),
            Combo::PolygonShape => [(t("Polygon"), false), (t("Star"), true)]
                .into_iter()
                .map(|(name, star)| item(name.into(), self.tool_options.star == star, cx, Box::new(move |this, _, _| this.tool_options.star = star)))
                .collect(),
            Combo::FillKind => {
                let current = self.current_fill_style().map(|style| style.kind);
                std::iter::once((t("Solid"), None))
                    .chain(crate::model::FillKind::ALL.into_iter().map(|kind| (kind.label(), Some(kind))))
                    .map(|(name, kind)| item(name.into(), kind == current, cx, Box::new(move |this, _, _| this.set_fill_kind(kind))))
                    .collect()
            }
            Combo::FillPattern => {
                let current = self.current_fill_style().map_or(0, |style| style.pattern as usize);
                crate::pseffects::PATTERNS
                    .iter()
                    .enumerate()
                    .map(|(index, name)| {
                        item(crate::pseffects::label(name).into(), index == current, cx, Box::new(move |this, _, _| this.change_fill_style(true, |style| style.pattern = index as u8)))
                    })
                    .collect()
            }
            Combo::PrintRange => {
                use crate::print::PageRange;
                let current = self.print_dialog.as_ref().map(|dialog| dialog.range);
                [(t("Current page"), PageRange::Current), (t("All pages"), PageRange::All), (t("Pages"), PageRange::Pages)]
                    .into_iter()
                    .map(|(name, range)| {
                        item(name.into(), current == Some(range), cx, Box::new(move |this, _, _| {
                            if let Some(dialog) = this.print_dialog.as_mut() {
                                dialog.range = range;
                            }
                        }))
                    })
                    .collect()
            }
            Combo::Paper => crate::print::PaperSize::ALL
                .into_iter()
                .map(|paper| item(paper.name().into(), self.print_setup.paper == paper, cx, Box::new(move |this, _, _| this.print_setup.paper = paper)))
                .collect(),
            Combo::BatchScale => (0..4)
                .map(|scale| {
                    item(t(crate::workspace::BATCH_SCALES[scale]).into(), self.batch_dialog.as_ref().is_some_and(|d| d.scale == scale), cx, Box::new(move |this, _, _| {
                        if let Some(dialog) = this.batch_dialog.as_mut() {
                            dialog.scale = scale;
                        }
                    }))
                })
                .collect(),
            Combo::ButtonLook => (0..4)
                .map(|look| {
                    item(crate::model::button_look_name(look).into(), self.ed.editing_symbol().is_some_and(|edit| edit.look == look), cx, Box::new(move |this, _, _| {
                        if this.ed.set_symbol_look(look) {
                            this.ed.commit(t("Edit Symbol"));
                        }
                    }))
                })
                .collect(),
            Combo::Orientation => [(t("Portrait"), false), (t("Landscape"), true)]
                .into_iter()
                .map(|(name, landscape)| item(name.into(), self.print_setup.landscape == landscape, cx, Box::new(move |this, _, _| this.print_setup.landscape = landscape)))
                .collect(),
            Combo::Printer => {
                let (printers, chosen) = self.print_dialog.as_ref().map_or((Vec::new(), 0), |dialog| (dialog.printers.clone(), dialog.chosen));
                printers
                    .into_iter()
                    .chain([t("Save as PDF...").to_string()])
                    .enumerate()
                    .map(|(index, name)| {
                        item(name.into(), index == chosen, cx, Box::new(move |this, _, _| {
                            if let Some(dialog) = this.print_dialog.as_mut() {
                                dialog.chosen = index;
                            }
                        }))
                    })
                    .collect()
            }
            Combo::TextFont => {
                let shown = self.shown_text_style().font;
                crate::fonts::families()
                    .iter()
                    .enumerate()
                    .map(|(index, name)| item(name.clone().into(), *name == shown, cx, Box::new(move |this, window, cx| this.run(Cmd::TextFont(index as u16), window, cx))))
                    .collect()
            }
            Combo::GradientPreset => crate::pseffects::PRESETS
                .iter()
                .enumerate()
                .map(|(which, (name, _))| item(crate::pseffects::label(name).into(), false, cx, Box::new(move |this, _, _| this.use_gradient_preset(which))))
                .collect(),
            Combo::FilterChoice(setting) => {
                let current = self.edited_filter().map_or(0, |(_, filter)| filter.value(setting).round() as usize);
                let names: Vec<&'static str> = match self.edited_filter().map(|(_, filter)| filter.kind) {
                    Some(crate::effects::FilterKind::ColorFill) => crate::blend::BlendMode::ALL.iter().map(|mode| mode.label()).collect(),
                    Some(crate::effects::FilterKind::PhotoshopEffects) => crate::pseffects::choices(setting).unwrap_or_default(),
                    Some(crate::effects::FilterKind::Levels | crate::effects::FilterKind::Curves) => crate::effects::CHANNELS.iter().map(|name| channel_label(name)).collect(),
                    _ => crate::effects::BEVEL_SHAPES.iter().map(|name| t(name)).collect(),
                };
                names
                    .into_iter()
                    .enumerate()
                    .map(|(index, name)| item(name.into(), index == current, cx, Box::new(move |this, _, _| this.set_filter_value(setting, index as f32))))
                    .collect()
            }
            Combo::MaskKind => {
                use crate::model::MaskKind;
                let current = self.current_mask().map(|mask| mask.kind);
                // A vector mask goes by its outline or its look; a bitmap mask by its look or its opacity.
                let bitmap = self.current_mask().is_some_and(|mask| mask.object.is_image());
                let kinds = if bitmap { [MaskKind::Grayscale, MaskKind::Alpha] } else { [MaskKind::PathOutline, MaskKind::Grayscale] };
                kinds
                    .into_iter()
                    .map(|kind| item(mask_kind_label(kind).into(), current == Some(kind), cx, Box::new(move |this, _, _| this.change_masks(|mask| mask.kind = kind))))
                    .collect()
            }
            Combo::Blend => crate::blend::BlendMode::ALL
                .into_iter()
                .map(|mode| item(mode.label().into(), self.current_blend() == mode, cx, Box::new(move |this, _, _| this.set_blend(mode))))
                .collect(),
            Combo::SymbolKind(symbol) => SymbolKind::ALL
                .into_iter()
                .map(|kind| {
                    let current = self.ed.doc.symbols.iter().any(|s| s.id == symbol && s.kind == kind);
                    item(kind.label().into(), current, cx, Box::new(move |this, _, _| {
                        if let Some(entry) = this.ed.doc.symbols.iter_mut().find(|s| s.id == symbol) {
                            entry.kind = kind;
                            this.ed.commit(t("Symbol Properties"));
                        }
                    }))
                })
                .collect(),
            Combo::NumericMode => NumericMode::ALL
                .into_iter()
                .map(|mode| {
                    let current = self.numeric_dialog.is_some_and(|d| d.mode == mode);
                    item(mode.label().into(), current, cx, Box::new(move |this, _, _| {
                        this.commit_field();
                        if let Some(dialog) = this.numeric_dialog.as_mut() {
                            dialog.mode = mode;
                        }
                    }))
                })
                .collect(),
            Combo::ToolGroup(group) => crate::menus::TOOL_GROUPS[group.min(crate::menus::TOOL_GROUPS.len() - 1)]
                .iter()
                .map(|slot| {
                    let SlotAction::Select(tool) = slot.action else { return div().into_any_element() };
                    let pick = move |this: &mut Workspace, cx: &mut Context<Workspace>| {
                        this.open_combo = None;
                        this.set_tool(tool);
                        cx.notify();
                    };
                    div()
                        .h(px(22.))
                        .px_2()
                        .flex()
                        .flex_none()
                        .items_center()
                        .gap_2()
                        .when(self.tool == tool, |d| d.bg(rgb(ACCENT)))
                        .hover(|s| s.bg(rgb(ACCENT)))
                        // The button may still be held: letting go over a tool picks it, and so does a click.
                        .on_mouse_up(MouseButton::Left, cx.listener(move |this, _, _, cx| pick(this, cx)))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _, _, cx| pick(this, cx)))
                        .child(color_icon(slot.icon, 15., true))
                        .child(t(slot.label))
                        .into_any_element()
                })
                .collect(),
            Combo::PanelMenu(group) => {
                let collapse = if self.groups[group].collapsed { t("Expand Panel Group") } else { t("Collapse Panel Group") };
                vec![
                    item(collapse.into(), false, cx, Box::new(move |this, _, _| this.groups[group].collapsed = !this.groups[group].collapsed)),
                    item(
                        t("Close Panel Group").into(),
                        false,
                        cx,
                        Box::new(move |this, _, _| {
                            for panel in DOCK_GROUPS[group] {
                                if this.panel_open(*panel) {
                                    this.hidden_panels.push(*panel);
                                }
                            }
                        }),
                    ),
                ]
            }
        };
        Some(
            deferred(
                anchored().position(position).snap_to_window().child(
                    // A long list, such as the blend modes, scrolls inside the window.
                    div()
                        .id("combo-popup")
                        .occlude()
                        .w(px(170.))
                        .max_h(px(COMBO_MAX_HEIGHT))
                        .overflow_y_scroll()
                        .py_1()
                        .flex()
                        .flex_col()
                        .bg(rgb(PANEL_RAISED))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .shadow_md()
                        .children(items),
                ),
            )
            .with_priority(2)
            .into_any_element(),
        )
    }

    pub(crate) fn optimize_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let settings = self.ed.doc.page().export;
        let format = settings.format;
        let size_text = match (&self.estimate, &self.estimate_error) {
            (_, Some(error)) => error.clone(),
            (Some(bytes), _) => tf("Estimated size: {}", &[&format_size(*bytes)]),
            _ => t("Estimating size...").to_string(),
        };
        let animated_note = (format == ExportFormat::AnimatedGif).then(|| {
            let frames = self.ed.doc.page().states.iter().filter(|s| s.exported).count();
            label(tf("{} frame(s) from the States panel", &[&frames]))
        });
        let preset = PRESETS.iter().find(|(_, p)| *p == settings).map_or(t("Custom"), |(name, _)| t(name));
        let greyed = |text: &'static str| div().text_color(rgb(TEXT_DISABLED)).child(text);
        let palette: Vec<_> = self.estimate_palette.iter().map(|color| Self::palette_chip(*color)).collect();
        let row = || div().min_h(px(23.)).flex().flex_none().items_center().gap_1();
        div()
            .size_full()
            .p_1()
            .flex()
            .flex_col()
            .child(row().child(self.combo(Combo::Preset, preset, 256., cx)))
            .child(
                row()
                    .justify_between()
                    .child(self.combo(Combo::Format, format.label(), 140., cx))
                    .when(format != ExportFormat::Png32, |d| {
                        d.child(div().flex().items_center().gap_1().child(label(t("Matte:"))).child(self.swatch(ColorTarget::Matte, 18., cx)))
                    }),
            )
            .when(format.is_indexed(), |d| {
                d.child(
                    row()
                        .justify_between()
                        .child(self.combo(Combo::Transparency, if settings.transparent { t("Index Transparency") } else { t("No Transparency") }, 140., cx))
                        .child(div().flex().items_center().gap_1().child(label(t("Colors:"))).child(self.field(FieldId::OptimizeColors, 44., cx))),
                )
            })
            .when(format == ExportFormat::Jpeg, |d| {
                d.child(row().child(label(t("Quality:"))).child(self.field(FieldId::OptimizeQuality, 44., cx)).child(label("%")))
                    .child(
                        row()
                            .child(self.check_box(t("Selective quality:"), settings.selective, cx, |this| this.set_export(|s| s.selective = !s.selective)))
                            .child(self.field(FieldId::OptimizeSelective, 36., cx))
                            .child(self.text_button(t("Settings..."), true, cx, |this, window, cx| this.run(Cmd::JpegSettings, window, cx))),
                    )
                    .child(row().child(label(t("Smoothing:"))).child(self.field(FieldId::OptimizeSmoothing, 36., cx)))
            })
            .children(animated_note)
            .child(
                div()
                    .id("color-table")
                    .flex_1()
                    .min_h_0()
                    .p(px(2.))
                    .overflow_y_scroll()
                    .bg(rgb(LIST))
                    .border_1()
                    .border_color(rgb(LIST_BORDER))
                    .child(div().flex().flex_wrap().children(palette)),
            )
            .child(
                row()
                    .justify_between()
                    .child(greyed(t("Sort by: None")))
                    .child(label(if format.is_indexed() { tf("{} colors", &[&self.estimate_palette.len()]) } else { String::new() })),
            )
            .child(
                row()
                    .justify_between()
                    .child(label(size_text))
                    // Image Preview has an Export button of its own.
                    .when(self.image_preview.is_none(), |d| d.child(self.text_button(t("Export..."), true, cx, |this, window, cx| this.run(Cmd::Export, window, cx)))),
            )
            .into_any_element()
    }

    fn palette_chip(color: Color) -> Div {
        div().size(px(12.)).m(px(1.)).flex_none().bg(rgb(color_u32(color)))
    }

    // ---- Color Palette ----------------------------------------------------------------

    fn palette_cell(color: Color, size: f32, cx: &mut Context<Self>) -> Div {
        div().size(px(size)).flex_none().bg(rgb(color_u32(color))).on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                this.finish_pending();
                let target = if ev.modifiers.shift { ColorTarget::Stroke } else { ColorTarget::Fill };
                this.apply_color(target, Some(color));
                cx.notify();
            }),
        )
    }

    pub(crate) fn color_palette_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let mut grid = Vec::with_capacity(PALETTE_HUES * PALETTE_ROWS);
        for row in 0..PALETTE_ROWS {
            for hue in 0..PALETTE_HUES {
                let hue = hue as f32 * 360.0 / PALETTE_HUES as f32;
                let t = row as f32 / (PALETTE_ROWS - 1) as f32;
                // Tints at the top, full color in the middle, shades at the bottom.
                let (saturation, value) = if t < 0.5 { (0.2 + 1.6 * t, 1.0) } else { (1.0, 1.0 - 1.4 * (t - 0.5)) };
                grid.push(Self::palette_cell(from_hsv(hue, saturation, value), PALETTE_CELL, cx));
            }
        }
        let base = self.current_color(ColorTarget::Fill).unwrap_or(Color::rgb(0xff, 0x66, 0x00));
        let (hue, saturation, value) = to_hsv(base);
        let harmony = |name: &'static str, offsets: &[f32], cx: &mut Context<Self>| {
            div()
                .h(px(20.))
                .flex()
                .flex_none()
                .items_center()
                .gap_1()
                .child(div().w(px(84.)).child(label(name)))
                .children(offsets.iter().map(|offset| Self::palette_cell(from_hsv(hue + offset, saturation, value), 16., cx)).collect::<Vec<_>>())
        };
        div()
            .id("color-palette")
            .size_full()
            .p_1()
            .flex()
            .flex_col()
            .gap_1()
            .overflow_y_scroll()
            .child(div().w(px(PALETTE_CELL * PALETTE_HUES as f32 + 1.)).flex().flex_none().flex_wrap().children(grid))
            .child(harmony(t("Complementary"), &[0., 180.], cx))
            .child(harmony(t("Analogous"), &[-30., 0., 30.], cx))
            .child(harmony(t("Triad"), &[0., 120., 240.], cx))
            .into_any_element()
    }

    // ---- Symbol Properties --------------------------------------------------------------

    pub(crate) fn symbol_properties_body(&self, cx: &mut Context<Self>) -> Div {
        let shell = div().flex_1().min_h_0().p_2().flex().items_start().gap_4().bg(rgb(PANEL));
        let selected = self.ed.selected();
        let symbol = selected.iter().find_map(|o| match o.shape {
            Shape::Instance { symbol, .. } => Some(symbol),
            _ => None,
        });
        let Some(symbol) = symbol.filter(|id| self.ed.doc.symbols.iter().any(|s| s.id == *id)) else {
            return shell.child(label(t("Select a symbol copy to see its properties")));
        };
        let copies = self
            .ed
            .doc
            .pages
            .iter()
            .flat_map(|page| page.objects())
            .filter(|o| matches!(o.shape, Shape::Instance { symbol: placed, .. } if placed == symbol))
            .count();
        let kind = self.ed.doc.symbols.iter().find(|s| s.id == symbol).map_or(SymbolKind::Graphic, |s| s.kind);
        shell
            .child(labelled_row(t("Symbol:"), 46., self.field(FieldId::RenameSymbol(symbol), 150., cx)))
            .child(labelled_row(t("Type:"), 30., self.combo(Combo::SymbolKind(symbol), kind.label(), 90., cx).bg(rgb(PANEL_RAISED))))
            .child(div().h(px(22.)).flex().items_center().child(label(tf(if copies == 1 { "{} copy in this document" } else { "{} copies in this document" }, &[&copies]))))
            .child(self.text_button(t("Break Apart"), true, cx, |this, window, cx| this.run(Cmd::BreakApart, window, cx)))
    }

    // ---- Size dialog ----------------------------------------------------------------------

    pub(crate) fn size_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let dialog = self.size_dialog?;
        let is_image = dialog.kind == SizeDialogKind::Image;
        let title = if is_image { t("Image Size") } else { t("Canvas Size") };
        let size_label_width = label_width(&[t("Width:"), t("Height:")], 50.);
        Some(
            deferred(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .occlude()
                    .child(
                        div()
                            .w(px(240.))
                            .p_3()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .bg(rgb(PANEL))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .shadow_lg()
                            .child(div().font_weight(FontWeight::BOLD).child(title))
                            .child(labelled_row(t("Width:"), size_label_width, div().flex().items_center().gap_1().child(self.field(FieldId::DialogW, 70., cx)).child(label(t("pixels")))))
                            .child(labelled_row(t("Height:"), size_label_width, div().flex().items_center().gap_1().child(self.field(FieldId::DialogH, 70., cx)).child(label(t("pixels")))))
                            .when(is_image, |d| {
                                d.child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _, _, cx| {
                                                this.commit_field();
                                                if let Some(dialog) = this.size_dialog.as_mut() {
                                                    dialog.constrain = !dialog.constrain;
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            div()
                                                .size(px(13.))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .bg(rgb(INPUT))
                                                .border_1()
                                                .border_color(rgb(INPUT_BORDER))
                                                .when(dialog.constrain, |d| d.child(icon("check", 9., TEXT))),
                                        )
                                        .child(t("Constrain Proportions")),
                                )
                            })
                            .child(
                                div()
                                    .flex()
                                    .justify_end()
                                    .gap_1()
                                    .child(self.text_button(t("OK"), true, cx, |this, _, _| this.apply_size_dialog()))
                                    .child(self.text_button(t("Cancel"), true, cx, |this, _, _| {
                                        this.field = None;
                                        this.size_dialog = None;
                                    })),
                            ),
                    ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    /// A box that is ticked or not, with its text, and what a click on it does.
    pub(crate) fn check_box(&self, text: &'static str, on: bool, cx: &mut Context<Self>, toggle: fn(&mut Workspace)) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.commit_field();
                    toggle(this);
                    cx.notify();
                }),
            )
            .child(
                div()
                    .size(px(13.))
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .bg(rgb(INPUT))
                    .border_1()
                    .border_color(rgb(INPUT_BORDER))
                    .when(on, |d| d.child(icon("check", 9., TEXT))),
            )
            .child(text)
    }

    /// The settings of a live filter. Levels, Curves and Photoshop Live Effects have
    /// dialogs of their own; the others get a field for each number, a box for each
    /// switch, a list for each choice and the color, if the filter has one.
    pub(crate) fn filter_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        use crate::effects::FilterKind;
        let (_, filter) = self.edited_filter()?;
        let (width, rows) = match filter.kind {
            FilterKind::Levels => (284., self.levels_rows(&filter, cx)),
            FilterKind::Curves => (284., self.curves_rows(&filter, cx)),
            FilterKind::PhotoshopEffects => (470., self.photoshop_rows(&filter, cx)),
            _ => (280., self.plain_filter_rows(&filter, cx)),
        };
        let tonal = matches!(filter.kind, FilterKind::Levels | FilterKind::Curves);
        // The three eyedroppers: the color clicked on the object becomes black, gray or white.
        let droppers: Vec<Div> = [(0x000000, PANEL_RAISED), (0x808080, PANEL_RAISED), (0xffffff, PANEL_RAISED)]
            .into_iter()
            .enumerate()
            .map(|(dropper, (color, back))| {
                div()
                    .size(px(22.))
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .bg(rgb(back))
                    .border_1()
                    .border_color(rgb(if self.filter_dropper == Some(dropper) { SELECTION } else { INPUT_BORDER }))
                    .when(self.filter_dropper == Some(dropper), |d| d.border_2())
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.pick_filter_dropper(dropper);
                            cx.notify();
                        }),
                    )
                    // The outline of the eyedropper, with its glass in the shade it sets.
                    .child(div().relative().size(px(16.)).child(crate::ui::icon_layer("eyedropper", 0, 16., color).absolute().top_0().left_0()).child(crate::ui::icon_layer("eyedropper", 1, 16., TEXT).absolute().top_0().left_0()))
            })
            .collect();
        Some(
            deferred(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .occlude()
                    // A point of the curve follows the mouse anywhere over the window, not only inside its box.
                    .on_mouse_move(cx.listener(|this, ev: &gpui::MouseMoveEvent, _, cx| {
                        if this.drag_curve(ev.position) || this.drag_level(ev.position) || this.drag_gradient(ev.position) {
                            cx.notify();
                        }
                    }))
                    // With an eyedropper in hand, a click beside the dialog picks a color from the object.
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, ev: &MouseDownEvent, _, cx| {
                            if this.use_filter_dropper(ev.position) {
                                cx.notify();
                            }
                        }),
                    )
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            this.release_curve();
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .w(px(width))
                            .p_3()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .bg(rgb(PANEL))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .shadow_lg()
                            // A click on the dialog itself is not one on the object behind it.
                            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                            .child(div().pb_1().font_weight(FontWeight::BOLD).child(filter.kind.label()))
                            .children(rows)
                            .when(tonal, |d| {
                                d.child(
                                    div()
                                        .pt_1()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(self.text_button(t("Auto"), self.filter_histogram.is_some(), cx, |this, _, _| this.auto_filter()))
                                        .child(div().flex_1())
                                        .children(droppers),
                                )
                            })
                            .child(
                                div()
                                    .pt_1()
                                    .flex()
                                    .items_center()
                                    .justify_end()
                                    .gap_1()
                                    .when(filter.kind != FilterKind::PhotoshopEffects, |d| {
                                        d.child(self.check_box(t("Preview"), self.filter_preview(), cx, |this| this.toggle_filter_preview())).child(div().flex_1())
                                    })
                                    .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_filter_dialog(true)))
                                    .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_filter_dialog(false))),
                            ),
                    ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    fn plain_filter_rows(&self, filter: &crate::effects::LiveFilter, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let settings = filter.kind.settings();
        let name_width = label_width(&settings.iter().map(|s| t(s.name)).collect::<Vec<_>>(), 60.);
        let mut rows: Vec<AnyElement> = Vec::new();
        for (index, setting) in settings.iter().enumerate() {
            let chosen_from_list = matches!((filter.kind, setting.name), (_, "Edge shape") | (crate::effects::FilterKind::ColorFill, "Blend Mode"));
            if setting.toggle {
                rows.push(self.filter_toggle(index, t(setting.name), filter.value(index) >= 0.5, cx).into_any_element());
            } else if chosen_from_list {
                let current = filter.value(index).round() as usize;
                let value = if setting.name == "Blend Mode" {
                    crate::blend::BlendMode::ALL[current.min(crate::blend::BlendMode::ALL.len() - 1)].label()
                } else {
                    t(crate::effects::BEVEL_SHAPES[current.min(crate::effects::BEVEL_SHAPES.len() - 1)])
                };
                rows.push(labelled_row(t(setting.name), name_width, self.combo(Combo::FilterChoice(index), value, 130., cx).bg(rgb(PANEL_RAISED))).into_any_element());
            } else {
                rows.push(labelled_row(t(setting.name), name_width, self.field(FieldId::FilterValue(index), 60., cx)).into_any_element());
            }
        }
        if filter.kind.start_color().is_some() {
            rows.push(labelled_row(t("Color"), name_width, self.swatch(ColorTarget::Filter, 18., cx)).into_any_element());
        }
        rows
    }

    /// A box that switches one setting of the filter being edited on and off.
    fn filter_toggle(&self, setting: usize, text: &'static str, on: bool, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .items_center()
            .gap_1()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, _, cx| {
                    this.commit_field();
                    this.set_filter_value(setting, if on { 0.0 } else { 1.0 });
                    cx.notify();
                }),
            )
            .child(div().size(px(13.)).flex().flex_none().items_center().justify_center().bg(rgb(INPUT)).border_1().border_color(rgb(INPUT_BORDER)).when(on, |d| d.child(icon("check", 9., TEXT))))
            .child(text)
    }

    /// The list that picks the channel Levels and Curves show.
    fn channel_row(&self, setting: usize, cx: &mut Context<Self>) -> AnyElement {
        let channel = self.filter_channel().min(crate::effects::CHANNELS.len() - 1);
        labelled_row(t("Channel:"), 50., self.combo(Combo::FilterChoice(setting), channel_label(crate::effects::CHANNELS[channel]), 110., cx).bg(rgb(PANEL_RAISED))).into_any_element()
    }

    /// Levels: the channel, how many pixels have each of its levels, and the levels
    /// taken as black, mid gray and white on the way in and as black and white on the way out.
    fn levels_rows(&self, filter: &crate::effects::LiveFilter, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let channel = self.filter_channel();
        let at = crate::effects::levels_start(channel);
        let histogram = self.filter_histogram.clone();
        let bars = canvas(|_, _, _| (), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
            window.paint_quad(fill(bounds, rgb(INPUT)));
            let Some(histogram) = histogram.as_ref() else { return };
            // All channels together count a pixel once for each of its three colors.
            let count = |level: usize| match channel {
                0 => histogram[0][level] + histogram[1][level] + histogram[2][level],
                c => histogram[(c - 1).min(2)][level],
            };
            let most = (0..256).map(count).max().unwrap_or(0).max(1) as f32;
            let (step, height) = (f32::from(bounds.size.width) / 256.0, f32::from(bounds.size.height));
            let color = [TEXT, 0xe04040, 0x40c040, 0x5080ff][channel.min(3)];
            for level in 0..256 {
                let bar = (count(level) as f32 / most * height).ceil().min(height);
                if bar > 0.0 {
                    let origin = point(bounds.origin.x + px(level as f32 * step), bounds.origin.y + px(height - bar));
                    window.paint_quad(fill(Bounds { origin, size: size(px(step + 0.3), px(bar)) }, rgb(color)));
                }
            }
        })
        .w(px(256.))
        .h(px(80.));
        let fields: Vec<Div> = (0..5).map(|offset| self.field(FieldId::FilterValue(at + offset), 44., cx)).collect();
        let mut fields = fields.into_iter();
        let input = div().flex().gap_1().children(fields.by_ref().take(3));
        let output = div().flex().gap_1().children(fields);
        // The sliders under the histogram and under the ramp: small triangles at their levels.
        let values = [0, 1, 2, 3, 4].map(|offset| filter.value(at + offset));
        let middle = values[0] + (values[2] - values[0]) * crate::effects::gamma_place(values[1]);
        let strip = |row: usize, sliders: Vec<(f32, u32)>, cx: &mut Context<Self>| {
            let cell = self.level_bounds[row].clone();
            div()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                        this.press_level(row, ev.position);
                        cx.notify();
                    }),
                )
                .child(
                    canvas(move |bounds, _, _| cell.set(bounds), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
                        for (level, color) in &sliders {
                            let x = bounds.origin.x + px(level / 255.0 * f32::from(bounds.size.width));
                            let (top, bottom) = (bounds.origin.y + px(1.), bounds.origin.y + bounds.size.height - px(1.));
                            // A dark triangle with the slider's own shade inside it.
                            for (inset, shade) in [(0., TEXT), (2., *color)] {
                                let mut path = gpui::PathBuilder::fill();
                                path.move_to(point(x, top + px(inset)));
                                path.line_to(point(x + px(5. - inset), bottom - px(inset / 2.)));
                                path.line_to(point(x - px(5. - inset), bottom - px(inset / 2.)));
                                path.close();
                                if let Ok(path) = path.build() {
                                    window.paint_path(path, rgb(shade));
                                }
                            }
                        }
                    })
                    .w(px(256.))
                    .h(px(11.)),
                )
        };
        let tone = [0xffffff, 0xff0000, 0x00ff00, 0x0000ff][channel.min(3)];
        let ramp = canvas(|_, _, _| (), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
            // From black to the full color of the channel, in steps too small to see.
            let step = f32::from(bounds.size.width) / 64.0;
            for i in 0..64u32 {
                let part = i * 255 / 63;
                let shade = (((tone >> 16) & 0xff) * part / 255) << 16 | (((tone >> 8) & 0xff) * part / 255) << 8 | ((tone & 0xff) * part / 255);
                let origin = point(bounds.origin.x + px(i as f32 * step), bounds.origin.y);
                window.paint_quad(fill(Bounds { origin, size: size(px(step + 0.5), bounds.size.height) }, rgb(shade)));
            }
        })
        .w(px(256.))
        .h(px(10.));
        let into = strip(0, vec![(values[0], 0x000000), (middle, 0x808080), (values[2], 0xffffff)], cx);
        let out_of = strip(1, vec![(values[3], 0x000000), (values[4], 0xffffff)], cx);
        vec![
            self.channel_row(5, cx),
            div().border_1().border_color(rgb(INPUT_BORDER)).child(bars).into_any_element(),
            into.into_any_element(),
            labelled_row(t("Input levels:"), 100., input).into_any_element(),
            div().mt_1().border_1().border_color(rgb(INPUT_BORDER)).child(ramp).into_any_element(),
            out_of.into_any_element(),
            labelled_row(t("Output levels:"), 100., output).into_any_element(),
        ]
    }

    /// Curves: the channel and its curve. A press on the curve puts a point there, a
    /// drag moves a point, and a point dragged out of the box is taken away.
    fn curves_rows(&self, filter: &crate::effects::LiveFilter, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let channel = self.filter_channel();
        let (points, table, picked) = (filter.curve(channel), filter.curve_table(channel), self.curve_point);
        let shown = picked.and_then(|index| points.get(index).copied());
        let cell = self.curve_bounds.clone();
        let graph = canvas(move |bounds, _, _| cell.set(bounds), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
            window.paint_quad(fill(bounds, rgb(INPUT)));
            let (width, height) = (f32::from(bounds.size.width), f32::from(bounds.size.height));
            let at = |x: f32, y: f32| point(bounds.origin.x + px(x / 255.0 * width), bounds.origin.y + px((1.0 - y / 255.0) * height));
            // A grid of quarters behind the curve.
            for quarter in 1..4 {
                let offset = quarter as f32 / 4.0;
                window.paint_quad(fill(Bounds { origin: point(bounds.origin.x + px(offset * width), bounds.origin.y), size: size(px(1.), bounds.size.height) }, rgb(BORDER)));
                window.paint_quad(fill(Bounds { origin: point(bounds.origin.x, bounds.origin.y + px(offset * height)), size: size(bounds.size.width, px(1.)) }, rgb(BORDER)));
            }
            let color = [TEXT, 0xe04040, 0x40c040, 0x5080ff][channel.min(3)];
            // The curve, one level at a time: each piece spans from this level's output to the next one's.
            for level in 0..255 {
                let (from, to) = (table[level], table[level + 1]);
                let (top, bottom) = (at(level as f32, from.max(to)), at(level as f32, from.min(to)));
                let tall = f32::from(bottom.y - top.y).max(1.5);
                window.paint_quad(fill(Bounds { origin: point(top.x, top.y - px(0.75)), size: size(px(width / 255.0 + 0.5), px(tall)) }, rgb(color)));
            }
            for (index, (x, y)) in points.iter().enumerate() {
                let center = at(*x, *y);
                let square = Bounds { origin: point(center.x - px(3.5), center.y - px(3.5)), size: size(px(7.), px(7.)) };
                window.paint_quad(fill(square, rgb(if picked == Some(index) { color } else { INPUT })).border_widths(1.).border_color(rgb(color)));
            }
        })
        .size(px(256.));
        let level = |value: Option<f32>| value.map_or_else(String::new, |v| format!("{}", v.round() as i32));
        vec![
            self.channel_row(0, cx),
            div()
                .border_1()
                .border_color(rgb(INPUT_BORDER))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, ev: &MouseDownEvent, _, cx| {
                        this.press_curve(ev.position);
                        cx.notify();
                    }),
                )
                .child(graph)
                .into_any_element(),
            div()
                .flex()
                .gap_3()
                .child(div().flex().gap_1().child(label(t("Input:"))).child(level(shown.map(|p| p.0))))
                .child(div().flex().gap_1().child(label(t("Output:"))).child(level(shown.map(|p| p.1))))
                .into_any_element(),
        ]
    }

    /// A gradient to edit in place: ready-made ones to start from, the gradient with
    /// its stops under it, and the color, place and opacity of the stop that is picked.
    /// A press under the gradient adds a stop, a drag moves one, and one pulled away is removed.
    fn gradient_editor(&self, name: &'static str, name_width: f32, stops: Vec<crate::pseffects::Stop>, cx: &mut Context<Self>) -> AnyElement {
        const WIDTH: f32 = 220.;
        let opacity_stops = self.shown_track(true).unwrap_or_default();
        let on_opacity = self.gradient_opacity;
        let picked_any = self.gradient_stop.filter(|index| *index < if on_opacity { opacity_stops.len() } else { stops.len() });
        let (picked, picked_opacity) = if on_opacity { (None, picked_any) } else { (picked_any, None) };
        let cell = self.gradient_bounds.clone();
        let (ramp_stops, marks) = (crate::pseffects::baked(&stops, &opacity_stops), stops);
        // The opacity stops stand above the gradient, pointing down at it: black where it is opaque, white where it is clear.
        let opacity_marks = canvas(|_, _, _| (), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
            for (index, (place, color)) in opacity_stops.iter().enumerate() {
                let x = bounds.origin.x + px(place * f32::from(bounds.size.width));
                let bottom = bounds.origin.y + bounds.size.height;
                let edge = rgb(if picked_opacity == Some(index) { TEXT } else { INPUT_BORDER });
                let mut roof = gpui::PathBuilder::fill();
                roof.move_to(point(x, bottom));
                roof.line_to(point(x + px(5.), bottom - px(5.)));
                roof.line_to(point(x - px(5.), bottom - px(5.)));
                roof.close();
                if let Ok(roof) = roof.build() {
                    window.paint_path(roof, edge);
                }
                let body = Bounds { origin: point(x - px(5.), bottom - px(13.)), size: size(px(10.), px(8.)) };
                let gray = 255 - color.a as u32;
                window.paint_quad(fill(body, rgb(gray << 16 | gray << 8 | gray)).border_widths(if picked_opacity == Some(index) { 2. } else { 1. }).border_color(edge));
            }
        })
        .w(px(WIDTH))
        .h(px(14.));
        let ramp = canvas(move |bounds, _, _| cell.set(bounds), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
            let (width, height) = (f32::from(bounds.size.width), f32::from(bounds.size.height));
            for x in 0..width.ceil() as usize {
                let (color, opacity) = crate::pseffects::gradient_color(&ramp_stops, x as f32 / (width - 1.0).max(1.0));
                // Where the gradient is see-through, a checkerboard shows through it: the upper half on light, the lower on dark squares.
                for half in 0..2 {
                    let back = if (x / 4 + half) % 2 == 0 { 1.0 } else { 0.75 };
                    let channel = |c: usize| ((back + (color[c] - back) * opacity) * 255.0).round().clamp(0.0, 255.0) as u32;
                    let origin = point(bounds.origin.x + px(x as f32), bounds.origin.y + px(half as f32 * height / 2.0));
                    window.paint_quad(fill(Bounds { origin, size: size(px(1.2), px(height / 2.0)) }, rgb(channel(0) << 16 | channel(1) << 8 | channel(2))));
                }
            }
        })
        .w(px(WIDTH))
        .h(px(16.));
        let stop_marks = canvas(|_, _, _| (), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
            for (index, (place, color)) in marks.iter().enumerate() {
                let x = bounds.origin.x + px(place * f32::from(bounds.size.width));
                let top = bounds.origin.y;
                // A small house under the gradient, pointing at its place, in the color of the stop.
                let mut roof = gpui::PathBuilder::fill();
                roof.move_to(point(x, top));
                roof.line_to(point(x + px(5.), top + px(5.)));
                roof.line_to(point(x - px(5.), top + px(5.)));
                roof.close();
                if let Ok(roof) = roof.build() {
                    window.paint_path(roof, rgb(if picked == Some(index) { TEXT } else { INPUT_BORDER }));
                }
                let body = Bounds { origin: point(x - px(5.), top + px(5.)), size: size(px(10.), px(8.)) };
                let shade = (color.r as u32) << 16 | (color.g as u32) << 8 | color.b as u32;
                window.paint_quad(fill(body, rgb(shade)).border_widths(if picked == Some(index) { 2. } else { 1. }).border_color(rgb(if picked == Some(index) { TEXT } else { INPUT_BORDER })));
            }
        })
        .w(px(WIDTH))
        .h(px(14.));
        div()
            .flex()
            .flex_col()
            .gap_1()
            .child(labelled_row(name, name_width, self.combo(Combo::GradientPreset, t("Presets"), 150., cx).bg(rgb(PANEL_RAISED))))
            .child(
                div()
                    .w(px(WIDTH))
                    .ml(px(5.))
                    .flex()
                    .flex_col()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, ev: &MouseDownEvent, _, cx| {
                            this.press_gradient(ev.position);
                            cx.notify();
                        }),
                    )
                    .child(opacity_marks)
                    .child(div().border_1().border_color(rgb(INPUT_BORDER)).child(ramp))
                    .child(stop_marks),
            )
            // A color stop has a color and a place; an opacity stop an opacity and a place.
            .when(picked.is_some(), |d| {
                d.child(div().flex().items_center().gap_1().child(self.swatch(ColorTarget::GradientStop, 18., cx)).child(label(t("Location:"))).child(self.field(FieldId::GradientPlace, 36., cx)))
            })
            .when(picked_opacity.is_some(), |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(label(t("Opacity:")))
                        .child(self.field(FieldId::GradientOpacity, 36., cx))
                        .child(label(t("Location:")))
                        .child(self.field(FieldId::GradientPlace, 36., cx)),
                )
            })
            .into_any_element()
    }

    /// Photoshop Live Effects: the effects in a list, each with a box that switches it
    /// on, and beside it the settings of the one picked.
    fn photoshop_rows(&self, filter: &crate::effects::LiveFilter, cx: &mut Context<Self>) -> Vec<AnyElement> {
        use crate::pseffects::{index, label, row_shows, rows, Row, NAMES, ORDER};
        let shown = if self.ps_effect < NAMES.len() { self.ps_effect } else { 0 };
        let list: Vec<AnyElement> = ORDER
            .iter()
            .map(|effect| {
                let effect = *effect;
                let on = filter.value(index(effect, 0)) >= 0.5;
                div()
                    .h(px(20.))
                    .px_1()
                    .flex()
                    .items_center()
                    .gap_1()
                    .when(effect == shown, |d| d.bg(rgb(ACCENT)))
                    .child(
                        div()
                            .size(px(13.))
                            .flex()
                            .flex_none()
                            .items_center()
                            .justify_center()
                            .bg(rgb(INPUT))
                            .border_1()
                            .border_color(rgb(INPUT_BORDER))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.commit_field();
                                    (this.ps_effect, this.gradient_stop) = (effect, None);
                                    this.set_filter_value(index(effect, 0), if on { 0.0 } else { 1.0 });
                                    cx.notify();
                                }),
                            )
                            .when(on, |d| d.child(icon("check", 9., TEXT))),
                    )
                    .child(
                        div()
                            .flex_1()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, _, cx| {
                                    this.commit_field();
                                    (this.ps_effect, this.gradient_stop) = (effect, None);
                                    cx.notify();
                                }),
                            )
                            .child(label(NAMES[effect])),
                    )
                    .into_any_element()
            })
            .collect();
        let lines: Vec<&Row> = rows(shown).iter().filter(|row| row_shows(filter, shown, row)).collect();
        let names: Vec<&'static str> = lines
            .iter()
            .filter_map(|row| match **row {
                Row::Number { name, .. } | Row::List { name, .. } | Row::Blend { name, .. } | Row::Color { name, .. } | Row::Gradient { name, .. } => Some(label(name)),
                Row::Heading(_) | Row::Toggle { .. } => None,
            })
            .collect();
        let name_width = label_width(&names, 60.);
        let mut settings: Vec<AnyElement> = vec![div().pb_1().font_weight(FontWeight::BOLD).child(label(NAMES[shown])).into_any_element()];
        for row in lines {
            settings.push(match *row {
                Row::Heading(name) => div().pt_1().font_weight(FontWeight::BOLD).child(label(name)).into_any_element(),
                Row::Number { name, slot, .. } => labelled_row(label(name), name_width, self.field(FieldId::FilterValue(index(shown, slot)), 60., cx)).into_any_element(),
                Row::Toggle { name, slot, .. } => self.filter_toggle(index(shown, slot), label(name), filter.value(index(shown, slot)) >= 0.5, cx).into_any_element(),
                Row::List { name, slot, items, .. } => {
                    let current = (filter.value(index(shown, slot)).round() as usize).min(items.len() - 1);
                    labelled_row(label(name), name_width, self.combo(Combo::FilterChoice(index(shown, slot)), label(items[current]), 150., cx).bg(rgb(PANEL_RAISED))).into_any_element()
                }
                Row::Blend { name, slot, .. } => {
                    let modes = crate::blend::BlendMode::ALL;
                    let current = (filter.value(index(shown, slot)).round() as usize).min(modes.len() - 1);
                    labelled_row(label(name), name_width, self.combo(Combo::FilterChoice(index(shown, slot)), modes[current].label(), 150., cx).bg(rgb(PANEL_RAISED))).into_any_element()
                }
                Row::Color { name, color } => labelled_row(label(name), name_width, self.swatch(ColorTarget::PsEffect(color), 18., cx)).into_any_element(),
                Row::Gradient { name, gradient } => self.gradient_editor(label(name), name_width, filter.gradient_stops(gradient, false), cx),
            });
        }
        vec![
            div()
                .flex()
                .gap_3()
                .child(div().w(px(160.)).flex().flex_col().flex_none().bg(rgb(PANEL_RAISED)).border_1().border_color(rgb(INPUT_BORDER)).children(list))
                .child(div().flex_1().min_h(px(345.)).flex().flex_col().gap_1().children(settings))
                .into_any_element(),
        ]
    }

    pub(crate) fn jpeg_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.jpeg_dialog?;
        let settings = self.ed.doc.page().export;
        Some(
            deferred(
                div().absolute().top_0().left_0().size_full().flex().items_center().justify_center().occlude().child(
                    div()
                        .w(px(280.))
                        .p_3()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .bg(rgb(PANEL))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .shadow_lg()
                        .child(div().font_weight(FontWeight::BOLD).child(t("Selective JPEG Settings")))
                        .child(self.check_box(t("Enable selective quality"), settings.selective, cx, |this| this.set_export(|s| s.selective = !s.selective)))
                        .child(div().flex().items_center().gap_1().child(label(t("Selective quality:"))).child(self.field(FieldId::OptimizeSelective, 44., cx)))
                        .child(div().flex().items_center().gap_1().child(label(t("Overlay color:"))).child(self.swatch(ColorTarget::JpegMask, 18., cx)))
                        .child(self.check_box(t("Preserve text quality"), settings.preserve_text, cx, |this| this.set_export(|s| s.preserve_text = !s.preserve_text)))
                        .child(self.check_box(t("Preserve button quality"), settings.preserve_buttons, cx, |this| this.set_export(|s| s.preserve_buttons = !s.preserve_buttons)))
                        .child(
                            div()
                                .flex()
                                .justify_end()
                                .gap_1()
                                .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_jpeg_dialog(true)))
                                .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_jpeg_dialog(false))),
                        ),
                ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    /// The dialog that edits the fill of the selection: the stops of a gradient, or which pattern and how large.
    pub(crate) fn fill_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.fill_dialog.as_ref()?;
        let style = self.current_fill_style()?;
        let pattern = style.kind == crate::model::FillKind::Pattern;
        let names = [t("Pattern:"), t("Scale:"), t("Gradient:")];
        let name_width = label_width(&names, 60.);
        let rows: Vec<AnyElement> = if pattern {
            let shown = crate::pseffects::PATTERNS[(style.pattern as usize).min(crate::pseffects::PATTERNS.len() - 1)];
            vec![
                labelled_row(names[0], name_width, self.combo(Combo::FillPattern, crate::pseffects::label(shown), 150., cx).bg(rgb(PANEL_RAISED))).into_any_element(),
                labelled_row(names[1], name_width, div().flex().items_center().gap_1().child(self.field(FieldId::FillScale, 50., cx)).child(label("%"))).into_any_element(),
            ]
        } else {
            vec![self.gradient_editor(names[2], name_width, style.stops.clone(), cx)]
        };
        Some(
            deferred(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .flex()
                    .items_end()
                    .justify_center()
                    .pb(px(150.))
                    .occlude()
                    // A stop follows the mouse anywhere over the window, not only under the gradient.
                    .on_mouse_move(cx.listener(|this, ev: &gpui::MouseMoveEvent, _, cx| {
                        if this.drag_gradient(ev.position) {
                            cx.notify();
                        }
                    }))
                    .on_mouse_up(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if this.gradient_drag.take().is_some() {
                                this.ed.commit(t("Fill"));
                            }
                            cx.notify();
                        }),
                    )
                    .child(
                        div()
                            .w(px(270.))
                            .p_3()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .bg(rgb(PANEL))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .shadow_lg()
                            .child(div().font_weight(FontWeight::BOLD).child(tf("{} Fill", &[&style.kind.label()])))
                            .children(rows)
                            .child(
                                div()
                                    .flex()
                                    .justify_end()
                                    .gap_1()
                                    .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_fill_dialog(true)))
                                    .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_fill_dialog(false))),
                            ),
                    ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    /// File > Image Preview: the Optimize settings on the left, the image they give
    /// on the right with its format and size, and a button that saves it.
    pub(crate) fn image_preview_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let preview = self.image_preview.as_ref()?;
        let (image, caption) = match &preview.pane {
            Some(pane) => (Some(pane.image.clone()), pane.caption.clone()),
            None => (None, t("Estimating size...").to_string()),
        };
        let (width, height) = preview.size;
        let picture = canvas(|_, _, _| (), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
            window.paint_quad(fill(bounds, rgb(PASTEBOARD)));
            let Some(image) = image.clone() else { return };
            // As large as fits, but never enlarged: a small image is shown pixel for pixel.
            let (room_w, room_h) = (f32::from(bounds.size.width) - 16.0, f32::from(bounds.size.height) - 16.0);
            let scale = (room_w / width as f32).min(room_h / height as f32).min(1.0);
            let (w, h) = (width as f32 * scale, height as f32 * scale);
            let origin = point(bounds.origin.x + px((f32::from(bounds.size.width) - w) / 2.0), bounds.origin.y + px((f32::from(bounds.size.height) - h) / 2.0));
            let area = Bounds { origin, size: size(px(w), px(h)) };
            window.paint_quad(fill(area, rgb(INPUT)));
            let _ = window.paint_image(area, gpui::Corners::default(), image, 0, false);
        })
        .w(px(440.))
        .h(px(330.));
        Some(
            deferred(
                div().absolute().top_0().left_0().size_full().flex().items_center().justify_center().occlude().child(
                    div()
                        .p_3()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .bg(rgb(PANEL))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .shadow_lg()
                        .child(div().font_weight(FontWeight::BOLD).child(t("Image Preview")))
                        .child(
                            div()
                                .flex()
                                .gap_3()
                                .child(div().w(px(300.)).h(px(330.)).flex().flex_col().child(self.optimize_body(cx)))
                                .child(div().flex().flex_col().gap_1().child(div().border_1().border_color(rgb(INPUT_BORDER)).child(picture)).child(label(format!("{caption}   {width} x {height}")))),
                        )
                        .child(
                            div()
                                .flex()
                                .justify_end()
                                .gap_1()
                                .child(self.text_button(t("Export..."), true, cx, |this, window, cx| this.export_image_preview(window, cx)))
                                .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_image_preview(true)))
                                .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_image_preview(false))),
                        ),
                ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    /// A dialog in the middle of the window, over everything else.
    fn dialog_frame(&self, width: f32, title: &'static str, rows: Vec<AnyElement>, buttons: Div) -> AnyElement {
        deferred(
            div().absolute().top_0().left_0().size_full().flex().items_center().justify_center().occlude().child(
                div()
                    .w(px(width))
                    .p_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .bg(rgb(PANEL))
                    .border_1()
                    .border_color(rgb(BORDER))
                    .shadow_lg()
                    .child(div().font_weight(FontWeight::BOLD).child(title))
                    .children(rows)
                    .child(buttons),
            ),
        )
        .with_priority(1)
        .into_any_element()
    }

    /// Commands > Manage Saved Commands: the saved commands, to rename or delete.
    pub(crate) fn commands_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let picked = self.commands_dialog?;
        let list: Vec<AnyElement> = self
            .saved_commands
            .iter()
            .enumerate()
            .map(|(index, (name, _))| {
                div()
                    .h(px(18.))
                    .px_1()
                    .flex()
                    .flex_none()
                    .items_center()
                    .when(picked == Some(index), |d| d.bg(rgb(ACCENT)))
                    .hover(|s| s.bg(rgb(HOVER)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.commands_dialog = Some(Some(index));
                            cx.notify();
                        }),
                    )
                    .child(name.clone())
                    .into_any_element()
            })
            .collect();
        let mut rows = vec![div().id("command-list").h(px(180.)).flex().flex_col().overflow_y_scroll().bg(rgb(INPUT)).border_1().border_color(rgb(INPUT_BORDER)).children(list).into_any_element()];
        if self.saved_commands.is_empty() {
            rows.push(div().text_color(rgb(TEXT_DIM)).child(t("No saved commands yet. Use Save Steps as Command in the History panel.")).into_any_element());
        }
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("Rename..."), picked.is_some(), cx, move |this, _, _| {
                if let Some((name, _)) = picked.and_then(|index| this.saved_commands.get(index)) {
                    this.command_name = Some((name.clone(), picked));
                }
            }))
            .child(self.text_button(t("Delete"), picked.is_some(), cx, |this, _, _| this.delete_saved_command()))
            .child(self.text_button(t("Close"), true, cx, |this, _, _| this.commands_dialog = None));
        Some(self.dialog_frame(300., t("Manage Saved Commands"), rows, buttons))
    }

    /// The name of a command: for steps being saved, or for a saved command being renamed.
    pub(crate) fn command_name_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (_, renaming) = self.command_name.as_ref()?;
        let mut rows = vec![labelled_row(t("Name:"), 40., self.field(FieldId::CommandName, 190., cx)).into_any_element()];
        if renaming.is_none() {
            let steps = self.steps_to_save();
            rows.push(div().text_color(rgb(TEXT_DIM)).child(tf("Steps: {}", &[&steps.len()])).into_any_element());
            let list: Vec<AnyElement> = steps
                .into_iter()
                .map(|step| {
                    // A step is kept in English, as the menus are written; here it shows as the menus do.
                    let shown = step.split_once(" > ").map_or(step.clone(), |(menu, name)| format!("{} > {}", t(menu), t(name).trim_end_matches("...")));
                    div().h(px(16.)).px_1().flex().flex_none().items_center().child(shown).into_any_element()
                })
                .collect();
            rows.push(div().id("step-list").h(px(100.)).flex().flex_col().overflow_y_scroll().bg(rgb(INPUT)).border_1().border_color(rgb(INPUT_BORDER)).children(list).into_any_element());
        }
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_command_name(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_command_name(false)));
        Some(self.dialog_frame(280., t("Save Command"), rows, buttons))
    }

    /// Commands > Creative > Add Arrowheads: which ends, and how large.
    pub(crate) fn arrow_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (at_start, at_end, _) = self.arrow_dialog?;
        let rows = vec![
            self.check_box(t("Add to Start"), at_start, cx, |this| this.arrow_dialog.iter_mut().for_each(|d| d.0 = !d.0)).into_any_element(),
            self.check_box(t("Add to End"), at_end, cx, |this| this.arrow_dialog.iter_mut().for_each(|d| d.1 = !d.1)).into_any_element(),
            labelled_row(t("Size:"), 60., div().flex().items_center().gap_1().child(self.field(FieldId::ArrowSize, 50., cx)).child(label(t("pixels")))).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_arrow_dialog(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_arrow_dialog(false)));
        Some(self.dialog_frame(230., t("Add Arrowheads"), rows, buttons))
    }

    /// Commands > Creative > Add Picture Frame: how wide the frame is.
    pub(crate) fn frame_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.frame_dialog?;
        let rows = vec![labelled_row(t("Frame size:"), 90., div().flex().items_center().gap_1().child(self.field(FieldId::FrameSize, 50., cx)).child(label(t("pixels")))).into_any_element()];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_frame_dialog(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_frame_dialog(false)));
        Some(self.dialog_frame(230., t("Add Picture Frame"), rows, buttons))
    }

    /// Commands > Document > Reverse State Range: the first and the last state to change places.
    pub(crate) fn reverse_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.reverse_dialog?;
        let rows = vec![labelled_row(t("States:"), 60., div().flex().items_center().gap_1().child(self.field(FieldId::ReverseState(false), 44., cx)).child(label(t("to"))).child(self.field(FieldId::ReverseState(true), 44., cx))).into_any_element()];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_reverse_dialog(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_reverse_dialog(false)));
        Some(self.dialog_frame(230., t("Reverse State Range"), rows, buttons))
    }

    /// File > Batch Process: the files to work on, how to scale and export them, what to call the results and where to put them.
    pub(crate) fn batch_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let dialog = self.batch_dialog.as_ref()?;
        let names = [t("Scale:"), t("Add Prefix:"), t("Add Suffix:"), t("Output:")];
        let name_width = label_width(&names, 70.);
        let files: Vec<AnyElement> = dialog
            .files
            .iter()
            .map(|path| div().h(px(18.)).px_1().flex().flex_none().items_center().child(path.file_name().unwrap_or_default().to_string_lossy().into_owned()).into_any_element())
            .collect();
        let mut scale = div().flex().items_center().gap_1().child(self.combo(Combo::BatchScale, t(crate::workspace::BATCH_SCALES[dialog.scale.min(3)]), 150., cx).bg(rgb(PANEL_RAISED)));
        scale = match dialog.scale {
            1 | 2 => scale.child(self.field(FieldId::Batch(0), 44., cx)).child(label("x")).child(self.field(FieldId::Batch(1), 44., cx)),
            3 => scale.child(self.field(FieldId::Batch(2), 44., cx)).child(label("%")),
            _ => scale,
        };
        let output: SharedString = dialog.output.as_ref().map_or_else(|| t("Same location as original file").to_string(), |path| path.display().to_string()).into();
        let rows = vec![
            div().child(tf("Files: {}", &[&dialog.files.len()])).into_any_element(),
            div().id("batch-files").h(px(110.)).flex().flex_col().overflow_y_scroll().bg(rgb(INPUT)).border_1().border_color(rgb(INPUT_BORDER)).children(files).into_any_element(),
            div()
                .flex()
                .gap_1()
                .child(self.text_button(t("Add Files..."), true, cx, |this, window, cx| this.add_batch_files(window, cx)))
                .child(self.text_button(t("Remove All"), !dialog.files.is_empty(), cx, |this, _, _| this.batch_dialog.iter_mut().for_each(|d| d.files.clear())))
                .into_any_element(),
            labelled_row(names[0], name_width, scale).into_any_element(),
            self.check_box(t("Export with this document's Optimize settings"), dialog.use_current, cx, |this| this.batch_dialog.iter_mut().for_each(|d| d.use_current = !d.use_current)).into_any_element(),
            labelled_row(names[1], name_width, self.field(FieldId::Batch(3), 120., cx)).into_any_element(),
            labelled_row(names[2], name_width, self.field(FieldId::Batch(4), 120., cx)).into_any_element(),
            labelled_row(names[3], name_width, div().flex().items_center().gap_1().child(div().max_w(px(190.)).overflow_hidden().child(output)).child(self.text_button(t("Browse..."), true, cx, |this, window, cx| this.pick_batch_folder(window, cx)))).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("Batch"), !dialog.files.is_empty(), cx, |this, _, _| this.run_batch()))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.batch_dialog = None));
        Some(self.dialog_frame(380., t("Batch Process"), rows, buttons))
    }

    /// File > New from Template: the templates that come with the program, group by group, then the user's own.
    pub(crate) fn template_dialog_view(&self, window: &mut Window, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (saved, picked) = self.template_dialog.as_ref()?;
        let picked = *picked;
        let row = |index: usize, name: String, cx: &mut Context<Self>| {
            div()
                .h(px(18.))
                .pl_4()
                .flex()
                .flex_none()
                .items_center()
                .when(picked == index, |d| d.bg(rgb(ACCENT)))
                .hover(|s| s.bg(rgb(HOVER)))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                        if let Some(dialog) = this.template_dialog.as_mut() {
                            dialog.1 = index;
                        }
                        if ev.click_count >= 2 {
                            this.close_template_dialog(true);
                        }
                        cx.notify();
                    }),
                )
                .child(name)
                .into_any_element()
        };
        let heading = |text: &'static str| div().h(px(18.)).px_1().flex().flex_none().items_center().font_weight(FontWeight::BOLD).child(text).into_any_element();
        let mut list = Vec::new();
        let mut group = "";
        for (index, template) in crate::templates::BUILT_IN.iter().enumerate() {
            if template.group != group {
                group = template.group;
                list.push(heading(t(group)));
            }
            list.push(row(index, t(template.name).to_string(), cx));
        }
        if !saved.is_empty() {
            list.push(heading(t("My Templates")));
        }
        for (index, (name, _)) in saved.iter().enumerate() {
            list.push(row(crate::templates::BUILT_IN.len() + index, name.clone(), cx));
        }
        let rows = vec![div().id("template-list").h(px(280.)).flex().flex_col().overflow_y_scroll().bg(rgb(INPUT)).border_1().border_color(rgb(INPUT_BORDER)).children(list).into_any_element()];
        let _ = window;
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("Browse..."), true, cx, |this, window, cx| this.pick_file(window, cx, |this, path| this.open_as_template(path))))
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_template_dialog(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_template_dialog(false)));
        Some(self.dialog_frame(320., t("New from Template"), rows, buttons))
    }

    /// File > Save as Template: the name the template is kept under.
    pub(crate) fn template_name_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.template_name.as_ref()?;
        let rows = vec![labelled_row(t("Name:"), 40., self.field(FieldId::TemplateName, 190., cx)).into_any_element()];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_template_name(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_template_name(false)));
        Some(self.dialog_frame(270., t("Save as Template"), rows, buttons))
    }

    /// Edit > Insert > New Symbol: the name and the kind of the symbol to draw.
    pub(crate) fn symbol_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (_, kind) = self.symbol_dialog.as_ref()?;
        let names = [t("Name:"), t("Type:")];
        let name_width = label_width(&names, 40.);
        let pick = |this: &Self, which: SymbolKind, cx: &mut Context<Self>, set: fn(&mut Workspace)| this.check_box(which.label(), *kind == which, cx, set).into_any_element();
        let rows = vec![
            labelled_row(names[0], name_width, self.field(FieldId::NewSymbolName, 170., cx)).into_any_element(),
            labelled_row(names[1], name_width, div()).into_any_element(),
            pick(self, SymbolKind::Graphic, cx, |this| this.symbol_dialog.iter_mut().for_each(|d| d.1 = SymbolKind::Graphic)),
            pick(self, SymbolKind::Animation, cx, |this| this.symbol_dialog.iter_mut().for_each(|d| d.1 = SymbolKind::Animation)),
            pick(self, SymbolKind::Button, cx, |this| this.symbol_dialog.iter_mut().for_each(|d| d.1 = SymbolKind::Button)),
        ];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_symbol_dialog(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_symbol_dialog(false)));
        Some(self.dialog_frame(260., t("New Symbol"), rows, buttons))
    }

    /// Modify > Animation > Animate Selection: how many states, and how the object changes across them.
    pub(crate) fn animate_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let settings = self.animate_dialog?;
        let names = [t("States:"), t("Move:"), t("Direction:"), t("Scale to:"), t("Opacity:"), t("Rotate:")];
        let name_width = label_width(&names, 60.);
        let with = |this: &Self, which: usize, unit: &'static str, cx: &mut Context<Self>| div().flex().items_center().gap_1().child(this.field(FieldId::Animate(which), 50., cx)).child(label(unit));
        let rows = vec![
            labelled_row(names[0], name_width, self.field(FieldId::Animate(0), 50., cx)).into_any_element(),
            labelled_row(names[1], name_width, with(self, 1, t("pixels"), cx)).into_any_element(),
            labelled_row(names[2], name_width, with(self, 2, "°", cx)).into_any_element(),
            labelled_row(names[3], name_width, with(self, 3, "%", cx)).into_any_element(),
            labelled_row(names[4], name_width, with(self, 4, t("to"), cx).child(self.field(FieldId::Animate(5), 50., cx))).into_any_element(),
            labelled_row(names[5], name_width, with(self, 6, "°", cx)).into_any_element(),
            self.check_box(t("CW"), settings.clockwise, cx, |this| this.animate_dialog.iter_mut().for_each(|s| s.clockwise = true)).into_any_element(),
            self.check_box(t("CCW"), !settings.clockwise, cx, |this| this.animate_dialog.iter_mut().for_each(|s| s.clockwise = false)).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_animate_dialog(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_animate_dialog(false)));
        Some(self.dialog_frame(250., t("Animate"), rows, buttons))
    }

    /// Edit > Preferences: undo steps, how near the pointer picks and things snap, and the grid.
    pub(crate) fn preferences_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.prefs_dialog?;
        let names = [t("Undo steps:"), t("Pick distance:"), t("Snap distance:"), t("Grid:"), t("Color:")];
        let name_width = label_width(&names, 90.);
        let heading = |text: &'static str| div().pt_1().font_weight(FontWeight::BOLD).child(text).into_any_element();
        let rows = vec![
            heading(t("General")),
            labelled_row(names[0], name_width, self.field(FieldId::PrefUndo, 50., cx)).into_any_element(),
            heading(t("Edit")),
            labelled_row(names[1], name_width, div().flex().items_center().gap_1().child(self.field(FieldId::PrefPick, 50., cx)).child(label(t("pixels")))).into_any_element(),
            labelled_row(names[2], name_width, div().flex().items_center().gap_1().child(self.field(FieldId::PrefSnap, 50., cx)).child(label(t("pixels")))).into_any_element(),
            heading(t("Guides and Grids")),
            labelled_row(names[3], name_width, div().flex().items_center().gap_1().child(self.field(FieldId::PrefGridX, 44., cx)).child(label("x")).child(self.field(FieldId::PrefGridY, 44., cx)).child(label(t("pixels")))).into_any_element(),
            labelled_row(names[4], name_width, div().flex().items_center().gap_2().child(self.swatch(ColorTarget::Grid, 18., cx)).child(label(t("Guides"))).child(self.swatch(ColorTarget::Guides, 18., cx))).into_any_element(),
            self.check_box(t("Show Grid"), self.show_grid, cx, |this| this.show_grid = !this.show_grid).into_any_element(),
            self.check_box(t("Snap to Grid"), self.prefs.snap_to_grid, cx, |this| this.prefs.snap_to_grid = !this.prefs.snap_to_grid).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_preferences(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_preferences(false)));
        Some(self.dialog_frame(300., t("Preferences"), rows, buttons))
    }

    /// Edit > Keyboard Shortcuts: every menu command with its keys. A command picked in the list can be given new keys by pressing them.
    pub(crate) fn shortcuts_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (picked, capturing) = self.shortcut_dialog?;
        let commands = crate::menus::menu_commands();
        let list: Vec<AnyElement> = commands
            .iter()
            .enumerate()
            .map(|(index, (menu, name, keys, _))| {
                let changed = self.shortcuts.contains_key(*name);
                div()
                    .h(px(18.))
                    .px_1()
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_between()
                    .when(picked == Some(index), |d| d.bg(rgb(ACCENT)))
                    .hover(|s| s.bg(rgb(HOVER)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.shortcut_dialog = Some((Some(index), false));
                            cx.notify();
                        }),
                    )
                    .child(format!("{} > {}", t(menu), t(name).trim_end_matches("...")))
                    .child(div().when(changed, |d| d.font_weight(FontWeight::BOLD)).child(self.shortcut_of(name, keys)))
                    .into_any_element()
            })
            .collect();
        let hint = if capturing { t("Press the new shortcut, or Esc to keep the old one.") } else { t("Pick a command, then Change.") };
        let rows = vec![
            div().id("shortcut-list").h(px(300.)).flex().flex_col().overflow_y_scroll().bg(rgb(INPUT)).border_1().border_color(rgb(INPUT_BORDER)).children(list).into_any_element(),
            div().text_color(rgb(TEXT_DIM)).child(hint).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .flex_wrap()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("Change"), picked.is_some(), cx, move |this, _, _| this.shortcut_dialog = Some((picked, true))))
            .child(self.text_button(t("Remove"), picked.is_some(), cx, |this, _, _| this.set_shortcut(Some(String::new()))))
            .child(self.text_button(t("Reset"), picked.is_some(), cx, |this, _, _| this.set_shortcut(None)))
            .child(self.text_button(t("Reset All"), !self.shortcuts.is_empty(), cx, move |this, _, _| {
                this.shortcuts.clear();
                this.shortcut_dialog = Some((picked, false));
                this.save_preferences();
            }))
            .child(self.text_button(t("Close"), true, cx, |this, _, _| this.shortcut_dialog = None));
        Some(self.dialog_frame(430., t("Keyboard Shortcuts"), rows, buttons))
    }

    /// Text > Check Spelling: the word not found, what to change it to, the checker's suggestions, and what to do with it.
    pub(crate) fn spell_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let session = self.spell.as_ref()?;
        let (_, _, _, word) = self.spell_current()?;
        let suggestions: Vec<AnyElement> = session
            .wrong
            .get(&word)
            .into_iter()
            .flatten()
            .take(8)
            .map(|suggestion| {
                let picked = suggestion.clone();
                div()
                    .h(px(18.))
                    .px_1()
                    .flex()
                    .flex_none()
                    .items_center()
                    .when(*suggestion == session.replacement, |d| d.bg(rgb(ACCENT)))
                    .hover(|s| s.bg(rgb(HOVER)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.field = None;
                            if let Some(session) = this.spell.as_mut() {
                                session.replacement = picked.clone();
                            }
                            cx.notify();
                        }),
                    )
                    .child(suggestion.clone())
                    .into_any_element()
            })
            .collect();
        let names = [t("Word not found:"), t("Change to:"), t("Suggestions:")];
        let name_width = label_width(&names, 80.);
        let rows = vec![
            labelled_row(names[0], name_width, div().font_weight(FontWeight::BOLD).child(word)).into_any_element(),
            labelled_row(names[1], name_width, self.field(FieldId::SpellChange, 170., cx)).into_any_element(),
            labelled_row(names[2], name_width, div().w(px(170.)).h(px(146.)).flex().flex_col().bg(rgb(INPUT)).border_1().border_color(rgb(INPUT_BORDER)).children(suggestions)).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .flex_wrap()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("Add to Personal"), true, cx, |this, _, _| this.spell_add()))
            .child(self.text_button(t("Ignore"), true, cx, |this, _, _| this.spell_ignore(false)))
            .child(self.text_button(t("Ignore All"), true, cx, |this, _, _| this.spell_ignore(true)))
            .child(self.text_button(t("Change"), true, cx, |this, _, _| this.spell_change(false)))
            .child(self.text_button(t("Change All"), true, cx, |this, _, _| this.spell_change(true)))
            .child(self.text_button(t("Close"), true, cx, |this, _, _| {
                this.field = None;
                this.spell = None;
            }));
        Some(self.dialog_frame(330., t("Check Spelling"), rows, buttons))
    }

    /// Move Guide: the place of the guide that was double-clicked, in pixels from the edge of the page.
    pub(crate) fn guide_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.guide_dialog?;
        let rows = vec![div().flex().items_center().gap_1().child(label(t("Position:"))).child(self.field(FieldId::GuidePosition, 60., cx)).child(label(t("pixels"))).into_any_element()];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.apply_guide_dialog()))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| {
                this.field = None;
                this.guide_dialog = None;
            }));
        Some(self.dialog_frame(230., t("Move Guide"), rows, buttons))
    }

    /// View > Guides > Edit Guides: their color, whether they show, snap and are locked, and a button that clears them.
    pub(crate) fn guides_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.guides_dialog {
            return None;
        }
        let rows = vec![
            div().flex().items_center().gap_1().child(label(t("Color:"))).child(self.swatch(ColorTarget::Guides, 18., cx)).into_any_element(),
            self.check_box(t("Show Guides"), self.show_guides, cx, |this| this.show_guides = !this.show_guides).into_any_element(),
            self.check_box(t("Snap to Guides"), self.snap_guides, cx, |this| this.snap_guides = !this.snap_guides).into_any_element(),
            self.check_box(t("Lock Guides"), self.lock_guides, cx, |this| this.lock_guides = !this.lock_guides).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("Clear All"), !self.ed.doc.page().guides.is_empty(), cx, |this, _, _| {
                this.ed.doc.page_mut().guides = Default::default();
                this.ed.commit(t("Clear Guides"));
            }))
            .child(div().flex_1())
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.guides_dialog = false));
        Some(self.dialog_frame(240., t("Guides"), rows, buttons))
    }

    /// File > Page Setup: the paper, which way it is turned, the margins, and whether the page is scaled to fit.
    pub(crate) fn page_setup_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        self.page_setup_dialog?;
        let setup = self.print_setup;
        let names = [t("Paper:"), t("Orientation:"), t("Left:"), t("Right:"), t("Top:"), t("Bottom:")];
        let name_width = label_width(&names, 60.);
        let margin = |this: &Self, side: usize, cx: &mut Context<Self>| labelled_row(names[2 + side], name_width, div().flex().items_center().gap_1().child(this.field(FieldId::PrintMargin(side), 50., cx)).child(label("mm")));
        let rows = vec![
            labelled_row(names[0], name_width, self.combo(Combo::Paper, setup.paper.name(), 110., cx).bg(rgb(PANEL_RAISED))).into_any_element(),
            labelled_row(names[1], name_width, self.combo(Combo::Orientation, if setup.landscape { t("Landscape") } else { t("Portrait") }, 110., cx).bg(rgb(PANEL_RAISED))).into_any_element(),
            div().pt_1().font_weight(FontWeight::BOLD).child(t("Margins")).into_any_element(),
            margin(self, 0, cx).into_any_element(),
            margin(self, 1, cx).into_any_element(),
            margin(self, 2, cx).into_any_element(),
            margin(self, 3, cx).into_any_element(),
            self.check_box(t("Fit to page"), setup.fit, cx, |this| this.print_setup.fit = !this.print_setup.fit).into_any_element(),
        ];
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("OK"), true, cx, |this, _, _| this.close_page_setup(true)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| this.close_page_setup(false)));
        Some(self.dialog_frame(260., t("Page Setup"), rows, buttons))
    }

    /// File > Print: the printer, or a PDF file, and the number of copies.
    pub(crate) fn print_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let dialog = self.print_dialog.as_ref()?;
        let names = [t("Printer:"), t("Copies:"), t("Print range:")];
        let name_width = label_width(&names, 60.);
        let chosen: SharedString = dialog.printers.get(dialog.chosen).cloned().unwrap_or_else(|| t("Save as PDF...").to_string()).into();
        let to_printer = dialog.chosen < dialog.printers.len();
        let page = self.ed.doc.page();
        let mut rows = vec![labelled_row(names[0], name_width, self.combo(Combo::Printer, chosen, 190., cx).bg(rgb(PANEL_RAISED))).into_any_element()];
        if to_printer {
            rows.push(labelled_row(names[1], name_width, self.field(FieldId::PrintCopies, 50., cx)).into_any_element());
        }
        // Which pages of the document: the one on show, all of them, or from one to another.
        use crate::print::PageRange;
        let range_name = match dialog.range {
            PageRange::Current => t("Current page"),
            PageRange::All => t("All pages"),
            PageRange::Pages => t("Pages"),
        };
        rows.push(
            labelled_row(
                t("Print range:"),
                name_width,
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(self.combo(Combo::PrintRange, range_name, 110., cx).bg(rgb(PANEL_RAISED)))
                    .when(dialog.range == PageRange::Pages, |d| d.child(self.field(FieldId::PrintFrom, 30., cx)).child(label("-")).child(self.field(FieldId::PrintTo, 30., cx))),
            )
            .into_any_element(),
        );
        if dialog.printers.is_empty() {
            rows.push(div().text_color(rgb(TEXT_DIM)).child(t("No printer is set up on this computer.")).into_any_element());
        }
        if self.print_setup.overflows(page.width, page.height) {
            let sheets = self.print_setup.sheets(page.width, page.height).len();
            rows.push(div().text_color(rgb(TEXT_DIM)).child(tf("The page is larger than the paper and will be printed on {} sheets. Fit to page in Page Setup prints it on one.", &[&sheets])).into_any_element());
        }
        let buttons = div()
            .flex()
            .justify_end()
            .gap_1()
            .child(self.text_button(t("Page Setup..."), true, cx, |this, _, _| {
                this.commit_field();
                this.page_setup_dialog = Some(this.print_setup);
            }))
            .child(div().flex_1())
            .child(self.text_button(if to_printer { t("Print") } else { t("Save") }, true, cx, |this, window, cx| this.apply_print_dialog(window, cx)))
            .child(self.text_button(t("Cancel"), true, cx, |this, _, _| {
                this.field = None;
                this.print_dialog = None;
            }));
        Some(self.dialog_frame(330., t("Print"), rows, buttons))
    }

    /// The dialog of Expand, Contract, Border and Smooth Marquee: one number of pixels.
    pub(crate) fn marquee_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let (op, _) = self.marquee_dialog?;
        Some(
            deferred(
                div().absolute().top_0().left_0().size_full().flex().items_center().justify_center().occlude().child(
                    div()
                        .w(px(260.))
                        .p_3()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .bg(rgb(PANEL))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .shadow_lg()
                        .child(div().font_weight(FontWeight::BOLD).child(op.label()))
                        .child(div().flex().items_center().gap_1().child(label(op.prompt())).child(self.field(FieldId::MarqueeAmount, 60., cx)).child(label(t("pixels"))))
                        .child(
                            div()
                                .flex()
                                .justify_end()
                                .gap_1()
                                .child(self.text_button(t("OK"), true, cx, |this, _, _| this.apply_marquee_dialog()))
                                .child(self.text_button(t("Cancel"), true, cx, |this, _, _| {
                                    this.field = None;
                                    this.marquee_dialog = None;
                                })),
                        ),
                ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    pub(crate) fn numeric_dialog_view(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let dialog = self.numeric_dialog?;
        let names = [t("Width:"), t("Height:"), t("Angle:")];
        let name_width = label_width(&names, 50.);
        let number_row = |this: &Self, name: &'static str, id: FieldId, unit: &'static str, cx: &mut Context<Self>| {
            labelled_row(name, name_width, div().flex().items_center().gap_1().child(this.field(id, 70., cx)).child(label(unit)))
        };
        let checkbox = |text: &'static str, on: bool, cx: &mut Context<Self>, toggle: fn(&mut NumericDialog)| {
            div()
                .flex()
                .items_center()
                .gap_1()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.commit_field();
                        if let Some(dialog) = this.numeric_dialog.as_mut() {
                            toggle(dialog);
                        }
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .size(px(13.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .bg(rgb(INPUT))
                        .border_1()
                        .border_color(rgb(INPUT_BORDER))
                        .when(on, |d| d.child(icon("check", 9., TEXT))),
                )
                .child(text)
        };
        let unit = if dialog.mode == NumericMode::Scale { "%" } else { t("pixels") };
        let sized = dialog.mode != NumericMode::Rotate;
        Some(
            deferred(
                div().absolute().top_0().left_0().size_full().flex().items_center().justify_center().occlude().child(
                    div()
                        .w(px(260.))
                        .p_3()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .bg(rgb(PANEL))
                        .border_1()
                        .border_color(rgb(BORDER))
                        .shadow_lg()
                        .child(div().font_weight(FontWeight::BOLD).child(t("Numeric Transform")))
                        .child(self.combo(Combo::NumericMode, dialog.mode.label(), 120., cx).bg(rgb(PANEL_RAISED)))
                        .when(sized, |d| {
                            d.child(number_row(self, names[0], FieldId::NumericA, unit, cx))
                                .child(number_row(self, names[1], FieldId::NumericB, unit, cx))
                                .child(checkbox(t("Scale Attributes"), dialog.scale_attributes, cx, |d| d.scale_attributes = !d.scale_attributes))
                                .child(checkbox(t("Constrain Proportions"), dialog.constrain, cx, |d| d.constrain = !d.constrain))
                        })
                        .when(!sized, |d| d.child(number_row(self, names[2], FieldId::NumericA, t("degrees"), cx)))
                        .child(
                            div()
                                .flex()
                                .justify_end()
                                .gap_1()
                                .child(self.text_button(t("OK"), true, cx, |this, _, _| this.apply_numeric_dialog()))
                                .child(self.text_button(t("Cancel"), true, cx, |this, _, _| {
                                    this.field = None;
                                    this.numeric_dialog = None;
                                })),
                        ),
                ),
            )
            .with_priority(1)
            .into_any_element(),
        )
    }

    // ---- States ---------------------------------------------------------------------

    pub(crate) fn states_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let page = self.ed.doc.page();
        let active = page.active_state;
        let rows: Vec<_> = page
            .states
            .iter()
            .enumerate()
            .map(|(i, state)| {
                Self::list_row()
                    .when(i == active, |d| d.bg(rgb(ACCENT)))
                    .when(i != active, |d| d.hover(|s| s.bg(rgb(HOVER))))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                            // A click on the delay field has already started editing it.
                            if this.keep_overlays {
                                return;
                            }
                            this.finish_pending();
                            this.stop_playing();
                            this.ed.set_active_state(i);
                            if ev.click_count >= 2 {
                                this.begin_field(FieldId::RenameState(i));
                            }
                            cx.notify();
                        }),
                    )
                    .child(Self::toggle("eye", state.exported).on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            let state = &mut this.ed.doc.page_mut().states[i];
                            state.exported = !state.exported;
                            this.ed.commit(t("State Export"));
                            cx.notify();
                        }),
                    ))
                    .child(div().w(px(14.)).flex_none().text_color(rgb(TEXT_DIM)).child(format!("{}", i + 1)))
                    .child(self.name_or_field(FieldId::RenameState(i), state.name.clone(), 120., cx))
                    .child(self.field(FieldId::StateDelay(i), 40., cx))
                    .child(label("ms"))
            })
            .collect();
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(div().id("states").flex_1().min_h_0().flex().flex_col().overflow_y_scroll().bg(rgb(LIST)).children(rows))
            .child(
                Self::footer()
                    .child(self.icon_button(if self.playing { "stop" } else { "play" }, t("Play or stop the animation"), self.playing, cx, |this, cx| this.toggle_play(cx)))
                    .child(self.icon_button("onion", t("Onion skinning: show neighbouring states"), self.onion, cx, |this, _| this.onion = !this.onion))
                    .child(self.icon_button("duplicate", t("Duplicate State"), false, cx, |this, _| {
                        this.stop_playing();
                        this.ed.selection.clear();
                        this.ed.leave_symbol_for_good();
                        this.ed.doc.page_mut().duplicate_state();
                        this.ed.commit(t("Duplicate State"));
                    }))
                    .child(self.icon_button("plus", t("New State"), false, cx, |this, _| {
                        this.stop_playing();
                        this.ed.selection.clear();
                        this.ed.leave_symbol_for_good();
                        this.ed.doc.page_mut().add_state();
                        this.ed.commit(t("New State"));
                    }))
                    .child(self.icon_button("minus", t("Delete State"), false, cx, |this, _| {
                        this.stop_playing();
                        this.ed.selection.clear();
                        this.ed.leave_symbol_for_good();
                        if this.ed.doc.page_mut().delete_state() {
                            this.ed.commit(t("Delete State"));
                        } else {
                            this.status = t("A page needs at least one state").into();
                        }
                    })),
            )
            .into_any_element()
    }

    // ---- Styles ---------------------------------------------------------------------

    pub(crate) fn styles_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let tiles: Vec<_> = self
            .ed
            .doc
            .styles
            .iter()
            .enumerate()
            .map(|(i, style)| {
                let selected = self.style_selection == Some(i);
                self.tile(
                    format!("style-{i}"),
                    style.name.clone(),
                    vec![style_preview(style.fill, style.stroke, style.opacity)],
                    selected,
                    cx,
                    Box::new(move |this, _, _, _| this.use_style(i)),
                )
            })
            .collect();
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(div().h(px(26.)).px_1().flex().flex_none().items_center().child(self.disabled_combo(t("Current Document"), 256.)))
            .child(div().id("styles").flex_1().min_h_0().p_1().overflow_y_scroll().bg(rgb(INPUT)).border_t_1().border_color(rgb(LIST_BORDER)).child(div().w(px(252.)).flex().flex_wrap().children(tiles)))
            .child(
                Self::footer()
                    .child(self.icon_button("plus", t("New Style from Selection"), false, cx, |this, _| {
                        if this.ed.add_style_from_selection() {
                            this.ed.commit(t("New Style"));
                        } else {
                            this.status = t("Select a vector object to make a style from it").into();
                        }
                    }))
                    .child(self.icon_button("minus", t("Delete Style"), false, cx, |this, _| {
                        match this.style_selection.take().filter(|i| *i < this.ed.doc.styles.len()) {
                            Some(index) => {
                                this.ed.doc.styles.remove(index);
                                this.ed.commit(t("Delete Style"));
                            }
                            None => this.status = t("Click a style first").into(),
                        }
                    })),
            )
            .into_any_element()
    }

    // ---- Path -----------------------------------------------------------------------

    pub(crate) fn path_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let button = |this: &Self, op: PathOp, cx: &mut Context<Self>| {
            let enabled = this.is_enabled(Cmd::PathOp(op));
            this.text_button(op.label(), enabled, cx, move |this, window, cx| this.run(Cmd::PathOp(op), window, cx))
        };
        let group = |this: &Self, ops: &[PathOp], cx: &mut Context<Self>| {
            div().flex().flex_wrap().gap_1().children(ops.iter().map(|op| button(this, *op, cx)).collect::<Vec<_>>())
        };
        div()
            .id("path-panel")
            .size_full()
            .p_1()
            .flex()
            .flex_col()
            .gap_1()
            .overflow_y_scroll()
            .child(heading(t("Combine Paths")))
            .child(group(self, &[PathOp::Union, PathOp::Intersect, PathOp::Punch, PathOp::Crop, PathOp::Join, PathOp::Split], cx))
            .child(heading(t("Alter Paths")))
            .child(group(self, &[PathOp::Simplify, PathOp::ExpandStroke, PathOp::Inset, PathOp::Outset], cx))
            .child(labelled_row(t("Amount"), 48., div().flex().items_center().gap_1().child(self.field(FieldId::PathAmount, 44., cx)).child(label("px"))))
            .into_any_element()
    }

    // ---- Auto Shapes ----------------------------------------------------------------

    pub(crate) fn auto_shapes_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let tiles: Vec<_> = AutoKind::ALL
            .iter()
            .map(|kind| {
                let kind = *kind;
                self.captioned_tile(format!("auto-{}", kind.label()), kind.label().into(), vec![auto_preview(kind)], cx, Box::new(move |this, ev, _, _| this.begin_panel_drag(PanelItem::AutoShape(kind), ev.position)))
            })
            .collect();
        div().id("auto-shapes").size_full().p_1().overflow_y_scroll().bg(rgb(INPUT)).child(div().w(px(252.)).flex().flex_wrap().children(tiles)).into_any_element()
    }

    pub(crate) fn auto_properties_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let selected = self.ed.selected();
        let Some(kind) = selected.iter().find_map(|o| match o.shape {
            Shape::Auto { kind, .. } => Some(kind),
            _ => None,
        }) else {
            return div().size_full().flex().items_center().justify_center().child(label(t("Select an auto shape"))).into_any_element();
        };
        let rows: Vec<_> = kind
            .params()
            .iter()
            .enumerate()
            .filter_map(|(slot, info)| info.map(|info| (slot, info)))
            .map(|(slot, info)| {
                let suffix = if info.max <= 1.0 { "%" } else { "" };
                labelled_row(info.label, 80., div().flex().items_center().gap_1().child(self.field(FieldId::AutoParam(slot), 52., cx)).child(label(suffix)))
            })
            .collect();
        div()
            .p_2()
            .flex()
            .flex_col()
            .child(div().pb_1().font_weight(FontWeight::BOLD).child(kind.label()))
            .children(rows)
            .child(div().pt_1().child(self.text_button(t("Convert to Path"), true, cx, |this, _, _| {
                this.ed.for_selected(|o| {
                    if matches!(o.shape, Shape::Auto { .. }) {
                        o.to_path();
                    }
                });
                this.ed.commit(t("Convert to Path"));
            })))
            .into_any_element()
    }

    // ---- Libraries ------------------------------------------------------------------

    pub(crate) fn document_library_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let picked = self.symbol_selection.filter(|id| self.ed.doc.symbols.iter().any(|s| s.id == *id));
        let tiles: Vec<_> = self
            .ed
            .doc
            .symbols
            .iter()
            .map(|symbol| {
                let id = symbol.id;
                self.tile(
                    format!("symbol-{id}"),
                    symbol.name.clone(),
                    symbol.objects.iter().flat_map(Object::leaves).collect(),
                    picked == Some(id),
                    cx,
                    Box::new(move |this, ev, _, _| {
                        this.symbol_selection = Some(id);
                        if ev.click_count >= 2 {
                            this.place_symbol(id);
                        } else {
                            this.begin_panel_drag(PanelItem::Symbol(id), ev.position);
                        }
                    }),
                )
            })
            .collect();
        let has_selection = !self.ed.selection.is_empty();
        let grid = if tiles.is_empty() {
            div().size_full().flex().items_center().justify_center().child(label(t("Select objects, then press New"))).into_any_element()
        } else {
            div().w(px(252.)).flex().flex_wrap().children(tiles).into_any_element()
        };
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(div().id("doc-library").flex_1().min_h_0().p_1().overflow_y_scroll().child(grid))
            .when_some(picked, |d, id| d.child(div().px_1().child(labelled_row(t("Name"), 34., self.field(FieldId::RenameSymbol(id), 200., cx)))))
            .child(
                Self::footer()
                    .child(self.text_button(t("New"), has_selection, cx, |this, window, cx| this.run(Cmd::ConvertToSymbol, window, cx)))
                    .child(self.text_button(t("Insert"), picked.is_some(), cx, move |this, _, _| {
                        if let Some(id) = picked {
                            this.place_symbol(id);
                        }
                    }))
                    .child(self.text_button(t("Update"), picked.is_some() && has_selection, cx, move |this, _, _| {
                        let Some(id) = picked else { return };
                        if this.ed.redefine_symbol(id) {
                            this.ed.commit(t("Update Symbol"));
                        } else {
                            this.status = t("A symbol cannot contain a copy of itself").into();
                        }
                    }))
                    .child(self.text_button(t("Delete"), picked.is_some(), cx, move |this, _, _| {
                        if let Some(id) = picked {
                            this.ed.delete_symbol(id);
                            this.symbol_selection = None;
                            this.ed.commit(t("Delete Symbol"));
                        }
                    })),
            )
            .into_any_element()
    }

    pub(crate) fn common_library_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let mut children: Vec<AnyElement> = Vec::new();
        let mut category = "";
        let mut tiles: Vec<AnyElement> = Vec::new();
        let flush = |children: &mut Vec<AnyElement>, tiles: &mut Vec<AnyElement>| {
            if !tiles.is_empty() {
                children.push(div().w(px(252.)).flex().flex_wrap().children(std::mem::take(tiles)).into_any_element());
            }
        };
        for (index, item) in common_items().iter().enumerate() {
            if item.category != category {
                flush(&mut children, &mut tiles);
                category = item.category;
                children.push(heading(t(category)).into_any_element());
            }
            let objects = item.objects().iter().flat_map(Object::leaves).collect();
            tiles.push(
                self.captioned_tile(format!("common-{}", item.name), t(item.name).into(), objects, cx, Box::new(move |this, ev, _, _| this.begin_panel_drag(PanelItem::Common(index), ev.position)))
                    .into_any_element(),
            );
        }
        flush(&mut children, &mut tiles);
        div().id("common-library").size_full().p_1().flex().flex_col().overflow_y_scroll().bg(rgb(INPUT)).children(children).into_any_element()
    }

    // ---- Color Mixer ----------------------------------------------------------------

    pub(crate) fn mixer_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let current = self.current_color(self.mixer_target);
        let chip = |this: &Self, text: &'static str, target: ColorTarget, cx: &mut Context<Self>| {
            let on = this.mixer_target == target;
            div()
                .h(px(20.))
                .px_2()
                .flex()
                .items_center()
                .when(on, |d| d.bg(rgb(ACCENT)))
                .when(!on, |d| d.bg(rgb(PANEL_RAISED)).hover(|s| s.bg(rgb(HOVER))))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, _, cx| {
                        this.finish_pending();
                        this.mixer_target = target;
                        cx.notify();
                    }),
                )
                .child(text)
        };
        let base = current.unwrap_or(Color::BLACK);
        let channels: Vec<_> = (0..3usize)
            .map(|channel| {
                let cell = self.mixer_bounds[channel].clone();
                let ramp = canvas(move |bounds, _, _| cell.set(bounds), move |bounds: Bounds<gpui::Pixels>, _, window: &mut Window, _| {
                    let step = f32::from(bounds.size.width) / RAMP_STEPS as f32;
                    for i in 0..RAMP_STEPS {
                        let value = (i * 255 / (RAMP_STEPS - 1)) as u8;
                        let color = match channel {
                            0 => Color { r: value, ..base },
                            1 => Color { g: value, ..base },
                            _ => Color { b: value, ..base },
                        };
                        let origin = point(bounds.origin.x + px(i as f32 * step), bounds.origin.y);
                        window.paint_quad(fill(Bounds { origin, size: size(px(step + 0.5), bounds.size.height) }, rgb(color_u32(color))));
                    }
                })
                .size_full();
                div()
                    .h(px(22.))
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap_1()
                    .child(div().w(px(12.)).child(label(["R", "G", "B"][channel])))
                    .child(
                        div()
                            .flex_1()
                            .h(px(14.))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                                    this.mixer_pick(channel, ev.position);
                                    cx.notify();
                                }),
                            )
                            .child(ramp),
                    )
                    .child(self.field(FieldId::Mixer(channel), 36., cx))
            })
            .collect();
        div()
            .p_2()
            .flex()
            .flex_col()
            .gap_1()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1()
                    .child(chip(self, t("Fill"), ColorTarget::Fill, cx))
                    .child(chip(self, t("Stroke"), ColorTarget::Stroke, cx))
                    .child(div().ml_auto().child(self.swatch(self.mixer_target, 20., cx)))
                    .child(label(current.map_or(t("None").to_string(), Color::hex))),
            )
            .children(channels)
            .into_any_element()
    }

    // ---- Info -----------------------------------------------------------------------

    pub(crate) fn info_body(&self) -> AnyElement {
        let p = self.cursor_doc;
        let bounds = self.ed.selection_bounds();
        let color = self.color_under(p);
        let row = |name: &'static str, value: String| div().h(px(18.)).flex().justify_between().child(label(name)).child(value);
        div()
            .p_2()
            .flex()
            .flex_col()
            .child(row("X", format!("{:.0}", p.x)))
            .child(row("Y", format!("{:.0}", p.y)))
            .child(row(t("Color"), color.map_or(t("None").to_string(), |c| format!("{}  (R{} G{} B{})", c.hex(), c.r, c.g, c.b))))
            .child(row(t("Selection"), bounds.map_or(t("None").to_string(), |b| tf("{} x {} at {}, {}", &[&b.w.round(), &b.h.round(), &b.x.round(), &b.y.round()]))))
            .into_any_element()
    }

    // ---- URL ------------------------------------------------------------------------

    pub(crate) fn url_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let rows: Vec<_> = self
            .ed
            .doc
            .urls
            .iter()
            .enumerate()
            .map(|(i, url)| {
                let chosen = self.url_selection == Some(i);
                Self::list_row()
                    .when(chosen, |d| d.bg(rgb(ACCENT)))
                    .when(!chosen, |d| d.hover(|s| s.bg(rgb(HOVER))))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            this.finish_pending();
                            this.use_url(i);
                            cx.notify();
                        }),
                    )
                    .child(div().overflow_hidden().child(url.clone()))
            })
            .collect();
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(div().p_1().flex_none().child(labelled_row(t("Add"), 26., self.field(FieldId::NewUrl, 220., cx))))
            .child(div().id("urls").flex_1().min_h_0().flex().flex_col().overflow_y_scroll().children(rows))
            .child(Self::footer().child(self.icon_button("minus", t("Delete URL"), false, cx, |this, _| {
                match this.url_selection.take().filter(|i| *i < this.ed.doc.urls.len()) {
                    Some(index) => {
                        this.ed.doc.urls.remove(index);
                        this.ed.commit(t("Delete URL"));
                    }
                    None => this.status = t("Click a URL first").into(),
                }
            })))
            .into_any_element()
    }

    // ---- Find -----------------------------------------------------------------------

    pub(crate) fn find_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let chip = |this: &Self, text: &'static str, on: bool, cx: &mut Context<Self>, action: fn(&mut Workspace)| {
            this.text_button(text, true, cx, move |this, _, _| action(this)).when(on, |d| d.bg(rgb(ACCENT)))
        };
        let text_mode = self.find_mode == FindMode::Text;
        let find_label_width = label_width(&[t("Find"), t("Change")], 44.);
        div()
            .p_2()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .gap_1()
                    .child(chip(self, t("Text"), text_mode, cx, |w| w.find_mode = FindMode::Text))
                    .child(chip(self, t("Color"), !text_mode, cx, |w| w.find_mode = FindMode::Color))
                    .child(div().ml_auto().child(chip(self, t("All Pages"), self.find_all_pages, cx, |w| w.find_all_pages = !w.find_all_pages))),
            )
            .child(labelled_row(t("Find"), find_label_width, self.field(FieldId::FindText, 190., cx)))
            .child(labelled_row(t("Change"), find_label_width, self.field(FieldId::ReplaceText, 190., cx)))
            .child(
                div()
                    .pt_1()
                    .flex()
                    .gap_1()
                    .child(self.text_button(t("Find"), true, cx, |this, _, _| this.run_find(false)))
                    .child(self.text_button(t("Replace All"), true, cx, |this, _, _| this.run_find(true))),
            )
            .into_any_element()
    }

    // ---- Special Characters -----------------------------------------------------------

    pub(crate) fn special_characters_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let cells: Vec<_> = SPECIAL_CHARACTERS
            .chars()
            .map(|ch| {
                div()
                    .size(px(24.))
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .text_size(px(14.))
                    .hover(|s| s.bg(rgb(HOVER)))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, _, cx| {
                            // Keeps an open field or text edit active so the character lands in it.
                            this.keep_overlays = true;
                            this.insert_special(ch);
                            cx.notify();
                        }),
                    )
                    .child(ch.to_string())
            })
            .collect();
        div().id("characters").size_full().p_1().overflow_y_scroll().child(div().w(px(252.)).flex().flex_wrap().children(cells)).into_any_element()
    }

    // ---- Image Editing ----------------------------------------------------------------

    pub(crate) fn image_editing_body(&self, cx: &mut Context<Self>) -> AnyElement {
        let bitmap_tools = TOOL_SECTIONS.iter().find(|(name, _)| *name == "Bitmap").map_or(&[][..], |(_, slots)| *slots);
        let slots: Vec<_> = bitmap_tools.iter().chain(COLOR_TOOLS).map(|s| self.tool_slot(s, cx)).collect();
        let filters = [Filter::Brighten, Filter::Darken, Filter::Grayscale, Filter::Invert, Filter::Blur, Filter::Sharpen];
        let buttons: Vec<_> = filters
            .into_iter()
            .map(|filter| {
                let enabled = self.is_enabled(Cmd::Filter(filter));
                self.text_button(filter.label(), enabled, cx, move |this, window, cx| this.run(Cmd::Filter(filter), window, cx))
            })
            .collect();
        div()
            .id("image-editing")
            .size_full()
            .p_1()
            .flex()
            .flex_col()
            .gap_1()
            .overflow_y_scroll()
            .child(div().flex().flex_wrap().children(slots))
            .child(heading(t("Filters")))
            .child(div().flex().flex_wrap().gap_1().children(buttons))
            .into_any_element()
    }

    /// Renders one dock panel.
    pub(crate) fn panel_body(&self, panel: Panel, cx: &mut Context<Self>) -> AnyElement {
        match panel {
            Panel::Optimize => self.optimize_body(cx),
            Panel::History => self.history_body(cx),
            Panel::Align => self.align_body(cx),
            Panel::Pages => self.pages_body(cx),
            Panel::States => self.states_body(cx),
            Panel::Layers => self.layers_body(cx),
            Panel::Styles => self.styles_body(cx),
            Panel::Path => self.path_body(cx),
            Panel::ColorPalette => self.color_palette_body(cx),
            Panel::Swatches => self.swatches_body(cx),
            Panel::AutoShapeProperties => self.auto_properties_body(cx),
            Panel::AutoShapes => self.auto_shapes_body(cx),
            Panel::DocumentLibrary => self.document_library_body(cx),
            Panel::CommonLibrary => self.common_library_body(cx),
            Panel::Mixer => self.mixer_body(cx),
            Panel::Info => self.info_body(),
            Panel::Url => self.url_body(cx),
            Panel::Find => self.find_body(cx),
            Panel::SpecialCharacters => self.special_characters_body(cx),
            Panel::ImageEditing => self.image_editing_body(cx),
            Panel::Toolbar | Panel::Tools | Panel::Properties => div().into_any_element(),
        }
    }
}

/// The name of a channel as lists show it: RGB is the same in every language.
fn channel_label(name: &'static str) -> &'static str {
    if name == "RGB" { name } else { t(name) }
}
