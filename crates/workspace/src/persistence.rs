use std::{
    any::{Any, TypeId},
    os::unix::prelude::OsStrExt,
    path::PathBuf,
};

use anyhow::Result;

use async_recursion::async_recursion;
use collections::HashMap;
use gpui::{
    geometry::{rect::RectF, vector::vec2f},
    AsyncAppContext, Axis, ModelHandle, ViewHandle, WindowBounds,
};

use project::Project;
use serde::{Deserialize, Serialize};
use store::{Record, Store};

use crate::{
    dock::DockPosition,
    pane::{Pane, PaneState},
    Member, Workspace,
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct WorkspaceState {
    pub worktree_abs_paths: Vec<PathBuf>,
    pub dock_position: DockPosition,
    pub center_group: PaneGroupState,
    pub dock_pane: PaneState,
    pub left_sidebar_open: bool,
    pub bounds: WindowBoundsState,
    pub screen_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "type")]
pub enum WindowBoundsState {
    Fullscreen,
    #[default]
    Maximized,
    Fixed {
        origin_x: f32,
        origin_y: f32,
        width: f32,
        height: f32,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum PaneGroupState {
    Group {
        axis: Axis,
        members: Vec<PaneGroupState>,
    },
    Pane(PaneState),
}

impl Record for WorkspaceState {
    fn namespace() -> &'static str {
        "WorkspaceState"
    }

    fn schema_version() -> u64 {
        0
    }

    fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    fn deserialize(_: u64, data: Vec<u8>) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice(&data)?)
    }
}

impl WorkspaceState {
    pub fn storage_key(mut abs_paths: Vec<PathBuf>) -> Vec<u8> {
        abs_paths.sort_unstable();
        let mut key = Vec::new();
        for abs_path in &abs_paths {
            key.extend_from_slice(&abs_path.as_os_str().as_bytes());
            key.push(0);
        }
        key
    }

    pub async fn load_items<'a>(
        &'a self,
        store: &'a Store,
        cx: AsyncAppContext,
    ) -> HashMap<TypeId, HashMap<u64, Box<dyn Any>>> {
        let mut item_states = HashMap::default();
        self.center_group
            .load_items(store, &mut item_states, cx.clone())
            .await;
        self.dock_pane.load_items(store, &mut item_states, cx).await;
        item_states
    }
}

impl WindowBoundsState {
    pub fn from_window_bounds(bounds: WindowBounds) -> Self {
        match bounds {
            WindowBounds::Fullscreen => Self::Fullscreen,
            WindowBounds::Maximized => Self::Maximized,
            WindowBounds::Fixed(bounds) => Self::Fixed {
                origin_x: bounds.origin_x(),
                origin_y: bounds.origin_y(),
                width: bounds.width(),
                height: bounds.height(),
            },
        }
    }

    pub fn to_window_bounds(&self) -> WindowBounds {
        match self {
            WindowBoundsState::Fullscreen => WindowBounds::Fullscreen,
            WindowBoundsState::Maximized => WindowBounds::Maximized,
            WindowBoundsState::Fixed {
                origin_x,
                origin_y,
                width,
                height,
            } => WindowBounds::Fixed(RectF::new(
                vec2f(*origin_x, *origin_y),
                vec2f(*width, *height),
            )),
        }
    }
}

#[cfg(test)]
impl Default for PaneGroupState {
    fn default() -> Self {
        Self::Pane(PaneState {
            items: vec![crate::ItemState::default()],
            active: false,
        })
    }
}

impl PaneGroupState {
    #[async_recursion(?Send)]
    pub async fn load_items<'a>(
        &'a self,
        store: &'a Store,
        items: &'a mut HashMap<TypeId, HashMap<u64, Box<dyn Any>>>,
        cx: AsyncAppContext,
    ) {
        match self {
            PaneGroupState::Group { members, .. } => {
                for member in members {
                    member.load_items(store, items, cx.clone()).await;
                }
            }
            PaneGroupState::Pane(pane) => pane.load_items(store, items, cx).await,
        }
    }

    #[async_recursion(?Send)]
    pub(crate) async fn deserialize(
        &self,
        project: &ModelHandle<Project>,
        workspace: &ViewHandle<Workspace>,
        cx: &mut AsyncAppContext,
    ) -> Option<(Member, Option<ViewHandle<Pane>>)> {
        todo!()
        // match self {
        //     PaneGroupState::Group { axis, members } => {
        //         let mut current_active_pane = None;
        //         let mut members = Vec::new();
        //         for child in members {
        //             if let Some((new_member, active_pane)) =
        //                 child.deserialize(project, workspace, cx).await
        //             {
        //                 members.push(new_member);
        //                 current_active_pane = current_active_pane.or(active_pane);
        //             }
        //         }

        //         if members.is_empty() {
        //             return None;
        //         }

        //         if members.len() == 1 {
        //             return Some((members.remove(0), current_active_pane));
        //         }

        //         Some((
        //             Member::Axis(PaneAxis {
        //                 axis: *axis,
        //                 members,
        //             }),
        //             current_active_pane,
        //         ))
        //     }
        //     PaneGroupState::Pane(serialized_pane) => {
        //         let pane = workspace.update(cx, |workspace, cx| workspace.add_pane(cx));
        //         let active = serialized_pane.active;
        //         serialized_pane
        //             .deserialize_to(project, &pane, workspace, cx)
        //             .await;

        //         if pane.read_with(cx, |pane, _| pane.items_len() != 0) {
        //             Some((Member::Pane(pane.clone()), active.then(|| pane)))
        //         } else {
        //             workspace.update(cx, |workspace, cx| workspace.remove_pane(pane, cx));
        //             None
        //         }
        //     }
        // }
    }
}
