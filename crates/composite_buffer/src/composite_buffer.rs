use anyhow::Result;
use gpui::{Entity, ModelContext, ModelHandle, Task};
use language::{
    Buffer, Diagnostic, File, FromAnchor, Language, Point, Selection, SelectionSetId, ToOffset,
    Tree,
};
use std::{
    ops::Range,
    sync::Arc,
    time::{Instant, SystemTime},
};

pub struct CompositeBuffer {}

pub struct Snapshot {}

impl CompositeBuffer {
    pub fn singleton(buffer: ModelHandle<Buffer>) -> Self {
        todo!()
    }

    pub fn snapshot(&self) -> Snapshot {
        todo!()
    }

    pub fn save(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Result<Task<Result<(clock::Global, SystemTime)>>> {
        todo!()
    }

    pub fn set_language(
        &mut self,
        language: Option<Arc<Language>>,
        language_server: Option<Arc<lsp::LanguageServer>>,
        cx: &mut ModelContext<Self>,
    ) {
    }

    pub fn did_save(
        &mut self,
        version: clock::Global,
        mtime: SystemTime,
        new_file: Option<Box<dyn File>>,
        cx: &mut ModelContext<Self>,
    ) {
    }

    pub fn close(&mut self, cx: &mut ModelContext<Self>) {
        // cx.emit(Event::Closed);
    }

    pub fn language(&self, position: Point) -> Option<&Arc<Language>> {
        todo!()
    }

    pub fn parse_count(&self) -> usize {
        todo!()
    }

    pub(crate) fn syntax_tree(&self, position: Point) -> Option<Tree> {
        todo!()
    }

    pub fn diagnostics_in_range<'a, T, O>(
        &'a self,
        search_range: Range<T>,
    ) -> impl Iterator<Item = (Range<O>, &Diagnostic)> + 'a
    where
        T: 'a + ToOffset,
        O: 'a + FromAnchor,
    {
        Vec::new().into_iter()
    }

    pub fn diagnostic_group<'a, O>(
        &'a self,
        group_id: usize,
    ) -> impl Iterator<Item = (Range<O>, &Diagnostic)> + 'a
    where
        O: 'a + FromAnchor,
    {
        Vec::new().into_iter()
    }

    pub fn diagnostics_update_count(&self) -> usize {
        todo!()
    }

    pub fn indent_column_for_line(&self, row: u32) -> u32 {
        todo!()
    }

    pub fn range_for_syntax_ancestor<T: ToOffset>(&self, range: Range<T>) -> Option<Range<usize>> {
        todo!()
    }

    pub fn enclosing_bracket_ranges<T: ToOffset>(
        &self,
        range: Range<T>,
    ) -> Option<(Range<usize>, Range<usize>)> {
        todo!()
    }

    pub fn is_dirty(&self) -> bool {
        todo!()
    }

    pub fn has_conflict(&self) -> bool {
        todo!()
    }

    pub fn start_transaction(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
    ) -> Result<()> {
        todo!()
    }

    pub(crate) fn start_transaction_at(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
        now: Instant,
    ) -> Result<()> {
        todo!()
    }

    pub fn end_transaction(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub(crate) fn end_transaction_at(
        &mut self,
        selection_set_ids: impl IntoIterator<Item = SelectionSetId>,
        now: Instant,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn edit<I, S, T>(&mut self, ranges_iter: I, new_text: T, cx: &mut ModelContext<Self>)
    where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        todo!()
    }

    pub fn edit_with_autoindent<I, S, T>(
        &mut self,
        ranges_iter: I,
        new_text: T,
        cx: &mut ModelContext<Self>,
    ) where
        I: IntoIterator<Item = Range<S>>,
        S: ToOffset,
        T: Into<String>,
    {
        todo!()
    }

    pub fn add_selection_set<T: ToOffset>(
        &mut self,
        selections: &[Selection<T>],
        cx: &mut ModelContext<Self>,
    ) -> SelectionSetId {
        todo!()
    }

    pub fn update_selection_set<T: ToOffset>(
        &mut self,
        set_id: SelectionSetId,
        selections: &[Selection<T>],
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn set_active_selection_set(
        &mut self,
        set_id: Option<SelectionSetId>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn remove_selection_set(
        &mut self,
        set_id: SelectionSetId,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        todo!()
    }

    pub fn undo(&mut self, cx: &mut ModelContext<Self>) {
        todo!()
    }

    pub fn redo(&mut self, cx: &mut ModelContext<Self>) {
        todo!()
    }
}

impl Entity for CompositeBuffer {
    type Event = ();
}
