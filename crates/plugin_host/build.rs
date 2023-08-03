fn main() {
    let base = std::path::Path::new("../../plugins");

    let crates = std::fs::read_dir(base).unwrap();
    for dir in crates {
        let path = dir.unwrap().path();
        let name = path.file_name().and_then(|x| x.to_str());
        if !matches!(name, Some("target" | "bin" | ".DS_Store")) {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    _ = std::fs::remove_dir_all(base.join("bin"));
    std::fs::create_dir_all(base.join("bin")).unwrap();

    let crates = std::fs::read_dir(base).unwrap();
    for dir in crates {
        let path = dir.unwrap().path();
        let name = path.file_name().and_then(|x| x.to_str());
        if !path.is_dir() || matches!(name, Some("target" | "bin")) {
            continue;
        }

        let name = path.file_name().and_then(|x| x.to_str()).unwrap();

        let status = std::process::Command::new("cargo")
            .args([
                "build",
                "--target",
                "wasm32-wasi",
                "--manifest-path",
                path.join("Cargo.toml").to_str().unwrap(),
                "--release",
            ])
            .status();
        let path = path.display();
        assert!(matches!(status, Ok(e) if e.success()), "{path}: {status:?}");

        let status = std::process::Command::new("wasm-tools")
            .arg("component")
            .arg("new")
            .arg(format!("../../target/wasm32-wasi/release/{name}.wasm",))
            .arg("-o")
            .arg(format!("../../plugins/bin/{name}.wasm"))
            // .arg("--adapt")
            // .arg("./wasi_snapshot_preview1.wasm")
            .status();
        assert!(matches!(status, Ok(e) if e.success()), "{path}: {status:?}");
    }

    // wasm-tools component new ./target/wasm32-wasi/debug/my-project.wasm -o my-component.wasm --adapt ./wasi_snapshot_preview1.wasm
    // std::process::Command::new("wasm-tools")
    //     .arg("component")
    //     .arg("new")
    //     .arg(format!("./target/wasm32-wasi/{}/json_language.wasm", std::process::env))
    //     .arg("-o")
    //     .arg("my-component.wasm")
    //     .arg("--adapt")
    //     .arg("./wasi_snapshot_preview1.wasm")
    //     .spawn()
    //     .expect("wasm-tools failed to start");
}
