use crate::geometry::{
    rect::RectF,
    vector::{vec2f, Vector2F},
};
use crate::{
    color::Color, fonts::TextStyle, platform::CursorStyle, AnyElement, Element, LayoutContext,
    SceneBuilder, SizeConstraint, View, ViewContext,
};
use serde_json::Value;
use std::any::Any;
use std::{f32::INFINITY, ops::Range, rc::Rc};

pub struct Div<V: View> {
    style: Rc<DivStyle>,
    children: Vec<AnyElement<V>>,
}

#[derive(Default)]
pub struct DivStyle {
    // Size and alignment
    // ------------------
    size: Size,
    margin: Margin,
    padding: Padding,
    alignment: Option<Alignment>,
    flex: Option<f32>,

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

#[derive(Default)]
pub enum Orientation {
    #[default]
    Vertical,
    Horizontal,
    Stacked,
}

impl Orientation {
    fn linear(&self) -> Option<LinearOrientation> {
        match self {
            Orientation::Vertical => Some(LinearOrientation::Vertical),
            Orientation::Horizontal => Some(LinearOrientation::Horizontal),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum LinearOrientation {
    Vertical,
    Horizontal,
}

impl LinearOrientation {
    fn invert(&self) -> Self {
        match self {
            LinearOrientation::Vertical => LinearOrientation::Horizontal,
            LinearOrientation::Horizontal => LinearOrientation::Vertical,
        }
    }
}

#[derive(Default)]
struct Size {
    min_width: Option<f32>,
    max_width: Option<f32>,
    maximize_width: bool,
    min_height: Option<f32>,
    max_height: Option<f32>,
    maximize_height: bool,
}

#[derive(Default)]
pub struct Margin {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

#[derive(Default)]
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

#[derive(Default)]
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

impl<V: View> Div<V> {
    pub fn new(style: Rc<DivStyle>) -> Self {
        Self {
            style,
            children: Vec::new(),
        }
    }

    fn inner_constraint(&self, mut constraint: SizeConstraint) -> SizeConstraint {
        // Constrain width
        if let Some(max_width) = self.style.size.max_width {
            constraint.max.set_x(constraint.max.x().min(max_width))
        }

        if self.style.size.maximize_width {
            constraint
                .min
                .set_x(constraint.max.x().max(constraint.min.x()));
        } else if let Some(min_width) = self.style.size.min_width {
            constraint.min.set_x(min_width.max(constraint.min.x()));
        }

        // Constrain height
        if let Some(max_height) = self.style.size.max_height {
            constraint.max.set_y(constraint.max.y().min(max_height))
        }

        if self.style.size.maximize_height {
            constraint
                .min
                .set_y(constraint.max.y().max(constraint.min.y()));
        } else if let Some(min_height) = self.style.size.min_height {
            constraint.min.set_y(min_height.max(constraint.min.y()));
        }

        // Account for margin, border, and padding
        let mut inset = self.margin_size() + self.padding_size();
        if !self.style.border.overlay {
            inset += self.border_size();
        }

        SizeConstraint {
            min: (constraint.min - inset).max(Vector2F::zero()),
            max: (constraint.max - inset).max(Vector2F::zero()),
        }
    }

    fn layout_linear_children(
        &self,
        orientation: LinearOrientation,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Vector2F {
        let cross_axis = orientation.invert();

        let mut total_flex: Option<f32> = None;
        let mut total_size = 0.0;
        let mut cross_axis_max: f32 = 0.0;

        // First pass: Layout fixed children only
        for child in &mut self.children {
            if let Some(child_flex) = child
                .metadata::<DivStyle>()
                .and_then(|style| style.size.flex)
            {
                *total_flex.get_or_insert(0.) += child_flex;
            } else {
                let child_constraint = match orientation {
                    LinearOrientation::Horizontal => SizeConstraint::new(
                        vec2f(0.0, constraint.min.y()),
                        vec2f(INFINITY, constraint.max.y()),
                    ),
                    LinearOrientation::Vertical => SizeConstraint::new(
                        vec2f(constraint.min.x(), 0.0),
                        vec2f(constraint.max.x(), INFINITY),
                    ),
                };
                let child_size = child.layout(child_constraint, view, cx);
                total_size += match orientation {
                    LinearOrientation::Horizontal => {
                        cross_axis_max = cross_axis_max.max(child_size.y());
                        child_size.x()
                    }
                    LinearOrientation::Vertical => {
                        cross_axis_max = cross_axis_max.max(child_size.x());
                        child_size.y()
                    }
                };
            }
        }

        let mut remaining_space = match orientation {
            LinearOrientation::Vertical => constraint.max.y() - total_size,
            LinearOrientation::Horizontal => constraint.max.x() - total_size,
        };

        // Second pass: Layout flexible children
        if let Some(total_flex) = total_flex {
            if total_flex > 0. {
                let space_per_flex = remaining_space.max(0.) / total_flex;

                for child in &mut self.children {
                    if let Some(child_flex) =
                        child.metadata::<DivStyle>().and_then(|style| style.flex)
                    {
                        let child_max = space_per_flex * child_flex;
                        let mut child_constraint = constraint;
                        match orientation {
                            LinearOrientation::Vertical => {
                                child_constraint.min.set_y(0.0);
                                child_constraint.max.set_y(child_max);
                            }
                            LinearOrientation::Horizontal => {
                                child_constraint.min.set_x(0.0);
                                child_constraint.max.set_x(child_max);
                            }
                        }

                        let child_size = child.layout(child_constraint, view, cx);

                        cross_axis_max = match orientation {
                            LinearOrientation::Vertical => {
                                total_size += child_size.y();
                                cross_axis_max.max(child_size.x())
                            }
                            LinearOrientation::Horizontal => {
                                total_size += child_size.x();
                                cross_axis_max.max(child_size.y())
                            }
                        };
                    }
                }
            }
        }

        let mut size = match orientation {
            LinearOrientation::Vertical => vec2f(cross_axis_max, total_size),
            LinearOrientation::Horizontal => vec2f(total_size, cross_axis_max),
        };

        size
    }

    fn layout_stacked_children(
        &self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> Vector2F {
        let mut size = Vector2F::zero();

        for child in &self.children {
            let child_size = child.layout(constraint, view, cx);
            size.set_x(size.x().max(child_size.x()));
            size.set_y(size.y().max(child_size.y()));
        }

        size
    }

    fn margin_size(&self) -> Vector2F {
        vec2f(
            self.style.margin.left + self.style.margin.right,
            self.style.margin.top + self.style.margin.bottom,
        )
    }

    fn padding_size(&self) -> Vector2F {
        vec2f(
            self.style.padding.left + self.style.padding.right,
            self.style.padding.top + self.style.padding.bottom,
        )
    }

    fn border_size(&self) -> Vector2F {
        let mut x = 0.0;
        if self.style.border.left {
            x += self.style.border.width;
        }
        if self.style.border.right {
            x += self.style.border.width;
        }

        let mut y = 0.0;
        if self.style.border.top {
            y += self.style.border.width;
        }
        if self.style.border.bottom {
            y += self.style.border.width;
        }

        vec2f(x, y)
    }
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
        let inner_constraint = self.inner_constraint(constraint);
        let mut size = match self.style.orientation {
            Orientation::Vertical => {
                self.layout_linear_children(LinearOrientation::Vertical, inner_constraint, view, cx)
            }
            Orientation::Horizontal => self.layout_linear_children(
                LinearOrientation::Horizontal,
                inner_constraint,
                view,
                cx,
            ),
            Orientation::Stacked => self.layout_stacked_children(inner_constraint, view, cx),
        };

        size += self.padding_size() + self.border_size() + self.margin_size();

        if constraint.min.x().is_finite() {
            size.set_x(size.x().max(constraint.min.x()));
        }
        if size.x() > constraint.max.x() {
            size.set_x(constraint.max.x());
        }

        if constraint.min.y().is_finite() {
            size.set_y(size.y().max(constraint.min.y()));
        }
        if size.y() > constraint.max.y() {
            size.set_y(constraint.max.y());
        }

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

    fn metadata(&self) -> Option<&dyn Any> {
        Some(&self.style)
    }
}

// struct TestStyle {
//     root: DivStyle,
// }

// struct TestView {}

// impl Entity for TestView {
//     type Event = ();
// }

// impl View for TestView {
//     type Style = TestStyle;

//     // type Style = TestStyle;

//     fn ui_name() -> &'static str {
//         "TestView"
//     }

//     fn render(&mut self, cx: &mut ViewContext<'_, '_, Self>) -> AnyElement<Self> {
//         let style = cx.style();

//         // For each view's style type, generate a typescript interface
//         // For each view's style type, have a typescript function that generates it given a theme
//         //

//         // div(style.root)
//         //     .with_child(
//         //         div(style.titlebar)
//         //             .with_child()
//         //     )
//         todo!()
//     }
// }
