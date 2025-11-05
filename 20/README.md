## Add panning

* `cargo run --release -p step_20`

- **Board**: Fixed size simulation grid (never resizes)
- **Camera**: Position `(camera_x, camera_y)` = center of visible area
- **Viewport**: Window showing subset of board based on camera + zoom



## TODO:
* Add more comments/documentation
* Add more tests
* Read : https://johnhw.github.io/hashlife/index.md.html


## NOT DONE
* Rendering in a thread such that it continue while we move the window on screen

## DONE
* Support CRL+O and CTRL+Q and ESC... 
* Add panning
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
