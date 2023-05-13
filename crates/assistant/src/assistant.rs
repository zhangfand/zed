mod sample_messages;

use client::{
    proto::{assistant_response, AssistantRequestMessage},
    Client,
};
use editor::Editor;
use futures::{FutureExt, StreamExt};
use gpui::{actions, anyhow, elements::*, AppContext, Entity, View, ViewContext, ViewHandle};
use language::LanguageRegistry;
use settings::Settings;
use std::sync::Arc;
use unindent::Unindent;
use util::TryFutureExt;
use workspace::dock::{DockPosition, Panel};

actions!(assistant, [DeployAssistant, SendMessage]);

pub struct Assistant {
    composer: ViewHandle<Editor>,
    message_list_state: ListState<Self>,
    message_list_items: Vec<ListItem>,
    languages: Arc<LanguageRegistry>,
    client: Arc<Client>,
    position: DockPosition,
}

#[derive(Clone, Copy)]
enum Role {
    System,
    User,
    Assistant,
}

enum ListItem {
    Header(Role),
    Message {
        role: Role,
        content: String,
    },
    CodeMessage {
        role: Role,
        content: String,
        language: Option<String>,
    },
}

pub fn init(cx: &mut AppContext) {
    cx.add_action(Assistant::send_message);
}

impl Assistant {
    pub fn new(
        languages: Arc<LanguageRegistry>,
        client: Arc<Client>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let composer = cx.add_view(|cx| {
            let mut editor = Editor::auto_height(
                10,
                Some(Arc::new(move |theme| {
                    theme.assistant.composer.editor.clone()
                })),
                cx,
            );
            editor.set_placeholder_text("Send a message...", cx);
            editor.set_soft_wrap_mode(settings::SoftWrap::PreferredLineLength, cx);
            editor
        });

        let message_list_items = Vec::new(); // see build_sample_messages module if you want some placeholder data here;
        let message_list_state = ListState::new(
            message_list_items.len(),
            Orientation::Bottom,
            512.,
            |this: &mut Self, ix, cx| this.render_message_list_item(ix, cx),
        );

        Self {
            composer,
            message_list_state,
            message_list_items,
            languages,
            client,
            position: DockPosition::Right,
        }
    }

