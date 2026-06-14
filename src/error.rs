#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("invalid opcode {0}")]
    InvalidOpcode(u8),

    #[error(transparent)]
    Utf8(#[from] core::str::Utf8Error),

    #[error(transparent)]
    FromUtf8(#[from] std::string::FromUtf8Error),

    #[error("connection closed")]
    ConnectionClosed,

    #[error("control frame too big")]
    FrameControlTooBig,

    #[error("unnegotiated reserved frame bits")]
    FrameReservedBitsNotNegotiated,

    #[error("Close frame was sent")]
    FrameCloseSent,

    #[error("Unexpected opcode frame")]
    FrameUnexpectedOpcode,

    #[error("server handshake: duplicate Sec-WebSocket-Key")]
    ServerHandshakeDuplicateKey,

    #[error("server handshake: Sec-WebSocket-Key is missing")]
    ServerHandshakeNoKey,

    #[error("client handshake: disconnected before request completed")]
    ClientHandshakeDisconnected,

    #[error("client handshake: bad Sec-WebSocket-Accept")]
    ClientHandshakeBadAccept,

    #[error("client handshake: duplicate Sec-WebSocket-Accept")]
    ClientHandshakeDuplicateAccept,

    #[error("client handshake: no Sec-WebSocket-Accept")]
    ClientHandshakeNoAccept,
}
