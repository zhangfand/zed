use crate::Workspace;
use gpui::{
    geometry::rect::RectF, Element, LayoutContext, MeasurementContext, PaintContext,
    SizeConstraint, WeakViewHandle,
};

pub struct WorkspaceElement(WeakViewHandle<Workspace>);
impl Element for WorkspaceElement {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (gpui::geometry::vector::Vector2F, Self::LayoutState) {
        todo!()
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        todo!()
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        cx: &gpui::DebugContext,
    ) -> serde_json::Value {
        todo!()
    }
}
