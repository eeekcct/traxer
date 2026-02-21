use serde_json::Value;

#[test]
fn config_defaults_match_cli_expectations() {
    let cfg = traxer::Config::new("traxer");

    assert!(cfg.policy.is_none());
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

#[test]
fn policy_default_auto_matches_intended_defaults() {
    let policy = traxer::Policy::default_auto();

    assert!(matches!(
        policy.tty.output_format,
        Some(traxer::OutputFormat::Plain)
    ));
    assert!(matches!(policy.tty.color, Some(traxer::Color::Auto)));
    assert_eq!(policy.tty.include_pid, Some(false));
    assert_eq!(policy.tty.include_version, Some(false));

    assert!(matches!(
        policy.non_tty.output_format,
        Some(traxer::OutputFormat::Json)
    ));
    assert!(matches!(policy.non_tty.color, Some(traxer::Color::Never)));
    assert_eq!(policy.non_tty.include_pid, Some(true));
    assert_eq!(policy.non_tty.include_version, Some(true));
}

#[test]
fn config_accepts_custom_tty_and_non_tty_policies() {
    let policy = traxer::Policy {
        tty: traxer::ConfigOverride::new()
            .output_format(traxer::OutputFormat::Plain)
            .color(traxer::Color::Always)
            .span(true),
        non_tty: traxer::ConfigOverride::new()
            .output_format(traxer::OutputFormat::Json)
            .color(traxer::Color::Never)
            .span(false)
            .include_pid(true),
    };

    let cfg = traxer::Config::new("traxer").policy(policy.clone());

    let assigned = cfg.policy.expect("policy should be set");
    assert!(matches!(
        assigned.tty.output_format,
        Some(traxer::OutputFormat::Plain)
    ));
    assert!(matches!(
        assigned.non_tty.output_format,
        Some(traxer::OutputFormat::Json)
    ));
    assert_eq!(assigned.non_tty.include_pid, Some(true));
}
