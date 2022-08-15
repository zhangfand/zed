use std::{
    any::{Any, TypeId},
    borrow::Cow,
    cell::RefCell,
    fmt, mem,
    path::PathBuf,
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering::SeqCst},
    time::Duration,
};

use anyhow::Result;
use client::proto;
use futures::{channel::oneshot, FutureExt};
use gpui::{
    AnyViewHandle, AppContext, ElementBox, ModelHandle, MutableAppContext, Task, View, ViewContext,
    ViewHandle, WeakViewHandle,
};
use project::{Project, ProjectEntryId, ProjectPath};
use settings::{Autosave, Settings};
use smallvec::SmallVec;
use util::ResultExt;

use crate::{pane, FollowableItemBuilders, ItemNavHistory, Pane, Workspace};

pub trait Item: View {
    fn deactivated(&mut self, _: &mut ViewContext<Self>) {}
    fn navigate(&mut self, _: Box<dyn Any>, _: &mut ViewContext<Self>) -> bool {
        false
    }
    fn tab_description<'a>(&'a self, _: usize, _: &'a AppContext) -> Option<Cow<'a, str>> {
        None
    }
    fn tab_content(&self, detail: Option<usize>, style: &theme::Tab, cx: &AppContext)
        -> ElementBox;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]>;
    fn is_singleton(&self, cx: &AppContext) -> bool;
    fn set_nav_history(&mut self, _: ItemNavHistory, _: &mut ViewContext<Self>);
    fn clone_on_split(&self, _: &mut ViewContext<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
    fn is_dirty(&self, _: &AppContext) -> bool {
        false
    }
    fn has_conflict(&self, _: &AppContext) -> bool {
        false
    }
    fn can_save(&self, cx: &AppContext) -> bool;
    fn save(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>>;
    fn save_as(
        &mut self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>>;
    fn reload(
        &mut self,
        project: ModelHandle<Project>,
        cx: &mut ViewContext<Self>,
    ) -> Task<Result<()>>;
    fn should_close_item_on_event(_: &Self::Event) -> bool {
        false
    }
    fn should_update_tab_on_event(_: &Self::Event) -> bool {
        false
    }
    fn is_edit_event(_: &Self::Event) -> bool {
        false
    }
    fn act_as_type(
        &self,
        type_id: TypeId,
        self_handle: &ViewHandle<Self>,
        _: &AppContext,
    ) -> Option<AnyViewHandle> {
        if TypeId::of::<Self>() == type_id {
            Some(self_handle.into())
        } else {
            None
        }
    }
}

pub trait ProjectItem: Item {
    type Item: project::Item;

    fn for_project_item(
        project: ModelHandle<Project>,
        item: ModelHandle<Self::Item>,
        cx: &mut ViewContext<Self>,
    ) -> Self;
}

pub trait FollowableItem: Item {
    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn from_state_proto(
        pane: ViewHandle<Pane>,
        project: ModelHandle<Project>,
        state: &mut Option<proto::view::Variant>,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<ViewHandle<Self>>>>;
    fn add_event_to_update_proto(
        &self,
        event: &Self::Event,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool;
    fn apply_update_proto(
        &mut self,
        message: proto::update_view::Variant,
        cx: &mut ViewContext<Self>,
    ) -> Result<()>;

    fn set_leader_replica_id(&mut self, leader_replica_id: Option<u16>, cx: &mut ViewContext<Self>);
    fn should_unfollow_on_event(event: &Self::Event, cx: &AppContext) -> bool;
}

pub trait FollowableItemHandle: ItemHandle {
    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut MutableAppContext);
    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant>;
    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool;
    fn apply_update_proto(
        &self,
        message: proto::update_view::Variant,
        cx: &mut MutableAppContext,
    ) -> Result<()>;
    fn should_unfollow_on_event(&self, event: &dyn Any, cx: &AppContext) -> bool;
}

impl<T: FollowableItem> FollowableItemHandle for ViewHandle<T> {
    fn set_leader_replica_id(&self, leader_replica_id: Option<u16>, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| {
            this.set_leader_replica_id(leader_replica_id, cx)
        })
    }

    fn to_state_proto(&self, cx: &AppContext) -> Option<proto::view::Variant> {
        self.read(cx).to_state_proto(cx)
    }

    fn add_event_to_update_proto(
        &self,
        event: &dyn Any,
        update: &mut Option<proto::update_view::Variant>,
        cx: &AppContext,
    ) -> bool {
        if let Some(event) = event.downcast_ref() {
            self.read(cx).add_event_to_update_proto(event, update, cx)
        } else {
            false
        }
    }

    fn apply_update_proto(
        &self,
        message: proto::update_view::Variant,
        cx: &mut MutableAppContext,
    ) -> Result<()> {
        self.update(cx, |this, cx| this.apply_update_proto(message, cx))
    }

    fn should_unfollow_on_event(&self, event: &dyn Any, cx: &AppContext) -> bool {
        if let Some(event) = event.downcast_ref() {
            T::should_unfollow_on_event(event, cx)
        } else {
            false
        }
    }
}

pub trait ItemHandle: 'static + fmt::Debug {
    fn tab_description<'a>(&self, detail: usize, cx: &'a AppContext) -> Option<Cow<'a, str>>;
    fn tab_content(&self, detail: Option<usize>, style: &theme::Tab, cx: &AppContext)
        -> ElementBox;
    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath>;
    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]>;
    fn is_singleton(&self, cx: &AppContext) -> bool;
    fn boxed_clone(&self) -> Box<dyn ItemHandle>;
    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemHandle>>;
    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    );
    fn deactivated(&self, cx: &mut MutableAppContext);
    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext) -> bool;
    fn id(&self) -> usize;
    fn to_any(&self) -> AnyViewHandle;
    fn is_dirty(&self, cx: &AppContext) -> bool;
    fn has_conflict(&self, cx: &AppContext) -> bool;
    fn can_save(&self, cx: &AppContext) -> bool;
    fn save(&self, project: ModelHandle<Project>, cx: &mut MutableAppContext) -> Task<Result<()>>;
    fn save_as(
        &self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut MutableAppContext,
    ) -> Task<Result<()>>;
    fn reload(&self, project: ModelHandle<Project>, cx: &mut MutableAppContext)
        -> Task<Result<()>>;
    fn act_as_type(&self, type_id: TypeId, cx: &AppContext) -> Option<AnyViewHandle>;
    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>>;
    fn on_release(
        &self,
        cx: &mut MutableAppContext,
        callback: Box<dyn FnOnce(&mut MutableAppContext)>,
    ) -> gpui::Subscription;
}

