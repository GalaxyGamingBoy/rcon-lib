pub fn serialize_i32(val: i32) -> [u8; 4] {
    val.to_le_bytes()
}

pub fn deserialize_i32(val: [u8; 4]) -> i32 {
    i32::from_le_bytes(val)
}