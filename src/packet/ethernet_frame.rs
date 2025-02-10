use crate::packet::inter::types::Types;

#[derive(Debug)]
pub struct EthernetFrame {
    pub destination: [u8; 6],
    pub source: [u8; 6],
    pub _type: Types
}

impl EthernetFrame {
    pub fn from_bytes(packet: &[u8]) -> Option<Self> {
        if packet.len() < 14 {
            return None;
        }

        Some(EthernetFrame {
            destination: [packet[0], packet[1], packet[2], packet[3], packet[4], packet[5]],
            source: [packet[6], packet[7], packet[8], packet[9], packet[10], packet[11]],
            _type: Types::get_type_from_code(u16::from_be_bytes([packet[12], packet[13]])).unwrap()
        })
    }
}
