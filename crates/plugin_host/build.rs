fn main() {
    // wasm-tools component new ./target/wasm32-wasi/debug/my-project.wasm -o my-component.wasm --adapt ./wasi_snapshot_preview1.wasm
    std::process::Command::new("wasm-tools")
        .arg("component")
        .arg("new")
        .arg(format!("./target/wasm32-wasi/{}/json_language.wasm", std::process::env))
        .arg("-o")
        .arg("my-component.wasm")
        .arg("--adapt")
        .arg("./wasi_snapshot_preview1.wasm")
        .spawn()
        .expect("wasm-tools failed to start");
}

