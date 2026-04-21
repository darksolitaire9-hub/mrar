use dialoguer::Completion;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use rfd::FileDialog;
use std::fs;
use std::path::PathBuf;

/// GUI: pick input folder via OS dialog.
pub fn prompt_input_dir_gui() -> anyhow::Result<PathBuf> {
    let folder = FileDialog::new()
        .set_title("Select image folder")
        .pick_folder();

    if let Some(path) = folder {
        println!("   → Images from: {}", path.display());
        Ok(path)
    } else {
        anyhow::bail!("No folder selected")
    }
}

/// GUI: choose output folder, with default based on input_dir.
pub fn prompt_output_dir_gui(
    theme: &ColorfulTheme,
    input_dir: &PathBuf,
) -> anyhow::Result<PathBuf> {
    let default_out = input_dir.join("output");

    let options = &[
        format!("Use default output: {}", default_out.display()),
        "Browse for output folder in a dialog".to_string(),
        "Type or paste a different output path".to_string(),
    ];

    let choice = Select::with_theme(theme)
        .with_prompt("📂 Where should we save the processed images?")
        .items(options)
        .default(0)
        .interact()?;

    match choice {
        0 => {
            println!("   → Saving to: {}", default_out.display());
            Ok(default_out)
        }
        1 => {
            let mut dialog = FileDialog::new().set_title("Select output folder");
            if let Some(parent) = default_out.parent() {
                dialog = dialog.set_directory(parent);
            }

            if let Some(path) = dialog.pick_folder() {
                println!("   → Saving to: {}", path.display());
                Ok(path)
            } else {
                anyhow::bail!("No output folder selected")
            }
        }
        2 => prompt_output_dir_typed(theme, &default_out),
        _ => Ok(default_out),
    }
}

/// TUI: ask for input folder by typing/pasting path, with simple completion.
pub fn prompt_input_dir_typed(theme: &ColorfulTheme) -> anyhow::Result<PathBuf> {
    let completion = PathCompleter::default();

    let s: String = Input::with_theme(theme)
        .with_prompt("📁 Image folder (Tab to complete)")
        .completion_with(&completion)
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

    let path = PathBuf::from(s.trim());
    println!("   → Images from: {}", path.display());
    Ok(path)
}

/// TUI: ask for output folder, default to `<input>/output`.
pub fn prompt_output_dir_typed(
    theme: &ColorfulTheme,
    input_dir: &PathBuf,
) -> anyhow::Result<PathBuf> {
    let default_out = input_dir.join("output");
    let default_str = default_out.display().to_string();

    let s: String = Input::with_theme(theme)
        .with_prompt("📂 Output folder")
        .default(default_str)
        .interact_text()?;

    let path = PathBuf::from(s.trim());
    println!("   → Saving to: {}", path.display());
    Ok(path)
}

#[derive(Default)]
struct PathCompleter;

impl Completion for PathCompleter {
    fn get(&self, input: &str) -> Option<String> {
        use std::path::Path;

        let path = Path::new(input);
        let parent = path.parent().unwrap_or(Path::new("."));
        let prefix = path.file_name().and_then(|s| s.to_str()).unwrap_or("");

        let entries = fs::read_dir(parent).ok()?;
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with(prefix) {
                let candidate = parent.join(&*name_str);
                return Some(candidate.to_string_lossy().to_string());
            }
        }
        None
    }
}
