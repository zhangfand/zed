use crate::{init, Engine};
use gpui::{AppContext, TestAppContext};

#[gpui::test]
async fn test_scripting_async(cx: &mut TestAppContext) {
    cx.update(init);
    cx.executor().allow_parking();

    let engine = cx.update(|cx| Engine::new(cx));

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

    assert_eq!(
        module1
            .call_export::<String>("repeat".into(), vec![Box::new("hello"), Box::new(3)])
            .await
            .unwrap(),
        "hellohellohello"
    );

    assert_eq!(
        module2
            .call_export::<usize>("return5".into(), vec![])
            .await
            .unwrap(),
        5
    );
}
