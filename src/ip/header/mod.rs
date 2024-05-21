mod flags_fragment_offset;
mod version_ihl;

use crate::{
    error::{CustomErr, CustomRes},
    util::checksum::Checksum,
};

use self::{flags_fragment_offset::FlagsFragmentOffset, version_ihl::VersionIhl};

use super::{
    addr::{Addr, AddrPair},
    protocol::Protocol,
    Deserialize, Serialize,
};

const IP_HDR_SIZE_MIN: usize = 20;

#[derive(Debug, Clone)]
pub struct IpHeader {
    version_ihl: VersionIhl,
    type_of_service: u8,
    total_length: u16,
    identification: u16,
    flags_fragment_offset: FlagsFragmentOffset,
    ttl: u8,
    protocol: Protocol,
    header_checksum: u16,
    source_address: Addr,
    destination_address: Addr,
}

impl IpHeader {
    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }

    pub fn size(&self) -> usize {
        IP_HDR_SIZE_MIN
    }

    pub fn create_addr_pair(&self) -> AddrPair {
        AddrPair::new(self.source_address, self.destination_address)
    }

    pub fn update_payload_length(&mut self, data_length: usize) {
        self.total_length = (self.size() + data_length) as u16;
    }

    pub fn update_checksum(&mut self) {
        self.header_checksum = Checksum::new()
            .u8((self.version_ihl.value(), self.type_of_service))
            .u16(self.total_length)
            .u16(self.identification)
            .u16(self.flags_fragment_offset.value())
            .u8((self.ttl, self.protocol.clone() as u8))
            .u32(self.source_address.value())
            .u32(self.destination_address.value())
            .export();
    }

    pub fn prepare_reply(&self) -> Self {
        Self {
            version_ihl: self.version_ihl.clone(),
            type_of_service: self.type_of_service,
            total_length: 0,
            identification: self.identification,
            flags_fragment_offset: self.flags_fragment_offset.clone(),
            ttl: self.ttl,
            protocol: self.protocol.clone(),
            header_checksum: 0,
            source_address: self.destination_address,
            destination_address: self.source_address,
        }
    }
}

impl Serialize for IpHeader {
    fn serialize(self, buf: &mut Vec<u8>) {
        buf.push(self.version_ihl.value());
        buf.push(self.type_of_service);
        buf.extend_from_slice(&self.total_length.to_be_bytes());
        buf.extend_from_slice(&self.identification.to_be_bytes());
        buf.extend_from_slice(&self.flags_fragment_offset.value().to_be_bytes());
        buf.push(self.ttl);
        buf.push(self.protocol as u8);
        buf.extend_from_slice(&self.header_checksum.to_be_bytes());
        buf.extend_from_slice(&self.source_address.value().to_be_bytes());
        buf.extend_from_slice(&self.destination_address.value().to_be_bytes());
    }
}

impl Deserialize for IpHeader {
    fn deserialize(buf: &[u8]) -> CustomRes<Self> {
        if buf.len() < IP_HDR_SIZE_MIN {
            return Err(CustomErr::PacketDropSilent);
        }
        let version_ihl = VersionIhl::new(buf[0]);
        if !version_ihl.is_supported() {
            return Err(CustomErr::PacketDropSilent);
        }
        Ok(IpHeader {
            version_ihl,
            type_of_service: buf[1],
            total_length: u16::from_be_bytes([buf[2], buf[3]]),
            identification: u16::from_be_bytes([buf[4], buf[5]]),
            flags_fragment_offset: FlagsFragmentOffset::new([buf[6], buf[7]]),
            ttl: buf[8],
            protocol: Protocol::from(buf[9]),
            header_checksum: u16::from_be_bytes([buf[10], buf[11]]),
            source_address: Addr::from([buf[12], buf[13], buf[14], buf[15]]),
            destination_address: Addr::from([buf[16], buf[17], buf[18], buf[19]]),
        })
    }
}