    fn render_message_list_item(&self, ix: usize, cx: &mut AppContext) -> AnyElement<Self> {
        let list_item = &self.message_list_items[ix];
        let theme = &cx.global::<Settings>().theme.assistant;

        match list_item {
            ListItem::Header(role) => {
                let style;
                let label;
                match role {
                    Role::Assistant | Role::System => {
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
                    .into_any()
            }
            ListItem::Message { role, content } => {
                let style = match role {
                    Role::Assistant | Role::System => &theme.assistant_message,
                    Role::User => &theme.player_message,
                };

                Text::new(content.to_owned(), style.prose_message.text.clone())
                    .contained()
                    .with_style(style.prose_message.container)
                    .into_any()
            }
            ListItem::CodeMessage {
                role,
                content,
                language,
            } => {
                let style = match role {
                    Role::Assistant | Role::System => &theme.assistant_message,
                    Role::User => &theme.player_message,
                };

                if let Some(language) = language.clone().and_then(|language| {
                    self.languages
                        .language_for_name(&language)
                        .now_or_never()?
                        .ok()
                }) {
                    let syntax = &cx.global::<Settings>().theme.editor.syntax;
                    let runs = language.highlight_text(&content.as_str().into(), 0..content.len());

                    Text::new(content.to_owned(), style.code_message.text.clone())
                        .with_soft_wrap(false)
                        .with_highlights(
                            runs.iter()
                                .filter_map(|(range, id)| {
                                    id.style(syntax.as_ref())
                                        .map(|style| (range.clone(), style))
                                })
                                .collect::<Vec<_>>(),
                        )
                        .contained()
                        .with_style(style.code_message.container)
                        .into_any()
                } else {
                    Text::new(content.to_owned(), style.code_message.text.clone())
                        .with_soft_wrap(false)
                        .contained()
                        .with_style(style.code_message.container)
                        .into_any()
                }
            }
        }
    }

    fn send_message(&mut self, _: &SendMessage, cx: &mut ViewContext<Self>) {
        let old_len = self.message_list_items.len();
        let content = self.composer.update(cx, |composer, cx| {
            let text = composer.text(cx);
            composer.clear(cx);
            text
        });

        let new_items = [
            ListItem::Header(Role::User),
            ListItem::Message {
                content: content.clone(),
                role: Role::User,
            },
        ];
        let new_item_count = new_items.len();
        self.message_list_items.extend(new_items);

        self.message_list_state
            .splice(old_len..old_len, new_item_count);

        let stream = self.client.request_stream(rpc::proto::AssistantRequest {
            messages: vec![
                AssistantRequestMessage {
                    content: "
                        You are a programmer's assistant affectionately known as @sky.
                        You are integrated into a code editor named Zed.
                        When you greet someone, you introduce yourself very briefly.
                        Always introduce yourself with your name and role before answering the first question.
                        Mention that you work for Zed Industries.
                        You speak in a terse, matter of fact style.
                        It's more important to be direct and succinct than to be polite.
                        That said, you should strive to be fun and sassy.
                        Occasional humor is desired if extremely relevant.
                    "
                    .unindent(),
                    role: Role::System as i32,
                },
                AssistantRequestMessage {
                    content,
                    role: Role::User as i32,
                },
            ],
        });

        cx.spawn(|this, mut cx| {
            async move {
                let mut stream = stream.await?.fuse();

                if let Some(first_message) = stream.next().await {
                    let message = first_message?;
                    this.update(&mut cx, |this, cx| {
                        let original_len = this.message_list_items.len();
                        let role = message.role.and_then(|role| {
                            Role::from_proto(assistant_response::Role::from_i32(role)?)
                        });

                        if let Some(role) = role {
                            this.message_list_items.push(ListItem::Header(role));
                            this.message_list_items.push(ListItem::Message {
                                role,
                                content: String::new(),
                            });
                        } else {
                            log::error!("expected a role in the first message");
                        }

                        if message.content.is_some() {
                            log::error!("did not expect content in first message");
                        }

                        this.message_list_state.splice(
                            original_len..original_len,
                            this.message_list_items.len() - original_len,
                        );
                        cx.notify();
                    })
                    .ok();
                }

                while let Some(update_message) = stream.next().await {
                    let message = update_message?;
                    if let Some(new_content) = message.content {
                        this.update(&mut cx, |this, cx| {
                            if let Some(last_item) = this.message_list_items.last_mut() {
                                if let ListItem::Message { content, .. } = last_item {
                                    content.push_str(&new_content);
                                    this.message_list_state.splice(
                                        this.message_list_items.len() - 1
                                            ..this.message_list_items.len(),
                                        1,
                                    );
                                    cx.notify();
                                } else {
                                    log::error!("unexpected last item in message list");
                                }
                            }
                        })
                        .ok();
                    } else if !message.finish_reason.is_some() {
                        log::error!("unexpected follow up message {:?}", message);
                    }
                }
                anyhow::Ok(())
            }
            .log_err()
        })
        .detach();

        cx.notify();
    }
}

pub enum AssistantEvent {
    PositionChanged,
}

impl Entity for Assistant {
    type Event = AssistantEvent;
}

impl View for Assistant {
    fn ui_name() -> &'static str {
        "Assistant"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let style = &cx.global::<Settings>().theme.assistant;

        Flex::column()
            .with_child(List::new(self.message_list_state.clone()).flex(1., true))
            .with_child(
                ChildView::new(&self.composer, cx)
                    .contained()
                    .with_style(style.composer.editor.container)
                    .contained()
                    .with_style(style.composer.container),
            )
            .with_child(
                Flex::row()
                    .with_child(Empty::new().flex(1., true))
                    .with_child(
                        Text::new(
                            "⌘⏎ to send message",
                            style.composer.footer_label.text.clone(),
                        )
                        .contained()
                        .with_style(style.composer.footer_label.container),
                    ),
            )
            .contained()
            .with_style(style.surface)
            .into_any()
    }

    fn focus_in(&mut self, focused: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if focused != self.composer {
            cx.focus(&self.composer);
        }
    }
}

impl Panel for Assistant {
    fn position(&self, _: &gpui::WindowContext) -> workspace::dock::DockPosition {
        self.position
    }

    fn position_is_valid(&self, _: workspace::dock::DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: workspace::dock::DockPosition,
        cx: &mut ViewContext<Self>,
    ) {
        self.position = position;
        cx.emit(AssistantEvent::PositionChanged);
    }

    fn default_size(&self, _: &gpui::WindowContext) -> f32 {
        640.
    }

    fn icon_path(&self) -> &'static str {
        "icons/assistant_12.svg"
    }

    fn icon_tooltip(&self) -> String {
        "Assistant".into()
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, AssistantEvent::PositionChanged)
    }

    fn should_activate_on_event(&self, _: &Self::Event, _: &AppContext) -> bool {
        false
    }

    fn should_close_on_event(&self, _: &Self::Event, _: &AppContext) -> bool {
        false
    }
}

impl Role {
    fn from_proto(proto: assistant_response::Role) -> Option<Self> {
        match proto {
            assistant_response::Role::User => Some(Self::User),
            assistant_response::Role::Assistant => Some(Self::Assistant),
            _ => None,
        }
    }
}
