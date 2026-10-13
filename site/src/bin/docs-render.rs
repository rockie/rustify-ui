fn main() {
    if let Err(error) = rustify_docs::serve_site(official_site::site()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
