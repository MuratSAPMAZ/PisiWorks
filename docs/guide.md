# Emberframe user guide

Hold Shift while drawing to constrain shapes and lines. Hold Space to pan. Ctrl+scroll zooms. Press Enter to finish a Pen path, or click its first point to close it.

## Curves

With the Pen (P), click to place a corner. Press and drag instead to place a smooth point: the two handles you pull out bend the path on both sides of it.

With Subselection (A), select a path to see its points. Click a point to pick it, which shows its handles; Shift+click picks more, and dragging a box over empty canvas picks every point inside it. Drag a picked point to move it, with any others you picked, or drag a handle to reshape the curve. The opposite handle turns with it so the point stays smooth; hold Alt to move one handle on its own and make a cusp. Alt+drag a corner pulls handles out of it, and Alt+click a point turns it back into a corner. Delete removes the picked points.

The Pen also edits a selected path. Click a segment to add a point there. Click a curve point to make it a corner, and click a corner to remove it. Drag from a point to pull handles out of it. Click the loose end of an open path to carry on drawing from it, and finish on the loose end of another open path to join the two.

Ellipses turn into paths of a few curve points when you edit their points. A rectangle asks first, because as a path it no longer has a roundness setting. Freeform, Knife and the path operations give curved paths back: a circle joined to another is still made of curves.

## Editing images

Use File > Open (Ctrl+O) to edit an image, or File > Import (Ctrl+R) to add one to the current document.

Brush, Pencil and Eraser change the pixels of the bitmap under the pointer. Off a bitmap, Brush and Pencil draw vector strokes. Brush paints in the stroke color, and Paint Bucket fills in the fill color. Set Brush size in the Properties panel while the Brush or Eraser is active.

Drag with Crop, then press Enter or double-click to apply. Esc cancels.

Save never overwrites the opened image. It creates a `.pisi` document that keeps your objects, and File > Export writes a flat image.

Marquee (M), Lasso (L) and Magic Wand (W) select pixels on a bitmap. Hold Shift to add to the selection, or Alt to subtract. Painting, filling and filters stay inside the selection. Cut, Copy and Delete act on the selected pixels, and Paste adds them as a new bitmap. Drag inside the selection with the Pointer, or press the arrow keys, to move the selected pixels; hold Alt to leave a copy behind. Dragging inside it with Marquee or Lasso moves only the selection border, and with Ctrl held it moves the pixels. Ctrl+D or Esc drops the selection.

The Select menu changes a pixel selection. Select Similar adds every pixel of a like color anywhere on the bitmap, and Select Inverse (Ctrl+Shift+I) selects everything else. Expand Marquee and Contract Marquee grow or shrink it by a number of pixels, Border Marquee turns it into a band along its edge, and Smooth Marquee evens out a ragged edge. Feather fades the edge of the selection out over a radius, so that deleting, copying, moving, painting and filters fade with it. With no pixels selected, Select Inverse selects the objects that were not selected.

Rubber Stamp (S): Alt+click to set the source, then drag to clone from it.

Modify > Flatten Selection (Ctrl+Alt+Shift+Z) turns the selected objects into one bitmap. Merge Down (Ctrl+E) flattens them into the bitmap right under them. Flatten Layers makes one bitmap of everything on the layers that show, on one layer; hidden layers are removed, and slices and hotspots are kept. Modify > Canvas > Trim Canvas cuts the canvas back to the artwork.

Edit > Insert > Empty Bitmap adds a clear bitmap the size of the canvas to paint on. Edit > Clone (Ctrl+Shift+D) copies the selection in place, and Paste Attributes (Ctrl+Alt+Shift+V) gives the selection the fill, stroke, opacity, blend mode and filters of what was copied. Filters > Repeat Filter (Ctrl+Alt+Shift+F) applies the last filter again. File > Revert goes back to the file as it was last saved. View > Fit Selection (Ctrl+Alt+0) zooms in on the selection.

## Reshaping vectors

Scale (Q) resizes everything selected from its handles. Corners keep proportions unless you hold Shift.

Skew and Distort share the Scale button: hold the button down to list them, or press Q again to step through the three. With any of them, drag outside the selection box to rotate; hold Shift to turn in steps of 15 degrees. Rotation turns around the small circle in the middle of the box, which you can drag somewhere else first.

