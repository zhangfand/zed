use crate::{
    clipboard::ClipboardItem,
    platform::{self, CursorStyle},
    executor, AnyAction, Menu,
};
use super::{window::Window, Dispatcher, FontSystem};
use anyhow::Result;
use time::UtcOffset;
use postage::oneshot;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

#[derive(Default)]
pub struct WindowsForegroundPlatform;

impl platform::ForegroundPlatform for WindowsForegroundPlatform {
    fn on_become_active(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_resign_active(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_quit(&self, callback: Box<dyn FnMut()>) {
        unimplemented!()
    }

    fn on_event(&self, callback: Box<dyn FnMut(crate::Event) -> bool>) {
        unimplemented!()
    }

    fn on_open_files(&self, callback: Box<dyn FnMut(Vec<PathBuf>)>) {
        unimplemented!()
    }

    fn run(&self, on_finish_launching: Box<dyn FnOnce() -> ()>) {
        unimplemented!()
    }

    fn on_menu_command(&self, callback: Box<dyn FnMut(&dyn AnyAction)>) {
        unimplemented!()
    }

    fn set_menus(&self, menus: Vec<Menu>) {
        unimplemented!()
    }

    fn prompt_for_paths(
        &self,
        options: platform::PathPromptOptions,
    ) -> oneshot::Receiver<Option<Vec<PathBuf>>> {
        unimplemented!()
    }

    fn prompt_for_new_path(&self, directory: &Path) -> oneshot::Receiver<Option<PathBuf>> {
        unimplemented!()
    }
}

pub struct WindowsPlatform {
    dispatcher: Arc<Dispatcher>,
    fonts: Arc<FontSystem>,
}

impl WindowsPlatform {
    pub fn new() -> Self {
        Self {
            dispatcher: Arc::new(Dispatcher),
            fonts: Arc::new(FontSystem::new()),
        }
    }
}

impl platform::Platform for WindowsPlatform {
    fn dispatcher(&self) -> Arc<dyn platform::Dispatcher> {
        self.dispatcher.clone()
    }

    fn activate(&self, ignoring_other_apps: bool) {
        unimplemented!()
    }

    fn open_window(
        &self,
        id: usize,
        options: platform::WindowOptions,
        executor: Rc<executor::Foreground>,
    ) -> Box<dyn platform::Window> {
        Box::new(Window::open(id, options, executor, self.fonts()))
    }

    fn key_window_id(&self) -> Option<usize> {
        Window::key_window_id()
    }

    fn fonts(&self) -> Arc<dyn platform::FontSystem> {
        self.fonts.clone()
    }

    fn quit(&self) {
        unimplemented!()
    }

    fn write_to_clipboard(&self, item: ClipboardItem) {
        unimplemented!()
    }

    fn read_from_clipboard(&self) -> Option<ClipboardItem> {
        unimplemented!()
    }

    fn open_url(&self, url: &str) {
        unimplemented!()
    }

    fn write_credentials(&self, url: &str, username: &str, password: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn read_credentials(&self, url: &str) -> Result<Option<(String, Vec<u8>)>> {
        unimplemented!()
    }

    fn delete_credentials(&self, url: &str) -> Result<()> {
        unimplemented!()
    }

    fn set_cursor_style(&self, style: CursorStyle) {
        unimplemented!()
    }

    fn local_timezone(&self) -> UtcOffset {
        unimplemented!()
    }

    fn path_for_resource(&self, name: Option<&str>, extension: Option<&str>) -> Result<PathBuf> {
        unimplemented!()
    }
}
