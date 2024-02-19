use crate::{init, Engine, Isolate};
use gpui::TestAppContext;

#[test]
fn test_scripting() {
    init();

    let mut isolate = Isolate::new();
    let mut module1 = isolate
        .compile_module(
            "test.js",
            "
            export function joinUppercase(objects) {
                return objects.map(x => x.toUpperCase()).join(',')
            }

            export function repeat(string, count) {
                return new Array(count + 1).join(string)
            }
            ",
        )
        .unwrap();
    let mut module2 = isolate
        .compile_module(
            "test2.js",
            "
            export function return5() {
                return 5
            }
            ",
        )
        .unwrap();

    assert_eq!(
        module1
            .call_export::<String>(
                "joinUppercase",
                vec![Box::new(["one", "two", "three"])],
                &mut isolate,
            )
            .unwrap(),
        "ONE,TWO,THREE"
    );

    assert_eq!(
        module1
            .call_export::<String>("joinUppercase", vec![Box::new(["one"])], &mut isolate)
            .unwrap(),
        "ONE"
    );

    assert_eq!(
        module1
            .call_export::<String>("repeat", vec![Box::new("hello"), Box::new(3)], &mut isolate)
            .unwrap(),
        "hellohellohello"
    );

    assert_eq!(
        module2
            .call_export::<usize>("return5", vec![], &mut isolate)
            .unwrap(),
        5
    );
}

#[gpui::test]
async fn test_scripting_async(cx: &mut TestAppContext) {
    init();
    cx.executor().allow_parking();

    let engine = Engine::new();

    let module1 = engine
        .compile_module(
            "test.js".into(),
            "
            export function joinUppercase(objects) {
                return objects.map(x => x.toUpperCase()).join(',')
            }

            export function repeat(string, count) {
                return new Array(count + 1).join(string)
            }
            "
            .into(),
        )
        .await
        .unwrap();

    let module2 = engine
        .compile_module(
            "test2.js".into(),
            "
            export function return5() {
                return 5
            }
            "
            .into(),
        )
        .await
        .unwrap();

    assert_eq!(
        module1
            .call_export::<String>(
                "joinUppercase".into(),
                vec![Box::new(["one", "two", "three"])],
            )
            .await
            .unwrap(),
        "ONE,TWO,THREE"
    );
}
