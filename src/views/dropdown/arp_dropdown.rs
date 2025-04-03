use std::io;
use std::net::IpAddr;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::views::dropdown::dropdown::{context_menu, create_row, create_row_with_icon, set_selection, Dropdown};
use crate::widgets::hex_editor::HexEditor;

pub trait ArpDropdown {

    fn from_arp_extension(db: &io::Result<Database>, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &ArpExtension, offset: usize) -> Self;
}

impl ArpDropdown for Dropdown {

    fn from_arp_extension(db: &io::Result<Database>, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &ArpExtension, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("hardware_type").unwrap()), layer.get_value("hardware_type").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("protocol_type").unwrap()), layer.get_value("protocol_type").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("hardware_size").unwrap()), layer.get_value("hardware_size").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("protocol_size").unwrap()), layer.get_value("protocol_size").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("opcode").unwrap()), layer.get_value("opcode").unwrap()));

        match db {
            Ok(db) => {
                match ethernet_to_company(db, layer.get_sender_mac()) {
                    Some(company) => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_mac").unwrap()), format!("{} ({})", company, layer.get_value("sender_mac").unwrap())));
                    }
                    None => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_mac").unwrap()), format!("({})", layer.get_value("sender_mac").unwrap())));
                    }
                }

                match ip_to_icon(db, IpAddr::V4(layer.get_sender_address())) {
                    Some(icon) => {
                        _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("sender_address").unwrap()), icon, layer.get_value("sender_address").unwrap()));
                    }
                    None => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_address").unwrap()), layer.get_value("sender_address").unwrap()));
                    }
                }

                match ethernet_to_company(db, layer.get_target_mac()) {
                    Some(company) => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_mac").unwrap()), format!("{} ({})", company, layer.get_value("target_mac").unwrap())));
                    }
                    None => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_mac").unwrap()), format!("({})", layer.get_value("target_mac").unwrap())));
                    }
                }

                match ip_to_icon(db, IpAddr::V4(layer.get_target_address())) {
                    Some(icon) => {
                        _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("target_address").unwrap()), icon, layer.get_value("target_address").unwrap()));
                    }
                    None => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_address").unwrap()), layer.get_value("target_address").unwrap()));
                    }
                }
            }
            Err(e) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_mac").unwrap()), format!("({})", layer.get_value("sender_mac").unwrap())));
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_address").unwrap()), layer.get_value("sender_address").unwrap()));
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_mac").unwrap()), format!("({})", layer.get_value("target_mac").unwrap())));
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_address").unwrap()), layer.get_value("target_address").unwrap()));
            }
        }

        _self
    }
}
