use bitflags::bitflags;

#[derive(Debug)]
pub(super) struct DataOffsetReserved(u8);
impl DataOffsetReserved {
    pub fn new(data_offset: u8) -> Self {
        DataOffsetReserved(data_offset)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
    pub fn data_offset(&self) -> u8 {
        (self.0 >> 4) * 4
    }
}

bitflags! {
    #[derive(Debug)]
    pub(super) struct TcpFlags: u8 {
        const FIN = 0b0000_0001;
        const SYN = 0b0000_0010;
        const RST = 0b0000_0100;
        const PSH = 0b0000_1000;
        const ACK = 0b0001_0000;
        const URG = 0b0010_0000;
        const ECE = 0b0100_0000;
        const CWR = 0b1000_0000;
    }
}
