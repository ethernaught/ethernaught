use std::net::IpAddr;
use gtk::{gdk, glib, Builder, Button, Container, CssProvider, Image, Label, ListBox, ListBoxRow, Orientation, StyleContext};
use gtk::gio::SimpleActionGroup;
use gtk::glib::Cast;
use gtk::glib::Propagation::Proceed;
use gtk::prelude::{BuilderExtManual, ButtonExt, ContainerExt, EditableExt, GestureSingleExt, ImageExt, LabelExt, ListBoxExt, ListBoxRowExt, TextExt, WidgetExt};
use rlibpcap::packet::layers::ethernet_frame::arp::arp_extension::ArpExtension;
use rlibpcap::packet::layers::sll2_frame::sll2_frame::Sll2Frame;
use crate::database::sqlite::Database;
use crate::pcap_ext::layers::inter::layer_ext::LayerExt;
use crate::utils::ethernet_utils::ethernet_to_company;
use crate::utils::ip_utils::ip_to_icon;
use crate::gtk3::views::dropdown::dropdown::{context_menu, create_row, create_row_with_icon, set_selection, Dropdown};
use crate::gtk3::widgets::hex_editor::HexEditor;

pub trait Sll2Dropdown {

    fn from_sll2_frame(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Sll2Frame, offset: usize) -> Self;
}

impl Sll2Dropdown for Dropdown {

    fn from_sll2_frame(hex_editor: &HexEditor, actions: &SimpleActionGroup, layer: &Sll2Frame, offset: usize) -> Self {
        let _self = Self::new(&layer.get_title("frame").unwrap());
        _self.list_box.connect_row_activated(set_selection(&hex_editor, layer, offset));
        _self.list_box.connect_button_press_event(context_menu(&hex_editor, actions, layer, offset));

        _self.list_box.add(&create_row(format!("{}:", layer.get_title("protocol").unwrap()), layer.get_value("protocol").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("if_index").unwrap()), layer.get_value("if_index").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("data_link_type").unwrap()), layer.get_value("data_link_type").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("packet_type").unwrap()), layer.get_value("packet_type").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("address_length").unwrap()), layer.get_value("address_length").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("address").unwrap()), layer.get_value("address").unwrap()));
        _self.list_box.add(&create_row(format!("{}:", layer.get_title("unused").unwrap()), layer.get_value("unused").unwrap()));

        _self
    }
}
