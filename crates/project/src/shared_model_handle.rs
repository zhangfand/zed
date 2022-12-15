use std::marker::PhantomData;

use crate::Project;
use anyhow::Context;

use client::PeerId;
use gpui::{
    AnyModelHandle, AsyncAppContext, Entity, ModelContext, ModelHandle, Task, WeakModelHandle,
};
use postage::{oneshot::Sender, sink::Sink, stream::Stream};
use util::ResultExt;

type ModelId = usize;
type RemoteId = u64;

#[derive(Default)]
pub struct RemoteModelHandleManager {
    remote_models: Vec<AnyModelHandle>,
}

impl RemoteModelHandleManager {}

impl Project {
    fn replicate<E: SharedModel>(
        &mut self,
        model: ModelId,
        peer_id: PeerId,
        cx: &mut ModelContext<Self>,
    ) -> anyhow::Result<()> {
        // Get create message from model

        // Assemble and send create message to peer

        //

        Ok(())
    }
}

// Creation of shared models works on a pull basis. Peers send a 'give me this model pls!' message
// And then the project assembles it's local representation

pub trait SharedModel: Entity {}

pub struct RemoteModelHandle<M: SharedModel> {
    remote_id: RemoteId,
    model_id: ModelId,
    remove_channel: Sender<()>,
    _pd: PhantomData<M>,
}

impl<M: SharedModel> RemoteModelHandle<M> {
    pub fn downgrade(&self) -> WeakRemoteModelHandle<M> {
        WeakRemoteModelHandle {
            remote_id: self.remote_id,
            model_id: self.model_id,
            _pd: PhantomData,
        }
    }
}

pub struct WeakRemoteModelHandle<M: SharedModel> {
    remote_id: RemoteId,
    model_id: ModelId,
    _pd: PhantomData<M>,
}

impl<M: SharedModel> WeakRemoteModelHandle<M> {
    pub fn upgrade(
        &self,
        project: &mut Project,
        cx: &mut ModelContext<Project>,
    ) -> Task<anyhow::Result<RemoteModelHandle<M>>> {
        unimplemented!()
    }
}

impl<M: SharedModel> Drop for RemoteModelHandle<M> {
    fn drop(&mut self) {
        self.remove_channel
            .blocking_send(())
            .context("RemoteModelHandle failed to send it's drop message")
            .log_err();
    }
}

pub trait SharedModelHandleExtension<M: SharedModel> {
    fn clone_remote(
        &self,
        project: &mut Project,
        cx: &mut ModelContext<Project>,
    ) -> RemoteModelHandle<M>;
}

impl<M: SharedModel> SharedModelHandleExtension<M> for ModelHandle<M> {
    fn clone_remote(
        &self,
        project: &mut Project,
        cx: &mut ModelContext<Project>,
    ) -> RemoteModelHandle<M> {
        let model_id = self.id();

        project
            .remote_model_manager
            .remote_models
            .push(self.clone().into());

        let remote_id = project
            .remote_id()
            .context("Project didn't have a remote id")
            .unwrap();

        let (sender, mut receiver) = postage::oneshot::channel();

        cx.spawn_weak(|project, mut cx| async move {
            if let Some(()) = receiver.recv().await {
                if let Some(project_handle) = project.upgrade(&mut cx) {
                    project_handle.update(&mut cx, |project, _cx| {
                        project
                            .remote_model_manager
                            .remote_models
                            .retain(|model_handle| model_handle.id() != model_id)
                    })
                }
            }
        })
        .detach();

        RemoteModelHandle {
            remote_id,
            model_id,
            remove_channel: sender,
            _pd: PhantomData,
        }
    }
}
