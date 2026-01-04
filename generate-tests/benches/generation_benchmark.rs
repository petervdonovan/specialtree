use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::path::Path;

use codegen_component::{CgDepList, Component2SynPath, Crate};
use extension_autobox::autobox;
use langspec::{
    flat::LangSpecFlat,
    langspec::{LangSpec, SortIdOf, TerminalLangSpec},
    sublang::{SublangsList, reflexive_sublang},
};

use generate_tests::shared::{ds_crate, traits_crate};

/// Delegates full generation to `shared::run_code_generation`.
/// The benchmark creates and removes the temporary directory around the call to
/// preserve the previous per-iteration behavior.
fn run_code_generation(temp_dir: &Path) -> usize {
    generate_tests::shared::run_code_generation(temp_dir)
}

fn benchmark_code_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("code_generation");

    // Configure for fast but reliable measurement
    group.sample_size(10); // Fewer samples since generation is expensive
    group.measurement_time(std::time::Duration::from_secs(20)); // 20 second measurement window

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            // Per-iteration temp dir to preserve previous behavior
            let temp_dir = std::env::temp_dir().join("benchmark_output");
            std::fs::create_dir_all(&temp_dir).unwrap();
            let components = run_code_generation(&temp_dir);
            std::fs::remove_dir_all(&temp_dir).ok();
            black_box(components) // Prevent optimization of the result
        })
    });

    group.finish();
}

fn micro_benchmark_ty_gen_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("micro_ty_gen_data");

    // Set up language specs once outside the benchmark
    let arena = bumpalo::Bump::new();
    let fib = LangSpecFlat::canonical_from(&langspec_examples::fib());
    let fib_autobox = autobox(&fib);
    let fib_pat = extension_pattern::patternfy(&arena, &fib);

    group.bench_function("ty_gen_datas_fib", |b| {
        b.iter(|| {
            use langspec_rs_syn::ty_gen_datas;
            let data: Vec<_> = ty_gen_datas(&fib, None).collect();
            black_box(data.len())
        })
    });

    group.bench_function("ty_gen_datas_fib_autobox", |b| {
        b.iter(|| {
            use langspec_rs_syn::ty_gen_datas;
            let data: Vec<_> = ty_gen_datas(&fib_autobox, None).collect();
            black_box(data.len())
        })
    });

    group.bench_function("ty_gen_datas_fib_pat", |b| {
        b.iter(|| {
            use langspec_rs_syn::ty_gen_datas;
            let data: Vec<_> = ty_gen_datas(&fib_pat, None).collect();
            black_box(data.len())
        })
    });

    group.finish();
}

fn micro_benchmark_sort_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("micro_sort_operations");

    let arena = bumpalo::Bump::new();
    let fib = LangSpecFlat::canonical_from(&langspec_examples::fib());

    // Benchmark heapbak_gen_datas which is public
    group.bench_function("heapbak_gen_datas_fib", |b| {
        b.iter(|| {
            use langspec_rs_syn::heapbak_gen_datas;
            let data = heapbak_gen_datas(&fib);
            black_box(data.len())
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_code_generation,
    micro_benchmark_ty_gen_data,
    micro_benchmark_sort_operations
);
criterion_main!(benches);
