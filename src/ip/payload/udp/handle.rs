use crate::ip::{
    addr::AddrPair,
    payload::{handle::HandlePacket, HandleStatus},
};

use super::UdpPacket;

impl HandlePacket for UdpPacket {
    async fn handle_packet(&self, addr_pair: AddrPair) -> HandleStatus {
        println!("UDP packet from({:?}): {:?}", addr_pair.src_addr, self);
        HandleStatus::Stop
    }
}
