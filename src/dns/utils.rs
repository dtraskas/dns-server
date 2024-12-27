pub trait ToBigEndian {
    fn to_big_endian(&self) -> Vec<u8>;
}

// Implement the trait for `u16`.
impl ToBigEndian for u16 {
    fn to_big_endian(&self) -> Vec<u8> {
        vec![
            (*self >> 8) as u8,    // Most significant byte
            (*self & 0xFF) as u8,   // Least significant byte
        ]
    }
}

// Implement the trait for `u32`.
impl ToBigEndian for u32 {
    fn to_big_endian(&self) -> Vec<u8> {
        vec![
            (*self >> 24) as u8,    // Most significant byte
            (*self >> 16) as u8,    // Second byte
            (*self >> 8) as u8,     // Third byte
            (*self & 0xFF) as u8,   // Least significant byte
        ]
    }
}