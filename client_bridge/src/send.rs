use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn send_over<T: Serialize>(stream: &mut TcpStream, data: T) -> Option<()> {
    let serialized = serde_json::to_vec(&data).ok()?;
    let len = serialized.len();

    stream.write_all(&len.to_le_bytes()).ok()?;
    stream.write_all(&serialized).ok()
}

pub fn recv_over<T: DeserializeOwned>(stream: &mut TcpStream) -> Option<T> {
    let mut len = [0u8; size_of::<usize>()];
    stream.read_exact(&mut len).ok()?;
    let len = usize::from_le_bytes(len);

    let mut vec = vec![0u8; len];
    stream.read_exact(&mut vec).ok()?;
    serde_json::from_slice(&vec).ok()
}
