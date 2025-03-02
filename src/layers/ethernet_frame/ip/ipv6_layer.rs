use pcap::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::layers::inter::selection::Selection;

impl Selection for Ipv6Layer {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "all" => {
                (0, 40)
            }
            "version" => {
                (0, 1)
            }
            "traffic_class" => {
                (1, 1)
            }
            "flow_label" => {
                (1, 4)
            }
            "payload_length" => {
                (4, 2)
            }
            "next_header" => {
                (6, 1)
            }
            "hop_limit" => {
                (7, 1)
            }
            "source_address" => {
                (8, 16)
            }
            "destination_address" => {
                (24, 16)
            }
            _ => unimplemented!()
        }
    }
}
