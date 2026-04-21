// tests/interactive_smoke.rs

#[test]
fn prompt_config_smoke_test_optional() {
    if std::env::var("MRAR_RUN_INTERACTIVE_TESTS").is_err() {
        eprintln!("skipped: set MRAR_RUN_INTERACTIVE_TESTS=1 to run");
        return;
    }

    let (config, mode) = mrar::interactive::prompt_config().expect("prompt_config ok");

    // Basic sanity checks; we can't assert specific paths.
    assert!(config.quality >= 1 && config.quality <= 100);

    match mode {
        mrar::interactive::InteractiveMode::Gui | mrar::interactive::InteractiveMode::Cli => {}
    }
}
