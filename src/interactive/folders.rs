use dialoguer::{Input, theme::ColorfulTheme};
use std::path::PathBuf;

pub fn prompt_input_dir(theme: &ColorfulTheme) -> anyhow::Result<PathBuf> {
    let s: String = Input::with_theme(theme)
        .with_prompt("📁 Image folder")
        .validate_with(|s: &String| -> Result<(), String> {
            let p = PathBuf::from(s.trim());
            if p.is_dir() {
                Ok(())
            } else {
                Err(format!(
                    "'{}' is not a directory or does not exist",
                    s.trim()
                ))
            }
        })
        .interact_text()?;

    Ok(PathBuf::from(s.trim()))
}

pub fn prompt_output_dir(theme: &ColorfulTheme, input_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    let default = input_dir.join("output").display().to_string();

    let s: String = Input::with_theme(theme)
        .with_prompt("📂 Output folder")
        .default(default)
        .interact_text()?;

    Ok(PathBuf::from(s.trim()))
}
