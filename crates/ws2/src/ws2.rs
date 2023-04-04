mod workspace_element;

use anyhow::{anyhow, Result};
use collections::HashMap;
use gpui::{
    actions, elements::*, AnyViewHandle, AppContext, Entity, ModelHandle, MutableAppContext, Task,
    View, ViewContext, ViewHandle,
};
use project::{Project, ProjectItem, ProjectItemHandle, WorktreePath};
use std::{
    any::{Any, TypeId},
    cmp,
    path::PathBuf,
};

actions!(ws2, [CloseActivePaneItem]);

type PaneId = usize;

type BuildProjectPaneItem = Box<
    dyn Fn(
        &ViewHandle<Workspace>,
        Box<dyn ProjectItemHandle>,
        &mut MutableAppContext,
    ) -> Box<dyn ProjectPaneItemHandle>,
>;
type ProjectPaneItemBuilders = HashMap<TypeId, BuildProjectPaneItem>;

type ConvertProjectPaneItemHandle = fn(AnyViewHandle) -> Box<dyn ProjectPaneItemHandle>;
type ProjectPaneItemHandleConverters = HashMap<TypeId, ConvertProjectPaneItemHandle>;

pub trait PaneItem: View {}

pub trait PaneItemHandle {
    fn to_project_pane_item(&self, cx: &AppContext) -> Option<Box<dyn ProjectPaneItemHandle>>;
    fn as_any(&self) -> &AnyViewHandle;
    fn boxed_clone(&self) -> Box<dyn PaneItemHandle>;
}

impl<T: PaneItem> PaneItemHandle for ViewHandle<T> {
    fn to_project_pane_item(&self, cx: &AppContext) -> Option<Box<dyn ProjectPaneItemHandle>> {
        let converter = cx
            .global::<ProjectPaneItemHandleConverters>()
            .get(&TypeId::of::<T>())?;
        Some((converter)(self.clone().into_any()))
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
    }

    fn boxed_clone(&self) -> Box<dyn PaneItemHandle> {
        Box::new(self.clone())
    }
}

pub trait ProjectPaneItem: PaneItem {
    type ProjectItem: ProjectItem;
    type Dependencies: Any;

    fn for_project_item(
        model: ModelHandle<Self::ProjectItem>,
        dependencies: &Self::Dependencies,
        cx: &mut ViewContext<Self>,
    ) -> Self;

    fn project_item(&self, cx: &AppContext) -> &ModelHandle<Self::ProjectItem>;
}

pub trait ProjectPaneItemHandle {
    fn project_item<'a>(&'a self, cx: &'a AppContext) -> &'a dyn ProjectItemHandle;
    fn as_any(&self) -> &AnyViewHandle;
    fn as_pane_item(&self) -> &dyn PaneItemHandle;
    fn boxed_clone(&self) -> Box<dyn ProjectPaneItemHandle>;
}

impl<T: ProjectPaneItem> ProjectPaneItemHandle for ViewHandle<T> {
    fn project_item<'a>(&'a self, cx: &'a AppContext) -> &'a dyn ProjectItemHandle {
        self.read(cx).project_item(cx)
    }

    fn as_any(&self) -> &AnyViewHandle {
        self
    }

    fn as_pane_item(&self) -> &dyn PaneItemHandle {
        self
    }

    fn boxed_clone(&self) -> Box<dyn ProjectPaneItemHandle> {
        Box::new(self.clone())
    }
}

pub struct Workspace {
    project: ModelHandle<Project>,
    pane_tree: PaneTree,
    next_pane_id: PaneId,
    active_pane_id: PaneId,
}

pub enum WorkspaceEvent {
    PathOpened(WorktreePath),
}

enum SplitOrientation {
    Horizontal,
    Vertical,
}

enum PaneTree {
    Split {
        orientation: SplitOrientation,
        children: Vec<PaneTree>,
    },
    Pane(Pane),
}

pub struct Pane {
    id: PaneId,
    items: Vec<Box<dyn PaneItemHandle>>,
    active_item_index: usize,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Workspace::close_active_pane_item);
}

