use std::borrow::Cow;

use gpui::{
    elements::{
        ConstrainedBox, Container, ContainerStyle, Dimensions, Empty, Flex, KeystrokeLabel, Label,
        MouseEventHandler, ParentElement, Stack, Svg, SvgStyle,
    },
    fonts::TextStyle,
    geometry::vector::Vector2F,
    platform,
    platform::MouseButton,
    scene::MouseClick,
    Action, Element, EventContext, MouseState, View, ViewContext,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{ContainedText, Interactive};

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct CheckboxStyle {
    pub icon: SvgStyle,
    pub label: ContainedText,
    pub default: ContainerStyle,
    pub checked: ContainerStyle,
    pub hovered: ContainerStyle,
    pub hovered_and_checked: ContainerStyle,
}

pub fn checkbox<Tag, V, F>(
    label: &'static str,
    style: &CheckboxStyle,
    checked: bool,
    id: usize,
    cx: &mut ViewContext<V>,
    change: F,
) -> MouseEventHandler<Tag, V>
where
    Tag: 'static,
    V: View,
    F: 'static + Fn(&mut V, bool, &mut EventContext<V>),
{
    let label = Label::new(label, style.label.text.clone())
        .contained()
        .with_style(style.label.container);
    checkbox_with_label(label, style, checked, id, cx, change)
}

pub fn checkbox_with_label<Tag, D, V, F>(
    label: D,
    style: &CheckboxStyle,
    checked: bool,
    id: usize,
    cx: &mut ViewContext<V>,
    change: F,
) -> MouseEventHandler<Tag, V>
where
    Tag: 'static,
    D: Element<V>,
    V: View,
    F: 'static + Fn(&mut V, bool, &mut EventContext<V>),
{
    MouseEventHandler::new(id, cx, |state, _| {
        let indicator = if checked {
            svg(&style.icon)
        } else {
            Empty::new()
                .constrained()
                .with_width(style.icon.dimensions.width)
                .with_height(style.icon.dimensions.height)
        };

        Flex::row()
            .with_child(indicator.contained().with_style(if checked {
                if state.hovered() {
                    style.hovered_and_checked
                } else {
                    style.checked
                }
            } else {
                if state.hovered() {
                    style.hovered
                } else {
                    style.default
                }
            }))
            .with_child(label)
            .align_children_center()
    })
    .on_click(platform::MouseButton::Left, move |_, view, cx| {
        change(view, !checked, cx)
    })
    .with_cursor_style(platform::CursorStyle::PointingHand)
}

pub fn svg<V: View>(style: &SvgStyle) -> ConstrainedBox<V> {
    Svg::new(style.asset.clone())
        .with_color(style.color)
        .constrained()
        .with_width(style.dimensions.width)
        .with_height(style.dimensions.height)
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct IconStyle {
    pub icon: SvgStyle,
    pub container: ContainerStyle,
}

impl IconStyle {
    pub fn width(&self) -> f32 {
        self.icon.dimensions.width
            + self.container.padding.left
            + self.container.padding.right
            + self.container.margin.left
            + self.container.margin.right
    }
}

pub fn icon<V: View>(style: &IconStyle) -> Container<V> {
    svg(&style.icon).contained().with_style(style.container)
}

pub fn keystroke_label<V: View>(
    label_text: &'static str,
    label_style: &ContainedText,
    keystroke_style: &ContainedText,
    action: Box<dyn Action>,
    cx: &mut ViewContext<V>,
) -> Container<V> {
    // FIXME: Put the theme in it's own global so we can
    // query the keystroke style on our own
    Flex::row()
        .with_child(Label::new(label_text, label_style.text.clone()).contained())
        .with_child(
            KeystrokeLabel::new(
                cx.view_id(),
                action,
                keystroke_style.container,
                keystroke_style.text.clone(),
            )
            .flex_float(),
        )
        .contained()
        .with_style(label_style.container)
}

pub type ButtonStyle = Interactive<ContainedText>;

pub fn cta_button<Tag, L, V, F>(
    label: L,
    max_width: f32,
    style: &ButtonStyle,
    cx: &mut ViewContext<V>,
    f: F,
) -> MouseEventHandler<Tag, V>
where
    Tag: 'static,
    L: Into<Cow<'static, str>>,
    V: View,
    F: Fn(MouseClick, &mut V, &mut EventContext<V>) + 'static,
{
    MouseEventHandler::<Tag, V>::new(0, cx, |state, _| {
        let style = style.style_for(state);
        Label::new(label, style.text.to_owned())
            .aligned()
            .contained()
            .with_style(style.container)
            .constrained()
            .with_max_width(max_width)
    })
    .on_click(MouseButton::Left, f)
    .with_cursor_style(platform::CursorStyle::PointingHand)
}

#[derive(Clone, Deserialize, Default, JsonSchema)]
pub struct ModalStyle {
    close_icon: Interactive<IconStyle>,
    container: ContainerStyle,
    titlebar: ContainerStyle,
    title_text: Interactive<TextStyle>,
    dimensions: Dimensions,
}

impl ModalStyle {
    pub fn dimensions(&self) -> Vector2F {
        self.dimensions.to_vec()
    }
}

pub fn modal<Tag, V, I, D, F>(
    title: I,
    style: &ModalStyle,
    cx: &mut ViewContext<V>,
    build_modal: F,
) -> impl Element<V>
where
    Tag: 'static,
    V: View,
    I: Into<Cow<'static, str>>,
    D: Element<V>,
    F: FnOnce(&mut gpui::ViewContext<V>) -> D,
{
    const TITLEBAR_HEIGHT: f32 = 28.;

    Flex::column()
        .with_child(
            Stack::new()
                .with_child(Label::new(
                    title,
                    style
                        .title_text
                        .style_for(&mut MouseState::default())
                        .clone(),
                ))
                .with_child(
                    // FIXME: Get a better tag type
                    MouseEventHandler::<Tag, V>::new(999999, cx, |state, _cx| {
                        let style = style.close_icon.style_for(state);
                        icon(style)
                    })
                    .on_click(platform::MouseButton::Left, move |_, _, cx| {
                        cx.remove_window();
                    })
                    .with_cursor_style(platform::CursorStyle::PointingHand)
                    .aligned()
                    .right(),
                )
                .contained()
                .with_style(style.titlebar)
                .constrained()
                .with_height(TITLEBAR_HEIGHT),
        )
        .with_child(
            build_modal(cx)
                .contained()
                .with_style(style.container)
                .constrained()
                .with_width(style.dimensions().x())
                .with_height(style.dimensions().y() - TITLEBAR_HEIGHT),
        )
        .constrained()
        .with_height(style.dimensions().y())
}

pub mod collab_panel {

    use std::ops::Deref;

    use gpui::{
        elements::{ContainerStyle, ElementRender, ImageStyle},
        fonts::TextStyle,
        AnyElement, Element, View, ViewContext,
    };
    use schemars::JsonSchema;
    use serde_derive::Deserialize;

    use crate::{Interactive, Toggleable};

    use super::IconStyle;

    #[derive(Clone, Deserialize, Default, JsonSchema)]
    pub struct DecoratedLabelStructure<L, R> {
        pub left: L,
        pub label: TextStyle,
        pub right: R,
    }
    pub type LabelWithIcon = DecoratedLabelStructure<IconStyle, ()>;
    pub type ChannelName = LabelWithIcon;

    #[derive(Clone, Deserialize, Default, JsonSchema)]
    pub struct ListItemStructure<L, R> {
        pub container: Toggleable<Interactive<ContainerStyle>>,
        pub contents: L,
        pub right: R,
    }

    impl<L, R> ListItemStructure<L, R> {
        pub fn list_container(&self) -> &Toggleable<Interactive<ContainerStyle>> {
            &self.container
        }
        pub fn list_contents(&self) -> &L {
            &self.contents
        }
        pub fn right_item(&self) -> &R {
            &self.right
        }
    }

    #[derive(Clone, Deserialize, Default, JsonSchema)]
    pub struct Disclosable<T> {
        pub disclosure: Toggleable<Interactive<IconStyle>>,
        #[serde(flatten)] // Simulate the typescript kidna thing
        pub item: T,
    }

    impl<T> Deref for Disclosable<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.item
        }
    }

    pub type ListItemWithMeta<L, R> = ListItemStructure<L, R>;

    pub struct ChannelListItemStyle(
        Disclosable<ListItemWithMeta<ChannelName, ChannelListItemRight>>,
    );

    impl Deref for ChannelListItemStyle {
        type Target = Disclosable<ListItemWithMeta<ChannelName, ChannelListItemRight>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    #[derive(Element)]
    pub struct ChannelListItem {
        style: ChannelListItemStyle,
    }

    impl ElementRender for ChannelListItem {
        fn render<V: View>(&mut self, _: &mut V, _: &mut ViewContext<V>) -> AnyElement<V> {
            todo!()
            //     Flex::row()
            //         .with_child(
            //             Svg::new("icons/channel_hash.svg")
            //                 .with_color(theme.channel_hash.color)
            //                 .constrained()
            //                 .with_width(theme.channel_hash.width)
            //                 .aligned()
            //                 .left(),
            //         )
            //         .with_child(
            //             Label::new(channel.name.clone(), theme.contact_username.text.clone())
            //                 .contained()
            //                 .with_style(theme.contact_username.container)
            //                 .aligned()
            //                 .left()
            //                 .flex(1., true),
            //         )
            //         .with_child(
            //             FacePile::new(theme.face_overlap).with_children(
            //                 self.channel_store
            //                     .read(cx)
            //                     .channel_participants(channel_id)
            //                     .iter()
            //                     .filter_map(|user| {
            //                         Some(
            //                             Image::from_data(user.avatar.clone()?)
            //                                 .with_style(theme.contact_avatar),
            //                         )
            //                     }),
            //             ),
            //         )
            //         .align_children_center()
            //         .constrained()
            //         .with_height(theme.row_height)
            //         .contained()
            //         .with_style(*theme.contact_row.style_for(is_selected, state))
            //         .with_padding_left(
            //             theme.contact_row.default_style().padding.left
            //                 + theme.channel_indent * channel.depth as f32,
            //         )
            //         .into_any()
        }
    }

    impl ChannelListItem {
        pub fn new(style: ChannelListItemStyle) -> Self {
            return Self { style };
        }
    }

    pub type ListItem<L> = ListItemStructure<L, ()>;

    // ---- Specific Channels items ----

    pub type ContactName = LabelWithImage;

    pub enum ChannelListItemRight {
        FacePile(Facepile),
        ContextItem(IconButton),
    }

    // pub type ChannelListItem = Disclosable<ListItemWithMeta<ChannelName, ChannelListItemRight>>;
    pub type ContactListItem = ListItemWithMeta<ContactName, IconButton>;
    pub type CurrentCallProjectItem = ListItem<TextStyle>;
    pub type CurrentCallScreenItem = ListItem<LabelWithIcon>;

    pub type SectionHeader = Disclosable<ListItem<TextStyle>>;

    // ---- Generalizable infrastructure and aliases ----

    pub type IconButton = Interactive<IconStyle>;

    pub type Label = DecoratedLabelStructure<(), ()>;
    pub type LabelWithImage = DecoratedLabelStructure<ImageStyle, ()>;

    pub type Facepile = FlexStyle<ImageStyle>;

    #[derive(Clone, Deserialize, Default, JsonSchema)]
    pub struct FlexStyle<T> {
        spacing: f32,
        container: Option<ContainerStyle>,
        children: T,
    }
}
