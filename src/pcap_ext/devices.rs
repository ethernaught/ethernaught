use std::net::{IpAddr, Ipv4Addr};
use rlibpcap::devices::Device;
use rlibpcap::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use rlibpcap::utils::data_link_types::DataLinkTypes;

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

        match self.get_mac() {
            Some(mac) => {
                buf.push(6);
                buf.extend_from_slice(&mac.to_bytes());
            }
            None => {
                buf.push(0);
            }
        }

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

        let mac = match buf[offset + 8] {
            6 => Some(EthernetAddress::new(buf[offset + 9], buf[offset + 10], buf[offset + 11], buf[offset + 12], buf[offset + 13], buf[offset + 14])),
            _ => None
        };

        offset += 9 + buf[offset + 8] as usize;

        let address = match buf[offset] {
            4 => Some(IpAddr::from(Ipv4Addr::new(buf[offset + 1], buf[offset + 2], buf[offset + 3], buf[offset + 4]))),
            6 => None,
            _ => None
        };

        Device::new(name, address, index, data_link_type, mac, Vec::new())
    }
}
