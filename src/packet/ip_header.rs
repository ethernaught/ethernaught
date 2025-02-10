use std::net::Ipv4Addr;
use crate::packet::inter::protocols::Protocols;

#[derive(Debug)]
pub struct IpHeader {
    pub version: u8,
    pub ihl: u8,
    pub tos: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: Protocols,
    pub checksum: u16,
    pub source_ip: Ipv4Addr,
    pub destination_ip: Ipv4Addr,
}

impl IpHeader {

    pub fn from_bytes(packet: &[u8]) -> Option<Self> {
        if packet.len() < 20 {
            return None;
        }

        let version_ihl = packet[0];
        let version = version_ihl >> 4;
        let ihl = version_ihl & 0x0F;

        Some(Self {
            version,
            ihl,
            tos: packet[1],
            total_length: u16::from_be_bytes([packet[2], packet[3]]),
            identification: u16::from_be_bytes([packet[4], packet[5]]),
            flags: packet[6] >> 5,
            fragment_offset: u16::from_be_bytes([packet[6] & 0x1F, packet[7]]),
            ttl: packet[8],
            protocol: Protocols::get_protocol_from_code(packet[9]).unwrap(),
            checksum: u16::from_be_bytes([packet[10], packet[11]]),
            source_ip: Ipv4Addr::new(packet[12], packet[13], packet[14], packet[15]),
            destination_ip: Ipv4Addr::new(packet[16], packet[17], packet[18], packet[19]),
        })
    }
}
