fn main() {
    if let Err(e) = fastn_xtask::cli() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
