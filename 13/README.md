## Add logging

* `cargo run -p step_13`
* `cargo add log  -p step_13`
* In addition to `env_logger` already here

* In lib.rs

```rust
pub mod prelude {
    pub use log::{debug, error, info, trace, warn};
}
```
* In `main.rs`
    * `env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("step_13=info, wgpu_core=warn, wgpu_hal=warn, wgpu=warn")).init();`
        * It loads the `RUST_LOG` environment variable if it exists.
        * Otherwise, it applies the fallback `"step_13=info, wgpu_core=info, wgpu_hal=warn, wgpu=warn"`.
        * So by default, we see info logs for our app and warn logs for WGPU.
        * If we want to see everything, you can force it: `$env:RUST_LOG = "info"; cargo run -p step_13`
            * `$env:RUST_LOG = "debug"; cargo run -p step_13`
            * `$env:RUST_LOG = "trace"; cargo run -p step_13`
            * `$env:RUST_LOG = "info"; cargo run -p step_13`
        * **IMPORTANT**: `Remove-Item Env:RUST_LOG` otherwise `RUST_LOG` is still taken into account on the next `cargo run -p step_13`
    * `log::info!()` 

* Other files
    * `use crate::prelude::*;`
    * `info!("handle_resize(): nothing to do.");`







## TODO:
* Logging in files
* Add more comments/documentation
* Add more tests

## NOT DONE
* Rendering in a thread such that it continue while we move the window on screen

## DONE
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
