// use settings::Settings;
// use theme::ThemeSettings;
// use ui::{ElementId, IntoElement, RenderOnce, WindowContext};

// use crate::{setting_item::VisualSetting, SettingsItem};

// pub struct EditorFontFamilySetting {}

// pub struct EditorVisualSettings {
//     font_family: SettingsItem,
// }

// impl VisualSetting for EditorFontFamilySetting {
//     fn id(&self) -> ElementId {
//         "editor_font_family".into()
//     }

//     fn name(&self) -> ui::SharedString {
//         "Font Family".into()
//     }

//     fn icon(&self) -> Option<ui::IconName> {
//         Some(ui::IconName::Font)
//     }

//     fn value(&self, cx: &WindowContext) -> ui::SharedString {
//         ThemeSettings::get_global(cx).buffer_font.family.clone()
//     }

//     fn set_value(
//         &mut self,
//         settings: &mut ThemeSettings,
//         value: ui::SharedString,
//         cx: &mut WindowContext,
//     ) {
//         let mut theme_settings = ThemeSettings::get_global(cx).clone();
//         let mut buffer_font = theme_settings.buffer_font.clone();
//         let family = value.to_string();
//         buffer_font.family = family.into();
//         theme_settings.buffer_font = buffer_font;
//         ThemeSettings::override_global(theme_settings, cx);
//     }
// }

// #[derive(IntoElement)]
// struct RenderVisualSetting {
//     setting: Box<dyn VisualSetting>,
// }

// impl RenderOnce for RenderVisualSetting {
//     fn render(self, cx: &mut WindowContext) -> impl IntoElement {

//         // self.setting.

//         // SettingsItem::new(

//         // )
//     }
// }

// impl VisualSettings for _ {}
