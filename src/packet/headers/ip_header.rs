use std::net::Ipv4Addr;
use crate::packet::inter::protocols::Protocols;

#[derive(Debug, Clone)]
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

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 20 {
            return None;
        }

        let version_ihl = buf[0];
        let version = version_ihl >> 4;
        let ihl = version_ihl & 0x0F;

        Some(Self {
            version,
            ihl,
            tos: buf[1],
            total_length: u16::from_be_bytes([buf[2], buf[3]]),
            identification: u16::from_be_bytes([buf[4], buf[5]]),
            flags: buf[6] >> 5,
            fragment_offset: u16::from_be_bytes([buf[6] & 0x1F, buf[7]]),
            ttl: buf[8],
            protocol: Protocols::get_protocol_from_code(buf[9]).unwrap(),
            checksum: u16::from_be_bytes([buf[10], buf[11]]),
            source_ip: Ipv4Addr::new(buf[12], buf[13], buf[14], buf[15]),
            destination_ip: Ipv4Addr::new(buf[16], buf[17], buf[18], buf[19]),
        })
    }
}
