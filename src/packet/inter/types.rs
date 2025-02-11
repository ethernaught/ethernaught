#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Types {
    IPv4,
    Arp,
    IPv6,
    Broadcast
}

impl Types {

    pub fn get_type_from_code(code: u16) -> Result<Self, String> {
        for c in [Self::IPv4, Self::Arp, Self::IPv6, Self::Broadcast] {
            if c.get_code() == code {
                return Ok(c);
            }
        }

        Err(format!("Couldn't find for code: {}", code))
    }

    pub fn get_code(&self) -> u16 {
        match self {
            Self::IPv4 => 2048,
            Self::Arp => 2054,
            //Self::WoLan => 2114,
            Self::IPv6 => 34525,
            Self::Broadcast => 34969
            /*
            Self::Srp => 8938,
            Self::Avtp => 8944,
            Self::Trill => 8947,
            Self::Rarp => 32821,
            Self::EtherTalk => 32923,
            Self::Aarp => 33011,
            Self::VLan => 33024,
            Self::Ipx => 33079,

            Self::Qnx => 33284, //: QNX Qnet
            Self::Efc => 34824, //: Ethernet flow control
            Self::Esp => 34825, //: Ethernet Slow Protocols (e.g., Link Aggregation Control Protocol)
            Self::MplsUnicast => 34887, //: MPLS unicast
            Self::MplsMulticast => 34888, //: MPLS multicast
            Self::PppoEDiscovery => 34915, //: PPPoE Discovery Stage
            Self::PppoESession => 34916, //: PPPoE Session Stage
            Self::Eap => 34926, //: EAP over LAN (IEEE 802.1X)
            Self::ProfiNet => 34930, //: PROFINET Protocol
            Self::Ata => 34946, //: ATA over Ethernet
            Self::EtherCat => 34948, //: EtherCAT Protocol
            Self:: => 34952, //: Service VLAN tag identifier (S-Tag) on Q-in-Q tunnel
            Self::Lldp => 35020, //: Link Layer Discovery Protocol (LLDP)
            Self::HomePlug => 35041, //: HomePlug Green PHY
            Self::MacSec => 35045, //: IEEE 802.1AE MAC security (MACsec)
            Self::Pbb => 35047, //: Provider Backbone Bridges (PBB) (IEEE 802.1ah)
            Self::Ptp => 35063, //: Precision Time Protocol (PTP) over IEEE 802.3 Ethernet
            Self::Cfm => 35074, //: IEEE 802.1ag Connectivity Fault Management (CFM) Protocol
            Self::FcoEChannel => 35078, //: Fibre Channel over Ethernet (FCoE)
            Self::FcoEInitialization => 35092, //: FCoE Initialization Protocol
            Self::Rdma => 35093, //: RDMA over Converged Ethernet (RoCE)
            Self::Hsr => 35119, //: High-availability Seamless Redundancy (HSR)
            Self:: => 36864, //: Ethernet Configuration Testing Protocol
            */
        }
    }
}

