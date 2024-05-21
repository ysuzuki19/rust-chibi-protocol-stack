use crate::{
    ip::{
        addr::AddrPair,
        payload::{handle::HandlePacket, HandleStatus},
    },
    util::checksum::Checksum,
};

use super::{IcmpPacket, IcmpType};

impl HandlePacket for IcmpPacket {
    async fn handle_packet(&self, addr_pair: AddrPair) -> HandleStatus {
        println!("ICMP packet from({:?}): {:?}", addr_pair.src_addr, self);
        match self.icmp_type {
            IcmpType::EchoRequest => {
                let mut reply = Self {
                    icmp_type: IcmpType::EchoReply,
                    code: 0,
                    checksum: 0,
                    identifier: self.identifier,
                    sequence_number: self.sequence_number,
                    payload: self.payload.clone(),
                };
                reply.checksum = Checksum::new()
                    .u8((reply.icmp_type.clone() as u8, reply.code))
                    // ignore checksum field
                    .u16(reply.identifier)
                    .u16(reply.sequence_number)
                    .vec(&reply.payload)
                    .export();
                HandleStatus::Send(vec![reply.into()])
            }
            _ => HandleStatus::Stop,
        }
    }
}
