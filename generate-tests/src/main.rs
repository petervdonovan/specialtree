use std::path::{Path, PathBuf};

use codegen_component::{CgDepList, Component2SynPath, Crate};
use langspec::{flat::LangSpecFlat, langspec::TerminalLangSpec};
use parse_gen::cst;

pub fn main() {
    let fib: LangSpecFlat<_> = LangSpecFlat::canonical_from(&langspec_examples::fib());
    let arena = bumpalo::Bump::new();
    let root_cgd = CgDepList::new();
    let crates = vec![
        Crate {
            id: "fib".into(),
            provides: vec![
                words::targets::words_mod(&arena, root_cgd.subtree(), &fib),
                term_trait_gen::targets::default(&arena, root_cgd.subtree(), &fib),
                parse_gen::targets::parsells(&arena, root_cgd.subtree(), &fib),
                term_pattern_match_strategy_provider_gen::targets::default(
                    &arena,
                    root_cgd.subtree(),
                    &fib,
                ),
            ],
            global_workspace_deps: vec![("tymetafuncspec-core", Path::new("."))],
        },
        Crate {
            id: "fib-pat".into(),
            provides: vec![
            //     pattern_gen::targets::default(
            //     &arena,
            //     root_cgd.subtree(),
            //     &fib,
            //     patterns_path().as_path(),
            // )
            ],
            global_workspace_deps: vec![("tymetafuncspec-core", Path::new("."))],
        },
        Crate {
            id: "fib-ds".into(),
            provides: vec![
                term_specialized_gen::targets::default(&arena, root_cgd.subtree(), &fib),
                term_specialized_impl_gen::targets::default(&arena, root_cgd.subtree(), &fib),
                term_pattern_match_strategy_provider_impl_gen::targets::words_impls(
                    &arena,
                    root_cgd.subtree(),
                    &fib,
                ),
                term_pattern_match_strategy_provider_impl_gen::targets::default(
                    &arena,
                    root_cgd.subtree(),
                    &fib,
                ),
                has_own_sort_id_gen::targets::default(&arena, root_cgd.subtree(), &fib),
            ],
            global_workspace_deps: vec![("tymetafuncspec-core", Path::new("."))],
        },
        Crate {
            id: "fib-parse".into(),
            provides: vec![
                parse_gen::targets::default(&arena, root_cgd.subtree(), &fib),
                has_own_sort_id_gen::targets::default(
                    &arena,
                    root_cgd.subtree(),
                    cst(&arena, &fib),
                ),
            ],
            global_workspace_deps: vec![("tymetafuncspec-core", Path::new("."))],
        },
    ];
    let bp = base_path();
    let mut c2sp = Component2SynPath::default();
    for c in crates.iter() {
        c.generate(&bp, &mut c2sp, crates.as_slice());
    }
}

fn patterns_path() -> PathBuf {
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    manifest_dir.join("patterns")
}

fn base_path() -> PathBuf {
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    manifest_dir.parent().unwrap().join("generated")
}
