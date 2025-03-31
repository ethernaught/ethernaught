use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::{ArpExtension, ARP_HEADER_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for ArpExtension {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "hardware_type",
            "protocol_type",
            "hardware_size",
            "protocol_size",
            "opcode",
            "sender_mac",
            "sender_address",
            "target_mac",
            "target_address"
        ]
    }

    fn get_selection(&self, key: &str) -> (usize, usize) {
        match key {
            "frame" => (0, 28),
            "hardware_type" => (0, 2),
            "protocol_type" => (2, 2),
            "hardware_size" => (4, 1),
            "protocol_size" => (5, 1),
            "opcode" => (6, 2),
            "sender_mac" => (8, 6),
            "sender_address" => (14, 4),
            "target_mac" => (18, 6),
            "target_address" => (24, 4),
            _ => unimplemented!()
        }
    }

    fn get_field_name(&self, key: &str) -> String {
        match key {
            "frame" => "arp",
            "hardware_type" => "arp.hardware_type",
            "protocol_type" => "arp.protocol_type",
            "hardware_size" => "arp.hardware_size",
            "protocol_size" => "arp.protocol_size",
            "opcode" => "arp.opcode",
            "sender_mac" => "arp.sender_mac",
            "sender_address" => "arp.sender_address",
            "target_mac" => "arp.target_mac",
            "target_address" => "arp.target_address",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_title(&self, key: &str) -> String {
        match key {
            "frame" => "Address Resolution Protocol",
            "hardware_type" => "Hardware Type",
            "protocol_type" => "Protocol Type",
            "hardware_size" => "Hardware Size",
            "protocol_size" => "Protocol Size",
            "opcode" => "Opcode",
            "sender_mac" => "Sender MAC Address",
            "sender_address" => "Sender IP Address",
            "target_mac" => "Target MAC Address",
            "target_address" => "Target IP Address",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_value(&self, key: &str) -> String {
        match key {
            "hardware_type" => format!("{} ({})", self.get_hardware_type().to_string(), self.get_hardware_type()),
            "protocol_type" => self.get_protocol_type().to_string(),
            "hardware_size" => self.get_hardware_size().to_string(),
            "protocol_size" => self.get_protocol_size().to_string(),
            "opcode" => format!("{} ({})", self.get_opcode().to_string(), self.get_opcode().get_code()),
            "sender_mac" => self.get_sender_mac().to_string(),
            "sender_address" => self.get_sender_address().to_string(),
            "target_mac" => self.get_target_mac().to_string(),
            "target_address" => self.get_target_address().to_string(),
            _ => unimplemented!()
        }
    }

    fn get_value_as_bytes(&self, key: &str) -> Vec<u8> {
        match key {
            "frame" => {
                let mut buf = vec![0; ARP_HEADER_LEN];

                buf.splice(0..2, self.get_hardware_type().to_be_bytes());
                buf.splice(2..4, self.get_protocol_type().get_code().to_be_bytes());
                buf[4] = self.get_hardware_size();
                buf[5] = self.get_protocol_size();
                buf.splice(6..8, self.get_opcode().get_code().to_be_bytes());
                buf.splice(8..14, self.get_sender_mac().to_bytes());
                buf.splice(14..18, self.get_sender_address().octets());
                buf.splice(18..24, self.get_target_mac().to_bytes());
                buf.splice(24..28, self.get_target_address().octets());

                buf
            }
            "hardware_type" => self.get_hardware_type().to_be_bytes().to_vec(),
            "protocol_type" => self.get_protocol_type().get_code().to_be_bytes().to_vec(),
            "hardware_size" => vec![self.get_hardware_size()],
            "protocol_size" => vec![self.get_protocol_size()],
            "opcode" => self.get_opcode().get_code().to_be_bytes().to_vec(),
            "sender_mac" => self.get_sender_mac().to_bytes().to_vec(),
            "sender_address" => self.get_sender_address().octets().to_vec(),
            "target_mac" => self.get_target_mac().to_bytes().to_vec(),
            "target_address" => self.get_target_address().octets().to_vec(),
            _ => unimplemented!()
        }
    }

    fn to_string(&self) -> String {
        todo!()
    }

    fn clone_ext(&self) -> Box<dyn LayerExt> {
        Box::new(self.clone())
    }
}
