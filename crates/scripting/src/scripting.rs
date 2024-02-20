use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::{
    channel::{mpsc, oneshot},
    Future, StreamExt,
};
use gpui::{AppContext, Global};
use serde::{de::DeserializeOwned, Deserialize};
use serde_v8::Serializable;
use std::sync::{Arc, Once};
use util::post_inc;

#[cfg(test)]
mod scripting_test;

pub fn init(cx: &mut AppContext) {
    static INIT_V8: Once = Once::new();
    INIT_V8.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });

    let engine = Engine::new(cx);
    cx.set_global(GlobalEngine(engine));
}

pub fn run_script<'de, T: Deserialize<'de>>(
    source: &str,
    export_name: &str,
    args: Vec<Box<dyn Send + Serializable>>,
) -> Result<T> {
    let mut isolate = Isolate::new();
    let mut module = isolate.compile_module("the-script.js", source)?;
    module.call_export(export_name, args, &mut isolate)
}

#[derive(Clone)]
pub struct Engine {
    operations: mpsc::UnboundedSender<Operation>,
}

struct GlobalEngine(Engine);

impl Global for GlobalEngine {}

pub struct ModuleHandle {
    operations: mpsc::UnboundedSender<Operation>,
    id: ModuleId,
}

pub struct Isolate {
    isolate: v8::OwnedIsolate,
    context: v8::Global<v8::Context>,
}

pub struct Module(v8::Global<v8::Object>);

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
        callback: Box<dyn 'static + Send + FnOnce(Option<&mut Module>, &mut Isolate)>,
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
    ) -> impl Future<Output = Result<ModuleHandle>> {
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
            Ok(ModuleHandle {
                id: rx.await??,
                operations,
            })
        }
    }

    async fn run(mut rx: mpsc::UnboundedReceiver<Operation>) {
        let mut isolate = Isolate::new();
        let mut next_module_id = 0;
        let mut modules = HashMap::default();

        isolate.populate_native_module();

        while let Some(operation) = rx.next().await {
            match operation {
                Operation::CompileModule {
                    name,
                    source,
                    callback,
                } => {
                    let module_id = ModuleId(post_inc(&mut next_module_id));
                    let module = isolate.compile_module(&name, &source);
                    match module {
                        Ok(module) => {
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
                    callback,
                } => {
                    let module = modules.get_mut(&module_id);
                    callback(module, &mut isolate);
                }
            }
        }
    }
}

impl ModuleHandle {
    pub fn call_export<T>(
        &self,
        function_name: Arc<str>,
        args: Vec<Box<dyn Send + Serializable>>,
    ) -> impl Future<Output = Result<T>>
    where
        T: 'static + Send + DeserializeOwned,
    {
        let (tx, rx) = oneshot::channel();
        self.operations
            .unbounded_send(Operation::Invoke {
                module_id: self.id,
                callback: Box::new(move |module, isolate| {
                    if let Some(module) = module {
                        tx.send(module.call_export(function_name.as_ref(), args, isolate))
                            .ok();
                    } else {
                        tx.send(Err(anyhow!("module was dropped"))).ok();
                    }
                }),
            })
            .ok();
        async move { rx.await? }
    }
}

impl Drop for ModuleHandle {
    fn drop(&mut self) {
        self.operations
            .unbounded_send(Operation::DropModule { module_id: self.id })
            .ok();
    }
}

struct ZedModule(v8::Global<v8::Module>);

impl Isolate {
    pub fn new() -> Self {
        let mut isolate = v8::Isolate::new(Default::default());
        let context = {
            let scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);
            v8::Global::new(scope, context)
        };
        Self { isolate, context }
    }

    fn populate_native_module(&mut self) {
        let scope = &mut v8::HandleScope::new(&mut self.isolate);
        let context = v8::Local::new(scope, &self.context);
        let scope = &mut v8::ContextScope::new(scope, context);

        let key = v8::String::new(scope, "latestNpmPackageVersion").unwrap();

        let module_name = v8::String::new(scope, "zed").unwrap();
        let module = v8::Module::create_synthetic_module(
            scope,
            module_name,
            &[key],
            Self::synthetic_module_evaluation_steps,
        );
        let module = v8::Global::new(scope, module);
        context.set_slot(scope, ZedModule(module));
        println!("Set zed module in slot");
    }

    fn synthetic_module_evaluation_steps<'a>(
        context: v8::Local<'a, v8::Context>,
        module: v8::Local<v8::Module>,
    ) -> Option<v8::Local<'a, v8::Value>> {
        // SAFETY: `CallbackScope` can be safely constructed from `Local<Context>`
        let scope = &mut unsafe { v8::CallbackScope::new(context) };
        let scope = &mut v8::TryCatch::new(scope);
        //   let module_map = JsRealm::module_map_from(try_catch_scope);

        //   let handle = v8::Global::<v8::Module>::new(try_catch_scope, module);
        //   let exports = module_map
        //     .data
        //     .borrow_mut()
        //     .synthetic_module_exports_store
        //     .remove(&handle)
        //     .unwrap();

        let key = v8::String::new(scope, "latestNpmPackageVersion").unwrap();
        let value = v8::Function::new(
            scope,
            |scope: &mut v8::HandleScope,
             args: v8::FunctionCallbackArguments,
             mut ret: v8::ReturnValue| {
                ret.set(v8::String::new(scope, "the-version").unwrap().into())
            },
        )
        .unwrap();
        module
            .set_synthetic_module_export(scope, key, value.into())
            .unwrap();
        //   for (export_name, export_value) in exports {
        //     let name = v8::Local::new(try_catch_scope, export_name);
        //     let value = v8::Local::new(try_catch_scope, export_value);

        //     // This should never fail
        //     assert!(module
        //       .set_synthetic_module_export(try_catch_scope, name, value)
        //       .unwrap());
        //     assert!(!try_catch_scope.has_caught());
        //   }

        //   // Since Top-Level Await is active we need to return a promise.
        //   // This promise is resolved immediately.
        // let resolver = v8::PromiseResolver::new(try_catch_scope).unwrap();
        // let undefined = v8::undefined(try_catch_scope);
        // resolver.resolve(try_catch_scope, undefined.into());
        Some(v8::undefined(scope).into())
    }

    pub fn compile_module(&mut self, resource_name: &str, src: &str) -> Result<Module> {
        let module = {
            let scope = &mut v8::HandleScope::new(&mut self.isolate);
            let context = v8::Local::new(scope, &self.context);
            let scope = &mut v8::ContextScope::new(scope, context);

            let code = v8::String::new(scope, src)
                .ok_or_else(|| anyhow!("failed to initialize V8 string for script"))?;

            let resource_name = v8::String::new(scope, resource_name).unwrap();
            let source_map_url = v8::String::new(scope, "source_map_url").unwrap();
            let script_origin = v8::ScriptOrigin::new(
                scope,
                resource_name.into(),
                0,
                0,
                true,
                123,
                source_map_url.into(),
                true,
                false,
                true,
            );

            let source = v8::script_compiler::Source::new(code, Some(&script_origin));
            let module = v8::script_compiler::compile_module(scope, source)
                .ok_or_else(|| anyhow!("failed to compile script"))?;

            let scope = &mut v8::TryCatch::new(scope);
            let instantiate_result = module.instantiate_module(scope, resolve_callback);
            if let Some(exception) = scope.exception() {
                Err(anyhow!(
                    "JS Exception: {}",
                    exception.to_rust_string_lossy(scope)
                ))?;
            }
            let evaluate_result = module.evaluate(scope);
            if let Some(exception) = scope.exception() {
                Err(anyhow!(
                    "JS Exception: {}",
                    exception.to_rust_string_lossy(scope)
                ))?;
            }

            if instantiate_result.is_none() {
                Err(anyhow!("failed to instantiate module"))?;
            }
            evaluate_result.ok_or_else(|| anyhow!("failed to evaluate module"))?;

            let namespace = module
                .get_module_namespace()
                .to_object(scope)
                .ok_or_else(|| anyhow!("module did not export an object"))?;

            v8::Global::new(scope, namespace)
        };
        Ok(Module(module))
    }
}

