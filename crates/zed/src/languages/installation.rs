use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::path::Path;

pub struct GitHubLspBinaryVersion {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct NpmInfo {
    #[serde(default)]
    dist_tags: NpmInfoDistTags,
    versions: Vec<String>,
}

#[derive(Deserialize, Default)]
struct NpmInfoDistTags {
    latest: Option<String>,
}

pub async fn npm_package_latest_version(name: &str) -> Result<String> {
    let output = smol::process::Command::new("npm")
        .args(["-fetch-retry-mintimeout", "2000"])
        .args(["-fetch-retry-maxtimeout", "5000"])
        .args(["info", name, "--json"])
        .output()
        .await
        .context("failed to run npm info")?;
    if !output.status.success() {
        Err(anyhow!(
            "failed to execute npm info:\nstdout: {:?}\nstderr: {:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))?;
    }
    let mut info: NpmInfo = serde_json::from_slice(&output.stdout)?;
    info.dist_tags
        .latest
        .or_else(|| info.versions.pop())
        .ok_or_else(|| anyhow!("no version found for npm package {}", name))
}

pub async fn npm_install_packages(
    packages: impl IntoIterator<Item = (&str, &str)>,
    directory: &Path,
) -> Result<()> {
    let output = smol::process::Command::new("npm")
        .args(["-fetch-retry-mintimeout", "2000"])
        .args(["-fetch-retry-maxtimeout", "5000"])
        .arg("install")
        .arg("--prefix")
        .arg(directory)
        .args(
            packages
                .into_iter()
                .map(|(name, version)| format!("{name}@{version}")),
        )
        .output()
        .await
        .context("failed to run npm install")?;
    if !output.status.success() {
        Err(anyhow!(
            "failed to execute npm install:\nstdout: {:?}\nstderr: {:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))?;
    }
    Ok(())
}
