use crate::{
    executor,
    geometry::vector::Vector2F,
    keymap::Keystroke,
    platform::{self, Event},
    Scene,
};
use super::renderer::Renderer;

use postage::oneshot;
use std::{
    sync::Arc,
    any::Any,
    cell::RefCell,
    rc::Rc,
};

pub struct Window(Rc<RefCell<WindowState>>);

struct WindowState {
    id: usize,
    event_callback: Option<Box<dyn FnMut(Event)>>,
    resize_callback: Option<Box<dyn FnMut()>>,
    close_callback: Option<Box<dyn FnOnce()>>,
    synthetic_drag_counter: usize,
    executor: Rc<executor::Foreground>,
    scene_to_render: Option<Scene>,
    renderer: Renderer,
    // command_queue: wgpu::CommandQueue,
    last_fresh_keydown: Option<(Keystroke, String)>,
    // layer: id,
    traffic_light_position: Option<Vector2F>,
}

impl Window {
    pub fn open(
        id: usize,
        options: platform::WindowOptions,
        executor: Rc<executor::Foreground>,
        fonts: Arc<dyn platform::FontSystem>,
    ) -> Self {
        unimplemented!()
    }

    pub fn key_window_id() -> Option<usize> {
        unimplemented!()
    }
}

impl platform::Window for Window {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn on_event(&mut self, callback: Box<dyn FnMut(Event)>) {
        self.0.as_ref().borrow_mut().event_callback = Some(callback);
    }

    fn on_resize(&mut self, callback: Box<dyn FnMut()>) {
        self.0.as_ref().borrow_mut().resize_callback = Some(callback);
    }

    fn on_close(&mut self, callback: Box<dyn FnOnce()>) {
        self.0.as_ref().borrow_mut().close_callback = Some(callback);
    }

    fn prompt(
        &self,
        level: platform::PromptLevel,
        msg: &str,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        unimplemented!()
    }

    fn activate(&self) {
        unimplemented!()
    }
}

impl platform::WindowContext for Window {
    fn size(&self) -> Vector2F {
        unimplemented!()
    }

    fn scale_factor(&self) -> f32 {
        unimplemented!()
    }

    fn present_scene(&mut self, scene: Scene) {
        unimplemented!()
    }

    fn titlebar_height(&self) -> f32 {
        unimplemented!()
    }
}

