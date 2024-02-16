use anyhow::{anyhow, Context, Result};
use gpui::{AppContext, Global};

// https://github.com/matejkoncal/v8-experiments/blob/master/src/main.rs
// pub fn
//
pub struct ScriptModule {
    exports: v8::Global<v8::Object>,
}

// struct V8Platform(v8::SharedRef<v8::Platform>);

// struct GlobalV8Platform(V8Platform);

// impl Global for GlobalV8Platform {}

// struct GlobalV8Isolate(v8::Isolate);

pub fn init(cx: &mut AppContext) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    // cx.set_global(GlobalV8Platform(V8Platform(platform)));

    //
    // cx.set_global(GlobalV8Isolate(v8::Isolate::new(Default::default())));

    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let first_function = v8::Function::new(
        scope,
        |scope: &mut v8::HandleScope,
         args: v8::FunctionCallbackArguments,
         mut rv: v8::ReturnValue| {
            let arg = args.get(0);
            let arg_string = arg.to_string(scope).unwrap().to_rust_string_lossy(scope);
            println!("passed from JS to rust: {}", arg_string);
            let returned_value_string =
                v8::String::new(scope, "This is returned from rust to javascript")
                    .unwrap()
                    .into();
            rv.set(returned_value_string);
        },
    )
    .unwrap()
    .into();

    let name = v8::String::new(scope, "testFunction").unwrap().into();
    let global = context.global(scope);
    global.set(scope, name, first_function);
    // global.in

    // v8::script_compiler::compile_module

    let code = v8::String::new(scope, "const x = 'foo '; x + testFunction('bar') ").unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap().to_rust_string_lossy(scope);
    dbg!(&result);

    let result = run_script("function foo(a, b) { return 73; }", "foo").unwrap();

    // v8::Global::new(isolate, script);

    // scope.
    // cx.set_global(Arc::new())
}

pub fn run_script(script: &str, entrypoint: &str) -> Result<String> {
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
    let scope = &mut v8::ContextScope::new(scope, context);

    let code = v8::String::new(scope, script)
        .ok_or_else(|| anyhow!("failed to initialize V8 string for script"))?;
    let script = v8::Script::compile(scope, code, None)
        .ok_or_else(|| anyhow!("failed to compile script"))?;
    let result = script
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

    let arg_a = v8::String::new(scope, "foo").unwrap();
    let arg_b = v8::Boolean::new(scope, false);

    let result = entrypoint
        .call(scope, global.into(), &[arg_a.into(), arg_b.into()])
        .ok_or_else(|| anyhow!("failed to call entrypoint"))?;

    Ok(result.to_rust_string_lossy(scope))
}
