pub mod handle;
pub mod icmp;
pub mod tcp;
mod udp;

use crate::error::CustomRes;

use self::{
    handle::{HandlePacket, HandleStatus},
    icmp::IcmpPacket,
    tcp::TcpPacket,
    udp::UdpPacket,
};

use super::{addr::AddrPair, protocol::Protocol, Deserialize, Serialize};

pub enum IpPayload {
    Icmp(IcmpPacket),
    Udp(UdpPacket),
    Tcp(TcpPacket),
    Undefined(Vec<u8>),
}

pub enum PacketFilter {
    Pass,
    Drop,
}

impl IpPayload {
    pub fn size(&self) -> usize {
        match self {
            IpPayload::Icmp(icmp) => icmp.size(),
            IpPayload::Udp(udp) => udp.size(),
            IpPayload::Tcp(tcp) => tcp.size(),
            IpPayload::Undefined(data) => data.len(),
        }
    }

    pub fn filter(&self, port: u16) -> PacketFilter {
        match self {
            IpPayload::Icmp(_) => PacketFilter::Pass,
            IpPayload::Udp(udp) => udp.filter(port),
            IpPayload::Tcp(tcp) => tcp.filter(port),
            _ => PacketFilter::Drop,
        }
    }

    pub async fn handle(&self, addr_pair: AddrPair) -> HandleStatus {
        match self {
            IpPayload::Icmp(icmp) => icmp.handle_packet(addr_pair).await,
            IpPayload::Udp(udp) => udp.handle_packet(addr_pair).await,
            // sample: udp echo
            // IpPayload::Udp(udp) => {
            //     udp.swap_ports();
            //     HandleStatus::Send
            // }
            IpPayload::Tcp(tcp) => tcp.handle_packet(addr_pair).await,
            _ => HandleStatus::Stop,
        }
    }
}

impl IpPayload {
    pub fn deserialize(protocol: &Protocol, buf: &[u8]) -> CustomRes<Self> {
        match protocol {
            Protocol::Icmp => Ok(IpPayload::Icmp(IcmpPacket::deserialize(buf)?)),
            Protocol::Udp => Ok(IpPayload::Udp(UdpPacket::deserialize(buf)?)),
            Protocol::Tcp => Ok(IpPayload::Tcp(TcpPacket::deserialize(buf)?)),
            _ => Ok(IpPayload::Undefined(buf.to_vec())),
        }
    }
}

impl Serialize for IpPayload {
    fn serialize(self, buf: &mut Vec<u8>) {
        match self {
            IpPayload::Icmp(icmp) => icmp.serialize(buf),
            IpPayload::Udp(udp) => udp.serialize(buf),
            IpPayload::Tcp(tcp) => tcp.serialize(buf),
            IpPayload::Undefined(data) => buf.extend_from_slice(&data),
        }
    }
}
