use std::{borrow::Cow, env::consts, path::Path};

use smol::{
    fs::{self, DirEntry},
    stream::StreamExt,
};
use util::ResultExt;

mod github;

pub use github::*;

#[derive(Debug)]
pub enum ResourceName {
    Static(&'static str),
    /// Replaces {} in the asset name with a passed in version
    Versioned(&'static str),
    /// Allows resources with {version} and {arch} to be replaced by a passed in version
    /// and consts::ARCH
    Formatted(&'static str),
}

impl ResourceName {
    pub fn to_string(&self, version: &str) -> Cow<'static, str> {
        match self {
            ResourceName::Static(value) => Cow::Borrowed(value),
            ResourceName::Versioned(value) => Cow::Owned(value.replace("{}", version)),
            ResourceName::Formatted(value) => Cow::Owned(
                value
                    .replace("{version}", version)
                    .replace("{arch}", consts::ARCH),
            ),
        }
    }
}

// Removes all files and directories matching the given predicate
pub async fn retain_dir_entries<F>(dir: &Path, predicate: F)
where
    F: Fn(&DirEntry) -> bool,
{
    if let Some(mut entries) = fs::read_dir(dir).await.log_err() {
        while let Some(entry) = entries.next().await {
            if let Some(entry) = entry.log_err() {
                if !predicate(&entry) {
                    let entry_path = entry.path();
                    if let Ok(metadata) = fs::metadata(&entry_path).await {
                        if metadata.is_file() {
                            fs::remove_file(&entry_path).await.log_err();
                        } else {
                            fs::remove_dir_all(&entry_path).await.log_err();
                        }
                    }
                }
            }
        }
    }
}
