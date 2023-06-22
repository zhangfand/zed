use crate::{
    color::Color, fonts::TextStyle, platform::CursorStyle, AnyElement, Element, Entity,
    LayoutContext, SceneBuilder, SizeConstraint, View, ViewContext,
};
use pathfinder_geometry::{rect::RectF, vector::Vector2F};
use serde_json::Value;
use std::{ops::Range, rc::Rc};
use ts_rs::TS;

pub struct Div<V: View> {
    style: Rc<DivStyle>,
    children: Vec<AnyElement<V>>,
}

pub struct DivStyle {
    // Size and alignment
    // ------------------
    size: Size,
    margin: Margin,
    padding: Padding,
    alignment: Option<Alignment>,

    // Appearance
    // ----------
    background_color: Option<Color>,
    overlay_color: Option<Color>,
    border: Border,
    corner_radius: f32,
    shadow: Option<Shadow>,
    cursor: Option<CursorStyle>,

    // Children
    // --------
    /// How to layout the children.
    orientation: Orientation,
    /// This style cascades to children.
    text_style: Option<TextStyle>,
}

pub enum Orientation {
    Vertical,
    Horizontal,
    Stacked,
}

#[derive(TS)]
#[ts(export, export_to = "styles/types/", rename_all = "camelCase")]
struct Size {
    min_width: Option<f32>,
    max_width: Option<f32>,
    maximize_width: bool,
    min_height: Option<f32>,
    max_height: Option<f32>,
    maximize_height: bool,
}

pub struct Margin {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

pub struct Padding {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

pub struct Alignment {
    horizontal: f32,
    vertical: f32,
}

pub struct Border {
    pub width: f32,
    pub color: Color,
    pub overlay: bool,
    pub top: bool,
    pub right: bool,
    pub bottom: bool,
    pub left: bool,
}

pub struct Shadow {
    offset: Vector2F,
    blur: f32,
    color: Color,
}

impl<V: View> Element<V> for Div<V> {
    type LayoutState = ();

    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        todo!()
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        todo!()
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Value {
        todo!()
    }
}

pub fn div<V: View>(style: Rc<DivStyle>) -> Div<V> {
    Div {
        style,
        children: Vec::new(),
    }
}

struct TestStyle {
    root: DivStyle,
}

struct TestView {}

impl Entity for TestView {
    type Event = ();
}

impl View for TestView {
    type Style = ();

    // type Style = TestStyle;

    fn ui_name() -> &'static str {
        "TestView"
    }

    fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> AnyElement<Self> {
        // div(cx.style().root).with_child()
        todo!()
    }
}

impl<V: View> Element<V> for &'static str {
    type LayoutState = ();
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: crate::SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        todo!()
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &mut Self::LayoutState,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        todo!()
    }

    fn rect_for_text_range(
        &self,
        range_utf16: Range<usize>,
        bounds: RectF,
        visible_bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> Option<RectF> {
        todo!()
    }

    fn debug(
        &self,
        bounds: RectF,
        layout: &Self::LayoutState,
        paint: &Self::PaintState,
        view: &V,
        cx: &ViewContext<V>,
    ) -> serde_json::Value {
        todo!()
    }
}
