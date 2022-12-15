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
    fn to_remote(&self, project: &Project) -> WeakRemoteModelHandle<M>;
}

impl<M: SharedModel> SharedModelHandleExtension<M> for ModelHandle<M> {
    fn to_remote(&self, project: &Project) -> WeakRemoteModelHandle<M> {
        WeakRemoteModelHandle {
            remote_id: project.remote_id().unwrap(),
            model_id: self.id(),
            _pd: PhantomData,
        }
    }
}
