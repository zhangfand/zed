use std::sync::Arc;

use gpui::{anyhow::Result, AppContext, ModelHandle, Task};
use language::{Buffer, Diff};
use node_runtime::NodeRuntime;
use util::channel::ReleaseChannel;

pub struct Prettier;

impl Prettier {
    pub const CONFIG_FILE_NAMES: &'static [&'static str] = &[
        ".prettierrc",
        ".prettierrc.json",
        ".prettierrc.json5",
        ".prettierrc.yaml",
        ".prettierrc.yml",
        ".prettierrc.toml",
        ".prettierrc.js",
        ".prettierrc.cjs",
        "package.json",
        "prettier.config.js",
        "prettier.config.cjs",
        ".editorconfig",
    ];

    pub async fn start_default(node: Arc<NodeRuntime>) -> Result<Self> {
        Self::install(&node).await?;

        todo!()
    }

    pub async fn start(node: Arc<NodeRuntime>) -> Result<Self> {
        Self::install(&node).await?;
        todo!()
    }

    pub async fn install(node: &NodeRuntime) -> Result<()> {
        let prettier_server = if util::channel::RELEASE_CHANNEL == ReleaseChannel::Dev {

        } else {
            ""
        }
        node.npm_install_packages(&util::paths::PRETTIER_DIR, [("prettier_server")])
            .await?;
        Ok(())
    }

    pub fn format(&self, buffer: &ModelHandle<Buffer>, cx: &AppContext) -> Task<Result<Diff>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    #[test]
    fn test() {
        let mut cmd = Command::new("/Users/as-cii/.volta/bin/node")
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .spawn()
            .unwrap();
        let stdin = cmd.stdin.as_mut().unwrap();
        stdin.write_all(Prettier::SERVER.as_bytes()).unwrap();
        cmd.wait().unwrap();
    }
}
