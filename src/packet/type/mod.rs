/// An enum containing all the potential packet types as defined by the VDC.
/// * SD = ServerData
///
/// ## Warning
/// SERVERDATA_EXECCOMMAND and SERVERDATA_AUTH_RESPONSE have the same ID
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PacketType {
    SDAuth = 3,
    SDExecCommandAndAuthResponse = 2,
    SDResponseValue = 0
}

impl From<PacketType> for i32 {
    fn from(value: PacketType) -> i32 {
        value as i32
    }
}

impl From<i32> for PacketType {
    fn from(value: i32) -> Self {
        match value {
            3 => PacketType::SDAuth,
            2 => PacketType::SDExecCommandAndAuthResponse,
            _ => PacketType::SDResponseValue
        }
    }
}
