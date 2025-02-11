#[derive(Clone)]
pub struct UdpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16
}

impl UdpHeader {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 8 {
            return None;
        }

        Some(Self {
            source_port: u16::from_be_bytes([buf[0], buf[1]]),
            destination_port: u16::from_be_bytes([buf[2], buf[3]]),
            length: u16::from_be_bytes([buf[4], buf[5]]),
            checksum: u16::from_be_bytes([buf[6], buf[7]])
        })
    }
}
