<h1 align="center">PisiWorks</h1>

<p align="center">
  <strong>An open-source Adobe Fireworks look-alike for Pardus, with a Turkish interface.</strong><br>
  For the teachers who still teach Fireworks, and their students.
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-ff6a2b" alt="MIT license"></a>
  <img src="https://img.shields.io/badge/Rust-1.91%2B-1b2042?logo=rust" alt="Rust 1.91 or later">
  <img src="https://img.shields.io/badge/UI-GPUI-7a5cff" alt="Built with GPUI">
  <img src="https://img.shields.io/badge/platform-Linux-2ed3b7" alt="Linux">
</p>

<p align="center">
  <a href="#get-started">Get started</a> ·
  <a href="#features">Features</a> ·
  <a href="docs/guide.md">User guide</a> ·
  <a href="README.md">Türkçe</a>
</p>

<p align="center">
  <img src="docs/screenshots/pisiworks-calisma-alani.png" alt="The PisiWorks workspace in Turkish, with the tools on the left, Properties below, the panel dock on the right and a landing page design in the middle">
</p>

> PisiWorks is a fork of [Emberframe](https://github.com/gregstein/emberframe) by [Med Amine Belbachir](https://medamineweb.com/). Emberframe's code is its foundation; what was built on it is listed [below](#what-pisiworks-adds-to-emberframe).

## Why PisiWorks

Adobe stopped developing Fireworks in 2013, but the program that mixed vector shapes, bitmap editing, states and web export in one window is still taught in schools. PisiWorks aims to offer the same way of working on the [Pardus](https://www.pardus.org.tr/) operating system, in Turkish, as free software.

It is deliberately a look-alike: the same docked layout, tool groups, panels and light gray interface, and in Turkish the menus and command names are those of the Turkish edition of Fireworks. It is published under its own name, every icon is newly drawn, and it contains no Adobe code or artwork.

## Features

| Area | What you get |
|---|---|
| **Tools** | Pointer, Select Behind, Subselection, Scale, Skew, Distort, Crop, Marquee, Oval Marquee, Lasso, Polygon Lasso, Magic Wand, Brush, Pencil, Eraser, Blur, Sharpen, Dodge, Burn, Smudge, Rubber Stamp, Replace Color, Red Eye Removal, Eyedropper, Paint Bucket, Gradient, Line, Pen, Vector Path, Rectangle, Ellipse, Polygon, Text, Freeform, Knife, Hotspot, Slice, Hand, Zoom |
| **Panels** | Optimize, History, Align, Pages, States, Layers, Styles, Color Palette, Swatches, Path, Auto Shapes, Document Library, Common Library, Color Mixer, Info, URL, Find, Special Characters, Image Editing, Properties, Symbol Properties |
| **Vector** | Bezier curves and point editing, Auto Shapes, Union, Intersect, Punch, Crop, Join, Split, Simplify, Expand Stroke, Inset and Outset |
| **Transforming** | Scale, Skew, Distort, a movable center point, Numeric Transform, rotating and flipping |
| **Appearance** | Gradient fills of 12 kinds and pattern fills, 45 blend modes, masks, 29 live filters including Levels and Curves, Photoshop Live Effects with ten effects, styles |
| **Bitmap** | Paint, erase, clone, flood fill, pixel selections, Feather, Select Similar and Inverse, the marquee commands, Flatten Selection and Flatten Layers |
| **Text** | Installed fonts, bold, italic, underline, alignment, Convert to Paths |
| **Layout** | Rulers, grid, guides, smart guides, slice guides, groups with Superselect and Subselect |
| **Screens and animation** | Pages, layers, shared layers, states, onion skinning, symbols and a library |
| **Export** | PNG 8, PNG 24, PNG 32, JPEG with selective quality and smoothing, GIF and animated GIF, with Preview, 2-Up and 4-Up comparison, slices and HTML image maps |
| **Documents** | Native `.pisi` files (`.ember` files from Emberframe open too), opening and importing common image formats, Canvas Size, Image Size, Fit Canvas and Trim Canvas |
| **Printing** | Page Setup, printing to an installed printer or saving as PDF |
| **Language** | Turkish and English, changed from Edit > Language |

<p align="center">
  <img src="docs/screenshots/pisiworks-menuler.png" alt="The Modify menu and its Arrange submenu, with Turkish command names">
</p>

## Get started

Building, packaging and running were checked on Linux Mint 22.3. The target is Pardus 25, but **it has not been tried on Pardus 25 yet**.

### Installing a package

There is no ready-made `.deb` in the repository; one command builds it from source (the build requirements below must be installed):

```bash
packaging/build-deb.sh
```

The package is written to `target/deb/` and installed with:

```bash
sudo apt install ./target/deb/pisiworks_0.1.0_amd64.deb
```

After that PisiWorks is in the Graphics section of the application menu, and `.pisi` files open in it on a double-click. The maintainer written into the package is taken from the `DEB_MAINTAINER` environment variable, or failing that from git's `user.name` and `user.email`.

### Building from source

You need Rust 1.91 or later (from [rustup](https://rustup.rs/)) and these packages:

```bash
sudo apt install build-essential pkg-config libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev
```

The program draws with Vulkan. If the graphics driver has no Vulkan support, the software driver will do:

```bash
sudo apt install mesa-vulkan-drivers
```

To build and run:

```bash
git clone https://github.com/MuratSAPMAZ/PisiWorks.git
```

```bash
cd PisiWorks
```

```bash
git checkout pisiworks
```

```bash
cargo run --release
```

The first build takes a while. Then open [`examples/solstice-landing.pisi`](examples/solstice-landing.pisi) with Ctrl+O to explore the design in the screenshots. The [user guide](docs/guide.md) explains the tools and panels.

To run the tests:

```bash
cargo test
```

Printing needs the system's print service (CUPS).

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
| **Other** | Printing and Page Setup, dragging from the library panels onto the canvas, the `.pisi` file extension, and more than thirty menu commands that were greyed out |

The figures are from November 2026 and will drift as work goes on.

## Not in this version

- The commands that appear greyed out in the menus: New from Template, Batch Process, New Button, New Symbol (an editor for an empty symbol), Preferences, Keyboard Shortcuts, Animation and the Commands menu.
- Layers inside Fireworks PNG files. Only the flat image is read.
- Photoshop's own pattern and gradient libraries in Photoshop Live Effects.
- A ready-made package to download. For now the `.deb` is built from source.

The program has not been tested side by side with Adobe Fireworks itself; the pixel results of filters and blend modes may differ from those of Fireworks. Reports of differences and bugs are welcome.

## License and credits

PisiWorks is released under the same [MIT license](LICENSE) as Emberframe, and the license keeps the copyright notice of Emberframe's author.

Emberframe was created by **Med Amine Belbachir**:

- Website: [medamineweb.com](https://medamineweb.com/)
- Project page: [medamineweb.com/en/emberframe](https://medamineweb.com/en/emberframe/)
- GitHub: [@gregstein](https://github.com/gregstein)

Adobe and Fireworks are trademarks of Adobe Inc. PisiWorks is not affiliated with Adobe.
