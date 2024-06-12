use gpui::View;

use ui::prelude::*;

use crate::{
    SecondarySettingType, SettingLayout, SettingType, SettingsGroup, SettingsItem, SettingsMenu,
    ToggleType,
};

pub struct SettingsMenuStory {
    menus: Vec<(SharedString, View<SettingsMenu>)>,
}

impl SettingsMenuStory {
    pub fn new() -> Self {
        Self { menus: Vec::new() }
    }

    pub fn init(cx: &mut ViewContext<Self>) -> Self {
        let mut story = Self::new();
        story.empty_menu(cx);
        story.menu_single_group(cx);
        story
    }
}

impl SettingsMenuStory {
    pub fn empty_menu(&mut self, cx: &mut ViewContext<Self>) {
        let menu = cx.new_view(|_cx| SettingsMenu::new("Empty Menu"));

        self.menus.push(("Empty Menu".into(), menu));
    }

    pub fn menu_single_group(&mut self, cx: &mut ViewContext<Self>) {
        let theme_setting = SettingsItem::new(
            "theme-setting",
            "Theme".into(),
            SettingType::Dropdown,
            Some(cx.theme().name.clone().into()),
        )
        .layout(SettingLayout::Stacked);
        let high_contrast_setting = SettingsItem::new(
            "theme-contrast",
            "Use high contrast theme".into(),
            SettingType::Toggle(ToggleType::Checkbox),
            Some(true.into()),
        );
        let appearance_setting = SettingsItem::new(
            "switch-appearance",
            "Match system appearance".into(),
            SettingType::ToggleAnd(SecondarySettingType::Dropdown),
            Some("When Dark".to_string().into()),
        )
        .layout(SettingLayout::FullLineJustified);

        let group = SettingsGroup::new("Appearance")
            .add_setting(theme_setting)
            .add_setting(appearance_setting)
            .add_setting(high_contrast_setting);

        let menu = cx.new_view(|_cx| SettingsMenu::new("Appearance").add_group(group));

        self.menus.push(("Single Group".into(), menu));
    }
}

impl Render for SettingsMenuStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .bg(cx.theme().colors().background)
            .text_color(cx.theme().colors().text)
            .children(self.menus.iter().map(|(name, menu)| {
                v_flex()
                    .p_2()
                    .gap_2()
                    .child(Headline::new(name.clone()).size(HeadlineSize::Medium))
                    .child(menu.clone())
            }))
    }
}
