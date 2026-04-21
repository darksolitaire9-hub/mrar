use dialoguer::{Confirm, theme::ColorfulTheme};

pub fn prompt_dry_run(theme: &ColorfulTheme) -> anyhow::Result<bool> {
    let dry_run = Confirm::with_theme(theme)
        .with_prompt("🔍 Preview only (dry run)?")
        .default(false)
        .interact()?;

    if dry_run {
        println!("   → No files will be created or overwritten.");
    }

    Ok(dry_run)
}
