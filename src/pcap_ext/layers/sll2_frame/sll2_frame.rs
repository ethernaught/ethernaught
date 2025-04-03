use rlibpcap::packet::layers::sll2_frame::sll2_frame::{Sll2Frame, SLL2_FRAME_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::views::dropdown::dropdown::create_row;

impl LayerExt for Sll2Frame {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "protocol",
            //"reserved",
            "if_index",
            "data_link_type",
            "packet_type",
            "address_length",
            "address",
            "unused"
        ]
    }

    fn get_selection(&self, key: &str) -> Option<(usize, usize)> {
        Some(match key {
            "frame" => (0, SLL2_FRAME_LEN),
            "protocol" => (0, 2),
            //"reserved" => (2, 2),
            "if_index" => (4, 4),
            "data_link_type" => (8, 2),
            "packet_type" => (10, 1),
            "address_length" => (11, 1),
            "address" => (12, self.get_address_length() as usize),
            "unused" => (12 + self.get_address_length() as usize, 8 - self.get_address_length() as usize),
            _ => return None
        })
    }

    fn get_field_name(&self, key: &str) -> Option<String> {
        Some(match key {
            "frame" => "sll2",
            "protocol" => "sll2.protocol",
            //"reserved" => "sll2.reserved",
            "if_index" => "sll2.if_index",
            "data_link_type" => "sll2.data_link_type",
            "packet_type" => "sll2.packet_type",
            "address_length" => "sll2.address_length",
            "address" => "sll2.address",
            "unused" => "sll2.unused",
            _ => return None
        }.to_string())
    }

    fn get_title(&self, key: &str) -> Option<String> {
        Some(match key {
            "frame" => "Linux Cooked Capture v2",
            "protocol" => "Protocol",
            //"reserved" => "",
            "if_index" => "Interface Index",
            "data_link_type" => "Link-Layer Address Type",
            "packet_type" => "Packet Type",
            "address_length" => "Link-Layer Address Length",
            "address" => "Source",
            "unused" => "Unused",
            _ => unimplemented!()
        }.to_string())
    }

    fn get_value(&self, key: &str) -> Option<String> {
        Some(match key {
            "protocol" => format!("{} (0x{:04X})", self.get_protocol().to_string(), self.get_protocol().get_code()),
            //"reserved" => ,
            "if_index" => self.get_if_index().to_string(),
            "data_link_type" => format!("{} ({})", self.get_data_link_type().to_string(), self.get_data_link_type().get_code()),
            "packet_type" => format!("{} ({})", self.get_packet_type().to_string(), self.get_packet_type().get_code()),
            "address_length" => self.get_address_length().to_string(),
            "address" => {
                self.get_address().iter()
                    .take(self.get_address_length() as usize)
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .join(":")
            },
            "unused" => {
                self.get_address().iter()
                    .skip(self.get_address_length() as usize)
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<String>>()
                    .concat()
            }
            _ => return None
        })
    }

    fn get_value_as_bytes(&self, key: &str) -> Option<Vec<u8>> {
        Some(match key {
            "frame" => {
                let mut buf = vec![0; SLL2_FRAME_LEN];

                buf.splice(0..2, self.get_protocol().get_code().to_be_bytes());
                buf.splice(4..8, self.get_if_index().to_be_bytes());
                buf.splice(8..10, self.get_data_link_type().get_code().to_be_bytes());
                buf[10] = self.get_packet_type().get_code();
                buf[11] = self.get_address_length();
                buf.splice(12..20, self.get_address().clone());

                buf
            }
            "protocol" => self.get_protocol().get_code().to_be_bytes().to_vec(),
            //"reserved" => ,
            "if_index" => self.get_if_index().to_be_bytes().to_vec(),
            "data_link_type" => self.get_data_link_type().get_code().to_be_bytes().to_vec(),
            "packet_type" => vec![self.get_packet_type().get_code()],
            "address_length" => vec![self.get_address_length()],
            "address" => self.get_address()[..self.get_address_length() as usize].to_vec(),
            "unused" => self.get_address()[self.get_address_length() as usize..].to_vec(),
            _ => return None
        })
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn clone_ext(&self) -> Box<dyn LayerExt> {
        Box::new(self.clone())
    }
}
