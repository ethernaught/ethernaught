use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use rlibpcap::packet::layers::ethernet_frame::llc::llc_extension::LlcExtension;
use rlibpcap::packet::layers::ip::icmp::icmp_layer::IcmpLayer;
use rlibpcap::packet::layers::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use rlibpcap::packet::layers::ip::inter::ip_protocols::IpProtocols;
use rlibpcap::packet::layers::ip::inter::ip_versions::IpVersions;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use rlibpcap::packet::layers::ip::ipv6_layer::Ipv6Layer;
use rlibpcap::packet::layers::ip::tcp::tcp_layer::TcpLayer;
use rlibpcap::packet::layers::ip::udp::udp_layer::UdpLayer;
use rlibpcap::packet::layers::loop_frame::inter::loop_types::LoopTypes;
use rlibpcap::packet::layers::loop_frame::loop_frame::LoopFrame;
use rlibpcap::packet::layers::raw_frame::raw_frame::RawFrame;
use rlibpcap::packet::layers::sll2_frame::sll2_frame::Sll2Frame;
use rlibpcap::packet::packet::Packet;
use rlibpcap::utils::data_link_types::DataLinkTypes;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::pcap_ext::packet_query::PacketQuery;

pub trait PacketExt {

    fn matches(&self, query: &Vec<Vec<PacketQuery>>) -> bool;
}

impl PacketExt for Packet {

    fn matches(&self, query: &Vec<Vec<PacketQuery>>) -> bool {
        let mut layers: Vec<&dyn LayerExt> = Vec::new();

        match self.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let layer = self.get_frame::<EthernetFrame>();
                layers.push(layer);

                match layer.get_type() {
                    EthernetTypes::Ipv4 => match_ipv4_layer(&mut layers, layer.get_data::<Ipv4Layer>().unwrap()),
                    EthernetTypes::Arp => layers.push(layer.get_data::<ArpExtension>().unwrap()),
                    EthernetTypes::Ipv6 => match_ipv6_layer(&mut layers, layer.get_data::<Ipv6Layer>().unwrap()),
                    EthernetTypes::Broadcast => {}
                    EthernetTypes::Length(_) => layers.push(layer.get_data::<LlcExtension>().unwrap())
                }
            }
            DataLinkTypes::Sll2 => {
                let layer = self.get_frame::<Sll2Frame>();
                layers.push(layer);

                match layer.get_protocol() {
                    EthernetTypes::Ipv4 => match_ipv4_layer(&mut layers, layer.get_data::<Ipv4Layer>().unwrap()),
                    EthernetTypes::Arp => layers.push(layer.get_data::<ArpExtension>().unwrap()),
                    EthernetTypes::Ipv6 => match_ipv6_layer(&mut layers, layer.get_data::<Ipv6Layer>().unwrap()),
                    EthernetTypes::Broadcast => {}
                    EthernetTypes::Length(_) => layers.push(layer.get_data::<LlcExtension>().unwrap())
                }
            }
            DataLinkTypes::Raw => {
                let layer = self.get_frame::<RawFrame>();

                match layer.get_version() {
                    IpVersions::Ipv4 => match_ipv4_layer(&mut layers, layer.get_data::<Ipv4Layer>().unwrap()),
                    IpVersions::Ipv6 => match_ipv6_layer(&mut layers, layer.get_data::<Ipv6Layer>().unwrap())
                }
            }
            DataLinkTypes::Loop => {
                let layer = self.get_frame::<LoopFrame>();

                match layer.get_type() {
                    LoopTypes::Ipv4 => match_ipv4_layer(&mut layers, layer.get_data::<Ipv4Layer>().unwrap()),
                    LoopTypes::Ipv6 | LoopTypes::Ipv6e2 | LoopTypes::Ipv6e3 => match_ipv6_layer(&mut layers, layer.get_data::<Ipv6Layer>().unwrap()),
                    _ => {
                        unimplemented!()
                    }
                }
            }
            _ => {}
        };

        for or_group in query {
            let mut and_conditions_met = vec![false; or_group.len()];

            for (i, pq) in or_group.iter().enumerate() {
                for layer in &layers {
                    if match_layer(pq, *layer) {
                        and_conditions_met[i] = true;
                        break;
                    }
                }
            }

            if and_conditions_met.iter().all(|&met| met) {
                return true;
            }
        }

        false
    }
}

pub fn match_layer(query: &PacketQuery, layer: &dyn LayerExt) -> bool {
    if !layer.get_field_name("frame").unwrap().eq(&query.layer) {
        return false;
    }

    match query.field {
        Some(ref field) => {
            match layer.get_value(&field.name) {
                Some(value) => {
                    if value.eq(&field.value) {
                        return true;
                    }

                    false
                }
                None => false
            }
        }
        None => true
    }
}

pub fn match_ipv4_layer<'a>(layers: &mut Vec<&'a dyn LayerExt>, layer: &'a Ipv4Layer) {
    layers.push(layer);

    match layer.get_protocol() {
        IpProtocols::HopByHop => {}
        IpProtocols::Icmp => layers.push(layer.get_data::<IcmpLayer>().unwrap()),
        IpProtocols::Igmp => {}
        IpProtocols::Tcp => layers.push(layer.get_data::<TcpLayer>().unwrap()),
        IpProtocols::Udp => layers.push(layer.get_data::<UdpLayer>().unwrap()),
        IpProtocols::Ipv6 => {}
        IpProtocols::Gre => {}
        IpProtocols::Icmpv6 => {}
        IpProtocols::Ospf => {}
        IpProtocols::Sps => {}
    }
}

pub fn match_ipv6_layer<'a>(layers: &mut Vec<&'a dyn LayerExt>, layer: &'a Ipv6Layer) {
    layers.push(layer);

    match layer.get_next_header() {
        IpProtocols::HopByHop => {}
        IpProtocols::Icmp => {}
        IpProtocols::Igmp => {}
        IpProtocols::Tcp => layers.push(layer.get_data::<TcpLayer>().unwrap()),
        IpProtocols::Udp => layers.push(layer.get_data::<UdpLayer>().unwrap()),
        IpProtocols::Ipv6 => {}
        IpProtocols::Gre => {}
        IpProtocols::Icmpv6 => layers.push(layer.get_data::<Icmpv6Layer>().unwrap()),
        IpProtocols::Ospf => {}
        IpProtocols::Sps => {}
    }
}
