use anyhow::Result;
use async_trait::async_trait;
use binary_manager::{GithubBinary, Init, ResourceName, WithVersion};
pub use language::*;
use lsp::{LanguageServerBinary, SymbolKind};
use schemars::JsonSchema;
use serde_derive::{Deserialize, Serialize};
use settings::Setting;
use std::{any::Any, ops::Deref, path::PathBuf, sync::Arc};

use super::SyncedGithubBinaryExt;

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct ElixirSettings {
    pub next: ElixirNextSetting,
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ElixirNextSetting {
    Off,
    On,
    Local {
        path: String,
        arguments: Vec<String>,
    },
}

#[derive(Clone, Serialize, Default, Deserialize, JsonSchema)]
pub struct ElixirSettingsContent {
    next: Option<ElixirNextSetting>,
}

impl Setting for ElixirSettings {
    const KEY: Option<&'static str> = Some("elixir");

    type FileContent = ElixirSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &gpui::AppContext,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}

#[cfg(target_arch = "x86_64")]
const NEXT_BINARY: GithubBinary<Init> = GithubBinary::new(
    "next-ls",
    "elixir-tools/next-ls",
    ResourceName::Static("next_ls_darwin_amd64"),
    None,
);

#[cfg(target_arch = "aarch64")]
const NEXT_BINARY: GithubBinary<Init> = GithubBinary::new(
    "next-ls",
    "elixir-tools/next-ls",
    ResourceName::Static("next_ls_darwin_arm64"),
    None,
);

pub struct NextLspAdapter;

#[async_trait]
impl LspFetcher for NextLspAdapter {
    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let binary = NEXT_BINARY
            .fetch_latest(delegate.http_client().as_ref(), |release| Ok(&release.name))
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
            .sync_to(container_dir, delegate.http_client().as_ref(), None)
            .await?;

        Ok(binary.with_arguments(&["--stdio"]))
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(
            NEXT_BINARY
                .cached(container_dir)
                .await?
                .with_arguments(&["--stdio"]),
        )
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        Some(
            NEXT_BINARY
                .cached(container_dir)
                .await?
                .with_arguments(&["--help"]),
        )
    }
}

#[async_trait]
impl LspAdapter for NextLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("next-ls".into())
    }

    fn short_name(&self) -> &'static str {
        "next-ls"
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        _: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        Some(CodeLabel {
            runs: language.highlight_text(&name.into(), 0..name.len()),
            text: name.to_string(),
            filter_range: 0..name.len(),
        })
    }
}

pub struct LocalNextLspAdapter {
    pub path: String,
    pub arguments: Vec<String>,
}

#[async_trait]
impl LspFetcher for LocalNextLspAdapter {
    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(()) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let path = shellexpand::full(&self.path)?;
        Ok(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        let path = shellexpand::full(&self.path).ok()?;
        Some(LanguageServerBinary {
            path: PathBuf::from(path.deref()),
            arguments: self.arguments.iter().map(|arg| arg.into()).collect(),
        })
    }
}

#[async_trait]
impl LspAdapter for LocalNextLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("local-next-ls".into())
    }

    fn short_name(&self) -> &'static str {
        "next-ls"
    }

    async fn label_for_symbol(
        &self,
        name: &str,
        _: SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        Some(CodeLabel {
            runs: language.highlight_text(&name.into(), 0..name.len()),
            text: name.to_string(),
            filter_range: 0..name.len(),
        })
    }
}
