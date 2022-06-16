use alacritty_terminal::event::{Event, EventListener};

///This module translates alacritty_terminal events to their Zed equivalents

///The event loop
#[derive(Clone, Copy)]
pub struct ZedTranslator {}

impl EventListener for ZedTranslator {
    fn send_event(&self, event: Event) {
        match event {
            Event::MouseCursorDirty => dbg!("MouseCursorDirty"),
            Event::Title(_) => dbg!("Title"),
            Event::ResetTitle => dbg!("ResetTitle"),
            Event::ClipboardStore(_, _) => dbg!("ClipboardStore"),
            Event::ClipboardLoad(_, _) => dbg!("ClipboardLoad"),
            Event::ColorRequest(_, _) => dbg!("ColorRequest"),
            Event::PtyWrite(_) => dbg!("PtyWrite"),
            Event::CursorBlinkingChange => dbg!("CursorBlinkingChange"),
            Event::Wakeup => dbg!("Wakeup"),
            Event::Bell => dbg!("Bell"),
            Event::Exit => dbg!("Exit"),
        };
    }
}
