use crate::ip::addr::AddrPair;

use super::IpPayload;

pub trait HandlePacket {
    async fn handle_packet(&self, addr_pair: AddrPair) -> HandleStatus;
}

pub enum HandleStatus {
    Send(Vec<IpPayload>),
    Stop,
}
