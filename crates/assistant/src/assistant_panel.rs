use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    humanize_token_count, parse_next_edit_suggestion,
    prompt_library::open_prompt_library,
    search::*,
    slash_command::{
        default_command::DefaultSlashCommand,
        docs_command::{DocsSlashCommand, DocsSlashCommandArgs},
        SlashCommandCompletionProvider, SlashCommandRegistry,
    },
    terminal_inline_assistant::TerminalInlineAssistant,
    ApplyEdit, Assist, CompletionProvider, ConfirmCommand, Context, ContextEvent, ContextStore,
    CycleMessageRole, DeployHistory, DeployPromptLibrary, EditSuggestion, InlineAssist,
    InlineAssistant, InsertIntoEditor, MessageId, MessageStatus, ModelSelector,
    PendingSlashCommand, PendingSlashCommandStatus, QuoteSelection, ResetKey, Role,
    SavedContextMetadata, Split, ToggleFocus, ToggleModelSelector,
};
use anyhow::{anyhow, Result};
use assistant_slash_command::{SlashCommand, SlashCommandOutputSection};
use breadcrumbs::Breadcrumbs;
use collections::{BTreeSet, HashMap, HashSet};
use editor::{
    actions::{FoldAt, MoveToEndOfLine, Newline, ShowCompletions, UnfoldAt},
    display_map::{
        BlockDisposition, BlockId, BlockProperties, BlockStyle, Crease, RenderBlock, ToDisplayPoint,
    },
    scroll::{Autoscroll, AutoscrollStrategy},
    Anchor, Editor, EditorEvent, RowExt, ToOffset as _, ToPoint,
};
use editor::{display_map::CreaseId, FoldPlaceholder};
use fs::Fs;
use gpui::{
    div, percentage, point, Action, Animation, AnimationExt, AnyElement, AnyView, AppContext,
    AsyncWindowContext, ClipboardItem, DismissEvent, Empty, EventEmitter, FocusHandle,
    FocusableView, InteractiveElement, IntoElement, Model, ParentElement, Pixels, Render,
    SharedString, StatefulInteractiveElement, Styled, Subscription, Task, Transformation,
    UpdateGlobal, View, ViewContext, VisualContext, WeakView, WindowContext,
};
use indexed_docs::IndexedDocsStore;
use language::{
    language_settings::SoftWrap, AutoindentMode, Buffer, LanguageRegistry, LspAdapterDelegate,
    OffsetRangeExt as _, Point, ToOffset,
};
use multi_buffer::MultiBufferRow;
use picker::{Picker, PickerDelegate};
use project::{Project, ProjectLspAdapterDelegate, ProjectTransaction};
use search::{buffer_search::DivRegistrar, BufferSearchBar};
use settings::Settings;
use std::{cmp, fmt::Write, ops::Range, path::PathBuf, sync::Arc, time::Duration};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use theme::ThemeSettings;
use ui::{
    prelude::*, ButtonLike, ContextMenu, Disclosure, ElevationIndex, KeyBinding, ListItem,
    ListItemSpacing, PopoverMenu, PopoverMenuHandle, Tooltip,
};
use util::ResultExt;
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    item::{BreadcrumbText, Item, ItemHandle},
    pane,
    searchable::{SearchEvent, SearchableItem},
    Pane, Save, ToggleZoom, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};
use workspace::{searchable::SearchableItemHandle, NewFile};

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace
                .register_action(|workspace, _: &ToggleFocus, cx| {
                    let settings = AssistantSettings::get_global(cx);
                    if !settings.enabled {
                        return;
                    }

                    workspace.toggle_panel_focus::<AssistantPanel>(cx);
                })
                .register_action(AssistantPanel::inline_assist)
                .register_action(ContextEditor::quote_selection)
                .register_action(ContextEditor::insert_selection);
        },
    )
    .detach();
}

pub enum AssistantPanelEvent {
    ContextEdited,
}

pub struct AssistantPanel {
    pane: View<Pane>,
    workspace: WeakView<Workspace>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    context_store: Model<ContextStore>,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    subscriptions: Vec<Subscription>,
    authentication_prompt: Option<AnyView>,
    model_selector_menu_handle: PopoverMenuHandle<ContextMenu>,
}

struct SavedContextPickerDelegate {
    store: Model<ContextStore>,
    matches: Vec<SavedContextMetadata>,
    selected_index: usize,
}

enum SavedContextPickerEvent {
    Confirmed { path: PathBuf },
}

enum InlineAssistTarget {
    Editor(View<Editor>, bool),
    Terminal(View<TerminalView>),
}

impl EventEmitter<SavedContextPickerEvent> for Picker<SavedContextPickerDelegate> {}

impl SavedContextPickerDelegate {
    fn new(store: Model<ContextStore>) -> Self {
        Self {
            store,
            matches: Vec::new(),
            selected_index: 0,
        }
    }
}

