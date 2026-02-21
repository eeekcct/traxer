fn main() {
    let mut use_json = false;
    for arg in std::env::args().skip(1) {
        if arg == "--plain" {
            use_json = false;
        } else if arg == "--json" {
            use_json = true;
        }
    }

    let base = traxer::Config::new("traxer-fields")
        .color(traxer::Color::Always)
        .verbose(1);
    let config = if use_json { base.json() } else { base.plain() };
    traxer::init(config);

    let user_id = 42;
    let elapsed_ms = 128;
    traxer::info!(
        user_id,
        elapsed_ms,
        action = "login",
        "user action processed"
    );
}
