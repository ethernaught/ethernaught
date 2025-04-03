use std::io;
use std::net::IpAddr;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::ethernet_frame::EthernetFrame;
use rlibpcap::packet::layers::ip::ipv4_layer::Ipv4Layer;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::views::dropdown::dropdown::{context_menu, create_row, create_row_with_icon, set_selection, Dropdown};
use crate::views::sidebar_view::SidebarView;
use crate::widgets::hex_editor::HexEditor;

pub trait Ipv4Dropdown {

    fn from_ipv4_layer(db: &io::Result<Database>, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv4Layer, offset: usize) -> Self;
}

impl Ipv4Dropdown for Dropdown {

    fn from_ipv4_layer(db: &io::Result<Database>, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv4Layer, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("version").unwrap()), layer.get_value("version").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("tos").unwrap()), layer.get_value("tos").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("total_length").unwrap()), layer.get_value("total_length").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("identification").unwrap()), layer.get_value("identification").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("ttl").unwrap()), layer.get_value("ttl").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("protocol").unwrap()), layer.get_value("protocol").unwrap()));

        let checksum_string = if layer.validate_checksum() { "correct" } else { "incorrect" };
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("checksum").unwrap()), format!("{} [{}]", layer.get_value("checksum").unwrap(), checksum_string)));

        match db {
            Ok(db) => {
                match ip_to_icon(db, IpAddr::V4(layer.get_source_address())) {
                    Some(icon) => {
                        _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("source_address").unwrap()), icon, layer.get_value("source_address").unwrap()));
                    }
                    None => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("source_address").unwrap()), layer.get_value("source_address").unwrap()));
                    }
                }

                match ip_to_icon(db, IpAddr::V4(layer.get_destination_address())) {
                    Some(icon) => {
                        _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("destination_address").unwrap()), icon, layer.get_value("destination_address").unwrap()));
                    }
                    None => {
                        _self.list_box.add(&create_row(format!("{}:", layer.get_title("destination_address").unwrap()), layer.get_value("destination_address").unwrap()));
                    }
                }
            }
            Err(e) => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("source_address").unwrap()), layer.get_value("source_address").unwrap()));
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("destination_address").unwrap()), layer.get_value("destination_address").unwrap()));
            }
        }

        _self
    }
}
