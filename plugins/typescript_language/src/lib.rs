use installation::{npm_install_packages, npm_package_latest_version};
use plugin::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Versions {
    typescript_version: String,
    server_version: String,
}

const BIN_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";

#[export]
pub fn name() -> &'static str {
    "typescript-language-server"
}

#[export]
pub fn server_args() -> Vec<String> {
    ["--stdio", "--tsserver-path", "node_modules/typescript/lib"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

#[export]
pub fn fetch_latest_server_version() -> Option<String> {
    serde_json::to_string(&Versions {
        typescript_version: npm_package_latest_version("typescript")?,
        server_version: npm_package_latest_version("typescript-language-server")?,
    })
    .ok()
}

#[export]
pub fn fetch_server_binary(container_dir: PathBuf, versions: &str) -> Result<PathBuf, String> {
    let versions: Versions = serde_json::from_str(versions).unwrap();
    let version_dir = container_dir.join(&format!(
        "typescript-{}:server-{}",
        versions.typescript_version, versions.server_version
    ));
    fs::create_dir_all(&version_dir)
        .map_err(|_| "failed to create version directory".to_string())?;
    let binary_path = version_dir.join(BIN_PATH);

    if fs::metadata(&binary_path).is_err() {
        npm_install_packages(
            &[
                ("typescript", versions.typescript_version.as_str()),
                (
                    "typescript-language-server",
                    &versions.server_version.as_str(),
                ),
            ],
            &version_dir,
        )
        .ok_or_else(|| "failed to install typescript and language server packages")?;

        if let Some(mut entries) = fs::read_dir(&container_dir).ok() {
            while let Some(entry) = entries.next() {
                if let Some(entry) = entry.ok() {
                    let entry_path = entry.path();
                    if entry_path.as_path() != version_dir {
                        fs::remove_dir_all(&entry_path).ok();
                    }
                }
            }
        }
    }

    Ok(binary_path)
}

#[export]
pub fn cached_server_binary(container_dir: PathBuf) -> Option<PathBuf> {
    let mut last_version_dir = None;
    let mut entries = fs::read_dir(&container_dir).ok()?;
    while let Some(entry) = entries.next() {
        let entry = entry.ok()?;
        if entry.file_type().ok()?.is_dir() {
            last_version_dir = Some(entry.path());
        }
    }
    let last_version_dir = last_version_dir?;
    let bin_path = last_version_dir.join(BIN_PATH);
    if bin_path.exists() {
        Some(bin_path)
    } else {
        None
    }
}

#[export]
pub fn label_for_completion(
    item: &lsp::CompletionItem,
    language: &plugin_handles::LanguageHandle,
) -> Option<language::CodeLabel> {
    use lsp::CompletionItemKind as Kind;
    let len = item.label.len();
    let grammar = language.grammar()?;
    let highlight_id = match item.kind? {
        Kind::CLASS | Kind::INTERFACE => grammar.highlight_id_for_name("type"),
        Kind::CONSTRUCTOR => grammar.highlight_id_for_name("type"),
        Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
        Kind::FUNCTION | Kind::METHOD => grammar.highlight_id_for_name("function"),
        Kind::PROPERTY | Kind::FIELD => grammar.highlight_id_for_name("property"),
        _ => None,
    }?;
    Some(language::CodeLabel {
        text: item.label.clone(),
        runs: vec![(0..len, highlight_id)],
        filter_range: 0..len,
    })
}

#[export]
pub fn initialization_options() -> Option<String> {
    Some("{ \"provideFormatter\": true }".to_string())
}
