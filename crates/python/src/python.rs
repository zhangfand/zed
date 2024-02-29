pub mod python_settings;

use anyhow::{anyhow, bail};
use db::kvp::KEY_VALUE_STORE;
use gpui::AppContext;
use python_settings::PythonSettings;
use settings::SettingsStore;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug)]
pub struct Interpreter {
    interpreter_path: PathBuf,
}

// TODO INTERPRETER
// In this implementation, the interpreter is the single source of truth.
// activatation_script paths and venv paths are derived from it, but may not exist
// if the interpreter is not associated with a venv.

impl Interpreter {
    // Additional validation happens in find_local_interpreters() and find_global_interpreters()
    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        if !path.ends_with("python") {
            bail!("Not a valid interpreter path");
        }

        Ok(Self {
            interpreter_path: path,
        })
    }

    pub fn interpreter_path(&self) -> PathBuf {
        self.interpreter_path.clone()
    }

    pub fn activation_script_path(&self, settings: &PythonSettings) -> anyhow::Result<PathBuf> {
        let parent = self
            .interpreter_path
            .parent()
            .ok_or_else(|| anyhow!("failed to find the parent directory of the interpreter"))?;
        let user_activation_script = Self::user_activation_script(settings);
        let activation_script_path = parent.join(user_activation_script);
        if !activation_script_path.exists() {
            bail!("could not find activation script");
        }
        Ok(activation_script_path)
    }

    pub fn venv_path(&self, settings: &PythonSettings) -> anyhow::Result<PathBuf> {
        // Since venvs might not have a consistent naming, we rely on the presence of
        // the activation script to find the venv directory.
        let activation_script_path = self.activation_script_path(settings)?;
        let venv_path = activation_script_path.parent().ok_or(anyhow!(
            "failed to find the parent directory of the activation script"
        ))?;
        let venv_file_name = venv_path
            .file_name()
            .ok_or(anyhow!("failed to get directory file name"))?;
        if !settings
            .venv_detection_directories
            .contains(&PathBuf::from(venv_file_name))
        {
            bail!("did not find expected venv directory");
        }

        Ok(venv_path.to_path_buf())
    }

    // TODO INTERPRETER - store in project-specific data (default_interpreter_path) - this is temp for testing
    pub fn store_in_local_settings(&self, cx: &mut AppContext) -> anyhow::Result<()> {
        // selections from find_local_interpreters() will be stored as relative
        // selections from find_global_interpreters() will be stored as absolute
        let interpreter_path = self.interpreter_path.to_string_lossy().to_string();

        cx.spawn(|_| async move {
            KEY_VALUE_STORE
                .write_kvp("interpreter_path".to_string(), interpreter_path)
                .await
                .ok();
        })
        .detach();

        Ok(())
    }

    // TODO INTERPRETER - store in project-specific data (default_interpreter_path) - this is temp for testing
    pub fn retrieve_from_local_settings(worktree_path: Arc<Path>) -> anyhow::Result<Self> {
        let path_string = KEY_VALUE_STORE
            .read_kvp("interpreter_path")?
            .ok_or(anyhow!(
                "failed to read interpreter path from local settings"
            ))?;
        let mut path = PathBuf::from(path_string);

        // Reconstruct any local paths, that were stored as relative to the project,
        // as absolute paths. Not sure this is the best way to handle this.
        // We might want to save a piece of state into the settings "is_local"
        if !path.starts_with(&worktree_path) && path.is_relative() {
            path = path.join(worktree_path)
        }

        if !path.exists() {
            bail!(
                "interpreter does exist at path: {}",
                path.to_string_lossy().to_string()
            )
        }

        Self::new(path)
        // Interpreter::try_from(path_string)
    }

    pub fn user_activation_script(settings: &PythonSettings) -> &'static str {
        match settings.venv_activation_script {
            python_settings::ActivateScript::Default => "activate",
            python_settings::ActivateScript::Csh => "activate.csh",
            python_settings::ActivateScript::Fish => "activate.fish",
            python_settings::ActivateScript::Nushell => "activate.nu",
        }
    }

    pub fn find_interpreters(worktree_path: Arc<Path>, settings: &PythonSettings) -> Vec<Self> {
        let mut interpreters = Self::find_local_interpreters(worktree_path.clone(), settings);
        interpreters.extend(Self::find_global_interpreters(worktree_path, settings));
        interpreters
    }

    pub fn find_local_interpreters(
        worktree_path: Arc<Path>,
        settings: &PythonSettings,
    ) -> Vec<Self> {
        // Should only return relative paths
        // TODO INTERPRETER - Add decorations to local venv ("recommended - see VS Code")
        let paths = settings
            .venv_detection_directories
            .iter()
            .filter_map(|virtual_environment_name| {
                let relative_path = virtual_environment_name.join("bin").join("python");
                let absolute_path = worktree_path.join(relative_path.clone());

                if !absolute_path.exists() {
                    return None;
                }

                Self::new(relative_path).ok()
            })
            .collect::<Vec<_>>();

        paths
    }

    pub fn find_global_interpreters(
        _worktree_path: Arc<Path>,
        _settings: &PythonSettings,
    ) -> Vec<Self> {
        // TODO INTERPRETER - this will require different logic than find_local_interpreters(),
        // since interpreters can exist outside of venvs.
        // TODO INTERPRETER - Add decorations to global venv ("global" - see VS Code")
        // Should only return absolute paths
        Vec::new()
    }

    pub fn get_activate_command(settings: &PythonSettings) -> &'static str {
        match settings.venv_activation_script {
            python_settings::ActivateScript::Nushell => "overlay use",
            _ => "source",
        }
    }
}

impl TryFrom<String> for Interpreter {
    type Error = anyhow::Error;

    fn try_from(path: String) -> anyhow::Result<Self> {
        Self::new(PathBuf::from(path))
    }
}

// TODO INTERPRETER
#[cfg(test)]
mod tests {
    use super::*;
    use fs::{FakeFs, Fs};
    use gpui::TestAppContext;
    use python_settings::ActivateScript;
    use serde_json::json;

    #[gpui::test]
    async fn test_local_interpreter_in_venv(cx: &mut TestAppContext) {
        let settings = init_local_interpreter_in_venv_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.create_file(
            "/user/project/.venv/bin/python".as_ref(),
            Default::default(),
        )
        .await
        .expect("unable to create file");
        let path = fs.paths(false).first().unwrap();
        // let interpreter = Interpreter::try_from("/user/project/".to_string()).unwrap();
        // let venv_path = interpreter.venv_path(&settings).unwrap();
        // assert_eq!(
        //     venv_path,
        //     PathBuf::from("~/user/project/.venv/".to_string())
        // );
        assert!(true)
    }

    #[cfg(any(test, feature = "test-support"))]
    fn init_local_interpreter_in_venv_test(cx: &mut TestAppContext) -> PythonSettings {
        let settings = PythonSettings {
            interpreter_path: PathBuf::from(".venv/bin/python".to_string()),
            venv_activation_script: ActivateScript::Default,
            venv_detection_directories: vec![PathBuf::from(".venv".to_string())],
        };
        cx.set_global(settings.clone());

        settings
    }
}

// PUSH OFF
// -- Adding button
// ---- Get to skip worrying about when to show the button and when to hide
// -- Looking for global interpretters

// Make sure to store local paths as relative paths and global as absolute
// TODO INTERPRETER
