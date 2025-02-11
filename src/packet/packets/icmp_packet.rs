use std::any::Any;
use crate::packet::headers::ethernet_frame::EthernetFrame;
use crate::packet::headers::icmp_header::IcmpHeader;
use crate::packet::headers::ipv4_header::Ipv4Header;
use crate::packet::inter::types::Types;
use crate::packet::packets::inter::packet_base::Packet;

#[derive(Clone)]
pub struct IcmpPacket {
    ethernet_frame: EthernetFrame,
    ip_header: Ipv4Header,
    icmp_header: IcmpHeader,
    frame_time: u32,
    frame_length: usize,
    payload: Vec<u8>
}

impl IcmpPacket {

    pub fn from_bytes(ethernet_frame: EthernetFrame, ip_header: Ipv4Header, icmp_header: IcmpHeader, frame_time: u32, frame_length: usize, buf: &[u8]) -> Option<Self> {
        Some(Self {
            ethernet_frame,
            ip_header,
            icmp_header,
            frame_time,
            frame_length,
            payload: buf.to_vec()
        })
    }

    pub fn get_ip_header(&self) -> &Ipv4Header {
        &self.ip_header
    }
}

impl Packet for IcmpPacket {

    fn get_ethernet_frame(&self) -> &EthernetFrame {
        &self.ethernet_frame
    }

    fn get_type(&self) -> Types {
        Types::Icmp
    }

    fn get_data(&self) -> Vec<u8> {
        self.payload.clone()
    }

    fn len(&self) -> usize {
        self.frame_length
    }

    fn get_frame_time(&self) -> u32 {
        self.frame_time
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
