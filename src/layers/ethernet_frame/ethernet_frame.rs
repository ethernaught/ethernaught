use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use crate::layers::inter::selection::Selection;

impl Selection for EthernetFrame {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "destination" => {
                (0, 6)
            }
            "source" => {
                (6, 12)
            }
            "type" => {
                (12, 14)
            }
            _ => unimplemented!()
        }
    }
}
