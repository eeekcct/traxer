fn main() {
    let cfg = traxer::Config::new("traxer-try-init")
        .error_report(false)
        .with_base_field("component", "example");

    traxer::try_init(cfg.clone()).expect("first try_init should succeed");
    traxer::try_init(cfg).expect("second try_init should be a no-op");
    assert!(traxer::is_initialized());

    traxer::info!("try_init ok");
}
