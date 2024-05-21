use super::byte_extension::DivideBytes;

pub struct Checksum {
    sum: u32,
}

impl Checksum {
    pub fn new() -> Self {
        Self { sum: 0 }
    }

    pub fn u8(self, nums: (u8, u8)) -> Self {
        self.u16(u16::from_be_bytes([nums.0, nums.1]))
    }

    pub fn u16(mut self, num: u16) -> Self {
        self.sum += num as u32;
        self
    }

    pub fn u32(self, num: u32) -> Self {
        let (high, low) = num.divide_bytes();
        self.u16(high).u16(low)
    }

    pub fn vec(self, vec: &[u8]) -> Self {
        let mut cs = self;

        let chunks = vec.chunks_exact(2);
        for chunk in chunks.clone() {
            cs = cs.u8((chunk[0], chunk[1]));
        }
        if let Some(last) = chunks.remainder().first() {
            cs = cs.u8((*last, 0));
        }

        cs
    }

    fn handle_overflow(&mut self) {
        while (self.sum >> 16) != 0 {
            let (high, low) = self.sum.divide_bytes();
            self.sum = high as u32 + low as u32;
        }
    }

    pub fn export(mut self) -> u16 {
        self.handle_overflow();
        !self.sum as u16
    }
}
