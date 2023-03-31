mod workspace_element;

use std::path::PathBuf;

use anyhow::Result;
use gpui::{actions, Entity, ModelHandle, MutableAppContext, Task, View, ViewContext};
use project::{Project, ProjectPath};

actions!(ws2, [CloseActivePaneItem]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Workspace::close_active_pane_item);
}

type PaneId = usize;

pub trait PaneItem: View {}

pub trait PaneItemHandle {}

pub struct Workspace {
    project: ModelHandle<Project>,
    pane_tree: PaneTree,
    next_pane_id: PaneId,
    active_pane_id: PaneId,
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

impl Entity for Workspace {
    type Event = ();
}

impl View for Workspace {
    fn ui_name() -> &'static str {
        "Workspace"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        todo!()
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

    pub fn active_pane_mut(&mut self) -> &mut Pane {
        self.pane_tree.pane_mut(self.active_pane_id).unwrap()
    }

    pub fn open_abs_path(
        &self,
        abs_path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<Box<dyn PaneItemHandle>>> {
        let project_path = self
            .project
            .update(cx, |project, cx| project.open_abs_path(abs_path, cx));

        cx.spawn(|this, cx| async move {
            let project_path = project_path.await?;

            Ok(todo!())
        })
    }

    pub fn open_project_path(
        &self,
        project_path: ProjectPath,
        cx: &mut ViewContext<Self>,
    ) -> Task<Box<dyn PaneItemHandle>> {
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

    fn add_item(&mut self, item: Box<dyn PaneItemHandle>, cx: &mut ViewContext<Workspace>) {
        self.items.push(item);
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
