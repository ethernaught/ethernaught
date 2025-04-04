use rlibpcap::packet::layers::ethernet_frame::llc::llc_extension::{LlcExtension, LLC_EXTENSION_LEN};
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;

impl LayerExt for LlcExtension {

    fn get_fields(&self) -> Vec<&str> {
        vec![
            "dsap",
            "ssap",
            "control"
        ]
    }

    fn get_selection(&self, key: &str) -> Option<(usize, usize)> {
        Some(match key {
            "frame" => (0, 3),
            "dsap" => (0, 1),
            "ssap" => (1, 1),
            "control" => (2, 1),
            _ => return None
        })
    }

    fn get_field_name(&self, key: &str) -> Option<String> {
        Some(match key {
            "frame" => "llc",
            "dsap" => "llc.dsap",
            "ssap" => "llc.ssap",
            "control" => "llc.control",
            _ => return None
        }.to_string())
    }

    fn get_title(&self, key: &str) -> Option<String> {
        Some(match key {
            "frame" => "Logical-Link Control",
            "dsap" => "DSAP",
            "ssap" => "SSAP",
            "control" => "Control Field",
            _ => return None
        }.to_string())
    }

    fn get_value(&self, key: &str) -> Option<String> {
        Some(match key {
            "dsap" => format!("{} (0x{:02X})", self.get_dsap().to_string(), self.get_dsap()), //NULL LSAP (0x00)
            "ssap" => format!("{} (0x{:02X})", self.get_ssap().to_string(), self.get_ssap()), //NULL LSAP (0x00)
            "control" => format!("func={} ({:02X})", self.get_control().to_string(), self.get_control().get_code()), //U, func=XID (0xAF)
            _ => return None
        })
    }

    fn get_value_as_bytes(&self, key: &str) -> Option<Vec<u8>> {
        Some(match key {
            "frame" => {
                let mut buf = vec![0; LLC_EXTENSION_LEN];

                buf[0] = self.get_dsap();
                buf[1] = self.get_ssap();
                buf[2] = self.get_control().get_code();

                buf
            }
            "dsap" => vec![self.get_dsap()],
            "ssap" => vec![self.get_ssap()],
            "control" => vec![self.get_control().get_code()],
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
