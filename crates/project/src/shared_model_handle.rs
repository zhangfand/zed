use std::marker::PhantomData;

use crate::Project;
use anyhow::Context;

use collections::HashMap;
use gpui::{
    AnyModelHandle, AppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task,
};
use postage::{oneshot::Sender, sink::Sink, stream::Stream};
use util::ResultExt;

// Goals:
// - Have a generic, extensible mechanism for describing model synchronization
// - Have it be able to subsume all possible and existing behaviors
// - Have it be type safe

// TODO:
// [x] Sketch a kind of pointer which is serializable, can be sent over the network, and can be upgraded over the network
// [ ] Sketch a way for the replicated models to communicate with each other back and forth, nicely with the SharedModel type
// [ ] Work on the protos for all of this
// [ ] Get it running and testing

type ModelId = usize;
type RemoteId = u64;

#[derive(Default)]
pub struct RemoteModelHandleManager {
    remote_models: Vec<Box<dyn SharedModelHandle>>,
}

impl RemoteModelHandleManager {}

impl Project {
    // TODO
}

pub trait SharedModel: Entity {
    fn type_key() -> &'static str;
    fn create_message(&self) -> CreateMessage;
    fn create_from_create_message(msg: CreateMessage, cx: &mut ModelContext<Self>) -> Self;
}

pub trait SharedModelHandle {
    fn type_key(&self) -> &'static str;
    fn boxed_clone(&self) -> Box<dyn SharedModelHandle>;
    fn create_message(&self, cx: &AppContext) -> CreateMessage;
}

impl<M: SharedModel> SharedModelHandle for ModelHandle<M> {
    fn type_key(&self) -> &'static str {
        M::type_key()
    }

    fn boxed_clone(&self) -> Box<dyn SharedModelHandle> {
        Box::new(self.clone())
    }

    fn create_message(&self, cx: &AppContext) -> CreateMessage {
        self.read(cx).create_message()
    }
}

pub struct RemoteModelHandle<M: SharedModel> {
    remote_id: RemoteId,
    model: ModelHandle<M>,
    remove_channel: Sender<()>,
}

impl<M: SharedModel> RemoteModelHandle<M> {
    pub fn downgrade(&self) -> WeakRemoteModelHandle<M> {
        WeakRemoteModelHandle {
            remote_id: self.remote_id,
            model_id: self.model.id(),
            _pd: PhantomData,
        }
    }

    pub fn read_with<F, R>(&self, cx: &AppContext, f: F) -> R
    where
        F: FnOnce(&M, &AppContext) -> R,
    {
        let m = self.model.read(cx);
        f(m, cx)
    }
}

pub struct WeakRemoteModelHandle<M: SharedModel> {
    remote_id: RemoteId,
    model_id: ModelId,
    _pd: PhantomData<*const M>,
}

impl<M: SharedModel> Clone for WeakRemoteModelHandle<M> {
    fn clone(&self) -> Self {
        WeakRemoteModelHandle {
            remote_id: self.remote_id,
            model_id: self.model_id,
            _pd: PhantomData,
        }
    }
}

type TypeKey = String;

impl<M: SharedModel> WeakRemoteModelHandle<M> {
    pub fn upgrade(
        &self,
        project: &mut Project,
        cx: &mut ModelContext<Project>,
    ) -> Task<anyhow::Result<RemoteModelHandle<M>>> {
        ///////////////////
        // On remote end://
        ///////////////////
        if let Some(handle) = local_model_with_this_id(self.clone(), project, cx) {
            return Task::ready(Ok(handle));
        }

        let self_clone = self.clone();
        cx.spawn(|project, mut cx| async move {
            let create_message = send_model_request_to_peer(
                self_clone.clone(),
                ///////////////////
                // On hosts end: //
                ///////////////////
                {
                    let cx = get_new_cx();
                    let project = project.clone();
                    move |model_id, type_key| {
                        let model = get_model_for_id(model_id, type_key, cx);

                        if let Some(model) = model {
                            project.update(cx, |project, _cx| {
                                project
                                    .remote_model_manager
                                    .remote_models
                                    .push(model.boxed_clone());
                            });

                            return Ok(model.create_message(&cx));
                        } else {
                            return None.context("Couldn't find model :(");
                        }
                    }
                },
            )
            .await?;
            ///////////////////
            // On remote end://
            ///////////////////

            let shared_model = cx.add_model(|cx| M::create_from_create_message(create_message, cx));

            let (sender, mut receiver) = postage::oneshot::channel();

            let project_handle = project.downgrade();
            let self_clone2 = self_clone.clone();

            cx.spawn(|mut cx| async move {
                if let Some(()) = receiver.recv().await {
                    if let Some(project_handle) = project_handle.upgrade(&mut cx) {
                        project_handle
                            .update(&mut cx, |_project, _cx| {
                                send_drop_notification_to_peer(self_clone2, |model_id| {
                                    ///////////////////
                                    // On hosts end: //
                                    ///////////////////
                                    remove_one_stored_remote_handle_for(model_id)
                                })
                            })
                            .await
                    }
                }
            })
            .detach();

            Ok(RemoteModelHandle {
                remote_id: self_clone.remote_id,
                model: shared_model,
                remove_channel: sender,
            })
        })
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

pub struct CreateMessage(pub u64);

fn local_model_with_this_id<M: SharedModel>(
    _handle: WeakRemoteModelHandle<M>,
    _project: &mut Project,
    _cx: &mut ModelContext<Project>,
) -> Option<RemoteModelHandle<M>> {
    unreachable!()
}

async fn send_model_request_to_peer<F, E: SharedModel>(
    _address: WeakRemoteModelHandle<E>,
    _f: F,
) -> anyhow::Result<CreateMessage>
where
    F: FnOnce(ModelId, String) -> anyhow::Result<CreateMessage>,
{
    unimplemented!()
}

async fn send_drop_notification_to_peer<F, E: SharedModel>(
    _address: WeakRemoteModelHandle<E>,
    _f: F,
) where
    F: FnOnce(ModelId),
{
    unimplemented!()
}

lazy_static::lazy_static! {
    static ref TYPE_MAP: HashMap<TypeKey, fn(AnyModelHandle) -> Box<dyn SharedModelHandle>> =
        HashMap::default();
}

type TypeMap = HashMap<TypeKey, fn(AnyModelHandle) -> Box<dyn SharedModelHandle>>;

pub fn register_shared_model<E: SharedModel>(cx: &mut MutableAppContext) {
    cx.update_default_global::<TypeMap, _, _>(|type_map, _cx| {
        type_map.insert(E::type_key().to_string(), |any_model| {
            any_model
                .downcast::<E>()
                .map(|model_handle| Box::new(model_handle))
                .unwrap()
        });
    })
}

fn get_model_for_id(
    id: ModelId,
    type_key: String,
    cx: &mut MutableAppContext,
) -> Option<Box<dyn SharedModelHandle>> {
    let model = any_model(id);
    cx.global::<TypeMap>()
        .get(&type_key)
        .map(|cast| (cast)(model))
}

fn any_model(_id: ModelId) -> AnyModelHandle {
    unimplemented!()
}

fn get_new_cx() -> &'static mut MutableAppContext {
    unimplemented!()
}

fn remove_one_stored_remote_handle_for(_id: ModelId) {}
