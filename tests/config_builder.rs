use serde_json::Value;

#[test]
fn config_defaults_match_cli_expectations() {
    let cfg = traxer::Config::new("traxer");

    assert!(matches!(cfg.output_format, traxer::OutputFormat::Plain));
    assert!(matches!(cfg.stream, traxer::Stream::Stderr));
    assert!(matches!(cfg.color, traxer::Color::Auto));
    assert_eq!(cfg.verbose, 0);
    assert_eq!(cfg.quiet, 0);
    assert!(cfg.filter_directives.is_none());
    assert!(cfg.base_fields.is_empty());
    assert!(!cfg.include_pid);
    assert!(!cfg.include_exe);
    assert!(!cfg.include_version);
    assert!(!cfg.span);
    assert!(cfg.error_report);
}

#[test]
fn config_builder_sets_new_options() {
    let cfg = traxer::Config::new("traxer")
        .json()
        .stream(traxer::Stream::Stdout)
        .with_filter_directives("myapp=debug")
        .with_base_field("service", "cli")
        .with_base_field("attempt", 3)
        .with_pid()
        .with_exe()
        .with_version()
        .span(true)
        .error_report(false);

    assert!(matches!(cfg.output_format, traxer::OutputFormat::Json));
    assert!(matches!(cfg.stream, traxer::Stream::Stdout));
    assert_eq!(cfg.filter_directives.as_deref(), Some("myapp=debug"));
    assert!(
        cfg.base_fields
            .contains(&("service".into(), Value::String("cli".to_string())))
    );
    assert!(
        cfg.base_fields
            .contains(&("attempt".into(), Value::Number(3u64.into())))
    );
    assert!(cfg.include_pid);
    assert!(cfg.include_exe);
    assert!(cfg.include_version);
    assert!(cfg.span);
    assert!(!cfg.error_report);
}
