mod headless_project;

use anyhow::Result;
use collections::HashMap;
use futures::{channel::mpsc::UnboundedSender, future::LocalBoxFuture, Future, FutureExt as _};
use gpui::{AppContext, AsyncAppContext, Context, Model};
use headless_project::HeadlessProject;
use rpc::proto::{
    AnyTypedEnvelope, Envelope, EnvelopedMessage as _, Error, RequestMessage, TypedEnvelope,
};
use settings::{Settings, SettingsStore};
use std::{any::TypeId, sync::Once};
use worktree::WorktreeSettings;

#[derive(Clone)]
pub struct Server {
    sender: UnboundedSender<Envelope>,
    handlers: &'static Handlers,
    project: Model<HeadlessProject>,
}

#[derive(Default)]
struct Handlers(HashMap<TypeId, MessageHandler>);

type MessageHandler = Box<
    dyn Send
        + Sync
        + Fn(Server, Box<dyn AnyTypedEnvelope>, AsyncAppContext) -> LocalBoxFuture<'static, ()>,
>;

impl Server {
    pub fn init(cx: &mut AppContext) {
        cx.set_global(SettingsStore::default());
        WorktreeSettings::register(cx);
    }

    pub fn new(outgoing_tx: UnboundedSender<Envelope>, cx: &mut AppContext) -> Self {
        static mut HANDLERS: Option<Handlers> = None;
        static INIT_HANDLERS: Once = Once::new();

        let handlers = unsafe {
            INIT_HANDLERS.call_once(|| {
                let mut handlers = Handlers::default();
                HeadlessProject::init(&mut handlers);
                HANDLERS = Some(handlers);
            });
            HANDLERS.as_ref().unwrap()
        };

        Self {
            handlers,
            sender: outgoing_tx.clone(),
            project: cx.new_model(|_| HeadlessProject::new(outgoing_tx)),
        }
    }

    pub async fn handle_message(
        &mut self,
        message: Box<dyn AnyTypedEnvelope>,
        cx: AsyncAppContext,
    ) {
        if let Some(handler) = self.handlers.0.get(&message.payload_type_id()) {
            let type_name = message.payload_type_name();
            eprintln!("received {type_name}");
            handler(self.clone(), message, cx).await;
            eprintln!("responded {type_name}");
        } else {
            self.sender
                .unbounded_send(
                    Error {
                        message: format!("unhandled request type {:?}", message.payload_type_id()),
                        code: 0,
                        tags: Default::default(),
                    }
                    .into_envelope(0, Some(message.message_id()), None),
                )
                .ok();
        }
    }
}

impl Handlers {
    fn add<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(Model<HeadlessProject>, M, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = Result<M::Response>>,
        M: RequestMessage,
    {
        self.0.insert(
            TypeId::of::<M>(),
            Box::new(move |server, envelope, cx| {
                let envelope = *envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                let tx = server.sender.clone();
                let result = handler(server.project.clone(), envelope.payload, cx);
                async move {
                    let msg = match result.await {
                        Ok(response) => response.into_envelope(0, Some(envelope.message_id), None),
                        Err(error) => Error {
                            code: 0,
                            tags: Vec::new(),
                            message: error.to_string(),
                        }
                        .into_envelope(0, Some(envelope.message_id), None),
                    };
                    tx.unbounded_send(msg).ok();
                }
                .boxed_local()
            }),
        );
        self
    }
}
