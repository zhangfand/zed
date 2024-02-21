use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use deno_core::{
    error::AnyError, FastString, JsRuntime, ModuleLoadResponse, ModuleSource, ModuleSourceCode,
    ModuleSpecifier, ModuleType, OpState, PollEventLoopOptions,
};
use futures::{
    channel::{mpsc, oneshot},
    Future, StreamExt,
};
use gpui::{AppContext, Global};
use serde::de::DeserializeOwned;
use serde_v8::Serializable;
use std::{path::PathBuf, vec::Vec};
use std::{rc::Rc, sync::Arc};
use util::maybe;

#[cfg(test)]
mod scripting_test;

#[derive(Clone)]
struct Extension {
    name: &'static str,
    import_specifier: &'static str,
    source: &'static str,
    op_state: Option<Arc<dyn Any>>,
    ops: &'static [deno_core::OpDecl],
}

unsafe impl Send for Extension {}

pub fn register_extension(
    name: &'static str,
    import_specifier: &'static str,
    source: &'static str,
    ops: &'static [deno_core::OpDecl],
    cx: &mut AppContext,
) {
    let list = cx.default_global::<GlobalExtensionList>();
    list.0.push(Extension {
        name,
        import_specifier,
        source,
        ops,
    });
}

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

#[derive(Default)]
struct GlobalExtensionList(Vec<Extension>);

impl Global for GlobalExtensionList {}

pub struct Module {
    operations: mpsc::UnboundedSender<Operation>,
    id: ModuleId,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct ModuleId(usize);

enum Operation {
    CompileModule {
        path: PathBuf,
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

    pub fn new(cx: &mut AppContext) -> Self {
        let (ops_tx, ops_rx) = mpsc::unbounded();

        let extensions = cx.default_global::<GlobalExtensionList>().0.clone();

        cx.background_executor()
            .spawn_local(move || Self::run(extensions, ops_rx));

        Self { operations: ops_tx }
    }

    pub fn compile_module(
        &self,
        path: PathBuf,
        source: String,
    ) -> impl Future<Output = Result<Module>> {
        let (tx, rx) = oneshot::channel();
        let operations = self.operations.clone();
        self.operations
            .unbounded_send(Operation::CompileModule {
                path,
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

    async fn run(extensions: Vec<Extension>, mut rx: mpsc::UnboundedReceiver<Operation>) {
        let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
            extensions: extensions
                .iter()
                .map(|extension| deno_core::Extension {
                    name: extension.name,
                    ops: extension.ops.into(),
                    op_state_fn: extension.op_state.map(|state| Box::new(|op_state| {
                        *op_state = state;
                    })
                    ..Default::default()
                })
                .collect(),
            module_loader: Some(Rc::new(ModuleLoader {
                extension_sources_by_import_specifier: extensions
                    .iter()
                    .map(|extension| (extension.import_specifier.to_string(), extension.source))
                    .collect(),
            })),
            ..Default::default()
        });

        let mut modules = HashMap::default();

        while let Some(operation) = rx.next().await {
            match operation {
                Operation::CompileModule {
                    path,
                    source,
                    callback,
                } => {
                    let result = (|| async {
                        let module_id = runtime
                            .load_main_module(
                                &ModuleSpecifier::from_file_path(&path).unwrap(),
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

struct ModuleLoader {
    extension_sources_by_import_specifier: HashMap<String, &'static str>,
}

const EXTENSION_SCHEME: &'static str = "scripting-extension";

impl deno_core::ModuleLoader for ModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        _referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<ModuleSpecifier> {
        Ok(
            if self
                .extension_sources_by_import_specifier
                .contains_key(specifier)
            {
                ModuleSpecifier::parse(&format!("{EXTENSION_SCHEME}:///{specifier}"))?
            } else {
                ModuleSpecifier::parse(specifier)?
            },
        )
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        _requested_module_type: deno_core::RequestedModuleType,
    ) -> ModuleLoadResponse {
        ModuleLoadResponse::Sync(if module_specifier.scheme() == EXTENSION_SCHEME {
            let path = module_specifier.path();
            if let Some(source) = self
                .extension_sources_by_import_specifier
                .get(path)
                .copied()
            {
                Ok(ModuleSource::new(
                    ModuleType::JavaScript,
                    ModuleSourceCode::String(FastString::from_static(source)),
                    module_specifier,
                ))
            } else {
                Err(anyhow!("unknown extension '{path}'"))
            }
        } else {
            Err(anyhow!("failed to load module"))
        })
    }
}
