## Measure Performances

* See state.rs/PerformanceMetrics
* See events.rs/WindowEvent::RedrawRequested
* `cargo run --release -p step_17_winit_030 -- --pattern rle/glider`
* `cargo run --release -p step_17_winit_030 -- --pattern rle/112P51_synth`



With  112P51_synth 140 W Release Intel, Vulkan, Fifo

```
cargo run --release -p step_17_winit_030 -- --pattern rle/112P51_synth
    Finished `release` profile [optimized] target(s) in 0.25s
     Running `target\release\step_17_winit_030.exe --pattern rle/112P51_synth`
INFO [step_17_winit_030] Logger initialized.
INFO [step_17_winit_030] Using pattern file: rle/112P51_synth.rle
INFO [step_17_winit_030] App initialized successfully, starting event loop...
INFO [wgpu_core::instance] Adapter Vulkan AdapterInfo { name: "Intel(R) Iris(R) Xe Graphics", vendor: 32902, device: 18086, device_type: IntegratedGpu, driver: "Intel Corporation", driver_info: "Intel driver", backend: Vulkan }
INFO [step_17_winit_030::app::state] Present mode: Fifo
INFO [step_17_winit_030::app::events] Perf: step=  0.30ms (p95=  0.30ms) | render=  1.56ms | total=  1.86ms | theo_fps= 538 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.30ms) | render=  0.50ms | total=  0.78ms | theo_fps=1285 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.32ms) | render=  0.51ms | total=  0.79ms | theo_fps=1261 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.29ms) | render=  0.50ms | total=  0.78ms | theo_fps=1290 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.28ms) | render=  0.25ms | total=  0.53ms | theo_fps=1904 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.28ms) | render=  0.21ms | total=  0.49ms | theo_fps=2040 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.29ms) | render=  0.23ms | total=  0.51ms | theo_fps=1980 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.28ms) | render=  0.24ms | total=  0.52ms | theo_fps=1926 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.28ms) | render=  0.23ms | total=  0.51ms | theo_fps=1968 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.28ms) | render=  0.25ms | total=  0.53ms | theo_fps=1879 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.31ms) | render=  0.28ms | total=  0.56ms | theo_fps=1785 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.29ms) | render=  0.26ms | total=  0.54ms | theo_fps=1865 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.28ms) | render=  0.23ms | total=  0.51ms | theo_fps=1960 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.30ms) | render=  0.25ms | total=  0.53ms | theo_fps=1886 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.30ms) | render=  0.25ms | total=  0.54ms | theo_fps=1869 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.28ms (p95=  0.30ms) | render=  0.26ms | total=  0.54ms | theo_fps=1862 | board=356x200
INFO [step_17_winit_030] Application terminated.

```






With  112P51_synth 300 W Release

```
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.26ms) | render=  0.78ms | total=  1.02ms | theo_fps= 980 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.27ms) | render=  0.71ms | total=  0.96ms | theo_fps=1037 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.26ms (p95=  0.27ms) | render=  0.69ms | total=  0.95ms | theo_fps=1055 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.26ms) | render=  0.71ms | total=  0.96ms | theo_fps=1041 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.24ms (p95=  0.26ms) | render=  0.69ms | total=  0.93ms | theo_fps=1074 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.24ms (p95=  0.25ms) | render=  0.84ms | total=  1.08ms | theo_fps= 925 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.26ms) | render=  0.69ms | total=  0.94ms | theo_fps=1059 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.27ms) | render=  0.70ms | total=  0.95ms | theo_fps=1049 | board=356x200
```

Alim 300W + NVIDIA only

```
cargo run --release -p step_17_winit_030 -- --pattern rle/112P51_synth                                                                                                                                                                   
    Finished `release` profile [optimized] target(s) in 0.20s
     Running `target\release\step_17_winit_030.exe --pattern rle/112P51_synth`
INFO [step_17_winit_030] Logger initialized.
INFO [step_17_winit_030] Using pattern file: rle/112P51_synth.rle
INFO [step_17_winit_030] App initialized successfully, starting event loop...
WARN [wgpu_core::instance] Missing downlevel flags: DownlevelFlags(VERTEX_AND_INSTANCE_INDEX_RESPECTS_RESPECTIVE_FIRST_VALUE_IN_INDIRECT_DRAW)
The underlying API or device in use does not support enough features to be a fully compliant implementation of WebGPU. A subset of the features can still be used. If you are running this program on native and not in a browser and wish to limit the features you use to the supported subset, call Adapter::downlevel_properties or Device::downlevel_properties to get a listing of the features the current platform supports.
WARN [wgpu_core::instance] DownlevelCapabilities {
    flags: DownlevelFlags(
        COMPUTE_SHADERS | FRAGMENT_WRITABLE_STORAGE | INDIRECT_EXECUTION | BASE_VERTEX | READ_ONLY_DEPTH_STENCIL | NON_POWER_OF_TWO_MIPMAPPED_TEXTURES | CUBE_ARRAY_TEXTURES | COMPARISON_SAMPLERS | INDEPENDENT_BLEND | VERTEX_STORAGE | ANISOTROPIC_FILTERING | FRAGMENT_STORAGE | MULTISAMPLED_SHADING | DEPTH_TEXTURE_AND_BUFFER_COPIES | WEBGPU_TEXTURE_FORMAT_SUPPORT | BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED | UNRESTRICTED_INDEX_BUFFER | FULL_DRAW_INDEX_UINT32 | DEPTH_BIAS_CLAMP | VIEW_FORMATS | UNRESTRICTED_EXTERNAL_TEXTURE_COPIES | SURFACE_VIEW_FORMATS | NONBLOCKING_QUERY_RESOLVE,
    ),
    limits: DownlevelLimits,
    shader_model: Sm5,
}
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.30ms) | render=  0.67ms | total=  0.92ms | theo_fps=1083 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.26ms) | render=  0.73ms | total=  0.98ms | theo_fps=1020 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.27ms (p95=  0.39ms) | render=  0.58ms | total=  0.86ms | theo_fps=1165 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.27ms) | render=  0.54ms | total=  0.79ms | theo_fps=1264 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.26ms (p95=  0.29ms) | render=  0.55ms | total=  0.80ms | theo_fps=1246 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.26ms (p95=  0.28ms) | render=  0.56ms | total=  0.82ms | theo_fps=1225 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.29ms) | render=  0.56ms | total=  0.81ms | theo_fps=1233 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.27ms) | render=  0.53ms | total=  0.78ms | theo_fps=1290 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.27ms) | render=  0.56ms | total=  0.81ms | theo_fps=1242 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.28ms) | render=  0.56ms | total=  0.81ms | theo_fps=1230 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.26ms (p95=  0.29ms) | render=  0.60ms | total=  0.86ms | theo_fps=1164 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.25ms (p95=  0.27ms) | render=  0.57ms | total=  0.82ms | theo_fps=1215 | board=356x200
INFO [step_17_winit_030::app::events] Perf: step=  0.26ms (p95=  0.29ms) | render=  0.56ms | total=  0.82ms | theo_fps=1226 | board=356x200
INFO [step_17_winit_030] Application terminated.
```
















## TODO:
* Add more comments/documentation
* Add more tests

## NOT DONE
* Rendering in a thread such that it continue while we move the window on screen

## DONE
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

