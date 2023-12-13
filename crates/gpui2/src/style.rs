use std::{iter, mem, ops::Range};

use crate::{
    black, phi, point, rems, AbsoluteLength, BorrowAppContext, BorrowWindow, Bounds, ContentMask,
    Corners, CornersRefinement, CursorStyle, DefiniteLength, Edges, EdgesRefinement, Font,
    FontFeatures, FontStyle, FontWeight, Hsla, Length, Pixels, Point, PointRefinement, Rgba,
    SharedString, Size, SizeRefinement, Styled, TextRun, WindowContext,
};
use collections::HashSet;
use refineable::{Cascade, Refineable};
use smallvec::SmallVec;
pub use taffy::style::{
    AlignContent, AlignItems, AlignSelf, Display, FlexDirection, FlexWrap, JustifyContent,
    Overflow, Position,
};

pub type StyleCascade = Cascade<Style>;

#[derive(Clone, Debug)]
pub struct Style {
    /// What layout strategy should be used?
    pub display: Display,

    /// Should the element be painted on screen?
    pub visibility: Visibility,

    // Overflow properties
    /// How children overflowing their container should affect layout
    pub overflow: Point<Overflow>,
    /// How much space (in points) should be reserved for the scrollbars of `Overflow::Scroll` and `Overflow::Auto` nodes.
    pub scrollbar_width: f32,

    // Position properties
    /// What should the `position` value of this struct use as a base offset?
    pub position: Position,
    /// How should the position of this element be tweaked relative to the layout defined?
    pub inset: Edges<Length>,

    // Size properies
    /// Sets the initial size of the item
    pub size: Size<Length>,
    /// Controls the minimum size of the item
    pub min_size: Size<Length>,
    /// Controls the maximum size of the item
    pub max_size: Size<Length>,
    /// Sets the preferred aspect ratio for the item. The ratio is calculated as width divided by height.
    pub aspect_ratio: Option<f32>,

    // Spacing Properties
    /// How large should the margin be on each side?
    pub margin: Edges<Length>,
    /// How large should the padding be on each side?
    pub padding: Edges<DefiniteLength>,
    /// How large should the border be on each side?
    pub border_widths: Edges<AbsoluteLength>,

    // Alignment properties
    /// How this node's children aligned in the cross/block axis?
    pub align_items: Option<AlignItems>,
    /// How this node should be aligned in the cross/block axis. Falls back to the parents [`AlignItems`] if not set
    pub align_self: Option<AlignSelf>,
    /// How should content contained within this item be aligned in the cross/block axis
    pub align_content: Option<AlignContent>,
    /// How should contained within this item be aligned in the main/inline axis
    pub justify_content: Option<JustifyContent>,
    /// How large should the gaps between items in a flex container be?
    pub gap: Size<DefiniteLength>,

    // Flexbox properies
    /// Which direction does the main axis flow in?
    pub flex_direction: FlexDirection,
    /// Should elements wrap, or stay in a single line?
    pub flex_wrap: FlexWrap,
    /// Sets the initial main axis size of the item
    pub flex_basis: Length,
    /// The relative rate at which this item grows when it is expanding to fill space, 0.0 is the default value, and this value must be positive.
    pub flex_grow: f32,
    /// The relative rate at which this item shrinks when it is contracting to fit into space, 1.0 is the default value, and this value must be positive.
    pub flex_shrink: f32,

    /// The fill color of this element
    pub background: Option<Fill>,

    /// The border color of this element
    pub border_color: Option<Hsla>,

    /// The radius of the corners of this element
    pub corner_radii: Corners<AbsoluteLength>,

    /// Box Shadow of the element
    pub box_shadow: SmallVec<[BoxShadow; 2]>,

    /// TEXT
    pub text: TextStyleRefinement,

    /// The mouse cursor style shown when the mouse pointer is over an element.
    pub mouse_cursor: Option<CursorStyle>,

    pub z_index: Option<u32>,
}

