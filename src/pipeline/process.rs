use anyhow::Context;
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::path::PathBuf;

use crate::cli::Config;
use crate::error::MrarError;
use crate::pipeline::compress;
use crate::pipeline::metadata::{bytes_saved, strip_all};
use crate::pipeline::rename::resolve_target_path;

/// Represents one unit of work: a source file and its intended target path
#[derive(Debug, Clone)]
pub struct WorkItem {
    pub index: u32,
    pub original_path: PathBuf,
    pub target_path: PathBuf,
}

/// Result of processing one image
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub index: u32,
    pub original_path: PathBuf,
    pub target_path: PathBuf,
    pub bytes_before: u64,
    pub bytes_after: u64,
    pub bytes_saved: u64,
    pub skipped: bool, // true on dry_run
}

/// Pure: plan all work items from sorted paths
pub fn plan_work(paths: Vec<PathBuf>, config: &Config) -> Vec<WorkItem> {
    paths
        .into_iter()
        .enumerate()
        .map(|(i, original_path)| {
            let index = config.start + i as u32;
            let target_path = resolve_target_path(
                &original_path,
                &config.output_dir,
                index,
                config.pad,
                config.force_ext.as_deref(),
            );
            WorkItem {
                index,
                original_path,
                target_path,
            }
        })
        .collect()
}

/// Impure shell: process one work item (read → strip → write)
fn process_one(
    config: &Config,
    item: &WorkItem,
    pb: &ProgressBar,
) -> Result<ProcessResult, MrarError> {
    let filename = item
        .original_path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    pb.set_message(format!("processing {}", filename));

    // ── Impure: read ─────────────────────────────────────────────────────
    let original_bytes = std::fs::read(&item.original_path).map_err(|e| MrarError::Io {
        path: item.original_path.clone(),
        source: e,
    })?;
    let bytes_before = original_bytes.len() as u64;

    // ── Pure: strip metadata ──────────────────────────────────────────────
    let stripped = strip_all(&original_bytes).map_err(|e| MrarError::Strip {
        path: item.original_path.clone(),
        source: e,
    })?;

    // ── Pure: compress (optional) ─────────────────────────────────────────
    let ext = item
        .original_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");

    let final_bytes = if config.shrink.is_some() {
        compress::compress(&stripped, config.shrink, config.quality, ext).map_err(|e| {
            MrarError::Io {
                path: item.original_path.clone(),
                source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
            }
        })?
    } else {
        stripped
    };

    let bytes_after = final_bytes.len() as u64;
    let saved = bytes_saved(original_bytes.len(), final_bytes.len());

    if config.dry_run {
        pb.inc(1);
        return Ok(ProcessResult {
            index: item.index,
            original_path: item.original_path.clone(),
            target_path: item.target_path.clone(),
            bytes_before,
            bytes_after,
            bytes_saved: saved,
            skipped: true,
        });
    }

    if let Some(parent) = item.target_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| MrarError::Io {
            path: parent.to_path_buf(),
            source: e,
        })?;
    }

    std::fs::write(&item.target_path, &final_bytes).map_err(|e| MrarError::Io {
        path: item.target_path.clone(),
        source: e,
    })?;

    pb.inc(1);

    Ok(ProcessResult {
        index: item.index,
        original_path: item.original_path.clone(),
        target_path: item.target_path.clone(),
        bytes_before,
        bytes_after,
        bytes_saved: saved,
        skipped: false,
    })
}

/// Impure: run the full parallel pipeline over all work items
pub fn run_pipeline(config: &Config, items: Vec<WorkItem>) -> anyhow::Result<Vec<ProcessResult>> {
    let pb = ProgressBar::new(items.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::with_template(
            "{spinner:.cyan} [{bar:40.cyan/blue}] {pos}/{len} {msg}",
        )
        .unwrap()
        .progress_chars("█▓░"),
    );

    let results: Vec<_> = items
        .par_iter()
        .map(|item| process_one(config, item, &pb))
        .collect();

    pb.finish_with_message("✓ done");

    results
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .context("Pipeline failed")
}
