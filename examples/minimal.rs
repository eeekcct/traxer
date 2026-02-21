fn main() {
    let cfg = traxer::Config::new("traxer-example").color(traxer::Color::Always);
    traxer::init(cfg);

    traxer::error!("error message");
    traxer::warn!("warn message");
    traxer::info!("info message");
    traxer::debug!("debug message");
    traxer::trace!("trace message");
}
