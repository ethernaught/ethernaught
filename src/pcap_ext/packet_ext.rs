use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::packet::Packet;
use rlibpcap::utils::data_link_types::DataLinkTypes;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::pcap_ext::packet_query::PacketQuery;

pub trait PacketExt {

    fn matches(&self, query: &Vec<PacketQuery>) -> bool;
}

impl PacketExt for Packet {

    fn matches(&self, query: &Vec<PacketQuery>) -> bool {
        let query = query.get(0).unwrap();

        match self.get_data_link_type() {
            DataLinkTypes::En10mb => {
                let layer = self.get_frame::<EthernetFrame>();

            }
            DataLinkTypes::Raw => {}
            DataLinkTypes::Loop => {}
            DataLinkTypes::Sll2 => {}
            _ => {}
        }

        false
    }
}

pub fn match_layer(query: &PacketQuery, layer: &dyn LayerExt) -> bool {
    if !layer.get_field_name("frame").eq(&query.layer) {
        return false;
    }

    match query.field {
        Some(ref field) => {
            if layer.get_value(&field.name).eq(&field.value) {
                return true;
            }

            false
        }
        None => false
    }
}

pub fn match_ipv4_layer(query: &PacketQuery) -> bool {
    false
}
