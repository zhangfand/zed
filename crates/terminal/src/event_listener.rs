use alacritty_terminal::event::{Event, EventListener};

///This module translates alacritty_terminal events to their Zed equivalents

///The event loop
pub struct ZedTranslator {}

impl EventListener for ZedTranslator {
    fn send_event(&self, event: Event) {
        match event {
            Event::MouseCursorDirty => todo!(),
            Event::Title(_) => todo!(),
            Event::ResetTitle => todo!(),
            Event::ClipboardStore(_, _) => todo!(),
            Event::ClipboardLoad(_, _) => todo!(),
            Event::ColorRequest(_, _) => todo!(),
            Event::PtyWrite(_) => todo!(),
            Event::CursorBlinkingChange => todo!(),
            Event::Wakeup => todo!(),
            Event::Bell => todo!(),
            Event::Exit => todo!(),
        }
    }
}
