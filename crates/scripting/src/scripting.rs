use anyhow::{anyhow, Context, Result};
use gpui::AppContext;
use serde::Deserialize;
use serde_v8::Serializable;

pub fn init(_cx: &mut AppContext) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}

pub fn run_script<'de, T: Deserialize<'de>>(
    script: &str,
    entrypoint: &str,
    args: Vec<Box<dyn Serializable>>,
) -> Result<T> {
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, script)
        .ok_or_else(|| anyhow!("failed to initialize V8 string for script"))?;
    let script = v8::Script::compile(scope, code, None)
        .ok_or_else(|| anyhow!("failed to compile script"))?;
    script
        .run(scope)
        .ok_or_else(|| anyhow!("failed to run script"))?;

    let entrypoint_name = entrypoint;
    let v8_entrypoint_name = v8::String::new(scope, entrypoint)
        .ok_or_else(|| anyhow!("failed to initialize V8 string for entrypoint"))?;

    let global = context.global(scope);
    let entrypoint = global
        .get(scope, v8_entrypoint_name.into())
        .ok_or_else(|| anyhow!("entrypoint function '{entrypoint_name}' not found"))?;
    let entrypoint = v8::Local::<v8::Function>::try_from(entrypoint)
        .with_context(|| format!("entrypoint function '{entrypoint_name}' is not a function"))?;

    let v8_args = args
        .into_iter()
        .map(|mut arg| Ok(arg.to_v8(scope)?))
        .collect::<Result<Vec<_>>>()?;

    let result = entrypoint
        .call(scope, global.into(), &v8_args)
        .ok_or_else(|| anyhow!("failed to call entrypoint"))?;

    let return_value: T = serde_v8::from_v8(scope, result)
        .with_context(|| format!("failed to deserialize return value"))?;

    Ok(return_value)
}
