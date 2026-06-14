use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::error::Error;

pub mod error;
pub mod frame;
pub mod handshake;
pub mod message;
pub mod server;

pub struct Socket<S>
where
    S: Read + Write,
{
    #[expect(dead_code)]
    stream: S,
}

fn main() -> Result<(), Error> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9001);
    let listener = TcpListener::bind(address)?;

    println!("listening on http://{address}");

    for stream in listener.incoming() {
        let mut stream = stream?;

        let req = match read_http_request(&mut stream) {
            Ok(r) => r,
            Err(Error::ConnectionClosed) => continue, // browser closed early
            Err(e) => return Err(e),
        };

        println!("{req}");

        let Some(key) = handshake::websocket_key(&req) else {
            let response =
                "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            stream.write_all(response.as_bytes())?;
            continue;
        };

        let response = handshake::upgrade_response(key);

        stream.write_all(response.as_bytes())?;

        println!("websocket upgraded");
    }

    Ok(())
}

fn read_http_request(stream: &mut TcpStream) -> Result<String, Error> {
    let mut bytes = Vec::new();

    loop {
        let mut chunk = [0_u8; 1024];

        let n = stream.read(&mut chunk)?;

        if n == 0 {
            return Err(Error::ConnectionClosed);
        }

        let Some(v) = chunk.get(..n) else {
            eprintln!("can't happen as read guarantees that 0 <= n <= buf.len()");
            continue;
        };

        bytes.extend_from_slice(v);

        if bytes.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }

    Ok(String::from_utf8(bytes)?)
}
