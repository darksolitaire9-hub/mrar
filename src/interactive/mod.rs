mod compression;
mod dry_run;
mod folders;

use crate::cli::Config;
use dialoguer::theme::ColorfulTheme;

pub fn prompt_config() -> anyhow::Result<Config> {
    let theme = ColorfulTheme::default();

    println!("\n  mrar — interactive mode\n");

    let input_dir = folders::prompt_input_dir(&theme)?;
    let output_dir = folders::prompt_output_dir(&theme, &input_dir)?;
    let (shrink, quality) = compression::prompt_compression(&theme)?;
    let dry_run = dry_run::prompt_dry_run(&theme)?;

    println!();

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
