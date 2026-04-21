// src/main.rs
mod cli;
mod error;
mod interactive;
mod manifest;
mod pipeline;
mod scan;

use anyhow::Context;
use clap::Parser;

use crate::error::MrarError;
use cli::{CliArgs, Config};
use manifest::{build_manifest, write_manifest};
use pipeline::{plan_work, run_pipeline};

fn main() -> anyhow::Result<()> {
    // ── Impure: parse CLI ─────────────────────────────
    let args = CliArgs::parse();
    let config: Config = if args.input.is_none() {
        interactive::prompt_config()?
    } else {
        Config::resolve(args)
    };

    // ── Impure: discover files ────────────────────────────
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

    // ── Pure: plan work items ─────────────────────────────
    let work_items = plan_work(paths, &config);

    // ── Impure: run parallel pipeline ─────────────────────
    let results = run_pipeline(&config, work_items)
        .context("Pipeline failed — check the output folder has write access")?;

    // ── Pure: build manifest ──────────────────────────────
    let manifest = build_manifest(&results, config.dry_run);

    // ── Impure: write manifest ────────────────────────────
    if !config.dry_run {
        write_manifest(&config.output_dir, &manifest).with_context(|| {
            format!(
                "Failed to write manifest to {}",
                config.output_dir.display()
            )
        })?;
    }

    // ── Print summary ─────────────────────────────────────
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
