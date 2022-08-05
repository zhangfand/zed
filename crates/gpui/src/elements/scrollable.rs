use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    json::{self, ToJson},
    presenter::MeasurementContext,
    DebugContext, Element, ElementBox, ElementStateHandle, Event, EventContext, LayoutContext,
    PaintContext, RenderContext, SizeConstraint, View,
};

pub struct Scrollable {
    child: ElementBox,
    horizontal: bool,
    vertical: bool,
    state: ElementStateHandle<State>,
}

#[derive(Default)]
struct State {
    scroll: Vector2F,
}

impl Scrollable {
    pub fn new<Tag, V>(id: usize, child: ElementBox, cx: &mut RenderContext<V>) -> Self
    where
        Tag: 'static,
        V: View,
    {
        Self {
            child,
            horizontal: false,
            vertical: false,
            state: cx.element_state::<Tag, State>(id),
        }
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
}

impl Element for Scrollable {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        cx: &mut LayoutContext,
    ) -> (Vector2F, Self::LayoutState) {
        let mut child_constraint = constraint;
        if self.horizontal {
            assert!(constraint.max.x().is_finite());
            child_constraint.max.set_x(f32::INFINITY);
        }
        if self.vertical {
            assert!(constraint.max.y().is_finite());
            child_constraint.max.set_y(f32::INFINITY);
        }

        let child_size = self.child.layout(child_constraint, cx);
        let size = vec2f(
            constraint.max.x().min(child_size.x()),
            constraint.max.y().min(child_size.y()),
        );
        self.state.update(cx, |state, _| {
            state.scroll = state
                .scroll
                .min(self.child.size() - size)
                .max(Vector2F::zero())
        });

        (size, ())
    }

    fn paint(
        &mut self,
        bounds: RectF,
        visible_bounds: RectF,
        _: &mut Self::LayoutState,
        cx: &mut PaintContext,
    ) -> Self::PaintState {
        let scroll = self.state.read(cx).scroll;
        let child_bounds = RectF::new(bounds.origin() - scroll, self.child.size() - scroll);
        let overflowing = !bounds.contains_rect(child_bounds);

        if overflowing {
            cx.scene.push_layer(Some(bounds));
        }

        self.child.paint(child_bounds.origin(), visible_bounds, cx);

        if overflowing {
            cx.scene.pop_layer();
        }
    }

    fn dispatch_event(
        &mut self,
        event: &Event,
        _: RectF,
        _: RectF,
        _: &mut Self::LayoutState,
        _: &mut Self::PaintState,
        cx: &mut EventContext,
    ) -> bool {
        let mut handled = self.child.dispatch_event(event, cx);
        if !handled {
            if let Event::ScrollWheel(event) = event {
                let mut delta = event.delta;
                if !event.precise {
                    delta *= 20.;
                }

                self.state.update(cx, |state, cx| {
                    if self.horizontal && delta.x() != 0. {
                        state.scroll.set_x(state.scroll.x() - delta.x());
                        cx.notify();
                        handled = true;
                    }

                    if self.vertical && delta.y() != 0. {
                        state.scroll.set_y(state.scroll.y() - delta.y());
                        cx.notify();
                        handled = true;
                    }
                });
            }
        }
        handled
    }

    fn rect_for_text_range(
        &self,
        range_utf16: std::ops::Range<usize>,
        _: RectF,
        _: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &MeasurementContext,
    ) -> Option<RectF> {
        self.child.rect_for_text_range(range_utf16, cx)
    }

    fn debug(
        &self,
        bounds: RectF,
        _: &Self::LayoutState,
        _: &Self::PaintState,
        cx: &DebugContext,
    ) -> json::Value {
        json::json!({
            "type": "Scrollable",
            "bounds": bounds.to_json(),
            "horizontal": self.horizontal,
            "vertical": self.vertical,
            "child": self.child.debug(cx)
        })
    }
}
