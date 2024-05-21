use super::PacketFilter;

mod handle;
mod serializer;

const HEADER_LEN: usize = 8;

#[derive(Debug)]
pub struct UdpPacket {
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16,
    payload: Vec<u8>,
}

impl UdpPacket {
    pub fn size(&self) -> usize {
        HEADER_LEN + self.payload.len()
    }

    // pub fn payload(&self) -> &[u8] {
    //     &self.payload
    // }

    pub fn filter(&self, port: u16) -> PacketFilter {
        if self.destination_port == port {
            PacketFilter::Pass
        } else {
            PacketFilter::Drop
        }
    }
}
