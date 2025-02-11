#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Protocols {
    Icmp,
    Tcp,
    Udp,
    Gre,
    Sps
}

impl Protocols {

    pub fn get_protocol_from_code(code: u8) -> Result<Self, String> {
        for c in [Self::Icmp, Self::Tcp, Self::Udp, Self::Gre, Self::Sps] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Self::Icmp => 1,
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Gre => 47,
            Self::Sps => 128
        }
    }
}