- **Skew**: drag a side handle to slant the selection, or a corner handle for perspective, where the corner and its neighbour move apart or together.
- **Distort**: drag a corner handle to move that corner alone, or a side handle to move the whole side.

While you drag, the artwork only previews the transform, and you can go on with another handle or another of the three tools. Press Enter or double-click inside the box to apply it, or Esc to put everything back. Dragging inside the box moves the selection.

Modify > Transform lists the same tools, with Rotate 180, Rotate 90 and the two flips. Numeric Transform (Ctrl+Shift+T) scales by a percentage, resizes to a size in pixels or rotates by an angle. It works around the center point if you have dragged it, and around the middle of the selection otherwise.

Text stays text when it is transformed, so you can still edit it and change its size. A symbol stays linked to the library, a rectangle keeps its roundness and an auto shape its settings. Modify > Transform > Remove Transformations stands any of them upright again. Flip Horizontal and Flip Vertical mirror text, symbols and groups too. Bitmaps are redrawn. Web areas are not transformed. Text > Convert to Paths turns text into the outlines of its letters.

Freeform (O) pulls a path or shape near the pointer, and Knife (Y, press again from Pencil) cuts selected vectors along a dragged line. With nothing selected, Knife cuts every vector it crosses. Both turn rectangles and ellipses into paths.

## Live filters

Filters in Properties adds effects to the selected objects: color adjustments, bevels and embosses, blurs, noise, shadows and glows, and sharpening. Pick one from the list to add it and set it. Each filter is listed with a box that switches it off, its name, which opens its settings again, and a minus that removes it. Filters are applied from the top of the list down, every time the object is drawn, so the object itself can still be edited. Drag a filter's name up or down to change its place. A style made from an object keeps its filters.

Levels shows how many pixels have each level, with the levels taken as black, mid gray and white on the way in and as black and white on the way out. Curves shows a curve from input level to output level: click it to add a point, drag a point to bend it, and drag a point out of the box to remove it. Both have a Channel list, with settings for red, green and blue apart from the ones all three share, and an Auto button that stretches each color from its darkest level to its lightest. In Levels the triangles under the histogram and under the ramp can be dragged instead of typing the numbers. The three eyedroppers set the black, gray or white point: pick one, then click a color on the object.

Filter dialogs have a Preview box. With it off, the object shows as it was before the dialog opened, until the box is ticked again or the dialog is closed with OK.

Photoshop Live Effects is one filter holding Drop Shadow, Inner Shadow, Outer Glow, Inner Glow, Bevel and Emboss, Satin, Color Overlay, Gradient Overlay, Pattern Overlay and Stroke. Tick the ones you want, and click a name to see its settings: blend mode, opacity, color, angle, distance, spread or choke, size and noise for shadows and glows; style, technique, depth, direction, soften, altitude, highlight and shadow for a bevel; style, angle and scale for a gradient; position and fill type for a stroke. Patterns come from a short built-in list.

A gradient is shown with its stops under it. Click under the gradient to add a stop, drag a stop to move it, and pull one away from the gradient to remove it. The stop that is picked shows its color, its place in percent and its opacity. Presets replaces the gradient with a ready-made one. The overlay, a gradient stroke and a gradient glow each have a gradient of their own; a gradient glow starts at the edge of the object, and Jitter scatters its colors.

The drop shadow and the outer glow are mixed into what lies under the object by their own blend modes. A filter that comes after Photoshop Live Effects in the list works on them as it does on the object.

## Masks

A mask hides part of an object. Modify > Mask has the commands:

- **Group as Mask**: select two or more objects; the one on top becomes the mask of the others. The lighter the mask is, the more shows through it, so use a white shape to cut out a clean piece.
- **Paste as Mask**: copy an object, select another and choose this; the copied object masks it by its outline. **Paste Inside** (Ctrl+Shift+V) does the opposite: what you copied appears inside the selected object, which stays in view around it.
- **Reveal All** and **Hide All** add a bitmap mask that shows everything or nothing, to paint on. With pixels selected on a bitmap, **Reveal Selection** and **Hide Selection** make the mask from the selection.
- **Disable Mask** switches the mask off without removing it, and **Delete Mask** removes it, after asking whether to apply it to the object first.

