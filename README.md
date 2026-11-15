<p align="center">
  <img src="docs/ef-logo.png" width="250" alt="Emberframe logo">
</p>

<h1 align="center">Emberframe</h1>

<p align="center">
  <strong>An open-source Adobe Fireworks look-alike, rebuilt from scratch in Rust.</strong><br>
  Vector and bitmap screen design in the docked workspace Fireworks users remember.
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-ff6a2b" alt="MIT license"></a>
  <img src="https://img.shields.io/badge/Rust-1.91%2B-1b2042?logo=rust" alt="Rust 1.91 or later">
  <img src="https://img.shields.io/badge/UI-GPUI-7a5cff" alt="Built with GPUI">
  <img src="https://img.shields.io/badge/platform-Windows-2ed3b7" alt="Windows">
  <a href="https://github.com/gregstein/emberframe/stargazers"><img src="https://img.shields.io/github/stars/gregstein/emberframe?style=flat&color=ffb547" alt="GitHub stars"></a>
</p>

<p align="center">
  <a href="#get-started">Get started</a> ·
  <a href="#features">Features</a> ·
  <a href="docs/guide.md">User guide</a> ·
  <a href="https://medamineweb.com/en/emberframe/">Project page</a>
</p>

<p align="center">
  <img src="docs/screenshots/workspace.png" alt="Emberframe workspace editing a landing page design, with the tools on the left, Properties below and the panel dock on the right">
</p>

