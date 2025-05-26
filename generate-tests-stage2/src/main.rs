use std::path::PathBuf;

use langspec::{flat::LangSpecFlat, langspec::LangSpec as _};

fn main() {
    let fib: LangSpecFlat<tymetafuncspec_core::Core> = langspec_examples::fib().canonical_into();
    // let patterns = pattern_gen::load::<LangSpecFlat<tymetafuncspec_core::Core>>(&patterns_path());
    // println!("{:?}", patterns);
}

fn patterns_path() -> PathBuf {
    let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    manifest_dir.join("patterns")
}
