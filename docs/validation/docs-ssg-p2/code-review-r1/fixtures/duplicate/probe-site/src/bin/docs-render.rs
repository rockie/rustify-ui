fn main() {
    if let Err(error) = rustify_docs::serve_site(site_docs::site()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
