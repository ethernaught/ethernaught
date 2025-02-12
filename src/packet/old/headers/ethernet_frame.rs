use crate::packet::inter::ethernet_types::EthernetTypes;

#[derive(Debug, Clone)]
pub struct EthernetFrame {
    destination: [u8; 6],
    source: [u8; 6],
    _type: EthernetTypes
}

impl EthernetFrame {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 14 {
            return None;
        }

        Some(EthernetFrame {
            destination: [buf[0], buf[1], buf[2], buf[3], buf[4], buf[5]],
            source: [buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]],
            _type: EthernetTypes::get_type_from_code(u16::from_be_bytes([buf[12], buf[13]])).unwrap()
        })
    }

    pub fn get_destination(&self) -> [u8; 6] {
        self.destination
    }

    pub fn get_source(&self) -> [u8; 6] {
        self.source
    }

    pub fn get_type(&self) -> EthernetTypes {
        self._type
    }
}
