use call::ActiveCall;
use gpui::{executor::Deterministic, Entity, ModelContext, TestAppContext};
use project::shared_model_handle::{
    register_shared_model, CreateMessage, SharedModel, SharedModelHandleExtension,
};
use serde_json::json;

use crate::integration_tests::TestServer;

#[gpui::test]
async fn test_model_sharing(
    deterministic: std::sync::Arc<Deterministic>,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    struct TestModel {
        num: u64,
    }
    impl Entity for TestModel {
        type Event = ();
    }
    impl SharedModel for TestModel {
        fn type_key() -> &'static str {
            "TestModel"
        }
        fn create_message(&self) -> CreateMessage {
            CreateMessage(self.num)
        }
        fn create_from_create_message(msg: CreateMessage, _cx: &mut ModelContext<Self>) -> Self {
            TestModel { num: msg.0 }
        }
    }

    cx_a.update(|cx| {
        register_shared_model::<TestModel>(cx);
    });
    cx_b.update(|cx| {
        register_shared_model::<TestModel>(cx);
    });

    deterministic.forbid_parking();

    let mut server = TestServer::start(&deterministic).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a.fs.insert_tree("/a", json!({})).await;
    let (project_a, _) = client_a.build_local_project("/a", cx_a).await;

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_remote_project(project_id, cx_b).await;

    deterministic.run_until_parked();

    let a_model = cx_a.add_model(|_cx| TestModel { num: 999 });

    let a_remote_model = project_a.update(cx_a, |project, _cx| a_model.to_remote(project));

    // Simulate transmitting a_remote_model to b with the closure
    let b_model = project_b
        .update(cx_b, |project, cx| a_remote_model.upgrade(project, cx))
        .await
        .unwrap();

    deterministic.run_until_parked();

    assert_eq!(
        a_model.read_with(cx_a, |m, _| m.num),
        cx_b.read(|cx| b_model.read_with(cx, |m, _| m.num))
    )
}
