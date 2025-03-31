use rlibpcap::packet::layers::ip::ipv6_layer::{Ipv6Layer, IPV6_HEADER_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for Ipv6Layer {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "version",
            "payload_length",
            "next_header",
            "hop_limit",
            "source_address",
            "destination_address",
        ]
    }

    fn get_selection(&self, key: &str) -> (usize, usize) {
        match key {
            "frame" => (0, IPV6_HEADER_LEN),
            "version" => (0, 1),
            "traffic_class" => (1, 1),
            "flow_label" => (1, 4),
            "payload_length" => (4, 2),
            "next_header" => (6, 1),
            "hop_limit" => (7, 1),
            "source_address" => (8, 16),
            "destination_address" => (24, 16),
            _ => unimplemented!()
        }
    }

    fn get_field_name(&self, key: &str) -> String {
        match key {
            "frame" => "ipv6",
            "version" => "ipv6.version",
            "traffic_class" => "ipv6.traffic_class",
            "flow_label" => "ipv6.flow_label",
            "payload_length" => "ipv6.payload_length",
            "next_header" => "ipv6.next_header",
            "hop_limit" => "ipv6.hop_limit",
            "source_address" => "ipv6.source",
            "destination_address" => "ipv6.destination",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_title(&self, key: &str) -> String {
        match key {
            "frame" => "Internet Protocol Version 6",
            "version" => "Version",
            "traffic_class" => todo!(),
            "flow_label" => todo!(),
            "payload_length" => "Payload Length",
            "next_header" => "Next Header",
            "hop_limit" => "Hop Limit",
            "source_address" => "Source Address",
            "destination_address" => "Destination Address",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_value(&self, key: &str) -> String {
        match key {
            "version" => format!("{} ({})", self.get_version().to_string(), self.get_version().get_code()),
            "traffic_class" => self.get_traffic_class().to_string(),
            "flow_label" => self.get_flow_label().to_string(),
            "payload_length" => self.get_payload_length().to_string(),
            "next_header" => format!("{} ({})", self.get_next_header().to_string(), self.get_next_header().get_code()),
            "hop_limit" => self.get_hop_limit().to_string(),
            "source_address" => self.get_source_address().to_string(),
            "destination_address" => self.get_destination_address().to_string(),
            _ => unimplemented!()
        }
    }

    fn get_value_as_bytes(&self, key: &str) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn clone_ext(&self) -> Box<dyn LayerExt> {
        Box::new(self.clone())
    }
}