impl PickerDelegate for SavedContextPickerDelegate {
    type ListItem = ListItem;

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _cx: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Search...".into()
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        let search = self.store.read(cx).search(query, cx);
        cx.spawn(|this, mut cx| async move {
            let matches = search.await;
            this.update(&mut cx, |this, cx| {
                this.delegate.matches = matches;
                this.delegate.selected_index = 0;
                cx.notify();
            })
            .ok();
        })
    }

    fn confirm(&mut self, _secondary: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(metadata) = self.matches.get(self.selected_index) {
            cx.emit(SavedContextPickerEvent::Confirmed {
                path: metadata.path.clone(),
            })
        }
    }

    fn dismissed(&mut self, _cx: &mut ViewContext<Picker<Self>>) {}

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let context = self.matches.get(ix)?;
        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(
                    div()
                        .flex()
                        .w_full()
                        .gap_2()
                        .child(
                            Label::new(context.mtime.format("%F %I:%M%p").to_string())
                                .color(Color::Muted)
                                .size(LabelSize::Small),
                        )
                        .child(Label::new(context.title.clone()).size(LabelSize::Small)),
                ),
        )
    }
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let context_store = workspace
                .update(&mut cx, |workspace, cx| {
                    let app_state = workspace.app_state();
                    ContextStore::new(
                        app_state.fs.clone(),
                        app_state.languages.clone(),
                        Some(app_state.client.telemetry().clone()),
                        cx,
                    )
                })?
                .await?;
            workspace.update(&mut cx, |workspace, cx| {
                // TODO: deserialize state.
                cx.new_view(|cx| Self::new(workspace, context_store, cx))
            })
        })
    }

    fn new(
        workspace: &Workspace,
        context_store: Model<ContextStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let model_selector_menu_handle = PopoverMenuHandle::default();
        let pane = cx.new_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.project().clone(),
                Default::default(),
                None,
                NewFile.boxed_clone(),
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(true, cx);
            pane.display_nav_history_buttons(None);
            pane.set_should_display_tab_bar(|_| true);
            pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                h_flex()
                    .gap(Spacing::Small.rems(cx))
                    .child(
                        IconButton::new("menu", IconName::Menu)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(|pane, _, cx| {
                                let zoom_label = if pane.is_zoomed() {
                                    "Zoom Out"
                                } else {
                                    "Zoom In"
                                };
                                let menu = ContextMenu::build(cx, |menu, cx| {
                                    menu.context(pane.focus_handle(cx))
                                        .action("New Context", Box::new(NewFile))
                                        .action("History", Box::new(DeployHistory))
                                        .action("Prompt Library", Box::new(DeployPromptLibrary))
                                        .action(zoom_label, Box::new(ToggleZoom))
                                });
                                cx.subscribe(&menu, |pane, _, _: &DismissEvent, _| {
                                    pane.new_item_menu = None;
                                })
                                .detach();
                                pane.new_item_menu = Some(menu);
                            })),
                    )
                    .when_some(pane.new_item_menu.as_ref(), |el, new_item_menu| {
                        el.child(Pane::render_menu_overlay(new_item_menu))
                    })
                    .into_any_element()
            });
            pane.toolbar().update(cx, |toolbar, cx| {
                toolbar.add_item(cx.new_view(|_| Breadcrumbs::new()), cx);
                toolbar.add_item(
                    cx.new_view(|_| {
                        ContextEditorToolbarItem::new(workspace, model_selector_menu_handle.clone())
                    }),
                    cx,
                );
                toolbar.add_item(cx.new_view(BufferSearchBar::new), cx)
            });
            pane
        });

        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe(&pane, Self::handle_pane_event),
            cx.observe_global::<CompletionProvider>({
                let mut prev_settings_version = CompletionProvider::global(cx).settings_version();
                move |this, cx| {
                    this.completion_provider_changed(prev_settings_version, cx);
                    prev_settings_version = CompletionProvider::global(cx).settings_version();
                }
            }),
        ];

        Self {
            pane,
            workspace: workspace.weak_handle(),
            width: None,
            height: None,
            context_store,
            languages: workspace.app_state().languages.clone(),
            fs: workspace.app_state().fs.clone(),
            subscriptions,
            authentication_prompt: None,
            model_selector_menu_handle,
        }
    }

    fn handle_pane_event(
        &mut self,
        _pane: View<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::Remove => cx.emit(PanelEvent::Close),
            pane::Event::ZoomIn => cx.emit(PanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(PanelEvent::ZoomOut),

            pane::Event::AddItem { item } => {
                if let Some(workspace) = self.workspace.upgrade() {
                    workspace.update(cx, |workspace, cx| {
                        item.added_to_pane(workspace, self.pane.clone(), cx)
                    });
                }
            }

            pane::Event::RemoveItem { .. } | pane::Event::ActivateItem { .. } => {
                cx.emit(AssistantPanelEvent::ContextEdited);
            }

            _ => {}
        }
    }

    fn completion_provider_changed(
        &mut self,
        prev_settings_version: usize,
        cx: &mut ViewContext<Self>,
    ) {
        if self.is_authenticated(cx) {
            self.authentication_prompt = None;

            if let Some(editor) = self.active_context_editor(cx) {
                editor.update(cx, |active_context, cx| {
                    active_context
                        .context
                        .update(cx, |context, cx| context.completion_provider_changed(cx))
                })
            }

            if self.active_context_editor(cx).is_none() {
                self.new_context(cx);
            }
            cx.notify();
        } else if self.authentication_prompt.is_none()
            || prev_settings_version != CompletionProvider::global(cx).settings_version()
        {
            self.authentication_prompt =
                Some(cx.update_global::<CompletionProvider, _>(|provider, cx| {
                    provider.authentication_prompt(cx)
                }));
            cx.notify();
        }
    }

    pub fn inline_assist(
        workspace: &mut Workspace,
        _: &InlineAssist,
        cx: &mut ViewContext<Workspace>,
    ) {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled {
            return;
        }

        let Some(assistant_panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };

        let Some(inline_assist_target) =
            Self::resolve_inline_assist_target(workspace, &assistant_panel, cx)
        else {
            return;
        };

        if assistant_panel.update(cx, |assistant, cx| assistant.is_authenticated(cx)) {
            match inline_assist_target {
                InlineAssistTarget::Editor(active_editor, include_context) => {
                    InlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_editor,
                            Some(cx.view().downgrade()),
                            include_context.then_some(&assistant_panel),
                            cx,
                        )
                    })
                }
                InlineAssistTarget::Terminal(active_terminal) => {
                    TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                        assistant.assist(
                            &active_terminal,
                            Some(cx.view().downgrade()),
                            Some(&assistant_panel),
                            cx,
                        )
                    })
                }
            }
        } else {
            let assistant_panel = assistant_panel.downgrade();
            cx.spawn(|workspace, mut cx| async move {
                assistant_panel
                    .update(&mut cx, |assistant, cx| assistant.authenticate(cx))?
                    .await?;
                if assistant_panel.update(&mut cx, |panel, cx| panel.is_authenticated(cx))? {
                    cx.update(|cx| match inline_assist_target {
                        InlineAssistTarget::Editor(active_editor, include_context) => {
                            let assistant_panel = if include_context {
                                assistant_panel.upgrade()
                            } else {
                                None
                            };
                            InlineAssistant::update_global(cx, |assistant, cx| {
                                assistant.assist(
                                    &active_editor,
                                    Some(workspace),
                                    assistant_panel.as_ref(),
                                    cx,
                                )
                            })
                        }
                        InlineAssistTarget::Terminal(active_terminal) => {
                            TerminalInlineAssistant::update_global(cx, |assistant, cx| {
                                assistant.assist(
                                    &active_terminal,
                                    Some(workspace),
                                    assistant_panel.upgrade().as_ref(),
                                    cx,
                                )
                            })
                        }
                    })?
                } else {
                    workspace.update(&mut cx, |workspace, cx| {
                        workspace.focus_panel::<AssistantPanel>(cx)
                    })?;
                }

                anyhow::Ok(())
            })
            .detach_and_log_err(cx)
        }
    }

    fn resolve_inline_assist_target(
        workspace: &mut Workspace,
        assistant_panel: &View<AssistantPanel>,
        cx: &mut WindowContext,
    ) -> Option<InlineAssistTarget> {
        if let Some(terminal_panel) = workspace.panel::<TerminalPanel>(cx) {
            if terminal_panel
                .read(cx)
                .focus_handle(cx)
                .contains_focused(cx)
            {
                use feature_flags::FeatureFlagAppExt;
                if !cx.has_flag::<feature_flags::TerminalInlineAssist>() {
                    return None;
                }

                if let Some(terminal_view) = terminal_panel
                    .read(cx)
                    .pane()
                    .read(cx)
                    .active_item()
                    .and_then(|t| t.downcast::<TerminalView>())
                {
                    return Some(InlineAssistTarget::Terminal(terminal_view));
                }
            }
        }
        let context_editor =
            assistant_panel
                .read(cx)
                .active_context_editor(cx)
                .and_then(|editor| {
                    let editor = &editor.read(cx).editor;
                    if editor.read(cx).is_focused(cx) {
                        Some(editor.clone())
                    } else {
                        None
                    }
                });

        if let Some(context_editor) = context_editor {
            Some(InlineAssistTarget::Editor(context_editor, false))
        } else if let Some(workspace_editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        {
            Some(InlineAssistTarget::Editor(workspace_editor, true))
        } else {
            None
        }
    }

    fn new_context(&mut self, cx: &mut ViewContext<Self>) -> Option<View<ContextEditor>> {
        let context = self.context_store.update(cx, |store, cx| store.create(cx));
        let workspace = self.workspace.upgrade()?;
        let lsp_adapter_delegate = workspace.update(cx, |workspace, cx| {
            make_lsp_adapter_delegate(workspace.project(), cx).log_err()
        });

        let editor = cx.new_view(|cx| {
            let mut editor = ContextEditor::for_context(
                context,
                self.fs.clone(),
                workspace,
                lsp_adapter_delegate,
                cx,
            );
            editor.insert_default_prompt(cx);
            editor
        });

        self.show_context(editor.clone(), cx);
        Some(editor)
    }

    fn show_context(&mut self, context_editor: View<ContextEditor>, cx: &mut ViewContext<Self>) {
        let focus = self.focus_handle(cx).contains_focused(cx);
        let prev_len = self.pane.read(cx).items_len();
        self.pane.update(cx, |pane, cx| {
            pane.add_item(Box::new(context_editor.clone()), focus, focus, None, cx)
        });

        if prev_len != self.pane.read(cx).items_len() {
            self.subscriptions
                .push(cx.subscribe(&context_editor, Self::handle_context_editor_event));
        }

        cx.emit(AssistantPanelEvent::ContextEdited);
        cx.notify();
    }

    fn handle_context_editor_event(
        &mut self,
        _: View<ContextEditor>,
        event: &ContextEditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ContextEditorEvent::TabContentChanged => cx.notify(),
            ContextEditorEvent::Edited => cx.emit(AssistantPanelEvent::ContextEdited),
        }
    }

    fn deploy_history(&mut self, _: &DeployHistory, cx: &mut ViewContext<Self>) {
        let history_item_ix = self
            .pane
            .read(cx)
            .items()
            .position(|item| item.downcast::<ContextHistory>().is_some());

        if let Some(history_item_ix) = history_item_ix {
            self.pane.update(cx, |pane, cx| {
                pane.activate_item(history_item_ix, true, true, cx);
            });
        } else {
            let assistant_panel = cx.view().downgrade();
            let history = cx.new_view(|cx| {
                ContextHistory::new(self.context_store.clone(), assistant_panel, cx)
            });
            self.pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(history), true, true, None, cx);
            });
        }
    }

    fn deploy_prompt_library(&mut self, _: &DeployPromptLibrary, cx: &mut ViewContext<Self>) {
        open_prompt_library(self.languages.clone(), cx).detach_and_log_err(cx);
    }

    fn reset_credentials(&mut self, _: &ResetKey, cx: &mut ViewContext<Self>) {
        CompletionProvider::global(cx)
            .reset_credentials(cx)
            .detach_and_log_err(cx);
    }

    fn toggle_model_selector(&mut self, _: &ToggleModelSelector, cx: &mut ViewContext<Self>) {
        self.model_selector_menu_handle.toggle(cx);
    }

    fn active_context_editor(&self, cx: &AppContext) -> Option<View<ContextEditor>> {
        self.pane
            .read(cx)
            .active_item()?
            .downcast::<ContextEditor>()
    }

    pub fn active_context(&self, cx: &AppContext) -> Option<Model<Context>> {
        Some(self.active_context_editor(cx)?.read(cx).context.clone())
    }

    fn open_context(&mut self, path: PathBuf, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        let existing_context = self.pane.read(cx).items().find_map(|item| {
            item.downcast::<ContextEditor>()
                .filter(|editor| editor.read(cx).context.read(cx).path() == Some(&path))
        });
        if let Some(existing_context) = existing_context {
            self.show_context(existing_context, cx);
            return Task::ready(Ok(()));
        }

        let context = self.context_store.read(cx).load(path.clone(), cx);
        let fs = self.fs.clone();
        let workspace = self.workspace.clone();

        let lsp_adapter_delegate = workspace
            .update(cx, |workspace, cx| {
                make_lsp_adapter_delegate(workspace.project(), cx).log_err()
            })
            .log_err()
            .flatten();

        cx.spawn(|this, mut cx| async move {
            let context = context.await?;
            this.update(&mut cx, |this, cx| {
                let workspace = workspace
                    .upgrade()
                    .ok_or_else(|| anyhow!("workspace dropped"))?;
                let editor = cx.new_view(|cx| {
                    ContextEditor::for_context(context, fs, workspace, lsp_adapter_delegate, cx)
                });
                this.show_context(editor, cx);
                anyhow::Ok(())
            })??;
            Ok(())
        })
    }

    fn is_authenticated(&mut self, cx: &mut ViewContext<Self>) -> bool {
        CompletionProvider::global(cx).is_authenticated()
    }

    fn authenticate(&mut self, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        cx.update_global::<CompletionProvider, _>(|provider, cx| provider.authenticate(cx))
    }

    fn render_signed_in(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut registrar = DivRegistrar::new(
            |panel, cx| {
                panel
                    .pane
                    .read(cx)
                    .toolbar()
                    .read(cx)
                    .item_of_type::<BufferSearchBar>()
            },
            cx,
        );
        BufferSearchBar::register(&mut registrar);
        let registrar = registrar.into_div();

        v_flex()
            .key_context("AssistantPanel")
            .size_full()
            .on_action(cx.listener(|this, _: &workspace::NewFile, cx| {
                this.new_context(cx);
            }))
            .on_action(cx.listener(AssistantPanel::deploy_history))
            .on_action(cx.listener(AssistantPanel::deploy_prompt_library))
            .on_action(cx.listener(AssistantPanel::reset_credentials))
            .on_action(cx.listener(AssistantPanel::toggle_model_selector))
            .child(registrar.size_full().child(self.pane.clone()))
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        if let Some(authentication_prompt) = self.authentication_prompt.as_ref() {
            authentication_prompt.clone().into_any()
        } else {
            self.render_signed_in(cx).into_any_element()
        }
    }
}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanel"
    }

    fn position(&self, cx: &WindowContext) -> DockPosition {
        match AssistantSettings::get_global(cx).dock {
            AssistantDockPosition::Left => DockPosition::Left,
            AssistantDockPosition::Bottom => DockPosition::Bottom,
            AssistantDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<AssistantSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                DockPosition::Left => AssistantDockPosition::Left,
                DockPosition::Bottom => AssistantDockPosition::Bottom,
                DockPosition::Right => AssistantDockPosition::Right,
            };
            settings.set_dock(dock);
        });
    }

    fn size(&self, cx: &WindowContext) -> Pixels {
        let settings = AssistantSettings::get_global(cx);
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or(settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or(settings.default_height),
        }
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => self.width = size,
            DockPosition::Bottom => self.height = size,
        }
        cx.notify();
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            let load_credentials = self.authenticate(cx);
            cx.spawn(|this, mut cx| async move {
                load_credentials.await?;
                this.update(&mut cx, |this, cx| {
                    if this.is_authenticated(cx) && this.active_context_editor(cx).is_none() {
                        this.new_context(cx);
                    }
                })
            })
            .detach_and_log_err(cx);
        }
    }

    fn icon(&self, cx: &WindowContext) -> Option<IconName> {
        let settings = AssistantSettings::get_global(cx);
        if !settings.enabled || !settings.button {
            return None;
        }

        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}
