use crate::types::{WarpResult, WsMessage};

use anyhow::{anyhow, Result};
use futures::{Sink, SinkExt, Stream, StreamExt};

pub(crate) async fn communicate_with_client<W, R>(
    write_half: &mut W,
    read_half: &mut R,
    bytes: &[u8],
) -> Result<Vec<u8>>
where
    W: Sink<WsMessage> + Unpin,
    R: Stream<Item = WarpResult<WsMessage>> + Unpin,
{
    // Some error which doesn't implement Debug is here
    let _ = write_half.send(WsMessage::binary(bytes)).await;

    let response = read_half
        .next()
        .await
        .ok_or_else(|| anyhow!("Can't read client response, the stream was closed"))?
        .map_err(|e| anyhow!("Error reading from stream: {:?}", e))?;

    if !response.is_binary() {
        return err!("Wrong message format, expected to be a binary data");
    }

    Ok(response.as_bytes().to_vec())
}
