use std::fmt::Debug;

use num_enum::FromPrimitive;

#[derive(Debug, FromPrimitive, Clone)]
#[repr(u8)]
pub enum Protocol {
    Icmp = 1,
    Tcp = 6,
    Udp = 17,
    #[num_enum(default)]
    Unknown,
}