impl EventEmitter<AssistantPanelEvent> for AssistantPanel {}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.pane.focus_handle(cx)
    }
}

pub enum ContextEditorEvent {
    Edited,
    TabContentChanged,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: gpui::Point<f32>,
    cursor: Anchor,
}

pub struct ContextEditor {
    context: Model<Context>,
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
    editor: View<Editor>,
    blocks: HashSet<BlockId>,
    scroll_position: Option<ScrollPosition>,
    pending_slash_command_creases: HashMap<Range<language::Anchor>, CreaseId>,
    pending_slash_command_blocks: HashMap<Range<language::Anchor>, BlockId>,
    _subscriptions: Vec<Subscription>,
}

impl ContextEditor {
    const MAX_TAB_TITLE_LEN: usize = 16;

    fn for_context(
        context: Model<Context>,
        fs: Arc<dyn Fs>,
        workspace: View<Workspace>,
        lsp_adapter_delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let completion_provider = SlashCommandCompletionProvider::new(
            Some(cx.view().downgrade()),
            Some(workspace.downgrade()),
        );

        let editor = cx.new_view(|cx| {
            let mut editor = Editor::for_buffer(context.read(cx).buffer().clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor.set_show_indent_guides(false, cx);
            editor.set_completion_provider(Box::new(completion_provider));
            editor
        });

        let _subscriptions = vec![
            cx.observe(&context, |_, _, cx| cx.notify()),
            cx.subscribe(&context, Self::handle_context_event),
            cx.subscribe(&editor, Self::handle_editor_event),
            cx.subscribe(&editor, Self::handle_editor_search_event),
        ];

        let sections = context.read(cx).slash_command_output_sections().to_vec();
        let mut this = Self {
            context,
            editor,
            lsp_adapter_delegate,
            blocks: Default::default(),
            scroll_position: None,
            fs,
            workspace: workspace.downgrade(),
            pending_slash_command_creases: HashMap::default(),
            pending_slash_command_blocks: HashMap::default(),
            _subscriptions,
        };
        this.update_message_headers(cx);
        this.insert_slash_command_output_sections(sections, cx);
        this
    }

    fn insert_default_prompt(&mut self, cx: &mut ViewContext<Self>) {
        let command_name = DefaultSlashCommand.name();
        self.editor.update(cx, |editor, cx| {
            editor.insert(&format!("/{command_name}"), cx)
        });
        self.split(&Split, cx);
        let command = self.context.update(cx, |context, cx| {
            context.set_message_role(MessageId::default(), Role::System, cx);
            context.reparse_slash_commands(cx);
            context.pending_slash_commands()[0].clone()
        });

        self.run_command(
            command.source_range,
            &command.name,
            command.argument.as_deref(),
            false,
            self.workspace.clone(),
            cx,
        );
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);

        let user_messages = self.context.update(cx, |context, cx| {
            let selected_messages = context
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            context.assist(selected_messages, cx)
        });
        let new_selections = user_messages
            .iter()
            .map(|message| {
                let cursor = message
                    .start
                    .to_offset(self.context.read(cx).buffer().read(cx));
                cursor..cursor
            })
            .collect::<Vec<_>>();
        if !new_selections.is_empty() {
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Fit)),
                    cx,
                    |selections| selections.select_ranges(new_selections),
                );
            });
            // Avoid scrolling to the new cursor position so the assistant's output is stable.
            cx.defer(|this, _| this.scroll_position = None);
        }
    }

    fn cancel_last_assist(&mut self, _: &editor::actions::Cancel, cx: &mut ViewContext<Self>) {
        if !self
            .context
            .update(cx, |context, _| context.cancel_last_assist())
        {
            cx.propagate();
        }
    }

    fn cycle_message_role(&mut self, _: &CycleMessageRole, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);
        self.context.update(cx, |context, cx| {
            let messages = context
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            context.cycle_message_roles(messages, cx)
        });
    }

    fn cursors(&self, cx: &AppContext) -> Vec<usize> {
        let selections = self.editor.read(cx).selections.all::<usize>(cx);
        selections
            .into_iter()
            .map(|selection| selection.head())
            .collect()
    }

    fn insert_command(&mut self, name: &str, cx: &mut ViewContext<Self>) {
        if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
            self.editor.update(cx, |editor, cx| {
                editor.transact(cx, |editor, cx| {
                    editor.change_selections(Some(Autoscroll::fit()), cx, |s| s.try_cancel());
                    let snapshot = editor.buffer().read(cx).snapshot(cx);
                    let newest_cursor = editor.selections.newest::<Point>(cx).head();
                    if newest_cursor.column > 0
                        || snapshot
                            .chars_at(newest_cursor)
                            .next()
                            .map_or(false, |ch| ch != '\n')
                    {
                        editor.move_to_end_of_line(
                            &MoveToEndOfLine {
                                stop_at_soft_wraps: false,
                            },
                            cx,
                        );
                        editor.newline(&Newline, cx);
                    }

                    editor.insert(&format!("/{name}"), cx);
                    if command.requires_argument() {
                        editor.insert(" ", cx);
                        editor.show_completions(&ShowCompletions::default(), cx);
                    }
                });
            });
            if !command.requires_argument() {
                self.confirm_command(&ConfirmCommand, cx);
            }
        }
    }

    pub fn confirm_command(&mut self, _: &ConfirmCommand, cx: &mut ViewContext<Self>) {
        let selections = self.editor.read(cx).selections.disjoint_anchors();
        let mut commands_by_range = HashMap::default();
        let workspace = self.workspace.clone();
        self.context.update(cx, |context, cx| {
            context.reparse_slash_commands(cx);
            for selection in selections.iter() {
                if let Some(command) =
                    context.pending_command_for_position(selection.head().text_anchor, cx)
                {
                    commands_by_range
                        .entry(command.source_range.clone())
                        .or_insert_with(|| command.clone());
                }
            }
        });

        if commands_by_range.is_empty() {
            cx.propagate();
        } else {
            for command in commands_by_range.into_values() {
                self.run_command(
                    command.source_range,
                    &command.name,
                    command.argument.as_deref(),
                    true,
                    workspace.clone(),
                    cx,
                );
            }
            cx.stop_propagation();
        }
    }

    pub fn run_command(
        &mut self,
        command_range: Range<language::Anchor>,
        name: &str,
        argument: Option<&str>,
        insert_trailing_newline: bool,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(command) = SlashCommandRegistry::global(cx).command(name) {
            if let Some(lsp_adapter_delegate) = self.lsp_adapter_delegate.clone() {
                let argument = argument.map(ToString::to_string);
                let output = command.run(argument.as_deref(), workspace, lsp_adapter_delegate, cx);
                self.context.update(cx, |context, cx| {
                    context.insert_command_output(
                        command_range,
                        output,
                        insert_trailing_newline,
                        cx,
                    )
                });
            }
        }
    }

    fn handle_context_event(
        &mut self,
        _: Model<Context>,
        event: &ContextEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let context_editor = cx.view().downgrade();

        match event {
            ContextEvent::MessagesEdited => {
                self.update_message_headers(cx);
                self.context.update(cx, |context, cx| {
                    context.save(Some(Duration::from_millis(500)), self.fs.clone(), cx);
                });
            }
            ContextEvent::EditSuggestionsChanged => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let excerpt_id = *buffer.as_singleton().unwrap().0;
                    let context = self.context.read(cx);
                    let highlighted_rows = context
                        .edit_suggestions()
                        .iter()
                        .map(|suggestion| {
                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, suggestion.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, suggestion.source_range.end)
                                .unwrap();
                            start..=end
                        })
                        .collect::<Vec<_>>();

                    editor.clear_row_highlights::<EditSuggestion>();
                    for range in highlighted_rows {
                        editor.highlight_rows::<EditSuggestion>(
                            range,
                            Some(
                                cx.theme()
                                    .colors()
                                    .editor_document_highlight_read_background,
                            ),
                            false,
                            cx,
                        );
                    }
                });
            }
            ContextEvent::SummaryChanged => {
                cx.emit(ContextEditorEvent::TabContentChanged);
                self.context.update(cx, |context, cx| {
                    context.save(None, self.fs.clone(), cx);
                });
            }
            ContextEvent::StreamedCompletion => {
                self.editor.update(cx, |editor, cx| {
                    if let Some(scroll_position) = self.scroll_position {
                        let snapshot = editor.snapshot(cx);
                        let cursor_point = scroll_position.cursor.to_display_point(&snapshot);
                        let scroll_top =
                            cursor_point.row().as_f32() - scroll_position.offset_before_cursor.y;
                        editor.set_scroll_position(
                            point(scroll_position.offset_before_cursor.x, scroll_top),
                            cx,
                        );
                    }
                });
            }
            ContextEvent::PendingSlashCommandsUpdated { removed, updated } => {
                self.editor.update(cx, |editor, cx| {
                    let buffer = editor.buffer().read(cx).snapshot(cx);
                    let (excerpt_id, buffer_id, _) = buffer.as_singleton().unwrap();
                    let excerpt_id = *excerpt_id;

                    editor.remove_creases(
                        removed
                            .iter()
                            .filter_map(|range| self.pending_slash_command_creases.remove(range)),
                        cx,
                    );

                    editor.remove_blocks(
                        HashSet::from_iter(
                            removed.iter().filter_map(|range| {
                                self.pending_slash_command_blocks.remove(range)
                            }),
                        ),
                        None,
                        cx,
                    );

                    let crease_ids = editor.insert_creases(
                        updated.iter().map(|command| {
                            let workspace = self.workspace.clone();
                            let confirm_command = Arc::new({
                                let context_editor = context_editor.clone();
                                let command = command.clone();
                                move |cx: &mut WindowContext| {
                                    context_editor
                                        .update(cx, |context_editor, cx| {
                                            context_editor.run_command(
                                                command.source_range.clone(),
                                                &command.name,
                                                command.argument.as_deref(),
                                                false,
                                                workspace.clone(),
                                                cx,
                                            );
                                        })
                                        .ok();
                                }
                            });
                            let placeholder = FoldPlaceholder {
                                render: Arc::new(move |_, _, _| Empty.into_any()),
                                constrain_width: false,
                                merge_adjacent: false,
                            };
                            let render_toggle = {
                                let confirm_command = confirm_command.clone();
                                let command = command.clone();
                                move |row, _, _, _cx: &mut WindowContext| {
                                    render_pending_slash_command_gutter_decoration(
                                        row,
                                        &command.status,
                                        confirm_command.clone(),
                                    )
                                }
                            };
                            let render_trailer = {
                                let command = command.clone();
                                move |row, _unfold, cx: &mut WindowContext| {
                                    // TODO: In the future we should investigate how we can expose
                                    // this as a hook on the `SlashCommand` trait so that we don't
                                    // need to special-case it here.
                                    if command.name == DocsSlashCommand::NAME {
                                        return render_docs_slash_command_trailer(
                                            row,
                                            command.clone(),
                                            cx,
                                        );
                                    }

                                    Empty.into_any()
                                }
                            };

                            let start = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.start)
                                .unwrap();
                            let end = buffer
                                .anchor_in_excerpt(excerpt_id, command.source_range.end)
                                .unwrap();
                            Crease::new(start..end, placeholder, render_toggle, render_trailer)
                        }),
                        cx,
                    );

                    let block_ids = editor.insert_blocks(
                        updated
                            .iter()
                            .filter_map(|command| match &command.status {
                                PendingSlashCommandStatus::Error(error) => {
                                    Some((command, error.clone()))
                                }
                                _ => None,
                            })
                            .map(|(command, error_message)| BlockProperties {
                                style: BlockStyle::Fixed,
                                position: Anchor {
                                    buffer_id: Some(buffer_id),
                                    excerpt_id,
                                    text_anchor: command.source_range.start,
                                },
                                height: 1,
                                disposition: BlockDisposition::Below,
                                render: slash_command_error_block_renderer(error_message),
                            }),
                        None,
                        cx,
                    );

                    self.pending_slash_command_creases.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(crease_ids),
                    );

                    self.pending_slash_command_blocks.extend(
                        updated
                            .iter()
                            .map(|command| command.source_range.clone())
                            .zip(block_ids),
                    );
                })
            }
            ContextEvent::SlashCommandFinished {
                output_range,
                sections,
                run_commands_in_output,
            } => {
                self.insert_slash_command_output_sections(sections.iter().cloned(), cx);

                if *run_commands_in_output {
                    let commands = self.context.update(cx, |context, cx| {
                        context.reparse_slash_commands(cx);
                        context
                            .pending_commands_for_range(output_range.clone(), cx)
                            .to_vec()
                    });

                    for command in commands {
                        self.run_command(
                            command.source_range,
                            &command.name,
                            command.argument.as_deref(),
                            false,
                            self.workspace.clone(),
                            cx,
                        );
                    }
                }
            }
        }
    }

    fn insert_slash_command_output_sections(
        &mut self,
        sections: impl IntoIterator<Item = SlashCommandOutputSection<language::Anchor>>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let mut buffer_rows_to_fold = BTreeSet::new();
            let mut creases = Vec::new();
            for section in sections {
                let start = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.start)
                    .unwrap();
                let end = buffer
                    .anchor_in_excerpt(excerpt_id, section.range.end)
                    .unwrap();
                let buffer_row = MultiBufferRow(start.to_point(&buffer).row);
                buffer_rows_to_fold.insert(buffer_row);
                creases.push(Crease::new(
                    start..end,
                    FoldPlaceholder {
                        render: Arc::new({
                            let editor = cx.view().downgrade();
                            let icon = section.icon;
                            let label = section.label.clone();
                            move |fold_id, fold_range, _cx| {
                                let editor = editor.clone();
                                ButtonLike::new(fold_id)
                                    .style(ButtonStyle::Filled)
                                    .layer(ElevationIndex::ElevatedSurface)
                                    .child(Icon::new(icon))
                                    .child(Label::new(label.clone()).single_line())
                                    .on_click(move |_, cx| {
                                        editor
                                            .update(cx, |editor, cx| {
                                                let buffer_start = fold_range
                                                    .start
                                                    .to_point(&editor.buffer().read(cx).read(cx));
                                                let buffer_row = MultiBufferRow(buffer_start.row);
                                                editor.unfold_at(&UnfoldAt { buffer_row }, cx);
                                            })
                                            .ok();
                                    })
                                    .into_any_element()
                            }
                        }),
                        constrain_width: false,
                        merge_adjacent: false,
                    },
                    render_slash_command_output_toggle,
                    |_, _, _| Empty.into_any_element(),
                ));
            }

            editor.insert_creases(creases, cx);

            for buffer_row in buffer_rows_to_fold.into_iter().rev() {
                editor.fold_at(&FoldAt { buffer_row }, cx);
            }
        });
    }

    fn handle_editor_event(
        &mut self,
        _: View<Editor>,
        event: &EditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            EditorEvent::ScrollPositionChanged { autoscroll, .. } => {
                let cursor_scroll_position = self.cursor_scroll_position(cx);
                if *autoscroll {
                    self.scroll_position = cursor_scroll_position;
                } else if self.scroll_position != cursor_scroll_position {
                    self.scroll_position = None;
                }
            }
            EditorEvent::SelectionsChanged { .. } => {
                self.scroll_position = self.cursor_scroll_position(cx);
            }
            EditorEvent::BufferEdited => cx.emit(ContextEditorEvent::Edited),
            _ => {}
        }
    }

    fn handle_editor_search_event(
        &mut self,
        _: View<Editor>,
        event: &SearchEvent,
        cx: &mut ViewContext<Self>,
    ) {
        cx.emit(event.clone());
    }

    fn cursor_scroll_position(&self, cx: &mut ViewContext<Self>) -> Option<ScrollPosition> {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let cursor = editor.selections.newest_anchor().head();
            let cursor_row = cursor
                .to_display_point(&snapshot.display_snapshot)
                .row()
                .as_f32();
            let scroll_position = editor
                .scroll_manager
                .anchor()
                .scroll_position(&snapshot.display_snapshot);

            let scroll_bottom = scroll_position.y + editor.visible_line_count().unwrap_or(0.);
            if (scroll_position.y..scroll_bottom).contains(&cursor_row) {
                Some(ScrollPosition {
                    cursor,
                    offset_before_cursor: point(scroll_position.x, cursor_row - scroll_position.y),
                })
            } else {
                None
            }
        })
    }

    fn update_message_headers(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let old_blocks = std::mem::take(&mut self.blocks);
            let new_blocks = self
                .context
                .read(cx)
                .messages(cx)
                .map(|message| BlockProperties {
                    position: buffer
                        .anchor_in_excerpt(excerpt_id, message.anchor)
                        .unwrap(),
                    height: 2,
                    style: BlockStyle::Sticky,
                    render: Box::new({
                        let context = self.context.clone();
                        move |cx| {
                            let message_id = message.id;
                            let sender = ButtonLike::new("role")
                                .style(ButtonStyle::Filled)
                                .child(match message.role {
                                    Role::User => Label::new("You").color(Color::Default),
                                    Role::Assistant => Label::new("Assistant").color(Color::Info),
                                    Role::System => Label::new("System").color(Color::Warning),
                                })
                                .tooltip(|cx| {
                                    Tooltip::with_meta(
                                        "Toggle message role",
                                        None,
                                        "Available roles: You (User), Assistant, System",
                                        cx,
                                    )
                                })
                                .on_click({
                                    let context = context.clone();
                                    move |_, cx| {
                                        context.update(cx, |context, cx| {
                                            context.cycle_message_roles(
                                                HashSet::from_iter(Some(message_id)),
                                                cx,
                                            )
                                        })
                                    }
                                });

                            h_flex()
                                .id(("message_header", message_id.0))
                                .pl(cx.gutter_dimensions.full_width())
                                .h_11()
                                .w_full()
                                .relative()
                                .gap_1()
                                .child(sender)
                                .children(
                                    if let MessageStatus::Error(error) = message.status.clone() {
                                        Some(
                                            div()
                                                .id("error")
                                                .tooltip(move |cx| Tooltip::text(error.clone(), cx))
                                                .child(Icon::new(IconName::XCircle)),
                                        )
                                    } else {
                                        None
                                    },
                                )
                                .into_any_element()
                        }
                    }),
                    disposition: BlockDisposition::Above,
                })
                .collect::<Vec<_>>();

            editor.remove_blocks(old_blocks, None, cx);
            let ids = editor.insert_blocks(new_blocks, None, cx);
            self.blocks = HashSet::from_iter(ids);
        });
    }

    fn insert_selection(
        workspace: &mut Workspace,
        _: &InsertIntoEditor,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(context_editor_view) = panel.read(cx).active_context_editor(cx) else {
            return;
        };
        let Some(active_editor_view) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        let context_editor = context_editor_view.read(cx).editor.read(cx);
        let anchor = context_editor.selections.newest_anchor();
        let text = context_editor
            .buffer()
            .read(cx)
            .read(cx)
            .text_for_range(anchor.range())
            .collect::<String>();

        // If nothing is selected, don't delete the current selection; instead, be a no-op.
        if !text.is_empty() {
            active_editor_view.update(cx, |editor, cx| {
                editor.insert(&text, cx);
                editor.focus(cx);
            })
        }
    }

    fn quote_selection(
        workspace: &mut Workspace,
        _: &QuoteSelection,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
        else {
            return;
        };

        let editor = editor.read(cx);
        let range = editor.selections.newest::<usize>(cx).range();
        let buffer = editor.buffer().read(cx).snapshot(cx);
        let start_language = buffer.language_at(range.start);
        let end_language = buffer.language_at(range.end);
        let language_name = if start_language == end_language {
            start_language.map(|language| language.code_fence_block_name())
        } else {
            None
        };
        let language_name = language_name.as_deref().unwrap_or("");

        let selected_text = buffer.text_for_range(range).collect::<String>();
        let text = if selected_text.is_empty() {
            None
        } else {
            Some(if language_name == "markdown" {
                selected_text
                    .lines()
                    .map(|line| format!("> {}", line))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                format!("```{language_name}\n{selected_text}\n```")
            })
        };

        // Activate the panel
        if !panel.focus_handle(cx).contains_focused(cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        }

        if let Some(text) = text {
            panel.update(cx, |_, cx| {
                // Wait to create a new context until the workspace is no longer
                // being updated.
                cx.defer(move |panel, cx| {
                    if let Some(context) = panel
                        .active_context_editor(cx)
                        .or_else(|| panel.new_context(cx))
                    {
                        context.update(cx, |context, cx| {
                            context
                                .editor
                                .update(cx, |editor, cx| editor.insert(&text, cx))
                        });
                    };
                });
            });
        }
    }

    fn copy(&mut self, _: &editor::actions::Copy, cx: &mut ViewContext<Self>) {
        let editor = self.editor.read(cx);
        let context = self.context.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let mut copied_text = String::new();
            let mut spanned_messages = 0;
            for message in context.messages(cx) {
                if message.offset_range.start >= selection.range().end {
                    break;
                } else if message.offset_range.end >= selection.range().start {
                    let range = cmp::max(message.offset_range.start, selection.range().start)
                        ..cmp::min(message.offset_range.end, selection.range().end);
                    if !range.is_empty() {
                        spanned_messages += 1;
                        write!(&mut copied_text, "## {}\n\n", message.role).unwrap();
                        for chunk in context.buffer().read(cx).text_for_range(range) {
                            copied_text.push_str(chunk);
                        }
                        copied_text.push('\n');
                    }
                }
            }

            if spanned_messages > 1 {
                cx.write_to_clipboard(ClipboardItem::new(copied_text));
                return;
            }
        }

        cx.propagate();
    }

    fn split(&mut self, _: &Split, cx: &mut ViewContext<Self>) {
        self.context.update(cx, |context, cx| {
            let selections = self.editor.read(cx).selections.disjoint_anchors();
            for selection in selections.as_ref() {
                let buffer = self.editor.read(cx).buffer().read(cx).snapshot(cx);
                let range = selection
                    .map(|endpoint| endpoint.to_offset(&buffer))
                    .range();
                context.split_message(range, cx);
            }
        });
    }

    fn apply_edit(&mut self, _: &ApplyEdit, cx: &mut ViewContext<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let project = workspace.read(cx).project().clone();

        struct Edit {
            old_text: String,
            new_text: String,
        }

        let context = self.context.read(cx);
        let context_buffer = context.buffer().read(cx);
        let context_buffer_snapshot = context_buffer.snapshot();

        let selections = self.editor.read(cx).selections.disjoint_anchors();
        let mut selections = selections.iter().peekable();
        let selected_suggestions = context
            .edit_suggestions()
            .iter()
            .filter(|suggestion| {
                while let Some(selection) = selections.peek() {
                    if selection
                        .end
                        .text_anchor
                        .cmp(&suggestion.source_range.start, context_buffer)
                        .is_lt()
                    {
                        selections.next();
                        continue;
                    }
                    if selection
                        .start
                        .text_anchor
                        .cmp(&suggestion.source_range.end, context_buffer)
                        .is_gt()
                    {
                        break;
                    }
                    return true;
                }
                false
            })
            .cloned()
            .collect::<Vec<_>>();

        let mut opened_buffers: HashMap<PathBuf, Task<Result<Model<Buffer>>>> = HashMap::default();
        project.update(cx, |project, cx| {
            for suggestion in &selected_suggestions {
                opened_buffers
                    .entry(suggestion.full_path.clone())
                    .or_insert_with(|| {
                        project.open_buffer_for_full_path(&suggestion.full_path, cx)
                    });
            }
        });

        cx.spawn(|this, mut cx| async move {
            let mut buffers_by_full_path = HashMap::default();
            for (full_path, buffer) in opened_buffers {
                if let Some(buffer) = buffer.await.log_err() {
                    buffers_by_full_path.insert(full_path, buffer);
                }
            }

            let mut suggestions_by_buffer = HashMap::default();
            cx.update(|cx| {
                for suggestion in selected_suggestions {
                    if let Some(buffer) = buffers_by_full_path.get(&suggestion.full_path) {
                        let (_, edits) = suggestions_by_buffer
                            .entry(buffer.clone())
                            .or_insert_with(|| (buffer.read(cx).snapshot(), Vec::new()));

                        let mut lines = context_buffer_snapshot
                            .as_rope()
                            .chunks_in_range(
                                suggestion.source_range.to_offset(&context_buffer_snapshot),
                            )
                            .lines();
                        if let Some(suggestion) = parse_next_edit_suggestion(&mut lines) {
                            let old_text = context_buffer_snapshot
                                .text_for_range(suggestion.old_text_range)
                                .collect();
                            let new_text = context_buffer_snapshot
                                .text_for_range(suggestion.new_text_range)
                                .collect();
                            edits.push(Edit { old_text, new_text });
                        }
                    }
                }
            })?;

            let edits_by_buffer = cx
                .background_executor()
                .spawn(async move {
                    let mut result = HashMap::default();
                    for (buffer, (snapshot, suggestions)) in suggestions_by_buffer {
                        let edits =
                            result
                                .entry(buffer)
                                .or_insert(Vec::<(Range<language::Anchor>, _)>::new());
                        for suggestion in suggestions {
                            if let Some(range) =
                                fuzzy_search_lines(snapshot.as_rope(), &suggestion.old_text)
                            {
                                let edit_start = snapshot.anchor_after(range.start);
                                let edit_end = snapshot.anchor_before(range.end);
                                if let Err(ix) = edits.binary_search_by(|(range, _)| {
                                    range.start.cmp(&edit_start, &snapshot)
                                }) {
                                    edits.insert(
                                        ix,
                                        (edit_start..edit_end, suggestion.new_text.clone()),
                                    );
                                }
                            } else {
                                log::info!(
                                    "assistant edit did not match any text in buffer {:?}",
                                    &suggestion.old_text
                                );
                            }
                        }
                    }
                    result
                })
                .await;

            let mut project_transaction = ProjectTransaction::default();
            let (editor, workspace, title) = this.update(&mut cx, |this, cx| {
                for (buffer_handle, edits) in edits_by_buffer {
                    buffer_handle.update(cx, |buffer, cx| {
                        buffer.start_transaction();
                        buffer.edit(
                            edits,
                            Some(AutoindentMode::Block {
                                original_indent_columns: Vec::new(),
                            }),
                            cx,
                        );
                        buffer.end_transaction(cx);
                        if let Some(transaction) = buffer.finalize_last_transaction() {
                            project_transaction
                                .0
                                .insert(buffer_handle.clone(), transaction.clone());
                        }
                    });
                }

                (
                    this.editor.downgrade(),
                    this.workspace.clone(),
                    this.title(cx),
                )
            })?;

            Editor::open_project_transaction(
                &editor,
                workspace,
                project_transaction,
                format!("Edits from {}", title),
                cx,
            )
            .await
        })
        .detach_and_log_err(cx);
    }

    fn save(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        self.context
            .update(cx, |context, cx| context.save(None, self.fs.clone(), cx));
    }

    fn title(&self, cx: &AppContext) -> String {
        self.context
            .read(cx)
            .summary()
            .map(|summary| summary.text.clone())
            .unwrap_or_else(|| "New Context".into())
    }

    fn render_send_button(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle(cx).clone();
        ButtonLike::new("send_button")
            .style(ButtonStyle::Filled)
            .layer(ElevationIndex::ModalSurface)
            .children(
                KeyBinding::for_action_in(&Assist, &focus_handle, cx)
                    .map(|binding| binding.into_any_element()),
            )
            .child(Label::new("Send"))
            .on_click(move |_event, cx| {
                focus_handle.dispatch_action(&Assist, cx);
            })
    }
}

