use crate::{
    color::Color, fonts::TextStyle, platform::CursorStyle, AnyElement, Element, Entity,
    LayoutContext, SceneBuilder, SizeConstraint, View, ViewContext,
};
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    Vector2FExt,
};
use serde_json::Value;
use std::{f32::INFINITY, ops::Range, rc::Rc};

use super::FlexItemMetadata;

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
    ) {
        let mut flex_total = None;
        let mut fixed_space = 0.0;

        let cross_axis = match orientation {
            LinearOrientation::Vertical => LinearOrientation::Horizontal,
            LinearOrientation::Horizontal => LinearOrientation::Vertical,
        };

        // First pass: Layout fixed children and add up the total flex factor of flexible children.
        let mut cross_axis_max: f32 = 0.0;
        for child in &mut self.children {
            let metadata = child.metadata::<FlexItemMetadata>();

            if let Some(flex) = metadata.and_then(|metadata| metadata.flex.map(|(flex, _)| flex)) {
                *flex_total.get_or_insert(0.) += flex;
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
                fixed_space += match orientation {
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
            LinearOrientation::Vertical => constraint.max.y() - fixed_space,
            LinearOrientation::Horizontal => constraint.max.x() - fixed_space,
        };

        let mut size = if let Some(mut remaining_flex) = flex_total {
            if remaining_space.is_infinite() {
                panic!("flex contains flexible children but has an infinite constraint along the flex axis");
            }

            self.layout_flex_children(
                false,
                constraint,
                &mut remaining_space,
                &mut remaining_flex,
                &mut cross_axis_max,
                view,
                cx,
            );
            self.layout_flex_children(
                true,
                constraint,
                &mut remaining_space,
                &mut remaining_flex,
                &mut cross_axis_max,
                view,
                cx,
            );

            match orientation {
                LinearOrientation::Vertical => {
                    vec2f(cross_axis_max, constraint.max.y() - remaining_space)
                }
                LinearOrientation::Horizontal => {
                    vec2f(constraint.max.x() - remaining_space, cross_axis_max)
                }
            }
        } else {
            match orientation {
                LinearOrientation::Vertical => vec2f(cross_axis_max, fixed_space),
                LinearOrientation::Horizontal => vec2f(fixed_space, cross_axis_max),
            }
        };

        if constraint.min.x().is_finite() {
            size.set_x(size.x().max(constraint.min.x()));
        }
        if constraint.min.y().is_finite() {
            size.set_y(size.y().max(constraint.min.y()));
        }

        if size.x() > constraint.max.x() {
            size.set_x(constraint.max.x());
        }
        if size.y() > constraint.max.y() {
            size.set_y(constraint.max.y());
        }

        if let Some(scroll_state) = self.scroll_state.as_ref() {
            scroll_state.0.update(cx.view_context(), |scroll_state, _| {
                if let Some(scroll_to) = scroll_state.scroll_to.take() {
                    let visible_start = scroll_state.scroll_position.get();
                    let visible_end = visible_start + size.along(self.axis);
                    if let Some(child) = self.children.get(scroll_to) {
                        let child_start: f32 = self.children[..scroll_to]
                            .iter()
                            .map(|c| c.size().along(self.axis))
                            .sum();
                        let child_end = child_start + child.size().along(self.axis);
                        if child_start < visible_start {
                            scroll_state.scroll_position.set(child_start);
                        } else if child_end > visible_end {
                            scroll_state
                                .scroll_position
                                .set(child_end - size.along(self.axis));
                        }
                    }
                }

                scroll_state.scroll_position.set(
                    scroll_state
                        .scroll_position
                        .get()
                        .min(-remaining_space)
                        .max(0.),
                );
            });
        }
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

        if let Some(linear_orientation) = self.style.orientation.linear() {
            self.layout_linear_children(linear_orientation, inner_constraint, view, cx);
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
