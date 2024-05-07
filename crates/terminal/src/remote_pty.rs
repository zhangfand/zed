use std::{future::Future, io::Write, ops::ControlFlow, os::unix::net::UnixStream};

use alacritty_terminal::{
    event::OnResize,
    tty::{EventedPty, EventedReadWrite},
};
use anyhow::Context;
use futures::{
    channel::mpsc::{self, UnboundedSender},
    AsyncReadExt,
};
use gpui::{AppContext, Task};
use polling::{Event, PollMode, Poller};
use smol::stream::StreamExt;
use util::ResultExt;

pub struct RemotePty {
    reader: UnixStream,
    writer: UnixStream,
    _host_data_writer_task: Task<()>,
    _host_input_task: Task<()>,
}

impl RemotePty {
    pub fn new<F, Fut>(
        send_remotely: F,
        cx: &AppContext,
    ) -> anyhow::Result<(Self, UnboundedSender<Vec<u8>>)>
    where
        F: Fn(Vec<u8>) -> Fut + Send + 'static,
        Fut: Future<Output = ControlFlow<(), ()>> + Send + 'static,
    {
        let (host_tx, mut host_rx) = mpsc::unbounded::<Vec<u8>>();
        // TODO kb this would not work on Windows, and can we have less channels around?
        let (mut sender, reader) =
            UnixStream::pair().context("creating remote pty reader counterpart")?;
        let (receiver, writer) =
            UnixStream::pair().context("creating remote pty writer counterpart")?;

        reader
            .set_nonblocking(true)
            .context("setting remote pty reader to nonblocking")?;

        let host_data_writer_task = cx.background_executor().spawn(async move {
            loop {
                if let Some(data) = host_rx.next().await {
                    if sender.write_all(data.as_ref()).log_err().is_none() {
                        break;
                    }
                } else {
                    break;
                }
            }
        });
        let host_input_task = cx.background_executor().spawn(async move {
            let mut buffer = [0u8; 1024];
            let mut receiver = smol::Unblock::new(receiver);
            loop {
                match receiver.read(&mut buffer).await.log_err() {
                    None | Some(0) => break,
                    Some(bytes_read) => {
                        let input_bytes = buffer[..bytes_read].to_vec();
                        match send_remotely(input_bytes).await {
                            ControlFlow::Continue(()) => {}
                            ControlFlow::Break(()) => break,
                        }
                    }
                }
            }
        });

        let remote_pty = Self {
            reader,
            writer,
            _host_data_writer_task: host_data_writer_task,
            _host_input_task: host_input_task,
        };

        Ok((remote_pty, host_tx))
    }
}

impl EventedReadWrite for RemotePty {
    type Reader = UnixStream;
    type Writer = UnixStream;

    unsafe fn register(
        &mut self,
        poll: &std::sync::Arc<Poller>,
        mut interest: Event,
        mode: PollMode,
    ) -> std::io::Result<()> {
        println!("register");
        interest.key = 0; // PTY_READ_WRITE_TOKEN
        poll.add_with_mode(&self.reader, interest, mode)?;
        // if !self.reader.is_empty() {
        //     poll.notify()?;
        // }
        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &std::sync::Arc<Poller>,
        interest: Event,
        mode: PollMode,
    ) -> std::io::Result<()> {
        println!("reregister");
        poll.modify_with_mode(&self.reader, interest, mode)?;
        // if !self.reader.is_empty() {
        //     poll.notify()?;
        // }
        Ok(())
    }

    fn deregister(&mut self, poll: &std::sync::Arc<Poller>) -> std::io::Result<()> {
        println!("deregister");
        poll.delete(&self.reader)?;
        Ok(())
    }

    fn reader(&mut self) -> &mut Self::Reader {
        println!("reader");
        &mut self.reader
    }

    fn writer(&mut self) -> &mut Self::Writer {
        println!("writer");
        &mut self.writer
    }
}

impl EventedPty for RemotePty {
    fn next_child_event(&mut self) -> Option<alacritty_terminal::tty::ChildEvent> {
        None
    }
}

impl OnResize for RemotePty {
    fn on_resize(&mut self, window_size: alacritty_terminal::event::WindowSize) {
        // todo!()
        println!("resize")
    }
}
