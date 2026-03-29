fn main() {
    if let Err(error) = rust_rpg::run() {
        eprintln!("Fatal error: {error}");
        std::process::exit(1);
    }
}
