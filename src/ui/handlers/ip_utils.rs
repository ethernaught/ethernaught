use std::net::{IpAddr, Ipv4Addr};
use crate::database::sqlite::Database;

pub fn ip_to_code(db: &Database, address: IpAddr) -> Option<String> {
    match address {
        IpAddr::V4(address) => {
            let address = u32::from(address);
            Some(db.get(
                "ipv4_location",
                Some(vec!["id", "country_code"]),
                Some(format!("start <= {} AND end >= {}", address, address).as_str())
            ).get(0).unwrap().get("country_code").unwrap().to_string())
        }
        IpAddr::V6(address) => {
            None
        }
    }
}
