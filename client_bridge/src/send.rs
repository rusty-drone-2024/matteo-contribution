use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

pub async fn send_over<T: Serialize>(stream: &mut TcpStream, data: T) -> Option<()> {
    let serialized = serde_json::to_vec(&data).ok()?;
    let len = serialized.len();

    stream.write_all(&len.to_le_bytes()).await.ok()?;
    stream.write_all(&serialized).await.ok()
}

pub async fn recv_over<T: DeserializeOwned>(stream: &mut TcpStream) -> Option<T> {
    let mut len = [0u8; size_of::<usize>()];
    stream.read_exact(&mut len).await.ok()?;
    let len = usize::from_le_bytes(len);

    let mut vec = vec![0u8; len];
    stream.read_exact(&mut vec).await.ok()?;
    serde_json::from_slice(&vec).ok()
}