pub fn register_project_pane_item<T: ProjectPaneItem>(
    dependencies: T::Dependencies,
    cx: &mut MutableAppContext,
) {
    cx.update_default_global(|builders: &mut ProjectPaneItemBuilders, _| {
        builders.insert(
            TypeId::of::<T::ProjectItem>(),
            Box::new(move |workspace, model, cx| {
                Box::new(cx.add_view(workspace, |cx| {
                    T::for_project_item(
                        model.as_any().clone().downcast().unwrap(),
                        &dependencies,
                        cx,
                    )
                }))
            }),
        );
    });

    cx.update_default_global(|converters: &mut ProjectPaneItemHandleConverters, _| {
        converters.insert(TypeId::of::<T>(), |any_handle| {
            Box::new(any_handle.downcast::<T>().unwrap())
        });
    });
}

fn build_project_pane_item(
    project_item: Box<dyn ProjectItemHandle>,
    cx: &mut ViewContext<Workspace>,
) -> Result<Box<dyn ProjectPaneItemHandle>> {
    let workspace = cx.handle();
    cx.update_default_global(|builders: &mut ProjectPaneItemBuilders, cx| {
        let builder = builders
            .get(&project_item.item_type())
            .ok_or_else(|| anyhow!("no ProjectPaneItem registered for model type"))?;
        Ok(builder(&workspace, project_item, cx))
    })
}

impl Entity for Workspace {
    type Event = WorkspaceEvent;
}

impl View for Workspace {
    fn ui_name() -> &'static str {
        "Workspace"
    }

    fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        Empty::new().boxed()
    }
}

impl Workspace {
    pub fn new(project: ModelHandle<Project>) -> Self {
        let pane_tree = PaneTree::new();
        Self {
            project,
            pane_tree,
            next_pane_id: 1,
            active_pane_id: 0,
        }
    }

    pub fn active_pane(&self) -> &Pane {
        self.pane_tree.pane(self.active_pane_id).unwrap()
    }

    pub fn active_pane_mut(&mut self) -> &mut Pane {
        self.pane_tree.pane_mut(self.active_pane_id).unwrap()
    }

