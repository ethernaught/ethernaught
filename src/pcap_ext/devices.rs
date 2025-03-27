use std::net::{IpAddr, Ipv4Addr};
use pcap::devices::Device;
use pcap::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use pcap::utils::data_link_types::DataLinkTypes;

pub trait Serialize {

    fn serialize(&self) -> Vec<u8>;

    fn unserialize(buf: &[u8]) -> Self;
}

impl Serialize for Device {

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        let name = self.get_name().into_bytes();
        buf.push(name.len() as u8);
        buf.extend_from_slice(&name);

        buf.extend_from_slice(&self.get_index().to_ne_bytes());
        buf.extend_from_slice(&self.get_data_link_type().get_code().to_ne_bytes());
        //FLAGS - U32
        buf.extend_from_slice(&self.get_mac().to_bytes());

        match self.get_address() {
            Some(address) => {
                match address {
                    IpAddr::V4(address) => {
                        buf.push(4);
                        buf.extend_from_slice(&address.octets());
                    }
                    IpAddr::V6(address) => {
                        buf.push(6);
                        buf.extend_from_slice(&address.octets());
                    }
                }
            }
            None => {
                buf.push(0);
            }
        }

        buf
    }

    fn unserialize(buf: &[u8]) -> Self {
        let mut offset: usize = buf[0] as usize + 1;
        let name = String::from_utf8_lossy(&buf[1..offset]).to_string();

        let index = i32::from_ne_bytes([buf[offset], buf[offset + 1], buf[offset + 2], buf[offset + 3]]);
        let data_link_type = DataLinkTypes::from_code(u32::from_ne_bytes([buf[offset + 4], buf[offset + 5], buf[offset + 6], buf[offset + 7]])).unwrap();

        let mac = EthernetAddress::new(buf[offset + 8], buf[offset + 9], buf[offset + 10], buf[offset + 11], buf[offset + 12], buf[offset + 13]);

        offset += 14;

        let address = match buf[14] {
            4 => {
                Some(IpAddr::from(Ipv4Addr::new(buf[offset + 15], buf[offset + 16], buf[offset + 17], buf[offset + 18])))
            }
            6 => {
                None
            }
            _ => None
        };

        Self {
            name,
            address,
            index,
            data_link_type,
            mac,
            flags: Vec::new(),
        }
    }
}
