use crate::packet::Packet;
use crate::packet::serialization::{deserialize_i32, serialize_i32};

/// The raw TCP packet
#[derive(Debug, Clone, PartialEq)]
pub struct RawPacket {
    sz: i32,
    id: i32,
    r#type: i32,
    body: Vec<u8>,
}

impl From<RawPacket> for Packet {
    fn from(value: RawPacket) -> Self {
        Self {
            id: value.id,
            r#type: value.r#type.into(),
            body: String::from_utf8(value.body).unwrap()
        }
    }
}

impl RawPacket {
    pub fn from(packet: Packet) -> RawPacket {
        RawPacket {
            sz: (packet.body.len() + 4 + 4 + 1 + 1) as i32,
            id: packet.id,
            r#type: packet.r#type as i32,
            body: packet.body.as_bytes().to_vec(),
        }
    }

    pub fn deserialize(packet: Vec<u8>) -> RawPacket {
        let sz = deserialize_i32(packet[0..4].try_into().unwrap());
        let id = deserialize_i32(packet[4..8].try_into().unwrap());
        let r#type = deserialize_i32(packet[8..12].try_into().unwrap());
        let body: Vec<u8> = packet[12..(packet.len() - 2)].to_vec();

        RawPacket {sz, id, r#type, body}
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = [
            serialize_i32(self.sz),
            serialize_i32(self.id),
            serialize_i32(self.r#type),
        ].concat();

        packet.extend(&self.body[..]);
        packet.extend([0x00, 0x00]);

        packet
    }
}
