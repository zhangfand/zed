use std::path::PathBuf;

use gpui::Global;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ActivateScript {
    #[default]
    Default,
    Csh,
    Fish,
    Nushell,
}

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct PythonSettings {
    pub interpreter_path: PathBuf,
    pub venv_activation_script: ActivateScript,
    pub venv_detection_directories: Vec<PathBuf>,
}

impl Global for PythonSettings {}

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct PythonSettingsContent {
    /// The path to the interpreter. Can be relative to the project or absolute.
    pub interpreter_path: Option<PathBuf>,
    // TODO
    pub venv_activation_script: Option<ActivateScript>,
    // TODO
    pub venv_detection_directories: Option<Vec<PathBuf>>,
}

impl settings::Settings for PythonSettings {
    const KEY: Option<&'static str> = Some("python");
    type FileContent = PythonSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _cx: &mut gpui::AppContext,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Self::load_via_json_merge(default_value, user_values)
    }
}
