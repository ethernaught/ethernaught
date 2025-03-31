use rlibpcap::packet::layers::ip::tcp::tcp_layer::{TcpLayer, TCP_HEADER_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::pcap_ext::layers::ip::tcp::inter::tcp_ports::TcpPorts;

impl LayerExt for TcpLayer {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "source_port",
            "destination_port",
            "sequence_number",
            "acknowledgment_number",
            //"data_offset",
            //"flags",
            "window_size",
            "checksum",
            "urgent_pointer"
        ]
    }

    fn get_selection(&self, key: &str) -> (usize, usize) {
        match key {
            "frame" => (0, TCP_HEADER_LEN),
            "source_port" => (0, 2),
            "destination_port" => (2, 2),
            "sequence_number" => (4, 4),
            "acknowledgment_number" => (8, 4),
            //"data_offset" => (12, 1),
            //"flags" => (13, 2),
            "window_size" => (14, 2),
            "checksum" => (16, 2),
            "urgent_pointer" => (18, 2),
            _ => unimplemented!()
        }
    }

    fn get_field_name(&self, key: &str) -> String {
        match key {
            "frame" => "tcp",
            "source_port" => "tcp.source_port",
            "destination_port" => "tcp.destination_port",
            "sequence_number" => "tcp.sequence_number",
            "acknowledgment_number" => "tcp.acknowledgment_number",
            "data_offset" => "tcp.data_offset",
            "flags" => "tcp",
            "window_size" => "tcp.window_size",
            "checksum" => "tcp.checksum",
            "urgent_pointer" => "tcp.urgent_pointer",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_title(&self, key: &str) -> String {
        match key {
            "frame" => "Transmission Control Protocol",
            "source_port" => "Source Port",
            "destination_port" => "Destination Port",
            "sequence_number" => "Sequence Number",
            "acknowledgment_number" => "Acknowledgment Number",
            //"data_offset" => ,
            //"flags" => ,
            "window_size" => "Window",
            "checksum" => "Checksum",
            "urgent_pointer" => "Urgent Pointer",
            _ => unimplemented!()
        }.to_string()
    }

    fn get_value(&self, key: &str) -> String {
        match key {
            "source_port" => {
                match TcpPorts::from_code(self.get_source_port()) {
                    Ok(port) => {
                        format!("{} ({})", port.to_string(), self.get_source_port())
                    }
                    Err(_) => {
                        self.get_source_port().to_string()
                    }
                }
            },
            "destination_port" => {
                match TcpPorts::from_code(self.get_destination_port()) {
                    Ok(port) => {
                        format!("{} ({})", port.to_string(), self.get_destination_port())
                    }
                    Err(_) => {
                        self.get_destination_port().to_string()
                    }
                }
            },
            "sequence_number" => self.get_sequence_number().to_string(),
            "acknowledgment_number" => self.get_acknowledgment_number().to_string(),
            //"data_offset" => ,
            //"flags" => ,
            "window_size" => self.get_window_size().to_string(),
            "checksum" => format!("0x{:04X}", self.get_checksum()),
            "urgent_pointer" => self.get_urgent_pointer().to_string(),
            _ => unimplemented!()
        }
    }

    fn get_value_as_bytes(&self, key: &str) -> Vec<u8> {
        match key {
            "frame" => {
                let mut buf = vec![0; TCP_HEADER_LEN];

                buf.splice(0..2, self.get_source_port().to_be_bytes());
                buf.splice(2..4, self.get_destination_port().to_be_bytes());
                buf.splice(4..8, self.get_sequence_number().to_be_bytes());
                buf.splice(8..12, self.get_acknowledgment_number().to_be_bytes());
                buf[12] = ((self.get_data_offset() / 4) << 4) | ((self.get_flags() >> 8) as u8 & 0x0F);

                buf[13] = (self.get_flags() & 0xFF) as u8;
                buf.splice(14..16, self.get_window_size().to_be_bytes());
                buf.splice(16..18, self.get_checksum().to_be_bytes());
                buf.splice(18..20, self.get_urgent_pointer().to_be_bytes());

                buf
            }
            "source_port" => self.get_source_port().to_be_bytes().to_vec(),
            "destination_port" => self.get_destination_port().to_be_bytes().to_vec(),
            "sequence_number" => self.get_sequence_number().to_be_bytes().to_vec(),
            "acknowledgment_number" => self.get_acknowledgment_number().to_be_bytes().to_vec(),
            //"data_offset" => ,
            //"flags" => ,
            "window_size" => self.get_window_size().to_be_bytes().to_vec(),
            "checksum" => self.get_checksum().to_be_bytes().to_vec(),
            "urgent_pointer" => self.get_urgent_pointer().to_be_bytes().to_vec(),
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
