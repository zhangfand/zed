use anyhow::Result;
use clock::ReplicaId;
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task};
use language::{
    rope::TextDimension, Anchor, AnchorRangeSet, Bias, Buffer, Chunk, Diagnostic, File, FromAnchor,
    Language, Patch, Point, PointUtf16, Selection, SelectionSetId, Subscription, TextSummary, Tree,
};
use std::{
    cmp::Ordering,
    io,
    ops::{Deref, Range},
    sync::Arc,
    time::{Instant, SystemTime},
};
use sum_tree::SumTree;
use theme::SyntaxTheme;

pub use language::Event;

pub struct CompositeBuffer {
    snapshot: Snapshot,
    excerpt_sets: Vec<ExcerptSet>,
    singleton: bool,
}

#[derive(Clone)]
pub struct Snapshot {
    excerpts: SumTree<Excerpt>,
}

struct ExcerptSet {
    buffer: ModelHandle<Buffer>,
    subscription: Subscription,
    ranges: AnchorRangeSet,
}

#[derive(Clone)]
struct Excerpt {
    buffer: ModelHandle<Buffer>,
    rows: Range<u32>,
    summary: TextSummary,
    include_buffer_header: bool,
}

pub struct CompositeBufferSubscription {}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeAnchor {}

#[derive(Debug, Clone)]
pub struct CompositeAnchorRangeSet {}

#[derive(Debug, Clone)]
pub struct CompositeSelectionSet {
    pub active: bool,
}

pub struct Chunks<'a> {
    cursor: sum_tree::Cursor<'a, Excerpt, usize>,
    range: Range<usize>,
    buffer_chunks: language::Chunks<'a>,
    cx: &'a AppContext,
}

pub struct Bytes<'a> {
    snapshot: &'a Snapshot,
}

pub trait ToPoint {
    fn to_point<'a>(&self, content: &Snapshot) -> Point;
}

pub trait ToOffset {
    fn to_offset<'a>(&self, content: &Snapshot) -> usize;
}

impl CompositeBuffer {
    pub fn new<T>(
        iter: impl IntoIterator<Item = (ModelHandle<Buffer>, Vec<Range<T>>)>,
        cx: &mut ModelContext<Self>,
    ) -> Self
    where
        T: language::ToPoint + language::ToOffset,
    {
        let mut excerpts = SumTree::new();
        let mut excerpt_sets = Vec::new();
        for (buffer_handle, ranges) in iter {
            buffer_handle.update(cx, |buffer, cx| {
                for (ix, range) in ranges.iter().enumerate() {
                    let rows = range.start.to_point(buffer).row..range.end.to_point(buffer).row + 1;
                    excerpts.push(Excerpt::new(buffer_handle.clone(), rows, ix == 0, cx), &());
                }

                let ranges = buffer.anchor_range_set(
                    Bias::Left,
                    Bias::Right,
                    ranges
                        .iter()
                        .map(|range| range.start.to_offset(buffer)..range.end.to_offset(buffer)),
                );

                excerpt_sets.push(ExcerptSet {
                    buffer: buffer_handle.clone(),
                    subscription: buffer.subscribe(),
                    ranges,
                });
            })
        }

        Self {
            snapshot: Snapshot { excerpts },
            excerpt_sets,
            singleton: false,
        }
    }

    pub fn singleton(buffer: ModelHandle<Buffer>, cx: &mut ModelContext<Self>) -> Self {
        let buffer_len = buffer.read(cx).len();
        let mut this = Self::new([(buffer, vec![0..buffer_len])], cx);
        this.singleton = true;
        this
    }

    pub fn subscribe(&mut self) -> CompositeBufferSubscription {
        todo!()
    }

    pub fn snapshot(&self) -> Snapshot {
        todo!()
    }

    pub fn file(&self) -> Option<&dyn File> {
        None
    }

    pub fn replica_id(&self) -> ReplicaId {
        todo!()
    }

