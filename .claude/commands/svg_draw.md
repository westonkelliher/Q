---
description: Draw an object as SVG in Super Auto Pets style
model: opus
---

# Draw: $ARGUMENTS

Generate an SVG of "$ARGUMENTS" in Super Auto Pets style.

## Style Requirements

1. **ViewBox**: Always use `viewBox="0 0 100 100"`
2. **Colors**: Pick 3 colors for this object:
   - **Base**: Main fill color
   - **Highlight**: 10-20% lighter, used for top/light-facing areas
   - **Shadow**: 10-20% darker, used for bottom/shadow areas
3. **Outlines**: Every visible shape gets `stroke="#000" stroke-width="5"`
4. **Shapes**: Use only:
   - `<circle>` and `<ellipse>` for rounded forms
   - `<rect rx="...">` with rounded corners
   - `<polygon>` for simple angular shapes
   - `<path>` for simple curves (no complex bezier chains)
5. **No gradients, no filters, no animations, no effects**
6. **Layer order**: Draw back-to-front (background shapes first)

## Composition Guidelines

- Object should fill 70-90% of the viewBox
- Center the object in the canvas
- Use cute/rounded proportions (Super Auto Pets style)
- If it has a face: white circle eyes with black dot pupils, or simple dot eyes
- Simple expressions only (dots, small curves)

## Example Structure

```svg
<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
  <!-- Back layers first -->
  <ellipse cx="50" cy="60" rx="35" ry="30" fill="#8B4513" stroke="#000" stroke-width="5"/>
  <!-- Front layers last -->
  <circle cx="50" cy="45" rx="25" fill="#A0522D" stroke="#000" stroke-width="5"/>
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
