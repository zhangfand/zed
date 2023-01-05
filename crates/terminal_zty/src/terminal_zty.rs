use std::{os::fd::AsRawFd, sync::Arc};

use alacritty_terminal::{
    config::PtyConfig,
    event_loop::{EventLoop, Notifier},
    sync::FairMutex,
    tty, Term,
};
use anyhow::{bail, Result};
use gpui::Entity;
use terminal_util::{TerminalError, TerminalSize, ZedListener};

pub enum ZTY {
    Remote(RemoteZTY),
    Local(LocalZTY),
}

impl Entity for ZTY {
    type Event = ();
}

pub struct RemoteZTY {}

pub struct LocalZTY {
    pty_tx: Notifier,
    shell_pid: u32,
    fd: i32,
}

impl ZTY {
    fn new_remote() -> Result<ZTY> {
        bail!("remote terminals are ")
    }

    fn new_local(
        term: Arc<FairMutex<Term<ZedListener>>>,
        working_directory: Option<std::path::PathBuf>,
        shell: settings::Shell,
        pty_config: PtyConfig,
        events_tx: ZedListener,
        window_id: u64,
    ) -> Result<()> {
        let pty = match tty::new(&pty_config, TerminalSize::default().into(), window_id) {
            Ok(pty) => pty,
            Err(error) => {
                bail!(TerminalError {
                    directory: working_directory,
                    shell,
                    source: error,
                });
            }
        };

        let fd = pty.file().as_raw_fd();
        let shell_pid = pty.child().id();

        //And connect them together
        let event_loop = EventLoop::new(term.clone(), events_tx, pty, pty_config.hold, false);

        //Kick things off
        let pty_tx = event_loop.channel();
        let _io_thread = event_loop.spawn();

        Ok(LocalZTY {
            fd,
            shell_pid,
            pty_tx,
        })
    }
}
