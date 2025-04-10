use std::io;
use std::net::IpAddr;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use rlibpcap::packet::layers::ethernet_frame::llc::llc_extension::LlcExtension;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::gtk3::views::dropdown::dropdown::{context_menu, create_row, create_row_with_icon, set_selection, Dropdown};
use crate::gtk3::widgets::hex_editor::HexEditor;

pub trait LlcDropdown {

    fn from_llc_extension(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &LlcExtension, offset: usize) -> Self;
}

impl LlcDropdown for Dropdown {

    fn from_llc_extension(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &LlcExtension, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("dsap").unwrap()), layer.get_value("dsap").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("ssap").unwrap()), layer.get_value("ssap").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("control").unwrap()), layer.get_value("control").unwrap()));

        _self
    }
}
