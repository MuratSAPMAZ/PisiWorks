//! Tool palette and menu definitions. An entry with no command is shown disabled.

use crate::i18n::{t, Language};
use crate::bitmap::Filter;
use crate::paths::PathOp;
use crate::model::{Align, Arrange};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Pointer,
    Subselect,
    Crop,
    Scale,
    Skew,
    Distort,
    Eraser,
    Marquee,
    Lasso,
    Wand,
    Blur,
    Stamp,
    Freeform,
    Knife,
    Hotspot,
    Slice,
    Rect,
    Ellipse,
    Line,
    Pen,
    Pencil,
    Brush,
    Text,
    Eyedropper,
    Bucket,
    Hand,
    Zoom,
    SelectBehind,
    OvalMarquee,
    PolygonLasso,
    Sharpen,
    Dodge,
    Burn,
    Smudge,
    ReplaceColor,
    RedEye,
    VectorPath,
    Polygon,
    Gradient,
    ExportArea,
    CircleHotspot,
    PolygonHotspot,
    PolygonSlice,
    RedrawPath,
    ReshapeArea,
    NineSlice,
    /// The Path Scrubber that makes a stroke heavier, and the one that makes it lighter.
    ScrubAdd,
    ScrubSubtract,
}

pub struct ToolSlot {
    pub label: &'static str,
    pub icon: &'static str,
    pub action: SlotAction,
}

#[derive(Clone, Copy)]
pub enum SlotAction {
    Select(Tool),
    Run(Cmd),
}

const fn on(label: &'static str, icon: &'static str, tool: Tool) -> ToolSlot {
    ToolSlot { label, icon, action: SlotAction::Select(tool) }
}

const fn run(label: &'static str, icon: &'static str, cmd: Cmd) -> ToolSlot {
    ToolSlot { label, icon, action: SlotAction::Run(cmd) }
}

pub const TOOL_SECTIONS: &[(&str, &[ToolSlot])] = &[
    (
        "Select",
        &[
            on("Pointer (V)", "pointer", Tool::Pointer),
            on("Subselection (A)", "subselect", Tool::Subselect),
            // One button for the three transform tools, see [`TRANSFORM_TOOLS`].
            on("Scale (Q)", "scale", Tool::Scale),
            on("Crop (C)", "crop", Tool::Crop),
        ],
    ),
    (
        "Bitmap",
        &[
            on("Marquee (M)", "marquee", Tool::Marquee),
            on("Lasso (L)", "lasso", Tool::Lasso),
            on("Magic Wand (W)", "wand", Tool::Wand),
            on("Brush (B)", "brush", Tool::Brush),
            on("Pencil (Y)", "pencil", Tool::Pencil),
            on("Eraser (E)", "eraser", Tool::Eraser),
            on("Blur (R)", "blur", Tool::Blur),
            on("Rubber Stamp (S): Alt+click sets the source", "stamp", Tool::Stamp),
        ],
    ),
    (
        "Vector",
        &[
            on("Line (N)", "line", Tool::Line),
            on("Pen (P)", "pen", Tool::Pen),
            // One button for Rectangle, Ellipse and Polygon, as in Fireworks.
            on("Rectangle (U)", "rect", Tool::Rect),
            on("Text (T)", "text", Tool::Text),
            on("Freeform (O)", "freeform", Tool::Freeform),
            on("Knife (Y)", "knife", Tool::Knife),
        ],
    ),
    (
        "Web",
        &[
            on("Hotspot (J)", "hotspot", Tool::Hotspot),
            on("Slice (K)", "slice", Tool::Slice),
            run("Hide Slices and Hotspots (2)", "hide", Cmd::ShowWeb(false)),
            run("Show Slices and Hotspots (2)", "show", Cmd::ShowWeb(true)),
        ],
    ),
];

/// The tools that share the Scale button. Holding the button down lists them, and
/// the button shows whichever was used last.
pub const TRANSFORM_TOOLS: &[ToolSlot] =
    &[on("Scale (Q)", "scale", Tool::Scale), on("Skew (Q)", "skew", Tool::Skew), on("Distort (Q)", "distort", Tool::Distort), on("9-Slice Scaling (Q)", "nine-slice", Tool::NineSlice)];

