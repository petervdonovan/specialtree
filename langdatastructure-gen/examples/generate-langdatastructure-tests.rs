pub fn main() {
    let target_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = std::path::Path::new(&target_dir);
    let target_dir = target_dir.join("tests");
    std::fs::create_dir_all(&target_dir).unwrap();

    let formatted = langdatastructure_gen::refbased::formatted(&langspec_examples::fib());
    let target_file = target_dir.join("fib_refbased.rs");
    std::fs::write(target_file, formatted).unwrap();

    // let formatted = langdatastructure_gen::idxbased::formatted(&langspec_examples::fib());
    // let target_file = target_dir.join("fib_idxbased.rs");
    // std::fs::write(target_file, formatted).unwrap();
}
