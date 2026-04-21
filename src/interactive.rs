// Impure: prompts the user and returns a fully resolved Config.
// Everything inside is I/O — the Config it returns is pure data.
use crate::cli::Config;
use dialoguer::{Confirm, Input, theme::ColorfulTheme};
use std::path::PathBuf;

pub fn prompt_config() -> anyhow::Result<Config> {
    let theme = ColorfulTheme::default();

    println!("\n  mrar — interactive mode\n");

    // ── 1. Input folder ───────────────────────────────────────────────────
    let input_str: String = Input::with_theme(&theme)
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
    let input_dir = PathBuf::from(input_str.trim());

    // ── 2. Output folder ──────────────────────────────────────────────────
    let default_out = input_dir.join("output").display().to_string();
    let output_str: String = Input::with_theme(&theme)
        .with_prompt("📂 Output folder")
        .default(default_out)
        .interact_text()?;
    let output_dir = PathBuf::from(output_str.trim());

    // ── 3. Compress? ──────────────────────────────────────────────────────
    let do_compress = Confirm::with_theme(&theme)
        .with_prompt("🗜  Compress images? (reduces file size, retains quality)")
        .default(false)
        .interact()?;

    let (shrink, quality) = if do_compress {
        let max_dim: u32 = Input::with_theme(&theme)
            .with_prompt("   Max dimension in pixels (width or height)")
            .default(1920u32)
            .validate_with(|v: &u32| -> Result<(), &str> {
                if *v >= 64 {
                    Ok(())
                } else {
                    Err("Must be at least 64px")
                }
            })
            .interact_text()?;

        let q: u8 = Input::with_theme(&theme)
            .with_prompt("   JPEG quality (1–100, 85–90 is visually lossless)")
            .default(90u8)
            .validate_with(|v: &u8| -> Result<(), &str> {
                if *v >= 1 {
                    Ok(())
                } else {
                    Err("Must be between 1 and 100")
                }
            })
            .interact_text()?;

        (Some(max_dim), q.clamp(1, 100))
    } else {
        (None, 90u8)
    };

    // ── 4. Dry run? ───────────────────────────────────────────────────────
    let dry_run = Confirm::with_theme(&theme)
        .with_prompt("🔍 Dry run? (preview only, no files written)")
        .default(false)
        .interact()?;

    println!();

    // Pure data out — no more I/O past this point
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
