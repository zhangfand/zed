use gpui::ViewContext;
use playground_macros::Element;

use super::element::Element;

use crate::{
    div::div,
    element::ParentElement,
    style::{StyleHelpers, Styleable},
    themes::Theme,
};
struct MessageData {
    author: String,
    content: String,
    timestamp: String,
}

pub fn chat<V: 'static>(theme: Theme) -> impl Element<V> {
    let messages = &[
        MessageData {
            author: "Mikayla".to_string(),
            content: "Hello there".to_string(),
            timestamp: "1/2/3".to_string(),
        },
        MessageData {
            author: "Mikayla".to_string(),
            content: "Hello there".to_string(),
            timestamp: "1/2/3".to_string(),
        },
        MessageData {
            author: "Mikayla".to_string(),
            content: "Hello there".to_string(),
            timestamp: "1/2/3".to_string(),
        },
    ];

    div()
        .children(messages.iter().enumerate().map(|(ix, data)| Message {
            author: data.author.clone(),
            content: data.content.clone(),
            timestamp: data.timestamp.clone(),
            ix,
            phantom: std::marker::PhantomData,
        }))
        .full()
        .fill(theme.colors.highlight_low(0.5))
        .themed(theme)
}

use crate as playground;
#[derive(Element)]
struct Message<V: 'static> {
    author: String,
    content: String,
    timestamp: String,
    ix: usize,
    phantom: std::marker::PhantomData<V>,
}

impl<V: 'static> Message<V> {
    fn render(&mut self, view: &mut V, cx: &mut ViewContext<V>) -> impl Element<V> {
        let theme = cx.theme::<Theme>();
        div()
            .child(self.author.clone())
            .child(self.content.clone())
            .child(self.timestamp.clone())
            .fill(if self.ix % 2 == 0 {
                theme.colors.base(0.3)
            } else {
                theme.colors.base(0.7)
            })
            .hoverable()
    }
}
