use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use serde_v8::Serializable;
use std::sync::Once;

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
    args: Vec<Box<dyn Serializable>>,
) -> Result<T> {
    let mut isolate = Isolate::new();
    let mut module = isolate.compile_module("the-script.js", source)?;
    module.call_export(export_name, args, &mut isolate)
}

pub struct Isolate(v8::OwnedIsolate);

pub struct Module(v8::Global<v8::Object>);

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
        args: Vec<Box<dyn Serializable>>,
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
