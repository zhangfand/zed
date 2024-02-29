use fuzzy::{match_strings, StringMatch, StringMatchCandidate};
use gpui::{
    actions, AppContext, DismissEvent, EventEmitter, FocusHandle, FocusableView, Model,
    ParentElement, Render, Styled, View, ViewContext, VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use project::Worktree;
use python::{python_settings::PythonSettings, Interpreter};
use settings::Settings;
use std::{path::PathBuf, sync::Arc};
use ui::{prelude::*, HighlightedLabel, ListItem, ListItemSpacing};
use util::ResultExt;
use workspace::{ModalView, Workspace};

actions!(interpreter_selector, [Toggle]);

pub fn init(cx: &mut AppContext) {
    PythonSettings::register(cx);
    cx.observe_new_views(InterpreterSelector::register).detach();
}

pub struct InterpreterSelector {
    picker: View<Picker<InterpreterSelectorDelegate>>,
}

impl InterpreterSelector {
    fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
        let Some(worktree) = workspace.worktrees(cx).next() else {
            return;
        };

        workspace.register_action(move |workspace, _: &Toggle, cx| {
            Self::toggle(workspace, worktree.clone(), cx);
        });
    }

    fn toggle(
        workspace: &mut Workspace,
        worktree: Model<Worktree>,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<()> {
        workspace.toggle_modal(cx, move |cx| InterpreterSelector::new(worktree, cx));
        Some(())
    }

    fn new(worktree: Model<Worktree>, cx: &mut ViewContext<Self>) -> Self {
        let delegate = InterpreterSelectorDelegate::new(cx.view().downgrade(), worktree, cx);
        let picker = cx.new_view(|cx| Picker::uniform_list(delegate, cx));
        Self { picker }
    }
}

impl Render for InterpreterSelector {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex().w(rems(34.)).child(self.picker.clone())
    }
}

impl FocusableView for InterpreterSelector {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl EventEmitter<DismissEvent> for InterpreterSelector {}
impl ModalView for InterpreterSelector {}

pub struct InterpreterSelectorDelegate {
    worktree: Model<Worktree>,
    interpreter_selector: WeakView<InterpreterSelector>,
    candidates: Vec<StringMatchCandidate>,
    matches: Vec<StringMatch>,
    selected_index: usize,
}

impl InterpreterSelectorDelegate {
    fn new(
        interpreter_selector: WeakView<InterpreterSelector>,
        worktree: Model<Worktree>,
        cx: &mut AppContext,
    ) -> Self {
        let worktree_path = worktree.update(cx, |worktree, _| worktree.abs_path());
        let settings = PythonSettings::get_global(cx);
        let candidates = Interpreter::find_interpreters(worktree_path, settings)
            .iter()
            .enumerate()
            .map(|(candidate_id, interpeter)| {
                let path = interpeter
                    .interpreter_path()
                    .as_os_str()
                    .to_string_lossy()
                    .to_string();
                StringMatchCandidate::new(candidate_id, path)
            })
            .collect::<Vec<_>>();

        Self {
            worktree,
            interpreter_selector,
            candidates,
            matches: vec![],
            selected_index: 0,
        }
    }
}

impl PickerDelegate for InterpreterSelectorDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
        "Select an interpreter...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(mat) = self.matches.get(self.selected_index) {
            let interpreter_path_string = self.candidates[mat.candidate_id].string.clone();
            let interpeter_path = PathBuf::from(interpreter_path_string);
            let worktree_path = self.worktree.update(cx, |worktree, _| worktree.abs_path());
            Interpreter::store_in_local_settings(worktree_path, interpeter_path, cx);
        }
        self.dismissed(cx);
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
        self.interpreter_selector
            .update(cx, |_, cx| cx.emit(DismissEvent))
            .log_err();
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        cx: &mut ViewContext<Picker<Self>>,
    ) -> gpui::Task<()> {
        let background = cx.background_executor().clone();
        let candidates = self.candidates.clone();
        cx.spawn(|this, mut cx| async move {
            let matches = if query.is_empty() {
                candidates
                    .into_iter()
                    .enumerate()
                    .map(|(index, candidate)| StringMatch {
                        candidate_id: index,
                        string: candidate.string,
                        positions: Vec::new(),
                        score: 0.0,
                    })
                    .collect()
            } else {
                match_strings(
                    &candidates,
                    &query,
                    false,
                    100,
                    &Default::default(),
                    background,
                )
                .await
            };

            this.update(&mut cx, |this, cx| {
                let delegate = &mut this.delegate;
                delegate.matches = matches;
                delegate.selected_index = delegate
                    .selected_index
                    .min(delegate.matches.len().saturating_sub(1));
                cx.notify();
            })
            .log_err();
        })
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let mat = &self.matches[ix];
        let label = mat.string.clone();

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
                .child(HighlightedLabel::new(label, mat.positions.clone())),
        )
    }
}
