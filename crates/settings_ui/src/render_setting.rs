use settings::Settings;
use ui::{prelude::*, IntoElement, RenderOnce, SharedString};

use crate::{SettingType, SettingsItem, ToggleType};

impl From<SettingValue2> for SettingType {
    fn from(value: SettingValue2) -> Self {
        match value {
            SettingValue2::Boolean(_) => Self::Toggle(ToggleType::Checkbox),
            SettingValue2::Other => Self::Unsupported,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SettingValue2 {
    Boolean(bool),
    Other,
}

impl From<bool> for SettingValue2 {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

#[derive(IntoElement)]
struct RenderSetting<S: Settings> {
    name: SharedString,
    value: SettingValue2,
    setting_type: SettingType,
    settings: S,
}

impl<S: Settings> RenderSetting<S> {
    pub fn new(setting: SharedString, value: SettingValue2, settings: S) -> Self {
        Self {
            name: setting,
            value: value.clone(),
            setting_type: value.into(),
            settings,
        }
    }

    pub fn name_from_id(&self, id: SharedString) -> SharedString {
        SharedString::from(
            id.replace('_', " ")
                .split_whitespace()
                .map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    }
                })
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

impl<S: Settings> RenderOnce for RenderSetting<S> {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let id = self.name.clone();
        let name = self.name_from_id(self.name.clone());
        let setting_type = self.setting_type;
        let current_value = self.value;

        SettingsItem::new(id, name, setting_type, None)
    }
}
