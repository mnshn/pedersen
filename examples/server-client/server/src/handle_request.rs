use std::time::Instant;

use crate::utils::communicate_with_client;
use anyhow::Result;
use futures::StreamExt;
use warp::ws::WebSocket;

use crate::types::Storage;

pub(crate) async fn handle_request(
    ws: WebSocket,
    _storage: Storage,
    client_id: u128,
) -> Result<()> {
    let (mut _writer, mut _reader) = ws.split();

    info!("Starting handle req for client {:x}", client_id);
    let start = Instant::now();

    // stuff
    let data = vec![];
    let response = communicate_with_client(&mut _writer, &mut _reader, &data).await?;
    info!("{:?}", response);

    let time_passed = start.elapsed().as_millis();
    info!(
        "Completed handle req for client {:x}, time passed: {}ms",
        client_id, time_passed
    );

    Ok(())
}
