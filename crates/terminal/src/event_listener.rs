use std::sync::Arc;

use alacritty_terminal::event::{Event, EventListener};
use gpui::ViewContext;
use workspace::Workspace;

///This module translates alacritty_terminal events to their Zed equivalents

///The event loop
#[derive(Clone, Copy)]
pub struct ZedTerminalHandle {}
impl EventListener for ZedTerminalHandle {
    fn send_event(&self, event: Event) {
        match event {
            Event::MouseCursorDirty => dbg!("MouseCursorDirty"),
            Event::Title(t) => dbg!("???"),
            Event::ResetTitle => dbg!("ResetTitle"),
            Event::ClipboardStore(_, _) => dbg!("ClipboardStore"),
            Event::ClipboardLoad(_, _) => dbg!("ClipboardLoad"),
            Event::ColorRequest(_, _) => dbg!("ColorRequest"),
            Event::PtyWrite(_) => dbg!("PtyWrite"),
            Event::CursorBlinkingChange => dbg!("CursorBlinkingChange"),
            Event::Wakeup => dbg!("Wakeup"), //Trigger re-render
            Event::Bell => dbg!("Bell"),
            Event::Exit => dbg!("Exit"),
        };
    }
}