#[derive(Clone)]
enum StyleField {
    Display(Display),
    Visibility(Visibility),
    Overflow(PointRefinement<Overflow>),
    ScrollbarWidth(f32),
    Position(Position),
    Inset(EdgesRefinement<Length>),
    Size(SizeRefinement<Length>),
    MinSize(SizeRefinement<Length>),
    MaxSize(SizeRefinement<Length>),
    AspectRatio(Option<f32>),
    Margin(EdgesRefinement<Length>),
    Padding(EdgesRefinement<DefiniteLength>),
    BorderWidths(EdgesRefinement<AbsoluteLength>),
    AlignItems(Option<AlignItems>),
    AlignSelf(Option<AlignSelf>),
    AlignContent(Option<AlignContent>),
    JustifyContent(Option<JustifyContent>),
    Gap(SizeRefinement<DefiniteLength>),
    FlexDirection(FlexDirection),
    FlexWrap(FlexWrap),
    FlexBasis(Length),
    FlexGrow(f32),
    FlexShrink(f32),
    Background(Option<Fill>),
    BorderColor(Option<Hsla>),
    CornerRadii(CornersRefinement<AbsoluteLength>),
    BoxShadow(SmallVec<[BoxShadow; 2]>),
    Text(TextStyleRefinement),
    MouseCursor(Option<CursorStyle>),
    ZIndex(Option<u32>),
}

#[derive(Clone, Default)]
pub struct StyleRefinement(Vec<StyleField>);

impl Refineable for Style {
    type Refinement = StyleRefinement;

    fn refine(&mut self, refinement: &Self::Refinement) {
        for field in refinement.0.clone() {
            match field {
                StyleField::Display(display) => self.display = display,
                StyleField::Visibility(visibility) => self.visibility = visibility,
                StyleField::Overflow(overflow) => self.overflow.refine(&overflow),
                StyleField::ScrollbarWidth(width) => self.scrollbar_width = width,
                StyleField::Position(position) => self.position = position,
                StyleField::Inset(inset) => self.inset.refine(&inset),
                StyleField::Size(size) => self.size.refine(&size),
                StyleField::MinSize(min_size) => self.min_size.refine(&min_size),
                StyleField::MaxSize(max_size) => self.max_size.refine(&max_size),
                StyleField::AspectRatio(aspect_ratio) => self.aspect_ratio = aspect_ratio,
                StyleField::Margin(margin) => self.margin.refine(&margin),
                StyleField::Padding(padding) => self.padding.refine(&padding),
                StyleField::BorderWidths(border_widths) => {
                    self.border_widths.refine(&border_widths)
                }
                StyleField::AlignItems(align_items) => self.align_items = align_items,
                StyleField::AlignSelf(align_self) => self.align_self = align_self,
                StyleField::AlignContent(align_content) => self.align_content = align_content,
                StyleField::JustifyContent(justify_content) => {
                    self.justify_content = justify_content
                }
                StyleField::Gap(gap) => self.gap.refine(&gap),
                StyleField::FlexDirection(flex_direction) => self.flex_direction = flex_direction,
                StyleField::FlexWrap(flex_wrap) => self.flex_wrap = flex_wrap,
                StyleField::FlexBasis(flex_basis) => self.flex_basis = flex_basis,
                StyleField::FlexGrow(flex_grow) => self.flex_grow = flex_grow,
                StyleField::FlexShrink(flex_shrink) => self.flex_shrink = flex_shrink,
                StyleField::Background(background) => self.background = background,
                StyleField::BorderColor(border_color) => self.border_color = border_color,
                StyleField::CornerRadii(corner_radii) => self.corner_radii.refine(&corner_radii),
                StyleField::BoxShadow(box_shadow) => self.box_shadow = box_shadow,
                StyleField::Text(text) => self.text.refine(&text),
                StyleField::MouseCursor(mouse_cursor) => self.mouse_cursor = mouse_cursor,
                StyleField::ZIndex(z_index) => self.z_index = z_index,
            }
        }
    }

    fn refined(self, refinement: Self::Refinement) -> Self {
        let mut style = self;
        style.refine(&refinement);
        style
    }
}

impl Refineable for StyleRefinement {
    type Refinement = Self;

