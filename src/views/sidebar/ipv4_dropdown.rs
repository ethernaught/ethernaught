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
use crate::views::sidebar::dropdown::{context_menu, create_row, create_row_with_icon, set_selection, Dropdown};
use crate::views::sidebar_view::SidebarView;
use crate::widgets::hex_editor::HexEditor;

pub trait Ipv4Dropdown {

    fn from_ipv4_layer(db: &Database, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv4Layer, offset: usize) -> Self;
}

impl Ipv4Dropdown for Dropdown {

    fn from_ipv4_layer(db: &Database, hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Ipv4Layer, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame"));
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("version")), layer.get_value("version")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("tos")), layer.get_value("tos")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("total_length")), layer.get_value("total_length")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("identification")), layer.get_value("identification")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("ttl")), layer.get_value("ttl")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("protocol")), layer.get_value("protocol")));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("checksum")), layer.get_value("checksum")));

        match ip_to_icon(db, IpAddr::V4(layer.get_source_address())) {
            Some(icon) => {
                _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("source_address")), icon, layer.get_value("source_address")));
            }
            None => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("source_address")), layer.get_value("source_address")));
            }
        }

        match ip_to_icon(db, IpAddr::V4(layer.get_destination_address())) {
            Some(icon) => {
                _self.list_box.add(&create_row_with_icon(format!("{}:", layer.get_title("destination_address")), icon, layer.get_value("destination_address")));
            }
            None => {
                _self.list_box.add(&create_row(format!("{}:", layer.get_title("destination_address")), layer.get_value("destination_address")));
            }
        }

        _self
    }
}