/// The buttons that stand for several tools. Holding one down lists its tools, its
/// key walks through them, and the button shows whichever was used last. The first
/// tool of each group is the one named in [`TOOL_SECTIONS`] or [`COLOR_TOOLS`].
pub const TOOL_GROUPS: &[&[ToolSlot]] = &[
    TRANSFORM_TOOLS,
    &[on("Pointer (V)", "pointer", Tool::Pointer), on("Select Behind (V)", "select-behind", Tool::SelectBehind)],
    &[on("Marquee (M)", "marquee", Tool::Marquee), on("Oval Marquee (M)", "oval-marquee", Tool::OvalMarquee)],
    &[on("Lasso (L)", "lasso", Tool::Lasso), on("Polygon Lasso (L)", "polygon-lasso", Tool::PolygonLasso)],
    &[
        on("Blur (R)", "blur", Tool::Blur),
        on("Sharpen (R)", "sharpen", Tool::Sharpen),
        on("Dodge (R)", "dodge", Tool::Dodge),
        on("Burn (R)", "burn", Tool::Burn),
        on("Smudge (R)", "smudge", Tool::Smudge),
    ],
    &[
        on("Rubber Stamp (S): Alt+click sets the source", "stamp", Tool::Stamp),
        on("Replace Color (S)", "replace-color", Tool::ReplaceColor),
        on("Red Eye Removal (S)", "red-eye", Tool::RedEye),
    ],
    &[on("Pen (P)", "pen", Tool::Pen), on("Vector Path (P)", "vector-path", Tool::VectorPath), on("Redraw Path (P)", "redraw-path", Tool::RedrawPath)],
    &[on("Crop (C)", "crop", Tool::Crop), on("Export Area (C)", "export-area", Tool::ExportArea)],
    &[
        on("Freeform (O)", "freeform", Tool::Freeform),
        on("Reshape Area (O)", "reshape-area", Tool::ReshapeArea),
        on("Path Scrubber - Additive (O)", "scrub-add", Tool::ScrubAdd),
        on("Path Scrubber - Subtractive (O)", "scrub-subtract", Tool::ScrubSubtract),
    ],
    &[on("Hotspot (J)", "hotspot", Tool::Hotspot), on("Circle Hotspot (J)", "circle-hotspot", Tool::CircleHotspot), on("Polygon Hotspot (J)", "polygon-hotspot", Tool::PolygonHotspot)],
    &[on("Slice (K)", "slice", Tool::Slice), on("Polygon Slice (K)", "polygon-slice", Tool::PolygonSlice)],
    &[on("Rectangle (U)", "rect", Tool::Rect), on("Ellipse (U)", "ellipse", Tool::Ellipse), on("Polygon (U)", "polygon", Tool::Polygon)],
    &[on("Paint Bucket (G)", "bucket", Tool::Bucket), on("Gradient (G)", "gradient", Tool::Gradient)],
];

/// Which group a tool belongs to, if it shares its button.
pub fn tool_group(tool: Tool) -> Option<usize> {
    TOOL_GROUPS.iter().position(|group| group.iter().any(|slot| matches!(slot.action, SlotAction::Select(other) if other == tool)))
}

pub const COLOR_TOOLS: &[ToolSlot] = &[on("Eyedropper (I)", "eyedropper", Tool::Eyedropper), on("Paint Bucket (G)", "bucket", Tool::Bucket)];

pub const SCREEN_MODES: &[ToolSlot] = &[
    run("Standard Screen", "screen-standard", Cmd::ScreenMode(ScreenMode::Standard)),
    run("Full Screen with Menus", "screen-menus", Cmd::ScreenMode(ScreenMode::FullWithMenus)),
    run("Full Screen (F4 brings the panels back)", "screen-full", Cmd::ScreenMode(ScreenMode::Full)),
];

pub const VIEW_TOOLS: &[ToolSlot] = &[on("Hand (H)", "hand", Tool::Hand), on("Zoom (Z)", "zoom", Tool::Zoom)];

pub const ZOOM_LEVELS: &[u32] = &[6, 12, 25, 50, 66, 100, 150, 200, 300, 400, 800, 1600, 3200, 6400];

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cmd {
    New,
    Open,
    Save,
    SaveAs,
    Import,
    Export,
    Exit,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Clear,
    Duplicate,
    ZoomIn,
    ZoomOut,
    ZoomTo(u32),
    FitAll,
    Rulers,
    Grid,
    SelectAll,
    Deselect,
    Arrange(Arrange),
    Align(Align),
    FlipH,
    FlipV,
    Filter(Filter),
    ShowWeb(bool),
    AddLayer,
    AddPage,
    About,
    Panel(Panel),
    HidePanels,
    ConvertToSymbol,
    NewSymbol,
    NewFromTemplate,
    ManageCommands,
    RunScript,
    /// Plays the saved command with this place in the list.
    RunSaved(u16),
    /// Saves the steps done so far as a command.
    SaveSteps,
    Ready(ReadyCmd),
    BatchProcess,
    SaveAsTemplate,
    NewButton,
    /// Opens the selected copy of a symbol for editing in place, or ends the edit.
    EditSymbol,
    Animate,
    AnimateSettings,
    RemoveAnimation,
    BreakApart,
    PathOp(PathOp),
    Group,
    Ungroup,
    Rotate { clockwise: bool },
    FreeTransform,
    /// Adds a mask that shows everything (true) or hides everything.
    MaskAll(bool),
    /// Adds a mask that shows only the selected pixels (true) or hides them.
    MaskSelection(bool),
    PasteAsMask,
    PasteInside,
    GroupAsMask,
    MaskToggle,
    MaskDelete,
    JpegMaskSave,
    JpegMaskRestore,
    JpegMaskRemove,
    JpegSettings,
    NumericTransform,
    SelectTool(Tool),
    Rotate180,
    RemoveTransform,
    TextToPaths,
    CanvasSize,
    ImageSize,
    FitCanvas,
    ScreenMode(ScreenMode),
    FirstState,
    PreviousState,
    NextState,
    LastState,
    PlayStates,
    Close,
    Language(Language),
    Revert,
    Clone,
    PasteAttributes,
    /// A hotspot or a slice over the selection.
    InsertWeb(crate::model::WebKind),
    InsertBitmap,
    AddState,
    FitSelection,
    SelectSimilar,
    SelectInverse,
    Marquee(MarqueeOp),
    TrimCanvas,
    FlattenSelection,
    MergeDown,
    FlattenLayers,
    RepeatFilter,
    Guides(GuideCmd),
    /// Sets text in the font at this place in the list of installed fonts.
    TextFont(u16),
    TextSize(u16),
    /// Makes text a size larger (true) or smaller.
    TextSizeStep(bool),
    TextStyle(TextStyleCmd),
    TextAlign(crate::model::TextAlign),
    /// The same alignments, as the Align menu names them for text that runs down.
    TextAlignDown(crate::model::TextAlign),
    TextOrientation(crate::model::TextOrientation),
    Superselect,
    Subselect,
    PageSetup,
    Print,
    ImagePreview,
    AttachToPath,
    DetachFromPath,
    ReversePathText,
    CheckSpelling,
    Preferences,
    KeyboardShortcuts,
    SnapToGrid,
    PathOrientation(crate::model::PathOrientation),
}

