use collections::HashMap;
use gpui::font_cache::FamilyId;
use language::Language;
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Settings {
    pub buffer_font_family: FamilyId,
    pub buffer_font_size: f32,
    pub tab_size: u32,
    pub soft_wrap: SoftWrap,
    pub preferred_line_length: u32,
    pub language_overrides: HashMap<Arc<str>, LanguageOverride>,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema)]
pub struct LanguageOverride {
    pub tab_size: Option<u32>,
    pub soft_wrap: Option<SoftWrap>,
    pub preferred_line_length: Option<u32>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SoftWrap {
    None,
    EditorWidth,
    PreferredLineLength,
}

impl Default for SoftWrap {
    fn default() -> Self {
        SoftWrap::None
    }
}

impl Settings {
    pub fn tab_size(&self, language: Option<&Arc<Language>>) -> u32 {
        language
            .and_then(|language| self.language_overrides.get(language.name().as_ref()))
            .and_then(|settings| settings.tab_size)
            .unwrap_or(self.tab_size)
    }

    pub fn soft_wrap(&self, language: Option<&Arc<Language>>) -> SoftWrap {
        language
            .and_then(|language| self.language_overrides.get(language.name().as_ref()))
            .and_then(|settings| settings.soft_wrap)
            .unwrap_or(self.soft_wrap)
    }

    pub fn preferred_line_length(&self, language: Option<&Arc<Language>>) -> u32 {
        language
            .and_then(|language| self.language_overrides.get(language.name().as_ref()))
            .and_then(|settings| settings.preferred_line_length)
            .unwrap_or(self.preferred_line_length)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test(cx: &gpui::AppContext) -> Settings {
        Settings {
            buffer_font_family: cx.font_cache().load_family(&["Monaco"]).unwrap(),
            buffer_font_size: 14.,
            tab_size: 4,
            soft_wrap: SoftWrap::None,
            preferred_line_length: 80,
            language_overrides: Default::default(),
        }
    }
}
