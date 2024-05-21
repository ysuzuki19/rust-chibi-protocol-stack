use crate::{
    error::{CustomErr, CustomRes},
    ip::{Deserialize, Serialize},
};

use super::{
    types::{DataOffsetReserved, TcpFlags},
    TcpPacket, HEADER_MIN_LEN,
};

impl Serialize for TcpPacket {
    fn serialize(self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.source_port.to_be_bytes());
        buf.extend_from_slice(&self.destination_port.to_be_bytes());
        buf.extend_from_slice(&self.sequence_number.to_be_bytes());
        buf.extend_from_slice(&self.acknowledgment_number.to_be_bytes());
        buf.push(self.data_offset_reserved.value());
        buf.push(self.flags.bits());
        buf.extend_from_slice(&self.window_size.to_be_bytes());
        buf.extend_from_slice(&self.checksum.to_be_bytes());
        buf.extend_from_slice(&self.urgent_pointer.to_be_bytes());
        buf.extend_from_slice(&self.options);
        buf.extend_from_slice(&self.payload);
    }
}

impl Deserialize for TcpPacket {
    fn deserialize(buf: &[u8]) -> CustomRes<Self> {
        if buf.len() < HEADER_MIN_LEN {
            return Err(CustomErr::Undefined("Invalid TCP packet".into()));
        }
        let data_offset_reserved = DataOffsetReserved::new(buf[12]);
        let data_offset = data_offset_reserved.data_offset();
        let options = buf[HEADER_MIN_LEN..data_offset as usize].to_vec();
        let payload = buf[data_offset as usize..].to_vec();
        Ok(Self {
            source_port: u16::from_be_bytes([buf[0], buf[1]]),
            destination_port: u16::from_be_bytes([buf[2], buf[3]]),
            sequence_number: u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]),
            acknowledgment_number: u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]),
            data_offset_reserved,
            flags: TcpFlags::from_bits_truncate(buf[13]),
            window_size: u16::from_be_bytes([buf[14], buf[15]]),
            checksum: u16::from_be_bytes([buf[16], buf[17]]),
            urgent_pointer: u16::from_be_bytes([buf[18], buf[19]]),
            options,
            payload,
        })
    }
}
