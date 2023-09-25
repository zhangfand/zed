use anyhow::Result;
use async_trait::async_trait;
use binary_manager::{Compression, GithubBinary, Init, WithVersion};
use language::{LanguageServerName, LspAdapterDelegate, LspFetcher};
use lsp::LanguageServerBinary;
use std::{any::Any, path::PathBuf};

use super::SyncedGithubBinaryExt;

// TODO:
// - Figure out compression for next-ls
// - Test:
//   - c
//   - elixir
//   - elixir_next
//   - lua
//   - rust

#[cfg(target_arch = "x86_64")]
const LUA_LSP: GithubBinary<Init> = GithubBinary::new(
    "lua-language-server",
    "LuaLS/lua-language-server",
    binary_manager::ResourceName::Formatted("lua-language-server-{}-darwin-x64.tar.gz"),
    Some("bin/lua-language-server"),
);

#[cfg(target_arch = "aarch64")]
const LUA_LSP: GithubBinary<Init> = GithubBinary::new(
    "lua-language-server",
    "LuaLS/lua-language-server",
    binary_manager::ResourceName::Formatted("lua-language-server-{}-darwin-arm64.tar.gz"),
    Some("bin/lua-language-server"),
);

#[derive(Copy, Clone)]
pub struct LuaLspAdapter;

#[async_trait]
impl LspFetcher for LuaLspAdapter {
    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let binary = LUA_LSP
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
            .sync_to(
                container_dir,
                delegate.http_client().as_ref(),
                Some(Compression::GZip),
            )
            .await?;

        Ok(binary.with_arguments(&[]))
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        Some(LUA_LSP.cached(container_dir).await?.with_arguments(&[]))
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        Some(
            LUA_LSP
                .cached(container_dir)
                .await?
                .with_arguments(&["--version"]),
        )
    }
}

#[async_trait]
impl super::LspAdapter for LuaLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("lua-language-server".into())
    }

    fn short_name(&self) -> &'static str {
        "lua"
    }
}
