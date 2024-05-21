use crate::{
    error::{CustomErr, CustomRes},
    ip::{Buf, Deserialize, Serialize},
};

use super::{IcmpPacket, IcmpType, HEADER_LEN};

impl Serialize for IcmpPacket {
    fn serialize(self, buf: &mut Buf) {
        buf.push(self.icmp_type as u8);
        buf.push(self.code);
        buf.extend_from_slice(&self.checksum.to_be_bytes());
        buf.extend_from_slice(&self.identifier.to_be_bytes());
        buf.extend_from_slice(&self.sequence_number.to_be_bytes());
        buf.extend_from_slice(&self.payload);
    }
}

impl Deserialize for IcmpPacket {
    fn deserialize(buf: &[u8]) -> CustomRes<Self> {
        if buf.len() < HEADER_LEN {
            return Err(CustomErr::Undefined("ICMP Packet Len is too short".into()));
        }
        Ok(Self {
            icmp_type: IcmpType::from(buf[0]),
            code: buf[1],
            checksum: u16::from_be_bytes([buf[2], buf[3]]),
            identifier: u16::from_be_bytes([buf[4], buf[5]]),
            sequence_number: u16::from_be_bytes([buf[6], buf[7]]),
            payload: buf[8..].to_vec(),
        })
    }
}
