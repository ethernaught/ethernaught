use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
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
                self.get_version().to_string()
            }
            "tos" => {
                self.get_tos().to_string()
            }
            "total_length" => {
                self.get_total_length().to_string()
            }
            "identification" => {
                self.get_identification().to_string()
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
                self.get_protocol().to_string()
            }
            "checksum" => {
                self.get_checksum().to_string()
            }
            "source_address" => {
                self.get_source_address().to_string()
            }
            "destination_address" => {
                self.get_destination_address().to_string()
            }
            _ => unimplemented!()
        }.to_string()
    }

    fn get_description(&self, variable: &str) -> String {
        todo!()
    }

    fn get_value_as_bytes(&self, variable: &str) -> Vec<u8> {
        todo!()
    }

    fn to_string(&self) -> String {
        todo!()
    }
}
