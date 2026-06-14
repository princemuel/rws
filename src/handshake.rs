use base64::engine::general_purpose;
use base64::prelude::Engine as _;
use sha1::{Digest as _, Sha1};

pub const GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

#[must_use]
pub fn upgrade_response(key: &str) -> String {
    let accept = websocket_accept(key);

    format!(
        concat!(
            "HTTP/1.1 101 Switching Protocols\r\n",
            "Upgrade: websocket\r\n",
            "Connection: Upgrade\r\n",
            "Sec-WebSocket-Accept: {}\r\n",
            "\r\n"
        ),
        accept
    )
}

#[must_use]
pub fn websocket_key(request: &str) -> Option<&str> {
    request.lines().find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.trim().eq_ignore_ascii_case("Sec-WebSocket-Key").then(|| value.trim())
    })
}

#[must_use]
pub fn websocket_accept(key: &str) -> String {
    let mut sha1 = Sha1::new();

    sha1.update(key.as_bytes());
    sha1.update(GUID.as_bytes());

    let digest = sha1.finalize();

    general_purpose::STANDARD.encode(digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_example() {
        let key = "dGhlIHNhbXBsZSBub25jZQ==";

        let accept = websocket_accept(key);

        assert_eq!(accept, "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=");
    }
}
