use crate::{
    error::{CustomErr, CustomRes},
    ip::{Buf, Deserialize, Serialize},
};

use super::{UdpPacket, HEADER_LEN};

impl Serialize for UdpPacket {
    fn serialize(self, buf: &mut Buf) {
        buf.extend_from_slice(&self.source_port.to_be_bytes());
        buf.extend_from_slice(&self.destination_port.to_be_bytes());
        buf.extend_from_slice(&self.length.to_be_bytes());
        buf.extend_from_slice(&self.checksum.to_be_bytes());
        buf.extend_from_slice(&self.payload);
    }
}

impl Deserialize for UdpPacket {
    fn deserialize(buf: &[u8]) -> CustomRes<Self> {
        if buf.len() < HEADER_LEN {
            return Err(CustomErr::Undefined("Invalid UDP packet".into()));
        }
        Ok(Self {
            source_port: u16::from_be_bytes([buf[0], buf[1]]),
            destination_port: u16::from_be_bytes([buf[2], buf[3]]),
            length: u16::from_be_bytes([buf[4], buf[5]]),
            checksum: u16::from_be_bytes([buf[6], buf[7]]),
            payload: buf[8..].to_vec(),
        })
    }
}
