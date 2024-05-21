use crate::ip::payload::IpPayload;

use super::TcpPacket;

impl From<TcpPacket> for IpPayload {
    fn from(packet: TcpPacket) -> Self {
        IpPayload::Tcp(packet)
    }
}