    pub fn save(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Result<Task<Result<(clock::Global, SystemTime)>>> {
        todo!()
    }

    pub fn close(&mut self, cx: &mut ModelContext<Self>) {
        // cx.emit(Event::Closed);
    }

    pub fn language<T: ToOffset>(&self, position: T) -> Option<&Arc<Language>> {
        todo!()
    }

    pub fn parse_count(&self) -> usize {
        todo!()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn is_parsing(&self) -> bool {
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

    pub fn selection_set(&self, set_id: SelectionSetId) -> Result<&CompositeSelectionSet> {
        todo!()
    }

    pub fn selection_sets(
        &self,
    ) -> impl Iterator<Item = (&SelectionSetId, &CompositeSelectionSet)> {
        todo!();
        None.into_iter()
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

#[cfg(any(test, feature = "test-support"))]
impl CompositeBuffer {
    pub fn randomly_edit<T>(&mut self, rng: &mut T, old_range_count: usize)
    where
        T: rand::Rng,
    {
        todo!()
    }

    pub fn randomly_mutate<T>(&mut self, rng: &mut T)
    where
        T: rand::Rng,
    {
        todo!()
    }
}

impl Excerpt {
    fn new(
        buffer: ModelHandle<Buffer>,
        rows: Range<u32>,
        include_buffer_header: bool,
        cx: &AppContext,
    ) -> Self {
        let mut summary = buffer.read(cx).text_summary_for_range::<TextSummary, _>(
            Point::new(rows.start, 0)..Point::new(rows.end, 0),
        );

        if include_buffer_header {
            summary.bytes += 1;
            summary.lines.row += 1;
            summary.lines_utf16.row += 1;
        }
        summary.bytes += 1;
        summary.lines.row += 1;
        summary.lines_utf16.row += 1;
        summary.first_line_chars = 0;

        Self {
            buffer,
            rows,
            summary,
            include_buffer_header,
        }
    }
}

impl sum_tree::Item for Excerpt {
    type Summary = TextSummary;

    fn summary(&self) -> Self::Summary {
        self.summary.clone()
    }
}

impl Snapshot {
    pub fn text(&self, cx: &AppContext) -> String {
        self.chunks(0..self.len(), None, cx)
            .map(|chunk| chunk.text)
            .collect()
    }

    pub fn text_summary(&self) -> TextSummary {
        todo!()
    }

    pub fn text_summary_for_range<'a, D, O: ToOffset>(&'a self, range: Range<O>) -> D
    where
        D: TextDimension<'a>,
    {
        todo!()
    }

    pub fn clip_offset(&self, offset: usize, bias: Bias) -> usize {
        todo!()
    }

    pub fn clip_point(&self, point: Point, bias: Bias) -> Point {
        todo!()
    }

    pub fn clip_point_utf16(&self, point: PointUtf16, bias: Bias) -> PointUtf16 {
        todo!()
    }

    pub fn text_for_range<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        cx: &'a AppContext,
    ) -> impl Iterator<Item = &'a str> {
        self.chunks(range, None, cx).map(|chunk| chunk.text)
    }

    pub fn contains_str_at<T>(&self, position: T, needle: &str) -> bool
    where
        T: ToOffset,
    {
        todo!()
    }

    pub fn anchor_before<T: ToOffset>(&self, offset: T) -> CompositeAnchor {
        self.anchor_at(offset, Bias::Left)
    }

    pub fn anchor_after<T: ToOffset>(&self, offset: T) -> CompositeAnchor {
        self.anchor_at(offset, Bias::Right)
    }

    pub fn anchor_at<T: ToOffset>(&self, offset: T, bias: Bias) -> CompositeAnchor {
        todo!()
    }

    pub fn anchor_range_set<E>(
        &self,
        start_bias: Bias,
        end_bias: Bias,
        entries: E,
    ) -> CompositeAnchorRangeSet
    where
        E: IntoIterator<Item = Range<usize>>,
    {
        todo!()
    }

    pub fn to_offset(&self, point: Point) -> usize {
        todo!()
    }

    pub fn to_point(&self, offset: usize) -> Point {
        todo!()
    }

    pub fn point_for_offset(&self, offset: usize) -> Result<Point> {
        todo!()
    }

    pub fn selection_point_range<T: ToPoint>(&self, selection: &Selection<T>) -> Range<Point> {
        todo!()
    }

    pub fn len(&self) -> usize {
        self.excerpts.summary().bytes
    }

    pub fn line_len(&self, row: u32) -> u32 {
        todo!()
    }

    pub fn is_line_blank(&self, row: u32) -> bool {
        todo!()
    }

    pub fn max_point(&self) -> Point {
        todo!()
    }

    pub fn chunks<'a, T: ToOffset>(
        &'a self,
        range: Range<T>,
        theme: Option<&'a SyntaxTheme>,
        cx: &'a AppContext,
    ) -> Chunks<'a> {
        let range = range.start.to_offset(self)..range.end.to_offset(self);
        let mut cursor = self.excerpts.cursor::<usize>();
        cursor.seek(&range.start, Bias::Left, &());
        let excerpt = cursor.item().unwrap();
        let buffer = excerpt.buffer.read(cx);

        let overshoot = range.start - cursor.start();
        let excerpt_start_point = Point::new(excerpt.rows.start, 0);
        let excerpt_start_offset = excerpt_start_point.to_offset(buffer);
        let start_offset = excerpt_start_offset + overshoot;
        let end_offset = excerpt_start_offset + cmp::min(range.len(), excerpt.summary.bytes);
        let buffer_chunks = buffer.chunks(start_offset..end_offset, theme);

        Chunks {
            cursor,
            range,
            buffer_chunks,
            cx,
        }
    }

    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        self.chars_at(0)
    }

    pub fn chars_at<'a, T: ToOffset>(&'a self, position: T) -> impl Iterator<Item = char> + 'a {
        todo!();
        None.into_iter()
    }

    pub fn reversed_chars_at<'a, T: ToOffset>(
        &'a self,
        position: T,
    ) -> impl Iterator<Item = char> + 'a {
        todo!();
        None.into_iter()
    }

    pub fn bytes_in_range<T: ToOffset>(&self, range: Range<T>) -> Bytes {
        todo!()
    }

    pub fn chars_for_range<T: ToOffset>(&self, range: Range<T>) -> impl Iterator<Item = char> + '_ {
        todo!();
        None.into_iter()
    }

    pub fn diagnostics_update_count(&self) -> usize {
        todo!()
    }

    pub fn parse_count(&self) -> usize {
        todo!()
    }
}

