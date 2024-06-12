//! # Settings UI
//!
//! This crate is used to create visual components for settings, such as menus & windows.
//!

#[cfg(feature = "stories")]
mod stories;

#[cfg(feature = "stories")]
pub use stories::*;

use ui::{prelude::*, Checkbox, List, ListHeader};

#[derive(Debug, Clone, IntoElement)]
struct DropdownMenu {
    id: ElementId,
    current_item: Option<SharedString>,
    items: Vec<SharedString>,
    full_width: bool,
}

impl DropdownMenu {
    pub fn new(id: impl Into<ElementId>, _cx: &WindowContext) -> Self {
        Self {
            id: id.into(),
            current_item: None,
            items: Vec::new(),
            full_width: false,
        }
    }

    pub fn current_item(mut self, current_item: Option<SharedString>) -> Self {
        self.current_item = current_item;
        self
    }

    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
}

impl RenderOnce for DropdownMenu {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        h_flex()
            .id(self.id)
            .justify_between()
            .rounded_md()
            .bg(cx.theme().colors().editor_background)
            .px_1p5()
            .py_0p5()
            .gap_2()
            .when(self.full_width, |this| this.w_full())
            .cursor_pointer()
            .child(Label::new(self.current_item.unwrap_or("".into())))
            .child(
                Icon::new(IconName::ChevronDown)
                    .size(IconSize::XSmall)
                    .color(Color::Muted),
            )
    }
}

#[derive(PartialEq, Clone, Eq, Debug)]
pub enum ToggleType {
    Checkbox,
    Switch,
}

