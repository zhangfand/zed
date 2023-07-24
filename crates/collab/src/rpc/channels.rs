use rpc::proto;

use super::{Server, Session, Response, Result};



pub fn init(server: &mut Server) {
    server.add_request_handler(get_channels);
}

async fn get_channels(
    request: proto::GetChannels,
    response: Response<proto::GetChannels>,
    _session: Session,
) -> Result<()> {

    for channel in request.channel_roots.iter() {
        println!("Channel: {}", channel);
    }

    response.send(proto::GetChannelsResponse {
        channels: vec![proto::Channel { id: 5, name: "SERVER CHANNEL".to_string() }],
    })?;

    Ok(())
}