impl CompositeBufferSubscription {
    pub fn consume(&self) -> Patch<usize> {
        todo!()
    }
}

impl CompositeAnchor {
    pub fn min() -> Self {
        todo!()
    }

    pub fn max() -> Self {
        todo!()
    }

    pub fn cmp(&self, other: &Self, buffer: &Snapshot) -> Result<Ordering> {
        todo!()
    }

    pub fn summary<'a, D>(&self, content: &'a Snapshot) -> D
    where
        D: TextDimension<'a>,
    {
        todo!()
    }
}

impl CompositeAnchorRangeSet {
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn ranges<'a, D>(&'a self, content: &'a Snapshot) -> impl 'a + Iterator<Item = Range<Point>>
    where
        D: 'a + TextDimension<'a>,
    {
        todo!();
        None.into_iter()
    }
}

impl CompositeSelectionSet {
    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn selections<'a, D>(
        &'a self,
        content: &'a Snapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: 'a + TextDimension<'a>,
    {
        todo!();
        None.into_iter()
    }

    pub fn intersecting_selections<'a, D, I>(
        &'a self,
        range: Range<(I, Bias)>,
        content: &'a Snapshot,
    ) -> impl 'a + Iterator<Item = Selection<D>>
    where
        D: 'a + TextDimension<'a>,
        I: 'a + ToOffset,
    {
        todo!();
        None.into_iter()
    }

    pub fn oldest_selection<'a, D>(&'a self, content: &'a Snapshot) -> Option<Selection<D>>
    where
        D: 'a + TextDimension<'a>,
    {
        todo!()
    }

    pub fn newest_selection<'a, D>(&'a self, content: &'a Snapshot) -> Option<Selection<D>>
    where
        D: 'a + TextDimension<'a>,
    {
        todo!()
    }
}

impl<'a> Iterator for Chunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> Chunks<'a> {
    pub fn seek(&mut self, offset: usize) {
        todo!()
    }

    pub fn offset(&self) -> usize {
        todo!()
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a> io::Read for Bytes<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}

impl Deref for CompositeBuffer {
    type Target = Snapshot;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl Entity for CompositeBuffer {
    type Event = language::Event;
}

impl ToPoint for Point {
    fn to_point<'a>(&self, _: &Snapshot) -> Point {
        *self
    }
}

impl ToPoint for usize {
    fn to_point<'a>(&self, _: &Snapshot) -> Point {
        todo!()
    }
}

impl ToPoint for CompositeAnchor {
    fn to_point<'a>(&self, _: &Snapshot) -> Point {
        todo!()
    }
}

impl ToOffset for Point {
    fn to_offset<'a>(&self, content: &Snapshot) -> usize {
        todo!()
    }
}

impl ToOffset for usize {
    fn to_offset<'a>(&self, content: &Snapshot) -> usize {
        todo!()
    }
}

impl ToOffset for CompositeAnchor {
    fn to_offset<'a>(&self, content: &Snapshot) -> usize {
        todo!()
    }
}

pub trait AnchorRangeExt {
    fn cmp(&self, b: &Range<CompositeAnchor>, buffer: &Snapshot) -> Result<Ordering>;
    fn to_offset(&self, content: &Snapshot) -> Range<usize>;
}

impl AnchorRangeExt for Range<CompositeAnchor> {
    fn cmp(&self, other: &Range<CompositeAnchor>, buffer: &Snapshot) -> Result<Ordering> {
        Ok(match self.start.cmp(&other.start, buffer)? {
            Ordering::Equal => other.end.cmp(&self.end, buffer)?,
            ord @ _ => ord,
        })
    }

    fn to_offset(&self, content: &Snapshot) -> Range<usize> {
        self.start.to_offset(&content)..self.end.to_offset(&content)
    }
}

#[cfg(test)]
mod tests {
    use gpui::MutableAppContext;
    use language::Buffer;

    use crate::CompositeBuffer;

    #[gpui::test]
    fn test_singleton_composite_buffer(cx: &mut MutableAppContext) {
        let buffer = cx.add_model(|cx| Buffer::new(0, "abc", cx));
        let composite = cx.add_model(|cx| CompositeBuffer::singleton(buffer, cx));
        assert_eq!(composite.read(cx).text(), "abc");
    }

    #[gpui::test]
    fn test_multi_composite_buffer(cx: &mut MutableAppContext) {
        // cx.add_model(build_model)
    }
}
