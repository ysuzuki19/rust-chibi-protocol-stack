mod connection;
mod converts;
mod serializer;
mod types;

use once_cell::sync::Lazy;

use crate::{
    ip::{
        addr::AddrPair,
        payload::tcp::connection::{SockAddr, TcpState},
        protocol::Protocol,
    },
    util::checksum::Checksum,
};

use self::{
    connection::ConnectionPool,
    types::{DataOffsetReserved, TcpFlags},
};

use super::{handle::HandlePacket, HandleStatus, PacketFilter};

const HEADER_MIN_LEN: usize = 20;

#[derive(Debug)]
pub struct TcpPacket {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgment_number: u32,
    data_offset_reserved: DataOffsetReserved,
    flags: TcpFlags,
    window_size: u16,
    checksum: u16,
    urgent_pointer: u16,
    options: Vec<u8>,
    payload: Vec<u8>,
}

impl TcpPacket {
    pub fn size(&self) -> usize {
        HEADER_MIN_LEN + self.options.len() + self.payload.len()
    }

    pub fn filter(&self, port: u16) -> PacketFilter {
        if self.destination_port == port {
            PacketFilter::Pass
        } else {
            PacketFilter::Drop
        }
    }

    fn update_checksum(&mut self, addr_pair: AddrPair) {
        self.checksum = Checksum::new()
            // pseudo header
            .u32(addr_pair.src_addr.value())
            .u32(addr_pair.dst_addr.value())
            .u8((0, Protocol::Tcp as u8))
            .u32(self.size() as u32) // tcp length
            // header
            .u16(self.source_port)
            .u16(self.destination_port)
            .u32(self.sequence_number)
            .u32(self.acknowledgment_number)
            .u8((self.data_offset_reserved.value(), self.flags.bits()))
            .u16(self.window_size)
            .u16(self.urgent_pointer)
            .vec(&self.options)
            .vec(&self.payload)
            .export();
    }
}

static CONNECTION_POOL: Lazy<ConnectionPool> = Lazy::new(ConnectionPool::new);

impl HandlePacket for TcpPacket {
    async fn handle_packet(&self, addr_pair: AddrPair) -> HandleStatus {
        println!("TCP Packet from({:?}): {:?}", addr_pair.src_addr, self);
        let remote_sock_addr = SockAddr::new(addr_pair.src_addr, self.source_port);
        let con = CONNECTION_POOL.get(remote_sock_addr).await;

        /*
        {
            // Quick Response for Established Connection without mut connection
            if matches!(con.read().await.state, TcpState::Established)
                && !self.flags.contains(TcpFlags::FIN)
            {
                println!(
                    "Connection Established and Continue Msg={}",
                    String::from_utf8_lossy(&self.payload)
                );
                let sequence_number = self.acknowledgment_number;
                let acknowledgment_number = self.sequence_number + self.payload.len() as u32;
                let mut reply = Self {
                    source_port: self.destination_port,
                    destination_port: self.source_port,
                    sequence_number,
                    acknowledgment_number,
                    data_offset_reserved: DataOffsetReserved(0x50),
                    flags: TcpFlags::ACK,
                    window_size: self.window_size,
                    checksum: 0,
                    urgent_pointer: 0,
                    options: self.options.clone(),
                    payload: vec![],
                };
                reply.update_checksum(addr_pair);
                return HandleStatus::Send(vec![reply.into()]);
            }
        }
        */

        let mut con = con.write().await;
        match con.state {
            // TcpState::Closed => {
            //     if self.flags.contains(TcpFlags::SYN) {
            //         con.state = TcpState::Listen;
            //         println!("Connection State: {:?}", con.state);
            //         self.flags.insert(TcpFlags::ACK);
            //         return HandleStatus::Send;
            //     }
            // }
            TcpState::Listen => {
                if self.flags.contains(TcpFlags::SYN) {
                    let mut reply = Self {
                        source_port: self.destination_port,
                        destination_port: self.source_port,
                        sequence_number: 1,
                        acknowledgment_number: self.sequence_number + 1,
                        data_offset_reserved: DataOffsetReserved::new(0x50),
                        flags: TcpFlags::SYN | TcpFlags::ACK,
                        window_size: self.window_size,
                        checksum: 0,
                        urgent_pointer: 0,
                        options: self.options.clone(),
                        payload: vec![],
                    };
                    con.state = TcpState::SynReceived;
                    reply.update_checksum(addr_pair);
                    println!("Connection State: {:?}", con.state);
                    return HandleStatus::Send(vec![reply.into()]);
                } else {
                    return HandleStatus::Stop;
                }
            }
            TcpState::SynReceived => {
                println!("SynReceived Packet: {:?}", self);
                if self.flags.contains(TcpFlags::ACK) {
                    con.state = TcpState::Established;
                    println!("Connection State: {:?}", con.state);
                    return HandleStatus::Stop;
                } else {
                    println!("Connection State: {:?}", con.state);
                    return HandleStatus::Stop;
                }
            }
            TcpState::Established => {
                if self.flags.contains(TcpFlags::FIN) {
                    let mut reply = Self {
                        source_port: self.destination_port,
                        destination_port: self.source_port,
                        sequence_number: self.acknowledgment_number,
                        acknowledgment_number: self.sequence_number + 1,
                        data_offset_reserved: DataOffsetReserved::new(0x50),
                        flags: TcpFlags::ACK,
                        window_size: self.window_size,
                        checksum: 0,
                        urgent_pointer: 0,
                        options: self.options.clone(),
                        payload: vec![],
                    };
                    if self.flags.contains(TcpFlags::ACK) {
                        con.state = TcpState::Closed;
                    } else {
                        con.state = TcpState::LastAck;
                    }
                    println!("Connection State: {:?}", con.state);
                    reply.update_checksum(addr_pair);
                    return HandleStatus::Send(vec![reply.into()]);
                } else {
                    println!(
                        "Connection Established and Continue Msg={}",
                        String::from_utf8_lossy(&self.payload)
                    );
                    let mut reply = Self {
                        source_port: self.destination_port,
                        destination_port: self.source_port,
                        sequence_number: self.acknowledgment_number,
                        acknowledgment_number: self.sequence_number + self.payload.len() as u32,
                        data_offset_reserved: DataOffsetReserved::new(0x50),
                        flags: TcpFlags::ACK,
                        window_size: self.window_size,
                        checksum: 0,
                        urgent_pointer: 0,
                        options: self.options.clone(),
                        payload: vec![],
                    };
                    reply.update_checksum(addr_pair);
                    return HandleStatus::Send(vec![reply.into()]);
                }
            }

            // TcpState::CloseWait => {
            //     if self.flags.contains(TcpFlags::ACK) {
            // }
            TcpState::LastAck => {
                if self.flags.contains(TcpFlags::ACK) {
                    con.state = TcpState::Closed;
                    println!("Connection State: {:?}", con.state);
                    return HandleStatus::Stop;
                }
            }
            _ => {
                println!("Connection State: {:?}", con.state);
                return HandleStatus::Stop;
            }
        }
        HandleStatus::Stop
    }
}
