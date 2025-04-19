fn main() {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    let manifest_path = std::env::current_dir().unwrap();
    cmd.manifest_path(manifest_path.join("Cargo.toml"));
    let metadata = cmd.exec().unwrap();
    let examples: Vec<_> = metadata
        .packages
        .iter()
        .filter(|p| p.manifest_path.starts_with(manifest_path.as_path()))
        .flat_map(|p| {
            p.targets
                .iter()
                .filter(|t| t.kind.contains(&cargo_metadata::TargetKind::Example))
        })
        .collect();
    for example in examples.iter() {
        let run_example_cmd = std::process::Command::new("cargo")
            .arg("run")
            .arg("--example")
            .arg(&example.name)
            .current_dir(&manifest_path)
            .spawn()
            .expect("Failed to run example");
        let output = run_example_cmd
            .wait_with_output()
            .expect("Failed to wait for example");
        if !output.status.success() {
            eprintln!(
                "Example {} failed with status: {}",
                example.name, output.status
            );
            std::process::exit(1);
        }
    }
    println!("All examples ran successfully:");
    for example in examples {
        println!("- {}", example.name);
    }
}
