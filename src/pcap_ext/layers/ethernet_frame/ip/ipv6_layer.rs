use pcap::packet::layers::ip::ipv6_layer::Ipv6Layer;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for Ipv6Layer {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "frame" => {
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

    fn get_field_name(&self, variable: &str) -> String {
        todo!()
    }

    fn get_value(&self, variable: &str) -> String {
        todo!()
    }

    fn get_description(&self, variable: &str) -> String {
        todo!()
    }

    fn get_value_as_bytes(&self, variable: &str) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}
