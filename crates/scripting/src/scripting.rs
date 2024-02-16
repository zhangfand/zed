use gpui::AppContext;

// https://github.com/matejkoncal/v8-experiments/blob/master/src/main.rs
// pub fn
//
pub struct ScriptModule {
    exports: v8::Global<v8::Object>,
}

// struct GlobalV8Isolate(v8::Isolate);

pub fn init(cx: &mut AppContext) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

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

    let code = v8::String::new(scope, "const x = 'foo '; x + testFunction('bar') ").unwrap();
    let script = v8::Script::compile(scope, code, None).unwrap();
    let result = script.run(scope).unwrap().to_rust_string_lossy(scope);
    dbg!(&result);

    // v8::Global::new(isolate, script);

    // scope.
    // cx.set_global(Arc::new())
}
