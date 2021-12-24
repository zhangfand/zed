use gpui::{
    action, color::Color, elements::*, keymap::Binding, ElementBox, Entity, MutableAppContext,
    RenderContext, View, ViewContext,
};

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings(vec![Binding::new(
        "escape",
        CancelFindPanel,
        Some("FindPanel"),
    )]);
    cx.add_action(FindPanel::cancel)
}

action!(CancelFindPanel);

pub enum Event {
    Cancel,
}

pub struct FindPanel {}

impl FindPanel {
    pub fn new() -> Self {
        Self {}
    }

    fn cancel(&mut self, _: &CancelFindPanel, cx: &mut ViewContext<Self>) {
        log::info!("CANCEL FIND PANEL");
        cx.emit(Event::Cancel);
    }
}

impl Entity for FindPanel {
    type Event = Event;
}

impl View for FindPanel {
    fn ui_name() -> &'static str {
        "FindPanel"
    }

    fn render(&mut self, _: &mut RenderContext<'_, Self>) -> ElementBox {
        Empty::new()
            .constrained()
            .with_width(400.)
            .with_height(200.)
            .contained()
            .with_background_color(Color::red())
            .with_margin_right(20.)
            .with_corner_radius(5.)
            .aligned()
            .top()
            .right()
            .named("find panel")
    }
}
