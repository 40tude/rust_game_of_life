## Add icon

### 1
* create .ico (https://www.icoconverter.com/, all resolutions)
* create build.rs
* cargo add winres

### 2
* cargo add image
* add src/app/events `fn load_icon() -> Option<Icon> {...}`
* In resumed() set the icon if loaded (see ``)





## TODO:
* Add more comments/documentation
* Add more tests

## NOT DONE
* Rendering in a thread such that it continue while we move the window on screen

## DONE
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

