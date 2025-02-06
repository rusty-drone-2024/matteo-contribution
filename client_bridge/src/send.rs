use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Send an object over the given stream.
/// It is an async function which must be awaited.
/// # Return
/// The `Ok(())` if it successfully written the data to the stream.
/// # Errors
/// Return a `BridgeSendError` based on the problem that occurred.
pub async fn send_over<T: Serialize>(
    stream: &mut TcpStream,
    data: T,
) -> Result<(), BridgeSendError> {
    let serialized = serde_json::to_vec(&data).map_err(|_| BridgeSendError::ParseFailure)?;
    let len = serialized.len();

    stream
        .write_all(&len.to_le_bytes())
        .await
        .map_err(|_| BridgeSendError::IoError)?;
    stream
        .write_all(&serialized)
        .await
        .map_err(|_| BridgeSendError::IoError)
}

/// Receive an object over the given stream.
/// It is an async function which must be awaited.
/// # Return
/// The `Ok(t: T)` if it successfully read the data from the stream.
/// # Errors
/// Return a `BridgeRecvError` based on the problem that occurred.
pub async fn recv_over<T: DeserializeOwned>(stream: &mut TcpStream) -> Result<T, BridgeRecvError> {
    let mut len = [0u8; size_of::<usize>()];
    stream
        .read_exact(&mut len)
        .await
        .map_err(|_| BridgeRecvError::NotEnoughBytes)?;

    let len = usize::from_le_bytes(len);
    let mut vec = vec![0u8; len];
    stream
        .read_exact(&mut vec)
        .await
        .map_err(|_| BridgeRecvError::NotEnoughBytes)?;

    serde_json::from_slice(&vec).map_err(|_| BridgeRecvError::Unparsable)
}

/// Error that the bridge can encounter during reading
pub enum BridgeRecvError {
    /// Stream was closed (EOF found) before full message written
    NotEnoughBytes,
    /// The data found has right size but it is not in json
    /// or does not represent the right object.
    Unparsable,
}

/// Error that the bridge can encounter during writing
pub enum BridgeSendError {
    /// IO error while trying to write.
    IoError,
    /// The data coudn't be parsed. Possible an internal bug.
    ParseFailure,
}
