use crate::{build_window_options, build_workspace, AppState};
use assets::Assets;
use client::{test::FakeHttpClient, ChannelList, Client, UserStore};
use gpui::MutableAppContext;
use language::LanguageRegistry;
use project::fs::FakeFs;
use settings::Settings;
use std::sync::Arc;
use theme::ThemeRegistry;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

pub fn test_app_state(cx: &mut MutableAppContext) -> Arc<AppState> {
    let settings = Settings::test(cx);
    editor::init(cx);
    cx.set_global(settings);
    cx.set_global(ThemeRegistry::new(Assets, cx.font_cache().clone()));
    cx.set_global(FakeHttpClient::with_404_response());
    let client = Client::new();
    let user_store = cx.add_model(|cx| UserStore::new(client.clone(), cx));
    cx.set_global(user_store.clone());
    LanguageRegistry::global(cx).add(Arc::new(language::Language::new(
        language::LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    )));

    Arc::new(AppState {
        channel_list: cx.add_model(|cx| ChannelList::new(client.clone(), cx)),
        client,
        user_store,
        fs: FakeFs::new(cx.background().clone()),
        build_window_options: &build_window_options,
        build_workspace: &build_workspace,
    })
}
