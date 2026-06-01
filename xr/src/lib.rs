pub mod app;
pub mod config;
pub mod openxr;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}

#[cfg(target_os = "android")]
#[cfg_attr(target_os = "android", ndk_glue::main)]
fn main() {
    if let Err(err) = run() {
        eprintln!("xr prototype failed: {err}");
    }
}