A masked object shows Mask in Properties: Path Outline or Grayscale Appearance for a vector mask, with Show fill and stroke, and Grayscale Appearance or Alpha Channel for a bitmap mask. Edit Mask there, or a click on the mask's thumbnail in Layers, selects the mask itself so you can move, reshape or paint it with any tool. White paint reveals and black hides. Select something else to finish.

In the Layers panel a chain between an object and its mask shows that the two are linked and move together. Click it to let the mask go, so the object can be moved under it, and again to link them. Paste as Mask asks whether a vector is to become a bitmap mask or stay a vector, and whether to replace a mask that is already there or add to it.

## Tool groups

A button with a small triangle stands for several tools. Hold it down to list them, or press its key again to walk through them; the button shows the one used last.

- Pointer (V): Select Behind picks the next object down under the mouse with each click.
- Scale (Q): after Skew and Distort comes 9-Slice Scaling. Four blue lines cut the selection into nine parts; drag a line to move it, then drag a handle of the selection: the corners keep their size and only the middle stretches, so rounded corners stay round.
- Crop (C): Export Area exports only the part of the page you drag over. Press Enter or double-click, and Image Preview opens for that part.
- Marquee (M): Oval Marquee selects an oval. Lasso (L): Polygon Lasso selects with straight sides; click each corner, and double-click or click the first corner to close it.
- Blur (R): Sharpen, Dodge (lightens), Burn (darkens) and Smudge (drags paint along the stroke) work under the brush like Blur. Dodge and Burn have an Exposure and a Range of tones in Properties; Smudge has a Pressure and can start loaded with the stroke color.
- Rubber Stamp (S): Replace Color changes the From color into the To color, within its Tolerance; with no From color it changes the color the stroke starts on. Red Eye Removal takes the red out of eyes in the area you drag over, by its Tolerance and Strength.
- Pen (P): Vector Path draws a path freehand. Redraw Path replaces part of a path: start the stroke on the path and end it on the path again.
- Freeform (O): Reshape Area pushes every point within its circle along with the mouse. The two Path Scrubbers make a stroke heavier or lighter where you rub over it, at the Rate set in Properties; a path treated this way becomes many short straight pieces.
- Hotspot (J): Circle Hotspot is drawn from its middle outward, and Polygon Hotspot corner by corner like the Polygon Lasso. Slice (K): Polygon Slice is drawn the same way. In the exported image map they are written as circles and polygons.
- Rectangle (U): Ellipse, and Polygon, which is drawn from its middle outward; Properties sets its number of sides, or makes it a star.
- Paint Bucket (G): Gradient fills the selected object with a gradient from where you start dragging to where you let go.

## Gradient and pattern fills

With a vector object selected, the list beside its fill in Properties chooses what it is filled with: Solid for one color, one of twelve gradients (Linear, Radial, Ellipse, Rectangle, Cone, Contour, Satin, Starburst, Folds, Bars, Ripples, Waves), or Pattern. A new gradient runs from the object's fill color to white.

With the Pointer, a gradient shows two handles on the canvas: a round one where it starts and a square one where it ends. Drag them to move, turn, lengthen or shorten the gradient; they may lie outside the object. Contour follows the outline of the object and has no handles.

An Ellipse or Rectangle gradient has a third, smaller handle that sets how wide it is. Web Dither fills with two colors pixel by pixel in turn. Text can be filled with a gradient or pattern like any shape.

Under the fill list, Edge sets how the edge of the fill is drawn: Hard, Anti-Alias, or Feather with the number of pixels it fades over. Texture lays one of the patterns into the fill, as strongly as its percentage.

Edit opens the gradient's colors: click under the gradient to add a color stop, drag a stop to move it, pull one away to remove it, and set the color and place of the stop that is picked. Opacity stops work the same way above the gradient: each sets how opaque the gradient is at its place. Presets replaces the gradient with a ready-made one. For a pattern, Edit chooses the pattern and its scale. Styles, Paste Attributes and files keep the fill.

## Image Preview

File > Image Preview (Ctrl+Shift+X) shows the export settings beside the image they give, with its format, file size and dimensions, and the image changes as you change the settings. Export saves it; OK keeps the settings without exporting, and Cancel puts them back.

## Printing

File > Page Setup sets the paper, which way it is turned, the margins, and whether the page is scaled to fit the paper. File > Print (Ctrl+P) sends the page to a printer set up on the computer, or saves it as a PDF file. The page prints as it exports, without slices and hotspots, at 72 pixels to the inch; a page larger than the paper is printed on several sheets unless Fit to page is on. Print range chooses the page on show, all pages of the document, or a range of them. Printing needs the system's print service (CUPS).

