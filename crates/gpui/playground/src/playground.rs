#![allow(dead_code, unused_variables)]
use std::env::args_os;

use crate::{element::ParentElement, style::StyleHelpers};
use element::{Element, IntoElement};
use gpui::{
    color::Color,
    fonts::with_font_cache,
    geometry::{pixels, rect::RectF, vector::vec2f},
    platform::WindowOptions,
    ViewContext,
};
use log::LevelFilter;
use playground_macros::Element;
use simplelog::SimpleLogger;
use themes::{current_theme, one, rose_pine, Theme, ThemeColors};
use view::view;

mod adapter;
mod color;
mod components;
mod div;
mod element;
mod hoverable;
mod interactive;
mod layout_context;
mod paint_context;
mod pressable;
mod style;
mod text;
mod themes;
mod view;

mod chat;

fn main() {
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("could not initialize logger");

    gpui::App::new(()).unwrap().run(|cx| {
        cx.add_window(
            WindowOptions {
                bounds: gpui::platform::WindowBounds::Fixed(RectF::new(
                    vec2f(0., 0.),
                    vec2f(400., 300.),
                )),
                center: true,
                ..Default::default()
            },
            |_| {
                view(|cx| {
                    let text = {
                        let fc = cx.font_cache();
                        with_font_cache(fc.clone(), || gpui::fonts::TextStyle {
                            color: Color::white(),
                            ..Default::default()
                        })
                    };

                    if let Some("c") = args_os()
                        .skip(1)
                        .next()
                        .and_then(|s| s.into_string().ok())
                        .as_deref()
                    {
                        cx.push_text_style(text.clone());
                        let result = chat::chat(Theme {
                            colors: one::dark(),
                            text,
                        })
                        .into_any();
                        cx.pop_text_style();
                        result
                    } else {
                        playground(Theme {
                            colors: rose_pine::dawn(),
                            text,
                        })
                        .into_any()
                    }
                })
            },
        );
        cx.platform().activate(true);
    });
}

fn playground<V: 'static>(theme: Theme) -> impl Element<V> {
    workspace().themed(theme)
}

fn workspace<V: 'static>() -> impl Element<V> {
    WorkspaceElement
}

use crate as playground;
#[derive(Element)]
struct WorkspaceElement;

impl WorkspaceElement {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        use div::div;
        let theme = &cx.theme::<Theme>().colors;
        // one line change1!
        div()
            .full()
            .flex()
            .flex_col()
            .fill(theme.base(0.5))
            .child(self.title_bar(cx))
            .child(self.stage(cx))
            .child(self.status_bar(cx))
    }

    fn title_bar<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        use div::div;

        let theme = &current_theme(cx).colors;
        div().h(pixels(cx.titlebar_height())).fill(theme.base(0.))
    }

    fn status_bar<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        use div::div;

        let theme = &current_theme(cx).colors;
        div().h(pixels(cx.titlebar_height())).fill(theme.base(0.))
    }

    fn stage<V: 'static>(&mut self, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        use div::div;

        let theme = &current_theme(cx).colors;
        div().flex_grow()
    }
}
