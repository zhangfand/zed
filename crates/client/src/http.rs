pub use anyhow::{anyhow, Result};
use futures::future::BoxFuture;
use gpui::AppContext;
use std::sync::Arc;
pub use surf::{
    http::{Method, Response as ServerResponse},
    Request, Response, Url,
};

pub fn global(cx: &AppContext) -> &Arc<dyn HttpClient> {
    cx.global::<Arc<dyn HttpClient>>()
}

pub trait HttpClient: Send + Sync {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response>>;
}

pub fn client() -> Arc<dyn HttpClient> {
    Arc::new(surf::client())
}

impl HttpClient for surf::Client {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response>> {
        Box::pin(async move {
            Ok(self
                .send(req)
                .await
                .map_err(|e| anyhow!("http request failed: {}", e))?)
        })
    }
}
