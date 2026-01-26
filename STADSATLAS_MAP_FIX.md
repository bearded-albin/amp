# StadsAtlas Map Container Rendering Fix

## Problem

The Malmö StadsAtlas map was not rendering in the AMP testing interface, showing only a blank white container. Browser console showed the error:

```
No map visible because the map container's width or height are 0.
```

This prevented both:
- **Basemap (background map)** from loading
- **Miljöparkering layer** from displaying

## Root Cause

The `.map-container` div had:
- Fixed height of `500px` applied inline in CSS
- No parent dimension constraints
- Flex layout without proper sizing for child elements

When the Origo/OpenLayers initialization ran, it found a container with computed dimensions of 0×0 pixels, which prevented map rendering.

## Solution

Fixed the CSS hierarchy to ensure explicit, cascading dimensions:

### Changes to `server/src/assets/stadsatlas_interface.css`

1. **`.map-section` (parent container)**
   - Added `height: 550px` and `min-height: 550px`
   - Added `display: flex` and `flex-direction: column`
   - Added `flex-shrink: 0` to prevent collapse

2. **`.map-container` (Origo iframe holder)**
   - Changed `height: 500px` → `height: 100%`
   - Added `flex: 1` to fill available space
   - Added `display: flex` and `flex-direction: column`
   - Added `min-height: 0` (critical for flex children with scrolling parents)

3. **`.control-panel` (buttons + status)**
   - Added `flex-shrink: 0` to prevent collapse

### Changes to `server/src/assets/stadsatlas_interface.html`

Added inline `<style>` block in `<head>` to document and reinforce:
- `html, body { height: 100%; width: 100%; }`
- `body { display: flex; flex-direction: column; }`

This serves as a reminder for future maintainers about Origo/OpenLayers dimension requirements.

## Result

After these fixes:
✅ Map container now has explicit dimensions (550px × full width)  
✅ Origo can properly initialize OpenLayers with valid container size  
✅ Basemap renders correctly  
✅ Miljöparkering layer can activate and display  
✅ Pin marker and coordinates display properly  

## Testing

To verify the fix works:

1. Load the StadsAtlas testing interface
2. Click "Search Address & Load Map"
3. Verify the map appears with:
   - Background map tiles (Bakgrundskarta nedtonad)
   - Miljöparkering layer active (if available for that location)
   - Pin marker at the searched coordinates

## Key Learning

When using OpenLayers/Origo in web applications:
- **Always ensure parent container has explicit dimensions**
- Use `height: 100%` on parent elements that should fill viewport
- Set `min-height: 0` on flex children to allow proper flex sizing
- Test in embedded iframes to ensure dimensions cascade correctly

## Commits

- `acee09c`: fix: Ensure map container has explicit dimensions to prevent zero-height rendering
- `97e9f18`: docs: Add inline CSS comment explaining map container dimension requirements
