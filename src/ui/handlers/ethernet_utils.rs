use gtk::prelude::SocketExt;
use pcap::packet::layers::ethernet_frame::inter::ethernet_address::EthernetAddress;
use crate::database::sqlite::Database;

pub fn ethernet_to_company(db: &Database, mac: EthernetAddress) -> Option<String> {
    if mac.is_broadcast() {
        return Some(String::from("Broadcast"));
    }

    let documents = db.get(
        "oui",
        Some(vec!["id", "name", "suffix"]),
        Some(format!("'{}' LIKE '%' || suffix", mac.to_string()).as_str())
    );

    match documents.get(0) {
        Some(document) => {
            Some(format!("{}_{}", document.get("name").unwrap(), document.get("suffix").unwrap()))
        }
        None => None
    }
}
