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

    fn from_arp_extension(db: &Database, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &ArpExtension, offset: usize) -> Self;
}

impl ArpDropdown for Dropdown {

    fn from_arp_extension(db: &Database, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &ArpExtension, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame"));
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("hardware_type")), layer.get_value("hardware_type")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("protocol_type")), layer.get_value("protocol_type")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("hardware_size")), layer.get_value("hardware_size")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("protocol_size")), layer.get_value("protocol_size")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("opcode")), layer.get_value("opcode")));

        match ethernet_to_company(db, layer.get_sender_mac()) {
            Some(company) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_mac")), format!("{} ({})", company, layer.get_value("sender_mac"))));
            }
            None => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_mac")), format!("({})", layer.get_value("sender_mac"))));
            }
        }

        match ip_to_icon(db, IpAddr::V4(layer.get_sender_address())) {
            Some(icon) => {
                _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("sender_address")), icon, layer.get_value("sender_address")));
            }
            None => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("sender_address")), layer.get_value("sender_address")));
            }
        }

        match ethernet_to_company(db, layer.get_target_mac()) {
            Some(company) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_mac")), format!("{} ({})", company, layer.get_value("target_mac"))));
            }
            None => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_mac")), format!("({})", layer.get_value("target_mac"))));
            }
        }

        match ip_to_icon(db, IpAddr::V4(layer.get_target_address())) {
            Some(icon) => {
                _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("target_address")), icon, layer.get_value("target_address")));
            }
            None => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("target_address")), layer.get_value("target_address")));
            }
        }

        _self
    }
}
