use editor::Editor;
#[allow(unused_imports)]
use gpui::ModelContext;
#[allow(unused_imports)]
use gpui::{
    canvas, div, fill, img, opaque_grey, point, size, AnyElement, AppContext, Bounds, Context,
    EventEmitter, FocusHandle, FocusableView, Img, InteractiveElement, IntoElement, Model,
    ObjectFit, ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};
use language::{Buffer, File as _};
use persistence::NOTEBOOK_EDITOR;
use project::{Item, Project, ProjectEntryId, ProjectPath};
#[allow(unused_imports)]
use std::{ffi::OsStr, path::PathBuf, sync::Arc};
use ui::prelude::*;
use util::ResultExt;
use workspace::{
    item::{ProjectItem, TabContentParams},
    ItemId, Pane, Workspace, WorkspaceId,
};
use worktree::File;

const NOTEBOOK_EDITOR_KIND: &str = "NotebookEditor";

// We either need to store the notebook in memory as individual cells or we make
// one big buffer that contains all the cells. Nate suggested that we try out a multi-buffer.
// For now, I'm tempted to use the approach from assistant2, where we had individual entries in a
// GPUI List. That won't be as performant as a single buffer, but it'll be the most accurate.
pub struct Output {}

#[allow(dead_code)]
pub struct CodeCell {
    source: View<Editor>,
    outputs: Vec<Output>,
}

#[allow(dead_code)]
pub struct MarkdownCell {
    source: View<Editor>,
}

/// Raw cell is a cell that contains raw text, for fairly arcane purposes.
/// Just render a text cell.
#[allow(dead_code)]
pub struct RawCell {
    source: View<Editor>,
}

#[allow(dead_code)]
enum NotebookCell {
    CodeCell(CodeCell),
    MarkdownCell(MarkdownCell),
}

#[allow(dead_code)]
pub struct NotebookItem {
    // path: PathBuf,
    project_path: ProjectPath,
    buffer: Model<Buffer>,
    cells: Vec<NotebookCell>,
}

impl project::Item for NotebookItem {
    fn try_open(
        project: &Model<Project>,
        path: &ProjectPath,
        cx: &mut AppContext,
    ) -> Option<Task<gpui::Result<Model<Self>>>> {
        let path = path.clone();
        let project = project.clone();

        let ext = path
            .path
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or_default();

        // Only open the item if it's a Jupyter notebook (.ipynb)
        if ext.contains("ipynb") {
            let buffer_task =
                project.update(cx, |project, cx| project.open_buffer(path.clone(), cx));

            Some(cx.spawn(|mut cx| async move {
                let buffer = buffer_task.await?;

                // let abs_path = project
                //     .read_with(&cx, |project, cx| project.absolute_path(&path, cx))?
                //     .ok_or_else(|| anyhow::anyhow!("Failed to find the absolute path"))?;

                // We probably read the cells in ~this point.

                cx.new_model(|_| NotebookItem {
                    // path: abs_path,
                    project_path: path,
                    buffer,
                    cells: vec![],
                })
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, cx: &AppContext) -> Option<ProjectEntryId> {
        File::from_dyn(self.buffer.read(cx).file()).and_then(|file| file.project_entry_id(cx))
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        File::from_dyn(self.buffer.read(cx).file()).map(|file| ProjectPath {
            worktree_id: file.worktree_id(cx),
            path: file.path().clone(),
        })
    }
}

pub struct NotebookEditor {
    focus_handle: FocusHandle,
    notebook: Model<NotebookItem>,
}

impl workspace::item::Item for NotebookEditor {
    type Event = ();

    fn tab_content(&self, params: TabContentParams, cx: &WindowContext) -> AnyElement {
        let project_path = self.notebook.read(cx).buffer.read(cx).project_path(cx);

        let title = project_path
            .map(|path| path.path.file_name().unwrap().to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled.ipynb".to_string());

        Label::new(title)
            .single_line()
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .italic(params.preview)
            .into_any_element()
    }

    fn added_to_workspace(&mut self, workspace: &mut Workspace, cx: &mut ViewContext<Self>) {
        let item_id = cx.entity_id().as_u64();
        let workspace_id = workspace.database_id();

        let notebook_path = self
            .notebook
            .read(cx)
            .project_path
            .path
            .clone()
            .to_path_buf();

        cx.background_executor()
            .spawn({
                let notebook_path = notebook_path.clone();
                async move {
                    NOTEBOOK_EDITOR
                        .save_notebook_path(item_id, workspace_id, notebook_path)
                        .await
                        .log_err();
                }
            })
            .detach();
    }

    fn serialized_item_kind() -> Option<&'static str> {
        Some(NOTEBOOK_EDITOR_KIND)
    }

    fn deserialize(
        _project: Model<Project>,
        _workspace: WeakView<Workspace>,
        workspace_id: WorkspaceId,
        item_id: ItemId,
        cx: &mut ViewContext<Pane>,
    ) -> Task<anyhow::Result<View<Self>>> {
        cx.spawn(|_pane, mut cx| async move {
            let notebook_path = NOTEBOOK_EDITOR
                .get_notebook_path(item_id, workspace_id)?
                .ok_or_else(|| anyhow::anyhow!("No image path found"))?;

            cx.new_view(|cx| NotebookEditor {
                notebook: todo!(),
                focus_handle: cx.focus_handle(),
            })
        })
    }

    fn clone_on_split(
        &self,
        _workspace_id: WorkspaceId,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(|cx| Self {
            notebook: todo!(),
            focus_handle: cx.focus_handle(),
        }))
    }
}

impl EventEmitter<()> for NotebookEditor {}
impl FocusableView for NotebookEditor {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for NotebookEditor {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child("Notebook Viewer")
    }
}

impl ProjectItem for NotebookEditor {
    type Item = NotebookItem;

    fn for_project_item(
        project: Model<Project>,
        item: Model<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            focus_handle: cx.focus_handle(),
            notebook: item,
        }
    }
}

pub fn init(cx: &mut AppContext) {
    workspace::register_project_item::<NotebookEditor>(cx);
    workspace::register_deserializable_item::<NotebookEditor>(cx)
}

mod persistence {
    use std::path::PathBuf;

    use db::{define_connection, query, sqlez_macros::sql};
    use workspace::{ItemId, WorkspaceDb, WorkspaceId};

    define_connection! {
        pub static ref NOTEBOOK_EDITOR: NotebookDb<WorkspaceDb> =
            &[sql!(
                CREATE TABLE notebooks (
                    workspace_id INTEGER,
                    item_id INTEGER UNIQUE,

                    notebook_path BLOB,

                    PRIMARY KEY(workspace_id, item_id),
                    FOREIGN KEY(workspace_id) REFERENCES workspaces(workspace_id)
                    ON DELETE CASCADE
                ) STRICT;
            )];
    }

    impl NotebookDb {
        query! {
           pub async fn update_workspace_id(
                new_id: WorkspaceId,
                old_id: WorkspaceId,
                item_id: ItemId
            ) -> Result<()> {
                UPDATE notebooks
                SET workspace_id = ?
                WHERE workspace_id = ? AND item_id = ?
            }
        }

        query! {
            pub async fn save_notebook_path(
                item_id: ItemId,
                workspace_id: WorkspaceId,
                notebook_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO notebooks(item_id, workspace_id, notebook_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_notebook_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT notebook_path
                FROM notebooks
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
