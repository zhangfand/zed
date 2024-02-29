use std::path::PathBuf;

use crate::Project;
use gpui::{AnyWindowHandle, Context, Entity, Model, ModelContext, WeakModel};
use python::{self, python_settings::PythonSettings};
use settings::Settings;
use smol::channel::bounded;
use terminal::{
    terminal_settings::{Shell, TerminalSettings},
    SpawnTask, TaskState, Terminal, TerminalBuilder,
};

// #[cfg(target_os = "macos")]
// use std::os::unix::ffi::OsStrExt;

pub struct Terminals {
    pub(crate) local_handles: Vec<WeakModel<terminal::Terminal>>,
}

impl Project {
    pub fn create_terminal(
        &mut self,
        working_directory: Option<PathBuf>,
        spawn_task: Option<SpawnTask>,
        window: AnyWindowHandle,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<Model<Terminal>> {
        anyhow::ensure!(
            !self.is_remote(),
            "creating terminals as a guest is not supported yet"
        );

        let terminal_settings = TerminalSettings::get_global(cx);
        let python_settings = PythonSettings::get_global(cx);
        let (completion_tx, completion_rx) = bounded(1);
        let mut env = terminal_settings.env.clone();
        let (spawn_task, shell) = if let Some(spawn_task) = spawn_task {
            env.extend(spawn_task.env);
            (
                Some(TaskState {
                    id: spawn_task.id,
                    label: spawn_task.label,
                    completed: false,
                    completion_rx,
                }),
                Shell::WithArguments {
                    program: spawn_task.command,
                    args: spawn_task.args,
                },
            )
        } else {
            (None, terminal_settings.shell.clone())
        };

        let activate_venv_on_launch = terminal_settings.activate_venv_on_launch;
        let python_settings = python_settings.clone();

        let terminal = TerminalBuilder::new(
            working_directory.clone(),
            spawn_task,
            shell,
            env,
            Some(terminal_settings.blinking.clone()),
            terminal_settings.alternate_scroll,
            terminal_settings.max_scroll_history_lines,
            window,
            completion_tx,
        )
        .map(|builder| {
            let terminal_handle = cx.new_model(|cx| builder.subscribe(cx));

            self.terminals
                .local_handles
                .push(terminal_handle.downgrade());

            let id = terminal_handle.entity_id();
            cx.observe_release(&terminal_handle, move |project, _terminal, cx| {
                let handles = &mut project.terminals.local_handles;

                if let Some(index) = handles
                    .iter()
                    .position(|terminal| terminal.entity_id() == id)
                {
                    handles.remove(index);
                    cx.notify();
                }
            })
            .detach();

            // TODO INTERPRETER
            if activate_venv_on_launch {
                // Check to make sure path exists, show toast if not
                // logic should account for absolute and relative paths
                // if let Some(venv_settings) = &venv_settings.as_option() {
                //     let interpreter_path = python::retrieve_interpreter_path_from_local_settings();
                //     let activate_command = python::get_activate_command(venv_settings);
                //     let activate_script_path =
                //         python::find_activate_script_path(venv_settings, working_directory);
                //     self.activate_python_virtual_environment(
                //         activate_command,
                //         activate_script_path,
                //         &terminal_handle,
                //         cx,
                //     );
                // }
            }
            terminal_handle
        });

        terminal
    }

    fn activate_python_virtual_environment(
        &mut self,
        activate_command: &'static str,
        activate_script: Option<PathBuf>,
        terminal_handle: &Model<Terminal>,
        cx: &mut ModelContext<Project>,
    ) {
        if let Some(activate_script) = activate_script {
            // Paths are not strings so we need to jump through some hoops to format the command without `format!`
            let mut command = Vec::from(activate_command.as_bytes());
            command.push(b' ');
            // Wrapping path in double quotes to catch spaces in folder name
            command.extend_from_slice(b"\"");
            command.extend_from_slice(activate_script.as_os_str().as_encoded_bytes());
            command.extend_from_slice(b"\"");
            command.push(b'\n');

            terminal_handle.update(cx, |this, _| this.input_bytes(command));
        }
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakModel<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

// TODO: Add a few tests for adding and removing terminal tabs
