use pcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use crate::layers::inter::layer_ext::LayerExt;

impl LayerExt for EthernetFrame {

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

    fn to_string(&self) -> String {
        todo!()
    }
}
