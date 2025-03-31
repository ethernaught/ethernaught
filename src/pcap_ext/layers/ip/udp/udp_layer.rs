use rlibpcap::packet::layers::ip::udp::udp_layer::{UdpLayer, UDP_HEADER_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::pcap_ext::layers::ip::udp::inter::udp_ports::UdpPorts;

impl LayerExt for UdpLayer {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "source_port",
            "destination_port",
            "length",
            "checksum"
        ]
    }

    fn get_selection(&self, key: &str) -> (usize, usize) {
        match key {
            "frame" => (0, UDP_HEADER_LEN),
            "source_port" => (0, 2),
            "destination_port" => (2, 2),
            "length" => (4, 2),
            "checksum" => (6, 2),
            _ => unimplemented!()
        }
    }

    fn get_field_name(&self, key: &str) -> String {
        match key {
            "frame" => "udp",
            "source_port" => "udp.source_port",
            "destination_port" => "udp.destination_port",
            "length" => "udp.length",
            "checksum" => "udp.checksum",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_title(&self, key: &str) -> String {
        match key {
            "frame" => "User Datagram Protocol",
            "source_port" => "Source Port",
            "destination_port" => "Destination Port",
            "length" => "Length",
            "checksum" => "Checksum",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_value(&self, key: &str) -> String {
        match key {
            "source_port" => {
                match UdpPorts::from_code(self.get_source_port()) {
                    Ok(port) => {
                        format!("{} ({})", port.to_string(), self.get_source_port())
                    }
                    Err(_) => {
                        self.get_source_port().to_string()
                    }
                }
            },
            "destination_port" => {
                match UdpPorts::from_code(self.get_destination_port()) {
                    Ok(port) => {
                        format!("{} ({})", port.to_string(), self.get_destination_port())
                    }
                    Err(_) => {
                        self.get_destination_port().to_string()
                    }
                }
            },
            "length" => self.get_length().to_string(),
            "checksum" => format!("0x{:04X}", self.get_checksum()),
            _ => unimplemented!()
        }
    }

    fn get_value_as_bytes(&self, key: &str) -> Vec<u8> {
        match key {
            "frame" => {
                let mut buf = vec![0; UDP_HEADER_LEN];

                buf.splice(0..2, self.get_source_port().to_be_bytes());
                buf.splice(2..4, self.get_destination_port().to_be_bytes());
                buf.splice(4..6, self.get_length().to_be_bytes());
                buf.splice(6..8, self.get_checksum().to_be_bytes());

                buf
            }
            "source_port" => self.get_source_port().to_be_bytes().to_vec(),
            "destination_port" => self.get_destination_port().to_be_bytes().to_vec(),
            "length" => self.get_length().to_be_bytes().to_vec(),
            "checksum" => self.get_checksum().to_be_bytes().to_vec(),
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