/// The commands of the Text > Style menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextStyleCmd {
    Plain,
    Bold,
    Italic,
    Underline,
}

/// The sizes the Text > Size menu offers, which Smaller and Larger also step through.
pub const TEXT_SIZES: [u16; 11] = [8, 9, 10, 12, 14, 18, 24, 36, 48, 72, 96];

/// The entries of the Text > Font menu: every font installed on this system.
pub fn font_entries() -> &'static [Entry] {
    static ENTRIES: std::sync::LazyLock<Vec<Entry>> = std::sync::LazyLock::new(|| {
        crate::fonts::families().iter().take(u16::MAX as usize).enumerate().map(|(index, name)| Entry::Item(name.as_str(), "", Some(Cmd::TextFont(index as u16)))).collect()
    });
    &ENTRIES
}

/// The commands that come ready in the Commands menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadyCmd {
    Grayscale,
    Sepia,
    DistributeToLayers,
    HideOtherLayers,
    LockOtherLayers,
    ReverseStates,
    ReverseStateRange,
}

/// The commands of the Guides, Smart Guides and Slice Guides menus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideCmd {
    Show,
    Lock,
    Snap,
    Clear,
    Edit,
    ShowSmart,
    SnapSmart,
    Slice,
}

/// The commands that change the marquee by a number of pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MarqueeOp {
    Expand,
    Contract,
    Border,
    Smooth,
    Feather,
}

impl MarqueeOp {
    pub fn label(self) -> &'static str {
        match self {
            MarqueeOp::Expand => crate::i18n::t("Expand Marquee"),
            MarqueeOp::Contract => crate::i18n::t("Contract Marquee"),
            MarqueeOp::Border => crate::i18n::t("Border Marquee"),
            MarqueeOp::Smooth => crate::i18n::t("Smooth Marquee"),
            MarqueeOp::Feather => crate::i18n::t("Feather Selection"),
        }
    }

    /// What the number in its dialog is called.
    pub fn prompt(self) -> &'static str {
        match self {
            MarqueeOp::Expand => crate::i18n::t("Expand by:"),
            MarqueeOp::Contract => crate::i18n::t("Contract by:"),
            MarqueeOp::Border => crate::i18n::t("Width:"),
            MarqueeOp::Smooth => crate::i18n::t("Sample radius:"),
            MarqueeOp::Feather => crate::i18n::t("Radius:"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenMode {
    Standard,
    FullWithMenus,
    Full,
}

/// Every window listed in the Window menu.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Panel {
    Toolbar,
    Tools,
    Properties,
    Optimize,
    History,
    Align,
    Pages,
    States,
    Layers,
    Styles,
    Path,
    ColorPalette,
    Swatches,
    AutoShapeProperties,
    AutoShapes,
    DocumentLibrary,
    CommonLibrary,
    Mixer,
    Info,
    Url,
    Find,
    SpecialCharacters,
    ImageEditing,
}

impl Panel {
    #[cfg(test)]
    pub const ALL: [Panel; 23] = [
        Panel::Toolbar,
        Panel::Tools,
        Panel::Properties,
        Panel::Optimize,
        Panel::History,
        Panel::Align,
        Panel::Pages,
        Panel::States,
        Panel::Layers,
        Panel::Styles,
        Panel::Path,
        Panel::ColorPalette,
        Panel::Swatches,
        Panel::AutoShapeProperties,
        Panel::AutoShapes,
        Panel::DocumentLibrary,
        Panel::CommonLibrary,
        Panel::Mixer,
        Panel::Info,
        Panel::Url,
        Panel::Find,
        Panel::SpecialCharacters,
        Panel::ImageEditing,
    ];

    pub fn title(self) -> &'static str {
        match self {
            Panel::Toolbar => t("Main Toolbar"),
            Panel::Tools => t("Tools"),
            Panel::Properties => t("Properties"),
            Panel::Optimize => t("Optimize"),
            Panel::History => t("History"),
            Panel::Align => t("Align"),
            Panel::Pages => t("Pages"),
            Panel::States => t("States"),
            Panel::Layers => t("Layers"),
            Panel::Styles => t("Styles"),
            Panel::Path => t("Path"),
            Panel::ColorPalette => t("Color Palette"),
            Panel::Swatches => t("Swatches"),
            Panel::AutoShapeProperties => t("Auto Shape Properties"),
            Panel::AutoShapes => t("Auto Shapes"),
            Panel::DocumentLibrary => t("Document Library"),
            Panel::CommonLibrary => t("Common Library"),
            Panel::Mixer => t("Color Mixer"),
            Panel::Info => t("Info"),
            Panel::Url => "URL",
            Panel::Find => t("Find"),
            Panel::SpecialCharacters => t("Special Characters"),
            Panel::ImageEditing => t("Image Editing"),
        }
    }
}

