use std::path::Path;

use codegen_component::{CgDepList, Component2SynPath, Crate};
use extension_autobox::autobox;
use langspec::{
    flat::LangSpecFlat,
    langspec::{LangSpec, SortIdOf, TerminalLangSpec},
    sublang::{SublangsList, reflexive_sublang},
};

/// Construct the "trait" crate used by both main and the benchmark.
pub fn traits_crate<'arena: 'b, 'b>(
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

/// Construct the data-structure crate used by both main and the benchmark.
pub fn ds_crate<'arena, L, Sl>(
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

/// Run the full code-generation pipeline, writing outputs to `output_dir`.
/// Returns the number of components generated (to avoid being optimized away).
pub fn run_code_generation(output_dir: &Path) -> usize {
    let arena = bumpalo::Bump::new();
    let fib = LangSpecFlat::canonical_from(&langspec_examples::fib());
    let fib_autobox = autobox(&fib);
    let fib_pat = extension_pattern::patternfy(&arena, &fib);
    let fib_pat_autobox = autobox(&fib_pat);

    let root_cgd = CgDepList::new();
    let fib_deps = [
        ("tymetafuncspec-core", Path::new(".")),
        ("aspect", Path::new(".")),
    ];
    let pat_deps = [
        ("tymetafuncspec-core", Path::new(".")),
        ("aspect", Path::new(".")),
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
                &*arena.alloc(reflexive_sublang(&fib_autobox)),
                (&*arena.alloc(fib_autobox.sublang(&fib).unwrap()), ()),
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
                &*arena.alloc(reflexive_sublang(&fib_pat_autobox)),
                (
                    &*arena.alloc(fib_pat_autobox.sublang(&fib_pat).unwrap()),
                    (&*arena.alloc(fib_pat_autobox.sublang(&fib).unwrap()), ()),
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
                (&*arena.alloc(fib_pat.sublang(&fib).unwrap()), ()),
            )],
            global_workspace_deps: pat_deps.to_vec(),
        },
    ];

    let mut c2sp = Component2SynPath::default();
    let mut total_components = 0;

    for c in crates.iter() {
        total_components += c.provides.len();
        c.generate(output_dir, &mut c2sp, crates.as_slice());
    }

    total_components
}
