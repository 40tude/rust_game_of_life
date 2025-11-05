## Add Zoom

1. **Zoom Convention**: Mouse wheel UP = zoom IN (industry standard: browsers, Google Maps, games)
2. **Zoom Type**: Exponential with factor 1.15 (+15% per wheel notch)
   - Consistent behavior across all zoom levels
   - Standard in games (Factorio, RimWorld) and tools (Photoshop)
3. **Zoom Limits**:
   - `ZOOM_MIN`: Fixed at 0.1 (allows viewing ~10x more cells)
   - `ZOOM_MAX`: Dynamic = `min(window_width, window_height) / CELL_SIZE` (1 cell fills entire screen)
4. **Board Strategy**: FIXED board size (like copy.sh/life)
   - Board dimensions: 2000×1500 cells (constant, never resizes)
   - All cells always calculated (no data loss when zooming)
   - Zoom only affects viewport/display, not simulation grid
5. **Pattern Preservation**: Accept cell clipping at board edges (no dynamic zoom_min)







## TODO:
* Add more comments/documentation
* Add more tests

## NOT DONE
* Rendering in a thread such that continues even when we move the window on screen. I don't understand why it seems so complicated (it was so easy in LabWindows/CVI)

## DONE
* Add zooming
* Add icon
* Download more patterns (see `99_rle_downloader`)
* Need to evaluate efficiency of the app
    * How long does it take between to requests for redraw compare to 60 FPS?
* Logging in files
* Add logging
* Use GPU 
* Load pattern from the command line
* Load pattern pressing 'O'
* utils::read_rle() should support patterns file without x and y
* Handle errors in place_pattern_centered() and when returning from place_pattern_centered()
* Remove cells_in_corners()
* Translate initial comments
* Split in modules and lib

## ??
* https://copy.sh/life/examples/

<div align="center>
<iframe width="560" height="315" src="https://www.youtube.com/embed/h73dFLZgfh4?si=JF5VBTGNSKTELR5A" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
</div>