    fn refine(&mut self, refinement: &Self::Refinement) {
        for field in &refinement.0 {
            match field {
                StyleField::Display(value) => *self.display_mut() = *value,
                StyleField::Visibility(value) => *self.visibility_mut() = *value,
                StyleField::Overflow(value) => self.overflow_mut().refine(value),
                StyleField::ScrollbarWidth(value) => *self.scrollbar_width_mut() = *value,
                StyleField::Position(value) => *self.position_mut() = *value,
                StyleField::Inset(value) => self.inset_mut().refine(value),
                StyleField::Size(value) => self.size_mut().refine(value),
                StyleField::MinSize(value) => self.min_size_mut().refine(value),
                StyleField::MaxSize(value) => self.max_size_mut().refine(value),
                StyleField::AspectRatio(value) => *self.aspect_ratio_mut() = *value,
                StyleField::Margin(value) => self.margin_mut().refine(value),
                StyleField::Padding(value) => self.padding_mut().refine(value),
                StyleField::BorderWidths(value) => self.border_widths_mut().refine(value),
                StyleField::AlignItems(value) => *self.align_items_mut() = *value,
                StyleField::AlignSelf(value) => *self.align_self_mut() = *value,
                StyleField::AlignContent(value) => *self.align_content_mut() = *value,
                StyleField::JustifyContent(value) => *self.justify_content_mut() = *value,
                StyleField::Gap(value) => self.gap_mut().refine(value),
                StyleField::FlexDirection(value) => *self.flex_direction_mut() = *value,
                StyleField::FlexWrap(value) => *self.flex_wrap_mut() = *value,
                StyleField::FlexBasis(value) => *self.flex_basis_mut() = *value,
                StyleField::FlexGrow(value) => *self.flex_grow_mut() = *value,
                StyleField::FlexShrink(value) => *self.flex_shrink_mut() = *value,
                StyleField::Background(value) => *self.background_mut() = value.clone(),
                StyleField::BorderColor(value) => *self.border_color_mut() = *value,
                StyleField::CornerRadii(value) => self.corner_radii_mut().refine(value),
                StyleField::BoxShadow(value) => *self.box_shadow_mut() = value.clone(),
                StyleField::Text(value) => self.text_mut().refine(value),
                StyleField::MouseCursor(value) => *self.mouse_cursor_mut() = *value,
                StyleField::ZIndex(value) => *self.z_index_mut() = *value,
            }
        }
    }

    fn refined(self, refinement: Self::Refinement) -> Self {
        let mut style = self;
        style.refine(&refinement);
        style
    }
}

impl Styled for StyleRefinement {
    fn style(&mut self) -> &mut StyleRefinement {
        self
    }
}

impl StyleRefinement {
    pub fn display(&self) -> Display {
        match self.0.iter().find_map(|field| match field {
            StyleField::Display(value) => Some(*value),
            _ => None,
        }) {
            Some(value) => value,
            None => Display::default(),
        }
    }

    pub fn visibility(&self) -> Option<Visibility> {
        self.0.iter().find_map(|field| match field {
            StyleField::Visibility(value) => Some(*value),
            _ => None,
        })
    }