impl From<ToggleType> for SettingType {
    fn from(toggle_type: ToggleType) -> Self {
        SettingType::Toggle(toggle_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputType {
    Text,
    Number,
}

impl From<InputType> for SettingType {
    fn from(input_type: InputType) -> Self {
        SettingType::Input(input_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecondarySettingType {
    Dropdown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingType {
    Toggle(ToggleType),
    ToggleAnd(SecondarySettingType),
    Input(InputType),
    Dropdown,
    Range,
}

pub enum SettingsItems {
    SettingsGroup(SettingsGroup),
    SettingsItem(SettingsItem),
    Button(ui::Button),
}

#[derive(Debug, Clone, IntoElement)]
struct SettingsGroup {
    name: String,
    settings: Vec<SettingsItem>,
}

impl SettingsGroup {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            settings: Vec::new(),
        }
    }

    pub fn add_setting(mut self, setting: SettingsItem) -> Self {
        self.settings.push(setting);
        self
    }
}

impl RenderOnce for SettingsGroup {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let empty_message = format!("No settings available for {}", self.name);

        let header = ListHeader::new(self.name);

        let settings = self.settings.clone().into_iter().map(|setting| setting);

        v_flex()
            .p_1()
            .gap_2()
            .child(header)
            .when(self.settings.len() == 0, |this| {
                this.child(Label::new(empty_message))
            })
            .children(settings)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum SettingLayout {
    Stacked,
    Inline,
    FullLine,
    FullLineJustified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingId(pub SharedString);

impl From<SettingId> for ElementId {
    fn from(id: SettingId) -> Self {
        ElementId::Name(id.0)
    }
}

impl From<&str> for SettingId {
    fn from(id: &str) -> Self {
        Self(id.to_string().into())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingValue(pub SharedString);

impl From<SharedString> for SettingValue {
    fn from(value: SharedString) -> Self {
        Self(value)
    }
}

impl From<String> for SettingValue {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<bool> for SettingValue {
    fn from(value: bool) -> Self {
        Self(value.to_string().into())
    }
}

impl From<SettingValue> for bool {
    fn from(value: SettingValue) -> Self {
        value.0 == "true"
    }
}

#[derive(Debug, Clone, IntoElement)]
struct SettingsItem {
    id: SettingId,
    name: SharedString,
    enabled: bool,
    setting_type: SettingType,
    current_value: Option<SettingValue>,
    possible_values: Option<Vec<SettingValue>>,
    layout: SettingLayout,
    hide_label: bool,
    toggled: Option<bool>,
}

impl SettingsItem {
    pub fn new(
        id: impl Into<SettingId>,
        name: SharedString,
        setting_type: SettingType,
        current_value: Option<SettingValue>,
    ) -> Self {
        let toggled = match setting_type {
            SettingType::Toggle(_) | SettingType::ToggleAnd(_) => Some(false),
            _ => None,
        };

        Self {
            id: id.into(),
            name,
            enabled: true,
            setting_type,
            current_value,
            possible_values: None,
            layout: SettingLayout::FullLine,
            hide_label: false,
            toggled,
        }
    }

    pub fn name(mut self, name: impl Into<SharedString>) -> Self {
        self.name = name.into();
        self
    }

    pub fn get_name(&self) -> &SharedString {
        &self.name
    }

    pub fn get_id(&self) -> &SettingId {
        &self.id
    }

    pub fn layout(mut self, layout: SettingLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn toggled(mut self, toggled: bool) -> Self {
        self.toggled = Some(toggled);
        self
    }

    pub fn hide_label(mut self, hide_label: bool) -> Self {
        self.hide_label = hide_label;
        self
    }
}

impl RenderOnce for SettingsItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let id: ElementId = self.id.clone().into();

        let full_width = match self.layout {
            SettingLayout::FullLine | SettingLayout::FullLineJustified => true,
            _ => false,
        };

        let justified = match (self.layout.clone(), self.setting_type.clone()) {
            (_, SettingType::ToggleAnd(_)) => true,
            (SettingLayout::FullLineJustified, _) => true,
            _ => false,
        };

        let (setting_type, current_value) = (self.setting_type.clone(), self.current_value.clone());
        let current_string = if let Some(current_value) = current_value.clone() {
            Some(current_value.0)
        } else {
            None
        };

        let toggleable = match setting_type {
            SettingType::Toggle(_) => true,
            SettingType::ToggleAnd(_) => true,
            _ => false,
        };

        let setting_element = match setting_type {
            SettingType::Toggle(_) => None,
            SettingType::ToggleAnd(secondary_setting_type) => match secondary_setting_type {
                SecondarySettingType::Dropdown => Some(
                    DropdownMenu::new(id.clone(), &cx)
                        .current_item(current_string)
                        .into_any_element(),
                ),
            },
            SettingType::Input(input_type) => match input_type {
                InputType::Text => Some(div().child("text").into_any_element()),
                InputType::Number => Some(div().child("number").into_any_element()),
            },
            SettingType::Dropdown => Some(
                DropdownMenu::new(id.clone(), &cx)
                    .current_item(current_string)
                    .full_width(true)
                    .into_any_element(),
            ),
            SettingType::Range => Some(div().child("range").into_any_element()),
        };

        let item = if self.layout == SettingLayout::Stacked {
            v_flex()
        } else {
            h_flex()
        };

        item.id(id)
            .gap_1()
            .when(full_width, |this| this.w_full())
            .when(justified, |this| this.justify_between())
            .when(toggleable, |this| {
                let checkbox = Checkbox::new(
                    ElementId::Name(format!("toggle-{}", self.id.0).to_string().into()),
                    self.toggled.unwrap_or(false).into(),
                );

                let toggle_element = match self.setting_type.clone() {
                    SettingType::Toggle(toggle_type) => match toggle_type {
                        ToggleType::Checkbox => checkbox,
                        ToggleType::Switch => todo!(),
                    },
                    SettingType::ToggleAnd(_) => checkbox,
                    _ => unreachable!(),
                };

                this.child(
                    h_flex()
                        .gap_1()
                        .child(toggle_element)
                        .children(if self.hide_label {
                            None
                        } else {
                            Some(Label::new(self.name.clone()))
                        }),
                )
            })
            .when(!toggleable, |this| {
                this.children(if self.hide_label {
                    None
                } else {
                    Some(Label::new(self.name.clone()))
                })
            })
            .children(setting_element)
    }
}

struct SettingsMenu {
    name: SharedString,
    groups: Vec<SettingsGroup>,
}

impl SettingsMenu {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            groups: Vec::new(),
        }
    }

    pub fn add_group(mut self, group: SettingsGroup) -> Self {
        self.groups.push(group);
        self
    }

    pub fn get_groups(&self) -> &Vec<SettingsGroup> {
        &self.groups
    }
}

impl Render for SettingsMenu {
    fn render(&mut self, cx: &mut ui::ViewContext<Self>) -> impl IntoElement {
        let is_empty = self.groups.is_empty();
        div()
            .elevation_2(cx)
            .min_w_56()
            .max_w_96()
            .max_h_2_3()
            .px_2()
            .py_1()
            .when(is_empty, |this| {
                this.child(Label::new("No settings found").color(Color::Muted))
            })
            .children(self.groups.clone().into_iter().map(|group| group))
    }
}