impl EventEmitter<ContextEditorEvent> for ContextEditor {}
impl EventEmitter<SearchEvent> for ContextEditor {}

impl Render for ContextEditor {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .key_context("ContextEditor")
            .capture_action(cx.listener(ContextEditor::cancel_last_assist))
            .capture_action(cx.listener(ContextEditor::save))
            .capture_action(cx.listener(ContextEditor::copy))
            .capture_action(cx.listener(ContextEditor::cycle_message_role))
            .capture_action(cx.listener(ContextEditor::confirm_command))
            .on_action(cx.listener(ContextEditor::assist))
            .on_action(cx.listener(ContextEditor::split))
            .on_action(cx.listener(ContextEditor::apply_edit))
            .size_full()
            .v_flex()
            .child(
                div()
                    .flex_grow()
                    .bg(cx.theme().colors().editor_background)
                    .child(self.editor.clone())
                    .child(
                        h_flex()
                            .w_full()
                            .absolute()
                            .bottom_0()
                            .p_4()
                            .justify_end()
                            .child(self.render_send_button(cx)),
                    ),
            )
    }
}

impl FocusableView for ContextEditor {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for ContextEditor {
    type Event = ContextEditorEvent;

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        cx: &WindowContext,
    ) -> AnyElement {
        let color = if params.selected {
            Color::Default
        } else {
            Color::Muted
        };
        Label::new(util::truncate_and_trailoff(
            &self.title(cx),
            Self::MAX_TAB_TITLE_LEN,
        ))
        .color(color)
        .into_any_element()
    }

