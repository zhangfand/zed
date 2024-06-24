use crate::Handlers;
use anyhow::{anyhow, Result};
use fs::{Fs, RealFs};
use futures::channel::mpsc::UnboundedSender;
use gpui::{AsyncAppContext, Model};
use rpc::proto::{self, Envelope, EnvelopedMessage as _};
use smol::stream::StreamExt;
use std::{
    path::Path,
    sync::{atomic::AtomicUsize, Arc},
    time::UNIX_EPOCH,
};
use text::LineEnding;
use worktree::Worktree;

pub struct HeadlessProject {
    pub fs: Arc<RealFs>,
    pub sender: UnboundedSender<Envelope>,
    pub worktrees: Vec<Model<Worktree>>,
    pub next_entry_id: Arc<AtomicUsize>,
}

impl HeadlessProject {
    pub fn init(handlers: &mut Handlers) {
        handlers
            .add(Self::ping)
            .add(Self::write_file)
            .add(Self::stat)
            .add(Self::canonicalize)
            .add(Self::read_link)
            .add(Self::read_dir)
            .add(Self::read_file)
            .add(Self::add_worktree);
    }

    pub fn new(outgoing_tx: UnboundedSender<Envelope>) -> Self {
        HeadlessProject {
            sender: outgoing_tx,
            fs: Arc::new(RealFs::new(Default::default(), None)),
            worktrees: Vec::new(),
            next_entry_id: Default::default(),
        }
    }

    async fn add_worktree(
        this: Model<Self>,
        message: proto::AddWorktree,
        mut cx: AsyncAppContext,
    ) -> Result<proto::AddWorktreeResponse> {
        let worktree = this
            .update(&mut cx.clone(), |this, _| {
                Worktree::local(
                    Path::new(&message.path),
                    true,
                    this.fs.clone(),
                    this.next_entry_id.clone(),
                    &mut cx,
                )
            })?
            .await?;

        this.update(&mut cx, |this, cx| {
            let sender = this.sender.clone();
            this.worktrees.push(worktree.clone());
            worktree.update(cx, |worktree, cx| {
                worktree.observe_updates(0, cx, move |update| {
                    sender
                        .unbounded_send(update.into_envelope(0, None, None))
                        .ok();
                    futures::future::ready(true)
                });
                proto::AddWorktreeResponse {
                    worktree_id: worktree.id().to_proto(),
                }
            })
        })
    }

    async fn ping(
        _: Model<HeadlessProject>,
        _: proto::Ping,
        _cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        Ok(proto::Ack {})
    }

    async fn read_file(
        this: Model<Self>,
        request: proto::ReadFile,
        cx: AsyncAppContext,
    ) -> Result<proto::ReadFileResponse> {
        let fs = this.read_with(&cx, |state, _| state.fs.clone())?;
        let content = fs.load(Path::new(&request.path)).await?;
        Ok(proto::ReadFileResponse { content })
    }

    async fn read_link(
        this: Model<Self>,
        request: proto::ReadLink,
        cx: AsyncAppContext,
    ) -> Result<proto::PathResponse> {
        let fs = this.read_with(&cx, |state, _| state.fs.clone())?;
        let content = fs.read_link(Path::new(&request.path)).await?;
        Ok(proto::PathResponse {
            path: content.to_string_lossy().to_string(),
        })
    }

    async fn canonicalize(
        this: Model<Self>,
        request: proto::Canonicalize,
        cx: AsyncAppContext,
    ) -> Result<proto::PathResponse> {
        let fs = this.read_with(&cx, |state, _| state.fs.clone())?;
        let content = fs.canonicalize(Path::new(&request.path)).await?;
        Ok(proto::PathResponse {
            path: content.to_string_lossy().to_string(),
        })
    }

    async fn read_dir(
        this: Model<Self>,
        request: proto::ReadDir,
        cx: AsyncAppContext,
    ) -> Result<proto::ReadDirResponse> {
        let fs = this.read_with(&cx, |state, _| state.fs.clone())?;
        let mut stream = fs.read_dir(Path::new(&request.path)).await?;
        let mut paths = Vec::new();
        while let Some(item) = stream.next().await {
            paths.push(item?.to_string_lossy().to_string());
        }
        Ok(proto::ReadDirResponse { paths })
    }

    async fn stat(
        this: Model<Self>,
        request: proto::Stat,
        cx: AsyncAppContext,
    ) -> Result<proto::StatResponse> {
        let fs = this.read_with(&cx, |state, _| state.fs.clone())?;
        let metadata = fs.metadata(Path::new(&request.path)).await?;
        if let Some(metadata) = metadata {
            Ok(proto::StatResponse {
                is_dir: metadata.is_dir,
                is_symlink: metadata.is_symlink,
                mtime: metadata
                    .mtime
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                inode: metadata.inode,
            })
        } else {
            Err(anyhow!("file not found"))
        }
    }

    async fn write_file(
        this: Model<Self>,
        request: proto::WriteFile,
        cx: AsyncAppContext,
    ) -> Result<proto::Ack> {
        let fs = this.read_with(&cx, |state, _| state.fs.clone())?;
        fs.save(
            Path::new(&request.path),
            &request.content.into(),
            if request.line_ending == proto::write_file::LineEnding::Unix as i32 {
                LineEnding::Unix
            } else {
                LineEnding::Windows
            },
        )
        .await?;
        Ok(proto::Ack {})
    }
}
