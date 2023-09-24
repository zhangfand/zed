use std::path::PathBuf;

use anyhow::{anyhow, Context, Result};
use serde_derive::Deserialize;
use smol::{
    fs::{self, File},
    io::AsyncReadExt,
    stream::StreamExt,
};
use util::{http::HttpClient, ResultExt};

use crate::{retain_dir_entries, AssetName};

#[derive(Deserialize, Debug)]
pub struct GithubRelease {
    pub name: String,
    #[serde(rename = "prerelease")]
    pub pre_release: bool,
    pub assets: Vec<GithubReleaseAsset>,
    pub tarball_url: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn latest_github_release(
    repo_name_with_owner: &str,
    pre_release: bool,
    http: &dyn HttpClient,
) -> Result<GithubRelease, anyhow::Error> {
    let mut response = http
        .get(
            &format!("https://api.github.com/repos/{repo_name_with_owner}/releases"),
            Default::default(),
            true,
        )
        .await
        .context("error fetching latest release")?;

    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("error reading latest release")?;

    let releases = match serde_json::from_slice::<Vec<GithubRelease>>(body.as_slice()) {
        Ok(releases) => releases,

        Err(_) => {
            log::error!(
                "Error deserializing Github API response text: {:?}",
                String::from_utf8_lossy(body.as_slice())
            );
            return Err(anyhow!("error deserializing latest release"));
        }
    };

    releases
        .into_iter()
        .find(|release| release.pre_release == pre_release)
        .ok_or(anyhow!("Failed to find a release"))
}

pub struct Synced {
    path: PathBuf,
}

pub struct Init {
    name: &'static str,
    repo: &'static str,
    binary_path_in_asset: Option<&'static str>,
    preview: bool,
    asset_name: AssetName,
}

pub struct WithVersion {
    zed_name: &'static str,
    binary_path_in_asset: Option<&'static str>,
    version: String,
    url: String,
}

pub struct GithubBinary<P> {
    phase: P,
}

impl GithubBinary<()> {
    pub const fn new(
        name: &'static str,
        repo: &'static str,
        asset: AssetName,
        binary_path_in_asset: Option<&'static str>,
    ) -> GithubBinary<Init> {
        GithubBinary {
            phase: Init {
                name,
                repo,
                binary_path_in_asset,
                asset_name: asset,
                preview: false,
            },
        }
    }
}

impl GithubBinary<Init> {
    pub fn preview(mut self) -> Self {
        self.phase.preview = true;
        self
    }

    pub async fn fetch_latest(
        self,
        client: &dyn HttpClient,
        version: impl FnOnce(&GithubRelease) -> &str,
    ) -> Result<GithubBinary<WithVersion>> {
        let release = latest_github_release(self.phase.repo, false, client).await?;

        let version = version(&release).to_string();

        let asset_name = self.phase.asset_name.to_string(&version);

        let asset = release
            .assets
            .into_iter()
            .find(|asset| asset_name == asset.name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", self.phase.asset_name))?;

        Ok(GithubBinary {
            phase: WithVersion {
                version,
                url: asset.browser_download_url,
                zed_name: self.phase.name,
                binary_path_in_asset: self.phase.binary_path_in_asset,
            },
        })
    }

    pub async fn cached(self, container_dir: PathBuf) -> Option<GithubBinary<Synced>> {
        (|| async move {
            let mut last_asset_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_asset_dir = Some(entry.path());
                }
            }
            let asset_dir = last_asset_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let asset_binary = if let Some(binary_path) = self.phase.binary_path_in_asset {
                asset_dir.join(binary_path)
            } else {
                asset_dir.clone()
            };

            if asset_binary.exists() {
                Ok(GithubBinary {
                    phase: Synced { path: asset_binary },
                })
            } else {
                Err(anyhow!(
                    "missing {} binary in directory {:?}",
                    self.phase.name,
                    asset_dir
                ))
            }
        })()
        .await
        .log_err()
    }
}

impl GithubBinary<WithVersion> {
    pub async fn sync_to(
        self,
        container_dir: PathBuf,
        client: &dyn HttpClient,
    ) -> Result<GithubBinary<Synced>> {
        let zip_path = container_dir.join(format!(
            "{}_{}.zip",
            self.phase.zed_name, self.phase.version
        ));
        let version_dir =
            container_dir.join(format!("{}_{}", self.phase.zed_name, self.phase.version));

        let binary_path = if let Some(binary_path) = self.phase.binary_path_in_asset {
            version_dir.join(binary_path)
        } else {
            version_dir.clone()
        };

        if smol::fs::metadata(&binary_path).await.is_err() {
            let mut response = client
                .get(&self.phase.url, Default::default(), true)
                .await
                .context("error downloading release")?;
            let mut file = File::create(&zip_path).await?;
            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

            let unzip_status = smol::process::Command::new("unzip")
                .current_dir(&container_dir)
                .arg(&zip_path)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                Err(anyhow!("failed to unzip ????clangd archive"))?;
            }

            retain_dir_entries(&container_dir, |entry| entry.path() == version_dir).await;
        }

        Ok(GithubBinary {
            phase: Synced { path: binary_path },
        })
    }
}

impl GithubBinary<Synced> {
    pub fn path(self) -> PathBuf {
        self.phase.path
    }
}