/// Tab groups of the right-hand dock, top to bottom.
pub const DOCK_GROUPS: [&[Panel]; 7] = [
    &[Panel::Optimize, Panel::History, Panel::Align],
    &[Panel::Pages, Panel::States, Panel::Layers],
    &[Panel::Styles, Panel::ColorPalette, Panel::Swatches],
    &[Panel::SpecialCharacters, Panel::ImageEditing, Panel::Path, Panel::AutoShapes],
    &[Panel::DocumentLibrary, Panel::CommonLibrary],
    &[Panel::Mixer, Panel::Info, Panel::Url],
    &[Panel::AutoShapeProperties, Panel::Find],
];

/// Every command of the menus that has one, with the menu it is in, its English name and the keys it comes with.
pub fn menu_commands() -> Vec<(&'static str, &'static str, &'static str, Cmd)> {
    fn collect(menu: &'static str, entries: &'static [Entry], out: &mut Vec<(&'static str, &'static str, &'static str, Cmd)>) {
        for entry in entries {
            match entry {
                Entry::Item(name, keys, Some(cmd)) => out.push((menu, name, keys, *cmd)),
                Entry::Sub(_, children) => collect(menu, children, out),
                _ => {}
            }
        }
    }
    let mut out = Vec::new();
    for (menu, entries) in MENUS {
        collect(menu, entries, &mut out);
    }
    out
}

/// A key press written as the menus write shortcuts, such as "Ctrl+Shift+D" or "F7".
pub fn key_name(key: &str, control: bool, alt: bool, shift: bool) -> String {
    let mut name = String::new();
    for (held, word) in [(control, "Ctrl+"), (alt, "Alt+"), (shift, "Shift+")] {
        if held {
            name.push_str(word);
        }
    }
    let mut letters = key.chars();
    match letters.next() {
        Some(first) => {
            name.extend(first.to_uppercase());
            name.push_str(letters.as_str());
        }
        None => return String::new(),
    }
    name
}

/// Window-menu shortcuts. These work even while a field is being edited.
pub fn panel_shortcut(key: &str, control: bool, shift: bool, alt: bool) -> Option<Cmd> {
    let panel = match (key, control, shift, alt) {
        ("f4", false, false, false) => return Some(Cmd::HidePanels),
        ("f7", false, true, false) => return Some(Cmd::CheckSpelling),
        ("f8", false, false, false) => return Some(Cmd::ConvertToSymbol),
        ("f8", true, false, false) => return Some(Cmd::NewSymbol),
        ("u", false, true, true) => return Some(Cmd::InsertWeb(crate::model::WebKind::Slice)),
        ("f8", true, true, false) => return Some(Cmd::NewButton),
        ("f8", false, true, true) => return Some(Cmd::Animate),
        ("f2", true, false, false) => Panel::Tools,
        ("f3", true, false, false) => Panel::Properties,
        ("f6", false, false, false) => Panel::Optimize,
        ("f2", false, false, false) => Panel::Layers,
        ("f5", false, false, false) => Panel::Pages,
        ("f2", false, true, false) => Panel::States,
        ("f10", false, true, false) => Panel::History,
        ("f11", true, false, false) => Panel::Styles,
        ("f11", false, false, false) => Panel::DocumentLibrary,
        ("f7", false, false, false) => Panel::CommonLibrary,
        ("f10", false, true, true) => Panel::Url,
        ("f9", false, true, false) => Panel::Mixer,
        ("f9", true, false, false) => Panel::Swatches,
        ("f12", false, true, true) => Panel::Info,
        _ => return None,
    };
    Some(Cmd::Panel(panel))
}

#[derive(Clone, Copy)]
pub enum Entry {
    Item(&'static str, &'static str, Option<Cmd>),
    Sep,
    Sub(&'static str, &'static [Entry]),
}

use Entry::{Item, Sep, Sub};

const fn cmd(label: &'static str, keys: &'static str, c: Cmd) -> Entry {
    Item(label, keys, Some(c))
}


pub const MENUS: &[(&str, &[Entry])] = &[
    (
        "File",
        &[
            cmd("New", "Ctrl+N", Cmd::New),
            cmd("New from Template...", "", Cmd::NewFromTemplate),
            cmd("Open...", "Ctrl+O", Cmd::Open),
            Sep,
            cmd("Save", "Ctrl+S", Cmd::Save),
            cmd("Save As...", "Ctrl+Shift+S", Cmd::SaveAs),
            cmd("Save as Template...", "", Cmd::SaveAsTemplate),
            cmd("Revert", "", Cmd::Revert),
            Sep,
            cmd("Import...", "Ctrl+R", Cmd::Import),
            cmd("Export...", "Ctrl+Shift+R", Cmd::Export),
            cmd("Image Preview...", "Ctrl+Shift+X", Cmd::ImagePreview),
            Sep,
            cmd("Batch Process...", "", Cmd::BatchProcess),
            Sep,
            cmd("Page Setup...", "", Cmd::PageSetup),
            cmd("Print...", "Ctrl+P", Cmd::Print),
            Sep,
            cmd("Exit", "Alt+F4", Cmd::Exit),
        ],
    ),
    (
        "Edit",
        &[
            cmd("Undo", "Ctrl+Z", Cmd::Undo),
            cmd("Redo", "Ctrl+Y", Cmd::Redo),
            Sep,
            Sub(
                "Insert",
                &[
                    cmd("New Button...", "Ctrl+Shift+F8", Cmd::NewButton),
                    cmd("New Symbol...", "Ctrl+F8", Cmd::NewSymbol),
                    Sep,
                    cmd("Hotspot", "Ctrl+Shift+U", Cmd::InsertWeb(crate::model::WebKind::Hotspot)),
                    cmd("Rectangular Slice", "Alt+Shift+U", Cmd::InsertWeb(crate::model::WebKind::Slice)),
                    Sep,
                    cmd("Empty Bitmap", "", Cmd::InsertBitmap),
                    Sep,
                    cmd("Layer", "", Cmd::AddLayer),
                    cmd("State", "", Cmd::AddState),
                    cmd("Page", "", Cmd::AddPage),
                ],
            ),
            cmd("Find and Replace...", "Ctrl+F", Cmd::Panel(Panel::Find)),
            Sep,
            cmd("Cut", "Ctrl+X", Cmd::Cut),
            cmd("Copy", "Ctrl+C", Cmd::Copy),
            cmd("Paste", "Ctrl+V", Cmd::Paste),
            cmd("Clear", "Delete", Cmd::Clear),
            Sep,
            cmd("Paste as Mask", "", Cmd::PasteAsMask),
            cmd("Paste Inside", "Ctrl+Shift+V", Cmd::PasteInside),
            cmd("Paste Attributes", "Ctrl+Alt+Shift+V", Cmd::PasteAttributes),
            Sep,
            cmd("Duplicate", "Ctrl+Alt+D", Cmd::Duplicate),
            cmd("Clone", "Ctrl+Shift+D", Cmd::Clone),
            Sep,
            cmd("Preferences...", "Ctrl+U", Cmd::Preferences),
            cmd("Keyboard Shortcuts...", "", Cmd::KeyboardShortcuts),
            Sep,
            // Each language is listed under its own name, so it can be found from any language.
            Sub(
                "Language",
                &[
                    cmd("English", "", Cmd::Language(Language::English)),
                    cmd("Türkçe", "", Cmd::Language(Language::Turkish)),
                ],
            ),
        ],
    ),
    (
        "View",
        &[
            cmd("Zoom In", "Ctrl+=", Cmd::ZoomIn),
            cmd("Zoom Out", "Ctrl+-", Cmd::ZoomOut),
            Sub(
                "Magnification",
                &[
                    cmd("6%", "", Cmd::ZoomTo(6)),
                    cmd("12%", "", Cmd::ZoomTo(12)),
                    cmd("25%", "", Cmd::ZoomTo(25)),
                    cmd("50%", "", Cmd::ZoomTo(50)),
                    cmd("66%", "", Cmd::ZoomTo(66)),
                    cmd("100%", "Ctrl+1", Cmd::ZoomTo(100)),
                    cmd("150%", "", Cmd::ZoomTo(150)),
                    cmd("200%", "", Cmd::ZoomTo(200)),
                    cmd("300%", "", Cmd::ZoomTo(300)),
                    cmd("400%", "", Cmd::ZoomTo(400)),
                    cmd("800%", "", Cmd::ZoomTo(800)),
                    cmd("1600%", "", Cmd::ZoomTo(1600)),
                    cmd("3200%", "", Cmd::ZoomTo(3200)),
                    cmd("6400%", "", Cmd::ZoomTo(6400)),
                ],
            ),
            Sep,
            cmd("Fit Selection", "Ctrl+Alt+0", Cmd::FitSelection),
            cmd("Fit All", "Ctrl+0", Cmd::FitAll),
            Sep,
            cmd("Rulers", "Ctrl+Alt+R", Cmd::Rulers),
            Sub(
                "Grid",
                &[cmd("Show Grid", "Ctrl+Alt+G", Cmd::Grid), cmd("Snap to Grid", "Ctrl+Alt+Shift+G", Cmd::SnapToGrid), Sep, cmd("Edit Grid...", "", Cmd::Preferences)],
            ),
            Sub(
                "Guides",
                &[
                    cmd("Show Guides", "Ctrl+;", Cmd::Guides(GuideCmd::Show)),
                    cmd("Lock Guides", "Ctrl+Alt+;", Cmd::Guides(GuideCmd::Lock)),
                    cmd("Snap to Guides", "Ctrl+Shift+;", Cmd::Guides(GuideCmd::Snap)),
                    Sep,
                    cmd("Edit Guides...", "", Cmd::Guides(GuideCmd::Edit)),
                    cmd("Clear Guides", "", Cmd::Guides(GuideCmd::Clear)),
                ],
            ),
            Sub(
                "Smart Guides",
                &[cmd("Show Smart Guides", "Ctrl+\\", Cmd::Guides(GuideCmd::ShowSmart)), cmd("Snap to Smart Guides", "", Cmd::Guides(GuideCmd::SnapSmart))],
            ),
            cmd("Slice Guides", "Ctrl+Alt+Shift+;", Cmd::Guides(GuideCmd::Slice)),
        ],
    ),
    (
        "Select",
        &[
            cmd("Select All", "Ctrl+A", Cmd::SelectAll),
            cmd("Deselect", "Ctrl+D", Cmd::Deselect),
            cmd("Superselect", "", Cmd::Superselect),
            cmd("Subselect", "", Cmd::Subselect),
            Sep,
            cmd("Select Similar", "", Cmd::SelectSimilar),
            cmd("Select Inverse", "Ctrl+Shift+I", Cmd::SelectInverse),
            cmd("Feather...", "", Cmd::Marquee(MarqueeOp::Feather)),
            Sep,
            cmd("Expand Marquee...", "", Cmd::Marquee(MarqueeOp::Expand)),
            cmd("Contract Marquee...", "", Cmd::Marquee(MarqueeOp::Contract)),
            cmd("Border Marquee...", "", Cmd::Marquee(MarqueeOp::Border)),
            cmd("Smooth Marquee...", "", Cmd::Marquee(MarqueeOp::Smooth)),
        ],
    ),
    (
        "Modify",
        &[
            Sub(
                "Canvas",
                &[
                    cmd("Image Size...", "", Cmd::ImageSize),
                    cmd("Canvas Size...", "", Cmd::CanvasSize),
                    cmd("Trim Canvas", "", Cmd::TrimCanvas),
                    cmd("Fit Canvas", "", Cmd::FitCanvas),
                ],
            ),
            Sub(
                "Animation",
                &[cmd("Animate Selection...", "Alt+Shift+F8", Cmd::Animate), cmd("Settings...", "", Cmd::AnimateSettings), cmd("Remove Animation", "", Cmd::RemoveAnimation)],
            ),
            Sub(
                "Symbol",
                &[cmd("Convert to Symbol", "F8", Cmd::ConvertToSymbol), cmd("Edit Symbol", "", Cmd::EditSymbol), cmd("Break Apart", "Ctrl+B", Cmd::BreakApart)],
            ),
            Sub(
                "Mask",
                &[
                    cmd("Reveal All", "", Cmd::MaskAll(true)),
                    cmd("Hide All", "", Cmd::MaskAll(false)),
                    cmd("Paste as Mask", "", Cmd::PasteAsMask),
                    cmd("Group as Mask", "", Cmd::GroupAsMask),
                    cmd("Reveal Selection", "", Cmd::MaskSelection(true)),
                    cmd("Hide Selection", "", Cmd::MaskSelection(false)),
                    cmd("Disable Mask", "", Cmd::MaskToggle),
                    cmd("Delete Mask", "", Cmd::MaskDelete),
                ],
            ),
            Sub(
                "Selective JPEG",
                &[
                    cmd("Save Selection as JPEG Mask", "", Cmd::JpegMaskSave),
                    cmd("Restore JPEG Mask as Selection", "", Cmd::JpegMaskRestore),
                    cmd("Settings...", "", Cmd::JpegSettings),
                    cmd("Remove JPEG Mask", "", Cmd::JpegMaskRemove),
                ],
            ),
            Sep,
            cmd("Flatten Selection", "Ctrl+Alt+Shift+Z", Cmd::FlattenSelection),
            cmd("Merge Down", "Ctrl+E", Cmd::MergeDown),
            cmd("Flatten Layers", "", Cmd::FlattenLayers),
            Sep,
            Sub(
                "Transform",
                &[
                    cmd("Free Transform", "Ctrl+T", Cmd::FreeTransform),
                    cmd("Scale", "", Cmd::SelectTool(Tool::Scale)),
                    cmd("Skew", "", Cmd::SelectTool(Tool::Skew)),
                    cmd("Distort", "", Cmd::SelectTool(Tool::Distort)),
                    cmd("Numeric Transform...", "Ctrl+Shift+T", Cmd::NumericTransform),
                    Sep,
                    cmd("Rotate 180", "", Cmd::Rotate180),
                    cmd("Rotate 90 CW", "", Cmd::Rotate { clockwise: true }),
                    cmd("Rotate 90 CCW", "", Cmd::Rotate { clockwise: false }),
                    Sep,
                    cmd("Flip Horizontal", "", Cmd::FlipH),
                    cmd("Flip Vertical", "", Cmd::FlipV),
                    Sep,
                    cmd("Remove Transformations", "", Cmd::RemoveTransform),
                ],
            ),
            Sub(
                "Arrange",
                &[
                    cmd("Bring to Front", "Ctrl+Shift+Up", Cmd::Arrange(Arrange::Front)),
                    cmd("Bring Forward", "Ctrl+Up", Cmd::Arrange(Arrange::Forward)),
                    cmd("Send Backward", "Ctrl+Down", Cmd::Arrange(Arrange::Backward)),
                    cmd("Send to Back", "Ctrl+Shift+Down", Cmd::Arrange(Arrange::Back)),
                ],
            ),
            Sub(
                "Align",
                &[
                    cmd("Left", "", Cmd::Align(Align::Left)),
                    cmd("Center Vertical", "", Cmd::Align(Align::CenterX)),
                    cmd("Right", "", Cmd::Align(Align::Right)),
                    Sep,
                    cmd("Top", "", Cmd::Align(Align::Top)),
                    cmd("Center Horizontal", "", Cmd::Align(Align::CenterY)),
                    cmd("Bottom", "", Cmd::Align(Align::Bottom)),
                ],
            ),
            Sep,
            Sub(
                "Combine Paths",
                &[
                    // In the order of the Fireworks menu.
                    cmd("Join", "", Cmd::PathOp(PathOp::Join)),
                    cmd("Split", "", Cmd::PathOp(PathOp::Split)),
                    Sep,
                    cmd("Union", "", Cmd::PathOp(PathOp::Union)),
                    cmd("Intersect", "", Cmd::PathOp(PathOp::Intersect)),
                    cmd("Punch", "", Cmd::PathOp(PathOp::Punch)),
                    cmd("Crop", "", Cmd::PathOp(PathOp::Crop)),
                ],
            ),
            Sub(
                "Alter Path",
                &[
                    cmd("Simplify", "", Cmd::PathOp(PathOp::Simplify)),
                    cmd("Expand Stroke", "", Cmd::PathOp(PathOp::ExpandStroke)),
                    cmd("Inset", "", Cmd::PathOp(PathOp::Inset)),
                    cmd("Outset", "", Cmd::PathOp(PathOp::Outset)),
                ],
            ),
            Sep,
            cmd("Group", "Ctrl+G", Cmd::Group),
            cmd("Ungroup", "Ctrl+Shift+G", Cmd::Ungroup),
        ],
    ),
    (
        "Text",
        &[
            // Filled in with the installed fonts when the menu is shown.
            Sub("Font", &[]),
            Sub(
                "Size",
                &[
                    cmd("Smaller", "Ctrl+Shift+,", Cmd::TextSizeStep(false)),
                    cmd("Larger", "Ctrl+Shift+.", Cmd::TextSizeStep(true)),
                    Sep,
                    cmd("8", "", Cmd::TextSize(8)),
                    cmd("9", "", Cmd::TextSize(9)),
                    cmd("10", "", Cmd::TextSize(10)),
                    cmd("12", "", Cmd::TextSize(12)),
                    cmd("14", "", Cmd::TextSize(14)),
                    cmd("18", "", Cmd::TextSize(18)),
                    cmd("24", "", Cmd::TextSize(24)),
                    cmd("36", "", Cmd::TextSize(36)),
                    cmd("48", "", Cmd::TextSize(48)),
                    cmd("72", "", Cmd::TextSize(72)),
                    cmd("96", "", Cmd::TextSize(96)),
                ],
            ),
            Sub(
                "Style",
                &[
                    cmd("Plain", "", Cmd::TextStyle(TextStyleCmd::Plain)),
                    cmd("Bold", "Ctrl+B", Cmd::TextStyle(TextStyleCmd::Bold)),
                    cmd("Italic", "Ctrl+I", Cmd::TextStyle(TextStyleCmd::Italic)),
                    cmd("Underline", "Ctrl+U", Cmd::TextStyle(TextStyleCmd::Underline)),
                ],
            ),
            Sub(
                "Align",
                &[
                    cmd("Left", "Ctrl+Alt+Shift+L", Cmd::TextAlign(crate::model::TextAlign::Left)),
                    cmd("Centered Horizontally", "Ctrl+Alt+Shift+C", Cmd::TextAlign(crate::model::TextAlign::Center)),
                    cmd("Right", "Ctrl+Alt+Shift+R", Cmd::TextAlign(crate::model::TextAlign::Right)),
                    cmd("Justified", "Ctrl+Alt+Shift+J", Cmd::TextAlign(crate::model::TextAlign::Justify)),
                    cmd("Stretched", "Ctrl+Alt+Shift+S", Cmd::TextAlign(crate::model::TextAlign::Stretch)),
                    Sep,
                    cmd("Top", "", Cmd::TextAlignDown(crate::model::TextAlign::Left)),
                    cmd("Centered Vertically", "", Cmd::TextAlignDown(crate::model::TextAlign::Center)),
                    cmd("Bottom", "", Cmd::TextAlignDown(crate::model::TextAlign::Right)),
                    cmd("Justified Vertically", "", Cmd::TextAlignDown(crate::model::TextAlign::Justify)),
                    cmd("Stretched Vertically", "", Cmd::TextAlignDown(crate::model::TextAlign::Stretch)),
                ],
            ),
            Sep,
            cmd("Attach to Path", "Ctrl+Shift+Y", Cmd::AttachToPath),
            cmd("Detach from Path", "", Cmd::DetachFromPath),
            Sub(
                "Orientation",
                &[
                    cmd("Rotate Around Path", "", Cmd::PathOrientation(crate::model::PathOrientation::Rotate)),
                    cmd("Vertical", "", Cmd::PathOrientation(crate::model::PathOrientation::Vertical)),
                    cmd("Skew Vertical", "", Cmd::PathOrientation(crate::model::PathOrientation::SkewVertical)),
                    cmd("Skew Horizontal", "", Cmd::PathOrientation(crate::model::PathOrientation::SkewHorizontal)),
                ],
            ),
            cmd("Reverse Direction", "", Cmd::ReversePathText),
            cmd("Convert to Paths", "", Cmd::TextToPaths),
            Sep,
            cmd("Check Spelling...", "Shift+F7", Cmd::CheckSpelling),
        ],
    ),
    (
        "Commands",
        &[
            cmd("Manage Saved Commands...", "", Cmd::ManageCommands),
            cmd("Run Script...", "", Cmd::RunScript),
            Sep,
            Sub("Creative", &[cmd("Convert to Grayscale", "", Cmd::Ready(ReadyCmd::Grayscale)), cmd("Convert to Sepia Tone", "", Cmd::Ready(ReadyCmd::Sepia))]),
            Sub(
                "Document",
                &[
                    cmd("Distribute to Layers", "", Cmd::Ready(ReadyCmd::DistributeToLayers)),
                    cmd("Hide Other Layers", "", Cmd::Ready(ReadyCmd::HideOtherLayers)),
                    cmd("Lock Other Layers", "", Cmd::Ready(ReadyCmd::LockOtherLayers)),
                    cmd("Reverse All States", "", Cmd::Ready(ReadyCmd::ReverseStates)),
                    cmd("Reverse State Range...", "", Cmd::Ready(ReadyCmd::ReverseStateRange)),
                ],
            ),
        ],
    ),
    (
        "Filters",
        &[
            cmd("Repeat Filter", "Ctrl+Alt+Shift+F", Cmd::RepeatFilter),
            Sep,
            Sub(
                "Adjust Color",
                &[
                    cmd("Brighten", "", Cmd::Filter(Filter::Brighten)),
                    cmd("Darken", "", Cmd::Filter(Filter::Darken)),
                    cmd("Grayscale", "", Cmd::Filter(Filter::Grayscale)),
                    cmd("Invert", "", Cmd::Filter(Filter::Invert)),
                ],
            ),
            Sub(
                "Blur and Sharpen",
                &[cmd("Gaussian Blur", "", Cmd::Filter(Filter::Blur)), cmd("Sharpen", "", Cmd::Filter(Filter::Sharpen))],
            ),
        ],
    ),
    (
        "Window",
        &[
            cmd("Hide Panels", "F4", Cmd::HidePanels),
            Sep,
            cmd("Main Toolbar", "", Cmd::Panel(Panel::Toolbar)),
            cmd("Tools", "Ctrl+F2", Cmd::Panel(Panel::Tools)),
            cmd("Properties", "Ctrl+F3", Cmd::Panel(Panel::Properties)),
            Sep,
            cmd("Optimize", "F6", Cmd::Panel(Panel::Optimize)),
            cmd("Layers", "F2", Cmd::Panel(Panel::Layers)),
            cmd("Common Library", "F7", Cmd::Panel(Panel::CommonLibrary)),
            cmd("Pages", "F5", Cmd::Panel(Panel::Pages)),
            cmd("States", "Shift+F2", Cmd::Panel(Panel::States)),
            cmd("History", "Shift+F10", Cmd::Panel(Panel::History)),
            cmd("Auto Shapes", "", Cmd::Panel(Panel::AutoShapes)),
            cmd("Styles", "Ctrl+F11", Cmd::Panel(Panel::Styles)),
            cmd("Document Library", "F11", Cmd::Panel(Panel::DocumentLibrary)),
            cmd("URL", "Alt+Shift+F10", Cmd::Panel(Panel::Url)),
            Sep,
            cmd("Color Mixer", "Shift+F9", Cmd::Panel(Panel::Mixer)),
            cmd("Swatches", "Ctrl+F9", Cmd::Panel(Panel::Swatches)),
            cmd("Color Palette", "", Cmd::Panel(Panel::ColorPalette)),
            cmd("Info", "Alt+Shift+F12", Cmd::Panel(Panel::Info)),
            cmd("Find", "Ctrl+F", Cmd::Panel(Panel::Find)),
            cmd("Align", "", Cmd::Panel(Panel::Align)),
            cmd("Auto Shape Properties", "", Cmd::Panel(Panel::AutoShapeProperties)),
            cmd("Path", "", Cmd::Panel(Panel::Path)),
            cmd("Special Characters", "", Cmd::Panel(Panel::SpecialCharacters)),
            cmd("Image Editing", "", Cmd::Panel(Panel::ImageEditing)),
        ],
    ),
    ("Help", &[cmd("About PisiWorks", "", Cmd::About)]),
];
