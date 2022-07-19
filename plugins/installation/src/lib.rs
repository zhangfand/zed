use plugin::prelude::*;

use std::path::Path;

#[import]
pub fn npm_package_latest_version(name: &str) -> Option<String>;

#[import]
pub fn npm_install_packages(packages: &[(&str, &str)], directory: &Path) -> Option<()>;

// #[import]
// pub(crate) async fn latest_github_release(
//     repo_name_with_owner: &str,
//     http: HttpClientHandle,
// ) -> Result<GithubRelease, anyhow::Error>;
