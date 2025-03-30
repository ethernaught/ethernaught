use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::{EthernetFrame, ETHERNET_FRAME_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for EthernetFrame {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "frame" => {
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

    fn get_field_name(&self, variable: &str) -> String {
        match variable {
            "frame" => {
                "ethernet"
            }
            "destination" => {
                "ethernet.destination"
            }
            "source" => {
                "ethernet.source"
            }
            "type" => {
                "ethernet.type"
            }
            _ => unimplemented!()
        }.to_string()
    }

    fn get_value(&self, variable: &str) -> String {
        match variable {
            "frame" => {
                String::from("Ethernet")
            }
            "destination" => {
                self.get_destination_mac().to_string()
            }
            "source" => {
                self.get_source_mac().to_string()
            }
            "type" => {
                format!("{:?} (0x{:04X})", self.get_type(), self.get_type().get_code())
            }
            _ => unimplemented!()
        }
    }

    fn get_description(&self, variable: &str) -> String {
        match variable {
            "frame" => {
                self.get_value(variable)
            }
            "destination" => {
                format!("Destination: {}", self.get_value(variable))
            }
            "source" => {
                format!("Source: {}", self.get_value(variable))
            }
            "type" => {
                format!("Type: {}", self.get_value(variable))
            }
            _ => unimplemented!()
        }
    }

    fn get_value_as_bytes(&self, variable: &str) -> Vec<u8> {
        match variable {
            "frame" => {
                let mut buf = vec![0; ETHERNET_FRAME_LEN];
                buf.splice(0..6, self.get_destination_mac().to_bytes());
                buf.splice(6..12, self.get_source_mac().to_bytes());
                buf.splice(12..14, self.get_type().get_code().to_be_bytes());
                buf
            }
            "destination" => {
                self.get_destination_mac().to_bytes().to_vec()
            }
            "source" => {
                self.get_source_mac().to_bytes().to_vec()
            }
            "type" => {
                self.get_type().get_code().to_be_bytes().to_vec()
            }
            _ => unimplemented!()
        }
    }

    fn to_string(&self) -> String {
        todo!()
    }
}
