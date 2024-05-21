use std::fmt::Debug;

#[derive(Clone)]
pub struct VersionIhl(u8);

const IP_VERSION_IPV4: u8 = 4;
impl VersionIhl {
    pub fn new(data: u8) -> Self {
        Self(data)
    }

    pub fn version(&self) -> u8 {
        self.0 >> 4
    }

    pub fn is_supported(&self) -> bool {
        VersionIhl::new(self.0).version() == IP_VERSION_IPV4
    }

    fn ihl(&self) -> u8 {
        self.0 & 0x0f
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Debug for VersionIhl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Version: {}, IHL: {} }}", self.version(), self.ihl())
    }
}
