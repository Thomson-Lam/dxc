fn main() {
    if let Err(error) = dxc::run_args(std::env::args()) {
        eprintln!("dxc error: {error:?}");
        std::process::exit(1);
    }
}
