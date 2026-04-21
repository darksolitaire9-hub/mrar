// tests/config_flow.rs

use std::path::PathBuf;
use mrar::cli::{CliArgs, Config};

#[test]
fn cli_args_with_input_drive_non_interactive_config() {
    let args = CliArgs {
        input: Some(PathBuf::from("photos")),
        output: None,
        start: 1,
        pad: 3,
        ext: None,
        shrink: None,
        quality: 90,
        dry_run: false,
    };

    let config = Config::resolve(args);

    assert_eq!(config.input_dir, PathBuf::from("photos"));
    assert_eq!(config.output_dir, PathBuf::from("photos/output"));
}