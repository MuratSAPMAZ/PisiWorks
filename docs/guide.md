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

Rubber Stamp (S): Alt+click to set the source, then drag to clone from it.

## Reshaping vectors

Scale (Q) resizes everything selected from its handles. Corners keep proportions unless you hold Shift.

Skew and Distort share the Scale button: hold the button down to list them, or press Q again to step through the three. With any of them, drag outside the selection box to rotate; hold Shift to turn in steps of 15 degrees. Rotation turns around the small circle in the middle of the box, which you can drag somewhere else first.

- **Skew**: drag a side handle to slant the selection, or a corner handle for perspective, where the corner and its neighbour move apart or together.
- **Distort**: drag a corner handle to move that corner alone, or a side handle to move the whole side.

While you drag, the artwork only previews the transform, and you can go on with another handle or another of the three tools. Press Enter or double-click inside the box to apply it, or Esc to put everything back. Dragging inside the box moves the selection.

Modify > Transform lists the same tools, with Rotate 180, Rotate 90 and the two flips. Numeric Transform (Ctrl+Shift+T) scales by a percentage, resizes to a size in pixels or rotates by an angle. It works around the center point if you have dragged it, and around the middle of the selection otherwise.

Text stays text when it is transformed, so you can still edit it and change its size. A symbol stays linked to the library, a rectangle keeps its roundness and an auto shape its settings. Modify > Transform > Remove Transformations stands any of them upright again. Flip Horizontal and Flip Vertical mirror text, symbols and groups too. Bitmaps are redrawn. Web areas are not transformed. Text > Convert to Paths turns text into the outlines of its letters.

Freeform (O) pulls a path or shape near the pointer, and Knife (Y, press again from Pencil) cuts selected vectors along a dragged line. With nothing selected, Knife cuts every vector it crosses. Both turn rectangles and ellipses into paths.

## Masks

A mask hides part of an object. Modify > Mask has the commands:

- **Group as Mask**: select two or more objects; the one on top becomes the mask of the others. The lighter the mask is, the more shows through it, so use a white shape to cut out a clean piece.
- **Paste as Mask**: copy an object, select another and choose this; the copied object masks it by its outline. **Paste Inside** (Ctrl+Shift+V) does the opposite: what you copied appears inside the selected object, which stays in view around it.
- **Reveal All** and **Hide All** add a bitmap mask that shows everything or nothing, to paint on. With pixels selected on a bitmap, **Reveal Selection** and **Hide Selection** make the mask from the selection.
- **Disable Mask** switches the mask off without removing it, and **Delete Mask** removes it, after asking whether to apply it to the object first.

A masked object shows Mask in Properties: Path Outline or Grayscale Appearance for a vector mask, with Show fill and stroke, and Grayscale Appearance or Alpha Channel for a bitmap mask. Edit Mask there, or a click on the mask's thumbnail in Layers, selects the mask itself so you can move, reshape or paint it with any tool. White paint reveals and black hides. Select something else to finish.

In the Layers panel a chain between an object and its mask shows that the two are linked and move together. Click it to let the mask go, so the object can be moved under it, and again to link them. Paste as Mask asks whether a vector is to become a bitmap mask or stay a vector, and whether to replace a mask that is already there or add to it.

## Web areas

Hotspot (J) and Slice (K) draw areas over the artwork. Move and resize them with the Pointer, and set a hotspot's link in Properties. Press 2 to hide or show them.

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
