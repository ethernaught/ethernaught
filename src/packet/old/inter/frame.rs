use std::any::Any;
use std::net::Ipv4Addr;
use crate::packet::inter::ethernet_types::EthernetTypes;
use crate::packet::inter::protocols::Protocols;


pub struct IPv4Layer {
    version: u8,
    ihl: u8,
    tos: u8,
    total_length: u16,
    identification: u16,
    flags: u8,
    fragment_offset: u16,
    ttl: u8,
    protocol: Protocols,
    checksum: u16,
    source_ip: Ipv4Addr,
    destination_ip: Ipv4Addr
}

impl IPv4Layer {

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
            destination_ip: Ipv4Addr::new(buf[16], buf[17], buf[18], buf[19])
        })
    }
}

impl Layer for IPv4Layer {

    fn get_layer_name(&self) -> &str {
        "IPV4_HEADER"
    }

    fn len(&self) -> usize {
        20
    }

    fn get_type(&self) -> String {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Layer> {
        Box::new(self.clone())
    }
}
