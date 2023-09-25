use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use binary_manager::{Compression, GithubBinary, Init, ResourceName, WithVersion};
use gpui::{AsyncAppContext, Task};
pub use language::*;
use lsp::{CompletionItemKind, LanguageServerBinary, SymbolKind};
use std::{
    any::Any,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
};

use super::SyncedGithubBinaryExt;

const ELIXIR_LSP: GithubBinary<Init> = GithubBinary::new(
    "elixir-ls",
    "elixir-lsp/elixir-ls",
    ResourceName::Versioned("elixir-ls-{}.zip"),
    Some("language_server.sh"),
);

pub struct ElixirLspAdapter;

#[async_trait]
impl LspFetcher for ElixirLspAdapter {
    fn will_start_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        static DID_SHOW_NOTIFICATION: AtomicBool = AtomicBool::new(false);

        const NOTIFICATION_MESSAGE: &str = "Could not run the elixir language server, `elixir-ls`, because `elixir` was not found.";

        let delegate = delegate.clone();
        Some(cx.spawn(|mut cx| async move {
            let elixir_output = smol::process::Command::new("elixir")
                .args(["--version"])
                .output()
                .await;
            if elixir_output.is_err() {
                if DID_SHOW_NOTIFICATION
                    .compare_exchange(false, true, SeqCst, SeqCst)
                    .is_ok()
                {
                    cx.update(|cx| {
                        delegate.show_notification(NOTIFICATION_MESSAGE, cx);
                    })
                }
                return Err(anyhow!("cannot run elixir-ls"));
            }

            Ok(())
        }))
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let binary = ELIXIR_LSP
            .fetch_latest(delegate.http_client().as_ref(), |release| {
                release
                    .name
                    .strip_prefix("Release ")
                    .context("Elixir-ls release name does not start with prefix")
            })
            .await?;

        Ok(Box::new(binary) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let binary = version
            .downcast::<GithubBinary<WithVersion>>()
            .unwrap()
            .sync_to(
                container_dir,
                delegate.http_client().as_ref(),
                Some(Compression::Zip),
            )
            .await?;

        Ok(binary.with_arguments(&[]))
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(ELIXIR_LSP.cached(container_dir).await?.with_arguments(&[]))
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        Some(ELIXIR_LSP.cached(container_dir).await?.with_arguments(&[]))
    }
}

#[async_trait]
impl LspAdapter for ElixirLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("elixir-ls".into())
    }

    fn short_name(&self) -> &'static str {
        "elixir-ls"
    }

    async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        match completion.kind.zip(completion.detail.as_ref()) {
            Some((_, detail)) if detail.starts_with("(function)") => {
                let text = detail.strip_prefix("(function) ")?;
                let filter_range = 0..text.find('(').unwrap_or(text.len());
                let source = Rope::from(format!("def {text}").as_str());
                let runs = language.highlight_text(&source, 4..4 + text.len());
                return Some(CodeLabel {
                    text: text.to_string(),
                    runs,
                    filter_range,
                });
            }
            Some((_, detail)) if detail.starts_with("(macro)") => {
                let text = detail.strip_prefix("(macro) ")?;
                let filter_range = 0..text.find('(').unwrap_or(text.len());
                let source = Rope::from(format!("defmacro {text}").as_str());
                let runs = language.highlight_text(&source, 9..9 + text.len());
                return Some(CodeLabel {
                    text: text.to_string(),
                    runs,
                    filter_range,
                });
            }
            Some((
                CompletionItemKind::CLASS
                | CompletionItemKind::MODULE
                | CompletionItemKind::INTERFACE
                | CompletionItemKind::STRUCT,
                _,
            )) => {
                let filter_range = 0..completion
                    .label
                    .find(" (")
                    .unwrap_or(completion.label.len());
                let text = &completion.label[filter_range.clone()];
                let source = Rope::from(format!("defmodule {text}").as_str());
                let runs = language.highlight_text(&source, 10..10 + text.len());
                return Some(CodeLabel {
                    text: completion.label.clone(),
                    runs,
                    filter_range,
                });
            }
            _ => {}
        }

        None
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        kind: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let (text, filter_range, display_range) = match kind {
            SymbolKind::METHOD | SymbolKind::FUNCTION => {
                let text = format!("def {}", name);
                let filter_range = 4..4 + name.len();
                let display_range = 0..filter_range.end;
                (text, filter_range, display_range)
            }
            SymbolKind::CLASS | SymbolKind::MODULE | SymbolKind::INTERFACE | SymbolKind::STRUCT => {
                let text = format!("defmodule {}", name);
                let filter_range = 10..10 + name.len();
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
