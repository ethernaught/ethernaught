use crate::packet::headers::ipv4_header::Ipv4Header;
use crate::packet::packets::inter::packet_base::PacketBase;

pub trait UdpPacketBase: PacketBase {

    fn get_ip_header(&self) -> &Ipv4Header;
}
