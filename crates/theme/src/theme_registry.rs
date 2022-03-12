use crate::{resolution::resolve_references, Theme};
use anyhow::{anyhow, Result};
use gpui::{fonts, AssetSource, FontCache};
use parking_lot::Mutex;
use serde_json::{Map, Value};
use std::{collections::HashMap, sync::Arc};

pub struct ThemeRegistry {
    assets: Box<dyn AssetSource>,
    themes: Mutex<HashMap<String, Arc<Theme>>>,
    font_cache: Arc<FontCache>,
}

impl ThemeRegistry {
    pub fn new(source: impl AssetSource, font_cache: Arc<FontCache>) -> Arc<Self> {
        Arc::new(Self {
            assets: Box::new(source),
            themes: Default::default(),
            font_cache,
        })
    }

    pub fn list(&self) -> impl Iterator<Item = String> {
        ["light".to_string(), "dark".to_string()].into_iter()
    }

    pub fn clear(&self) {
        self.themes.lock().clear();
    }

    pub fn get(&self, name: &str) -> Result<Arc<Theme>> {
        if let Some(theme) = self.themes.lock().get(name) {
            return Ok(theme.clone());
        }

        let theme_data = self.load(name)?;
        let mut theme: Theme = fonts::with_font_cache(self.font_cache.clone(), || {
            serde_path_to_error::deserialize(theme_data)
        })?;

        theme.name = name.into();
        let theme = Arc::new(theme);
        self.themes.lock().insert(name.to_string(), theme.clone());
        Ok(theme)
    }

    fn load(&self, name: &str) -> Result<Value> {
        let base = serde_json::from_slice(self.assets.load("themes/base.json")?.as_ref())?;

        let mut tokens: Value =
            serde_json::from_slice(self.assets.load("themes/tokens.json")?.as_ref())?;
        flatten_token_values(&mut tokens);
        let tokens = tokens
            .as_object_mut()
            .ok_or_else(|| anyhow!("tokens must be an object"))?;
        let core = tokens
            .remove("core")
            .ok_or_else(|| anyhow!("core theme is not present in tokens.json"))?;
        let theme = tokens
            .remove(name)
            .ok_or_else(|| anyhow!("theme {} is not present in tokens.json", name))?;

        let mut base = if let Value::Object(base) = base {
            base
        } else {
            Err(anyhow!("base must be an object"))?
        };
        let core = if let Value::Object(core) = core {
            core
        } else {
            Err(anyhow!("core must be an object"))?
        };
        let theme = if let Value::Object(theme) = theme {
            theme
        } else {
            Err(anyhow!("{} theme must be an object", name))?
        };

        deep_merge_json(&mut base, core);
        deep_merge_json(&mut base, theme);
        resolve_references(Value::Object(base))
    }
}

fn flatten_token_values(value: &mut Value) {
    if let Some(object) = value.as_object_mut() {
        if object.contains_key("type") {
            if let Some(child) = object.remove("value") {
                *value = child;
                return;
            }
        }

        for (_, child) in object.iter_mut() {
            flatten_token_values(child);
        }
    }
}

fn deep_merge_json(base: &mut Map<String, Value>, extension: Map<String, Value>) {
    for (key, extension_value) in extension {
        if let Value::Object(extension_object) = extension_value {
            if let Some(base_object) = base.get_mut(&key).and_then(|value| value.as_object_mut()) {
                deep_merge_json(base_object, extension_object);
            } else {
                base.insert(key, Value::Object(extension_object));
            }
        } else {
            base.insert(key, extension_value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use gpui::MutableAppContext;

    // #[gpui::test]
    // fn test_theme_extension(cx: &mut MutableAppContext) {
    //     let assets = TestAssets(&[
    //         (
    //             "themes/_base.toml",
    //             r##"
    //             [ui.active_tab]
    //             extends = "$ui.tab"
    //             border.color = "#666666"
    //             text = "$text_colors.bright"

    //             [ui.tab]
    //             extends = "$ui.element"
    //             text = "$text_colors.dull"

    //             [ui.element]
    //             background = "#111111"
    //             border = {width = 2.0, color = "#00000000"}

    //             [editor]
    //             background = "#222222"
    //             default_text = "$text_colors.regular"
    //             "##,
    //         ),
    //         (
    //             "themes/light.toml",
    //             r##"
    //             extends = "_base"

    //             [text_colors]
    //             bright = "#ffffff"
    //             regular = "#eeeeee"
    //             dull = "#dddddd"

    //             [editor]
    //             background = "#232323"
    //             "##,
    //         ),
    //     ]);

    //     let registry = ThemeRegistry::new(assets, cx.font_cache().clone());
    //     let theme_data = registry.load("light", true).unwrap();

    //     assert_eq!(
    //         theme_data.as_ref(),
    //         &serde_json::json!({
    //           "ui": {
    //             "active_tab": {
    //               "background": "#111111",
    //               "border": {
    //                 "width": 2.0,
    //                 "color": "#666666"
    //               },
    //               "extends": "$ui.tab",
    //               "text": "#ffffff"
    //             },
    //             "tab": {
    //               "background": "#111111",
    //               "border": {
    //                 "width": 2.0,
    //                 "color": "#00000000"
    //               },
    //               "extends": "$ui.element",
    //               "text": "#dddddd"
    //             },
    //             "element": {
    //               "background": "#111111",
    //               "border": {
    //                 "width": 2.0,
    //                 "color": "#00000000"
    //               }
    //             }
    //           },
    //           "editor": {
    //             "background": "#232323",
    //             "default_text": "#eeeeee"
    //           },
    //           "extends": "_base",
    //           "text_colors": {
    //             "bright": "#ffffff",
    //             "regular": "#eeeeee",
    //             "dull": "#dddddd"
    //           }
    //         })
    //     );
    // }

    // #[gpui::test]
    // fn test_nested_extension(cx: &mut MutableAppContext) {
    //     let assets = TestAssets(&[(
    //         "themes/theme.toml",
    //         r##"
    //             [a]
    //             text = { extends = "$text.0" }

    //             [b]
    //             extends = "$a"
    //             text = { extends = "$text.1" }

    //             [text]
    //             0 = { color = "red" }
    //             1 = { color = "blue" }
    //         "##,
    //     )]);

    //     let registry = ThemeRegistry::new(assets, cx.font_cache().clone());
    //     let theme_data = registry.load("theme", true).unwrap();
    //     assert_eq!(
    //         theme_data
    //             .get("b")
    //             .unwrap()
    //             .get("text")
    //             .unwrap()
    //             .get("color")
    //             .unwrap(),
    //         "blue"
    //     );
    // }

    struct TestAssets(&'static [(&'static str, &'static str)]);

    impl AssetSource for TestAssets {
        fn load(&self, path: &str) -> Result<std::borrow::Cow<[u8]>> {
            if let Some(row) = self.0.iter().find(|e| e.0 == path) {
                Ok(row.1.as_bytes().into())
            } else {
                Err(anyhow!("no such path {}", path))
            }
        }

        fn list(&self, prefix: &str) -> Vec<std::borrow::Cow<'static, str>> {
            self.0
                .iter()
                .copied()
                .filter_map(|(path, _)| {
                    if path.starts_with(prefix) {
                        Some(path.into())
                    } else {
                        None
                    }
                })
                .collect()
        }
    }
}