    /// Opens the file at the given absolute path. If no worktree for that path
    /// exists in the project, one is added automatically. Then the path is opened
    /// with Self::open_path.
    pub fn open_abs_path(
        &self,
        abs_path: impl Into<PathBuf>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Option<Box<dyn ProjectPaneItemHandle>>>> {
        let worktree_path = self
            .project
            .update(cx, |project, cx| project.open_abs_path(abs_path, cx));

        cx.spawn(|this, mut cx| async move {
            let worktree_path = worktree_path.await?;
            this.update(&mut cx, |this, cx| this.open_path(worktree_path, cx))
                .await
        })
    }

    /// Open the given WorktreePath in the workspace if it exists. If the path
    /// points at a directory, emit an event notifying other parts of the UI
    /// that it was opened and return None. Otherwise activate or add a
    /// ProjectPaneItem to the active pane for the ProjectItem opened at this
    /// path.
    pub fn open_path(
        &mut self,
        path: WorktreePath,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Option<Box<dyn ProjectPaneItemHandle>>>> {
        let entry = self
            .project
            .update(cx, |project, cx| project.entry_for_path(&path, cx));

        cx.spawn(|this, mut cx| async move {
            let entry =
                entry.ok_or_else(|| anyhow!("could not find project entry for path {:?}", path))?;

            if entry.is_dir() {
                // If the entry is a directory, emit an event so that other parts of the UI
                // are notified, such as the project browser.
                this.update(&mut cx, |_, cx| cx.emit(WorkspaceEvent::PathOpened(path)));
                Ok(None)
            } else {
                // If the entry is a file, open a project item for it and display it in
                // the active pane.
                let project_item = this
                    .update(&mut cx, |this, cx| {
                        this.project
                            .update(cx, |project, cx| project.open_path2(path.clone(), cx))
                    })
                    .await?;

                this.update(&mut cx, |this, cx| {
                    let active_pane = this.active_pane_mut();
                    let pane_item = if let Some(existing_item) =
                        active_pane.activate_project_item(project_item.as_ref(), cx)
                    {
                        existing_item
                    } else {
                        let project_pane_item = build_project_pane_item(project_item, cx)?;
                        active_pane.add_item(project_pane_item.as_pane_item().boxed_clone(), cx);
                        project_pane_item
                    };
                    cx.emit(WorkspaceEvent::PathOpened(path));
                    Ok(Some(pane_item))
                })
            }
        })
    }

    // Actions

    fn close_active_pane_item(&mut self, _: &CloseActivePaneItem, cx: &mut ViewContext<Self>) {
        if !self.active_pane_mut().close_active_item(cx) {
            cx.propagate_action(); // If pane was empty, there's no item to close
        }
    }
}

impl PaneTree {
    fn new() -> Self {
        PaneTree::Pane(Pane::new(0))
    }

    fn pane(&self, id: PaneId) -> Option<&Pane> {
        match self {
            PaneTree::Split { children, .. } => children[0].pane(id),
            PaneTree::Pane(pane) => {
                if pane.id == id {
                    Some(pane)
                } else {
                    None
                }
            }
        }
    }

    fn pane_mut(&mut self, pane_id: PaneId) -> Option<&mut Pane> {
        match self {
            PaneTree::Split { children, .. } => {
                for child in children {
                    if let Some(pane) = child.pane_mut(pane_id) {
                        return Some(pane);
                    }
                }
                None
            }
            PaneTree::Pane(pane) => {
                if pane.id == pane_id {
                    Some(pane)
                } else {
                    None
                }
            }
        }
    }
}

impl Pane {
    fn new(id: PaneId) -> Self {
        Self {
            id,
            items: Vec::new(),
            active_item_index: 0,
        }
    }

    /// If there's a pane item corresponding to the given project item handle,
    /// activate it and return a handle to it. Otherwise do nothing and return
    /// None.
    ///
    /// This helps us avoid opening multiple pane items for the same project
    /// item in a single pane.
    fn activate_project_item(
        &mut self,
        new_item: &dyn ProjectItemHandle,
        cx: &mut ViewContext<Workspace>,
    ) -> Option<Box<dyn ProjectPaneItemHandle>> {
        let new_entry_id = new_item.entry_id(cx)?;
        let (found_ix, found_item) =
            self.items.iter().enumerate().find_map(|(ix, pane_item)| {
                let project_pane_item = pane_item.to_project_pane_item(cx)?;
                let entry_id = project_pane_item.project_item(cx).entry_id(cx)?;
                if entry_id == new_entry_id {
                    Some((ix, project_pane_item))
                } else {
                    None
                }
            })?;

        self.active_item_index = found_ix;
        cx.notify();
        Some(found_item)
    }

    fn items(&self) -> &[Box<dyn PaneItemHandle>] {
        &self.items
    }

    fn active_item(&self) -> Option<&dyn PaneItemHandle> {
        self.items
            .get(self.active_item_index)
            .map(|item| item.as_ref())
    }

    fn add_item(&mut self, item: Box<dyn PaneItemHandle>, cx: &mut ViewContext<Workspace>) {
        let ix = cmp::min(self.items.len(), self.active_item_index + 1);
        self.items.splice(ix..ix, Some(item));
        self.active_item_index = ix;
        cx.notify();
    }

    fn close_active_item(&mut self, cx: &mut ViewContext<Workspace>) -> bool {
        if self.items.is_empty() {
            false
        } else {
            self.items
                .splice(self.active_item_index..self.active_item_index + 1, []);
            cx.notify();
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use gpui::{serde_json::json, TestAppContext};
    use language::Buffer;
    use project::Project;
    use std::path::Path;

    #[gpui::test]
    async fn test_ws2_workspace(cx: &mut TestAppContext) {
        // Register TestEditor as the ProjectPaneItem for buffer
        cx.update(|cx| register_project_pane_item::<TestEditor>((), cx));

        let fs = FakeFs::new(cx.background());
        fs.insert_tree(
            "/root1",
            json!({
                "a": "",
                "b": ""
            }),
        )
        .await;

        let project = Project::test(fs, [], cx).await;
        let (_, workspace) = cx.add_window(|_| Workspace::new(project.clone()));

        // Project starts empty
        assert!(project.read_with(cx, |project, cx| project.worktrees(&cx).next().is_none()));

        // Then we open a direcotry via the workspace
        let result = workspace
            .update(cx, |workspace, cx| workspace.open_abs_path("/root1", cx))
            .await;
        // The result is None because we can't actually open a pane item for a directory
        assert!(result.unwrap().is_none());

        // But we add a worktree for that directory to the project
        project
            .read_with(cx, |project, cx| {
                let worktrees = project.worktrees(cx).collect::<Vec<_>>();
                assert_eq!(worktrees.len(), 1);
                let worktree = worktrees[0].read(cx);
                assert_eq!(worktree.abs_path().as_ref(), Path::new("/root1"));
                worktree.as_local().unwrap().scan_complete()
            })
            .await; // Wait for the worktree's scan to complete

        // Now open a file in the worktree
        let editor_1 = workspace
            .update(cx, |workspace, cx| workspace.open_abs_path("/root1/a", cx))
            .await
            .unwrap() // Result
            .unwrap() // Option
            .as_any() // Cast to AnyViewHandle
            .clone() // Get an owned value instead of a reference
            .downcast::<TestEditor>() // Downcast to the expected type
            .unwrap();

        // The opened editor has the expected path, and it is the active pane item.
        workspace.read_with(cx, |workspace, cx| {
            let file = editor_1.read(cx).0.read(cx).file().unwrap();
            assert_eq!(file.full_path(cx), Path::new("root1/a"));
            let active_item = workspace.active_pane().active_item().unwrap().as_any();
            assert_eq!(active_item, &editor_1);
        });

        // Now open a second file in the same worktree
        let editor_2 = workspace
            .update(cx, |workspace, cx| workspace.open_abs_path("/root1/b", cx))
            .await
            .unwrap()
            .unwrap()
            .as_any()
            .clone()
            .downcast::<TestEditor>()
            .unwrap();

        // The opened editor has the expected path, and it is the active pane item.
        workspace.read_with(cx, |workspace, cx| {
            let file = editor_2.read(cx).0.read(cx).file().unwrap();
            assert_eq!(file.full_path(cx), Path::new("root1/b"));
            let active_item = workspace.active_pane().active_item().unwrap().as_any();
            assert_eq!(active_item, &editor_2);
        });

        // Now open a path for which we already have an editor open
        // Now open a file in the worktree
        let editor_1b = workspace
            .update(cx, |workspace, cx| workspace.open_abs_path("/root1/a", cx))
            .await
            .unwrap() // Result
            .unwrap() // Option
            .as_any() // Cast to AnyViewHandle
            .clone() // Get an owned value instead of a reference
            .downcast::<TestEditor>() // Downcast to the expected type
            .unwrap();

        // We return a handle to the existing editor.
        assert_eq!(editor_1b, editor_1);

        workspace.read_with(cx, |workspace, _| {
            assert_eq!(workspace.active_pane().items().len(), 2);
            let active_item = workspace.active_pane().active_item().unwrap().as_any();
            assert_eq!(active_item, &editor_1);
        });
    }

    struct TestEditor(ModelHandle<Buffer>);

    impl Entity for TestEditor {
        type Event = ();
    }
    impl View for TestEditor {
        fn ui_name() -> &'static str {
            "TestEditor"
        }

        fn render(&mut self, _: &mut gpui::RenderContext<'_, Self>) -> ElementBox {
            Empty::new().boxed()
        }
    }
    impl PaneItem for TestEditor {}
    impl ProjectPaneItem for TestEditor {
        type ProjectItem = Buffer;
        type Dependencies = ();

        fn for_project_item(
            buffer: ModelHandle<Buffer>,
            _: &(),
            _: &mut ViewContext<Self>,
        ) -> Self {
            Self(buffer)
        }

        fn project_item(&self, _: &AppContext) -> &ModelHandle<Self::ProjectItem> {
            &self.0
        }
    }
}
