use std::ops::DerefMut;

use gpui::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    CursorRegion, CursorStyle, MouseButton, MouseRegion, PaintContext, Quad, View, ViewContext,
    WeakViewHandle,
};

pub trait ScrollableLayout {
    fn font_size(&self) -> f32;

    fn max_row(&self) -> u32;

    fn height_in_rows(&self) -> f32;

    fn scroll_position(&self) -> Vector2F;

    fn show_scrollbars(&self) -> bool;
}

pub trait ScrollableView: View {
    fn scroll_position(&self, cx: &mut ViewContext<Self>) -> Vector2F;

    fn set_scroll_position(&mut self, pos: Vector2F, cx: &mut ViewContext<Self>);

    fn make_scrollbar_visible(&mut self, cx: &mut ViewContext<Self>);
}

pub fn paint_scrollbar<L, V>(
    layout: &L,
    view: WeakViewHandle<V>,
    bounds: RectF,
    theme: &theme::Editor,
    cx: &mut PaintContext,
) where
    L: ScrollableLayout,
    V: ScrollableView,
{
    enum ScrollbarMouseHandlers {}

    let style = &theme.scrollbar;

    let top = bounds.min_y();
    let bottom = bounds.max_y();
    let right = bounds.max_x();
    let left = right - style.width;
    let scroll_y = layout.scroll_position().y();
    let row_range = scroll_y..(scroll_y + layout.height_in_rows());
    let max_row = layout.max_row() as f32 + (row_range.end - row_range.start);

    let mut height = bounds.height();
    let mut first_row_y_offset = 0.0;

    // Impose a minimum height on the scrollbar thumb
    let min_thumb_height = style.min_height_factor * cx.font_cache.line_height(layout.font_size());
    let thumb_height = (row_range.end - row_range.start) * height / max_row;
    if thumb_height < min_thumb_height {
        first_row_y_offset = (min_thumb_height - thumb_height) / 2.0;
        height -= min_thumb_height - thumb_height;
    }

    let y_for_row = |row: f32| -> f32 { top + first_row_y_offset + row * height / max_row };

    let thumb_top = y_for_row(row_range.start) - first_row_y_offset;
    let thumb_bottom = y_for_row(row_range.end) + first_row_y_offset;
    let track_bounds = RectF::from_points(vec2f(left, top), vec2f(right, bottom));
    let thumb_bounds = RectF::from_points(vec2f(left, thumb_top), vec2f(right, thumb_bottom));

    if layout.show_scrollbars() {
        cx.scene.push_quad(Quad {
            bounds: track_bounds,
            border: style.track.border,
            background: style.track.background_color,
            ..Default::default()
        });
        cx.scene.push_quad(Quad {
            bounds: thumb_bounds,
            border: style.thumb.border,
            background: style.thumb.background_color,
            corner_radius: style.thumb.corner_radius,
        });
    }

    cx.scene.push_cursor_region(CursorRegion {
        bounds: track_bounds,
        style: CursorStyle::Arrow,
    });
    cx.scene.push_mouse_region(
        MouseRegion::new::<ScrollbarMouseHandlers>(view.id(), view.id(), track_bounds)
            .on_move({
                let view = view.clone();
                move |_, cx| {
                    if let Some(view) = view.upgrade(cx.deref_mut()) {
                        view.update(cx.deref_mut(), |view, cx| {
                            view.make_scrollbar_visible(cx);
                        });
                    }
                }
            })
            .on_down(MouseButton::Left, {
                let view = view.clone();
                let row_range = row_range.clone();
                move |e, cx| {
                    let y = e.position.y();
                    if let Some(view) = view.upgrade(cx.deref_mut()) {
                        view.update(cx.deref_mut(), |view, cx| {
                            if y < thumb_top || thumb_bottom < y {
                                let center_row =
                                    ((y - top) * max_row as f32 / height).round() as u32;
                                let top_row = center_row
                                    .saturating_sub((row_range.end - row_range.start) as u32 / 2);
                                let mut position = view.scroll_position(cx);
                                position.set_y(top_row as f32);
                                view.set_scroll_position(position, cx);
                            } else {
                                view.make_scrollbar_visible(cx);
                            }
                        });
                    }
                }
            })
            .on_drag(MouseButton::Left, {
                let view = view.clone();
                move |e, cx| {
                    let y = e.prev_mouse_position.y();
                    let new_y = e.position.y();
                    if thumb_top < y && y < thumb_bottom {
                        if let Some(view) = view.upgrade(cx.deref_mut()) {
                            view.update(cx.deref_mut(), |view, cx| {
                                let mut position = view.scroll_position(cx);
                                position
                                    .set_y(position.y() + (new_y - y) * (max_row as f32) / height);
                                if position.y() < 0.0 {
                                    position.set_y(0.);
                                }
                                view.set_scroll_position(position, cx);
                            });
                        }
                    }
                }
            }),
    );
}
