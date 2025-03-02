use pcap::packet::layers::ethernet_frame::ip::ipv4_layer::Ipv4Layer;
use crate::layers::inter::selection::Selection;

impl Selection for Ipv4Layer {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "all" => {
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
                (7, 1)
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
}
