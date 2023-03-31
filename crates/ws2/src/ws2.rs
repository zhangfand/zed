mod workspace_element;

use gpui::{Entity, ModelHandle, View};
use project::Project;

pub trait PaneItem: View {}

pub trait PaneItemHandle {}

pub struct Workspace {
    panes: PaneTree,
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

struct Pane {
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
        todo!()
    }
}

impl PaneTree {
    fn new() -> Self {
        PaneTree::Pane(Pane::new())
    }
}

impl Pane {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            active_item_index: 0,
        }
    }
}
