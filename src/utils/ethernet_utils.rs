use gtk::prelude::SocketExt;
use rlibpcap::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::database::sqlite::Database;

pub fn ethernet_to_company(db: &Database, mac: EthernetAddress) -> Option<String> {
    if mac.is_broadcast() {
        return Some(String::from("Broadcast"));
    }

    let mac = mac.to_bytes();
    let mac = u64::from_be_bytes([0x00, 0x00, mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]]);

    let documents = db.get(
        "oldui",
        Some(vec!["id", "prefix", "name", "company"]),
        Some(format!("start <= {} AND end >= {}", mac, mac).as_str())
    );

    match documents.get(0) {
        Some(document) => {
            Some(format!("{}_{}", document.get("name").unwrap(), document.get("prefix").unwrap()))
        }
        None => None
    }
}
