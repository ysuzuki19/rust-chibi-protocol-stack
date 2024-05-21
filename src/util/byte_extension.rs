pub trait DivideBytes<T> {
    fn divide_bytes(&self) -> (T, T);
}

impl DivideBytes<u16> for u32 {
    fn divide_bytes(&self) -> (u16, u16) {
        let high = (*self >> 16) as u16;
        let low = (*self & 0xffff) as u16;
        (high, low)
    }
}
