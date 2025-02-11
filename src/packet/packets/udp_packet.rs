use std::any::Any;
use crate::packet::headers::ethernet_frame::EthernetFrame;
use crate::packet::headers::ip_header::IpHeader;
use crate::packet::packets::inter::packet::Packet;

#[derive(Clone)]
pub struct UdpPacket {
    ethernet_frame: EthernetFrame,
    ip_header: IpHeader,
    source_port: u16,
    destination_port: u16,
    length: u16,
    checksum: u16
}

impl UdpPacket {

    pub fn from_bytes(ethernet_frame: EthernetFrame, ip_header: IpHeader, buf: &[u8]) -> Option<Self> {
        if buf.len() < 8 {
            return None;
        }

        Some(Self {
            ethernet_frame,
            ip_header,
            source_port: u16::from_be_bytes([buf[0], buf[1]]),
            destination_port: u16::from_be_bytes([buf[2], buf[3]]),
            length: u16::from_be_bytes([buf[4], buf[5]]),
            checksum: u16::from_be_bytes([buf[6], buf[7]])
        })
    }

    pub fn get_ip_header(&self) -> &IpHeader {
        &self.ip_header
    }
}

impl Packet for UdpPacket {

    fn get_ethernet_frame(&self) -> &EthernetFrame {
        &self.ethernet_frame
    }

    fn get_data(&self) -> Vec<u8> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn upcast(&self) -> &dyn Packet {
        self
    }

    fn upcast_mut(&mut self) -> &mut dyn Packet {
        self
    }

    fn dyn_clone(&self) -> Box<dyn Packet> {
        Box::new(self.clone())
    }
}
