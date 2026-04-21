use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mrar",
    about = "Metadata Remover And Renamer — strips EXIF/IPTC/XMP and renames images sequentially",
    version
)]
pub struct CliArgs {
    /// Input directory containing raw images (omit to enter interactive mode)
    pub input: Option<PathBuf>,

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

    /// Shrink images so neither dimension exceeds this value (px)
    #[arg(long)]
    pub shrink: Option<u32>,

    /// Compression quality for JPEG output when shrinking (1–100)
    #[arg(long, default_value_t = 90)]
    pub quality: u8,

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
    pub shrink: Option<u32>,
    pub quality: u8,
    pub dry_run: bool,
}

impl Config {
    /// Pure: resolve config from CLI args
    pub fn resolve(args: CliArgs) -> Self {
        let input_dir = args.input.expect(
            "Config::resolve called without input — use interactive::prompt_config instead",
        );
        let output_dir = args.output.unwrap_or_else(|| input_dir.join("output"));

        Self {
            input_dir,
            output_dir,
            start: args.start,
            pad: args.pad,
            force_ext: args.ext,
            shrink: args.shrink,
            quality: args.quality.clamp(1, 100),
            dry_run: args.dry_run,
        }
    }
}
