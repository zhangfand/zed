use alacritty_terminal::{
    config::PtyConfig,
    event_loop::{EventLoop, Notifier},
    tty,
};
use anyhow::bail;
use gpui::Entity;
use terminal_util::TerminalSize;

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
}

impl ZTY {
    fn new(pty_config: PtyConfig, window_id: u64) -> Result<(), TerminalError> {
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
        let event_loop = EventLoop::new(
            term.clone(),
            ZedListener(events_tx.clone()),
            pty,
            pty_config.hold,
            false,
        );

        //Kick things off
        let pty_tx = event_loop.channel();
        let _io_thread = event_loop.spawn();
    }
}
