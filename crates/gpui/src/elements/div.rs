use crate::{
    color::Color, fonts::TextStyle, platform::CursorStyle, AnyElement, Element, LayoutContext,
    SceneBuilder, SizeConstraint, View, ViewContext,
};
use crate::{
    geometry::{
        rect::RectF,
        vector::{vec2f, Vector2F},
    },
    scene,
};
use crate::{Border, CursorRegion, Quad};
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
    /// How to align the children. 0.0 is center, -1.0 is top/left, 1.0 is bottom/right.
    child_alignment: Alignment,
    /// How to layout the children.
    orientation: Orientation,
    /// This style cascades to children.
    text_style: Option<TextStyle>,
}

pub struct Alignment {
    horizontal: f32,
    vertical: f32,
}

impl Default for Alignment {
    fn default() -> Self {
        Self {
            horizontal: -1.,
            vertical: -1.,
        }
    }
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

impl Border {
    fn is_visible(&self) -> bool {
        self.width > 0. && (self.top || self.right || self.bottom || self.left)
    }
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
        let inset = self.inset_size();
        SizeConstraint {
            min: (constraint.min - inset).max(Vector2F::zero()),
            max: (constraint.max - inset).max(Vector2F::zero()),
        }
    }

    fn layout_linear_children(
        &mut self,
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
            if let Some(child_flex) = child.metadata::<DivStyle>().and_then(|style| style.flex) {
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

    fn inset_size(&self) -> Vector2F {
        self.padding_size() + self.border_size() + self.margin_size()
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
        if self.style.border.overlay {
            return Vector2F::zero();
        }

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
    type LayoutState = Vector2F; // Content size
    type PaintState = ();

    fn layout(
        &mut self,
        constraint: SizeConstraint,
        view: &mut V,
        cx: &mut LayoutContext<V>,
    ) -> (Vector2F, Self::LayoutState) {
        let children_constraint = self.inner_constraint(constraint);
        let children_size = match self.style.orientation {
            Orientation::Vertical => self.layout_linear_children(
                LinearOrientation::Vertical,
                children_constraint,
                view,
                cx,
            ),
            Orientation::Horizontal => self.layout_linear_children(
                LinearOrientation::Horizontal,
                children_constraint,
                view,
                cx,
            ),
            Orientation::Stacked => self.layout_stacked_children(children_constraint, view, cx),
        };

        // Add back space for padding, border, and margin.
        let mut size = children_size + self.inset_size();

        // Impose horizontal constraints
        if constraint.min.x().is_finite() {
            size.set_x(size.x().max(constraint.min.x()));
        }
        if size.x() > constraint.max.x() {
            size.set_x(constraint.max.x());
        }

        // Impose vertical constraints
        if constraint.min.y().is_finite() {
            size.set_y(size.y().max(constraint.min.y()));
        }
        if size.y() > constraint.max.y() {
            size.set_y(constraint.max.y());
        }

        (size, children_size)
    }

    fn paint(
        &mut self,
        scene: &mut SceneBuilder,
        bounds: RectF,
        visible_bounds: RectF,
        content_size: &mut Vector2F,
        view: &mut V,
        cx: &mut ViewContext<V>,
    ) -> Self::PaintState {
        let margin = &self.style.margin;

        // Account for margins
        let content_bounds = RectF::from_points(
            bounds.origin() + vec2f(margin.left, margin.top),
            bounds.lower_right() - vec2f(margin.right, margin.bottom),
        );

        // Paint drop shadow
        if let Some(shadow) = self.style.shadow.as_ref() {
            scene.push_shadow(scene::Shadow {
                bounds: content_bounds + shadow.offset,
                corner_radius: self.style.corner_radius,
                sigma: shadow.blur,
                color: shadow.color,
            });
        }

        // Paint cursor style
        if let Some(hit_bounds) = content_bounds.intersection(visible_bounds) {
            if let Some(style) = self.style.cursor {
                scene.push_cursor_region(CursorRegion {
                    bounds: hit_bounds,
                    style,
                });
            }
        }

        // Render the background and/or the border (if it not an overlay border).
        if self.style.background_color.is_some()
            || (self.style.border.is_visible() && !self.style.border.overlay)
        {
            // If the border is overlay, render the background now and wait to
            // render the border until after the children are rendered.
            if self.style.border.overlay {
                scene.push_quad(Quad {
                    bounds: content_bounds,
                    background: self.style.background_color,
                    border: Default::default(),
                    corner_radius: self.style.corner_radius,
                });
            } else {
                scene.push_quad(Quad {
                    bounds: content_bounds,
                    background: self.style.background_color,
                    border: self.style.border,
                    corner_radius: self.style.corner_radius,
                });
            }
        }

        let padding = &self.style.padding;
        let children_bounds = RectF::from_points(
            content_bounds.origin() + vec2f(padding.left, padding.top),
            content_bounds.lower_right() - vec2f(padding.right, padding.top),
        );

        match self.style.orientation {
            Orientation::Vertical => todo!(),
            Orientation::Horizontal => todo!(),
            Orientation::Stacked => todo!(),
        }

        // Draw overlay border on top
        if self.style.border.is_visible() && self.style.border.overlay {
            scene.paint_layer(None, |scene| {
                scene.push_quad(Quad {
                    bounds: content_bounds,
                    background: Default::default(),
                    border: self.style.border,
                    corner_radius: self.style.corner_radius,
                });
            })
        }

        // self.child
        //     .paint(scene, child_origin, visible_bounds, view, cx);

        // scene.push_layer(None);
        // scene.push_quad(Quad {
        //     bounds: quad_bounds,
        //     background: self.style.overlay_color,
        //     border: self.style.border,
        //     corner_radius: self.style.corner_radius,
        // });
        // scene.pop_layer();
        // } else {
        //     scene.push_quad(Quad {
        //         bounds: quad_bounds,
        //         background: self.style.background_color,
        //         border: self.style.border,
        //         corner_radius: self.style.corner_radius,
        //     });

        //     let child_origin = child_origin
        //         + vec2f(
        //             self.style.border.left_width(),
        //             self.style.border.top_width(),
        //         );
        //     self.child
        //         .paint(scene, child_origin, visible_bounds, view, cx);

        //     if self.style.overlay_color.is_some() {
        //         scene.push_layer(None);
        //         scene.push_quad(Quad {
        //             bounds: quad_bounds,
        //             background: self.style.overlay_color,
        //             border: Default::default(),
        //             corner_radius: 0.,
        //         });
        //         scene.pop_layer();
        //     }
        // }
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
