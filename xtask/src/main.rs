fn main() {
    if let Err(e) = fastn_xtask::main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
