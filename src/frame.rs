use crate::error::Error;

#[derive(Clone, Copy)]
pub struct FrameHeader {
    pub fin: bool,
    pub opcode: Opcode,
    pub masked: bool,
    pub payload_len: u64,
    pub mask: Option<[u8; 4]>,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    Text = 0x1,
    Binary = 0x2,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
    Continuation = 0x0,
    Text = 0x1,
    Binary = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xa,
}

impl Opcode {
    #[must_use]
    pub const fn is_control(self) -> bool {
        matches!(self, Opcode::Close | Opcode::Ping | Opcode::Pong)
    }
}

impl TryFrom<u8> for Opcode {
    type Error = Error;

    /// Decode the low 4 bits of the first frame byte.
    fn try_from(v: u8) -> Result<Self, Self::Error> {
        let code = match v {
            0x0 => Self::Continuation,
            0x1 => Self::Text,
            0x2 => Self::Binary,
            0x8 => Self::Close,
            0x9 => Self::Ping,
            0xa => Self::Pong,
            other => return Err(Error::InvalidOpcode(other)),
        };
        Ok(code)
    }
}

impl From<Opcode> for u8 {
    #[inline]
    fn from(v: Opcode) -> Self {
        match v {
            Opcode::Continuation => 0x0,
            Opcode::Text => 0x1,
            Opcode::Binary => 0x2,
            Opcode::Close => 0x8,
            Opcode::Ping => 0x9,
            Opcode::Pong => 0xa,
        }
    }
}

/// `0` reads, returned by `Read`/`Write`, mean "the peer closed the
/// connection" in this protocol i.e. there's no valid zero-length frame I/O.
pub const fn nonzero_or_closed(n: usize) -> Result<usize, Error> {
    if n == 0 { Err(Error::ConnectionClosed) } else { Ok(n) }
}
