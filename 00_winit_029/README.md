## Ref

### winit
* crate: https://crates.io/crates/winit
* doc: https://docs.rs/winit/0.30.12/winit/
* examples: https://github.com/rust-windowing/winit/tree/master/winit/examples

### Pixels
* crate: https://crates.io/crates/pixels
* doc: https://docs.rs/pixels/0.15.0/pixels/
* examples: https://github.com/parasyte/pixels/tree/main/examples




## About the rwh_05 feature

See `cargo.toml` in the project `00_winit_030`

```toml
winit = { version = "0.30.12", features = ["rwh_05"] }
```

### What is Raw Window Handle?

**Raw Window Handle** is a standardized interface in Rust for sharing native window handles between different libraries, without them needing to know about each other directly.

### The problem it solves

Imagine:
- **winit** creates an OS window (Windows HWND, Linux XWindow, macOS NSWindow)
- **pixels** (which uses wgpu) needs access to this window for rendering
- **wgpu** must be able to draw on this window

Without `raw-window-handle`, each library would have to support all window types from all possible libraries—a compatibility nightmare!

### The solution

The `raw-window-handle` crate defines standard traits:

```rust
pub trait HasRawWindowHandle {
    fn raw_window_handle(&self) -> RawWindowHandle;
}
```

### Versions `rwh_05` vs `rwh_06`

The features indicate which version of the `raw-window-handle` crate is used

| Feature | Version crate | Utilisé par |
|---------|---------------|-------------|
| `rwh_05` | raw-window-handle **0.5.x** | winit 0.29, wgpu 0.19 (pixels 0.15) |
| `rwh_06` | raw-window-handle **0.6.x** | winit 0.30, wgpu 0.20+ |

### Why do we need to specify this feature?

#### With winit **0.30** + pixels **0.15**:

```toml
[dependencies]
pixels = “0.15.0”          # Internally uses rwh 0.5
winit = { version = “0.30.12”, features = [“rwh_05”] }  # Compatible with 0.5 AND 0.6
```

* Problem: winit 0.30 natively uses `rwh 0.6`, but pixels 0.15 uses `rwh 0.5`. 
* Solution: The `rwh_05` feature activates a compatibility mode in winit 0.30 so that it also exposes the old 0.5 interface, allowing pixels 0.15 to work.


#### With winit **0.29** + pixels **0.15**:

```toml
[dependencies]
pixels = "0.15.0"
winit = "0.29"  # Pas besoin de feature, utilise rwh 0.5 nativement
```


### Example 

When we write 

```rust
let surface = SurfaceTexture::new(width, height, &window);
```

Behind the scenes 
1. `SurfaceTexture` (of pixels) calls `window.raw_window_handle()`
2. winit returns a native handle (e.g., `HWND` on Windows)
3. wgpu uses this handle to create a DirectX/Vulkan/Metal rendering surface
4. It all works because they all speak the same “language”: `raw-window-handle`


### TL;DR

| Configuration                 | Cargo.toml                                            | Status                   |
|-------------------------------|-------------------------------------------------------|--------------------------|
| **pixels 0.15 + winit 0.29**  | `winit = "0.29"`                                      | Officially supported     |
| **pixels 0.15 + winit 0.30**  | `winit = { version = "0.30", features = ["rwh_05"] }` | Compatibilty hack        |














## About the .pptx in assets/

### Overall structure

1. **Board (400×300)**

   * Holds the logical state of our world, typically a `Vec<bool>` or `Vec<u8>` representing live/dead cells.
   * This is the *logical reality* of our simulation.

2. **Pixels buffer (400×300 RGBA)**

   * This is the image buffer in memory, directly accessible via:

     ```rust
     let frame = pixels.frame_mut();
     ```
   * Each pixel is stored as 4 bytes (R, G, B, A).
   * We draw our `Board` data into this buffer — that’s our software rendering step.

3. **Texture (800×600)**

   * The GPU texture that receives the RGBA buffer and is displayed on screen.
   * This is where `pixels.render()` comes in — it uploads the buffer to the GPU.

4. **Window (800×600)**

   * The actual OS-level window created with `winit`.
   * It displays the texture rendered by `pixels`.

5. **Screen**

   * What the user physically sees — a desktop window on the monitor.











### Creation pipeline

The diagram shows the data flow:

1. `surface = SurfaceTexture::new(win_w, win_h, window)`: creates the drawing surface linked to the window (physical display size).
2. `pixels = Pixels::new(buffer_w, buffer_h, surface)`: creates the `Pixels` object using a **logical resolution** (the 400×300 buffer), which may differ from the window size (800×600).

The difference in sizes means **automatic upscaling** — each logical pixel becomes 4 real pixels on screen.





### Logical vs. graphical rendering

* The **board** represents the *simulation state* (logic, CPU-side).
* The **pixels buffer** represents the *visual state* (image data, CPU-side).
* The **texture** and **window** are the *GPU-side* rendering targets.

So the data flow is:

```
board → frame → texture → window
```








### The three layers of resolution in `pixels`

When we use the `pixels` crate, there are **three different “sizes”** to keep in mind:

