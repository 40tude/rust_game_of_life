// build.rs
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../assets/40tude.ico"); // .ico + relative path
        res.compile().unwrap();
    }
}
