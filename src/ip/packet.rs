use crate::error::CustomRes;

use super::{header::IpHeader, payload::IpPayload, Deserialize, Serialize};

pub struct IpPacket {
    pub header: IpHeader,
    pub payload: IpPayload,
}

impl IpPacket {
    pub fn prepare_send(&mut self) {
        self.header.update_payload_length(self.payload.size());
        self.header.update_checksum();
    }
}

impl Serialize for IpPacket {
    fn serialize(self, buf: &mut Vec<u8>) {
        self.header.serialize(buf);
        self.payload.serialize(buf);
    }
}

impl Deserialize for IpPacket {
    fn deserialize(buf: &[u8]) -> CustomRes<Self> {
        let ihl = (buf[0] & 0x0F) as usize * 4;

        let (header_buf, data_buf) = (&buf[0..ihl], &buf[ihl..]);
        let header = IpHeader::deserialize(header_buf)?;
        let payload = IpPayload::deserialize(header.protocol(), data_buf)?;
        Ok(Self { header, payload })
    }
}
