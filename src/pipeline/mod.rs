// src/pipeline/mod.rs
pub mod metadata;
pub mod process;
pub mod rename;

pub use process::{plan_work, run_pipeline};