## Groups

Modify > Group (Ctrl+G) makes the selected objects move and scale as one. To change one member without ungrouping, click it with the Subselection tool, or choose Select > Subselect to select all the members of the selected group. A member can then be moved, recolored, reshaped or deleted on its own. The group is whole again as soon as none of its members is selected, and Select > Superselect selects the whole group from a member.

## Text

With a text object selected, Properties shows its font and size, buttons for bold, italic and underline, and buttons that align its lines left, centered or right. The same is in the Text menu: Font lists the fonts installed on the computer, Size has Smaller (Ctrl+Shift+,) and Larger (Ctrl+Shift+.) and a list of sizes, Style has Plain, Bold (Ctrl+B), Italic (Ctrl+I) and Underline (Ctrl+U), and Align has Left, Centered Horizontally, Right, Justified (every line but the last is spread from edge to edge by its spaces) and Stretched (every line is widened to fill the box). Set with nothing selected, they apply to the next text you type. For a font with no bold or italic face of its own, bold is made by thickening the letters and italic by leaning them.

Text > Attach to Path (Ctrl+Shift+Y) sets the selected text along the selected path or shape, which then belongs to the text and moves with it. Around a closed shape the text starts at the left and runs over the top. Properties shows how its letters stand (Text > Orientation: Rotate Around Path, Vertical, Skew Vertical, Skew Horizontal) and a Text offset that starts it further along. Reverse Direction runs it the other way, and Detach from Path gives the text and the path back.

Text > Check Spelling (Shift+F7) goes through the selected texts, or every text on the page, and stops at each word the dictionary does not have. Change puts the word in the field in its place, and Change All does so everywhere; Ignore passes over this one place and Ignore All over the word; Add to Personal makes the word right from now on. It checks in the language of the interface, with the spelling dictionaries installed on the system (the `enchant-2` or `hunspell` program and a dictionary such as `hunspell-tr`).

## Guides

Drag out of the ruler along the top or down the side to put a guide on the page. With the Pointer, drag a guide to move it, and drag it back onto a ruler to remove it. Objects being moved, drawn or resized snap to guides. Double-click a guide to type its place. The Pen and points dragged with the Subselection tool snap to guides too. View > Guides shows or hides them (Ctrl+;), locks them, turns snapping off, and clears them all; Edit Guides there also sets their color.

Smart guides appear while you move an object: a line shows where one of its edges or its middle lines up with another object or with the page, and the object snaps there. View > Smart Guides turns the lines or the snapping off. View > Slice Guides shows or hides the red lines along the edges of slices.

## Web areas

Hotspot (J) and Slice (K) draw areas over the artwork. Edit > Insert > Hotspot (Ctrl+Shift+U) and Rectangular Slice put one over the selected object; with several objects selected, you are asked whether to make one for all of them or one each. Move and resize them with the Pointer, and set a hotspot's link in Properties. Press 2 to hide or show them.

They never appear in the flattened image. Exporting `name.png` also writes `name_slice1.png`, `name_slice2.png` and so on for slices, and `name_map.html` with an image map for hotspots.

## Panels

The Window menu opens and closes every panel, and F4 hides or restores them all. Click a tab to bring it forward, and double-click the front tab to collapse its group.

