mod addr;
pub mod header;
pub mod packet;
pub mod payload;
pub mod protocol;

use crate::error::CustomRes;

pub type Buf = Vec<u8>;

pub trait Serialize {
    fn serialize(self, buf: &mut Vec<u8>);
}

pub trait Deserialize {
    fn deserialize(buf: &[u8]) -> CustomRes<Self>
    where
        Self: std::marker::Sized;
}
