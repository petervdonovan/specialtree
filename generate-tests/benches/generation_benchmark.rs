use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::path::Path;

use codegen_component::{CgDepList, Component2SynPath, Crate};
use extension_autobox::autobox;
use langspec::{
    flat::LangSpecFlat,
    langspec::{LangSpec, SortIdOf, TerminalLangSpec},
    sublang::{SublangsList, reflexive_sublang},
};

/// Core generation logic extracted from main() for benchmarking
/// Returns total number of components generated to ensure work isn't optimized away
fn run_code_generation() -> usize {
    let arena = bumpalo::Bump::new();
    let fib = LangSpecFlat::canonical_from(&langspec_examples::fib());
    let fib_autobox = autobox(&fib);
    let fib_pat = extension_pattern::patternfy(&arena, &fib);
    let fib_pat_autobox = autobox(&fib_pat);

    let root_cgd = CgDepList::new();
    let fib_deps = [("tymetafuncspec-core", Path::new("."))];
    let pat_deps = [
        ("tymetafuncspec-core", Path::new(".")),
        ("pattern-tmf", Path::new(".")),
        ("file-tmf", Path::new(".")),
    ];

    let crates = vec![
        traits_crate(&arena, &root_cgd, "fib", &fib, &fib_deps),
        ds_crate(
            &arena,
            &root_cgd,
            "fib-ds",
            &fib_autobox,
            arena.alloc((
                reflexive_sublang(&fib_autobox),
                (fib_autobox.sublang(&fib).unwrap(), ()),
            )),
            &fib_deps,
        ),
        Crate {
            id: tree_identifier::Identifier::from_kebab_str("fib-parse")
                .unwrap()
                .into(),
            provides: vec![parse_gen::targets::default(
                &arena,
                root_cgd.subtree(),
                &fib,
                (),
            )],
            global_workspace_deps: fib_deps.to_vec(),
        },
        traits_crate(&arena, &root_cgd, "fib-pat", &fib_pat, &pat_deps),
        ds_crate(
            &arena,
            &root_cgd,
            "fib-pat-ds",
            &fib_pat_autobox,
            arena.alloc((
                reflexive_sublang(&fib_pat_autobox),
                (
                    fib_pat_autobox.sublang(&fib_pat).unwrap(),
                    (fib_pat_autobox.sublang(&fib).unwrap(), ()),
                ),
            )),
            &pat_deps,
        ),
        Crate {
            id: tree_identifier::Identifier::from_kebab_str("fib-pat-parse")
                .unwrap()
                .into(),
            provides: vec![parse_gen::targets::default(
                &arena,
                root_cgd.subtree(),
                &fib_pat,
                (fib_pat.sublang(&fib).unwrap(), ()),
            )],
            global_workspace_deps: pat_deps.to_vec(),
        },
    ];

    // Create a temporary directory for generation (don't write to disk in benchmark)
    let temp_dir = std::env::temp_dir().join("benchmark_output");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let mut c2sp = Component2SynPath::default();
    let mut total_components = 0;

    for c in crates.iter() {
        total_components += c.provides.len();
        // Generate in memory only for benchmarking - don't write to disk
        c.generate(&temp_dir, &mut c2sp, crates.as_slice());
    }

    // Clean up temp directory
    std::fs::remove_dir_all(&temp_dir).ok();

    total_components
}

fn traits_crate<'arena: 'b, 'b>(
    arena: &'arena bumpalo::Bump,
    root_cgd: &CgDepList<'b>,
    id: &str,
    l: &'b impl LangSpec,
    global_workspace_deps: &[(&'static str, &'static Path)],
) -> Crate<'b> {
    Crate {
        id: tree_identifier::Identifier::from_kebab_str(id)
            .unwrap()
            .into(),
        provides: vec![
            words::targets::words_mod(arena, root_cgd.subtree(), l),
            term_trait_gen::targets::default(arena, root_cgd.subtree(), l),
            parse_gen::targets::parsells(arena, root_cgd.subtree(), l),
            term_pattern_match_strategy_provider_gen::targets::default(
                arena,
                root_cgd.subtree(),
                l,
            ),
            names_langspec_sort_gen::targets::default(arena, root_cgd.subtree(), l),
        ],
        global_workspace_deps: global_workspace_deps.to_vec(),
    }
}

fn ds_crate<'arena, L, Sl>(
    arena: &'arena bumpalo::Bump,
    root_cgd: &CgDepList<'arena>,
    id: &str,
    l: &'arena L,
    sublangs: &'arena Sl,
    global_workspace_deps: &[(&'static str, &'static Path)],
) -> Crate<'arena>
where
    L: LangSpec,
    Sl: SublangsList<'arena, SortIdOf<L>>,
{
    Crate {
        id: tree_identifier::Identifier::from_kebab_str(id)
            .unwrap()
            .into(),
        provides: vec![
            term_specialized_gen::targets::default(arena, root_cgd.subtree(), l),
            term_bridge_gen::targets::default(arena, root_cgd.subtree(), l, sublangs),
            term_pattern_match_strategy_provider_impl_gen::targets::default(
                arena,
                root_cgd.subtree(),
                l,
            ),
        ],
        global_workspace_deps: global_workspace_deps.to_vec(),
    }
}

fn benchmark_code_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("code_generation");

    // Configure for fast but reliable measurement
    group.sample_size(10); // Fewer samples since generation is expensive
    group.measurement_time(std::time::Duration::from_secs(20)); // 20 second measurement window

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            let components = run_code_generation();
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
