use std::path::PathBuf;

use codegen_component::{CgDepList, Component2SynPath, Crate};
use langspec::{flat::LangSpecFlat, langspec::TerminalLangSpec};

pub fn main() {
    let fib: LangSpecFlat<_> = LangSpecFlat::canonical_from(&langspec_examples::fib());
    let arena = bumpalo::Bump::new();
    let root_cgd = CgDepList::new();
    let words = Crate {
        id: "fib".into(),
        provides: vec![
            words::targets::words_mod(&arena, root_cgd.subtree(), &fib),
            words::targets::words_impls(&arena, root_cgd.subtree(), &fib),
            term_trait_gen::targets::term_trait(&arena, root_cgd.subtree(), &fib),
        ],
        global_workspace_deps: &["tymetafuncspec-core"],
    };
    let bp = base_path();
    let mut c2sp = Component2SynPath::default();
    words.generate(&bp, &mut c2sp);
}

fn base_path() -> PathBuf {
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    manifest_dir.parent().unwrap().join("generated")
}

// fn main() {
//     let mut cmd = cargo_metadata::MetadataCommand::new();
//     let manifest_path = std::env::current_dir().unwrap();
//     cmd.manifest_path(manifest_path.join("Cargo.toml"));
//     let metadata = cmd.exec().unwrap();
//     let examples: Vec<_> = metadata
//         .packages
//         .iter()
//         .filter(|p| p.manifest_path.starts_with(manifest_path.as_path()))
//         .flat_map(|p| {
//             p.targets
//                 .iter()
//                 .filter(|t| t.kind.contains(&cargo_metadata::TargetKind::Example))
//         })
//         .collect();
//     for example in examples.iter() {
//         let run_example_cmd = std::process::Command::new("cargo")
//             .arg("run")
//             .arg("--example")
//             .arg(&example.name)
//             .current_dir(&manifest_path)
//             .spawn()
//             .expect("Failed to run example");
//         let output = run_example_cmd
//             .wait_with_output()
//             .expect("Failed to wait for example");
//         if !output.status.success() {
//             eprintln!(
//                 "Example {} failed with status: {}",
//                 example.name, output.status
//             );
//             std::process::exit(1);
//         }
//     }
//     println!("All examples ran successfully:");
//     for example in examples {
//         println!("- {}", example.name);
//     }
// }
