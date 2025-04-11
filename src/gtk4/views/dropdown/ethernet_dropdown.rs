use std::io;
use gtk4::gio::SimpleActionGroup;
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ethernet_frame::inter::ethernet_types::EthernetTypes;
use crate::database::sqlite::Database;
use crate::gtk4::views::dropdown::dropdown::{context_menu, create_row, set_selection, Dropdown};
use crate::gtk4::widgets::hex_editor::HexEditor;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;

pub trait EthernetDropdown {

    fn from_ethernet_frame(db: &io::Result<Database>, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &EthernetFrame, offset: usize) -> Self;
}

impl EthernetDropdown for Dropdown {

    fn from_ethernet_frame(db: &io::Result<Database>, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &EthernetFrame, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        match db {
            Ok(db) => {
                match ethernet_to_company(db, layer.get_destination_mac()) {
                    Some(company) => {
                        _self.list_box.append(&create_row(format!("{}:", layer.get_title("destination").unwrap()), format!("{} ({})", company, layer.get_value("destination").unwrap())));
                    }
                    None => {
                        _self.list_box.append(&create_row(format!("{}:", layer.get_title("destination").unwrap()), format!("({})", layer.get_value("destination").unwrap())));
                    }
                }

                match ethernet_to_company(db, layer.get_source_mac()) {
                    Some(company) => {
                        _self.list_box.append(&create_row(format!("{}:", layer.get_title("source").unwrap()), format!("{} ({})", company, layer.get_value("source").unwrap())));
                    }
                    None => {
                        _self.list_box.append(&create_row(format!("{}:", layer.get_title("source").unwrap()), format!("({})", layer.get_value("source").unwrap())));
                    }
                }
            }
            Err(_) => {
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("destination").unwrap()), format!("({})", layer.get_value("destination").unwrap())));
                _self.list_box.append(&create_row(format!("{}:", layer.get_title("source").unwrap()), format!("({})", layer.get_value("source").unwrap())));
            }
        }

        match layer.get_type() {
            EthernetTypes::Length(_) => _self.list_box.append(&create_row(format!("{}:", layer.get_title("length").unwrap()), layer.get_value("length").unwrap())),
            _ => _self.list_box.append(&create_row(format!("{}:", layer.get_title("type").unwrap()), layer.get_value("type").unwrap()))
        }

        _self
    }
}
