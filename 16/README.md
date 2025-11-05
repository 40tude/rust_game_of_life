## Code review
* `cargo run -p step_16`

* I believe some error management and match can be avoided
* Check the invariants
* See tool in rust\01_xp\26_callgraph_ascii



## TODO:
* Need to evaluate efficiency of the app
    * ????
    * How long does it take between to requests for redraw compare to 60 FPS?
* Add more comments/documentation
* Add more tests

## NOT DONE
* Rendering in a thread such that it continue while we move the window on screen

## DONE
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