    fn to_item_events(event: &Self::Event, mut f: impl FnMut(workspace::item::ItemEvent)) {
        match event {
            ContextEditorEvent::Edited => {
                f(workspace::item::ItemEvent::Edit);
                f(workspace::item::ItemEvent::UpdateBreadcrumbs);
            }
            ContextEditorEvent::TabContentChanged => {
                f(workspace::item::ItemEvent::UpdateTab);
            }
        }
    }

    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<SharedString> {
        Some(self.title(cx).into())
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn breadcrumbs(
        &self,
        theme: &theme::Theme,
        cx: &AppContext,
    ) -> Option<Vec<workspace::item::BreadcrumbText>> {
        let editor = self.editor.read(cx);
        let cursor = editor.selections.newest_anchor().head();
        let multibuffer = &editor.buffer().read(cx);
        let (_, symbols) = multibuffer.symbols_containing(cursor, Some(&theme.syntax()), cx)?;

        let settings = ThemeSettings::get_global(cx);

        let mut breadcrumbs = Vec::new();

        let title = self.title(cx);
        if title.chars().count() > Self::MAX_TAB_TITLE_LEN {
            breadcrumbs.push(BreadcrumbText {
                text: title,
                highlights: None,
                font: Some(settings.buffer_font.clone()),
            });
        }

        breadcrumbs.extend(symbols.into_iter().map(|symbol| BreadcrumbText {
            text: symbol.text,
            highlights: Some(symbol.highlight_ranges),
            font: Some(settings.buffer_font.clone()),
        }));
        Some(breadcrumbs)
    }

    fn breadcrumb_location(&self) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn set_nav_history(&mut self, nav_history: pane::ItemNavHistory, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            Item::set_nav_history(editor, nav_history, cx)
        })
    }

    fn navigate(&mut self, data: Box<dyn std::any::Any>, cx: &mut ViewContext<Self>) -> bool {
        self.editor
            .update(cx, |editor, cx| Item::navigate(editor, data, cx))
    }

    fn deactivated(&mut self, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| Item::deactivated(editor, cx))
    }
}

