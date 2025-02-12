#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Protocols {
    Icmp,
    Igmp,
    Tcp,
    Udp,
    Ipv6,
    Gre,
    Ospf,
    Sps
}

impl Protocols {

    pub fn get_protocol_from_code(code: u8) -> Result<Self, String> {
        for c in [Self::Icmp, Self::Igmp, Self::Tcp, Self::Udp, Self::Ipv6, Self::Gre, Self::Ospf, Self::Sps] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u8 {
        match self {
            Self::Icmp => 1,
            Self::Igmp => 2,
            Self::Tcp => 6,
            Self::Udp => 17,
            Self::Ipv6 => 18,
            Self::Gre => 47,
            Self::Ospf => 89,
            Self::Sps => 128
        }
    }
}
