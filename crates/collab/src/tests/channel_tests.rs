use gpui::{TestAppContext, executor::Deterministic};

use crate::tests::TestServer;

#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
async fn test_get_channels(deterministic: std::sync::Arc<Deterministic>, cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    deterministic.forbid_parking();
    let mut server = TestServer::start(&deterministic).await;

    let _client_a = server.create_client(cx_a, "user_a").await;
    let _client_b = server.create_client(cx_b, "user_b").await;

    let channels_a = cx_a.read(channels::Channels::global);
    let channels_b = cx_a.read(channels::Channels::global);

    let get_channels = channels_a.update(cx_a, |channels, cx| {
        channels.get_channels(cx)
    });

    dbg!(get_channels.await);

    panic!();
}
