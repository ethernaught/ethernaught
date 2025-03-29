use pcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for Ipv4Layer {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "frame" => {
                (0, 20)
            }
            "version" => {
                (0, 1)
            }
            "tos" => {
                (1, 1)
            }
            "total_length" => {
                (2, 2)
            }
            "identification" => {
                (4, 2)
            }
            "flags" => {
                (6, 1)
            }
            "fragment_offset" => {
                (6, 2)
            }
            "ttl" => {
                (8, 1)
            }
            "protocol" => {
                (9, 1)
            }
            "checksum" => {
                (10, 2)
            }
            "source_address" => {
                (12, 4)
            }
            "destination_address" => {
                (16, 4)
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
