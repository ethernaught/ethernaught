use std::net::{IpAddr, Ipv4Addr};
use gtk::gdk_pixbuf::Pixbuf;
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

//TEMPORARY - MOVE THIS OUT LATER
fn code_to_icon(code: &str) -> Option<Pixbuf> {
    match code {
        "AM" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_amenia.svg").ok()
        }
        "AT" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_austria.svg").ok()
        }
        "BE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_belgium.svg").ok()
        }
        "BO" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_bolivia.svg").ok()
        }
        "BR" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_brazil.svg").ok()
        }
        "BG" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_bulgaria.svg").ok()
        }
        "CA" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_canada.svg").ok()
        }
        "TD" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_chad.svg").ok()
        }
        "CN" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_china.svg").ok()
        }
        "DK" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_denmark.svg").ok()
        }
        "EE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_estonia.svg").ok()
        }
        "FI" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_finland.svg").ok()
        }
        "FR" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_france.svg").ok()
        }
        "GA" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_gabon.svg").ok()
        }
        "GE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_georgia.svg").ok()
        }
        "DE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_germany.svg").ok()
        }
        "GN" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_guinea.svg").ok()
        }
        "HU" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_hungary.svg").ok()
        }
        "IE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_ireland.svg").ok()
        }
        "IT" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_italy.svg").ok()
        }
        "CI" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_ivory_coast.svg").ok()
        }
        "JP" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_japan.svg").ok()
        }
        "LV" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_latvia.svg").ok()
        }
        "LT" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_lithuania.svg").ok()
        }
        "LU" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_luxembourg.svg").ok()
        }
        "ML" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_mali.svg").ok()
        }
        "NL" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_netherlands.svg").ok()
        }
        "NG" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_nigeria.svg").ok()
        }
        "PA" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_panama.svg").ok()
        }
        "PE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_peru.svg").ok()
        }
        "RO" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_romania.svg").ok()
        }
        "RU" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_russia.svg").ok()
        }
        "SE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_sweden.svg").ok()
        }
        "CH" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_switzerland.svg").ok()
        }
        "UK" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_united_kingdom.svg").ok()
        }
        "US" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_united_states.svg").ok()
        }
        "YE" => {
            Pixbuf::from_file("../../../res/icons/flags/ic_yemen.svg").ok()
        }
        _ => None
    }
}
