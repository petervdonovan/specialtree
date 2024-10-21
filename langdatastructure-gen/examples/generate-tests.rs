use langdatastructure_gen::refbased::formatted;

// write formatted_fib to a file in the tests directory under cargo manifest dir
pub fn main() {
    let formatted = formatted(&langspec_examples::fib());
    let target_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let target_dir = std::path::Path::new(&target_dir);
    let target_dir = target_dir.join("tests");
    std::fs::create_dir_all(&target_dir).unwrap();
    let target_file = target_dir.join("fib.rs");
    std::fs::write(target_file, formatted).unwrap();
}
