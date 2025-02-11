#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Types {
    Icmp,
    Arp,
    Tcp,
    Udp,
    Broadcast,
    Dns,
    BitTorrent
}

impl Types {

    pub fn to_string(&self) -> String {
        match self {
            Self::Icmp => "ICMP".to_string(),
            Self::Arp => "ARP".to_string(),
            Self::Tcp => "TCP".to_string(),
            Self::Udp => "UDP".to_string(),
            Self::Broadcast => "Broadcast".to_string(),
            Self::Dns => "DNS".to_string(),
            Self::BitTorrent => "BITTORRENT".to_string()
        }
    }
}