    pub fn overflow(&self) -> PointRefinement<Overflow> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::Overflow(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn scrollbar_width(&self) -> Option<f32> {
        self.0.iter().find_map(|field| match field {
            StyleField::ScrollbarWidth(value) => Some(*value),
            _ => None,
        })
    }

    pub fn position(&self) -> Option<Position> {
        self.0.iter().find_map(|field| match field {
            StyleField::Position(value) => Some(*value),
            _ => None,
        })
    }

    pub fn inset(&self) -> EdgesRefinement<Length> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::Inset(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn size(&self) -> SizeRefinement<Length> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::Size(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn min_size(&self) -> SizeRefinement<Length> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::MinSize(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn max_size(&self) -> SizeRefinement<Length> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::MaxSize(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn aspect_ratio(&self) -> Option<f32> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::AspectRatio(value) => Some(*value),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn margin(&self) -> EdgesRefinement<Length> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::Margin(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn padding(&self) -> EdgesRefinement<DefiniteLength> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::Padding(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn border_widths(&self) -> EdgesRefinement<AbsoluteLength> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::BorderWidths(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn align_items(&self) -> Option<AlignItems> {
        match self.0.iter().find_map(|field| match field {
            StyleField::AlignItems(value) => Some(*value),
            _ => None,
        }) {
            Some(value) => value,
            None => None,
        }
    }

    pub fn align_self(&self) -> Option<AlignSelf> {
        match self.0.iter().find_map(|field| match field {
            StyleField::AlignSelf(value) => Some(*value),
            _ => None,
        }) {
            Some(value) => value,
            None => None,
        }
    }

    pub fn align_content(&self) -> Option<AlignContent> {
        match self.0.iter().find_map(|field| match field {
            StyleField::AlignContent(value) => Some(*value),
            _ => None,
        }) {
            Some(value) => value,
            None => None,
        }
    }

    pub fn justify_content(&self) -> Option<JustifyContent> {
        match self.0.iter().find_map(|field| match field {
            StyleField::JustifyContent(value) => Some(*value),
            _ => None,
        }) {
            Some(value) => value,
            None => None,
        }
    }

    pub fn gap(&self) -> SizeRefinement<DefiniteLength> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::Gap(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn flex_direction(&self) -> FlexDirection {
        match self.0.iter().find_map(|field| match field {
            StyleField::FlexDirection(value) => Some(*value),
            _ => None,
        }) {
            Some(value) => value,
            None => FlexDirection::default(),
        }
    }

    pub fn flex_wrap(&self) -> Option<FlexWrap> {
        self.0.iter().find_map(|field| match field {
            StyleField::FlexWrap(value) => Some(*value),
            _ => None,
        })
    }

    pub fn flex_basis(&self) -> Option<Length> {
        self.0.iter().find_map(|field| match field {
            StyleField::FlexBasis(value) => Some(*value),
            _ => None,
        })
    }

    pub fn flex_grow(&self) -> Option<f32> {
        self.0.iter().find_map(|field| match field {
            StyleField::FlexGrow(value) => Some(*value),
            _ => None,
        })
    }

    pub fn flex_shrink(&self) -> Option<f32> {
        self.0.iter().find_map(|field| match field {
            StyleField::FlexShrink(value) => Some(*value),
            _ => None,
        })
    }

    pub fn background(&self) -> Option<Fill> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::Background(value) => Some(value.clone()),
                _ => None,
            })
            .flatten()
    }

    pub fn border_color(&self) -> Option<Hsla> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::BorderColor(value) => Some(value.clone()),
                _ => None,
            })
            .flatten()
    }

    pub fn corner_radii(&self) -> CornersRefinement<AbsoluteLength> {
        self.0
            .iter()
            .find_map(|field| match field {
                StyleField::CornerRadii(value) => Some(value.clone()),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn box_shadow(&self) -> Option<SmallVec<[BoxShadow; 2]>> {
        self.0.iter().find_map(|field| match field {
            StyleField::BoxShadow(value) => Some(value.clone()),
            _ => None,
        })
    }

    pub fn text(&self) -> TextStyleRefinement {
        match self.0.iter().find_map(|field| match field {
            StyleField::Text(value) => Some(value.clone()),
            _ => None,
        }) {
            Some(value) => value,
            None => TextStyleRefinement::default(),
        }
    }

    pub fn mouse_cursor(&self) -> Option<CursorStyle> {
        match self.0.iter().find_map(|field| match field {
            StyleField::MouseCursor(value) => Some(value.clone()),
            _ => None,
        }) {
            Some(value) => value,
            None => None,
        }
    }

    pub fn z_index(&self) -> Option<u32> {
        match self.0.iter().find_map(|field| match field {
            StyleField::ZIndex(value) => Some(value.clone()),
            _ => None,
        }) {
            Some(value) => value,
            None => None,
        }
    }

    pub fn display_mut(&mut self) -> &mut Display {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Display(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Display(Display::default()));
                self.0.len() - 1
            });
        if let StyleField::Display(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn visibility_mut(&mut self) -> &mut Visibility {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Visibility(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Visibility(Visibility::default()));
                self.0.len() - 1
            });
        if let StyleField::Visibility(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn overflow_mut(&mut self) -> &mut PointRefinement<Overflow> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Overflow(_)))
            .unwrap_or_else(|| {
                self.0
                    .push(StyleField::Overflow(PointRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::Overflow(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn scrollbar_width_mut(&mut self) -> &mut f32 {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::ScrollbarWidth(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::ScrollbarWidth(Default::default()));
                self.0.len() - 1
            });
        if let StyleField::ScrollbarWidth(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn position_mut(&mut self) -> &mut Position {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Position(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Position(Position::default()));
                self.0.len() - 1
            });
        if let StyleField::Position(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn inset_mut(&mut self) -> &mut EdgesRefinement<Length> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Inset(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Inset(EdgesRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::Inset(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn size_mut(&mut self) -> &mut SizeRefinement<Length> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Size(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Size(SizeRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::Size(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn min_size_mut(&mut self) -> &mut SizeRefinement<Length> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::MinSize(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::MinSize(SizeRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::MinSize(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn max_size_mut(&mut self) -> &mut SizeRefinement<Length> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::MaxSize(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::MaxSize(SizeRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::MaxSize(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn aspect_ratio_mut(&mut self) -> &mut Option<f32> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::AspectRatio(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::AspectRatio(None));
                self.0.len() - 1
            });
        if let StyleField::AspectRatio(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn margin_mut(&mut self) -> &mut EdgesRefinement<Length> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Margin(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Margin(EdgesRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::Margin(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn padding_mut(&mut self) -> &mut EdgesRefinement<DefiniteLength> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Padding(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Padding(EdgesRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::Padding(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn border_widths_mut(&mut self) -> &mut EdgesRefinement<AbsoluteLength> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::BorderWidths(_)))
            .unwrap_or_else(|| {
                self.0
                    .push(StyleField::BorderWidths(EdgesRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::BorderWidths(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn align_items_mut(&mut self) -> &mut Option<AlignItems> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::AlignItems(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::AlignItems(None));
                self.0.len() - 1
            });
        if let StyleField::AlignItems(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn align_self_mut(&mut self) -> &mut Option<AlignSelf> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::AlignSelf(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::AlignSelf(None));
                self.0.len() - 1
            });
        if let StyleField::AlignSelf(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn align_content_mut(&mut self) -> &mut Option<AlignContent> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::AlignContent(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::AlignContent(None));
                self.0.len() - 1
            });
        if let StyleField::AlignContent(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn justify_content_mut(&mut self) -> &mut Option<JustifyContent> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::JustifyContent(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::JustifyContent(None));
                self.0.len() - 1
            });
        if let StyleField::JustifyContent(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn gap_mut(&mut self) -> &mut SizeRefinement<DefiniteLength> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Gap(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Gap(SizeRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::Gap(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn flex_direction_mut(&mut self) -> &mut FlexDirection {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::FlexDirection(_)))
            .unwrap_or_else(|| {
                self.0
                    .push(StyleField::FlexDirection(FlexDirection::default()));
                self.0.len() - 1
            });
        if let StyleField::FlexDirection(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn flex_wrap_mut(&mut self) -> &mut FlexWrap {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::FlexWrap(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::FlexWrap(FlexWrap::default()));
                self.0.len() - 1
            });
        if let StyleField::FlexWrap(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn flex_basis_mut(&mut self) -> &mut Length {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::FlexBasis(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::FlexBasis(Length::default()));
                self.0.len() - 1
            });
        if let StyleField::FlexBasis(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn flex_grow_mut(&mut self) -> &mut f32 {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::FlexGrow(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::FlexGrow(Default::default()));
                self.0.len() - 1
            });
        if let StyleField::FlexGrow(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn flex_shrink_mut(&mut self) -> &mut f32 {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::FlexShrink(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::FlexShrink(Default::default()));
                self.0.len() - 1
            });
        if let StyleField::FlexShrink(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn background_mut(&mut self) -> &mut Option<Fill> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Background(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::Background(None));
                self.0.len() - 1
            });
        if let StyleField::Background(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn border_color_mut(&mut self) -> &mut Option<Hsla> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::BorderColor(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::BorderColor(None));
                self.0.len() - 1
            });
        if let StyleField::BorderColor(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn corner_radii_mut(&mut self) -> &mut CornersRefinement<AbsoluteLength> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::CornerRadii(_)))
            .unwrap_or_else(|| {
                self.0
                    .push(StyleField::CornerRadii(CornersRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::CornerRadii(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn box_shadow_mut(&mut self) -> &mut SmallVec<[BoxShadow; 2]> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::BoxShadow(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::BoxShadow(SmallVec::new()));
                self.0.len() - 1
            });
        if let StyleField::BoxShadow(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn text_mut(&mut self) -> &mut TextStyleRefinement {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::Text(_)))
            .unwrap_or_else(|| {
                self.0
                    .push(StyleField::Text(TextStyleRefinement::default()));
                self.0.len() - 1
            });
        if let StyleField::Text(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn mouse_cursor_mut(&mut self) -> &mut Option<CursorStyle> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::MouseCursor(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::MouseCursor(None));
                self.0.len() - 1
            });
        if let StyleField::MouseCursor(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }

    pub fn z_index_mut(&mut self) -> &mut Option<u32> {
        let ix = self
            .0
            .iter()
            .position(|field| matches!(field, StyleField::ZIndex(_)))
            .unwrap_or_else(|| {
                self.0.push(StyleField::ZIndex(None));
                self.0.len() - 1
            });
        if let StyleField::ZIndex(value) = &mut self.0[ix] {
            value
        } else {
            unreachable!()
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
}

#[derive(Clone, Debug)]
pub struct BoxShadow {
    pub color: Hsla,
    pub offset: Point<Pixels>,
    pub blur_radius: Pixels,
    pub spread_radius: Pixels,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum WhiteSpace {
    #[default]
    Normal,
    Nowrap,
}

#[derive(Refineable, Clone, Debug)]
#[refineable(Debug)]
pub struct TextStyle {
    pub color: Hsla,
    pub font_family: SharedString,
    pub font_features: FontFeatures,
    pub font_size: AbsoluteLength,
    pub line_height: DefiniteLength,
    pub font_weight: FontWeight,
    pub font_style: FontStyle,
    pub background_color: Option<Hsla>,
    pub underline: Option<UnderlineStyle>,
    pub white_space: WhiteSpace,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            color: black(),
            font_family: "Helvetica".into(), // todo!("Get a font we know exists on the system")
            font_features: FontFeatures::default(),
            font_size: rems(1.).into(),
            line_height: phi(),
            font_weight: FontWeight::default(),
            font_style: FontStyle::default(),
            background_color: None,
            underline: None,
            white_space: WhiteSpace::Normal,
        }
    }
}

impl TextStyle {
    pub fn highlight(mut self, style: impl Into<HighlightStyle>) -> Self {
        let style = style.into();
        if let Some(weight) = style.font_weight {
            self.font_weight = weight;
        }
        if let Some(style) = style.font_style {
            self.font_style = style;
        }

        if let Some(color) = style.color {
            self.color = self.color.blend(color);
        }

        if let Some(factor) = style.fade_out {
            self.color.fade_out(factor);
        }

        if let Some(background_color) = style.background_color {
            self.background_color = Some(background_color);
        }

        if let Some(underline) = style.underline {
            self.underline = Some(underline);
        }

        self
    }

    pub fn font(&self) -> Font {
        Font {
            family: self.font_family.clone(),
            features: self.font_features.clone(),
            weight: self.font_weight,
            style: self.font_style,
        }
    }

    /// Returns the rounded line height in pixels.
    pub fn line_height_in_pixels(&self, rem_size: Pixels) -> Pixels {
        self.line_height.to_pixels(self.font_size, rem_size).round()
    }

    pub fn to_run(&self, len: usize) -> TextRun {
        TextRun {
            len,
            font: Font {
                family: self.font_family.clone(),
                features: Default::default(),
                weight: self.font_weight,
                style: self.font_style,
            },
            color: self.color,
            background_color: self.background_color,
            underline: self.underline.clone(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct HighlightStyle {
    pub color: Option<Hsla>,
    pub font_weight: Option<FontWeight>,
    pub font_style: Option<FontStyle>,
    pub background_color: Option<Hsla>,
    pub underline: Option<UnderlineStyle>,
    pub fade_out: Option<f32>,
}

impl Eq for HighlightStyle {}

impl Style {
    pub fn text_style(&self) -> Option<&TextStyleRefinement> {
        if self.text.is_some() {
            Some(&self.text)
        } else {
            None
        }
    }

    pub fn overflow_mask(&self, bounds: Bounds<Pixels>) -> Option<ContentMask<Pixels>> {
        match self.overflow {
            Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            } => None,
            _ => {
                let current_mask = bounds;
                let min = current_mask.origin;
                let max = current_mask.lower_right();
                let bounds = match (
                    self.overflow.x == Overflow::Visible,
                    self.overflow.y == Overflow::Visible,
                ) {
                    // x and y both visible
                    (true, true) => return None,
                    // x visible, y hidden
                    (true, false) => Bounds::from_corners(
                        point(min.x, bounds.origin.y),
                        point(max.x, bounds.lower_right().y),
                    ),
                    // x hidden, y visible
                    (false, true) => Bounds::from_corners(
                        point(bounds.origin.x, min.y),
                        point(bounds.lower_right().x, max.y),
                    ),
                    // both hidden
                    (false, false) => bounds,
                };
                Some(ContentMask { bounds })
            }
        }
    }

    pub fn apply_text_style<C, F, R>(&self, cx: &mut C, f: F) -> R
    where
        C: BorrowAppContext,
        F: FnOnce(&mut C) -> R,
    {
        if self.text.is_some() {
            cx.with_text_style(Some(self.text.clone()), f)
        } else {
            f(cx)
        }
    }

    /// Apply overflow to content mask
    pub fn apply_overflow<C, F, R>(&self, bounds: Bounds<Pixels>, cx: &mut C, f: F) -> R
    where
        C: BorrowWindow,
        F: FnOnce(&mut C) -> R,
    {
        let current_mask = cx.content_mask();

        let min = current_mask.bounds.origin;
        let max = current_mask.bounds.lower_right();

        let mask_bounds = match (
            self.overflow.x == Overflow::Visible,
            self.overflow.y == Overflow::Visible,
        ) {
            // x and y both visible
            (true, true) => return f(cx),
            // x visible, y hidden
            (true, false) => Bounds::from_corners(
                point(min.x, bounds.origin.y),
                point(max.x, bounds.lower_right().y),
            ),
            // x hidden, y visible
            (false, true) => Bounds::from_corners(
                point(bounds.origin.x, min.y),
                point(bounds.lower_right().x, max.y),
            ),
            // both hidden
            (false, false) => bounds,
        };
        let mask = ContentMask {
            bounds: mask_bounds,
        };

        cx.with_content_mask(Some(mask), f)
    }

    /// Paints the background of an element styled with this style.
    pub fn paint(&self, bounds: Bounds<Pixels>, cx: &mut WindowContext) {
        let rem_size = cx.rem_size();

        cx.with_z_index(0, |cx| {
            cx.paint_shadows(
                bounds,
                self.corner_radii.to_pixels(bounds.size, rem_size),
                &self.box_shadow,
            );
        });

        let background_color = self.background.as_ref().and_then(Fill::color);
        if background_color.is_some() || self.is_border_visible() {
            cx.with_z_index(1, |cx| {
                cx.paint_quad(
                    bounds,
                    self.corner_radii.to_pixels(bounds.size, rem_size),
                    background_color.unwrap_or_default(),
                    self.border_widths.to_pixels(rem_size),
                    self.border_color.unwrap_or_default(),
                );
            });
        }
    }

    fn is_border_visible(&self) -> bool {
        self.border_color
            .map_or(false, |color| !color.is_transparent())
            && self.border_widths.any(|length| !length.is_zero())
    }
}

impl Default for Style {
    fn default() -> Self {
        Style {
            display: Display::Block,
            visibility: Visibility::Visible,
            overflow: Point {
                x: Overflow::Visible,
                y: Overflow::Visible,
            },
            scrollbar_width: 0.0,
            position: Position::Relative,
            inset: Edges::auto(),
            margin: Edges::<Length>::zero(),
            padding: Edges::<DefiniteLength>::zero(),
            border_widths: Edges::<AbsoluteLength>::zero(),
            size: Size::auto(),
            min_size: Size::auto(),
            max_size: Size::auto(),
            aspect_ratio: None,
            gap: Size::default(),
            // Aligment
            align_items: None,
            align_self: None,
            align_content: None,
            justify_content: None,
            // Flexbox
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Length::Auto,
            background: None,
            border_color: None,
            corner_radii: Corners::default(),
            box_shadow: Default::default(),
            text: TextStyleRefinement::default(),
            mouse_cursor: None,
            z_index: None,
        }
    }
}

#[derive(Refineable, Copy, Clone, Default, Debug, PartialEq, Eq)]
#[refineable(Debug)]
pub struct UnderlineStyle {
    pub thickness: Pixels,
    pub color: Option<Hsla>,
    pub wavy: bool,
}

#[derive(Clone, Debug)]
pub enum Fill {
    Color(Hsla),
}

impl Fill {
    pub fn color(&self) -> Option<Hsla> {
        match self {
            Fill::Color(color) => Some(*color),
        }
    }
}

impl Default for Fill {
    fn default() -> Self {
        Self::Color(Hsla::default())
    }
}

impl From<Hsla> for Fill {
    fn from(color: Hsla) -> Self {
        Self::Color(color)
    }
}

impl From<TextStyle> for HighlightStyle {
    fn from(other: TextStyle) -> Self {
        Self::from(&other)
    }
}

impl From<&TextStyle> for HighlightStyle {
    fn from(other: &TextStyle) -> Self {
        Self {
            color: Some(other.color),
            font_weight: Some(other.font_weight),
            font_style: Some(other.font_style),
            background_color: other.background_color,
            underline: other.underline.clone(),
            fade_out: None,
        }
    }
}

impl HighlightStyle {
    pub fn highlight(&mut self, other: HighlightStyle) {
        match (self.color, other.color) {
            (Some(self_color), Some(other_color)) => {
                self.color = Some(Hsla::blend(other_color, self_color));
            }
            (None, Some(other_color)) => {
                self.color = Some(other_color);
            }
            _ => {}
        }

        if other.font_weight.is_some() {
            self.font_weight = other.font_weight;
        }

        if other.font_style.is_some() {
            self.font_style = other.font_style;
        }

        if other.background_color.is_some() {
            self.background_color = other.background_color;
        }

        if other.underline.is_some() {
            self.underline = other.underline;
        }

        match (other.fade_out, self.fade_out) {
            (Some(source_fade), None) => self.fade_out = Some(source_fade),
            (Some(source_fade), Some(dest_fade)) => {
                self.fade_out = Some((dest_fade * (1. + source_fade)).clamp(0., 1.));
            }
            _ => {}
        }
    }
}

impl From<Hsla> for HighlightStyle {
    fn from(color: Hsla) -> Self {
        Self {
            color: Some(color),
            ..Default::default()
        }
    }
}

impl From<FontWeight> for HighlightStyle {
    fn from(font_weight: FontWeight) -> Self {
        Self {
            font_weight: Some(font_weight),
            ..Default::default()
        }
    }
}

impl From<FontStyle> for HighlightStyle {
    fn from(font_style: FontStyle) -> Self {
        Self {
            font_style: Some(font_style),
            ..Default::default()
        }
    }
}

impl From<Rgba> for HighlightStyle {
    fn from(color: Rgba) -> Self {
        Self {
            color: Some(color.into()),
            ..Default::default()
        }
    }
}

pub fn combine_highlights(
    a: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
    b: impl IntoIterator<Item = (Range<usize>, HighlightStyle)>,
) -> impl Iterator<Item = (Range<usize>, HighlightStyle)> {
    let mut endpoints = Vec::new();
    let mut highlights = Vec::new();
    for (range, highlight) in a.into_iter().chain(b) {
        if !range.is_empty() {
            let highlight_id = highlights.len();
            endpoints.push((range.start, highlight_id, true));
            endpoints.push((range.end, highlight_id, false));
            highlights.push(highlight);
        }
    }
    endpoints.sort_unstable_by_key(|(position, _, _)| *position);
    let mut endpoints = endpoints.into_iter().peekable();

    let mut active_styles = HashSet::default();
    let mut ix = 0;
    iter::from_fn(move || {
        while let Some((endpoint_ix, highlight_id, is_start)) = endpoints.peek() {
            let prev_index = mem::replace(&mut ix, *endpoint_ix);
            if ix > prev_index && !active_styles.is_empty() {
                let mut current_style = HighlightStyle::default();
                for highlight_id in &active_styles {
                    current_style.highlight(highlights[*highlight_id]);
                }
                return Some((prev_index..ix, current_style));
            }

            if *is_start {
                active_styles.insert(*highlight_id);
            } else {
                active_styles.remove(highlight_id);
            }
            endpoints.next();
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use crate::{blue, green, red, yellow};

    use super::*;

    #[test]
    fn test_combine_highlights() {
        assert_eq!(
            combine_highlights(
                [
                    (0..5, green().into()),
                    (4..10, FontWeight::BOLD.into()),
                    (15..20, yellow().into()),
                ],
                [
                    (2..6, FontStyle::Italic.into()),
                    (1..3, blue().into()),
                    (21..23, red().into()),
                ]
            )
            .collect::<Vec<_>>(),
            [
                (
                    0..1,
                    HighlightStyle {
                        color: Some(green()),
                        ..Default::default()
                    }
                ),
                (
                    1..2,
                    HighlightStyle {
                        color: Some(green()),
                        ..Default::default()
                    }
                ),
                (
                    2..3,
                    HighlightStyle {
                        color: Some(green()),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    3..4,
                    HighlightStyle {
                        color: Some(green()),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    4..5,
                    HighlightStyle {
                        color: Some(green()),
                        font_weight: Some(FontWeight::BOLD),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    5..6,
                    HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        font_style: Some(FontStyle::Italic),
                        ..Default::default()
                    }
                ),
                (
                    6..10,
                    HighlightStyle {
                        font_weight: Some(FontWeight::BOLD),
                        ..Default::default()
                    }
                ),
                (
                    15..20,
                    HighlightStyle {
                        color: Some(yellow()),
                        ..Default::default()
                    }
                ),
                (
                    21..23,
                    HighlightStyle {
                        color: Some(red()),
                        ..Default::default()
                    }
                )
            ]
        );
    }
}
