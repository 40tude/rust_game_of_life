## GPU

* `cargo run -p step_11 --release --example demo_00`
* Read https://www.40tude.fr/docs/06_programmation/rust/017_game_of_life/gpu.html#rust-winit--pixels-understanding-gpu-selection-and-performance


* `cargo add env_logger  -p step_11`


* Add in main()

```rust
std::env::set_var("RUST_LOG", "wgpu=info");
env_logger::init();
```



## TODO:
* Use GPU ?
* Rendering in a thread such that it continue while we move the window on screen
* Add more comments/documentation
* Add more tests

## DONE
* Load pattern from the command line
* Load pattern pressing 'O'
* utils::read_rle() should support patterns file without x and y
* Handle errors in place_pattern_centered() and when returning from place_pattern_centered()
* Remove cells_in_corners()
* Translate initial comments
* Split in modules and lib

## WEB
* https://copy.sh/life/examples/