use gpui::{
    action, elements::*, geometry::vector::Vector2F, keymap::Binding, Entity,
    MutableAppContext, RenderContext, View, ViewContext, ViewHandle,
};
// use postage::watch;
use text::{Point, Selection};
use std::sync::Arc;
use editor::{Editor,EditorSettings,SoftWrap,Event,BuildSettings,};

pub fn init(cx: &mut MutableAppContext) {
    cx.add_bindings(vec![Binding::new(
        "escape",
        CancelFindPanel,
        Some("FindPanel"),
    )]);
    cx.add_action(FindPanel::cancel)
}

action!(CancelFindPanel);

struct RestoreState {
    scroll_position: Vector2F,
    selections: Vec<Selection<usize>>,
}

#[derive(Copy, Clone)]
pub enum Event {
    Cancel,
}
pub struct FindPanel {
    settings: BuildSettings,
    line_editor: ViewHandle<Editor>,
    active_editor: ViewHandle<Editor>,
    restore_state: Option<RestoreState>,
    line_selection: Option<Selection<usize>>,
    cursor_point: Point,
    max_point: Point,
}

impl FindPanel {
    pub fn new(
        active_editor: ViewHandle<Editor>,
        settings: BuildSettings,
        cx: &mut ViewContext<Editor>,
    ) -> Self {
        let line_editor = cx.add_view(|cx| {
            Editor::single_line(
                {
                    let settings = settings.clone();
                    Arc::new(move |_| {
                        let settings = settings.borrow();
                        EditorSettings {
                            tab_size: settings.tab_size,
                            style: settings.theme.selector.input_editor.as_editor(),
                            soft_wrap: SoftWrap::None,
                        }
                    })
                },
                cx,
            )
        });
        cx.subscribe(&line_editor, Self::on_line_editor_event)
            .detach();

        let (restore_state, cursor_point, max_point) = active_editor.update(cx, |editor, cx| {
            let restore_state = Some(RestoreState {
                scroll_position: editor.scroll_position(cx),
                selections: editor.local_selections::<usize>(cx),
            });

            let buffer = editor.buffer().read(cx).read(cx);
            (
                restore_state,
                editor.newest_selection(&buffer).head(),
                buffer.max_point(),
            )
        });

        Self {
            settings: settings.clone(),
            line_editor,
            active_editor,
            restore_state,
            line_selection: None,
            cursor_point,
            max_point,
        }
    }

 
    fn cancel(&mut self, _: &CancelFindPanel, cx: &mut ViewContext<Self>) {
        log::info!("CANCEL FIND PANEL");
        cx.emit(Event::Closed);
    }
    fn on_line_editor_event(
        e: &mut Editor,
        _: ViewHandle<Editor>,
        event: &Event,
        cx: &mut ViewContext<Editor>,
    ) {
        match event {
            Event::Blurred => cx.emit(Event::Closed),
            Event::Edited => {
                println!("edited");
            }
            _ => {}
        }
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
        let theme = &self.settings.borrow().theme.selector;

        Empty::new()
            .constrained()
            .with_width(400.)
            .with_height(200.)
            .contained()
            .with_style(theme.container)
            .with_margin_right(20.)
            .with_corner_radius(5.)
            .aligned()
            .top()
            .right()
            .named("find panel")
    }
}
