use rlibpcap::packet::layers::ip::ipv4_layer::{Ipv4Layer, IPV4_HEADER_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for Ipv4Layer {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "version",
            //"ihl",
            "tos",
            "total_length",
            //"flags",
            "identification",
            //"fragment_offset",
            "ttl",
            "protocol",
            "checksum",
            "source_address",
            "destination_address"
        ]
    }

    fn get_selection(&self, key: &str) -> Option<(usize, usize)> {
        Some(match key {
            "frame" => (0, IPV4_HEADER_LEN),
            "version" => (0, 1),
            "tos" => (1, 1),
            "total_length" => (2, 2),
            "identification" => (4, 2),
            "flags" => (6, 1),
            "fragment_offset" => (6, 2),
            "ttl" => (8, 1),
            "protocol" => (9, 1),
            "checksum" => (10, 2),
            "source_address" => (12, 4),
            "destination_address" => (16, 4),
            _ => return None
        })
    }

    fn get_field_name(&self, key: &str) -> Option<String> {
        Some(match key {
            "frame" => "ipv4",
            "version" => "ipv4.version",
            "ihl" => "ipv4.ihl",
            "tos" => "ipv4.tos",
            "total_length" => "ipv4.total_length",
            "identification" => "ipv4.identification",
            "flags" => "ipv4.flags",
            "fragment_offset" => "ipv4.offset",
            "ttl" => "ipv4.ttl",
            "protocol" => "ipv4.protocol",
            "checksum" => "ipv4.checksum",
            "source_address" => "ipv4.source_address",
            "destination_address" => "ipv4.destination_address",
            _ => return None
        }.to_string())
    }

    fn get_title(&self, key: &str) -> Option<String> {
        Some(match key {
            "frame" => "Internet Protocol Version 4",
            "version" => "Version",
            "ihl" => todo!(),
            "tos" => "TOS",
            "total_length" => "Total Length",
            "identification" => "Identification",
            "flags" => todo!(),
            "fragment_offset" => todo!(),
            "ttl" => "Time to Live",
            "protocol" => "Protocol",
            "checksum" => "Header Checksum",
            "source_address" => "Source Address",
            "destination_address" => "Destination Address",
            _ => return None
        }.to_string())
    }

    fn get_value(&self, key: &str) -> Option<String> {
        Some(match key {
            "version" => format!("{} ({})", self.get_version().to_string(), self.get_version().get_code()),
            "ihl" => todo!(),
            "tos" => self.get_tos().to_string(),
            "total_length" => self.get_total_length().to_string(),
            "identification" => format!("0x{:04X} ({})", self.get_identification(), self.get_identification()),
            "flags" => todo!(),
            "fragment_offset" => todo!(),
            "ttl" => self.get_ttl().to_string(),
            "protocol" => format!("{} ({})", self.get_protocol().to_string(), self.get_protocol().get_code()),
            "checksum" => format!("0x{:04X}", self.get_checksum()),
            "source_address" => self.get_source_address().to_string(),
            "destination_address" => self.get_destination_address().to_string(),
            _ => return None
        })
    }

    fn get_value_as_bytes(&self, key: &str) -> Option<Vec<u8>> {
        Some(match key {
            "frame" => {
                let mut buf = vec![0; IPV4_HEADER_LEN];

                buf[0] = (self.get_version().get_code() << 4) | (self.get_ihl() & 0x0F);
                buf[1] = self.get_tos();
                buf.splice(2..4, self.get_total_length().to_be_bytes());
                buf.splice(4..6, self.get_identification().to_be_bytes());
                buf[6] = (self.get_flags() << 5) | ((self.get_fragment_offset() >> 8) as u8 & 0x1F);
                buf[7] = (self.get_fragment_offset() & 0xFF) as u8;
                buf[8] = self.get_ttl();
                buf[9] = self.get_protocol().get_code();
                buf.splice(10..12, self.get_checksum().to_be_bytes());
                buf.splice(12..16, self.get_source_address().octets());
                buf.splice(16..20, self.get_destination_address().octets());

                buf
            }
            "ihl" => todo!(),
            "version" => vec![self.get_version().get_code() << 4],
            "tos" => vec![self.get_tos()],
            "total_length" => self.get_total_length().to_be_bytes().to_vec(),
            "identification" => self.get_identification().to_be_bytes().to_vec(),
            "flags" => todo!(),
            "fragment_offset" => todo!(),
            "ttl" => vec![self.get_ttl()],
            "protocol" => vec![self.get_protocol().get_code()],
            "checksum" => self.get_checksum().to_be_bytes().to_vec(),
            "source_address" => self.get_source_address().octets().to_vec(),
            "destination_address" => self.get_destination_address().octets().to_vec(),
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