impl SearchableItem for ContextEditor {
    type Match = <Editor as SearchableItem>::Match;

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            editor.clear_matches(cx);
        });
    }

    fn update_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.update_matches(matches, cx));
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.editor
            .update(cx, |editor, cx| editor.query_suggestion(cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.activate_match(index, matches, cx);
        });
    }

    fn select_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.select_matches(matches, cx));
    }

    fn replace(
        &mut self,
        identifier: &Self::Match,
        query: &project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor
            .update(cx, |editor, cx| editor.replace(identifier, query, cx));
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Vec<Self::Match>> {
        self.editor
            .update(cx, |editor, cx| editor.find_matches(query, cx))
    }

    fn active_match_index(
        &mut self,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        self.editor
            .update(cx, |editor, cx| editor.active_match_index(matches, cx))
    }
}

pub struct ContextEditorToolbarItem {
    fs: Arc<dyn Fs>,
    workspace: WeakView<Workspace>,
    active_context_editor: Option<WeakView<ContextEditor>>,
    model_selector_menu_handle: PopoverMenuHandle<ContextMenu>,
}

impl ContextEditorToolbarItem {
    pub fn new(
        workspace: &Workspace,
        model_selector_menu_handle: PopoverMenuHandle<ContextMenu>,
    ) -> Self {
        Self {
            fs: workspace.app_state().fs.clone(),
            workspace: workspace.weak_handle(),
            active_context_editor: None,
            model_selector_menu_handle,
        }
    }

