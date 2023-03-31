use std::sync::Arc;

use editor::Editor;
use gpui::{
    actions, elements::*, CursorStyle, Entity, MouseButton, MutableAppContext, RenderContext, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use settings::Settings;
use theme;
use workspace::{
    item::{Item, ItemHandle},
    StatusItemView, Workspace,
};

actions!(assistant, [DeployAssistant]);

pub struct Assistant {
    composer: ViewHandle<Editor>,
    message_list: ListState,
    list_items: Vec<ListItem>,
}

enum Role {
    Assistant,
    User,
}

enum ListItem {
    Header(Role),
    Message { role: Role, content: String },
    CodeMessage { role: Role, content: String },
}

pub struct AssistantButton {
    workspace: WeakViewHandle<Workspace>,
    active: bool,
}

actions!(assistant, [SendMessage]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(AssistantButton::deploy_assistant);
    cx.add_action(Assistant::send_message);
}

impl Assistant {
    pub fn new(cx: &mut ViewContext<Self>) -> Self {
        let request_message_1 = r#"
How can I add another message to this list in Rust?
"#
        .trim_start()
        .trim_end();
        let request_message_2 = r#"
let messages = vec![
    Message {
        text: "So... what do you think of this code.".to_owned(),
        from_assistant: false,
    },
    Message {
        text: "It's okay, but honestly your job is kind of at risk.".to_owned(),
        from_assistant: true,
    },
];
"#
        .trim_start()
        .trim_end();

        let response_message_1 = r#"
You can add another message to the `messages` vector by creating a new `Message` struct and pushing it onto the vector using the `push` method. Here's an example:
"#.trim_start().trim_end();

        let response_message_2 = r#"
let new_message = Message {
    text: "I think we can improve this code by using Rust macros.".to_owned(),
    from_assistant: true,
};

messages.push(new_message);
"#
        .trim_start()
        .trim_end();

        let response_message_3 = r#"
This creates a new `Message` struct with the text "I think we can improve this code by using Rust macros." and sets the `from_assistant` field to `true`. Then, it uses the `push` method to add the new message to the end of the `messages` vector.
"#.trim_start().trim_end();

        let list_items = vec![
            ListItem::Header(Role::User),
            ListItem::Message {
                content: request_message_1.to_owned(),
                role: Role::User,
            },
            ListItem::CodeMessage {
                content: request_message_2.to_owned(),
                role: Role::User,
            },
            ListItem::Message {
                content: "Thank you!".to_owned(),
                role: Role::User,
            },
            ListItem::Header(Role::Assistant),
            ListItem::Message {
                content: response_message_1.to_owned(),
                role: Role::Assistant,
            },
            ListItem::CodeMessage {
                content: response_message_2.to_owned(),
                role: Role::Assistant,
            },
            ListItem::Message {
                content: response_message_3.to_owned(),
                role: Role::Assistant,
            },
        ];

        let composer = cx.add_view(|cx| {
            let mut editor = Editor::auto_height(
                10,
                Some(Arc::new(move |theme| {
                    theme.assistant.composer.editor.clone()
                })),
                cx,
            );
            editor.set_placeholder_text("Send a message...", cx);
            editor
        });

        Self {
            composer,
            message_list: ListState::new(
                list_items.len(),
                Orientation::Bottom,
                512.,
                cx,
                |this, ix, cx| {
                    let list_item = &this.list_items[ix];
                    let theme = &cx.global::<Settings>().theme.assistant;

                    match list_item {
                        ListItem::Header(role) => {
                            let style;
                            let label;
                            match role {
                                Role::Assistant => {
                                    style = &theme.assistant_message;
                                    label = "Assistant"
                                }
                                Role::User => {
                                    style = &theme.player_message;
                                    label = "You"
                                }
                            }

                            Text::new(label, style.header.text.clone())
                                .contained()
                                .with_style(style.header.container)
                                .boxed()
                        }
                        ListItem::Message { role, content } => {
                            let style = match role {
                                Role::Assistant => &theme.assistant_message,
                                Role::User => &theme.player_message,
                            };

                            Text::new(content.to_owned(), style.prose_message.text.clone())
                                .contained()
                                .with_style(style.prose_message.container)
                                .boxed()
                        }
                        ListItem::CodeMessage { role, content } => {
                            let style = match role {
                                Role::Assistant => &theme.assistant_message,
                                Role::User => &theme.player_message,
                            };

                            Text::new(content.to_owned(), style.code_message.text.clone())
                                .contained()
                                .with_style(style.code_message.container)
                                .boxed()
                        }
                    }
                },
            ),
            list_items,
        }
    }

    fn send_message(&mut self, _: &SendMessage, cx: &mut ViewContext<Self>) {
        let old_len = self.list_items.len();
        let text = self.composer.update(cx, |composer, cx| {
            let text = composer.text(cx);
            composer.clear(cx);
            text
        });
        self.list_items.push(ListItem::Header(Role::User));
        self.list_items.push(ListItem::Message {
            content: text.clone(),
            role: Role::User,
        });
        let mut reply = "You said: ".to_owned();
        reply.push_str(&text);
        self.list_items.push(ListItem::Header(Role::Assistant));
        self.list_items.push(ListItem::Message {
            content: reply,
            role: Role::Assistant,
        });

        self.message_list.splice(old_len..old_len, 2);
        cx.notify();
    }
}

impl Entity for Assistant {
    type Event = ();
}

impl View for Assistant {
    fn ui_name() -> &'static str {
        "Assistant"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let style = &cx.global::<Settings>().theme.assistant;

        Flex::column()
            .with_child(List::new(self.message_list.clone()).flex(1., true).boxed())
            .with_child(
                ChildView::new(&self.composer, cx)
                    .contained()
                    .with_style(style.composer.editor.container)
                    .contained()
                    .with_style(style.composer.container)
                    .boxed(),
            )
            .with_child(
                Flex::row()
                    .with_child(Empty::new().flex(1., true).boxed())
                    .with_child(
                        Text::new(
                            "⌘⏎ to send message",
                            style.composer.footer_label.text.clone(),
                        )
                        .contained()
                        .with_style(style.composer.footer_label.container)
                        .boxed(),
                    )
                    .boxed(),
            )
            .contained()
            .with_style(style.surface)
            .boxed()
    }

    fn focus_in(&mut self, focused: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if focused != self.composer {
            cx.focus(&self.composer);
        }
    }
}

