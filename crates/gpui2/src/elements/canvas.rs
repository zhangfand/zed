use crate::{Bounds, Element, IntoElement, Pixels, Style, Styled, WindowContext};

pub fn canvas(callback: impl 'static + FnOnce(Bounds<Pixels>, &mut WindowContext)) -> Canvas {
    Canvas {
        paint_callback: Box::new(callback),
        style: Style::default(),
    }
}

pub struct Canvas {
    paint_callback: Box<dyn FnOnce(Bounds<Pixels>, &mut WindowContext)>,
    style: Style,
}

impl IntoElement for Canvas {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        None
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Canvas {
    type State = ();

    fn layout(
        &mut self,
        _: Option<Self::State>,
        cx: &mut WindowContext,
    ) -> (crate::LayoutId, Self::State) {
        let layout_id = cx.request_layout(&self.style, []);
        (layout_id, ())
    }

    fn paint(self, bounds: Bounds<Pixels>, _: &mut (), cx: &mut WindowContext) {
        (self.paint_callback)(bounds, cx)
    }
}

impl Styled for Canvas {
    fn style(&mut self) -> &mut crate::Style {
        &mut self.style
    }
}
