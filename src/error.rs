use std::io;
use thiserror::Error;

pub type CustomRes<T> = Result<T, CustomErr>;

#[derive(Debug, Error)]
pub enum CustomErr {
    #[error("I/O error occurred: {0}")]
    IoError(#[from] io::Error),

    #[error("Tun error occurred: {0}")]
    TunError(#[from] tun::Error),

    #[error("TryFromSlice error occurred: {0}")]
    TryFromSliceError(#[from] std::array::TryFromSliceError),

    #[error("Send<Packet> error occurred: {0}")]
    SendPacketErr(#[from] tokio::sync::mpsc::error::SendError<crate::ip::packet::IpPacket>),

    #[error("Send<Packet> error occurred: {0}")]
    SendVecU8Err(#[from] tokio::sync::mpsc::error::SendError<std::vec::Vec<u8>>),

    #[error("TokioJoin error occurred: {0}")]
    TokioJoinErr(#[from] tokio::task::JoinError),

    #[error("Undefined error occurred: {0}")]
    Undefined(String),

    #[error("Packet Drop Silent")]
    PacketDropSilent,
}

impl CustomErr {
    pub fn logging(&self) {
        if matches!(self, CustomErr::PacketDropSilent) {
            return;
        }
        println!("{self}");
    }
}
