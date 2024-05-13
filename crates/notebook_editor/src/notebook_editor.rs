#[allow(unused_imports)]
use gpui::{
    canvas, div, fill, img, opaque_grey, point, size, AnyElement, AppContext, Bounds, Context,
    EventEmitter, FocusHandle, FocusableView, Img, InteractiveElement, IntoElement, Model,
    ObjectFit, ParentElement, Render, Styled, Task, View, ViewContext, VisualContext, WeakView,
    WindowContext,
};
use persistence::NOTEBOOK_EDITOR;
use ui::prelude::*;

use project::{Project, ProjectEntryId, ProjectPath};
use std::{ffi::OsStr, path::PathBuf};
use util::ResultExt;
use workspace::{
    item::{Item, ProjectItem, TabContentParams},
    ItemId, Pane, Workspace, WorkspaceId,
};

const NOTEBOOK_EDITOR_KIND: &str = "NotebookEditor";

pub struct NotebookItem {
    path: PathBuf,
    project_path: ProjectPath,
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
            Some(cx.spawn(|mut cx| async move {
                let abs_path = project
                    .read_with(&cx, |project, cx| project.absolute_path(&path, cx))?
                    .ok_or_else(|| anyhow::anyhow!("Failed to find the absolute path"))?;

                cx.new_model(|_| NotebookItem {
                    path: abs_path,
                    project_path: path,
                })
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &AppContext) -> Option<ProjectEntryId> {
        None
    }

    fn project_path(&self, _: &AppContext) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }
}

pub struct NotebookEditor {
    path: PathBuf,
    focus_handle: FocusHandle,
}

impl Item for NotebookEditor {
    type Event = ();

    fn tab_content(&self, params: TabContentParams, _cx: &WindowContext) -> AnyElement {
        let title = self
            .path
            .file_name()
            .unwrap_or_else(|| self.path.as_os_str())
            .to_string_lossy()
            .to_string();
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
        let image_path = self.path.clone();

        cx.background_executor()
            .spawn({
                let image_path = image_path.clone();
                async move {
                    NOTEBOOK_EDITOR
                        .save_notebook_path(item_id, workspace_id, image_path)
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
            let image_path = NOTEBOOK_EDITOR
                .get_notebook_path(item_id, workspace_id)?
                .ok_or_else(|| anyhow::anyhow!("No image path found"))?;

            cx.new_view(|cx| NotebookEditor {
                path: image_path,
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
            path: self.path.clone(),
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
        _project: Model<Project>,
        item: Model<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            path: item.read(cx).path.clone(),
            focus_handle: cx.focus_handle(),
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
                image_path: PathBuf
            ) -> Result<()> {
                INSERT OR REPLACE INTO notebooks(item_id, workspace_id, image_path)
                VALUES (?, ?, ?)
            }
        }

        query! {
            pub fn get_notebook_path(item_id: ItemId, workspace_id: WorkspaceId) -> Result<Option<PathBuf>> {
                SELECT image_path
                FROM notebooks
                WHERE item_id = ? AND workspace_id = ?
            }
        }
    }
}
