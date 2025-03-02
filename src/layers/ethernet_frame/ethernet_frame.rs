use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use crate::layers::inter::selection::Selection;

impl Selection for EthernetFrame {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "all" => {
                (0, 14)
            }
            "destination" => {
                (0, 6)
            }
            "source" => {
                (6, 6)
            }
            "type" => {
                (12, 2)
            }
            _ => unimplemented!()
        }
    }
}
