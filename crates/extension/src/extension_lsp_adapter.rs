use anyhow::{anyhow, Result};
use async_trait::async_trait;
use gpui::AppContext;
use language::{CodeLabel, Language, LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use parking_lot::Mutex;
use scripting::ScriptModule;
use serde::Deserialize;
use std::{any::Any, borrow::Cow, path::PathBuf, str, sync::Arc};
use util::github::{latest_github_release, GitHubLspBinaryVersion};

pub struct ExtensionLspAdapter {
    config: ExtensionLspAdapterConfig,
    script: String,
    // script_module: Mutex<Option<ScriptModule>>,
    // node: Arc<dyn NodeRuntime>,
}

#[derive(Deserialize)]
pub struct ExtensionLspAdapterConfig {
    name: Arc<str>,
    short_name: Arc<str>,
    install: ExtensionLspAdapterInstall,
}

#[derive(Deserialize)]
enum ExtensionLspAdapterInstall {
    None,
    GitHubRelease {
        repository: String,
        asset: ExtensionLspAdapterAsset,
    },
    NpmPackage {
        name: String,
    },
}

#[derive(Deserialize)]
enum ExtensionLspAdapterAsset {
    Function(String),
    Name(String),
}

impl ExtensionLspAdapter {
    pub fn new(config: ExtensionLspAdapterConfig, script: String, cx: &mut AppContext) -> Self {
        Self { config, script }
    }
}

#[async_trait]
impl LspAdapter for ExtensionLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.config.name.clone())
    }

    fn short_name(&self) -> &'static str {
        "todo!()"
        // &self.config.short_name
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        return match &self.config.install {
            ExtensionLspAdapterInstall::None => Ok(Box::new(())),
            ExtensionLspAdapterInstall::GitHubRelease { repository, asset } => {
                let release =
                    latest_github_release(repository, true, false, delegate.http_client()).await?;

                match asset {
                    ExtensionLspAdapterAsset::Function(function_name) => {
                        //
                    }
                    ExtensionLspAdapterAsset::Name(asset_name) => {
                        let asset = release
                            .assets
                            .iter()
                            .find(|asset| &asset.name == asset_name)
                            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
                    }
                }

                Ok(Box::new(GitHubLspBinaryVersion {
                    name: release.tag_name,
                    url: "todo!()".to_string(), // url: asset.browser_download_url.clone(),
                }))
            }
            ExtensionLspAdapterInstall::NpmPackage { name } => {
                todo!()

                // let version = self.node.npm_package_latest_version(name).await?;
                // Ok(Box::new(version))
            }
        };
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        match &self.config.install {
            ExtensionLspAdapterInstall::None => {}
            ExtensionLspAdapterInstall::GitHubRelease { repository, asset } => {
                let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
                //
            }
            ExtensionLspAdapterInstall::NpmPackage { name } => todo!(),
        }

        // let destination_path = container_dir.join(format!("rust-analyzer-{}", version.name));

        // if fs::metadata(&destination_path).await.is_err() {
        //     let mut response = delegate
        //         .http_client()
        //         .get(&version.url, Default::default(), true)
        //         .await
        //         .map_err(|err| anyhow!("error downloading release: {}", err))?;
        //     let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
        //     let mut file = File::create(&destination_path).await?;
        //     futures::io::copy(decompressed_bytes, &mut file).await?;
        //     fs::set_permissions(
        //         &destination_path,
        //         <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
        //     )
        //     .await?;

        //     remove_matching(&container_dir, |entry| entry != destination_path).await;
        // }

        // Ok(LanguageServerBinary {
        //     path: destination_path,
        //     arguments: Default::default(),
        // })

        Err(anyhow!("failed"))
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        None

        // get_cached_server_binary(container_dir).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        None

        // get_cached_server_binary(container_dir)
        //     .await
        //     .map(|mut binary| {
        //         binary.arguments = vec!["--help".into()];
        //         binary
        //     })
    }

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        vec!["rustc".into()]
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        Some("rust-analyzer/flycheck".into())
    }

    fn process_diagnostics(&self, params: &mut lsp::PublishDiagnosticsParams) {
        // lazy_static! {
        //     static ref REGEX: Regex = Regex::new("(?m)`([^`]+)\n`$").unwrap();
        // }

        // for diagnostic in &mut params.diagnostics {
        //     for message in diagnostic
        //         .related_information
        //         .iter_mut()
        //         .flatten()
        //         .map(|info| &mut info.message)
        //         .chain([&mut diagnostic.message])
        //     {
        //         if let Cow::Owned(sanitized) = REGEX.replace_all(message, "`$1`") {
        //             *message = sanitized;
        //         }
        //     }
        // }
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        // match completion.kind {
        //     Some(lsp::CompletionItemKind::FIELD) if completion.detail.is_some() => {
        //         let detail = completion.detail.as_ref().unwrap();
        //         let name = &completion.label;
        //         let text = format!("{}: {}", name, detail);
        //         let source = Rope::from(format!("struct S {{ {} }}", text).as_str());
        //         let runs = language.highlight_text(&source, 11..11 + text.len());
        //         return Some(CodeLabel {
        //             text,
        //             runs,
        //             filter_range: 0..name.len(),
        //         });
        //     }
        //     Some(lsp::CompletionItemKind::CONSTANT | lsp::CompletionItemKind::VARIABLE)
        //         if completion.detail.is_some()
        //             && completion.insert_text_format != Some(lsp::InsertTextFormat::SNIPPET) =>
        //     {
        //         let detail = completion.detail.as_ref().unwrap();
        //         let name = &completion.label;
        //         let text = format!("{}: {}", name, detail);
        //         let source = Rope::from(format!("let {} = ();", text).as_str());
        //         let runs = language.highlight_text(&source, 4..4 + text.len());
        //         return Some(CodeLabel {
        //             text,
        //             runs,
        //             filter_range: 0..name.len(),
        //         });
        //     }
        //     Some(lsp::CompletionItemKind::FUNCTION | lsp::CompletionItemKind::METHOD)
        //         if completion.detail.is_some() =>
        //     {
        //         lazy_static! {
        //             static ref REGEX: Regex = Regex::new("\\(…?\\)").unwrap();
        //         }
        //         let detail = completion.detail.as_ref().unwrap();
        //         const FUNCTION_PREFIXES: [&'static str; 2] = ["async fn", "fn"];
        //         let prefix = FUNCTION_PREFIXES
        //             .iter()
        //             .find_map(|prefix| detail.strip_prefix(*prefix).map(|suffix| (prefix, suffix)));
        //         // fn keyword should be followed by opening parenthesis.
        //         if let Some((prefix, suffix)) = prefix {
        //             if suffix.starts_with('(') {
        //                 let text = REGEX.replace(&completion.label, suffix).to_string();
        //                 let source = Rope::from(format!("{prefix} {} {{}}", text).as_str());
        //                 let run_start = prefix.len() + 1;
        //                 let runs =
        //                     language.highlight_text(&source, run_start..run_start + text.len());
        //                 return Some(CodeLabel {
        //                     filter_range: 0..completion.label.find('(').unwrap_or(text.len()),
        //                     text,
        //                     runs,
        //                 });
        //             }
        //         }
        //     }
        //     Some(kind) => {
        //         let highlight_name = match kind {
        //             lsp::CompletionItemKind::STRUCT
        //             | lsp::CompletionItemKind::INTERFACE
        //             | lsp::CompletionItemKind::ENUM => Some("type"),
        //             lsp::CompletionItemKind::ENUM_MEMBER => Some("variant"),
        //             lsp::CompletionItemKind::KEYWORD => Some("keyword"),
        //             lsp::CompletionItemKind::VALUE | lsp::CompletionItemKind::CONSTANT => {
        //                 Some("constant")
        //             }
        //             _ => None,
        //         };
        //         let highlight_id = language.grammar()?.highlight_id_for_name(highlight_name?)?;
        //         let mut label = CodeLabel::plain(completion.label.clone(), None);
        //         label.runs.push((
        //             0..label.text.rfind('(').unwrap_or(label.text.len()),
        //             highlight_id,
        //         ));
        //         return Some(label);
        //     }
        //     _ => {}
        // }
        None
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            lsp::SymbolKind::METHOD | lsp::SymbolKind::FUNCTION => {
                let text = format!("fn {} () {{}}", name);
                let filter_range = 3..3 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::STRUCT => {
                let text = format!("struct {} {{}}", name);
                let filter_range = 7..7 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::ENUM => {
                let text = format!("enum {} {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::INTERFACE => {
                let text = format!("trait {} {{}}", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::CONSTANT => {
                let text = format!("const {}: () = ();", name);
                let filter_range = 6..6 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::MODULE => {
                let text = format!("mod {} {{}}", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            lsp::SymbolKind::TYPE_PARAMETER => {
                let text = format!("type {} {{}}", name);
                let filter_range = 5..5 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            runs: language.highlight_text(&text.as_str().into(), display_range.clone()),
            text: text[display_range].to_string(),
            filter_range,
        })
    }
}
