mod sample_messages;

use client::{
    proto::{assistant_response, AssistantRequestMessage},
    Client,
};
use editor::Editor;
use futures::{FutureExt, StreamExt};
use gpui::{
    actions, anyhow, elements::*, AppContext, CursorStyle, Entity, MouseButton, MutableAppContext,
    RenderContext, View, ViewContext, ViewHandle, WeakViewHandle,
};
use language::LanguageRegistry;
use settings::Settings;
use std::sync::Arc;
use theme;
use util::TryFutureExt;
use workspace::{
    item::{Item, ItemHandle},
    StatusItemView, Workspace,
};

actions!(assistant, [DeployAssistant, SendMessage]);

pub struct Assistant {
    composer: ViewHandle<Editor>,
    message_list_state: ListState,
    message_list_items: Vec<ListItem>,
    languages: Arc<LanguageRegistry>,
    client: Arc<Client>,
}

#[derive(Clone, Copy)]
enum Role {
    Assistant,
    User,
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

pub struct AssistantButton {
    workspace: WeakViewHandle<Workspace>,
    active: bool,
}

pub fn init(cx: &mut AppContext) {
    cx.add_action(AssistantButton::deploy_assistant);
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
            |this, ix, cx| this.render_message_list_item(ix, cx),
        );

        Self {
            composer,
            message_list_state,
            message_list_items,
            languages,
            client,
        }
    }

    fn render_message_list_item(&self, ix: usize, cx: &mut MutableAppContext) -> ElementBox {
        let list_item = &self.message_list_items[ix];
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
            ListItem::CodeMessage {
                role,
                content,
                language,
            } => {
                let style = match role {
                    Role::Assistant => &theme.assistant_message,
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
                                .collect(),
                        )
                        .contained()
                        .with_style(style.code_message.container)
                        .boxed()
                } else {
                    Text::new(content.to_owned(), style.code_message.text.clone())
                        .with_soft_wrap(false)
                        .contained()
                        .with_style(style.code_message.container)
                        .boxed()
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
            messages: vec![AssistantRequestMessage {
                content,
                role: Role::User as i32,
            }],
        });

        cx.spawn_weak(|this, mut cx| {
            async move {
                let mut stream = stream.await?.fuse();

                if let Some(first_message) = stream.next().await {
                    let message = first_message?;
                    if let Some(this) = this.upgrade(&cx) {
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
                        });
                    }
                }

                while let Some(update_message) = stream.next().await {
                    let message = update_message?;
                    if let Some(new_content) = message.content {
                        if let Some(this) = this.upgrade(&cx) {
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
                            });
                        }
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
            .with_child(
                List::new(self.message_list_state.clone())
                    .flex(1., true)
                    .boxed(),
            )
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

impl Role {
    fn from_proto(proto: assistant_response::Role) -> Option<Self> {
        match proto {
            assistant_response::Role::User => Some(Self::User),
            assistant_response::Role::Assistant => Some(Self::Assistant),
            _ => None,
        }
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
                let languages = workspace.project().read(cx).languages().clone();
                let client = workspace.client().clone();
                let assistant = workspace.items_of_type::<Assistant>(cx).next();
                if let Some(assistant) = assistant {
                    workspace.activate_item(&assistant, cx);
                } else {
                    workspace.show_dock(true, cx);
                    let assistant = cx.add_view(|cx| Assistant::new(languages, client, cx));
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
