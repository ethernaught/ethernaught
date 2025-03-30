use rlibpcap::packet::layers::ip::ipv4_layer::{Ipv4Layer, IPV4_HEADER_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for Ipv4Layer {

    fn get_selection(&self, variable: &str) -> (usize, usize) {
        match variable {
            "frame" => {
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
                (6, 2)
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

    fn get_field_name(&self, variable: &str) -> String {
        match variable {
            "frame" => {
                "ipv4"
            }
            "version" => {
                "ipv4.version"
            }
            "ihl" => {
                "ipv4.ihl"
            }
            "tos" => {
                "ipv4.tos"
            }
            "total_length" => {
                "ipv4.total-length"
            }
            "identification" => {
                "ipv4.identification"
            }
            "flags" => {
                "ipv4.flags"
            }
            "fragment_offset" => {
                "ipv4.offset"
            }
            "ttl" => {
                "ipv4.ttl"
            }
            "protocol" => {
                "ipv4.protocol"
            }
            "checksum" => {
                "ipv4.checksum"
            }
            "source_address" => {
                "ipv4.source"
            }
            "destination_address" => {
                "ipv4.destination"
            }
            _ => unimplemented!()
        }.to_string()
    }

    fn get_value(&self, variable: &str) -> String {
        match variable {
            "frame" => {
                String::from("Internet Protocol Version 4")
            }
            "version" => {
                self.get_version().get_code().to_string()
            }
            "ihl" => {
                todo!()
            }
            "tos" => {
                self.get_tos().to_string()
            }
            "total_length" => {
                self.get_total_length().to_string()
            }
            "identification" => {
                format!("0x{:04X} ({})", self.get_identification(), self.get_identification())
            }
            "flags" => {
                todo!()
            }
            "fragment_offset" => {
                todo!()
            }
            "ttl" => {
                self.get_ttl().to_string()
            }
            "protocol" => {
                format!("{:?} ({})", self.get_protocol(), self.get_protocol().get_code())
            }
            "checksum" => {
                format!("0x{:04X}", self.get_checksum())
            }
            "source_address" => {
                self.get_source_address().to_string()
            }
            "destination_address" => {
                self.get_destination_address().to_string()
            }
            _ => unimplemented!()
        }
    }

    fn get_description(&self, variable: &str) -> String {
        match variable {
            "frame" => {
                self.get_value(variable)
            }
            "version" => {
                format!("Version: {}", self.get_value(variable))
            }
            "ihl" => {
                todo!()
            }
            "tos" => {
                format!("TOS: {}", self.get_value(variable))
            }
            "total_length" => {
                format!("Total Length: {}", self.get_value(variable))
            }
            "identification" => {
                format!("Identification: {}", self.get_value(variable))
            }
            "flags" => {
                todo!()
            }
            "fragment_offset" => {
                todo!()
            }
            "ttl" => {
                format!("Time to Live: {}", self.get_value(variable))
            }
            "protocol" => {
                format!("Protocol: {}", self.get_value(variable))
            }
            "checksum" => {
                format!("Header Checksum: {}", self.get_value(variable))
            }
            "source_address" => {
                format!("Source Address: {}", self.get_value(variable))
            }
            "destination_address" => {
                format!("Destination Address: {}", self.get_value(variable))
            }
            _ => unimplemented!()
        }
    }

    fn get_value_as_bytes(&self, variable: &str) -> Vec<u8> {
        match variable {
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
            "ihl" => {
                todo!()
            }
            "version" => {
                vec![self.get_version().get_code() << 4]
            }
            "tos" => {
                vec![self.get_tos()]
            }
            "total_length" => {
                self.get_total_length().to_be_bytes().to_vec()
            }
            "identification" => {
                self.get_identification().to_be_bytes().to_vec()
            }
            "flags" => {
                todo!()
            }
            "fragment_offset" => {
                todo!()
            }
            "ttl" => {
                vec![self.get_ttl()]
            }
            "protocol" => {
                vec![self.get_protocol().get_code()]
            }
            "checksum" => {
                self.get_checksum().to_be_bytes().to_vec()
            }
            "source_address" => {
                self.get_source_address().octets().to_vec()
            }
            "destination_address" => {
                self.get_destination_address().octets().to_vec()
            }
            _ => unimplemented!()
        }
    }

    fn to_string(&self) -> String {
        todo!()
    }
}
