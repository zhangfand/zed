use gpui::*;
use ui::IconName;

use crate::SettingLayout;

pub trait VisualSetting {
    fn id(&self) -> ElementId;
    fn name(&self) -> SharedString;
    fn icon(&self) -> Option<IconName> {
        None
    }
    fn value(&self, cx: &WindowContext) -> SharedString;
    // TODO: Get a mutable copy of settings passed in here, don't use global
    fn set_value(&mut self, value: SharedString, cx: &mut WindowContext);
    fn disabled(&self) -> bool {
        false
    }
    fn layout(&self) -> SettingLayout {
        SettingLayout::AutoWidth
    }
    fn toggleable(&self) -> Option<bool> {
        None
    }
}

pub enum SettingJsonValue {
    String(SharedString),
    Bool(bool),
    F32(f32),
    I32(i32),
}

pub trait VisualSetting2 {
    fn icon(&self) -> Option<IconName> {
        None
    }
    fn update(&mut self, value: SettingJsonValue, cx: &mut WindowContext);
    fn disabled(&self) -> bool {
        false
    }
    fn layout(&self) -> SettingLayout {
        SettingLayout::AutoWidth
    }
    fn toggleable(&self) -> Option<bool> {
        None
    }
}

pub trait PossibleValues: VisualSetting {
    fn possible_values(&self) -> Vec<SharedString>;
}

// one or more settings from json are reflected in the visual settings
// need to be able to add: icon (optional), layout, for known values get known values for dropdown/typeahead
// need to be able to write back to settings

// impl VisualSetting for AssistantSettings {}
// impl VisualSetting for DefaultAssistantModel {}
// struct VisualEditorSettings {}
