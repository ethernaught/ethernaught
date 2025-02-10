use crate::packet::ethernet_frame::EthernetFrame;
use crate::packet::ip_header::IpHeader;
use crate::packet::inter::packet::Packet;

#[derive(Debug)]
pub struct UdpHeader {
    ethernet_frame: EthernetFrame,
    ip_header: IpHeader,
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16
}

impl UdpHeader {

    pub fn from_bytes(ethernet_frame: EthernetFrame, ip_header: IpHeader, packet: &[u8]) -> Option<Self> {
        if packet.len() < 8 {
            return None;
        }

        Some(Self {
            ethernet_frame,
            ip_header,
            source_port: u16::from_be_bytes([packet[0], packet[1]]),
            destination_port: u16::from_be_bytes([packet[2], packet[3]]),
            length: u16::from_be_bytes([packet[4], packet[5]]),
            checksum: u16::from_be_bytes([packet[6], packet[7]])
        })
    }
}

impl Packet for UdpHeader {


}
