use std::io::{BufRead, BufReader, Error, ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpStream};
use crate::packet::{Packet};
use crate::packet::r#type::PacketType;
use crate::packet::serialization::deserialize_i32;

/// A client for interfacing with a RCON server
#[derive(Debug)]
pub struct Client {
    packet_idx: i32,
    tcp_stream: TcpStream
}

impl Client {
    /// Creates a new RCON client
    ///
    /// # Arguments
    /// + ip_addr: The IP address of the RCON server
    pub fn new(ip_addr: SocketAddr) -> Result<Self, Error> {
        let tcp_stream: TcpStream;

        match TcpStream::connect(ip_addr) {
            Ok(stream) => tcp_stream = stream,
            Err(err) => return Err(err)
        };

        Ok(Client {packet_idx: 0, tcp_stream})
    }

    /// Creates a new RCON packet with an incremental ID for tracking
    ///
    /// # Arguments:
    /// + type: The RCON packet type
    /// + body: The packet body
    pub fn new_packet(&mut self, r#type: PacketType, body: String) -> Packet {
        let packet = Packet::new(self.packet_idx, r#type, body);
        self.packet_idx += 1;

        packet
    }

    /// Reads a RCON packet from the TCP stream
    pub fn read_packet(&mut self) -> Result<Packet, Error> {
        let mut sz: [u8; 4] = [0; 4];
        match self.tcp_stream.read_exact(&mut sz) {
            Err(err) => return Err(err),
            _ => {}
        };

        let mut data: Vec<u8> = vec![0; deserialize_i32(sz) as usize];
        match self.tcp_stream.read_exact(&mut data) {
            Err(err) => return Err(err),
            _ => {}
        }

        Ok(Packet::from_raw_with_size(sz, data))
    }

    /// Creates a new RCON packet and sends it
    ///
    /// # Arguments:
    /// + type: The RCON packet type
    /// + body: The packet body
    ///
    /// # Returns:
    /// Ok: The packet ID
    /// Err: The error
    pub fn send_packet(&mut self, r#type: PacketType, body: String) -> Result<i32, Error> {
        let packet = self.new_packet(r#type, body);
        match self.tcp_stream.write(&packet.serialize()) {
            Err(err) => return Err(err),
            _ => Ok(packet.id)
        }
    }

    /// Executes a RCON command
    ///
    /// # Arguments:
    /// + body: The packet body
    ///
    /// # Returns:
    /// Ok: The packet ID
    /// Err: The error
    pub fn exec_command(&mut self, body: String) -> Result<i32, Error> {
        self.send_packet(PacketType::SDExecCommandAndAuthResponse, body)
    }

    /// Authenticates with the RCON server
    ///
    /// # Arguments
    /// + password: The password to use for authentication
    ///
    /// # Errors
    /// + Returns a OTHER ErrorKind error when authentication **fails**
    pub fn authenticate(&mut self, password: String) -> Result<(), Error> {
        let packet: Packet = self.new_packet(PacketType::SDAuth, password.clone());

        match self.tcp_stream.write(&packet.serialize()) {
            Err(err) => return Err(err),
            _ => {}
        };

        match self.read_packet() {
            Err(err) => return Err(err),
            _ => {}
        };

        let response_packet = match self.read_packet() {
            Err(err) => return Err(err),
            Ok(packet) => packet
        };

        // Packet ID is -1 if auth fail
        if response_packet.id == -1 {
            return Err(Error::new(ErrorKind::Other, password))
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    use crate::net::Client;
    #[test]
    fn authentication() {
        let mut client = Client::new(
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(127, 0, 0, 1), 5555
            ))
        ).unwrap();

        assert!(client.authenticate("password".into()).is_ok());

    }

    #[test]
    fn send_command() {
        let mut client = Client::new(
            SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(127, 0, 0, 1), 5555
            ))
        ).unwrap();
        client.authenticate("password".into()).unwrap();

        let packet = client.exec_command("echo HLSW: Test".into());
        assert!(packet.is_ok());
        assert_eq!(packet.unwrap(), 1)
    }
}