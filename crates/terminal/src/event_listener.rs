use alacritty_terminal::event::{Event, EventListener};

///This module translates alacritty_terminal events to their Zed equivalents

///The event loop
#[derive(Clone)]
pub struct ZedTerminalHandle(pub futures::channel::mpsc::UnboundedSender<Event>);

impl EventListener for ZedTerminalHandle {
    fn send_event(&self, event: Event) {
        self.0.unbounded_send(event).ok();
    }
}
