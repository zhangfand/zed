pub mod atlas;
pub mod fonts;
pub mod sprite_cache;
mod dispatcher;
mod image_cache;
mod platform;
mod renderer;
mod window;

pub use dispatcher::Dispatcher;
pub use fonts::FontSystem;
use platform::{WindowsPlatform, WindowsForegroundPlatform};
use std::{
    rc::Rc,
    sync::Arc
};

pub(crate) fn platform() -> Arc<dyn super::Platform> {
    Arc::new(WindowsPlatform::new())
}

pub(crate) fn foreground_platform() -> Rc<dyn super::ForegroundPlatform> {
    Rc::new(WindowsForegroundPlatform::default())
}
