---
description: Draw an object as SVG in a cute cartoon style
model: opus
---

# Draw: $ARGUMENTS

Generate an SVG of "$ARGUMENTS" in a cute cartoon style with bold outlines.

## Style Requirements

1. **ViewBox**: Always use `viewBox="0 0 100 100"`
2. **Colors**: Pick 3 colors for this object:
   - **Base**: Main fill color
   - **Highlight**: 10-20% lighter, used for top/light-facing areas
   - **Shadow**: 10-20% darker, used for bottom/shadow areas
3. **Shapes**: Use only:
   - `<circle>` and `<ellipse>` for rounded forms
   - `<rect rx="...">` with rounded corners
   - `<polygon>` for simple angular shapes
   - `<path>` for simple curves (no complex bezier chains)
4. **No gradients, no filters, no animations, no effects**

## Layering Process

Draw in this order (back to front):

1. **Outline layer**: Draw each base shape in BLACK, slightly larger (add ~3-5 units to radii/dimensions). This creates the outline by being visible around the edges of the colored shapes.
2. **Base shapes**: Draw the same shapes at normal size with their base colors. These sit on top of the black shapes, leaving a black border visible around the edges.
3. **Detail shapes**: Draw highlights, shadows, and features WITHIN the bounds of the base shapes.
4. **Line details**: Draw any internal line details on top. Lines can connect to the outline edges only if they are black.


## Example Structure

```svg
<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <!-- 1. OUTLINE LAYER: Same shapes as base, but larger and black -->
  <ellipse cx="50" cy="60" rx="38" ry="33" fill="#000"/>
  <circle cx="50" cy="45" r="28" fill="#000"/>
  
  <!-- 2. BASE SHAPES: Normal size with color (covers most of the black) -->
  <ellipse cx="50" cy="60" rx="35" ry="30" fill="#8B4513"/>
  <circle cx="50" cy="45" r="25" fill="#A0522D"/>
  
  <!-- 3. DETAIL SHAPES: Highlights, shadows, features within base bounds -->
  <ellipse cx="45" cy="40" rx="8" ry="5" fill="#B8763D"/>
  
  <!-- 4. LINE DETAILS: Internal lines, can connect to black edges -->
</svg>
```

## Output

1. Save the SVG to a file at `assets/$ARGUMENTS.svg` (create the directory if needed)
2. Convert the SVG to PNG using one of these commands (try in order):
   - `sips -s format png -z 512 512 assets/$ARGUMENTS.svg --out assets/$ARGUMENTS.png` (macOS sips - BEST, preserves transparency)
   - `convert -background none -size 512x512 assets/$ARGUMENTS.svg assets/$ARGUMENTS.png` (ImageMagick, if installed)
   - `rsvg-convert -w 512 -h 512 assets/$ARGUMENTS.svg -o assets/$ARGUMENTS.png` (librsvg, if installed)
   - **DO NOT USE qlmanage** - it adds white backgrounds instead of transparency
3. Confirm both files were created with their paths
