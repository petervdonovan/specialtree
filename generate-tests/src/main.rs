use std::path::{Path, PathBuf};

use codegen_component::CgDepList;
use extension_autobox::autobox;
use langspec::{
    flat::LangSpecFlat,
    langspec::{LangSpec, SortIdOf, TerminalLangSpec},
    sublang::{SublangsList, reflexive_sublang},
};

use generate_tests::shared::run_code_generation;

pub fn main() {
    // let arena = bumpalo::Bump::new();
    // let fib = LangSpecFlat::canonical_from(&langspec_examples::fib());
    // let fib_autobox = autobox(&fib);
    // // let fib_cst = cst(&arena, &fib);
    // let fib_pat = extension_pattern::patternfy(&arena, &fib);
    // let fib_pat_autobox = autobox(&fib_pat);
    // // let fib_pat_file = extension_file::filefy_all_tmf(&fib_pat, );
    // // let fib_pat_cst = cst(&arena, &fib_pat);
    // let root_cgd = CgDepList::new();
    // let fib_deps = [("tymetafuncspec-core", Path::new("."))];
    // let pat_deps = [
    //     ("tymetafuncspec-core", Path::new(".")),
    //     ("pattern-tmf", Path::new(".")),
    //     ("file-tmf", Path::new(".")),
    // ];
    let bp = base_path();
    generate_tests::shared::run_code_generation(&bp);
}

fn base_path() -> PathBuf {
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    manifest_dir.parent().unwrap().join("generated")
}

// fn ds_crate_no_sublangs<'arena: 'b, 'b>(
//     arena: &'arena bumpalo::Bump,
//     root_cgd: &CgDepList<'b>,
//     id: &str,
//     l: &'b impl LangSpec,
//     global_workspace_deps: &[(&'static str, &'static Path)],
// ) -> Crate<'b> {
//     Crate {
//         id: id.into(),
//         provides: vec![
//             term_specialized_gen::targets::default(arena, root_cgd.subtree(), l),
//             term_specialized_impl_gen::targets::default(
//                 arena,
//                 root_cgd.subtree(),
//                 l,
//                 arena.alloc((reflexive_sublang(l), ())),
//             ),
//             term_pattern_match_strategy_provider_impl_gen::targets::words_impls(
//                 arena,
//                 root_cgd.subtree(),
//                 l,
//             ),
//             term_pattern_match_strategy_provider_impl_gen::targets::default(
//                 arena,
//                 root_cgd.subtree(),
//                 l,
//             ),
//         ],
//         global_workspace_deps: global_workspace_deps.to_vec(),
//     }
// }
