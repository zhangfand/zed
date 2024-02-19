use anyhow::{anyhow, Context, Result};
use collections::HashMap;
use futures::{channel::oneshot, Future};
use serde::{de::DeserializeOwned, Deserialize};
use serde_v8::Serializable;
use std::sync::{mpsc, Arc, Once};
use util::post_inc;

#[cfg(test)]
mod scripting_test;

pub fn init() {
    static INIT_V8: Once = Once::new();
    INIT_V8.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
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

pub struct Engine {
    operations: mpsc::Sender<Operation>,
    _worker_thread: std::thread::JoinHandle<()>,
}

pub struct ModuleHandle {
    operations: mpsc::Sender<Operation>,
    id: ModuleId,
}

pub struct Isolate(v8::OwnedIsolate);

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
    pub fn new() -> Self {
        let (ops_tx, ops_rx) = mpsc::channel();

        let thread = std::thread::spawn(move || {
            let mut isolate = Isolate::new();
            let mut next_module_id = 0;
            let mut modules = HashMap::default();
            while let Ok(operation) = ops_rx.recv() {
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
        });

        Self {
            operations: ops_tx,
            _worker_thread: thread,
        }
    }

    pub fn compile_module(
        &self,
        name: String,
        source: String,
    ) -> impl Future<Output = Result<ModuleHandle>> {
        let (tx, rx) = oneshot::channel();
        let operations = self.operations.clone();
        self.operations
            .send(Operation::CompileModule {
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
            .send(Operation::Invoke {
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
            .send(Operation::DropModule { module_id: self.id })
            .ok();
    }
}

impl Isolate {
    pub fn new() -> Self {
        Self(v8::Isolate::new(Default::default()))
    }

    pub fn compile_module(&mut self, resource_name: &str, src: &str) -> Result<Module> {
        let module = {
            let scope = &mut v8::HandleScope::new(&mut self.0);
            let context = v8::Context::new(scope);
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
            let result = module.instantiate_module(scope, resolve_callback);
            if result.is_none() {
                Err(anyhow!("failed to instantiate module"))?;
            }
            module
                .evaluate(scope)
                .ok_or_else(|| anyhow!("failed to evaluate module"))?;

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
        let scope = &mut v8::HandleScope::new(&mut isolate.0);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);
        let module = self.0.open(scope);

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

        serde_v8::from_v8(scope, result)
            .with_context(|| format!("failed to deserialize return value"))
    }
}

fn resolve_callback<'a>(
    _context: v8::Local<'a, v8::Context>,
    _specifier: v8::Local<'a, v8::String>,
    _import_assertions: v8::Local<'a, v8::FixedArray>,
    _referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    None
}