| Concept                         | Example size | Owner             | What it means                                                                        |
| ------------------------------- | ------------ | ----------------- | ------------------------------------------------------------------------------------ |
| **Logical resolution**          | 400×300      | You (your buffer) | The size of your simulation or “game board.” Each cell = one logical pixel.          |
| **Render resolution (buffer)**  | 400×300 RGBA | `pixels`          | The image buffer you draw into with `frame_mut()`. One byte array of width×height×4. |
| **Display resolution (window)** | 800×600      | `winit` / OS      | The actual pixel size of the window on screen, in **physical pixels**.               |

Now, where it gets tricky is that the display may also have a **device pixel ratio** — also known as *HiDPI scaling* or *scaling factor*.


#### Understanding the scaling factor (HiDPI)

Modern screens (4K monitors, macOS...) have **more physical pixels** per logical pixel.
So for example:
* Our window might be *logically* 800×600,
* But *physically* 1600×1200 on a 2× scaling display or Retina

That ratio (here `2.0`) is called the **scaling factor** or **device pixel ratio**.

`winit` exposes it as:

```rust
let scale_factor = window.scale_factor();
```


#### How `pixels` deals with that

When we call:

```rust
let surface = SurfaceTexture::new(window_width, window_height, &window);
let mut pixels = Pixels::new(buffer_width, buffer_height, surface)?;
```

We are explicitly telling `pixels`:

* “My logical frame buffer (the thing I draw into) is `buffer_width × buffer_height`.”
* “Please stretch or shrink it to fill the *window’s physical size* (`window_width × window_height`).”

So internally, `pixels` performs a **scaling operation** between the two.

That scaling can be:

* **Upscaling** (if your buffer is smaller than the window),
* **Downscaling** (if your buffer is larger),
* or **1:1** (if they match exactly).


##### Example

With the earlier numbers:

```rust
buffer_w = 400
buffer_h = 300
win_w = 800
win_h = 600
scale_factor = 1.0
```

The `pixels` crate draws your 400×300 buffer as an 800×600 texture = **upscaled by ×2**. Now, if we move the same app to a Retina display with a scale factor of 2.0, `winit` might report:

```rust
win_w = 1600
win_h = 1200
scale_factor = 2.0
```

Even though the *logical window* is still 800×600, the physical pixels available are doubled, so the texture is now scaled ×4 relative to our logical buffer.


#### Why it matters

* We should **always query the scale factor** from `winit` and adjust accordingly.

  ```rust
  let scale = window.scale_factor() as f32;
  ```

* When resizing, we use:

  ```rust
  pixels.resize_surface(new_physical_width, new_physical_height);
  pixels.resize_buffer(new_logical_width, new_logical_height);
  ```

* If the graphics look *blurry*, it’s usually because the **logical buffer** is much smaller than the **physical window** after scaling.

#### Summary

| Term                    | Controlled by         | Description                                                      |
| ----------------------- | --------------------- | ---------------------------------------------------------------- |
| **Logical resolution**  | Us                    | Size of our simulation buffer (what we draw)                     |
| **Physical resolution** | OS / monitor          | Actual pixels on the screen                                      |
| **Scaling factor**      | OS / windowing system | Converts between logical and physical coordinates                |
| **`pixels` role**       | `pixels` crate        | Scales our buffer to fit the surface according to those values   |

















### What happens after `pixels.render()`

When we call:

```rust
pixels.render()?;
```

We are triggering a **GPU rendering pipeline** managed internally by the `wgpu` backend.
Here’s what happens behind the curtain:

1. **The CPU uploads our frame buffer**
   The memory we wrote to with `frame = pixels.frame_mut()` (our RGBA bytes) is uploaded from system RAM to the GPU as a texture resource.

2. **A render pass is created**
   The `pixels` crate sets up a small render pipeline with a full-screen quad (two triangles covering the entire viewport).
   This is standard GPU technique: the texture is simply drawn as a rectangle filling the screen.

3. **The texture is sampled and drawn**
   The shader used by `pixels` reads your uploaded texture and writes it into the current render target — the *swap chain texture* provided by the `SurfaceTexture` (linked to our window).

4. **The frame is presented**
   When the render pass finishes, `pixels` calls `surface.present()` (internally via wgpu).
   This tells the GPU to swap the back buffer to the front buffer — the classic double-buffer mechanism.
   That’s the moment your new image actually becomes visible in the window.


### When does this happen?

* It happens **synchronously** when we call `pixels.render()`.
  The command queue is submitted to the GPU, and the frame is presented right afterward.
* However, the **display refresh rate** (typically 60 Hz) determines *when* the monitor will show that frame.
  So if our app runs at 300 FPS, the GPU might still only present frames at the monitor’s refresh rate (depending on VSync settings).


### So is it transparent?

Yes — for us as developers using the `pixels` crate, it’s **completely transparent**.
`pixels.render()` handles:

* uploading the data,
* issuing the GPU draw commands,
* and presenting the frame to the window.

We don’t have to touch `wgpu::Surface`, `SwapChain`, or any low-level GPU concepts.