impl Item for Assistant {
    fn tab_content(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &gpui::AppContext,
    ) -> ElementBox {
        Label::new("Assistant", style.label.clone()).boxed()
    }
}

impl AssistantButton {
    pub fn new(workspace: ViewHandle<Workspace>) -> Self {
        Self {
            workspace: workspace.downgrade(),
            active: false,
        }
    }

    fn deploy_assistant(&mut self, _: &DeployAssistant, cx: &mut ViewContext<Self>) {
        if let Some(workspace) = self.workspace.upgrade(cx) {
            workspace.update(cx, |workspace, cx| {
                let assistant = workspace.items_of_type::<Assistant>(cx).next();
                if let Some(assistant) = assistant {
                    workspace.activate_item(&assistant, cx);
                } else {
                    workspace.show_dock(true, cx);
                    let assistant = cx.add_view(|cx| Assistant::new(cx));
                    workspace.add_item_to_dock(Box::new(assistant.clone()), cx);
                }
            })
        }
    }
}

impl Entity for AssistantButton {
    type Event = ();
}

impl View for AssistantButton {
    fn ui_name() -> &'static str {
        "AssistantButton"
    }

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let active = self.active;
        let theme = cx.global::<Settings>().theme.clone();
        Stack::new()
            .with_child(
                MouseEventHandler::<Self>::new(0, cx, |state, _| {
                    let style = &theme
                        .workspace
                        .status_bar
                        .sidebar_buttons
                        .item
                        .style_for(state, active);

                    Svg::new("icons/assistant_12.svg")
                        .with_color(style.icon_color)
                        .constrained()
                        .with_width(style.icon_size)
                        .aligned()
                        .constrained()
                        .with_width(style.icon_size)
                        .with_height(style.icon_size)
                        .contained()
                        .with_style(style.container)
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    cx.dispatch_action(DeployAssistant)
                })
                .with_tooltip::<Self, _>(
                    0,
                    "Assistant".into(),
                    Some(Box::new(DeployAssistant)),
                    theme.tooltip.clone(),
                    cx,
                )
                .boxed(),
            )
            .boxed()
    }
}

impl StatusItemView for AssistantButton {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _: &mut gpui::ViewContext<Self>,
    ) {
    }
}
