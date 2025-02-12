#[derive(Clone)]
pub struct IcmpHeader {
    icmp_type: u8,
    code: u8,
    checksum: u16,
    identifier: Option<u16>,
    sequence_number: Option<u16>
}

impl IcmpHeader {

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        if buf.len() < 8 {
            return None;
        }

        let icmp_type = buf[0];
        let code = buf[1];
        let checksum = u16::from_be_bytes([buf[2], buf[3]]);

        let (identifier, sequence_number) = if icmp_type == 8 || icmp_type == 0 {
            let id = u16::from_be_bytes([buf[4], buf[5]]);
            let seq = u16::from_be_bytes([buf[6], buf[7]]);
            (Some(id), Some(seq))

        } else {
            (None, None)
        };

        Some(Self {
            icmp_type,
            code,
            checksum,
            identifier,
            sequence_number
        })
    }

    pub fn get_icmp_type(&self) -> u8 {
        self.icmp_type
    }

    pub fn get_code(&self) -> u8 {
        self.code
    }

    pub fn get_checksum(&self) -> u16 {
        self.checksum
    }

    pub fn get_identifier(&self) -> Option<u16> {
        self.identifier
    }

    pub fn get_sequence_number(&self) -> Option<u16> {
        self.sequence_number
    }
}
