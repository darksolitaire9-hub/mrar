use crate::pipeline::process::ProcessResult;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct ManifestEntry {
    pub index: u32,
    pub original: String,
    pub renamed_to: String,
    pub bytes_before: u64,
    pub bytes_after: u64,
    pub bytes_saved: u64,
    pub skipped: bool,
}

#[derive(Debug, Serialize)]
pub struct Manifest {
    pub total_images: usize,
    pub total_bytes_saved: u64,
    pub dry_run: bool,
    pub entries: Vec<ManifestEntry>,
}

/// Pure: build manifest from results
pub fn build_manifest(results: &[ProcessResult], dry_run: bool) -> Manifest {
    let total_bytes_saved = results.iter().map(|r| r.bytes_saved).sum();

    let entries = results
        .iter()
        .map(|r| ManifestEntry {
            index: r.index,
            original: r.original_path.to_string_lossy().into_owned(),
            renamed_to: r.target_path.to_string_lossy().into_owned(),
            bytes_before: r.bytes_before,
            bytes_after: r.bytes_after,
            bytes_saved: r.bytes_saved,
            skipped: r.skipped,
        })
        .collect();

    Manifest {
        total_images: results.len(),
        total_bytes_saved,
        dry_run,
        entries,
    }
}

/// Impure: write manifest.json to output dir
pub fn write_manifest(output_dir: &Path, manifest: &Manifest) -> anyhow::Result<()> {
    let path = output_dir.join("manifest.json");
    let json = serde_json::to_string_pretty(manifest)?;
    std::fs::write(&path, json)?;
    println!("manifest written → {}", path.display());
    Ok(())
}
