use rlibpcap::packet::layers::ip::icmp::icmp_layer::{IcmpLayer, ICMP_HEADER_LEN};
use rlibpcap::packet::layers::ip::icmpv6::icmpv6_layer::Icmpv6Layer;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for Icmpv6Layer {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "type",
            "code",
            "checksum",
            "identifier_be",
            "identifier_le",
            "sequence_number_be",
            "sequence_number_le"
        ]
    }

    fn get_selection(&self, key: &str) -> (usize, usize) {
        match key {
            "frame" => (0, ICMP_HEADER_LEN),
            "type" => (0, 1),
            "code" => (1, 1),
            "checksum" => (2, 2),
            "identifier_be" => (4, 2),
            "identifier_le" => (4, 2),
            "sequence_number_be" => (6, 2),
            "sequence_number_le" => (6, 2),
            _ => unimplemented!()
        }
    }

    fn get_field_name(&self, key: &str) -> String {
        match key {
            "frame" => "icmp",
            "type" => "icmp.type",
            "code" => "icmp.code",
            "checksum" => "icmp.checksum",
            "identifier_be" => "icmp.identifier_be",
            "identifier_le" => "icmp.identifier_le",
            "sequence_number_be" => "icmp.sequence_number_be",
            "sequence_number_le" => "icmp.sequence_number_le",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_title(&self, key: &str) -> String {
        match key {
            "frame" => "Internet Control Message Protocol Version 6",
            "type" => "Type",
            "code" => "Code",
            "checksum" => "Checksum",
            "identifier_be" => "Identifier (BE)",
            "identifier_le" => "Identifier (LE)",
            "sequence_number_be" => "Sequence Number (BE)",
            "sequence_number_le" => "Sequence Number (LE)",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_value(&self, key: &str) -> String {
        match key {
            "type" => format!("{} ({})", self.get_type(), self.get_type().to_string()),
            "code" => self.get_code().to_string(),
            "checksum" => format!("0x{:04X}", self.get_checksum()),
            "identifier_be" => {
                let be = self.get_identifier().to_be();
                format!("{} (0x{:04X})", be, be)
            }
            "identifier_le" => {
                let le = self.get_identifier().to_le();
                format!("{} (0x{:04X})", le, le)
            }
            "sequence_number_be" => {
                let be = self.get_sequence_number().to_be();
                format!("{} (0x{:04X})", be, be)
            }
            "sequence_number_le" => {
                let le = self.get_sequence_number().to_le();
                format!("{} (0x{:04X})", le, le)
            }
            _ => unimplemented!()
        }
    }

    fn get_value_as_bytes(&self, key: &str) -> Vec<u8> {
        match key {
            "frame" => {
                let mut buf = vec![0; ICMP_HEADER_LEN];

                buf[0] = self.get_type();
                buf[1] = self.get_code();
                buf.splice(2..4, self.get_checksum().to_be_bytes());
                buf.splice(4..6, self.get_identifier().to_be_bytes());
                buf.splice(6..8, self.get_sequence_number().to_be_bytes());

                buf
            }
            "type" => vec![self.get_type()],
            "code" => vec![self.get_code()],
            "checksum" => self.get_checksum().to_be_bytes().to_vec(),
            "identifier" => self.get_identifier().to_be_bytes().to_vec(),
            "sequence_number" => self.get_sequence_number().to_be_bytes().to_vec(),
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