impl Module {
    pub fn call_export<'de, T: Deserialize<'de>>(
        &mut self,
        export_name: &str,
        args: Vec<Box<dyn Send + Serializable>>,
        isolate: &mut Isolate,
    ) -> Result<T> {
        let scope = &mut v8::HandleScope::new(&mut isolate.isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);
        let module = self.0.open(scope);

        // v8::Function::new

        let export_name_string = v8::String::new(scope, export_name)
            .ok_or_else(|| anyhow!("failed to initialize V8 string for entrypoint"))?;
        let export_function: v8::Local<v8::Function> = module
            .get(scope, export_name_string.into())
            .ok_or_else(|| anyhow!("export '{export_name}' not found"))?
            .try_into()
            .with_context(|| format!("export '{export_name}' is not a function"))?;

        let v8_args = args
            .into_iter()
            .map(|mut arg| Ok(arg.to_v8(scope)?))
            .collect::<Result<Vec<_>>>()?;

        let receiver = v8::null(scope);
        let result = export_function
            .call(scope, receiver.into(), &v8_args)
            .ok_or_else(|| anyhow!("failed to call '{export_name}"))?;

        // https://github.com/denoland/roll-your-own-javascript-runtime/blob/main/src/main.rs

        // result

        serde_v8::from_v8(scope, result)
            .with_context(|| format!("failed to deserialize return value"))
    }
}

fn resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    _import_assertions: v8::Local<'a, v8::FixedArray>,
    _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let scope = &mut unsafe { v8::CallbackScope::new(context) };

    let zed_specifier = v8::String::new(scope, "zed/language-server").unwrap();
    if specifier == zed_specifier {
        let zed_module = context.get_slot::<ZedModule>(scope).unwrap();
        let zed_module = zed_module.0.clone();
        let module = v8::Local::new(scope, &zed_module);
        return Some(module);
    }

    None
}
