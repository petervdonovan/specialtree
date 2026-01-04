use criterion::{Criterion, criterion_group, criterion_main};
use memo::memo_cache::thread_local_cache;
use std::path::Path;

fn run_code_generation(temp_dir: &Path) -> usize {
    generate_tests::shared::run_code_generation(temp_dir)
}

fn benchmark_code_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("code_generation");

    group.sample_size(10);
    group.measurement_time(std::time::Duration::from_secs(20));

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            let temp_dir = std::env::temp_dir().join("benchmark_output");
            std::fs::create_dir_all(&temp_dir).unwrap();
            let components = run_code_generation(&temp_dir);
            std::fs::remove_dir_all(&temp_dir).ok();
            thread_local_cache().clear();
            std::hint::black_box(components)
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_code_generation);
criterion_main!(benches);
