fn main() {
    if let Err(err) = xr::run() {
        eprintln!("xr prototype failed: {err}");
        std::process::exit(1);
    }
}
