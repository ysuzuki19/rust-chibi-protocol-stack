use std::fmt::Debug;

#[derive(Clone)]
pub struct FlagsFragmentOffset(u16);

impl FlagsFragmentOffset {
    pub fn new(data: [u8; 2]) -> Self {
        Self(u16::from_be_bytes([data[0], data[1]]))
    }

    pub fn value(&self) -> u16 {
        self.0
    }

    fn flags(&self) -> u8 {
        (self.0 >> 13) as u8
    }

    fn fragment_offset(&self) -> u16 {
        self.0 & 0x1fff
    }
}

impl From<FlagsFragmentOffset> for u16 {
    fn from(value: FlagsFragmentOffset) -> Self {
        value.0
    }
}

impl From<u16> for FlagsFragmentOffset {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl Debug for FlagsFragmentOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ Flags: {}, Fragment Offset: {} }}",
            self.flags(),
            self.fragment_offset()
        )
    }
}