- **Layers**: the list at the top sets the blend mode of the selected objects, which is how their colors mix with the artwork under them, and Opacity how much of it shows through. The same list is in Properties. A page that uses blend modes is shown as it will export, pixel for pixel.
- **States**: each state has its own objects. Play the animation, turn on onion skinning, and share a layer across states from the Layers panel. Double-click a page, state, layer or object to rename it.
- **Optimize**: pick GIF, Animated GIF, JPEG, PNG 8, PNG 24 or PNG 32. The panel estimates the file size, and Export Page uses these settings. For a JPEG, Smoothing (0 to 8) blurs the picture a little to make the file smaller. Selective quality keeps part of the page at a quality of its own: select the pixels, choose Modify > Selective JPEG > Save Selection as JPEG Mask, and tick Selective quality. The mask shows as a pink tint. Settings can also keep all text and all buttons at the selective quality, and change the color of the tint. A symbol is a button when its Type in Symbol Properties says so, as the buttons of the Common Library do.
- **Styles**: click a style to apply it, or to set the colors for the next shape. Styles hold fill, stroke and opacity.
- **Path**: Union, Intersect, Punch, Crop, Join, Split, Simplify, Expand Stroke, Inset and Outset.
- **Auto Shapes**: drag a tile onto the canvas, or click it to place the shape in the middle of the page, then adjust it in Auto Shape Properties.
- **Document Library**: New makes a symbol from the selection. Insert places it, Update redefines it from the selection, and Delete removes it.
- **Common Library**: drag an item onto the canvas, or click it, to add it to the document as a symbol. Symbols of the Document Library can be dragged onto the canvas too.
- **Find**: replace text or colors on this page or on all pages.
- **Color Palette**: click a color to fill with it, or Shift+click for the stroke. It also suggests complementary, analogous and triad colors for the current fill.
- **Symbol Properties**: rename the selected symbol, see how many copies exist, and break it apart.

The panel menu at the right of each tab row collapses or closes that group. Expanded Mode in the top bar switches between open panels and tabs only.

## Comparing exports

Preview shows the page as it will export. 2-Up puts the original next to it, and 4-Up compares the original with three export settings and their file sizes. Drag to pan in these views.

## Preferences and Keyboard Shortcuts

**Edit > Preferences** (Ctrl+U) sets the number of undo steps, how near the pointer picks an object, how near things snap, the size and color of the grid and the color of the guides. With text selected, Ctrl+U underlines instead. **View > Grid > Snap to Grid** (Ctrl+Alt+Shift+G) makes moved objects snap to the grid lines while the grid is shown.

**Edit > Keyboard Shortcuts** lists every menu command with its keys. Pick a command, press **Change**, then the new keys. Keys that belonged to another command are taken from it. **Remove** leaves the command without keys and **Reset** gives back the ones it came with. Changes are kept in the settings file.

## Symbols, buttons and animation

**Double-clicking** a copy of a symbol opens the symbol where it lies: its parts lie loose on the page, the rest of the page is locked, and whatever is drawn meanwhile joins the symbol. A double-click on an empty spot, or the **◀ Page** button above the document, ends the edit, and every copy of the symbol changes. **Modify > Symbol > Edit Symbol** does the same.

**Edit > Insert > New Symbol** (Ctrl+F8) asks for a name and a type (Graphic, Animation, Button) and opens the empty symbol for drawing. **New Button** (Ctrl+Shift+F8) opens a button symbol straight away. A button has four states: Up, Over, Down and Over While Down. While editing, pick the state from the list above the document; in an empty state **Copy … Graphic** brings the state before it. With a button selected on the page, **Link** in Properties takes the address a click leads to.

In **Preview** the button shows its Over state under the mouse and its Down state when pressed. On export each button gets a picture for every state and an HTML page (`name.html`) that swaps them.

**Modify > Animation > Animate Selection** (Alt+Shift+F8) turns the selection into an animation symbol: it asks for the number of states, the distance and direction of the move (0° right, 90° up), scaling, opacity and rotation, and adds states if needed. While selected its path shows (green start, red end); dragging the red end changes distance and direction. **Settings** opens the same dialog again and **Remove Animation** leaves only the copy in the first state.

## Templates, batch processing and commands

**File > New from Template** lists ready page sizes, grid systems marked with guides, phone and tablet screens, and your own templates; **Browse** opens any document as a template. A document started from a template has no file, so saving asks for a name. **File > Save as Template** adds the document to your templates.

**File > Batch Process** scales, converts and renames many files at once: add the files, pick the scaling (to a size, to fit an area, to a percentage), optionally use this document's Optimize settings, give a prefix or suffix and pick the output folder. If a result would replace an existing file, the old one is first copied to an `Original Files` folder.

The **Commands** menu has ready-made commands: Creative (Convert to Grayscale, Convert to Sepia Tone) and Document (Distribute to Layers, Hide Other Layers, Lock Other Layers, Reverse All States, Reverse State Range). **Save Steps as Command** in the **History** panel keeps the menu commands that changed the document under a name; saved commands show at the foot of the menu and are renamed or deleted with **Manage Saved Commands**. Steps made with the mouse (drawing, dragging) are not recorded. **Run Script** plays such a file (one `Menu > Command` per line); it does not run Fireworks' JSF files.