pub trait WeakItemHandle {
    fn id(&self) -> usize;
    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>>;
}

impl dyn ItemHandle {
    pub fn downcast<T: View>(&self) -> Option<ViewHandle<T>> {
        self.to_any().downcast()
    }

    pub fn act_as<T: View>(&self, cx: &AppContext) -> Option<ViewHandle<T>> {
        self.act_as_type(TypeId::of::<T>(), cx)
            .and_then(|t| t.downcast())
    }
}

impl<T: Item> ItemHandle for ViewHandle<T> {
    fn tab_description<'a>(&self, detail: usize, cx: &'a AppContext) -> Option<Cow<'a, str>> {
        self.read(cx).tab_description(detail, cx)
    }

    fn tab_content(
        &self,
        detail: Option<usize>,
        style: &theme::Tab,
        cx: &AppContext,
    ) -> ElementBox {
        self.read(cx).tab_content(detail, style, cx)
    }

    fn project_path(&self, cx: &AppContext) -> Option<ProjectPath> {
        self.read(cx).project_path(cx)
    }

    fn project_entry_ids(&self, cx: &AppContext) -> SmallVec<[ProjectEntryId; 3]> {
        self.read(cx).project_entry_ids(cx)
    }

    fn is_singleton(&self, cx: &AppContext) -> bool {
        self.read(cx).is_singleton(cx)
    }

    fn boxed_clone(&self) -> Box<dyn ItemHandle> {
        Box::new(self.clone())
    }

    fn clone_on_split(&self, cx: &mut MutableAppContext) -> Option<Box<dyn ItemHandle>> {
        self.update(cx, |item, cx| {
            cx.add_option_view(|cx| item.clone_on_split(cx))
        })
        .map(|handle| Box::new(handle) as Box<dyn ItemHandle>)
    }

    fn added_to_pane(
        &self,
        workspace: &mut Workspace,
        pane: ViewHandle<Pane>,
        cx: &mut ViewContext<Workspace>,
    ) {
        let history = pane.read(cx).nav_history_for_item(self);
        self.update(cx, |this, cx| this.set_nav_history(history, cx));

        if let Some(followed_item) = self.to_followable_item_handle(cx) {
            if let Some(message) = followed_item.to_state_proto(cx) {
                workspace.update_followers(
                    proto::update_followers::Variant::CreateView(proto::View {
                        id: followed_item.id() as u64,
                        variant: Some(message),
                        leader_id: workspace.leader_for_pane(&pane).map(|id| id.0),
                    }),
                    cx,
                );
            }
        }

        let mut pending_autosave = None;
        let mut cancel_pending_autosave = oneshot::channel::<()>().0;
        let pending_update = Rc::new(RefCell::new(None));
        let pending_update_scheduled = Rc::new(AtomicBool::new(false));
        let pane = pane.downgrade();
        cx.subscribe(self, move |workspace, item, event, cx| {
            let pane = if let Some(pane) = pane.upgrade(cx) {
                pane
            } else {
                log::error!("unexpected item event after pane was dropped");
                return;
            };

            if let Some(item) = item.to_followable_item_handle(cx) {
                let leader_id = workspace.leader_for_pane(&pane);

                if leader_id.is_some() && item.should_unfollow_on_event(event, cx) {
                    workspace.unfollow(&pane, cx);
                }

                if item.add_event_to_update_proto(event, &mut *pending_update.borrow_mut(), cx)
                    && !pending_update_scheduled.load(SeqCst)
                {
                    pending_update_scheduled.store(true, SeqCst);
                    cx.after_window_update({
                        let pending_update = pending_update.clone();
                        let pending_update_scheduled = pending_update_scheduled.clone();
                        move |this, cx| {
                            pending_update_scheduled.store(false, SeqCst);
                            this.update_followers(
                                proto::update_followers::Variant::UpdateView(proto::UpdateView {
                                    id: item.id() as u64,
                                    variant: pending_update.borrow_mut().take(),
                                    leader_id: leader_id.map(|id| id.0),
                                }),
                                cx,
                            );
                        }
                    });
                }
            }

            if T::should_close_item_on_event(event) {
                Pane::close_item(workspace, pane, item.id(), cx).detach_and_log_err(cx);
                return;
            }

            if T::should_update_tab_on_event(event) {
                pane.update(cx, |_, cx| {
                    cx.emit(pane::Event::ChangeItemTitle);
                    cx.notify();
                });
            }

            if T::is_edit_event(event) {
                if let Autosave::AfterDelay { milliseconds } = cx.global::<Settings>().autosave {
                    let prev_autosave = pending_autosave
                        .take()
                        .unwrap_or_else(|| Task::ready(Some(())));
                    let (cancel_tx, mut cancel_rx) = oneshot::channel::<()>();
                    let prev_cancel_tx = mem::replace(&mut cancel_pending_autosave, cancel_tx);
                    let project = workspace.project.downgrade();
                    let _ = prev_cancel_tx.send(());
                    pending_autosave = Some(cx.spawn_weak(|_, mut cx| async move {
                        let mut timer = cx
                            .background()
                            .timer(Duration::from_millis(milliseconds))
                            .fuse();
                        prev_autosave.await;
                        futures::select_biased! {
                            _ = cancel_rx => return None,
                            _ = timer => {}
                        }

                        let project = project.upgrade(&cx)?;
                        cx.update(|cx| Pane::autosave_item(&item, project, cx))
                            .await
                            .log_err();
                        None
                    }));
                }
            }
        })
        .detach();

        cx.observe_focus(self, move |workspace, item, focused, cx| {
            if !focused && cx.global::<Settings>().autosave == Autosave::OnFocusChange {
                Pane::autosave_item(&item, workspace.project.clone(), cx).detach_and_log_err(cx);
            }
        })
        .detach();
    }

    fn deactivated(&self, cx: &mut MutableAppContext) {
        self.update(cx, |this, cx| this.deactivated(cx));
    }

    fn navigate(&self, data: Box<dyn Any>, cx: &mut MutableAppContext) -> bool {
        self.update(cx, |this, cx| this.navigate(data, cx))
    }

    fn id(&self) -> usize {
        self.id()
    }

    fn to_any(&self) -> AnyViewHandle {
        self.into()
    }

    fn is_dirty(&self, cx: &AppContext) -> bool {
        self.read(cx).is_dirty(cx)
    }

    fn has_conflict(&self, cx: &AppContext) -> bool {
        self.read(cx).has_conflict(cx)
    }

    fn can_save(&self, cx: &AppContext) -> bool {
        self.read(cx).can_save(cx)
    }

    fn save(&self, project: ModelHandle<Project>, cx: &mut MutableAppContext) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.save(project, cx))
    }

    fn save_as(
        &self,
        project: ModelHandle<Project>,
        abs_path: PathBuf,
        cx: &mut MutableAppContext,
    ) -> Task<anyhow::Result<()>> {
        self.update(cx, |item, cx| item.save_as(project, abs_path, cx))
    }

    fn reload(
        &self,
        project: ModelHandle<Project>,
        cx: &mut MutableAppContext,
    ) -> Task<Result<()>> {
        self.update(cx, |item, cx| item.reload(project, cx))
    }

    fn act_as_type(&self, type_id: TypeId, cx: &AppContext) -> Option<AnyViewHandle> {
        self.read(cx).act_as_type(type_id, self, cx)
    }

    fn to_followable_item_handle(&self, cx: &AppContext) -> Option<Box<dyn FollowableItemHandle>> {
        if cx.has_global::<FollowableItemBuilders>() {
            let builders = cx.global::<FollowableItemBuilders>();
            let item = self.to_any();
            Some(builders.get(&item.view_type())?.1(item))
        } else {
            None
        }
    }

    fn on_release(
        &self,
        cx: &mut MutableAppContext,
        callback: Box<dyn FnOnce(&mut MutableAppContext)>,
    ) -> gpui::Subscription {
        cx.observe_release(self, move |_, cx| callback(cx))
    }
}

impl From<Box<dyn ItemHandle>> for AnyViewHandle {
    fn from(val: Box<dyn ItemHandle>) -> Self {
        val.to_any()
    }
}

impl From<&Box<dyn ItemHandle>> for AnyViewHandle {
    fn from(val: &Box<dyn ItemHandle>) -> Self {
        val.to_any()
    }
}

impl Clone for Box<dyn ItemHandle> {
    fn clone(&self) -> Box<dyn ItemHandle> {
        self.boxed_clone()
    }
}

impl<T: Item> WeakItemHandle for WeakViewHandle<T> {
    fn id(&self) -> usize {
        self.id()
    }

    fn upgrade(&self, cx: &AppContext) -> Option<Box<dyn ItemHandle>> {
        self.upgrade(cx).map(|v| Box::new(v) as Box<dyn ItemHandle>)
    }
}

///The 'search interface' that items can implement to allow a standardized search
///bar to be deployed on cmd-f (or whatever the item's equivalent binding is)
pub mod search {
    use super::Item;

    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Direction {
        Prev,
        Next,
    }

    pub trait Searchable: Item {
        fn next_match();
    }
}
