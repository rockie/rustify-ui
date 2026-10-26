fn main() {
    if let Err(error) = rustify_docs::serve_site(docs_theme_v2_fixture::site()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
