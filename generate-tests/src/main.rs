use std::path::{Path, PathBuf};

use codegen_component::{CgDepList, Component2SynPath, Crate};
use langspec::{
    flat::LangSpecFlat,
    langspec::{LangSpec, TerminalLangSpec},
};
use parse_gen::cst;

pub fn main() {
    let arena = bumpalo::Bump::new();
    let fib = LangSpecFlat::canonical_from(&langspec_examples::fib());
    let fib_cst = cst(&arena, &fib);
    let fib_pat = extension_pattern::patternfy(&fib);
    // let fib_pat_cst = cst(&arena, &fib_pat);
    let root_cgd = CgDepList::new();
    let fib_deps = [("tymetafuncspec-core", Path::new("."))];
    let pat_deps = [
        ("tymetafuncspec-core", Path::new(".")),
        ("pattern-tmf", Path::new(".")),
    ];
    let crates = vec![
        traits_crate(&arena, &root_cgd, "fib", &fib, &fib_deps),
        ds_crate(&arena, &root_cgd, "fib-ds", &fib, &fib_deps),
        Crate {
            id: "fib-parse".into(),
            provides: vec![
                parse_gen::targets::default(&arena, root_cgd.subtree(), &fib),
                // has_own_sort_id_gen::targets::default(&arena, root_cgd.subtree(), fib_cst),
            ],
            global_workspace_deps: fib_deps.to_vec(),
        },
        // traits_crate(&arena, &root_cgd, "fib-pat", &fib_pat, &pat_deps),
        // ds_crate(&arena, &root_cgd, "fib-pat-ds", &fib_pat, &pat_deps),
        // Crate {
        //     id: "fib-pat-parse".into(),
        //     provides: vec![parse_gen::targets::default(
        //         &arena,
        //         root_cgd.subtree(),
        //         &fib_pat,
        //     )],
        //     global_workspace_deps: pat_deps.to_vec(),
        // },
    ];
    let bp = base_path();
    let mut c2sp = Component2SynPath::default();
    for c in crates.iter() {
        c.generate(&bp, &mut c2sp, crates.as_slice());
    }
}

fn base_path() -> PathBuf {
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    manifest_dir.parent().unwrap().join("generated")
}

fn traits_crate<'arena: 'b, 'b>(
    arena: &'arena bumpalo::Bump,
    root_cgd: &CgDepList<'b>,
    id: &str,
    l: &'b impl LangSpec,
    global_workspace_deps: &[(&'static str, &'static Path)],
) -> Crate<'b> {
    Crate {
        id: id.into(),
        provides: vec![
            words::targets::words_mod(arena, root_cgd.subtree(), l),
            term_trait_gen::targets::default(arena, root_cgd.subtree(), l),
            parse_gen::targets::parsells(arena, root_cgd.subtree(), l),
            term_pattern_match_strategy_provider_gen::targets::default(
                arena,
                root_cgd.subtree(),
                l,
            ),
        ],
        global_workspace_deps: global_workspace_deps.to_vec(),
    }
}
fn ds_crate<'arena: 'b, 'b>(
    arena: &'arena bumpalo::Bump,
    root_cgd: &CgDepList<'b>,
    id: &str,
    l: &'b impl LangSpec,
    global_workspace_deps: &[(&'static str, &'static Path)],
) -> Crate<'b> {
    Crate {
        id: id.into(),
        provides: vec![
            term_specialized_gen::targets::default(arena, root_cgd.subtree(), l),
            term_specialized_impl_gen::targets::default(arena, root_cgd.subtree(), l),
            term_pattern_match_strategy_provider_impl_gen::targets::words_impls(
                arena,
                root_cgd.subtree(),
                l,
            ),
            term_pattern_match_strategy_provider_impl_gen::targets::default(
                arena,
                root_cgd.subtree(),
                l,
            ),
            has_own_sort_id_gen::targets::default(arena, root_cgd.subtree(), l),
        ],
        global_workspace_deps: global_workspace_deps.to_vec(),
    }
}
