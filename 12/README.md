## Apply what we learn about GPU 

* `cargo run -p step_12` 
* See `state.rs`
    * We can select the GPU, the backend and the presentation mode
    * But decide to keep the default options



## TODO:
* Add a logger
* Add more comments/documentation
* Add more tests

## NOT DONE
* Rendering in a thread such that it continue while we move the window on screen

## DONE
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