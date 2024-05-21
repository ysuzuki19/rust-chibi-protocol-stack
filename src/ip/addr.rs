use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Addr(u32);

impl Addr {
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl From<u32> for Addr {
    fn from(addr: u32) -> Self {
        Addr(addr)
    }
}

impl From<[u8; 4]> for Addr {
    fn from(addr: [u8; 4]) -> Self {
        Addr(u32::from_be_bytes(addr))
    }
}

impl Debug for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parts: [u8; 4] = self.0.to_be_bytes();
        let ip_addr_text = format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], parts[3]);
        write!(f, "{}", ip_addr_text)
    }
}

pub struct AddrPair {
    pub src_addr: Addr,
    pub dst_addr: Addr,
}

impl AddrPair {
    pub fn new(src_addr: Addr, dst_addr: Addr) -> Self {
        AddrPair { src_addr, dst_addr }
    }
}
