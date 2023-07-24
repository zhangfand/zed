mod channels_panel;
mod channels_panel_settings;

pub use channels_panel::*;
use gpui::{AppContext, Entity, ModelContext, ModelHandle, Task};
use rpc::proto;

use std::sync::Arc;

use client::Client;

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let channels = cx.add_model(|cx| Channels::new(client, cx));
    cx.set_global(channels);
    channels_panel::init(cx);
}

#[derive(Debug, Clone)]
pub struct Channel {
    id: u64,
    name: String,
    sub_channels: Vec<Channel>,
    _room: Option<()>,
}

impl Channel {
    fn new(id: u64, name: impl AsRef<str>, members: Vec<Channel>) -> Channel {
        Channel {
            name: name.as_ref().to_string(),
            id,
            sub_channels: members,
            _room: None,
        }
    }

    fn members(&self) -> &[Channel] {
        &self.sub_channels
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub struct Channels {
    roots: Vec<u64>,
    client: Arc<Client>,
    channels: Vec<Channel>,
}

impl Channels {
    fn channels(&self) -> Vec<Channel> {
        self.channels.clone()
    }
}

pub enum ChannelEvents {}

impl Entity for Channels {
    type Event = ChannelEvents;
}

impl Channels {
    pub fn global(cx: &AppContext) -> ModelHandle<Self> {
        cx.global::<ModelHandle<Self>>().clone()
    }

    fn new(client: Arc<Client>, _cx: &mut AppContext) -> Self {
        //TODO: Subscribe to channel updates from the server
        Channels {
            roots: Default::default(),
            channels: Default::default(),
            client,
        }
    }

    pub fn add_root_channel(&mut self, cx: &mut ModelContext<'_, Self>, channel_id: u64) -> Task<anyhow::Result<Channel>> {
        self.roots.push(channel_id);
        self.get_channel(channel_id, cx)
    }

    /*
     *
     */

    pub fn get_channel(
        &mut self,
        cx: &mut ModelContext<'_, Self>,
        id: u64,
    ) -> Task<anyhow::Result<Vec<Channel>>> {
        let client = self.client.clone();
        cx.spawn(|_this, _cx| async move {
            client
                .request(proto::GetChannels {
                    channel_roots: vec![id],
                })
                .await
                .map(|response| {
                    response
                        .channels
                        .into_iter()
                        .map(|channel| Channel::new(channel.id, channel.name, vec![]))
                        .collect()
                })
        })
    }

    pub fn get_channels(
        &mut self,
        cx: &mut ModelContext<'_, Self>,
    ) -> Task<anyhow::Result<Vec<Channel>>> {
        let client = self.client.clone();
        let roots = self.roots.clone();
        cx.spawn(|_this, _cx| async move {
            client
                .request(proto::GetChannels {
                    channel_roots: roots,
                })
                .await
                .map(|response| {
                    response
                        .channels
                        .into_iter()
                        .map(|channel| Channel::new(channel.id, channel.name, vec![]))
                        .collect()
                })
        })
    }
}
