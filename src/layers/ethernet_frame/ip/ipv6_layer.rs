use pcap::packet::layers::ethernet_frame::ip::ipv6_layer::Ipv6Layer;
use crate::layers::inter::selection::Selection;

impl Selection for Ipv6Layer {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "all" => {
                (0, 40)
            }
            _ => unimplemented!()
        }
    }
}