> **Created by [Med Amine Belbachir](https://medamineweb.com/)**, who started this project to bring the Fireworks workflow back as free, native software.

> **This repository is PisiWorks**, a fork of [Emberframe](https://github.com/gregstein/emberframe) for the [Pardus](https://www.pardus.org.tr/) operating system, with a Turkish interface, for schools that still teach Fireworks. Emberframe's code is the foundation; see [What PisiWorks adds to Emberframe](#what-pisiworks-adds-to-emberframe) for what was built on it. The rest of this page is still Emberframe's own description.

## Why Emberframe

Adobe stopped developing Fireworks in 2013, and many designers still miss how it mixed vector shapes, bitmap editing, states and web export in one window. Emberframe brings that way of working back as a native Rust app you can read, build and change.

It is deliberately a look-alike: the same docked layout, tool groups, panels and light gray interface, under its own name and with every icon newly drawn. It contains no Adobe code or artwork.

## Features

| Area | What you get |
|---|---|
| **27 tools** | Pointer, Subselection, Scale, Crop, Marquee, Lasso, Magic Wand, Brush, Pencil, Eraser, Blur, Rubber Stamp, Eyedropper, Paint Bucket, Line, Pen, Rectangle, Ellipse, Text, Freeform, Knife, Hotspot, Slice, Hide Slices, Show Slices, Hand, Zoom |
| **23 panels** | Optimize, History, Align, Pages, States, Layers, Styles, Color Palette, Swatches, Path, Auto Shapes, Auto Shape Properties, Document Library, Common Library, Color Mixer, Info, URL, Find, Special Characters, Image Editing, Properties, Symbol Properties, Tools |
| **Vector** | Auto Shapes, Union, Intersect, Punch, Crop, Join, Split, Simplify, Expand Stroke, Inset and Outset |
| **Bitmap** | Paint, erase, clone, flood fill, pixel selections, Brighten, Darken, Grayscale, Invert, Blur and Sharpen |
| **Screens and animation** | Pages, layers, shared layers, states, onion skinning, symbols and a library |
| **Export** | PNG 8, PNG 24, PNG 32, JPEG, GIF and animated GIF, with Preview, 2-Up and 4-Up comparison, slices and HTML image maps |
| **Documents** | Native `.pisi` files (`.ember` files from Emberframe open too), opening and importing common image formats, Canvas Size, Image Size and Fit Canvas |

## Screenshots

**Compare export settings side by side in 4-Up**

<img src="docs/screenshots/four-up.png" alt="4-Up view comparing the original design with three PNG 8 export settings and their file sizes">

<table>
  <tr>
    <td width="72%" valign="top"><strong>The familiar menus</strong><br><br><img src="docs/screenshots/menus.png" alt="Modify menu open on the Canvas submenu"></td>
    <td width="28%" valign="top"><strong>The panel dock</strong><br><br><img src="docs/screenshots/panels.png" alt="Dock with Optimize, Layers, Color Palette, Auto Shapes and Common Library panels"></td>
  </tr>
</table>

## Get started

Download `emberframe.exe` for Windows from the [latest release](https://github.com/gregstein/emberframe/releases/latest) and run it.

To build from source, you need Rust 1.91 or later, the Visual Studio C++ tools and the Windows SDK.

```bash
git clone https://github.com/gregstein/emberframe.git
cd emberframe
cargo run --release
```

Then open [`examples/solstice-landing.pisi`](examples/solstice-landing.pisi) with Ctrl+O to explore the design in the screenshots. The [user guide](docs/guide.md) explains the tools and panels.

```bash
cargo test
```

## What PisiWorks adds to Emberframe

PisiWorks was forked from Emberframe 0.1 and keeps it as its foundation: the window layout, the menus, tool palette and panels, the icons, the object model with layers, pages, states and symbols, the basic drawing and bitmap tools, path operations, Auto Shapes and export all come from Emberframe. About 94% of Emberframe's lines are still in place.

| | Emberframe 0.1 | PisiWorks |
|---|---|---|
| Source code | 12,286 lines in 16 files | 25,179 lines in 25 files |
| Tests | 141 | 271 |

What was added on top:

| Area | Added in PisiWorks |
|---|---|
| **Linux and Pardus** | Builds and runs on Linux, a fix for the crash when typing Turkish letters through an input method, a font chosen from those installed, and a plain message when the graphics driver is missing |
| **Turkish** | The whole interface in Turkish, with the terms of the Turkish edition of Fireworks, a language menu, and tests that fail when a text has no translation |
| **Vectors** | Bezier curves, full point and handle editing with the Pen and Subselection tools, and curve fitting after path operations |
| **Transforming** | Scale, Skew and Distort with a movable center point, and Numeric Transform |
| **Appearance** | 45 blend modes, masks with all their commands, 29 live filters including Levels and Curves, and Photoshop Live Effects with ten effects and a gradient editor |
| **Bitmaps** | Moving selected pixels, Feather, Select Similar and Inverse, the marquee commands, Flatten Selection, Merge Down and Flatten Layers |
| **Text** | Fonts, bold, italic, underline and alignment |
| **Layout** | Guides, smart guides, slice guides, and Subselect and Superselect for groups |
| **Export** | Selective JPEG quality and smoothing, with a JPEG encoder of its own |
| **Other** | Printing and Page Setup, dragging from the library panels onto the canvas, the `.pisi` file extension (`.ember` files still open), and more than thirty menu commands that were greyed out |

The figures are from November 2026 and will drift as work goes on. PisiWorks is released under the same MIT license as Emberframe, and the license keeps its author's copyright notice.

## Not in this version

Some things are still missing:

- Photoshop's own pattern and gradient libraries in Photoshop Live Effects.
- Layers inside Fireworks PNG files. Only the flat image is read.
- Preferences, Keyboard Shortcuts, Batch Process, the button editor, animation symbols, text on a path and spell checking, which appear greyed out.
- Gradient and pattern fills for objects.
- Packages for Pardus, and macOS builds.

Issues and pull requests are welcome, especially for the items above.

## Creator

**Med Amine Belbachir** created Emberframe.

- Website: [medamineweb.com](https://medamineweb.com/)
- Project page: [medamineweb.com/en/emberframe](https://medamineweb.com/en/emberframe/)
- GitHub: [@gregstein](https://github.com/gregstein)

If Emberframe is useful or brings back good memories, a star helps other Fireworks fans find it.

## Legal

Emberframe is an independent project. It is not affiliated with, endorsed by or sponsored by Adobe Inc. Adobe and Fireworks are either registered trademarks or trademarks of Adobe in the United States and/or other countries, and are used here only to describe what this project resembles.

Released under the [MIT License](LICENSE).