    fn render_inject_context_menu(&self, cx: &mut ViewContext<Self>) -> impl Element {
        let commands = SlashCommandRegistry::global(cx);
        let active_editor_focus_handle = self.workspace.upgrade().and_then(|workspace| {
            Some(
                workspace
                    .read(cx)
                    .active_item_as::<Editor>(cx)?
                    .focus_handle(cx),
            )
        });
        let active_context_editor = self.active_context_editor.clone();

        PopoverMenu::new("inject-context-menu")
            .trigger(IconButton::new("trigger", IconName::Quote).tooltip(|cx| {
                Tooltip::with_meta("Insert Context", None, "Type / to insert via keyboard", cx)
            }))
            .menu(move |cx| {
                let active_context_editor = active_context_editor.clone()?;
                ContextMenu::build(cx, |mut menu, _cx| {
                    for command_name in commands.featured_command_names() {
                        if let Some(command) = commands.command(&command_name) {
                            let menu_text = SharedString::from(Arc::from(command.menu_text()));
                            menu = menu.custom_entry(
                                {
                                    let command_name = command_name.clone();
                                    move |_cx| {
                                        h_flex()
                                            .w_full()
                                            .justify_between()
                                            .child(Label::new(menu_text.clone()))
                                            .child(
                                                div().ml_4().child(
                                                    Label::new(format!("/{command_name}"))
                                                        .color(Color::Muted),
                                                ),
                                            )
                                            .into_any()
                                    }
                                },
                                {
                                    let active_context_editor = active_context_editor.clone();
                                    move |cx| {
                                        active_context_editor
                                            .update(cx, |context_editor, cx| {
                                                context_editor.insert_command(&command_name, cx)
                                            })
                                            .ok();
                                    }
                                },
                            )
                        }
                    }

                    if let Some(active_editor_focus_handle) = active_editor_focus_handle.clone() {
                        menu = menu
                            .context(active_editor_focus_handle)
                            .action("Quote Selection", Box::new(QuoteSelection));
                    }

                    menu
                })
                .into()
            })
    }

