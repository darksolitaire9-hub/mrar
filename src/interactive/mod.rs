use crate::cli::Config;
use dialoguer::{Select, theme::ColorfulTheme};

mod compression;
mod dry_run;
mod folders;

#[derive(Debug, Clone, Copy)]
pub enum InteractiveMode {
    Gui,
    Cli,
}

pub fn prompt_config() -> anyhow::Result<(Config, InteractiveMode)> {
    let theme = ColorfulTheme::default();

    println!("\n  mrar — interactive mode\n");

    let mode_idx = Select::with_theme(&theme)
        .with_prompt("How do you want to select your image folder?")
        .items(&[
            "Browse for a folder  – opens a dialog to click and select  (recommended)",
            "Type a folder path   – paste or type directly here",
        ])
        .default(0)
        .interact()?;

    let (config, mode) = match mode_idx {
        0 => (prompt_config_gui(&theme)?, InteractiveMode::Gui),
        1 => (prompt_config_cli(&theme)?, InteractiveMode::Cli),
        _ => (prompt_config_gui(&theme)?, InteractiveMode::Gui),
    };

    println!();

    Ok((config, mode))
}

fn prompt_config_gui(theme: &ColorfulTheme) -> anyhow::Result<Config> {
    // 1) Input via GUI folder picker
    let input_dir = folders::prompt_input_dir_gui()?;

    // 2) Output with default + optional GUI override
    let output_dir = folders::prompt_output_dir_gui(theme, &input_dir)?;

    // 3) Compression preset
    let (shrink, quality) = compression::prompt_compression(theme)?;

    // 4) Dry run
    let dry_run = dry_run::prompt_dry_run(theme)?;

    Ok(Config {
        input_dir,
        output_dir,
        start: 1,
        pad: 3,
        force_ext: None,
        shrink,
        quality,
        dry_run,
    })
}

fn prompt_config_cli(theme: &ColorfulTheme) -> anyhow::Result<Config> {
    // 1) Input via typed path
    let input_dir = folders::prompt_input_dir_typed(theme)?;

    // 2) Output via typed path (with default)
    let output_dir = folders::prompt_output_dir_typed(theme, &input_dir)?;

    // 3) Compression preset
    let (shrink, quality) = compression::prompt_compression(theme)?;

    // 4) Dry run
    let dry_run = dry_run::prompt_dry_run(theme)?;

    Ok(Config {
        input_dir,
        output_dir,
        start: 1,
        pad: 3,
        force_ext: None,
        shrink,
        quality,
        dry_run,
    })
}
