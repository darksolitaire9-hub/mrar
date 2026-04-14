use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use mrar::pipeline::metadata::strip_all;
use std::fs;
use std::hint::black_box;
use std::path::Path;

fn load_fixture(path: &str) -> Option<Vec<u8>> {
    let p = Path::new(path);
    if !p.exists() {
        eprintln!("skipping benchmark: {path} not found");
        return None;
    }
    match fs::read(p) {
        Ok(bytes) => Some(bytes),
        Err(e) => {
            eprintln!("skipping benchmark: failed to read {path}: {e}");
            None
        }
    }
}

fn bench_strip(c: &mut Criterion) {
    let fixtures = [
        ("small", "tests/fixtures/small.jpg"),
        ("large", "tests/fixtures/sample.jpg"),
    ];

    for (label, path) in fixtures {
        if let Some(data) = load_fixture(path) {
            let size = data.len();

            c.bench_with_input(
                BenchmarkId::new("strip_metadata", format!("{label}_{size}")),
                &data,
                |b, bytes| {
                    b.iter(|| {
                        let cleaned = strip_all(bytes).expect("strip_ok");
                        black_box(cleaned);
                    });
                },
            );
        }
    }
}

criterion_group!(benches, bench_strip);
criterion_main!(benches);
