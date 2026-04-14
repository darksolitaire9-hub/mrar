use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mrar",
    about = "Metadata Remover And Renamer — strips EXIF/IPTC/XMP and renames images sequentially",
    version
)]
pub struct CliArgs {
    /// Input directory containing raw images
    pub input: PathBuf,

    /// Output directory for cleaned, renamed images (default: <input>/output)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Starting index for sequential naming
    #[arg(long, default_value_t = 1)]
    pub start: u32,

    /// Zero-padding width  e.g. 3 → 001.jpg
    #[arg(long, default_value_t = 3)]
    pub pad: usize,

    /// Force output extension (e.g. jpg). Defaults to original extension.
    #[arg(long)]
    pub ext: Option<String>,

    /// Dry run: plan work and print manifest but do not write files
    #[arg(long)]
    pub dry_run: bool,
}

/// Resolved, validated configuration — pure data, no I/O
#[derive(Debug, Clone)]
pub struct Config {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub start: u32,
    pub pad: usize,
    pub force_ext: Option<String>,
    pub dry_run: bool,
}

impl Config {
    /// Pure: resolve config from CLI args
    pub fn resolve(args: CliArgs) -> Self {
        let output_dir = args.output.unwrap_or_else(|| args.input.join("output"));

        Self {
            input_dir: args.input,
            output_dir,
            start: args.start,
            pad: args.pad,
            force_ext: args.ext,
            dry_run: args.dry_run,
        }
    }
}
