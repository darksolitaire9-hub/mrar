// src/main.rs
mod cli;
mod error;
mod interactive;
mod manifest;
mod pipeline;
mod scan;

use anyhow::Context;
use clap::Parser;
use dialoguer::{Confirm, Select};
use std::process::Command;

use crate::error::MrarError;
use cli::{CliArgs, Config};
use interactive::InteractiveMode;
use manifest::{build_manifest, write_manifest};
use pipeline::{plan_work, run_pipeline};

fn main() -> anyhow::Result<()> {
    // ── Impure: parse CLI ─────────────────────────────
    let args = CliArgs::parse();

    // Non-interactive CLI: run once and exit
    if args.input.is_some() {
        let config = Config::resolve(args);
        run_once(&config)?;

        println!(
            "✅ Done. Processed images are in: {}",
            config.output_dir.display()
        );

        return Ok(());
    }

    // Interactive mode: loop with explicit menu
    loop {
        let (config, mode) = interactive::prompt_config()?;

        run_once(&config)?;

        // Post-run UX: success + optional open folder
        match mode {
            InteractiveMode::Gui => {
                println!(
                    "✅ Done. Processed images are in: {}",
                    config.output_dir.display()
                );

                let open = Confirm::new()
                    .with_prompt("Open this folder in Explorer/Finder?")
                    .default(true)
                    .interact()?;

                if open {
                    open_folder_in_file_manager(&config.output_dir)?;
                }
            }
            InteractiveMode::Cli => {
                println!(
                    "✅ Done. Processed images are in: {}",
                    config.output_dir.display()
                );
            }
        }

        // Next-action menu
        let options = &[
            "Process another folder",
            "Re-run with the same settings",
            "Exit",
        ];

        let choice = Select::new()
            .with_prompt("What would you like to do next?")
            .items(options)
            .default(2) // default to "Exit"
            .interact()?;

        match choice {
            0 => {
                println!();
                continue;
            }
            1 => {
                println!();
                run_once(&config)?;
            }
            2 | _ => {
                break;
            }
        }

        println!();
    }

    Ok(())
}

fn run_once(config: &Config) -> anyhow::Result<()> {
    let paths = scan::discover_images(&config.input_dir).with_context(|| {
        format!(
            "Could not scan '{}' — check the path exists and you have read access",
            config.input_dir.display()
        )
    })?;

    if paths.is_empty() {
        return Err(MrarError::NoImages(config.input_dir.clone()).into());
    }

    println!(
        "Found {} image(s) → output: {}",
        paths.len(),
        config.output_dir.display()
    );

    if config.dry_run {
        println!("[dry-run] no files will be written");
    }

    let work_items = plan_work(paths, config);

    let results = run_pipeline(config, work_items)
        .context("Pipeline failed — check the output folder has write access")?;

    let manifest = build_manifest(&results, config.dry_run);

    if !config.dry_run {
        write_manifest(&config.output_dir, &manifest).with_context(|| {
            format!(
                "Failed to write manifest to {}",
                config.output_dir.display()
            )
        })?;
    }

    let processed = results.iter().filter(|r| !r.skipped).count();
    let saved_kb = manifest.total_bytes_saved / 1024;
    println!(
        "\n✓ {}/{} processed — {} KB saved across all images.",
        processed,
        results.len(),
        saved_kb,
    );

    Ok(())
}

fn open_folder_in_file_manager(path: &std::path::Path) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("failed to open Explorer: {e}"))
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("failed to open Finder: {e}"))
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow::anyhow!("failed to open file manager: {e}"))
    }
}
