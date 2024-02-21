use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use deno_core::{error::AnyError, JsRuntime, ModuleSpecifier, PollEventLoopOptions};
use futures::{
    channel::{mpsc, oneshot},
    Future, StreamExt,
};
use gpui::{AppContext, Global};
use serde::de::DeserializeOwned;
use serde_v8::Serializable;
use std::sync::Arc;
use std::vec::Vec;
use util::maybe;

#[cfg(test)]
mod scripting_test;

#[deno_core::op2(async)]
#[string]
pub async fn op_latest_npm_package_version() -> Result<String, AnyError> {
    Ok("the-version".into())
}

deno_core::extension!(
    zed_ops,
    ops = [op_latest_npm_package_version],
    esm_entry_point = "ext:zed_ops/zed.js",
    esm = ["zed.js"],
);

pub fn init(cx: &mut AppContext) {
    let engine = Engine::new(cx);
    cx.set_global(GlobalEngine(engine));
}

#[derive(Clone)]
pub struct Engine {
    operations: mpsc::UnboundedSender<Operation>,
}

struct GlobalEngine(Engine);

impl Global for GlobalEngine {}

pub struct Module {
    operations: mpsc::UnboundedSender<Operation>,
    id: ModuleId,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct ModuleId(usize);

enum Operation {
    CompileModule {
        name: String,
        source: String,
        callback: oneshot::Sender<Result<ModuleId>>,
    },
    DropModule {
        module_id: ModuleId,
    },
    Invoke {
        module_id: ModuleId,
        export_name: Arc<str>,
        args: Vec<Box<dyn Send + Serializable>>,
        callback: Box<dyn 'static + Send + FnOnce(&mut JsRuntime, Result<v8::Global<v8::Value>>)>,
    },
}

impl Engine {
    /// Returns the global [`Engine`].
    pub fn global(cx: &AppContext) -> Self {
        cx.global::<GlobalEngine>().0.clone()
    }

    pub fn new(cx: &AppContext) -> Self {
        let (ops_tx, ops_rx) = mpsc::unbounded();

        cx.background_executor()
            .spawn_local(move || Self::run(ops_rx));

        Self { operations: ops_tx }
    }

    pub fn compile_module(
        &self,
        name: String,
        source: String,
    ) -> impl Future<Output = Result<Module>> {
        let (tx, rx) = oneshot::channel();
        let operations = self.operations.clone();
        self.operations
            .unbounded_send(Operation::CompileModule {
                name,
                source,
                callback: tx,
            })
            .ok();
        async move {
            Ok(Module {
                id: rx.await??,
                operations,
            })
        }
    }

    async fn run(mut rx: mpsc::UnboundedReceiver<Operation>) {
        let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            extensions: vec![zed_ops::init_ops_and_esm()],
            module_loader: None,
            ..Default::default()
        });

        let mut modules = HashMap::default();

        while let Some(operation) = rx.next().await {
            match operation {
                Operation::CompileModule {
                    name,
                    source,
                    callback,
                } => {
                    let result = (|| async {
                        let module_id = runtime
                            .load_side_module(
                                &ModuleSpecifier::from_file_path("/foo").unwrap(),
                                Some(source.into()),
                            )
                            .await?;
                        runtime.mod_evaluate(module_id).await?;
                        let namespace = runtime.get_module_namespace(module_id)?;
                        Ok((ModuleId(module_id), namespace))
                    })()
                    .await;

                    match result {
                        Ok((module_id, module)) => {
                            modules.insert(module_id, module);
                            callback.send(Ok(module_id)).ok();
                        }
                        Err(error) => {
                            callback.send(Err(error)).ok();
                        }
                    }
                }
                Operation::DropModule { module_id } => {
                    modules.remove(&module_id);
                }
                Operation::Invoke {
                    module_id,
                    export_name,
                    args,
                    callback,
                } => {
                    let result = maybe!({
                        let module = modules
                            .get_mut(&module_id)
                            .ok_or_else(|| anyhow!("module was dropped"))?;
                        let mut scope = runtime.handle_scope();
                        let v8_args = args
                            .into_iter()
                            .map(|mut arg| {
                                let arg = arg.to_v8(&mut scope)?;
                                Ok(v8::Global::new(&mut scope, arg))
                            })
                            .collect::<Result<Vec<_>>>()?;
                        let module = module.open(&mut scope);
                        let js_export_name =
                            v8::String::new(&mut scope, export_name.as_ref()).unwrap();
                        let export = module
                            .get(&mut scope, js_export_name.into())
                            .ok_or_else(|| anyhow!("no such export '{export_name}'"))?;
                        let function = v8::Local::<v8::Function>::try_from(export)?;
                        let function = v8::Global::new(&mut scope, function);
                        drop(scope);
                        Ok(runtime.call_with_args(&function, &v8_args))
                    });

                    match result {
                        Err(error) => callback(&mut runtime, Err(error)),
                        Ok(call) => {
                            let event_loop_result = runtime
                                .run_event_loop(PollEventLoopOptions::default())
                                .await;
                            let call_result = call.await;
                            callback(&mut runtime, event_loop_result.and_then(|_| call_result))
                        }
                    }
                }
            }
        }
    }
}

impl Module {
    pub fn call_export<T>(
        &self,
        export_name: Arc<str>,
        args: Vec<Box<dyn Send + Serializable>>,
    ) -> impl Future<Output = Result<T>>
    where
        T: 'static + Send + DeserializeOwned,
    {
        let (tx, rx) = oneshot::channel();
        self.operations
            .unbounded_send(Operation::Invoke {
                module_id: self.id,
                export_name,
                args,
                callback: Box::new(move |runtime, value| match value {
                    Ok(value) => {
                        let mut scope = runtime.handle_scope();
                        let value = v8::Local::new(&mut scope, &value);
                        let value = serde_v8::from_v8(&mut scope, value)
                            .with_context(|| format!("failed to deserialize return value"));
                        tx.send(value).ok();
                    }
                    Err(error) => {
                        tx.send(Err(error)).ok();
                    }
                }),
            })
            .ok();
        async move { rx.await? }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        self.operations
            .unbounded_send(Operation::DropModule { module_id: self.id })
            .ok();
    }
}
