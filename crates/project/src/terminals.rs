use std::path::PathBuf;

use crate::Project;
use gpui::{AnyWindowHandle, Context, Entity, Model, ModelContext, WeakModel};
use python::{self, python_settings::PythonSettings, Interpreter};
use settings::Settings;
use smol::channel::bounded;
use terminal::{
    terminal_settings::{Shell, TerminalSettings},
    SpawnTask, TaskState, Terminal, TerminalBuilder,
};
use util::ResultExt;

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

            if activate_venv_on_launch {
                // TODO INTERPRETER
                let worktree_path =
                    PathBuf::from("/Users/josephlyons/Desktop/pydantic_test".to_string());
                if let Some(interpreter) =
                    Interpreter::retrieve_from_local_settings(worktree_path.into()).log_err()
                {
                    if let Some(command) = interpreter
                        .activation_script_command(&python_settings)
                        .log_err()
                    {
                        terminal_handle.update(cx, |this, _| this.input_bytes(command));
                    };
                };
            }
            terminal_handle
        });

        terminal
    }

    pub fn local_terminal_handles(&self) -> &Vec<WeakModel<terminal::Terminal>> {
        &self.terminals.local_handles
    }
}

// TODO: Add a few tests for adding and removing terminal tabs
