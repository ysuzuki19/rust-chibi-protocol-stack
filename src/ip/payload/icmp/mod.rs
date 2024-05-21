mod handle;
mod serializer;

use num_enum::FromPrimitive;

use super::IpPayload;

const HEADER_LEN: usize = 8;

#[derive(Debug, FromPrimitive, Clone)]
#[repr(u8)]
pub enum IcmpType {
    EchoReply = 0,
    EchoRequest = 8,
    #[num_enum(default)]
    Unknown,
}

#[derive(Debug)]
pub struct IcmpPacket {
    icmp_type: IcmpType,
    code: u8,
    checksum: u16,
    identifier: u16,
    sequence_number: u16,
    payload: Vec<u8>,
}

impl IcmpPacket {
    pub fn size(&self) -> usize {
        HEADER_LEN + self.payload.len()
    }
}

impl From<IcmpPacket> for IpPayload {
    fn from(packet: IcmpPacket) -> Self {
        IpPayload::Icmp(packet)
    }
}
