//! The GPUI prelude is a collection of traits and types that are widely used
//! throughout the library. It is recommended to import this prelude into your
//! application to avoid having to import each trait individually.

pub use crate::{
    div, px, relative, rems, rgb, rgba, util::FluentBuilder, AbsoluteLength, BorrowAppContext,
    BorrowWindow, Context, DefiniteLength, Div, Element, ElementContext, ElementId,
    FocusableElement, InteractiveElement, IntoElement, ParentElement, Pixels, Refineable, Rems,
    Render, RenderOnce, SharedString, StatefulInteractiveElement, Styled, ViewContext,
    VisualContext, WindowContext,
};
