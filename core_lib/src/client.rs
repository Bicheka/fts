//! Contains the logic for the client, eg: sending requests, downloading files

use tokio::{io::AsyncWriteExt, net::TcpStream};
use bincode;
use crate::Request;

/// sends a request serialized as bytes through the tcp stream
pub async fn send_request(stream: &mut TcpStream, request: &Request) -> std::io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(request).unwrap();
    stream.write_all(&encoded).await?;
    Ok(())
}

// TODO add download logic