    fn render_remaining_tokens(&self, cx: &mut ViewContext<Self>) -> Option<impl IntoElement> {
        let model = CompletionProvider::global(cx).model();
        let context = &self
            .active_context_editor
            .as_ref()?
            .upgrade()?
            .read(cx)
            .context;
        let token_count = context.read(cx).token_count()?;
        let max_token_count = model.max_token_count();

        let remaining_tokens = max_token_count as isize - token_count as isize;
        let token_count_color = if remaining_tokens <= 0 {
            Color::Error
        } else if token_count as f32 / max_token_count as f32 >= 0.8 {
            Color::Warning
        } else {
            Color::Muted
        };

        Some(
            h_flex()
                .gap_0p5()
                .child(
                    Label::new(humanize_token_count(token_count))
                        .size(LabelSize::Small)
                        .color(token_count_color),
                )
                .child(Label::new("/").size(LabelSize::Small).color(Color::Muted))
                .child(
                    Label::new(humanize_token_count(max_token_count))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
    }
}

impl Render for ContextEditorToolbarItem {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        h_flex()
            .gap_2()
            .child(ModelSelector::new(
                self.model_selector_menu_handle.clone(),
                self.fs.clone(),
            ))
            .children(self.render_remaining_tokens(cx))
            .child(self.render_inject_context_menu(cx))
    }
}

impl ToolbarItemView for ContextEditorToolbarItem {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> ToolbarItemLocation {
        self.active_context_editor = active_pane_item
            .and_then(|item| item.act_as::<ContextEditor>(cx))
            .map(|editor| editor.downgrade());
        cx.notify();
        if self.active_context_editor.is_none() {
            ToolbarItemLocation::Hidden
        } else {
            ToolbarItemLocation::PrimaryRight
        }
    }

    fn pane_focus_update(&mut self, _pane_focused: bool, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl EventEmitter<ToolbarItemEvent> for ContextEditorToolbarItem {}

pub struct ContextHistory {
    picker: View<Picker<SavedContextPickerDelegate>>,
    _subscriptions: Vec<Subscription>,
    assistant_panel: WeakView<AssistantPanel>,
}

impl ContextHistory {
    fn new(
        context_store: Model<ContextStore>,
        assistant_panel: WeakView<AssistantPanel>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let picker = cx.new_view(|cx| {
            Picker::uniform_list(SavedContextPickerDelegate::new(context_store.clone()), cx)
                .modal(false)
                .max_height(None)
        });

        let _subscriptions = vec![
            cx.observe(&context_store, |this, _, cx| {
                this.picker.update(cx, |picker, cx| picker.refresh(cx));
            }),
            cx.subscribe(&picker, Self::handle_picker_event),
        ];

        Self {
            picker,
            _subscriptions,
            assistant_panel,
        }
    }

    fn handle_picker_event(
        &mut self,
        _: View<Picker<SavedContextPickerDelegate>>,
        event: &SavedContextPickerEvent,
        cx: &mut ViewContext<Self>,
    ) {
        let SavedContextPickerEvent::Confirmed { path } = event;
        self.assistant_panel
            .update(cx, |assistant_panel, cx| {
                assistant_panel
                    .open_context(path.clone(), cx)
                    .detach_and_log_err(cx);
            })
            .ok();
    }
}

impl Render for ContextHistory {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div().size_full().child(self.picker.clone())
    }
}

impl FocusableView for ContextHistory {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<()> for ContextHistory {}

impl Item for ContextHistory {
    type Event = ();

    fn tab_content(
        &self,
        params: workspace::item::TabContentParams,
        _: &WindowContext,
    ) -> AnyElement {
        let color = if params.selected {
            Color::Default
        } else {
            Color::Muted
        };
        Label::new("History").color(color).into_any_element()
    }
}

type ToggleFold = Arc<dyn Fn(bool, &mut WindowContext) + Send + Sync>;

fn render_slash_command_output_toggle(
    row: MultiBufferRow,
    is_folded: bool,
    fold: ToggleFold,
    _cx: &mut WindowContext,
) -> AnyElement {
    Disclosure::new(("slash-command-output-fold-indicator", row.0), !is_folded)
        .selected(is_folded)
        .on_click(move |_e, cx| fold(!is_folded, cx))
        .into_any_element()
}

fn render_pending_slash_command_gutter_decoration(
    row: MultiBufferRow,
    status: &PendingSlashCommandStatus,
    confirm_command: Arc<dyn Fn(&mut WindowContext)>,
) -> AnyElement {
    let mut icon = IconButton::new(
        ("slash-command-gutter-decoration", row.0),
        ui::IconName::TriangleRight,
    )
    .on_click(move |_e, cx| confirm_command(cx))
    .icon_size(ui::IconSize::Small)
    .size(ui::ButtonSize::None);

    match status {
        PendingSlashCommandStatus::Idle => {
            icon = icon.icon_color(Color::Muted);
        }
        PendingSlashCommandStatus::Running { .. } => {
            icon = icon.selected(true);
        }
        PendingSlashCommandStatus::Error(_) => icon = icon.icon_color(Color::Error),
    }

    icon.into_any_element()
}

fn render_docs_slash_command_trailer(
    row: MultiBufferRow,
    command: PendingSlashCommand,
    cx: &mut WindowContext,
) -> AnyElement {
    let Some(argument) = command.argument else {
        return Empty.into_any();
    };

    let args = DocsSlashCommandArgs::parse(&argument);

    let Some(store) = args
        .provider()
        .and_then(|provider| IndexedDocsStore::try_global(provider, cx).ok())
    else {
        return Empty.into_any();
    };

    let Some(package) = args.package() else {
        return Empty.into_any();
    };

    if !store.is_indexing(&package) {
        return Empty.into_any();
    }

    div()
        .id(("crates-being-indexed", row.0))
        .child(Icon::new(IconName::ArrowCircle).with_animation(
            "arrow-circle",
            Animation::new(Duration::from_secs(4)).repeat(),
            |icon, delta| icon.transform(Transformation::rotate(percentage(delta))),
        ))
        .tooltip(move |cx| Tooltip::text(format!("Indexing {package}…"), cx))
        .into_any_element()
}

fn make_lsp_adapter_delegate(
    project: &Model<Project>,
    cx: &mut AppContext,
) -> Result<Arc<dyn LspAdapterDelegate>> {
    project.update(cx, |project, cx| {
        // TODO: Find the right worktree.
        let worktree = project
            .worktrees()
            .next()
            .ok_or_else(|| anyhow!("no worktrees when constructing ProjectLspAdapterDelegate"))?;
        Ok(ProjectLspAdapterDelegate::new(project, &worktree, cx) as Arc<dyn LspAdapterDelegate>)
    })
}

fn slash_command_error_block_renderer(message: String) -> RenderBlock {
    Box::new(move |_| {
        div()
            .pl_6()
            .child(
                Label::new(format!("error: {}", message))
                    .single_line()
                    .color(Color::Error),
            )
            .into_any()
    })
}
