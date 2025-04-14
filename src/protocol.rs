use color_eyre::{Result, eyre::bail};
use serde_json::Value;
use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn ping_server(addr: SocketAddr, host: &str, debug: bool) -> Result<(Value, Duration)> {
    let mut conn = TcpStream::connect(addr).await?;

    let mut request = vec![0, 4];
    request.extend(encode_varint(host.len()));
    request.extend(host.as_bytes());
    request.extend(addr.port().to_le_bytes());
    request.push(1);
    let mut packet = encode_varint(request.len());
    packet.extend(request);
    packet.extend([1, 0]);
    conn.write_all(&packet).await?;

    let start = Instant::now();
    let _packet_length = read_varint(&mut conn).await?;
    let latency = start.elapsed();
    if conn.read_u8().await? != 0 {
        bail!("received unexpected response from server");
    }

    let mut buf = vec![0u8; read_varint(&mut conn).await?];
    conn.read_exact(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf);
    if debug {
        println!("{response}");
    }
    Ok((serde_json::from_str(&response)?, latency))
}

async fn read_varint(conn: &mut TcpStream) -> Result<usize> {
    let mut int = 0;
    let mut shift = 0;
    loop {
        let byte = conn.read_u8().await?;
        int |= ((byte & 0x7F) as usize) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 35 {
            bail!("received VarInt is too big");
        }
    }
    Ok(int)
}

#[allow(clippy::cast_possible_truncation)]
fn encode_varint(mut value: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(4);
    while value >= 0x80 {
        bytes.push(((value & 0x7F) | 0x80) as u8);
        value >>= 7;
    }
    bytes.push(value as u8);
    bytes
}
