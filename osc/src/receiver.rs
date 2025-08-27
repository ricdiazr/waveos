use std::net::{UdpSocket, SocketAddr};
use std::io;
use super::{AddrPattern, Packet, TypedData, Failure};
use super::MessageNode as OSCMessage;
use std::fmt;

static HASHTAG_BUNDLE: &'static [u8] = &"#bundle".as_bytes();

// binary array of data
#[derive (Clone, Debug)]
pub struct PacketData {
  pub size : u8,
  pub content:Vec<u8>,
}

/// Error types for parsing a binary vector into a PacketData
#[derive(Debug)]
pub enum PacketDataParseError {
    EmptyBuffer,
    SizeMismatch,
}

impl fmt::Display for PacketDataParseError {

  fn fmt(&self, f: &mut fmt::Formatter<'_> ) -> fmt::Result {
    match self {
      PacketDataParseError::EmptyBuffer => write!(f, "no data in received buffer"),
      PacketDataParseError::SizeMismatch => write!(f, "size byte does not match actual slice size"),
    }
  }
}

/// Component: OSC PacketData Receiver
pub fn accept<'a>(osc_packet: PacketData) -> Result<OSCMessage<'a>, Failure> {
    println!("Received OSC packet: {:#?}", osc_packet);
    match from_packet_data_content(osc_packet) {
      Ok(packet) => Ok(OSCMessage {packet: packet}),
      Err(err) => Err(Failure::ReceiverError(err.to_string())),
    }
}

pub fn from_packet_data_content<'a>(osc_packet_data: PacketData) -> Result<Packet<'a>, PacketDataParseError> {
  if HASHTAG_BUNDLE == &osc_packet_data.content[0..HASHTAG_BUNDLE.len()] {
    todo!()
  }
  Ok(Packet::Message {
    address: parse_addr_pattern(&osc_packet_data)?,
    arguments: parse_addr_arguments(&osc_packet_data)?,
  })
}

fn parse_addr_pattern(osc_packet_data: &PacketData) -> Result<AddrPattern, PacketDataParseError> {
  todo!()
}

fn parse_addr_arguments<'a>(osc_packet_data: &PacketData) -> Result<&'a [TypedData<'a>], PacketDataParseError> {
  todo!()
}

/// Function to parse a binary vector into an PacketData
pub fn parse_binary_to_packet(binary_data: Vec<u8>) -> Result<PacketData, PacketDataParseError> {
    // Ensure the binary vector is not empty
    if binary_data.is_empty() {
        return Err(PacketDataParseError::EmptyBuffer);
    }

    // The first byte represents the size of the content
    let size = binary_data[0] as usize;
    // Check if the binary data length matches the declared size
    if binary_data.len() < size + 1 {
        return Err(PacketDataParseError::SizeMismatch);
    }
    // Extract the content (all bytes except the last one)
    let content = binary_data[1..=size].to_vec();

    // Construct and return the PacketData
    Ok(PacketData {
        size: size as u8,
        content: content.to_vec(),
    })
}

pub fn read_socket_data(socket: &UdpSocket, buffer: &mut [u8]) -> Result<usize> {
    // Read data from the socket into the buffer
    let bytes_read = socket.read(buffer)?;

    // Calculate the number of bytes needed to pad the data to a multiple of 32 bits
    let padding_bytes = 4 - (bytes_read % 4);

    // If the number of bytes read is already a multiple of 32 bits, don't pad
    if padding_bytes == 4 {
        return Ok(bytes_read);
    }

    // Pad the data with zeros
    for i in bytes_read..bytes_read + padding_bytes {
        buffer[i] = 0;
    }

    // Return the total number of bytes, including padding
    Ok(bytes_read + padding_bytes)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_receiver() {
        let packet = PacketData {
            content: vec![1, 2, 3],
            size: 3,
        };
        let message = packet_receiver(packet).unwrap();
        assert_eq!(message.0.size, 3);
    }
    #[test]
    fn test_packet_receiver_hex() {
    // Example binary data
    let binary_data = vec![0x01, 0x02, 0x03, 0x03]; // Content: [0x01, 0x02, 0x03], Size: n3

    match parse_binary_to_packet(binary_data) {
        Ok(packet) => println!("Parsed PacketData: {:?}", packet),
        Err(error) => println!("Failed to parse packet: {:?}", error),
    }
    }
}
