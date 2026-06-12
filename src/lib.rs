#![feature(integer_widen_truncate)]
#![feature(const_trait_impl)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RwsOpcode {
    Cont = 0x0,
    Text = 0x1,
    Bin = 0x2,
    Close = 0x8,
    Ping = 0x9,
    Pong = 0xa,
}

impl RwsOpcode {
    #[inline]
    #[must_use]
    pub const fn is_opcode(&self) -> bool { 0x8 <= self.code() && self.code() <= 0xf }

    #[inline]
    #[must_use]
    #[expect(clippy::as_conversions)]
    pub const fn code(self) -> i8 { (self as isize).saturating_truncate() }
}

#[derive(Clone, Copy, Debug)]
pub struct RwsFrame {
    pub fin: bool,
    pub opcode: RwsOpcode,
    pub payload_len: usize,
    pub payload: *const u8,
}

#[derive(Clone, Copy, Debug)]
pub struct RwsMessage {
    kind: RwsMessageKind,
    chunks: *const RwsMessageChunk,
}

#[derive(Clone, Copy, Debug)]
pub enum RwsMessageKind {
    Text = 0x1,
    Bin = 0x2,
}
#[derive(Clone, Debug)]
pub struct RwsMessageChunk {
    pub next: Box<RwsMessageChunk>,
    pub payload_len: usize,
    pub payload: *const u8,
}
