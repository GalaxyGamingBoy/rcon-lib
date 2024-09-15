use crate::packet::r#type::PacketType;
use crate::packet::raw::RawPacket;

mod raw;
pub(crate) mod serialization;
pub mod r#type;

#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    /// Packet ID
    pub id: i32,
    /// Packet Type
    pub r#type: PacketType,
    /// Packet Body
    pub body: String
}

impl Packet {
    pub fn new(id: i32, r#type: PacketType, body: String) -> Self {
        Self {id, r#type, body}
    }

    pub fn from_raw(packet_data: Vec<u8>) -> Packet {
        RawPacket::deserialize(packet_data).into()
    }

    pub fn from_raw_with_size(packet_size: [u8; 4], packet_data: Vec<u8>) -> Packet {
        RawPacket::deserialize([Vec::from(packet_size), packet_data].concat()).into()
    }

    pub fn serialize(&self) -> Vec<u8> {
        RawPacket::from(self.clone()).serialize()
    }
}
#[cfg(test)]
mod tests {
    use std::fs;
    use crate::packet::{Packet, PacketType};

    fn new_packets() -> [Packet; 3] {
        [
            Packet::new(1, PacketType::SDAuth, "SERVERDATA_AUTH".to_string()),
            Packet::new(2, PacketType::SDExecCommandAndAuthResponse, "SERVERDATA_EXECCOMMAND".to_string()),
            Packet::new(3, PacketType::SDResponseValue, "SERVERDATA_RESPONSE_VALUE".to_string())
        ]
    }

    fn load_packet_test_files() -> [Vec<u8>; 3] {
        let mut test_files: [Vec<u8>; 3] = [vec![], vec![], vec![]];
        for i in 0..3 {
            test_files[i] = fs::read(format!("./test/packet_{i}.bin")).unwrap()
        };

        test_files
    }

    #[test]
    fn assert_equal_packet_serialization() {
        let packets = new_packets();
        let results = load_packet_test_files();

        for i in 0..3 {
            print!("Testing packet {}/3...", i + 1);
            assert_eq!(packets[i].serialize(), results[i]);
            println!("OK!")
        }
    }

    #[test]
    fn assert_equal_packet_deserialization() {
        let packets = new_packets();
        let results = load_packet_test_files();

        for i in 0..3 {
            print!("Testing packet {}/3...", i + 1);
            assert_eq!(Packet::from_raw(results[i].clone()), packets[i]);
            println!("OK!")
        }
    }
}